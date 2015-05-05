use conrod::{Ui, UiId, Button, Label, Positionable, Labelable, Sizeable};
use petgraph::Graph;
use petgraph::graph::{NodeIndex, EdgeIndex};
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
    ui_id: UiId,

    // NOTE - These state variables probably eventually belong in a conrod widget state..
    drag: bool,
    collapse: bool,
    destroy: bool,
}

impl<N: EditableNode> UiNode<N> {
    pub fn build_ui(&mut self, ui: &mut Ui<GlyphCache>, node_index: NodeIndex, ui_graph_state: &mut ConnectionState) {

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
            Label::new(self.source_node.get_label())
                .down_from(ui_id_start, 5.0)
                .set(ui_id_start + 3, ui);

            Button::new()
                .down_from(ui_id_start + 3, 5.0)
                .label("+")
                .dimensions(20.0, 20.0)
                .react(|| {
                    match ui_graph_state {
                        &mut ConnectionState::Connecting(source_node) => {
                            *ui_graph_state = ConnectionState::ConnectionAdded(source_node, node_index);
                        },
                        &mut ConnectionState::Default => {
                            *ui_graph_state = ConnectionState::Connecting(node_index);
                        },
                        _ => {}
                    }
                })
                .set(ui_id_start + 4, ui);
        }
    }
}

pub struct UiEdge<E> {
    source_edge: E,
    ui_id: UiId,
    destroy: bool,
}

impl<E: EditableEdge> UiEdge<E> {
    pub fn build_ui(&mut self, ui: &mut Ui<GlyphCache>, position: [f64; 2]) {
        // TEMP - put a 'delete' button between the nodes until we can get a proper Edge/Connection widget
        let ui_id_start = self.ui_id;
        Button::new()
            .xy(position[0], position[1])
            .label("X")
            .dimensions(50.0, 20.0)
            .react(|| self.destroy = true)
            .set(ui_id_start, ui);
    }
}

pub enum ConnectionState {
    Default,
    Connecting(NodeIndex),
    ConnectionAdded(NodeIndex, NodeIndex)
}

pub struct UiGraph<N: EditableNode, E: EditableEdge> {
    graph: Graph<UiNode<N>, UiEdge<E>>,
    ui_id_range: [UiId; 2],
    connection_state: ConnectionState,
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
            ui_id += 5;
        }

        // Node indices should be consistent between source graph and ui graph (as long as we don't remove nodes)
        // Add edges from source graph to ui graph

        for edge in source_graph.raw_edges().iter() {
            graph.add_edge(edge.source(), edge.target(), UiEdge {
                source_edge: edge.weight.clone(),
                ui_id: ui_id,
                destroy: false,
            });
            ui_id += 1;
        }

        UiGraph {
            graph: graph,
            ui_id_range: [start_index, ui_id],
            connection_state: ConnectionState::Default,
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
        self.ui_id_range = [ui_id_range[0], ui_id_range[1] + 5];
    }

    pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        let ui_id_range = self.ui_id_range;
        self.graph.add_edge(source, target, UiEdge {
            source_edge: E::default(),
            ui_id: ui_id_range[1],
            destroy: false,
        });
        self.ui_id_range = [ui_id_range[0], ui_id_range[1] + 1];
    }

    pub fn build_ui(&mut self, ui: &mut Ui<GlyphCache>) {
        self.update();
        let node_count = self.graph.node_count();
        let connection_state = &mut self.connection_state;

        for i in (0..node_count) {
            let index = NodeIndex::new(i);
            self.graph.node_weight_mut(index).map(|node| node.build_ui(ui, index, connection_state));
        }

        // This really blows. Fighting with borrow checker...
        let mut midpoints = Vec::new();
        for edge in self.graph.raw_edges().iter() {
            let source = self.graph.node_weight(edge.source()).unwrap();
            let target = self.graph.node_weight(edge.target()).unwrap();
            let position_1 = source.source_node.get_position();
            let position_2 = target.source_node.get_position();
            let position = [(position_1[0] + position_2[0]) / 2.0,
                            (position_1[1] + position_2[1]) / 2.0];
            midpoints.push(position);
        }

        let edge_count = self.graph.edge_count();
        for i in (0..edge_count) {
            let index = EdgeIndex::new(i);
            self.graph.edge_weight_mut(index).map(|edge| edge.build_ui(ui, midpoints[i]));
        }
    }

    /// Add edge if connection is pending
    /// Remove all nodes and edges marked for destruction
    pub fn update(&mut self) {

        if let ConnectionState::ConnectionAdded(source, target) = self.connection_state {
            self.add_edge(source, target);
            self.connection_state = ConnectionState::Default;
        }

        // This is really inefficient, try to find a better way :/
        // Remove destroyed nodes
        loop {
            if let Some((i, _)) = self.graph.raw_nodes().iter().enumerate().find(|&(_i, ref node)| node.weight.destroy) {
                let index = NodeIndex::new(i);
                self.graph.remove_node(index);

                // NodeIndices are not stable after removing nodes
                self.connection_state = ConnectionState::Default;
            }
            break;
        }

        // Remove destroyed edges
        loop {
            if let Some((i, _)) = self.graph.raw_edges().iter().enumerate().find(|&(_i, ref edge)| edge.weight.destroy) {
                let index = EdgeIndex::new(i);
                self.graph.remove_edge(index);
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
