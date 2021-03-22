#[macro_use]
extern crate diesel_migrations;

use actix_web::{App, HttpServer};
use async_graphql::{extensions::ApolloTracing, EmptyMutation, Schema};
use diesel::{r2d2::ConnectionManager, PgConnection};
use rust_graphql_sonic::{
    routes::{index_routes, SCOPE},
    ItemQuery, ItemSubscription,
};
use sonic_channel::{SearchChannel, SonicChannel};
use std::env;

// Embed migrations
embed_migrations!("migrations");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the environment variables with dotenv and get the required values from the environment
    dotenv::dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let sonic_url = env::var("SONIC_URL").expect("SONIC_URL missing!");
    let sonic_pwd = env::var("SONIC_PASSWORD").unwrap_or_default();
    let host_addr = env::var("HOST").unwrap_or_else(|_| String::from("0.0.0.0"));
    let port = env::var("PORT").unwrap_or_else(|_| String::from("7878"));
    let address = format!("{}:{}", host_addr, port);

    // Create connection manager and a connection pool using r2d2
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool!");

    let connection = pool.get().expect("Could not get connection from pool!");

    // Run migrations
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())
        .expect("Could not run migrations!");

    // Connect to a Sonic channel in search mode
    let channel = SearchChannel::start(sonic_url, sonic_pwd).expect("Failed to connect to sonic!");

    // Create a GraphQL Schema with the connection pool and Sonic channel as data
    let schema = Schema::build(ItemQuery, EmptyMutation, ItemSubscription)
        .data(pool.clone())
        .data(channel)
        .extension(ApolloTracing)
        .finish();

    // Print start acknowledgement message
    println!("Playground at: http://{}{}", &address, SCOPE);

    // Start the app and bind it to listen to requests only from the localhost on port 7878
    HttpServer::new(move || App::new().data(schema.clone()).service(index_routes()))
        .bind(address)?
        .run()
        .await
}
