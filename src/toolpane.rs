extern crate petgraph;

use ::EditableNode;
use ::EditableEdge;
use ::NodeContainer;
use opengl_graphics::glyph_cache::GlyphCache;
use conrod::{Ui,Label,Button,Positionable,Sizeable,Labelable};
use self::petgraph::{Graph};
use petgraph::graph::{NodeIndex};

// todo: consider generalizing nodecontainer to just a container, and use the methods here as well
pub struct ToolPane {
    noffset: usize, // offset for nodecontainer widgets, depends on widget count
    nodes: Vec<NodeContainer>,
}

impl ToolPane {

    pub fn new<N, E>(offset:usize, source_graph: &Graph<N, E>) -> ToolPane
        where N: EditableNode, E: EditableEdge
    {
        let mut tool_pane = ToolPane {
            noffset: offset,
            nodes: vec!()
        };

        for (i, node) in source_graph.raw_nodes().iter().enumerate() {
            let position = node.weight.get_position(); // data stored in 'weight'
            let nuid = (tool_pane.nodes.len() + 2) * tool_pane.noffset;
            let node_index = NodeIndex::new(i);
            tool_pane.nodes.push(NodeContainer::new(nuid, position, node_index));
        }

        tool_pane
    }

    // TODO - 'export' internal graph in original source graph format

    pub fn draw<N, E>(&mut self, ui: &mut Ui<GlyphCache>, graph: &mut Graph<N, E>)
        where N: EditableNode, E: EditableEdge
    {
        // we should use a canvas to place this appropriately
        Button::new()
            .xy(-1.0*ui.win_w/2.0+50.0,ui.win_h/2.0-20.0)
            .label("New Node")
            .dimensions(100.0,40.0)
            .react(|| {
                let node = N::default();
                let position = node.get_position();
                let node_index = graph.add_node(node);
                let nuid = (self.nodes.len() + 2) * self.noffset;
                self.nodes.push(NodeContainer::new(nuid, position, node_index));
            })
            .set(0,ui);

        let xy = ui.mouse.xy;
        for n in self.nodes.iter_mut() {
            n.update(xy, graph);
            n.draw(ui,graph);
        }
    }
}
