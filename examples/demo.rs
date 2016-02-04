extern crate mush;

use mush::{GraphNode,GraphEdge,Graph,Backend,NodeBase,EdgeGuard};
use mush::{UiNode,UiGraph,UiBase};
use mush::{ToolPane};

#[macro_use] extern crate conrod;
extern crate find_folder;
extern crate piston_window;

use conrod::{Theme, Widget,WidgetId, color, Colorable,Canvas};
use piston_window::{EventLoop, Glyphs, PistonWindow, UpdateEvent, WindowSettings};

type Ui = conrod::Ui<Glyphs>;



#[derive(Debug,Copy,Clone,PartialEq)]
struct MyEdge {
    factor: f64,
}
impl GraphEdge for MyEdge {
    fn default() -> MyEdge { MyEdge { factor:0.0f64, } }
}


#[allow(dead_code)]
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

    // Construct the window.
    let window: PistonWindow =
        WindowSettings::new("mush -> graph library gui", [1000, 600])
            .exit_on_esc(true).build().unwrap();

    // Construct our `Ui`.
    let mut ui = {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("fonts").unwrap();
        let font_path = assets.join("SourceCodePro-Regular.otf");
        let theme = Theme::default();
        let glyph_cache = Glyphs::new(&font_path, window.factory.borrow().clone());
        Ui::new(glyph_cache.unwrap(), theme)
    };

    // Initialize the graph structure
    let mut graph: Graph<MyEdge,MyNode> = Graph::default();
    let mut tools = ToolPane::new(&mut graph,"Some Project Name".to_string());

    let a = tools.new_node(&mut graph);
    let b = tools.new_node(&mut graph);
    let c = tools.new_node(&mut graph);

    graph.direct(&a,&b, MyEdge::default());
    graph.direct(&b,&c, MyEdge::default());
    graph.direct(&a,&c, MyEdge::default());
    
    // Poll events from the window.
    for event in window.ups(60) {
        ui.handle_event(&event);
        
        event.update(|_| {
            ui.set_widgets(|ui|{
                Canvas::new().color(color::DARK_CHARCOAL).set(WidgetId(0),ui);
                
                tools.render(ui,&mut graph);
                graph.render(ui);
            });
        });
            
        event.draw_2d(|c, g| {
            ui.draw(c, g);
        });
    }
}
