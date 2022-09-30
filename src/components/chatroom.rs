use egui::{Align, Layout, Ui, Vec2};

pub fn chatroom(
    ctx: &egui::Context,
    ui: &mut Ui,
    selected_chatroom: &mut String,
    message: &mut String,
) {
    let text_style = egui::TextStyle::Body;
    let row_height = ui.text_style_height(&text_style);
    // let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
    let total_rows = 100;
    let desired_height_rows = 2;
    // Use bottom_up layout to create user message field first and leave the remaining space for messages
    ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
        // The initial height of horizontal layout is "ui.style_mut().spacing.interact_size.y" so change it to match the size of the TextEdit
        // TODO: Make this size dynamic so it doesn't break when the message has more than 2 rows
        ui.style_mut().spacing.interact_size.y = 50.;
        ui.add_space(5.);
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::multiline(message)
                    .id_source("user_message")
                    .hint_text("Message ".to_string() + selected_chatroom)
                    .desired_width(ui.available_width() * 0.8)
                    .desired_rows(desired_height_rows)
                    .margin(Vec2 { x: 8., y: 4. }),
            );
            // Change this back to a smaller size before creating the button
            ui.style_mut().spacing.interact_size.y = 20.;
            let button = ui.add_sized(
                [ui.available_width() * 0.2, ui.available_height() * 0.5],
                egui::Button::new("Send"),
            );
            if button.clicked() {
                message.clear();
                println!("Message sent!");
            }
        });
        ui.add_space(5.);
        // Print messages
        ui.with_layout(Layout::top_down(Align::Center), |ui| {
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
        })
    });
}
