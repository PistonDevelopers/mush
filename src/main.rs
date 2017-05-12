extern crate imgui;

extern crate mush;
use mush::interface::Interface;
use mush::file::FileState;
use mush::app::AppState;



use std::time::{Duration,Instant};
use std::thread;

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.006, 0.006, 0.006, 1.0);

/// represents 15ms as nanos
static FPS_60: u32 = 15000000;

fn main() {
    let mut ifc = Interface::init();
    let mut app = AppState::default();
    let mut file = FileState::default();
    
    loop {
        let start_time = Instant::now();
        
        let r = ifc.render(CLEAR_COLOR, |ui| {
            app.render(ui);
            file.render(ui, &mut app);

            //ui.show_test_window(&mut true) // NOTE: use for imgui examples
        });
        if !r | app.exit{ break }

        
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
