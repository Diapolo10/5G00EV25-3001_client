//! Crate documentation goes here
mod configure_app;
pub use configure_app::ChatApp;
mod app;
mod components;
pub use components::{chatroom, side_pane, window_frame};
