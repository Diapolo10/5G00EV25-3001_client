use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub public: bool,
    pub owner: String,
}

#[derive(Deserialize, Debug)]
pub struct Rooms {
    pub rooms: Vec<Room>,
}

impl Rooms {
    /// Called once before the first frame.
    pub fn get_rooms(&self) -> &Vec<Room> {
        return &self.rooms;
    }
}

impl Default for Rooms {
    fn default() -> Self {
        Self { rooms: Vec::new() }
    }
}
