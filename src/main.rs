extern crate mush;

use mush::{ToolPane};

extern crate conrod;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

use conrod::{Background, Button, Colorable, Labelable, Sizeable, Theme, Ui,
             Positionable, TextBox, CustomWidget};
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event::*;
use piston::window::{ WindowSettings, Size };
use std::path::Path;

//fn resized(w:u32,h:u32) {width=w; height=h;}

fn main () {
    mush::graphs();

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

   // window.window.set_window_resize_callback(Some(resized as fn(u32,u32)));

    let event_iter = window.events().ups(180).max_fps(60);
    let mut gl = GlGraphics::new(opengl);
    let font_path = Path::new("fonts/SourceCodePro-Regular.otf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let ui = &mut Ui::new(glyph_cache, theme);

    let mut count: u32 = 0;


    for event in event_iter {
        ui.handle_event(&event);
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |_, gl| {

                // Draw the background.
                Background::new().rgb(0.2, 0.2, 0.2).draw(ui, gl);

                TextBox::new(&mut "Node".to_string())
                    .dimensions(100.0,60.0)
                    .xy(width as f64/2.0*-1.0+100.0,0.0)
                    .react(|_s: &mut String|{println!("{:?}",_s)})
                    .set(0,ui);


                // Draw the button and increment count if pressed..
                Button::new()
                    .dimensions(80.0, 40.0)
                    .label(&args.width.to_string())
                    .rgba(0.9,0.9,0.9,0.8)
                    .right(10.0)
                    .react(|| {})
                    .set(1, ui);

                mush::node::Node::new()
                    .label("Thingy")
                    .xy(100.0, 100.0)
                    .dimensions(100.0, 40.0)
                    .set(2, ui);

                // Draw our Ui!
                ui.draw(gl);

            });
        }
    }


}
