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
    pub is_logged_in: bool,
}
