#[macro_use]
extern crate imgui;
extern crate mush;
use mush::interface::Interface;
use imgui::{Ui,ImStr,ImString};

use std::fs;
//use std::path::Path;

use std::time::{Duration,Instant};
use std::thread;

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.006, 0.006, 0.006, 1.0);

/// represents 15ms as nanos
static FPS_60: u32 = 15000000;

fn main() {
    let mut ifc = Interface::init();
    let mut state = State::default();
    let mut filestate = FileState::default();
    
    loop {
        let start_time = Instant::now();
        
        let r = ifc.render(CLEAR_COLOR, |ui| {
            menu(ui, &mut state);
            open_file(ui, &mut state, &mut filestate);
        });
        if !r | state.exit{ break }

        
        maybe_sleep(start_time); // we must manage sleep if we run too fast
    }
}

fn maybe_sleep(start_time: Instant) {
    let duration = Duration::new(0,FPS_60);
    if start_time.elapsed() < duration {
        let delay = duration - start_time.elapsed();
        thread::sleep(delay);
    }
}

struct State {
    exit: bool,
    open_file: bool,
}
impl Default for State {
    fn default() -> Self {
        State {
            exit: false,
            open_file: true,
        }
    }
}

struct FileState {
    /// MAX_PATH set to 260 chars
    cd: ImString,
    idx: i32,
    files: Vec<String>,
}

impl Default for FileState {
    fn default() -> Self {
        let mut cd = ImString::from(im_str!("./"));
        cd.reserve(261); //arbitrary max path size
        
        FileState {
            cd: cd,
            idx: 0,
            files: vec![],
        }
    }
}

impl FileState {
    fn update(&mut self) {
        if let Ok(paths) = fs::read_dir(&self.cd.to_string()) {
            self.files = paths
                .filter(|p| p.is_ok())
                .map(|p| {
                    p.unwrap().path().as_path().to_str().unwrap().to_owned()
                })
                .collect();
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

fn open_file (ui: &Ui, state: &mut State, filestate: &mut FileState) {
    if !state.open_file { return }
    
    ui.window(im_str!("Select source"))
        .always_auto_resize(true)
        .movable(true)
        .show_borders(true)
        .opened(&mut state.open_file)
        .build(||{
            ui.text(im_str!("Select a source file to be parsed and evaluated.\nSource files must be parsable through the lichen crate."));
            ui.separator();
            ui.input_text(im_str!("Current Directory"), &mut filestate.cd)
                .enter_returns_true(true)
                .build();
            ui.separator();

            filestate.update();

            // NOTE: to appease the borrow checker and have the proper argument for list_box
            // we must do build the list in two steps
            let paths: Vec<ImString> = filestate.files.iter().map(|p| {
                    im_str!("{:}", p).to_owned()
                })
                .collect();

            let paths: Vec<&ImStr> = paths.iter().map(|p|p.as_ref()).collect();
                
            ui.list_box(im_str!("Files"),
                        &mut filestate.idx,
                        &paths[..],
                        paths.len() as i32);
        })
}
