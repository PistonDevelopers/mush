use conrod::{Ui, UiId, Button, Positionable, Sizeable, Labelable,Widget,WidgetId};
use conrod::color::{blue, light_grey, orange, dark_grey, red, white};
use conrod::{Colorable, Label, Split, WidgetMatrix, Floating};

use opengl_graphics::glyph_cache::GlyphCache;
use std::mem;

use ::{Graph,Backend,GraphNode,GraphEdge,UiNode,UiGraph};

pub struct ToolPane {
    next_widget_id: usize,
    //maybe_on_save: Option<F>
}

impl ToolPane {

    pub fn new<N:UiNode,G:Backend<Node=N>>(graph: &mut G) -> ToolPane
    {
        let mut count = 0;
        graph.with_nodes_mut(|n| {
            count += 10;
            n.get_ui_mut().set_id(count); //let's allocate 10 widget ids per node
        });
        ToolPane {
            next_widget_id: count,
           // maybe_on_save: None,
        }
    }

    //pub fn on_save(&mut self, on_save: F) {
  //      self.maybe_on_save = Some(on_save);
   // }

    pub fn render<N:UiNode,G:Backend<Node=N>>(&mut self, ui: &mut Ui<GlyphCache>, graph: &mut G)  {

        // Construct our Canvas tree.
        Split::new(1).flow_right(&[
            Split::new(2).color(dark_grey())
                ]).set(ui);
        
        // we should use a canvas to place this appropriately
        Button::new()
            .top_left_of(2)
            .label("Save")
            .dimensions(100.0, 40.0)
            .react(|| {})
            .set(3, ui);

        Button::new()
            .right(5.0)
            .label("New Node")
            .dimensions(100.0, 40.0)
            .react(|| {
                let mut n = N::default();
                self.next_widget_id += 10;
                n.get_ui_mut().set_id(self.next_widget_id);
                
                graph.add_node(n);
            })
            .set(4, ui);
     
    }

  //  pub fn save(&self) where F: Fn(Graph<N, E>) {
        /*if let Some(ref on_save) = self.maybe_on_save {
            on_save(self.ui_graph.as_source_graph());
        }*/
  //  }
}
