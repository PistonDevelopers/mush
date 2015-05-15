use conrod::{Ui, UiId, Button, Label, Positionable, Labelable, Sizeable,Widget,WidgetId, UserInput, Floating,Colorable};
use conrod::color::{blue, light_orange, orange, dark_orange, red, white};
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use opengl_graphics::glyph_cache::GlyphCache;

pub trait EditableNode: Clone {
    fn get_label(&self) -> &str;
    fn get_position(&self) -> [f64; 2];
    fn set_position(&mut self, [f64; 2]);
    fn default() -> Self;
}

pub trait EditableEdge: Clone {
    fn default() -> Self;
}

pub struct UiNode<N: EditableNode> {
    source_node: N,
    ui_id: WidgetId,

    // NOTE - These state variables probably eventually belong in a conrod widget state..
    drag: bool,
    collapse: bool,
    destroy: bool,
}

impl<N: EditableNode> UiNode<N> {
    pub fn build_ui(&mut self, ui: &mut Ui<GlyphCache>) {

        if self.destroy { return }

        let mut canvas_height = 100.0;
        let mut cl = "<"; //canvas label
        if self.collapse { cl = ">";
                           canvas_height = 10.0; }
        
        let ui_id_start: WidgetId = self.ui_id;

        let position = self.source_node.get_position();

        //floating canvas where our node data resides
        Floating::new()
            .label(self.source_node.get_label())
            .xy(position[0], position[1])
            .height(canvas_height) //I think floating canvas is missing dynamic dimensions, so this only works the once and cache is then set
            .color(blue())
            .label_color(white())
            .set(ui_id_start, ui);



        //build buttons in reverse order, to stay within canvas
        //start at top right, and head left from
        Button::new()
            .top_right_of(ui_id_start) //todo: shift up ~5.0
            .label("x")
            .dimensions(20.0,20.0)
            .react(|| self.destroy = true)
            .set(ui_id_start + 2, ui);
        
        Button::new()
            .left(5.0)
            .label(cl)
            .dimensions(20.0,20.0)
            .react(|| self.collapse = !self.collapse)
            .set(ui_id_start + 1, ui);

        //todo: collapse floating canvas above!
        if !self.collapse {
            Label::new("Node Data")
                .middle_of(ui_id_start)
                .set(ui_id_start + 3, ui);
        }
    }
}

pub struct UiEdge<E> {
    source_edge: E,
    ui_id: WidgetId,
}

pub struct UiGraph<N: EditableNode, E: EditableEdge> {
    graph: Graph<UiNode<N>, UiEdge<E>>,
    ui_id_range: [WidgetId; 2],
}

impl<N: EditableNode, E: EditableEdge> UiGraph<N, E> {

    /// Initialize a UiGraph from a source graph structure
    pub fn new(source_graph: &Graph<N, E>, start_index: WidgetId) -> UiGraph<N, E> {

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

        let mut source_graph = Graph::new();

        for node in self.graph.raw_nodes().iter() {
            source_graph.add_node(node.weight.source_node.clone());
        }

        for edge in self.graph.raw_edges().iter() {
            source_graph.add_edge(edge.source(), edge.target(), edge.weight.source_edge.clone());
        }

        source_graph
    }
}
