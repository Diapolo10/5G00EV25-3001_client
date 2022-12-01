use serde::{Deserialize, Serialize};

/// Defines fields for a room. Used for fetching and posting chatrooms
/// as well as storing them in a vector in an instance of Rooms.
#[derive(Deserialize, Serialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub public: bool,
    pub owner: String,
}

/// For storing chatrooms in a vector
#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Rooms {
    pub rooms: Vec<Room>,
}
