use egui::{Align, Layout, Stroke, Ui, Vec2};
use reqwest::header::CONTENT_TYPE;
use uuid::Uuid;

use crate::{HttpClient, Room, Rooms};

// Tests for side_pane component
#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::{side_pane, HttpClient, Room, Rooms};

    #[test]
    fn some_test() {
        let ctx = egui::Context::default();
        let http_client = HttpClient::default();
        let room_id = Uuid::new_v4().to_string();
        let user_id = Uuid::new_v4().to_string();

        let mut room = Room {
            id: room_id.clone(),
            name: "Chatroom 1".to_owned(),
            public: true,
            owner: user_id.clone(),
        };
        let mut rooms = Rooms {
            rooms: vec![
                Room {
                    id: room_id,
                    name: "Chatroom 1".to_owned(),
                    public: true,
                    owner: user_id,
                },
                Room {
                    id: Uuid::new_v4().to_string(),
                    name: "New room".to_owned(),
                    public: true,
                    owner: Uuid::new_v4().to_string(),
                },
            ],
        };

        egui::__run_test_ui(|ui| {
            side_pane(
                &ctx,
                ui,
                &http_client,
                &mut true,
                &mut true,
                &mut rooms,
                &mut room,
                &mut "chatroom".to_owned(),
            );
        });
    }
}

fn create_room(
    http_client: &HttpClient,
    trigger_fetch: &mut bool,
    room_name: &str,
    room_public: bool,
) {
    let id = Uuid::new_v4().to_string();
    let owner = Uuid::new_v4().to_string();
    let body = Room {
        id,
        name: room_name.to_owned(),
        public: room_public,
        owner,
    };

    match http_client
        .client
        .post(format!("{}{}", http_client.base_url, "api/v1/rooms"))
        .header(CONTENT_TYPE, "application/json")
        .json(&body)
        .send()
    {
        Ok(res) => {
            if !res.status().is_success() {
                println!("Post room error: {}", res.status());
            } else {
                println!("Room created: {:#?}", res.json::<Room>());
                *trigger_fetch = true;
            }
        }
        Err(err) => println!("Post room error: {}", err),
    };
}

fn fetch_rooms(http_client: &HttpClient) -> Rooms {
    match http_client
        .client
        .get(format!("{}{}", http_client.base_url, "api/v1/rooms"))
        .send()
    {
        Ok(res) => {
            if !res.status().is_success() {
                println!("Fetch rooms error: {}", res.status());
                Rooms { rooms: vec![] }
            } else {
                let rooms = res.json::<Vec<Room>>().unwrap_or_default();
                println!("{:#?}", rooms);
                Rooms { rooms }
            }
        }
        Err(err) => {
            println!("Fetch rooms error: {}", err);
            Rooms { rooms: vec![] }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn side_pane(
    ctx: &egui::Context,
    ui: &mut Ui,
    http_client: &HttpClient,
    trigger_fetch_rooms: &mut bool,
    trigger_fetch_messages: &mut bool,
    rooms: &mut Rooms,
    selected_room: &mut Room,
    chatroom_search: &mut String,
) {
    //! A component that takes up the left side of the screen.
    //! It shows user profile and all the available chatrooms with a search functionality.

    // Fetch rooms when opening app and when new room is created (TODO: fetch when deleted)
    if *trigger_fetch_rooms {
        *rooms = fetch_rooms(http_client);
        *trigger_fetch_rooms = false;
    }

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
            // TextEdit for searching for a chatroom
            ui.add(
                egui::TextEdit::singleline(chatroom_search)
                    .id_source("search_response")
                    .hint_text("Search for a chatroom")
                    .desired_width(ui.available_width())
                    .margin(Vec2 { x: 8., y: 4. }),
            );
            // Use bottom_up layout to add create chatroom functionality to the bottom
            // and leave the remaining space for chatroom scroll area
            ui.with_layout(Layout::bottom_up(Align::Center), |ui| {
                ui.add_space(28.);
                ui.horizontal(|ui| {
                    let button = ui.add_sized(
                        [ui.available_width(), 30.],
                        egui::Button::new("Create chatroom"),
                    );
                    if button.clicked() {
                        let room_name = "New room";
                        let room_public = true;
                        create_room(http_client, trigger_fetch_rooms, room_name, room_public);
                    }
                });
                ui.add_space(12.);

                let row_height = ui.spacing().interact_size.y;
                let total_rows = rooms.rooms.len();
                ui.with_layout(Layout::top_down(Align::Center), |ui| {
                    // ScrollArea to host all chatrooms as buttons
                    egui::ScrollArea::vertical()
                        .id_source("side_pane")
                        .max_width(ui.available_width())
                        .show_rows(ui, row_height, total_rows, |ui, _row_range| {
                            // Show all chatrooms and if chatroom search contains something filter case insensitively
                            for i in rooms
                                .rooms
                                .iter()
                                .filter(|x| {
                                    x.name
                                        .to_lowercase()
                                        .contains(&chatroom_search.to_lowercase())
                                })
                                .enumerate()
                            {
                                let room = i.1;
                                let button = ui.add_sized(
                                    [ui.available_width(), 30.],
                                    egui::Button::new(&room.name),
                                );
                                if button.clicked() {
                                    *selected_room = room.clone();
                                    *trigger_fetch_messages = true;
                                    println!("{:#?}", room);
                                }
                            }
                        });
                })
            });
        },
    );
}
