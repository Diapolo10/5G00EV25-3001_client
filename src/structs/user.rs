use serde::{Deserialize, Serialize};

/// Define fields for user. Used for creating and logging users in as well as storing non-sensitive information.
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub email: String,
    // Only used to take input when loggin in or creating a user. Not used to save password variable on client side!
    pub password: String,
    pub token: String,
    pub global_access_level: i8,
    pub is_logged_in: bool,
}

/// Used for taking user info when signing up
#[derive(Deserialize, Serialize, Debug)]
pub struct Signup {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Used for taking user info when logging in
#[derive(Deserialize, Serialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
}

/// Used for receiving user info when logging in
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Res {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub token: String,
    pub global_access_level: i8,
}
