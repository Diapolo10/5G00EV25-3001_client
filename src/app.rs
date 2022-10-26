use eframe::egui::{Align, Layout};
use eframe::App;
use egui::style::Margin;

use crate::{chatroom, side_pane, window_frame, ChatApp, HttpClient};

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
                // TODO: Add margin around the contents inside window_frame (this is not working)
                ui.style_mut().spacing.window_margin = Margin {
                    left: 5.,
                    top: 5.,
                    right: 5.,
                    bottom: 5.,
                };
                side_pane(
                    ctx,
                    ui,
                    &http_client,
                    &mut self.trigger_fetch_rooms,
                    &mut self.rooms,
                    &mut self.selected_room,
                    &mut self.chatroom_search,
                );
                chatroom(ctx, ui, &self.selected_room, &mut self.message);
            });
        });
    }
}
