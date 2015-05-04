extern crate petgraph;

use ::NodeContainer;
use opengl_graphics::glyph_cache::GlyphCache;
use conrod::{Ui,Label,Button,Positionable,Sizeable,Labelable};
use self::petgraph::{Graph};

// todo: consider generalizing nodecontainer to just a container, and use the methods here as well
pub struct ToolPane {
    noffset: usize, // offset for nodecontainer widgets, depends on widget count
    nodes: Vec<NodeContainer>,
}

impl ToolPane {
    pub fn new (offset:usize) -> ToolPane {
        ToolPane { noffset: offset,
                   nodes: vec!(),
        }
    }
    
    pub fn draw(&mut self, ui: &mut Ui<GlyphCache>, graph: &mut Graph<bool,bool>) {
        
        // we should use a canvas to place this appropriately
        Button::new()
            .xy(-1.0*ui.win_w/2.0+50.0,ui.win_h/2.0-20.0)
            .label("New Node")
            .dimensions(100.0,40.0)
            .react(|| {
                let n = graph.add_node(true);
                let nuid = (self.nodes.len() + 2) * self.noffset;
                self.nodes.push(NodeContainer::new(nuid,[0.0,0.0],n));
            })
            .set(0,ui);

        let xy = ui.mouse.xy;
        for n in self.nodes.iter_mut() {
            n.update(xy);
            n.draw(ui,graph);
        }
    }
}
