<<<<<<< HEAD
<<<<<<< HEAD
use serde::{Deserialize, Serialize};

/// Define fields for user. Used for creating and logging users.
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub email: String,
    // Only used to take input when loggin in or creating a user. Not used to save password variable on client side!
    pub password: String,
    pub token: String,
    pub is_logged_in: bool,
}

impl Default for User {
    fn default() -> Self {
        Self {
            username: "".to_owned(),
            email: "".to_owned(),
            password: "".to_owned(),
            token: "".to_owned(),
            is_logged_in: false,
        }
    }
}