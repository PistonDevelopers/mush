use imgui::{Ui,ImStr,ImString};
use lichen::parse::{StreamParser,Env};

use std::fs;
use std::path::Path;

use app::AppState;

use std::io::{BufReader};
use std::fs::File;



pub struct FileState {
    /// MAX_PATH set to 260 chars
    cd: ImString,
    idx: i32,
    files: Vec<String>,
    selected: Option<String>,
    stream: Option<StreamParser<BufReader<File>>>,
    failed: String,
}

impl Default for FileState {
    fn default() -> Self {
        let mut cd = ImString::from(im_str!("./"));
        cd.reserve(261); //arbitrary max path size
        
        FileState {
            cd: cd,
            idx: 1,
            files: vec![],
            selected: None,
            stream: None,
            failed: "".to_owned(),
        }
    }
}

impl FileState {
    fn update(&mut self) -> Option<Env> {
        if self.idx > 1 { // chose a file?
            self.failed.clear();
            
            if let Some(file) = self.files.get(self.idx as usize) {
                self.cd.clear();
                self.cd.push_str(file);
                self.idx = 1;
            }
        }
        else if self.idx < 1 { // up a directory?
            self.failed.clear();
            self.to_parent();
        }

        let cd = self.cd.to_string();
        if Path::new(&cd).is_dir() {
            if let Ok(paths) = fs::read_dir(&cd) {
                self.files = paths
                    .filter(|p| p.is_ok())
                    .map(|p| {
                        p.unwrap().path().as_path().to_str().unwrap().to_owned()
                    })
                    .collect();

                self.files.insert(0,"./".to_owned());
                self.files.insert(0,"..".to_owned());
            }
            // NOTE: we should throw a warning here if directory cannot be traversed
        }
        else {
            if !cd.is_empty() {
                self.selected = Some(cd);
            }
        }

        
        if self.stream.is_some() {
            let r = self.step_stream();
            if !self.failed.is_empty() {
                self.stream = None;
            }
            r
        }
        else { None }
    }

    /// Steps through the stream chunks, finally returns parsed lichen environment
    fn step_stream (&mut self) -> Option<Env> {
        let r = {
            if let Some(ref mut stream) = self.stream {
                stream.next()
            }
            else { None }
        };
        
        if r.is_none() { // close stream, we're probably done
            let mut env = Env::empty();
            if let Some(ref mut stream) = self.stream {
                if let Err(e) = stream.sink(&mut env) {
                    self.failed = e.to_owned();
                    return None;
                }
            }
            
            self.stream = None;
            self.to_parent();
            
            return Some(env);
        }

        None
    }

    fn to_parent(&mut self) {
        self.selected = None;
        if let Some(path) = Path::new(&self.cd.to_string()).parent() {
            self.cd.clear();
            self.cd.push_str(path.to_str().expect("ERROR: Cannot parse path into string"));
            self.idx = 1;
        }
        // NOTE: this fails if we're at the top of a relative directory listing,
        // or the root of the drive/share
    }

    
    pub fn render (&mut self, ui: &Ui, state: &mut AppState) {
        if !state.open_file { return }
        state.env = self.update(); // NOTE: we should check if the environment is empty
        state.open_file = !state.env.is_some(); // close file if done

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

                // NOTE: to appease the borrow checker and have the proper argument for list_box
                // we must build the list in two steps
                let paths: Vec<ImString> = self.files.iter().map(|p| {
                    im_str!("{:}", p).to_owned()
                }).collect();

                let paths: Vec<&ImStr> = paths.iter().map(|p|p.as_ref()).collect();
                
                ui.list_box(im_str!("Files"),
                            &mut self.idx,
                            &paths[..],
                            paths.len() as i32);

                if let Some(ref file) = self.selected {
                    ui.text(im_str!("Select File {:}?",file));
                    
                    if ui.small_button(im_str!("open")) {
                        if let Ok(f) = File::open(file) {
                            let f = BufReader::new(f);
                            let s = StreamParser::new(f,None);
                            self.stream = Some(s);
                        }
                    }
                }

                if !self.failed.is_empty() {
                    ui.text(im_str!("Failed to parse file {:}\nParse Error at: {:}",
                                    &self.cd.to_str(), &self.failed));
                }
            })
    }

}


