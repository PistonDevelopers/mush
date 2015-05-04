extern crate conrod;
extern crate elmesque;
extern crate opengl_graphics;
extern crate petgraph;

pub use toolpane::{ToolPane};
pub use graph::{graphs};

pub mod toolpane;
pub mod node;
pub mod graph;

pub trait EditableNode {
    fn get_position(&self) -> [f64; 2];
    fn set_position(&mut self, [f64; 2]);
    fn default() -> Self;
}

pub trait EditableEdge {
    fn default() -> Self;
}

// note: this should probably be moved to another file
// perhaps container.rs? where node.rs would contain node-graph specifics?
use conrod::{Ui,Label,Button,Positionable,Sizeable,Labelable,Frameable};
use opengl_graphics::glyph_cache::GlyphCache;
use petgraph::graph::{NodeIndex};
use petgraph::{Graph};
// we have to track actions outside of a conrod widget, from what I understand, for us to manipulate widget to widget
// hence the nodecontainer, which lays out the node object visually, and controls its appearance
pub struct NodeContainer {
    xy: [f64;2],
    drag: bool,
    collapse: bool,
    destroy: bool,
    sidx: usize, //start index
    nidx: NodeIndex, //index to node in petgraph
}

impl NodeContainer {
    pub fn new(sidx: usize, xy:[f64;2], nidx: NodeIndex) -> NodeContainer {
        NodeContainer {
            xy: xy,
            drag: false,
            collapse: false,
            destroy: false,
            // we need a starting index so conrod is happy, it'd make more sense if conrod returned an index when creating the widgets; instead we'll track manually
            sidx: sidx,
            nidx: nidx,
        }
    }

    pub fn draw<N, E>(&mut self, ui: &mut Ui<GlyphCache>, graph: &mut Graph<N, E>)
        where N: EditableNode, E: EditableEdge
    {
        if self.destroy { return }

        let idx = self.sidx;
        Button::new() //this should be a press-action, not a release; fixme with custom widget! also conrod should have a drag controller widget, which is basically what we're building
            .xy(self.xy[0], self.xy[1])
            .dimensions(100.0,20.0)
            .react(|| { self.drag = !self.drag; })
            .set(idx,ui);

        let mut cl = "<";
        if self.collapse { cl = ">"; }

        Button::new()
            .right(5.0)
            .label(cl)
            .dimensions(20.0,20.0)
            .react(|| { self.collapse = !self.collapse; })
            .set(idx+1,ui);

        Button::new()
            .right(5.0)
            .label("x")
            .dimensions(20.0,20.0)
            .react(|| {
                graph.remove_node(self.nidx);
                self.destroy=true;
            })
            .set(idx+2,ui);

        if !self.collapse {
            Label::new(&self.nidx.index().to_string())
                .down_from(idx, 5.0)
                .set(idx+3,ui);
        }
    }

    pub fn update<N, E>(&mut self, xy: [f64;2], graph: &mut Graph<N, E>)
        where N: EditableNode, E: EditableEdge
    {
        // check for destroy until we formally remove nodecontainer
        if !self.destroy && self.drag {
            self.xy = xy;
            graph[self.nidx].set_position(xy);
        }
    }
}
