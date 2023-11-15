pub mod models; // here rust automatically looks for models.rs or models/mod.rs

use crate::database::get_user;

pub fn login(creds: models::Credentials) {
    get_user();
}

fn logout() {
    // log out user..
}