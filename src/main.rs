extern crate mush;
use mush::interface::Interface;

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.2, 0.2, 0.2, 1.0);

fn main() {
    let mut ifc = Interface::init();

    loop {
        let r = ifc.render(CLEAR_COLOR, |ui| {} );
        if !r { break }
    }
}
