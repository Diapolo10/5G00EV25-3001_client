//! Crate documentation goes here
mod structs;
pub use structs::{ChatApp, HttpClient, Room, Rooms};
mod app;
mod components;
pub use components::{chatroom, side_pane, window_frame};
