#[macro_use]
extern crate diesel_migrations;

use diesel::{Connection, PgConnection};
use rust_graphql_sonic::{
    data::{get_tennis_matches, get_tennis_players},
    items::{insert_multiple, Item},
    BUCKET, COLLECTION,
};
use sonic_channel::{IngestChannel, SonicChannel};
use std::env;

// The maximum number of parameters a PostgreSQL query can accept
const MAX_PARAMS: usize = 65534;

// Embed migrations
embed_migrations!("migrations");

fn main() -> std::result::Result<(), Box<dyn std::error::Error + 'static>> {
    // Initialize the environment variables with dotenv and get the required values from the environment
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL missing!");
    let sonic_url = env::var("SONIC_URL").expect("SONIC_URL missing!");
    let sonic_pwd = env::var("SONIC_PASSWORD").unwrap_or_default();

    let tennis_atp_path = &env::args().collect::<Vec<String>>()[1];

    // Connect to a Sonic channel in ingest mode
    let channel = IngestChannel::start(sonic_url, sonic_pwd)?;

    // Create a connection manager and connect to database
    let connection = PgConnection::establish(&db_url)?;

    // Run migrations
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout())?;

    // Read all the players from the csv file and store them in a HashMap
    let players = get_tennis_players(&tennis_atp_path);

    // Read all the matches and tourneys from the csv files and store them in HashMaps
    let (tourneys, matches) = get_tennis_matches(&tennis_atp_path);

    // Allocate memory for a vector of Item objects
    let mut items = Vec::<Item>::with_capacity(MAX_PARAMS / 4);

    // Go through all the matches and add them to the Items vector and push them to the Sonic server
    for (id, tennis_match) in matches {
        // Create a match description string from the match, tourney and players' data
        let entry = format!(
            "{} beat {} at {} in a {}",
            players.get(&tennis_match.winner_id).unwrap(),
            players.get(&tennis_match.loser_id).unwrap(),
            tourneys.get(&tennis_match.tourney_id).unwrap(),
            tennis_match
        );

        // Push the item to the vector
        items.push(Item::new(&id, &entry));

        // Save the items to the database if the number of items to save is greater then
        // a quarter of the maximum allowed parameters for PostgreSQL and clear the vector
        if items.len() > MAX_PARAMS / 4 {
            insert_multiple(&connection, &items)?;
            items.clear();
        }

        // Push the item to the Sonic server
        channel.push(COLLECTION, BUCKET, &id, &entry)?;
    }

    // Save any remaining items to the database
    if !items.is_empty() {
        insert_multiple(&connection, &items)?;
    }

    // Quit the channel and finish the program
    match channel.quit() {
        Ok(_) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}
