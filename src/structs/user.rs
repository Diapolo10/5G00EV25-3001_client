use serde::{Deserialize, Serialize};

// Define fields for user. Used for creating and logging users.
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub token: String,
    pub is_logged_in: bool,
}
