use imgui::{Ui,ImStr,ImString};

use std::fs;
use std::path::Path;

use app::AppState;

pub struct FileState {
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
            idx: 1,
            files: vec![],
        }
    }
}

impl FileState {
    fn update(&mut self) {
        if self.idx != 1 {
            if let Some(file) = self.files.get(self.idx as usize) {
                self.cd.clear();
                self.cd.push_str(file);
                self.idx = 1;
            }
        }

        if self.idx < 1 {
            let cd = self.cd.to_string();
            let path = Path::new(&cd).parent().expect("ERROR: No Parent path");
            self.cd.clear();
            self.cd.push_str(im_str!("{:}",path.to_str().unwrap()));
            self.idx = 1;
        }
        
        if let Ok(paths) = fs::read_dir(&self.cd.to_string()) {
            self.files = paths
                .filter(|p| p.is_ok())
                .map(|p| {
                    p.unwrap().path().as_path().to_str().unwrap().to_owned()
                })
                .collect();
        }
        else {
            let cd = self.cd.to_string();
            let path = Path::new(&cd).parent().expect("ERROR: No Parent path");
            self.cd.clear();
            self.cd.push_str(im_str!("{:}",path.to_str().unwrap()));
            self.idx = 1;
        }

        self.files.insert(0,"./".to_owned());
        self.files.insert(0,"..".to_owned());
    }

    pub fn render (&mut self, ui: &Ui, state: &mut AppState) {
        if !state.open_file { return }
        
        ui.window(im_str!("Select source"))
            .always_auto_resize(true)
            .movable(true)
            .show_borders(true)
            .opened(&mut state.open_file)
            .build(||{
                ui.text(im_str!("Select a source file to be parsed and evaluated.\nSource files must be parsable through the lichen crate."));
                ui.separator();
                ui.input_text(im_str!("Current Directory"), &mut self.cd)
                    .enter_returns_true(true)
                    .build();
                ui.separator();

                self.update();

                // NOTE: to appease the borrow checker and have the proper argument for list_box
                // we must do build the list in two steps
                let paths: Vec<ImString> = self.files.iter().map(|p| {
                    im_str!("{:}", p).to_owned()
                })
                    .collect();

                let paths: Vec<&ImStr> = paths.iter().map(|p|p.as_ref()).collect();
                
                ui.list_box(im_str!("Files"),
                            &mut self.idx,
                            &paths[..],
                            paths.len() as i32);
            })
    }

}


