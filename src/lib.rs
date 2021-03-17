#[macro_use]
extern crate diesel;

use async_graphql::{EmptyMutation, Schema};

pub use crate::items::{ItemQuery, ItemSubscription};

pub mod data;
pub mod items;
mod pub_sub;
pub mod routes;
mod schema;

pub type RGSSchema = Schema<ItemQuery, EmptyMutation, ItemSubscription>;
/// The name of the collection in which to store the key value pairs in the Sonic server
pub const COLLECTION: &str = "search";
/// The name of the bucket in which to store the key value pairs in the Sonic server
pub const BUCKET: &str = "default";
