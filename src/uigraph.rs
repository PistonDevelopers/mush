extern crate elmesque;

use conrod::{Ui, UiId, Button, Label,Position,Positionable, Labelable, Sizeable,Widget,WidgetId, UserInput, Floating,Colorable};
use conrod::color::{blue, light_orange, orange, dark_orange, red, white,light_blue};

use opengl_graphics::glyph_cache::GlyphCache;

use ::{Backend,Graph,GraphNode,GraphEdge,NodeBase,EdgeGuard,Nid,Eid};
use ::edge::UiEdge;
use elmesque::{Form, Renderer};
use elmesque::form::{traced,solid,point_path}; //circle, group, ngon, oval, point_path, rect, solid, text, traced};

pub trait UiNode: GraphNode {
    fn get_ui(&self) -> &UiBase;
    fn get_ui_mut(&mut self) -> &mut UiBase;
    fn build_ui(&mut self, ui: &mut Ui<GlyphCache>) -> bool {

        if self.get_ui().destroy { return false }

        let mut canvas_height = 100.0;
        let mut cl = "<"; //canvas label
        if self.get_ui().collapse { cl = ">";
                           canvas_height = 10.0; }
        
        let ui_id_start: WidgetId = self.get_ui().ui_id;

        let position = self.get_position().clone();

        let mut color = white();
        if self.get_ui().select { color=red(); }
        
        //floating canvas where our node data resides
        Floating::new()
            .label(self.get_name())
            .xy(position[0], position[1])
            .height(canvas_height) //I think floating canvas is missing dynamic dimensions, so this only works the once and cache is then set
            .color(blue())
            .label_color(color)
            .set(ui_id_start, ui);



        //build buttons in reverse order, to stay within canvas
        //start at top right, and head left from
        //close button
        Button::new()
            .top_right_of(ui_id_start) //todo: shift up ~5.0
            .label("x")
            .dimensions(20.0,20.0)
            .react(|| self.get_ui_mut().destroy = true)
            .set(ui_id_start + 1, ui);

        //collapse
        Button::new()
            .left(5.0)
            .label(cl)
            .dimensions(20.0,20.0)
            .react(|| self.get_ui_mut().toggle_collapse())
            .set(ui_id_start + 2, ui);

        //connector
        Button::new()
            .top_left_of(ui_id_start) //todo: shift up ~5.0
            .label("o")
            .dimensions(20.0,20.0)
            .react(|| self.get_ui_mut().toggle_select())
            .set(ui_id_start + 3, ui);

        //todo: collapse floating canvas above!
        if !self.get_ui().collapse {
            Label::new("Node Data")
                .middle_of(ui_id_start)
                .set(ui_id_start + 4, ui);
        }

        self.get_ui().select
    }
}

#[derive(Debug,Clone,PartialEq)]
pub struct UiBase {
    ui_id: WidgetId,
    select: bool,
    collapse: bool,
    destroy: bool,
}

impl UiBase {
    pub fn default() -> UiBase {
        UiBase { ui_id: 0,
                 select: false,
                 collapse: false,
                 destroy: false,
        }
    }
    pub fn toggle_destroy(&mut self) { self.destroy = !self.destroy; }
    pub fn toggle_collapse(&mut self) { self.collapse = !self.collapse; }
    pub fn toggle_select(&mut self) { self.select = !self.select; }

    pub fn get_id(&self) -> WidgetId { self.ui_id }
    pub fn set_id(&mut self, id: WidgetId) { self.ui_id = id; }
}

pub trait UiGraph {
    fn render(&mut self, ui: &mut Ui<GlyphCache>);
}

impl<E:GraphEdge,N:UiNode> UiGraph for Graph<E,N> {
    fn render(&mut self, ui: &mut Ui<GlyphCache>) {
        let mut select: (Option<Nid>,Option<Nid>) = (None,None);
        let mut edges: Vec<(Nid,Vec<Nid>)> = vec!();
        
        self.with_nodes_mut(|n| {
            let is_select: bool = n.build_ui(ui);
            if is_select { //selected?
                if select.0.is_some() {
                    if !select.1.is_some() {
                        select.1 = Some(n.get_base().get_id());
                    }
                }
                else { select.0 = Some(n.get_base().get_id()); }
            }

            edges.push((n.get_base().get_id(),n.get_base().get_edges()));
        });

        // build edges
        for (nid,ev) in edges {
            let n = self.get_node(&nid).unwrap();
            for en in ev.iter() {
                let p = n.get_position();
                let id = n.get_ui().get_id();
                if let Some(n2) = self.get_node(&en) {
                    let p2 = n2.get_position();
                    UiEdge::new("edge",
                                Position::Absolute(p[0],p[1]),
                                Position::Absolute(p2[0],p2[1]))
                        .set(id +100, ui);
                }
            }
        }

        // build new edge
        match select {
            (Some(first),Some(second)) => {
                //clear node selection 
                self.get_node_mut(&first).unwrap().get_ui_mut().select = false;
                self.get_node_mut(&second).unwrap().get_ui_mut().select = false;
                
                //todo: prompt for edge data!
                self.direct(&first,&second,E::default());
            },
            _ => (),
        }
    }
}
