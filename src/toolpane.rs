use conrod::{Button, Positionable, Sizeable, Labelable,Widget,WidgetId,color};
use conrod::{Colorable, WidgetMatrix, Floating,Canvas};

use ::{Graph,Backend,GraphNode,GraphEdge,UiNode,UiGraph};

use piston_window::Glyphs;
pub type Ui = ::conrod::Ui<Glyphs>;

pub struct ToolPane {
    next_widget_id: usize,
    //maybe_on_save: Option<F>
}

impl ToolPane {

    pub fn new<N:UiNode,G:Backend<Node=N>>(graph: &mut G) -> ToolPane
    {
        let mut count = 10;
        graph.with_nodes_mut(|n| {
            count += 10;
            n.get_ui_mut().set_id(WidgetId(count)); //let's allocate 10 widget ids per node
        });
        ToolPane {
            next_widget_id: count,
           // maybe_on_save: None,
        }
    }

    //pub fn on_save(&mut self, on_save: F) {
  //      self.maybe_on_save = Some(on_save);
   // }

    pub fn render<N:UiNode,G:Backend<Node=N>>(&mut self, ui: &mut Ui, graph: &mut G)  {
        Canvas::new().flow_right(&[
            (WidgetId(2),Canvas::new().color(color::DARK_GRAY)),
               ]).set(WidgetId(1),ui);
        
        // we should use a canvas to place this appropriately
        Button::new()
            .top_left_of(WidgetId(2))
            .label("Save")
            .w_h(100.0, 40.0)
            .react(|| {})
            .set(WidgetId(3), ui);

        Button::new()
            .right(5.0)
            .label("New Node")
            .w_h(100.0, 40.0)
            .react(|| {
                let mut n = N::default();
                self.next_widget_id += 10;
                n.get_ui_mut().set_id(WidgetId(self.next_widget_id));
                
                graph.add_node(n);
            })
            .set(WidgetId(4), ui);
     
    }

  //  pub fn save(&self) where F: Fn(Graph<N, E>) {
        /*if let Some(ref on_save) = self.maybe_on_save {
            on_save(self.ui_graph.as_source_graph());
        }*/
  //  }
}
