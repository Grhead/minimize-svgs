mod structs;
mod graphql_structs;

use aragog::{DatabaseConnection, DatabaseRecord, Record};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::structs::{Taskie_edge_collection, Taskie_projects_collection, Taskie_users_collection};

fn main() {
    let database_connection = DatabaseConnection::builder()
        .with_credentials("http://localhost:8529", "Taskie", "root", "GY7Hxee1zlT5")
        .build()
        .unwrap();
    let id = Uuid::new_v4();
    let dish = DatabaseRecord::create(Taskie_projects_collection {
        id: id.to_string(),
        title: "some project".to_string(),
        shortDescription: "some description".to_string(),
        logo: "".to_string(),
        nova: "First".to_string(),
    }, &database_connection).unwrap();
    let id = Uuid::new_v4();
    let order = DatabaseRecord::create(Taskie_users_collection {
        id: id.to_string(),
        login: "Grenka".to_string(),
        nova: "First".to_string(),
    }, &database_connection).unwrap();
    let edge = DatabaseRecord::link(&dish, &order, &database_connection, {
        Taskie_edge_collection { }
    }).unwrap();
}