use egui::{Align2, Button, FontId, Id, Rect, RichText, Sense, Stroke, Vec2};

pub fn window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    //! A component that adds a window frame instead of default OS window decorations. It has a title bar with a close button.
    let text_color = ctx.style().visuals.text_color();
    // Height of the title bar
    let height = 40.0;

    // The central panel is the region left after adding TopPanel's and SidePanel's
    egui::CentralPanel::default().show(ctx, |ui| {
        let rect = ui.max_rect();
        let painter = ui.painter();

        // Create the title bar first
        // Paint the frame:
        painter.rect(
            rect.shrink(0.),
            10.0,
            ctx.style().visuals.window_fill(),
            Stroke::new(1.0, text_color),
        );

        // Paint the title:
        painter.text(
            rect.center_top()
                + Vec2 {
                    x: 0.0,
                    y: height / 2.0,
                },
            Align2::CENTER_CENTER,
            title,
            FontId::proportional(height * 0.8),
            text_color,
        );

        // Paint the line under the title:
        painter.line_segment(
            [
                rect.left_top() + Vec2 { x: 2.0, y: height },
                rect.right_top() + Vec2 { x: -2.0, y: height },
            ],
            Stroke::new(1.0, text_color),
        );

        // Add the close button:
        let close_response = ui.put(
            Rect::from_min_size(rect.left_top(), Vec2::splat(height)),
            Button::new(RichText::new("❌").size(height - 4.0)).frame(false),
        );
        if close_response.clicked() {
            frame.close();
        }

        // Interact with the title bar (drag to move window):
        let title_bar_rect = {
            let mut rect = rect;
            rect.max.y = rect.min.y + height;
            rect
        };
        let title_bar_response = ui.interact(title_bar_rect, Id::new("title_bar"), Sense::click());
        if title_bar_response.is_pointer_button_down_on() {
            frame.drag_window();
        }

        // Add the contents:
        let content_rect = {
            let mut rect = rect;
            rect.min.y = title_bar_rect.max.y;
            rect
        }
        .shrink(3.0);
        let mut content_ui = ui.child_ui(content_rect, *ui.layout());
        add_contents(&mut content_ui);
    });
}
