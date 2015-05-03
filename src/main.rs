extern crate mush;

use mush::{NodeContainer};

extern crate conrod;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

use conrod::{Background, Button, Toggle , Colorable, Labelable, Sizeable, Theme, Ui,
             Positionable, TextBox, CustomWidget, Position};
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event::*;
use piston::window::{ WindowSettings, Size };
use std::path::Path;

//fn resized(w:u32,h:u32) {width=w; height=h;}

fn main () {
    let mut width = 1024;
    let mut height = 768;

    let opengl = OpenGL::_3_2;
    let mut window = GlutinWindow::new(
        opengl,
        WindowSettings::new(
            "mush -> graph library gui".to_string(),
            Size { width: width, height: height }
            )
            .exit_on_esc(true)
            .samples(4)
       );

    

    let event_iter = window.events().ups(180).max_fps(60);
    let mut gl = GlGraphics::new(opengl);
    let font_path = Path::new("fonts/SourceCodePro-Regular.otf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let mut ui = &mut Ui::new(glyph_cache, theme);
    
    let mut cont = NodeContainer::new(0,[120.0,120.0]);
    let mut cont2 = NodeContainer::new(6,[400.0,200.0]); //for now trakc index manually, so no overlaps. It may be best to use a NodePane controller, which handles this state/count
    
    for event in event_iter {
        ui.handle_event(&event);
        
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |_, gl| {

                // Draw the background.
                Background::new().rgb(0.2, 0.2, 0.2).draw(ui, gl); //this swaps buffers for us

                cont.update(ui.mouse.xy);
                cont.draw(&mut ui);
                
                cont2.update(ui.mouse.xy);
                cont2.draw(&mut ui);
                
                /* mush::node::Node::new()
                .label("Thingy")
                .xy(100.0, 100.0)
                .dimensions(100.0, 40.0)
                .set(2, ui);*/
                
                // Draw our Ui!
                ui.draw(gl);
                
            });
        }
    }


}
