/// mush graph backend and abstraction to swap out with custom backend

use uuid::Uuid;
use std::collections::HashMap;

// graph functions
pub trait GraphBackend {
    fn add_element(GraphElement) -> bool;
    fn disable_element(GraphElement) -> bool;

    /// forcefully removes element from graph, all edge nodes should be updated
    fn remove_element(GraphElement) -> bool;
}

//----

pub enum GraphElement {
    GraphNode,
    //GraphEdge,
}

pub struct GraphNode {
    uuid: Uuid,
    edges: HashMap<Uuid,Option<f64>>, //to,weight
}

/// unidirectional edge, use two edges for bidirectional/undirected graph
/*pub struct GraphEdge {
    to: Uuid,
    maybe_weight: Option<f64>,
}*/
impl GraphNode {
    fn new (uuid: Uuid) -> GraphNode {
        GraphNode { uuid: uuid,
                    edges: HashMap::new() }
    }
    fn direct (&mut self, to:Uuid, weight: Option<f64>) {
        self.edges.insert(to,weight);
    }
}


//----


pub struct Graph {
    nodes: HashMap<Uuid,GraphNode>,
}
impl Graph {
    fn new () -> Graph {
        Graph { nodes: HashMap::new() }
    }
    fn add (&mut self) -> Uuid { //todo: maybe_edges fn arg
        let uuid = Uuid::new_v4();
        let n = GraphNode::new(uuid);
        self.nodes.insert(uuid,n);
        uuid
    }
    fn get_mut(&mut self, uuid: Uuid) -> Option<&mut GraphNode> {
        self.nodes.get_mut(&uuid)
    }
}




// ----
// tests
// ----
#[cfg(test)]
mod tests {
    extern crate test;
    use ::backend::{Graph,GraphNode};
    
    #[test]
    fn test_basic() {
        let mut graph = Graph::new();
        let a = graph.add();
        let b = graph.add();
        graph.get_mut(a).unwrap().direct(b,None);
    }
        
}
