use chrono::prelude::*;
use egui::{Align, Color32, Direction, Layout, Ui, Vec2};
use reqwest::header::CONTENT_TYPE;
use uuid::Uuid;

use crate::{
    structs::message,
    structs::message::{Message, Messages},
    structs::user::User,
    HttpClient, Room,
};

// Tests for chatroom component
#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::structs::message::Messages;
    use crate::structs::user::User;
    use crate::{chatroom, HttpClient, Room};

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
                &mut false,
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
    user_id: &str,
    room_id: &mut String,
    message: &mut str,
) {
    let id = Uuid::new_v4().to_string();
    let mut current_time = Local::now().to_string();
    // Cut timezone from current_time variable
    current_time = current_time[0..current_time.len() - 7].to_string();
    let body = Message {
        id,
        user_id: user_id.to_owned(),
        room_id: room_id.clone(),
        message: message.to_owned(),
        creation_time: current_time.clone(),
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
            if res.status().is_success() {
                println!("Message sent: {:#?}", res.json::<message::Res>());
                *trigger_fetch = true;
            } else {
                println!("Post messages error: {}", res.status());
            }
        }
        Err(err) => println!("Post messages error: {err}"),
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
            if res.status().is_success() {
                let messages = res.json::<Vec<message::Res>>().unwrap_or_default();
                println!("{messages:#?}");
                Messages { messages }
            } else {
                println!("Fetch messages error: {}", res.status());
                Messages { messages: vec![] }
            }
        }
        Err(err) => {
            println!("Fetch messages error: {err}");
            Messages { messages: vec![] }
        }
    }
}

fn delete_room(http_client: &HttpClient, trigger_fetch: &mut bool, selected_room: &mut Room) {
    match http_client
        .client
        .delete(format!(
            "{}{}{}",
            http_client.base_url, "api/v1/rooms/", selected_room.id
        ))
        .send()
    {
        Ok(res) => {
            if res.status().is_success() {
                println!("Room deleted");
                *selected_room = Room::default();
                *trigger_fetch = true;
            } else {
                println!("Delete room error: {}", res.status());
            }
        }
        Err(err) => println!("Delete room error: {err}"),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn chatroom(
    _ctx: &egui::Context,
    ui: &mut Ui,
    http_client: &HttpClient,
    trigger_fetch_rooms: &mut bool,
    trigger_fetch_messages: &mut bool,
    user_info: &User,
    messages: &mut Messages,
    selected_room: &mut Room,
    message: &mut String,
) {
    //! A component where chatroom is opened and all the messages are shown.
    //! It takes up a majority of the screen.

    if *trigger_fetch_messages {
        *messages = fetch_messages(http_client, &mut selected_room.id);
        *trigger_fetch_messages = false;
    }

    // If user has selected a room
    if *selected_room == Room::default() {
        // If no room is selected, request user to select or create one
        ui.with_layout(
            Layout::centered_and_justified(Direction::LeftToRight),
            |ui| {
                ui.heading("Open a room or create a new one to begin");
            },
        );
    } else {
        let desired_height_rows = 2;
        // Use bottom_up layout to create user message field and button first and leave the remaining space for messages
        ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
            // The initial height of horizontal layout is "ui.style_mut().spacing.interact_size.y" so change it to match the size of the TextEdit
            // TODO: Make this size dynamic so it doesn't break when the message has more than 2 rows
            ui.style_mut().spacing.interact_size.y = 50.;
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
                ui.add_space(ui.available_width() * 0.2);
                // Change this back to a smaller size before creating the button
                ui.style_mut().spacing.interact_size.y = 20.;
                let button = ui.add_sized(
                    [ui.available_width() * 0.5, ui.available_height() * 0.75],
                    egui::Button::new("Send"),
                );
                if button.clicked() && !message.is_empty() {
                    send_message(
                        http_client,
                        trigger_fetch_messages,
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
                ui.heading(&selected_room.name);
                ui.style_mut().spacing.interact_size.y = 10.;
                // Show delete room button if logged in user is the owner of the room
                if user_info.user_id == selected_room.owner
                    && ui.add(egui::Button::new("Delete room")).clicked()
                {
                    delete_room(http_client, trigger_fetch_rooms, selected_room);
                }

                egui::ScrollArea::vertical()
                    .id_salt("chatroom")
                    .max_width(ui.available_width())
                    .show(ui, |ui| {
                        for row in messages.messages.iter().enumerate() {
                            let message: &message::Res = row.1;
                            let text = &message.message;
                            let creation_date = message.creation_time[0..10].to_owned();
                            let creation_time = message.creation_time[11..16].to_owned();
                            // Show date on first message and every time message creation date changes
                            if row.0 == 0
                                || !creation_date
                                    .eq(&messages.messages[row.0 - 1].creation_time[0..10]
                                        .to_owned())
                            {
                                ui.label(creation_date);
                            }
                            egui::containers::Frame::none()
                                .outer_margin(egui::style::Margin {
                                    left: 5.,
                                    right: 5.,
                                    top: 3.,
                                    bottom: 3.,
                                })
                                .inner_margin(egui::style::Margin {
                                    left: 5.,
                                    right: 5.,
                                    top: 3.,
                                    bottom: 3.,
                                })
                                .rounding(egui::Rounding {
                                    nw: 5.0,
                                    ne: 5.0,
                                    sw: 5.0,
                                    se: 5.0,
                                })
                                .fill(Color32::from_rgb(50, 50, 50))
                                .show(ui, |ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        // TODO: Show own messages on the right side and messages from others on the left
                                        ui.style_mut().spacing.item_spacing = Vec2 { x: 5., y: 5. };
                                        ui.label(creation_time);
                                        ui.label(text);
                                    });
                                });
                        }
                    });
            })
        });
    }
}
