use conrod::{Button, Positionable, Sizeable, Labelable,Widget,WidgetId,color};
use conrod::{Colorable,Canvas};

use ::{Backend,GraphNode,GraphEdge,UiNode};

use piston_window::Glyphs;
pub type Ui = ::conrod::Ui<Glyphs>;

pub struct ToolPane {
    name: String,
    next_widget_id: usize,
    //maybe_on_save: Option<F>
}

impl ToolPane {

    pub fn new<N:UiNode,G:Backend<Node=N>>(graph: &mut G,name:String) -> ToolPane
    {
        //let's allocate 10 widget ids per node plus 10 connects
        let connects = 10;
        let mut count = 10+connects;
        
        graph.with_nodes_mut(|n| {
            count += 10+connects;
            n.get_ui_mut().set_id(WidgetId(count)); 
        });
        ToolPane {
            name: name,
            next_widget_id: count,
           // maybe_on_save: None,
        }
    }

    //pub fn on_save(&mut self, on_save: F) {
  //      self.maybe_on_save = Some(on_save);
   // }

    pub fn render<N:UiNode,G:Backend<Node=N>>(&mut self, ui: &mut Ui, graph: &mut G)  {
        let header_h = 80.;
        Canvas::new()
            .floating(true)
            .title_bar(&self.name) // TODO: derive this for projects
            .xy([0.,(ui.win_h/2.) - (header_h/2.)])
            .h(header_h)
            .color(color::DARK_GRAY)
            .label_color(color::DARK_CHARCOAL)
            .set(WidgetId(1), ui);
        
        Button::new()
            .top_left_of(WidgetId(1))
            .label("Save")
            .w_h(100.0, 40.0)
            .label_color(color::LIGHT_GRAY) //light grey, does nothing right now
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
