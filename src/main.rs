#[macro_use]
extern crate imgui;
extern crate mush;
use mush::interface::Interface;
use imgui::{Ui,ImStr};

use std::fs;

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.006, 0.006, 0.006, 1.0);

fn main() {
    let mut ifc = Interface::init();
    let mut state = State::default();
    
    loop {
        let r = ifc.render(CLEAR_COLOR, |ui| {
            menu(ui, &mut state);
            open_file(ui, &mut state);
        });
        if !r | state.exit{ break }
    }
}

struct State {
    exit: bool,
    open_file: bool,
    current_dir: String,
    current_dir_list: i32,
}
impl Default for State {
    fn default() -> Self {
        State {
            exit: false,
            open_file: true,
            current_dir: "./".to_owned(),
            current_dir_list: 0,
        }
    }
}


fn menu (ui: &Ui, state: &mut State) {
    ui.main_menu_bar(|| {
        ui.menu(im_str!("File"))
            .build(|| {
                ui.menu_item(im_str!("Open"))
                    .selected(&mut state.open_file)
                    .build();
                
                ui.menu_item(im_str!("Exit"))
                    .selected(&mut state.exit)
                    .build();
            });
    });
}

fn open_file (ui: &Ui, state: &mut State) {
    if !state.open_file { return }
    
    let mut cd = fs::read_dir(&state.current_dir);
    
    ui.window(im_str!("Select source"))
        .always_auto_resize(true)
        .movable(true)
        .show_borders(true)
        //.opened(&mut state.open_file) // FIXME:borrow check issues with closures
        .build(||{
            ui.text(im_str!("Select a source file to be parsed and evaluated.\nSource files must be parsable through the lichen crate."));
            ui.separator();
            ui.input_text(im_str!("Current Directory"), &mut state.current_dir)
                .build();
            ui.separator();

            if let Ok(ref mut paths) = cd {
                let paths: Vec<ImStr> = paths
                    .filter(|p| p.is_ok())
                    .map(|p| {
                        im_str!("{:}", p.unwrap().path().display())
                    })
                    .collect();
                
                ui.list_box(im_str!("Files"),
                            &mut state.current_dir_list,
                            &paths[..],
                            paths.len() as i32);
                
            }
        })
}
