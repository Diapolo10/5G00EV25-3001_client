use eframe::egui::{Align, Layout};
use eframe::App;
use egui::style::Margin;

use crate::{chatroom, loginpage, side_pane, window_frame, ChatApp, HttpClient};

impl App for ChatApp {
    //! Implement the app trait for the struct

    //(/) Called by the framework to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    fn persist_native_window(&self) -> bool {
        false
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { .. } = self;
        // Create the http client instance
        let http_client = HttpClient::default();
        let title = "EguiValet";
        window_frame(ctx, frame, title, |ui| {
            // Create layout and add the components
            ui.with_layout(Layout::left_to_right(Align::default()), |ui| {
                // TODO: Add margin around the contents inside window_frame (this is not working)
                ui.style_mut().spacing.window_margin = Margin {
                    left: 5.,
                    top: 5.,
                    right: 5.,
                    bottom: 5.,
                };
                // Show login page if no token is found / user isn't logged in
                if !self.user_info.is_logged_in {
                    loginpage(ctx, ui, &mut self.user_info);
                } else {
                    side_pane(
                        ctx,
                        ui,
                        &http_client,
                        &mut self.trigger_fetch_rooms,
                        &mut self.trigger_fetch_messages,
                        &mut self.rooms,
                        &mut self.selected_room,
                        &mut self.chatroom_search,
                    );
                    chatroom(
                        ctx,
                        ui,
                        &http_client,
                        &mut self.trigger_fetch_messages,
                        &self.user_info,
                        &mut self.messages,
                        &mut self.selected_room,
                        &mut self.message,
                    );
                }
            });
        });
    }
}
