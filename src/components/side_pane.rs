use egui::{Align, Layout, Ui};

pub fn side_pane(ctx: &egui::Context, ui: &mut Ui) {
    ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
        ui.heading("text");
        ui.add_space(2.);
        ui.label("another text");
    });
}
