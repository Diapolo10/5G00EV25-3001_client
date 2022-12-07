use chrono::prelude::*;
use egui::{Align, Color32, Direction, Layout, Ui, Vec2};
use reqwest::header::CONTENT_TYPE;
use uuid::Uuid;

use crate::{
    structs::{Message, Messages, ResMessage},
    HttpClient, Room, User,
};

// Tests for chatroom component
#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::{chatroom, structs::Messages, HttpClient, Room, User};

    #[test]
    fn some_test() {
        let ctx = egui::Context::default();
        let http_client = HttpClient::default();
        let mut messages = Messages::default();
        let mut user = User::default();

        let mut room = Room {
            id: Uuid::new_v4().to_string(),
            name: "Chatroom 1".to_owned(),
            public: true,
            owner: Uuid::new_v4().to_string(),
        };

        egui::__run_test_ui(|ui| {
            chatroom(
                &ctx,
                ui,
                &http_client,
                &mut true,
                &mut user,
                &mut messages,
                &mut room,
                &mut "New message".to_owned(),
            );
        });
    }
}

fn send_message(
    http_client: &HttpClient,
    trigger_fetch: &mut bool,
    user_id: &String,
    room_id: &mut String,
    message: &mut String,
) {
    let id = Uuid::new_v4().to_string();
    let mut current_time = Local::now().to_string();
    // Cut timezone from current_time variable
    current_time = current_time[0..current_time.len() - 7].to_string();
    let body = Message {
        id,
        user_id: user_id.to_owned(),
        room_id: room_id.to_owned(),
        message: message.to_owned(),
        creation_time: current_time.to_owned(),
        last_edited: current_time,
    };

    println!("{:#?}", &body);

    match http_client
        .client
        .post(format!(
            "{}{}{}",
            http_client.base_url, "api/v1/rooms/", room_id
        ))
        .header(CONTENT_TYPE, "application/json")
        .json(&body)
        .send()
    {
        Ok(res) => {
            if !res.status().is_success() {
                println!("Post messages error: {}", res.status());
            } else {
                println!("Message sent: {:#?}", res.json::<ResMessage>());
                *trigger_fetch = true;
            }
        }
        Err(err) => println!("Post messages error: {}", err),
    }
}

fn fetch_messages(http_client: &HttpClient, room_id: &mut String) -> Messages {
    match http_client
        .client
        .get(format!(
            "{}{}{}{}",
            http_client.base_url, "api/v1/rooms/", room_id, "/messages"
        ))
        .send()
    {
        Ok(res) => {
            if !res.status().is_success() {
                println!("Fetch messages error: {}", res.status());
                Messages { messages: vec![] }
            } else {
                let messages = res.json::<Vec<ResMessage>>().unwrap_or_default();
                println!("{:#?}", messages);
                Messages { messages }
            }
        }
        Err(err) => {
            println!("Fetch messages error: {}", err);
            Messages { messages: vec![] }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn chatroom(
    _ctx: &egui::Context,
    ui: &mut Ui,
    http_client: &HttpClient,
    trigger_fetch: &mut bool,
    user_info: &User,
    messages: &mut Messages,
    selected_room: &mut Room,
    message: &mut String,
) {
    //! A component where chatroom is opened and all the messages are shown.
    //! It takes up a majority of the screen.

    if *trigger_fetch {
        *messages = fetch_messages(http_client, &mut selected_room.id);
        *trigger_fetch = false;
    }

    // If user has selected a room
    if *selected_room != Room::default() {
        let desired_height_rows = 2;
        // Use bottom_up layout to create user message field and button first and leave the remaining space for messages
        ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
            // The initial height of horizontal layout is "ui.style_mut().spacing.interact_size.y" so change it to match the size of the TextEdit
            // TODO: Make this size dynamic so it doesn't break when the message has more than 2 rows
            ui.style_mut().spacing.interact_size.y = 50.;
            ui.add_space(5.);
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::multiline(message)
                        .id_source("user_message")
                        .hint_text("Message ".to_owned() + &selected_room.name)
                        .desired_width(ui.available_width() * 0.9)
                        .desired_rows(desired_height_rows)
                        .margin(Vec2 { x: 8., y: 4. }),
                );
                // Add space before the send button
                ui.add_space(ui.available_width() * 0.1);
                // Change this back to a smaller size before creating the button
                ui.style_mut().spacing.interact_size.y = 20.;
                let button = ui.add_sized(
                    [ui.available_width() * 0.2, ui.available_height() * 0.5],
                    egui::Button::new("Send"),
                );
                if button.clicked() && !message.is_empty() {
                    send_message(
                        http_client,
                        trigger_fetch,
                        &user_info.user_id,
                        &mut selected_room.id,
                        message,
                    );
                    message.clear();
                }
            });
            ui.add_space(5.);
            // Print chatroom
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.add_space(8.);
                ui.heading( &selected_room.name);
                ui.add_space(8.);
                egui::ScrollArea::vertical()
                    .id_source("chatroom")
                    .max_width(ui.available_width())
                    .show(ui, |ui| {
                        for row in messages.messages.iter().enumerate() {
                            let message: &ResMessage = row.1;
                            let text = &message.message;
                            let creation_time = &mut message.creation_time[0..10].to_owned();
                            creation_time.push_str("  ");
                            creation_time.push_str(&message.creation_time[11..19]);

                            egui::containers::Frame::none()
                                .outer_margin(egui::style::Margin {
                                    left: 5.,
                                    right: 5.,
                                    top: 8.,
                                    bottom: 8.,
                                })
                                .inner_margin(egui::style::Margin {
                                    left: 5.,
                                    right: 5.,
                                    top: 2.,
                                    bottom: 2.,
                                })
                                .rounding(egui::Rounding {
                                    nw: 5.0,
                                    ne: 5.0,
                                    sw: 5.0,
                                    se: 5.0,
                                })
                                .fill(Color32::DARK_GRAY)
                                .show(ui, |ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        // TODO: Show own messages on the right side and messages from others on the left
                                        ui.style_mut().spacing.item_spacing =
                                            Vec2 { x: 10., y: 10. };
                                        ui.label(creation_time.to_owned());
                                        ui.label(text);
                                    });
                                });
                        }
                    });
            })
        });
        // If no room is selected, request user to select or create one
    } else {
        ui.with_layout(
            Layout::centered_and_justified(Direction::LeftToRight),
            |ui| {
                ui.heading("Open or create a new room to begin");
            },
        );
    }
}
