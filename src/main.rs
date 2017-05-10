extern crate imgui;
extern crate glium;
extern crate imgui_glium_renderer;

mod support;
use support::Support;

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.2, 0.2, 0.2, 1.0);

fn main() {
    let mut support = Support::init();

    loop {
        let mut open = true;
        support.render(CLEAR_COLOR, |ui| ui.show_test_window(&mut open));
        let active = support.update_events();
        if !active || !open {
            break;
        }
    }
}
