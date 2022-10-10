use eframe::egui::{
    FontData, FontDefinitions, FontFamily,
    FontFamily::Proportional,
    FontId,
    TextStyle::{Body, Button, Heading, Monospace, Small},
};

// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ChatApp {
    // Vector with name and id
    pub chatrooms: Vec<(String, String)>,
    pub selected_chatroom: String,
    pub chatroom_search: String,
    pub message: String,
}

// impl Default for ChatApp {
//     fn default() -> Self {
//         Self {
//             chatrooms: Vec::new(),
//             selected_chatroom: "".to_owned(),
//             chatroom_search: "".to_owned(),
//             message: "".to_owned(),
//         }
//     }
// }

impl ChatApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        configure_fonts(&cc.egui_ctx);
        configure_text_styles(&cc.egui_ctx);

        // Dummy data for testing
        Self {
            chatrooms: vec![
                ("Chatroom 1".to_owned(), "id1".to_owned()),
                ("Room 2".to_owned(), "id2".to_owned()),
            ],
            selected_chatroom: "Chatroom 1".to_owned(),
            chatroom_search: "".to_owned(),
            message: "".to_owned(),
        }

        // Default::default();
    }
}

pub fn configure_fonts(ctx: &egui::Context) {
    // Create font def object
    let mut font_def = FontDefinitions::default();
    // Load up the font
    font_def.font_data.insert(
        "Raleway".to_owned(),
        FontData::from_static(include_bytes!("./fonts/Raleway-Regular.ttf")),
    ); // .ttf and .otf supported
       // Put my font first (highest priority):
    font_def
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "Raleway".to_owned());
    // Load the font using the context obejct
    font_def
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "Raleway".to_owned());
    ctx.set_fonts(font_def);
}

pub fn configure_text_styles(ctx: &egui::Context) {
    // Get current context style
    let mut style = (*ctx.style()).clone();
    // Redefine text_styles
    style.text_styles = [
        (Heading, FontId::new(30.0, Proportional)),
        (Body, FontId::new(18.0, Proportional)),
        (Monospace, FontId::new(14.0, Proportional)),
        (Button, FontId::new(20.0, Proportional)),
        (Small, FontId::new(10.0, Proportional)),
    ]
    .into();
    // Mutate global style with above changes
    ctx.set_style(style);
}