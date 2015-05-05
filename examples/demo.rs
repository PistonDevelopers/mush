extern crate mush;

use mush::{ToolPane, EditableNode, EditableEdge};

extern crate conrod;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

use conrod::{Background, Colorable, Theme, Ui, Positionable};
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event::*;
use piston::window::{ WindowSettings, Size };
use std::path::Path;

extern crate petgraph;
use self::petgraph::{Graph};

#[derive(Debug, Clone)]
struct DemoNode {
    position: [f64; 2]
}

impl EditableNode for DemoNode {
    fn get_position(&self) -> [f64; 2] {
        self.position
    }

    fn set_position(&mut self, position: [f64; 2]) {
        self.position = position;
    }

    fn default() -> Self {
        DemoNode { position: [0.0, 0.0] }
    }
}

#[derive(Debug, Clone)]
struct DemoEdge;

impl EditableEdge for DemoEdge {
    fn default() -> Self { DemoEdge }
}

fn main () {

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

    // Initialize the graph structure
    let mut graph = Graph::new();

    let a = graph.add_node(DemoNode { position: [100.0, 100.0] });
    let b = graph.add_node(DemoNode { position: [100.0, 0.0] });
    let c = graph.add_node(DemoNode { position: [0.0, 100.0] });
    graph.add_edge(a,b, DemoEdge::default());
    graph.add_edge(b,c, DemoEdge::default());

    // Let ui graph allocate UiIds starting at 100. Not sure if this is a good idea..
    let ui_id_offset = 100;
    let mut tools = ToolPane::new(ui_id_offset, &graph);

    let event_iter = window.events().ups(180).max_fps(60);
    let mut gl = GlGraphics::new(opengl);
    let font_path = Path::new("fonts/SourceCodePro-Regular.otf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let mut ui = &mut Ui::new(glyph_cache, theme);

    for event in event_iter {
        ui.handle_event(&event);

        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |_, gl| {

                // Draw the background.
                Background::new().rgb(0.2, 0.2, 0.2).draw(ui, gl); //this swaps buffers for us

                tools.build_ui(&mut ui);

                // Draw our Ui!
                ui.draw(gl);

            });
        }
    }

}
