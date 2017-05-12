use imgui::{Ui};
use lichen::parse::Env;

pub struct AppState {
    pub exit: bool,
    pub open_file: bool,
    pub env: Option<Env>,
}
impl Default for AppState {
    fn default() -> Self {
        AppState {
            exit: false,
            open_file: true,
            env: None,
        }
    }
}

impl AppState {
    pub fn render (&mut self, ui: &Ui) {
        ui.main_menu_bar(|| {
            ui.menu(im_str!("File"))
                .build(|| {
                    ui.menu_item(im_str!("Open"))
                        .selected(&mut self.open_file)
                        .build();
                    
                    ui.menu_item(im_str!("Exit"))
                        .selected(&mut self.exit)
                        .build();
                });
        });

        if let Some(ref env) = self.env {
            ui.window(im_str!("Environment Parsed"))
                .build(||{
                    for def in env.def.keys() {
                        ui.text(im_str!("{:?}",def));
                    }

                    for src in env.src.keys() {
                        ui.text(im_str!("{:?}",src));
                    }
                });
        }
    }
}
