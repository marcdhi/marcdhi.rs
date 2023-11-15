#![allow(dead_code, unused_variables)]

mod database; // here rust automatically looks for database.rs or database/mod.rs

mod auth_utils; // here rust automatically looks for auth_utils.rs or auth_utils/mod.rs

use auth_utils::models::Credentials; 

use database::Status;

pub fn authenticate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}