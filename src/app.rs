use imgui::{Ui};

pub struct AppState {
    pub exit: bool,
    pub open_file: bool,
}
impl Default for AppState {
    fn default() -> Self {
        AppState {
            exit: false,
            open_file: true,
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
    }
}
