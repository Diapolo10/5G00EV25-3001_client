use egui::{Align, Layout, Ui, Vec2};

pub fn chatroom(ctx: &egui::Context, ui: &mut Ui, selected_chatroom: &mut String) {
    let text_style = egui::TextStyle::Body;
    let row_height = ui.text_style_height(&text_style);
    // let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
    let total_rows = 100;
    ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
        ui.style_mut().spacing.item_spacing = Vec2 { x: 5., y: 5. };
        ui.heading("Current chatroom: ".to_string() + &selected_chatroom);
        egui::ScrollArea::vertical()
            .id_source("chatroom")
            .max_width(f32::INFINITY)
            .show_rows(ui, row_height, total_rows, |ui, row_range| {
                for row in row_range {
                    let text = format!("Row {}/{}", row + 1, total_rows);
                    ui.label(text);
                }
            });
    });
}
