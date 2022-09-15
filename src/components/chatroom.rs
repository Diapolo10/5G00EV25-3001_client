use egui::{Align, Layout, Ui};

pub fn chatroom(ctx: &egui::Context, ui: &mut Ui) {
    let text_style = egui::TextStyle::Body;
    let row_height = ui.text_style_height(&text_style);
    // let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
    let total_rows = 100;
    ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
        egui::ScrollArea::vertical()
            .max_width(f32::INFINITY)
            .show_rows(ui, row_height, total_rows, |ui, row_range| {
                for row in row_range {
                    let text = format!("Row {}/{}", row + 1, total_rows);
                    ui.label(text);
                }
            });
    });
}
