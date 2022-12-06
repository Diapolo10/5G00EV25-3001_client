/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct User {
    pub username: String,
    pub email: String,
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
