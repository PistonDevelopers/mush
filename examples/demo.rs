extern crate mush;

use mush::{GraphNode,GraphEdge,Graph,Backend,NodeBase,EdgeGuard};
use mush::{UiNode,UiGraph,UiBase};
use mush::{ToolPane};

extern crate conrod;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

use conrod::{Background, Colorable, Theme, Ui, Positionable, WidgetId};
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event::*;
use piston::window::{ WindowSettings, Size };
use std::path::Path;


#[derive(Debug,Copy,Clone,PartialEq)]
struct MyEdge {
    factor: f64,
}
impl GraphEdge for MyEdge {
    fn default() -> MyEdge { MyEdge { factor:0.0f64, } }
}


#[derive(Debug,Copy,Clone,PartialEq)]
enum MyGuard {
    In,
    Out,
    Root,
}

#[derive(Debug,Clone,PartialEq)]
struct MyNode {
    name: String,
    position: [f64;2],
    base: NodeBase,
    uibase: UiBase,
    guard: MyGuard,
    kind: MyGuard,
}
impl GraphNode for MyNode {
    fn default() -> MyNode { MyNode { name: "".to_string(),
                                      position: [0.0,0.0],
                                      base: NodeBase::new(),
                                      uibase: UiBase::default(),
                                      guard: MyGuard::In,
                                      kind: MyGuard::Out }}

    fn get_base(&self) -> &NodeBase { &self.base }
    fn get_base_mut(&mut self) -> &mut NodeBase { &mut self.base }
    
    fn get_name(&self) -> &str { &self.name }
    fn get_position(&self) -> &[f64;2] { &self.position }

    fn set_name(&mut self, s: &str) { self.name = s.to_string() }
    fn set_position(&mut self, p: [f64;2]) { self.position = p }
}
impl UiNode for MyNode {
    fn get_ui(&self) -> &UiBase {&self.uibase}
    fn get_ui_mut(&mut self) -> &mut UiBase {&mut self.uibase}
}
impl MyNode {
    fn new(p: [f64;2], n: String, id: WidgetId) -> MyNode {
        let mut node = MyNode::default();
        node.name = n;
        node.position = p;
        node.get_ui_mut().set_id(id); //fixme: we don't want to track this manually, toolpane did this for us
        node
    }
}

// setup node-edge guards
impl EdgeGuard for MyNode {
    fn guard(&self, other: &Self) -> bool {
        match (self.guard,other.kind) {
            (MyGuard::In,MyGuard::Root) => true,
            (MyGuard::In,MyGuard::Out) => true,
            _ => false,
        }
    }
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
    let mut graph = Graph::default();
    let default_node = MyNode::default();
    let a = graph.add_node(MyNode::new([100.0, 100.0], "Stuff".to_string(),20));
    let b = graph.add_node(MyNode::new([100.0, 0.0], "Things".to_string(),25));
    let c = graph.add_node(MyNode::new([0.0, 100.0], "Whatever".to_string(),30));
    graph.direct(&a,&b, MyEdge::default());
    graph.direct(&b,&c, MyEdge::default());

    println!("{:?}", graph);

    let mut tools = ToolPane::new(&mut graph);
    //tools.on_save(|graph| println!("{:?}", graph));

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

                tools.render(&mut ui,&mut graph);
                graph.render(&mut ui);
                
                // Draw our Ui!
                ui.draw(gl);

            });
        }
    }

}
