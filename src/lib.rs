//! EguiValet chat app
mod structs;
pub use structs::{ChatApp, HttpClient, Room, Rooms, User};
mod app;
mod components;
pub use components::{chatroom, loginpage, side_pane, window_frame};
