/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct LoginPage {
    // Example stuff:
    username: String,
    password: String,

    // this how you opt-out of serialization of a member
    //#[serde(skip)]
    //value: f32,
}

impl Default for LoginPage {
    fn default() -> Self {
        Self {
            username: "".to_owned(),
            password: "".to_owned(),
        }
    }
}

impl LoginPage {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for LoginPage {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {username, password } = self;
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("EguiValet");
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Log In");

            ui.horizontal(|ui| {
                ui.label("Username: ");
                ui.text_edit_singleline(username);
            });

            ui.horizontal(|ui| {
                ui.label("Password: ");
                ui.add(
                egui::TextEdit::singleline(password)
                    .password(true)
                    .id_source("user_message")
                );
            });

            ui.horizontal(|ui| {
                if ui.button("Log In").clicked() {
                    // Login user
                    println!("{}", username);
                    println!("{}", password);
                }
                if ui.button("Sign up").clicked() {
                    // Create user
                    println!("Change to sign up");
                }
            });

        });
    }
}
