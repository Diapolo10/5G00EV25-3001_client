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
=======
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
=======
use serde::{Deserialize, Serialize};

// Define fields for user. Used for creating and logging users.
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
>>>>>>> bb197f8 (Add user_id to user struct an add documentation)
pub struct User {
    pub user_id: String,
    pub username: String,
    pub email: String,
>>>>>>> 52bf42c (User struct and loginpage created)
    pub password: String,
    pub token: String,
    pub is_logged_in: bool,
}
<<<<<<< HEAD
<<<<<<< HEAD
=======

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
>>>>>>> 52bf42c (User struct and loginpage created)
=======
>>>>>>> bb197f8 (Add user_id to user struct an add documentation)
