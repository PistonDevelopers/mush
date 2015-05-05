use conrod::{Ui, UiId, Button, Label, Positionable, Labelable, Sizeable};
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use opengl_graphics::glyph_cache::GlyphCache;

pub trait EditableNode: Clone {
    fn get_position(&self) -> [f64; 2];
    fn set_position(&mut self, [f64; 2]);
    fn default() -> Self;
}

pub trait EditableEdge: Clone {
    fn default() -> Self;
}

pub struct UiNode<N: EditableNode> {
    source_node: N,
    ui_id: UiId,

    // NOTE - These state variables probably eventually belong in a conrod widget state..
    drag: bool,
    collapse: bool,
    destroy: bool,
}

impl<N: EditableNode> UiNode<N> {
    pub fn build_ui(&mut self, ui: &mut Ui<GlyphCache>) {

        if self.destroy { return }

        if self.drag {
            self.source_node.set_position(ui.mouse.xy);
        }

        let ui_id_start = self.ui_id;

        let position = self.source_node.get_position();

        Button::new() //this should be a press-action, not a release; fixme with custom widget! also conrod should have a drag controller widget, which is basically what we're building
            .xy(position[0], position[1])
            .dimensions(100.0,20.0)
            .react(|| self.drag = !self.drag)
            .set(ui_id_start, ui);

        let mut cl = "<";
        if self.collapse { cl = ">"; }

        Button::new()
            .right(5.0)
            .label(cl)
            .dimensions(20.0,20.0)
            .react(|| self.collapse = !self.collapse)
            .set(ui_id_start + 1, ui);

        Button::new()
            .right(5.0)
            .label("x")
            .dimensions(20.0,20.0)
            .react(|| self.destroy = true)
            .set(ui_id_start + 2, ui);

        if !self.collapse {
            Label::new("STUFF")
                .down_from(ui_id_start + 2, 5.0)
                .set(ui_id_start + 3, ui);
        }
    }
}

pub struct UiEdge<E> {
    source_edge: E,
    ui_id: UiId,
}

pub struct UiGraph<N: EditableNode, E: EditableEdge> {
    graph: Graph<UiNode<N>, UiEdge<E>>,
    ui_id_range: [UiId; 2],
}

impl<N: EditableNode, E: EditableEdge> UiGraph<N, E> {

    /// Initialize a UiGraph from a source graph structure
    pub fn new(source_graph: &Graph<N, E>, start_index: UiId) -> UiGraph<N, E> {

        let mut graph = Graph::new();

        let mut ui_id = start_index;

        // Add nodes from source graph to ui graph

        for node in source_graph.raw_nodes().iter() {
            graph.add_node(UiNode {
                source_node: node.weight.clone(),
                ui_id: ui_id,
                drag: false,
                collapse: false,
                destroy: false,
            });
            ui_id += 4;
        }

        // Node indices should be consistent between source graph and ui graph (as long as we don't remove nodes)
        // Add edges from source graph to ui graph

        for edge in source_graph.raw_edges().iter() {
            graph.add_edge(edge.source(), edge.target(), UiEdge {
                source_edge: edge.weight.clone(),
                ui_id: ui_id,
            });
            ui_id += 1;
        }

        UiGraph {
            graph: graph,
            ui_id_range: [start_index, ui_id]
        }
    }

    /// Add a node to the graph, adjusting the allocated UiId range
    pub fn add_node(&mut self) {
        let ui_id_range = self.ui_id_range;
        self.graph.add_node(UiNode {
            source_node: N::default(),
            ui_id: ui_id_range[1],
            drag: false,
            collapse: false,
            destroy: false,
        });
        self.ui_id_range = [ui_id_range[0], ui_id_range[1] + 4];
    }

    pub fn build_ui(&mut self, ui: &mut Ui<GlyphCache>) {
        self.cleanup();
        let node_count = self.graph.node_count();
        for i in (0..node_count) {
            let index = NodeIndex::new(i);
            self.graph.node_weight_mut(index).map(|node| node.build_ui(ui));
        }
    }

    /// Remove all nodes and edges marked for destruction
    pub fn cleanup(&mut self) {
        loop {
            if let Some((i, _)) = self.graph.raw_nodes().iter().enumerate().find(|&(_i, ref node)| node.weight.destroy) {
                let index = NodeIndex::new(i);
                self.graph.remove_node(index);
            }
            break;
        }
    }

    /// Export the UiGraph structure to the original source graph format
    pub fn as_source_graph(&self) -> Graph<N, E> {
        // TODO
        Graph::new()
    }
}
