use actix_web::{App, HttpServer};
use async_graphql::{extensions::ApolloTracing, EmptyMutation, Schema};
use diesel::{r2d2::ConnectionManager, PgConnection};
use rust_graphql_sonic::{routes::index_routes, ItemQuery, ItemSubscription};
use sonic_channel::{SearchChannel, SonicChannel};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the environment variables with dotenv and get the required values from the environment
    dotenv::dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let sonic_url = env::var("SONIC_URL").expect("SONIC_URL missing!");
    let sonic_pwd = env::var("SONIC_PWD").unwrap_or_default();

    // Create connection manager and a connection pool using r2d2
    let manager = ConnectionManager::<PgConnection>::new(url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool!");

    // Connect to a Sonic channel in search mode
    let channel = SearchChannel::start(sonic_url, sonic_pwd).expect("Failed to connect to sonic!");

    // Create a GraphQL Schema with the connection pool and Sonic channel as data
    let schema = Schema::build(ItemQuery, EmptyMutation, ItemSubscription)
        .data(pool.clone())
        .data(channel)
        .extension(ApolloTracing)
        .finish();

    // Print start acknowledgement message
    println!("Playground at: http://localhost:7878/graphql");

    // Start the app and bind it to listen to requests only from the localhost on port 7878
    HttpServer::new(move || App::new().data(schema.clone()).service(index_routes()))
        .bind("127.0.0.1:7878")?
        .run()
        .await
}
