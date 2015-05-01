extern crate mush;

extern crate conrod;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

use conrod::{Background, Button, Colorable, Labelable, Sizeable, Theme, Ui};
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event::*;
use piston::window::{ WindowSettings, Size };
use std::path::Path;


fn main () {
    mush::graphs();

    
    let opengl = OpenGL::_3_2;
    let window = GlutinWindow::new(
        opengl,
        WindowSettings::new(
            "mush -> graph library gui".to_string(),
            Size { width: 1024, height: 768 }
        )
        .exit_on_esc(true)
        .samples(4)
    );
    let event_iter = window.events().ups(180).max_fps(60);
    let mut gl = GlGraphics::new(opengl);
    let font_path = Path::new("../fonts/SourceCodePro-Regular.otf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let ui = &mut Ui::new(glyph_cache, theme);

    let mut count: u32 = 0;

    for event in event_iter {
        ui.handle_event(&event);
        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |_, gl| {

                // Draw the background.
                Background::new().rgb(0.2, 0.25, 0.4).draw(ui, gl);

                // Draw the button and increment count if pressed..
                Button::new()
                    .dimensions(80.0, 80.0)
                    .label(&count.to_string())
                    .react(|| count += 1)
                    .set(0, ui);

                // Draw our Ui!
                ui.draw(gl);

            });
        }
    }

}
