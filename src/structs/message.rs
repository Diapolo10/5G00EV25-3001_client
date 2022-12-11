use serde::{Deserialize, Serialize};

/// Defines fields for a message. Used for receiving and posting messages
/// as well as storing them in a vector in an instance of Messages.
#[derive(Deserialize, Serialize, Debug)]
pub struct Message {
    pub id: String,
    pub user_id: String,
    pub room_id: String,
    pub message: String,
    pub creation_time: String,
    pub last_edited: String,
}

/// Used for checking response when sending a new message to server
#[derive(Deserialize, Serialize, Debug)]
pub struct ResMessage {
    pub id: String,
    pub user_id: String,
    pub message: String,
    pub creation_time: String,
}

/// For storing messages in a vector
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Messages {
    pub messages: Vec<ResMessage>,
}
