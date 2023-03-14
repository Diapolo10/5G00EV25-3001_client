use crate::{
    structs::user as user_,
    HttpClient,
    User,
};
use egui::{Align, Button, Layout, Ui, Vec2};
use regex::Regex;
use reqwest::header::CONTENT_TYPE;
use uuid::Uuid;

fn signup(http_client: &HttpClient, user: &user_::Signup) -> bool {
    match http_client
        .client
        .post(format!("{}{}", http_client.base_url, "api/v1/users/"))
        .header(CONTENT_TYPE, "application/json")
        .json(&user)
        .send()
    {
        Ok(res) => {
            if res.status().is_success() {
                println!("Sign up successful");
                true
            } else {
                println!("Sign up error: {}", res.status());
                false
            }
        }
        Err(err) => {
            println!("Sign up error: {err}");
            false
        }
    }
}

#[allow(dead_code)]
fn login(http_client: &HttpClient, user: &user_::Login) -> User {
    match http_client
        .client
        .post(format!("{}{}", http_client.base_url, "api/v1/users/login"))
        .header(CONTENT_TYPE, "application/json")
        .json(&user)
        .send()
    {
        Ok(res) => {
            if res.status().is_success() {
                let user = res.json::<user_::Res>().unwrap_or_default();
                println!("{user:#?}");
                println!("Login successful");
                User {
                    user_id: user.user_id,
                    username: user.username,
                    email: user.email,
                    password: String::new(),
                    token: user.token,
                    global_access_level: user.global_access_level,
                    is_logged_in: true,
                }
            } else {
                println!("Login error: {}", res.status());
                User::default()
            }
        }
        Err(err) => {
            println!("Login error: {err}");
            User::default()
        }
    }
}

pub fn loginpage(
    _ctx: &egui::Context,
    ui: &mut Ui,
    http_client: &HttpClient,
    user_info: &mut user_::User,
    signupmode: &mut bool,
) {
    //! A component shown to an unauthorized user. It takes the entire screen
    //! and will disappear once user is logged in.
    //!
    //! # Panics
    //!
    //! Panics if there's an error parsing the regular expression literal

    // ui.allocate_ui_with_layout(
    //     Vec2 {
    //         x: ui.available_width() * 0.33,
    //         y: ui.available_height(),
    //     },
    //     Layout::centered_and_justified(egui::Direction::LeftToRight),
    //     |ui| {
    //         ui.heading("EguiValet");
    //     },
    // );

    ui.with_layout(Layout::top_down(Align::Center), |ui| {
        ui.add_space(ui.available_height() * 0.25);
        ui.style_mut().spacing.item_spacing = Vec2 { x: 5., y: 10. };

        let email_regex = Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .expect("The regex literal is invalid, fix it");

        if *signupmode {
            ui.heading("Signup");
            ui.add_space(10.);
            ui.label("Username");
            ui.text_edit_singleline(&mut user_info.username);
        } else {
            ui.heading("Login");
            ui.add_space(10.);
        }

        ui.label("Email");
        ui.text_edit_singleline(&mut user_info.email);

        ui.label("Password");
        ui.add(
            egui::TextEdit::singleline(&mut user_info.password)
                .password(true)
                .id_source("user_password"),
        );

        if *signupmode {
            // Show clickable signup button if email field contains an email and password and username fields aren't empty
            if !user_info.username.is_empty()
                && !user_info.password.is_empty()
                && email_regex.is_match(&user_info.email)
            {
                if ui.button("Sign up").clicked() {
                    // Sign up user
                    let user_signup = user_::Signup {
                        id: Uuid::new_v4().to_string(),
                        email: (*user_info.email.clone()).to_string(),
                        username: (*user_info.username.clone()).to_string(),
                        password: (*user_info.password.clone()).to_string(),
                    };
                    // If signup was successful
                    if signup(http_client, &user_signup) {
                        *user_info = User::default();
                        *signupmode = false;
                    }
                    // TODO: Tell user if signup was not successful
                }
            } else {
                // Show disabled signup button if input fields have invalid input
                ui.add_enabled(false, Button::new("Sign up"));
            }
            if ui.button("Login instead").clicked() {
                *signupmode = false;
            }
        } else {
            // Show clickable login button if email field contains an email and password field isn't empty
            if email_regex.is_match(&user_info.email) && !user_info.password.is_empty() {
                if ui.button("Log in").clicked() {
                    // Login user
                    // let user_login = UserLogin {
                    //     email: (*user_info.email.to_owned()).to_string(),
                    //     password: (*user_info.password.to_owned()).to_string(),
                    // };
                    // *user_info = login(http_client, user_login);

                    // If login was successful and login function returned something other than an empty user
                    if *user_info != User::default() {
                        // Temporary code until login is implemented on the backend
                        *user_info = User {
                            user_id: Uuid::new_v4().to_string(),
                            username: "username".to_owned(),
                            email: "user@user.com".to_owned(),
                            password: String::new(),
                            token: String::new(),
                            global_access_level: 2,
                            is_logged_in: true,
                        }
                    }
                    // TODO: Tell user if login was not successful
                }
            } else {
                // Show disabled login button if input fields have invalid input
                ui.add_enabled(false, Button::new("Log in"));
            }
            if ui.button("Signup instead").clicked() {
                *signupmode = true;
            }
        }
    });
}
