use eframe::egui::{Align, Layout};
use eframe::App;

use crate::{chatroom, loginpage, side_pane, window_frame, ChatApp, HttpClient};

impl App for ChatApp {
    //! Implement the app trait for the struct

    //(/) Called by the framework to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { .. } = self;
        // Create the http client instance
        let http_client = HttpClient::default();
        let title = "EguiValet";
        window_frame(ctx, frame, title, |ui| {
            // Create layout and add the components
            ui.with_layout(Layout::left_to_right(Align::default()), |ui| {
                // Show login page if no token is found / user isn't logged in
                if self.user_info.is_logged_in {
                    side_pane(
                        ctx,
                        ui,
                        &http_client,
                        &mut self.trigger_fetch_rooms,
                        &mut self.trigger_fetch_messages,
                        &mut self.show_modal,
                        &mut self.user_info,
                        &mut self.rooms,
                        &mut self.selected_room,
                        &mut self.chatroom_search,
                        &mut self.new_chatroom,
                    );
                    chatroom(
                        ctx,
                        ui,
                        &http_client,
                        &mut self.trigger_fetch_rooms,
                        &mut self.trigger_fetch_messages,
                        &self.user_info,
                        &mut self.messages,
                        &mut self.selected_room,
                        &mut self.message,
                    );
                } else {
                    loginpage(
                        ctx,
                        ui,
                        &http_client,
                        &mut self.user_info,
                        &mut self.signupmode,
                    );
                }
            });
        });
    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {}
}
