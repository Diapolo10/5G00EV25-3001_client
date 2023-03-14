use ::client::ChatApp;
use ::egui::Vec2;
// use winit;

fn main() {
    // let mut monitors = winit::event_loop::EventLoop::new().available_monitors();
    // let first_monitor_size = monitors.next().unwrap().size();
    let width = 1280.0;
    let height = 720.0;
    let native_options = eframe::NativeOptions {
        decorated: false, // Hides the top panel
        initial_window_size: Some(Vec2::new(width, height)),
        // Start app on the center of the main monitor
        // initial_window_pos: Some(Pos2::new(
        //     (first_monitor_size.width as f32 - width) / 2.0,
        //     (first_monitor_size.height as f32 - height) / 2.0,
        // )),
        ..Default::default()
    };

    let result = eframe::run_native(
        "EguiValet",
        native_options,
        Box::new(|cc| Box::new(ChatApp::new(cc))),
    );

    match result {
        Ok(_) => (),
        Err(e) => println!("{e:?}"),
    };
}
