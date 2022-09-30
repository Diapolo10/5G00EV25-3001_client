use ::client::ChatApp;
use ::egui::{Pos2, Vec2};

fn main() {
    let native_options = eframe::NativeOptions {
        decorated: false,                                  // Hides the top panel
        initial_window_size: Some(Vec2::new(1280., 720.)), // Not working
        initial_window_pos: Some(Pos2::new(300., 150.)),   // Not working
        // max_window_size: Some(Vec2::new(1280., 720.)),
        ..Default::default()
    };
    eframe::run_native(
        "Chat App",
        native_options,
        Box::new(|cc| Box::new(ChatApp::new(cc))),
    );
}
