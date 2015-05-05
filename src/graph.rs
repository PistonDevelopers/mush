use conrod::UiId;
use petgraph::Graph;

pub trait EditableNode {
    fn get_position(&self) -> [f64; 2];
    fn set_position(&mut self, [f64; 2]);
    fn default() -> Self;
}

pub trait EditableEdge {
    fn default() -> Self;
}

pub struct UiNode<N> {
    source_node: N,
    ui_id: UiId,
}

pub struct UiEdge<E> {
    source_edge: E,
    ui_id: UiId,
}

pub struct UiGraph<N, E> {
    graph: Graph<UiNode<N>, UiEdge<E>>,
    ui_id_range: [UiId; 2],
}

impl<N, E> UiGraph<N, E>
    where N: Clone,
          E: Clone
{
    pub fn new(source_graph: &Graph<N, E>, start_index: UiId) -> UiGraph<N, E> {

        let mut graph = Graph::new();

        let mut ui_id = start_index;

        // Add nodes from source graph to ui graph

        for node in source_graph.raw_nodes().iter() {
            graph.add_node(UiNode {
                source_node: node.weight.clone(),
                ui_id: ui_id,
            });
            ui_id += 1; // TODO - need to increment by number of widgets needed for node...
        }

        // Node indices should be consistent between source graph and ui graph (as long as we don't remove nodes)
        // Add edges from source graph to ui graph

        for edge in source_graph.raw_edges().iter() {
            graph.add_edge(edge.source(), edge.target(), UiEdge {
                source_edge: edge.weight.clone(),
                ui_id: ui_id,
            });
            ui_id += 1; // TODO - need to increment by number of widgets needed for edge...
        }

        UiGraph {
            graph: graph,
            ui_id_range: [start_index, ui_id]
        }
    }
}
