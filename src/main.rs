#[macro_use]
extern crate imgui;
extern crate mush;
use mush::interface::Interface;
use imgui::Ui;

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.006, 0.006, 0.006, 1.0);

fn main() {
    let mut ifc = Interface::init();
    let mut state = State::default();
    
    loop {
        let r = ifc.render(CLEAR_COLOR, |ui| { menu(ui, &mut state) } );
        if !r | state.exit{ break }
    }
}

struct State {
    exit: bool,
}
impl Default for State {
    fn default() -> Self {
        State {
            exit: false,
        }
    }
}


fn menu (ui: &Ui, state: &mut State) {
    let mut opening = false;
    
    ui.main_menu_bar(|| {
        ui.menu(im_str!("File"))
            .build(|| {
                ui.menu_item(im_str!("Open"))
                    .selected(&mut opening)
                    .build();
                
                ui.menu_item(im_str!("Exit"))
                    .selected(&mut state.exit)
                    .build();
            });
    });

    if opening { ui.open_popup(im_str!("select-file")); }
    ui.popup(im_str!("select-file"), || {
        ui.text(im_str!("Select File"));
        ui.separator();
        
    });
}
