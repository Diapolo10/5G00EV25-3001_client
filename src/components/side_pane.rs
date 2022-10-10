use egui::{Align, Layout, Stroke, Ui, Vec2};

pub fn side_pane(
    ctx: &egui::Context,
    ui: &mut Ui,
    chatrooms: &mut Vec<(String, String)>,
    selected_chatroom: &mut String,
    chatroom_search: &mut String,
) {
    // Use 20% of width for the side pane
    ui.allocate_ui_with_layout(
        Vec2 {
            x: ui.available_width() * 0.2,
            y: ui.available_height(),
        },
        Layout::top_down(Align::Center),
        |ui| {
            ui.style_mut().spacing.item_spacing = Vec2 { x: 5., y: 10. };
            ui.add_space(10.);
            ui.heading("User profile");
            ui.add_space(10.);
            // Add a line under "User Profile"
            let rect = ui.max_rect();
            let painter = ui.painter();
            let text_color = ctx.style().visuals.text_color();
            let height = 50.;
            painter.line_segment(
                [
                    rect.left_top() + Vec2 { x: 0., y: height },
                    rect.right_top() + Vec2 { x: 0., y: height },
                ],
                Stroke::new(1.0, text_color),
            );
            let row_height = ui.spacing().interact_size.y;
            // ScrollArea to host all chatrooms as buttons
            egui::ScrollArea::vertical()
                .id_source("side_pane")
                .max_width(ui.available_width())
                .show_rows(ui, row_height, chatrooms.len(), |ui, _row_range| {
                    // TextEdit for searching for a chatroom
                    ui.add(
                        egui::TextEdit::singleline(chatroom_search)
                            .id_source("search_response")
                            .hint_text("Search for a chatroom")
                            .desired_width(ui.available_width())
                            .margin(Vec2 { x: 8., y: 4. }),
                    );
                    // Show all chatrooms and if TextEdit contains something filter case insensitively
                    for i in chatrooms
                        .iter()
                        .filter(|x| x.0.to_lowercase().contains(&chatroom_search.to_lowercase()))
                        .enumerate()
                    {
                        let text = i.1;
                        let button =
                            ui.add_sized([ui.available_width(), 30.], egui::Button::new(&text.0));
                        if button.clicked() {
                            *selected_chatroom = text.0.clone();
                            println!("{}", &text.0);
                        }
                    }
                });
        },
    );
}