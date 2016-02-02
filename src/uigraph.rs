use conrod::{Button,Positionable, Text, Sizeable,Widget,WidgetId,Canvas,Colorable,color,Labelable,Line,Circle};
use piston_window::Glyphs;
use ::{Backend,Graph,GraphNode,GraphEdge,Nid};

pub type Ui = ::conrod::Ui<Glyphs>;


pub trait UiNode: GraphNode {
    fn get_ui(&self) -> &UiBase;
    fn get_ui_mut(&mut self) -> &mut UiBase;
    fn build_ui(&mut self, ui: &mut Ui) -> bool {

        if self.get_ui().destroy { return false }

        let mut canvas_height = self.get_ui().height;
        let mut cl = "-"; //canvas label
        if self.get_ui().collapse {
            cl = "+";
            canvas_height = 20.0;
        }
        
        let ui_id_start: WidgetId = self.get_ui().get_id();

        let position = self.get_position().clone();

        let mut color = color::WHITE;
        if self.get_ui().select { color=color::RED; }
        
        //floating canvas where our node data resides
        Canvas::new()
            .floating(true)
            .title_bar(self.get_name())
            .xy([position[0], position[1]])
            .h(canvas_height)
            .w(self.get_ui().width)
            .color(color::BLUE)
            .label_color(color)
            .set(ui_id_start, ui);



        //build buttons in reverse order, to stay within canvas
        //start at top right, and head left from
        //close button
        Button::new()
            .top_right_of(ui_id_start) //todo: shift up ~5.0
            .label("x")
            .w_h(20.0,20.0)
            .react(|| self.get_ui_mut().destroy = true)
            .set(ui_id_start + 1, ui);

        //collapse
        Button::new()
            .left(5.0)
            .label(cl)
            .w_h(20.0,20.0)
            .react(|| self.get_ui_mut().toggle_collapse())
            .set(ui_id_start + 2, ui);

        //connector
        Button::new()
            .top_left_of(ui_id_start) //todo: shift up ~5.0
            .label("o")
            .w_h(20.0,20.0)
            .react(|| self.get_ui_mut().toggle_select())
            .set(ui_id_start + 3, ui);

        if !self.get_ui().collapse {
            Text::new("Node Data") // TODO: derive actual node data, and represent somehow
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
    width: f64,
    height: f64,
}

impl UiBase {
    pub fn default() -> UiBase {
        UiBase { ui_id: WidgetId(10000),
                 select: false,
                 collapse: false,
                 destroy: false,
                 width: 150.0,
                 height: 100.0,
        }
    }
    pub fn toggle_destroy(&mut self) { self.destroy = !self.destroy; }
    pub fn toggle_collapse(&mut self) { self.collapse = !self.collapse; }
    pub fn toggle_select(&mut self) { self.select = !self.select; }

    pub fn get_id(&self) -> WidgetId { self.ui_id }
    pub fn set_id(&mut self, id: WidgetId) { self.ui_id = id; }
}

pub trait UiGraph {
    fn render(&mut self, ui: &mut Ui);
}

impl<E:GraphEdge,N:UiNode> UiGraph for Graph<E,N> {
    fn render(&mut self, ui: &mut Ui) {
        let mut select: (Option<Nid>,Option<Nid>) = (None,None);
        let mut edges: Vec<(Nid,Vec<Nid>)> = vec!();

        self.with_nodes_mut(|n| {
            let is_select: bool = n.build_ui(ui);
            
            {
                let base = n.get_base();
                if is_select { //selected?
                    if select.0.is_some() {
                        if !select.1.is_some() {
                            select.1 = Some(base.get_id());
                        }
                    }
                    else { select.0 = Some(base.get_id()); }
                }

                edges.push((base.get_id(),base.get_edges()));
            }

            if let Some(coord) = ui.xy_of(n.get_ui().get_id()) {
                n.set_position(coord);
            }
        });

        // build edges
        for (j,&(ref nid,ref ev)) in edges.iter().enumerate() {
            let n = self.get_node(&nid).unwrap();
            let id = n.get_ui().get_id().0 * 20; //allot 20 spaces per node
            let id = WidgetId(id + 1000); // place in 1k range
            let mut pos = *n.get_position();
            pos[1] = pos[1] + j as f64+12.;
            Circle::fill_with(10.,color::LIGHT_BLUE)
                .xy(pos)
                .set(id+1, ui);
            
            for en in ev.iter() {
                
                if let Some(n2) = self.get_node(&en) {
                    Line::abs(*n.get_position(), *n2.get_position())
                        .set(id+2, ui);
                    Circle::fill_with(10.,color::ORANGE)
                        .xy(*n2.get_position())
                        .set(id+3, ui);
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
