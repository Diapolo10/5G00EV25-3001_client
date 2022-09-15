use ::client::ChatApp;
use ::egui::Vec2;

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(1280., 720.));
    eframe::run_native(
        "Chat App",
        native_options,
        Box::new(|cc| Box::new(ChatApp::new(cc))),
    );
}
