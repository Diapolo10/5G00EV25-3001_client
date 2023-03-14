//! `EguiValet` chat app
mod structs;
pub use structs::{user::User, ChatApp, HttpClient, Room, Rooms};
mod app;
mod components;
pub use components::{chatroom, loginpage, side_pane, window_frame};
