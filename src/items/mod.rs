mod service;

pub use service::{get_count, insert_multiple};

use async_graphql::{Context, Object, Result, SimpleObject, Subscription};
use diesel::{prelude::PgConnection, r2d2::ConnectionManager, Insertable, Queryable};
use futures::Stream;
use r2d2::Pool;
use sonic_channel::SearchChannel;
use std::{convert::TryInto, time};

use self::service::{get_search_items_like, get_search_items_tsv};
use crate::{items::service::get_search_items, pub_sub::PubSub, schema::items, BUCKET, COLLECTION};

#[derive(Queryable, Clone, Insertable, Debug)]
#[table_name = "items"]
pub struct Item {
    pub id: String,
    pub text: String,
}

impl Item {
    pub fn new(id: &str, text: &str) -> Item {
        Item {
            id: id.parse().unwrap(),
            text: text.parse().unwrap(),
        }
    }
}

pub struct ItemSubscription;

#[Subscription]
impl ItemSubscription {
    /// Subscribe to the results of the search method provided.
    /// Returns an array of strings.
    async fn search_results(&self, method: String) -> impl Stream<Item = Vec<String>> {
        PubSub::subscribe(&format!("results_{}", method))
    }

    /// Subscribe to suggestions of a search.
    /// Returns an array of strings.
    async fn search_suggestion(&self) -> impl Stream<Item = Vec<String>> {
        PubSub::subscribe("suggestions")
    }
}

#[derive(SimpleObject)]
/// An object that gives you time measurements for various portions of a request.
struct TimeMeasurement {
    /// The total time required for performing the request.
    total: u64,

    /// The time required for fetching the results from Sonic.
    sonic_search: u64,

    /// The time required for fetching the suggestions from Sonic.
    sonic_suggest: u64,

    /// The time required for fetching the entries from the database.
    database: u64,
}

#[derive(Default)]
pub struct ItemQuery;

#[Object]
impl ItemQuery {
    /// Perform a search in Sonic and fetch the corresponding entries from the database.
    /// Returns a TimeMeasurement object.
    async fn search_sonic(&self, ctx: &Context<'_>, query: String) -> Result<TimeMeasurement> {
        let start_of_search = time::Instant::now();
        let pool = ctx
            .data::<Pool<ConnectionManager<PgConnection>>>()
            .expect("No connection manager");

        let channel = ctx.data::<SearchChannel>().expect("No search channel");
        let connection = pool.get().expect("Could not get connection from pool");

        let start_of_sonic_query = time::Instant::now();
        let ids = channel.query(COLLECTION, BUCKET, &query)?;
        let end_of_sonic_query = start_of_sonic_query.elapsed().as_millis();

        let start_of_sonic_suggest = time::Instant::now();
        let suggestions = channel.suggest(
            COLLECTION,
            BUCKET,
            &query.split_whitespace().last().unwrap_or_default(),
        )?;
        let end_of_sonic_suggest = start_of_sonic_suggest.elapsed().as_millis();

        let start_of_db_query = time::Instant::now();
        let results = get_search_items(&connection, ids);
        let end_of_db_query = start_of_db_query.elapsed().as_millis();

        PubSub::<Vec<String>>::publish("results_sonic", results);
        PubSub::<Vec<String>>::publish("suggestions", suggestions);

        let end_of_search = start_of_search.elapsed().as_millis();
        Ok(TimeMeasurement {
            total: end_of_search.try_into().unwrap(),
            sonic_search: end_of_sonic_query.try_into().unwrap(),
            sonic_suggest: end_of_sonic_suggest.try_into().unwrap(),
            database: end_of_db_query.try_into().unwrap(),
        })
    }

    /// Perform a search in the database using TsVector GIN index without Sonic.
    /// Returns a TimeMeasurement object.
    async fn search_tsv(&self, ctx: &Context<'_>, query: String) -> Result<TimeMeasurement> {
        let start_of_search = time::Instant::now();
        let pool = ctx
            .data::<Pool<ConnectionManager<PgConnection>>>()
            .expect("No connection manager");

        let connection = pool.get().expect("Could not get connection from pool");

        let start_of_db_query = time::Instant::now();
        let results = get_search_items_tsv(&connection, query);
        let end_of_db_query = start_of_db_query.elapsed().as_millis();

        PubSub::<Vec<String>>::publish("results_tsv", results);

        let end_of_search = start_of_search.elapsed().as_millis();
        Ok(TimeMeasurement {
            total: end_of_search.try_into().unwrap(),
            sonic_search: 0,
            sonic_suggest: 0,
            database: end_of_db_query.try_into().unwrap(),
        })
    }

    /// Perform a search in the database using regular BTree index without Sonic.
    /// Returns a TimeMeasurement object.
    async fn search_like(&self, ctx: &Context<'_>, query: String) -> Result<TimeMeasurement> {
        let start_of_search = time::Instant::now();
        let pool = ctx
            .data::<Pool<ConnectionManager<PgConnection>>>()
            .expect("No connection manager");

        let connection = pool.get().expect("Could not get connection from pool");

        let start_of_db_query = time::Instant::now();
        let results = get_search_items_like(&connection, query);
        let end_of_db_query = start_of_db_query.elapsed().as_millis();

        PubSub::<Vec<String>>::publish("results_like", results);

        let end_of_search = start_of_search.elapsed().as_millis();
        Ok(TimeMeasurement {
            total: end_of_search.try_into().unwrap(),
            sonic_search: 0,
            sonic_suggest: 0,
            database: end_of_db_query.try_into().unwrap(),
        })
    }
}
