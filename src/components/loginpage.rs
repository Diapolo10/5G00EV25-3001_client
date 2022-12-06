use crate::User;
use egui::{Align, Layout, Ui, Vec2};

pub fn loginpage(_ctx: &egui::Context, ui: &mut Ui, user_info: &mut User) {
    ui.allocate_ui_with_layout(
        Vec2 {
            x: ui.available_width() * 0.33,
            y: ui.available_height(),
        },
        Layout::centered_and_justified(egui::Direction::LeftToRight),
        |ui| {
            ui.heading("EguiValet");
        },
    );

    ui.with_layout(Layout::top_down(Align::Center), |ui| {
        ui.add_space(ui.available_height() * 0.33);
        ui.style_mut().spacing.item_spacing = Vec2 { x: 5., y: 10. };
        ui.heading("Log In");

        ui.label("Email");
        ui.text_edit_singleline(&mut user_info.email);
        ui.label("Password");
        ui.add(
            egui::TextEdit::singleline(&mut user_info.password)
                .password(true)
                .id_source("user_password"),
        );

        if ui.button("Log In").clicked() {
            // Login user
            println!("{}", &mut user_info.username);
            println!("{}", &mut user_info.password);
            *user_info = User {
                username: user_info.username.clone(),
                email: user_info.email.clone(),
                password: user_info.password.clone(),
                token: user_info.token.clone(),
                is_logged_in: true,
            };
        }
        if ui.button("Sign up instead").clicked() {
            // Create user
            println!("Change to sign up");
        }
    });
}
