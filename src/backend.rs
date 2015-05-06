/// mush graph backend and abstraction to swap out with custom backend

use uuid::Uuid;
use std::collections::HashMap;

// graph functions
/*pub trait GraphBackend<I:PartialEq> {
    fn add(&mut self) -> I;
    //fn disable(N) -> bool;

    /// forcefully removes element from graph, all edge nodes should be updatedb
    fn remove(&mut self, node: &I);// -> Vec<E>;

    fn direct(&mut self, from: &I, to: &I);
    fn undirect(&mut self, from: &I, to: &I);
}*/

//pub trait<I> GraphSearch<I> {
  //  fn connections(&self, from: I, to: I) -> Vec<I>;
//}

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
    fn direct (&mut self, to:&Uuid, weight: Option<f64>) -> bool {
        self.edges.insert(*to,weight).is_some()
    }
    fn undirect (&mut self, to:&Uuid, weight: Option<f64>) -> bool {
        self.edges.remove(to).is_some()
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

    fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut GraphNode> {
        self.nodes.get_mut(uuid)
    }

    fn add (&mut self) -> Uuid { //todo: maybe_edges fn arg
        let uuid = Uuid::new_v4();
        let n = GraphNode::new(uuid);
        self.nodes.insert(uuid,n);
        uuid
    }
    fn remove(&mut self, node: &Uuid) {
        self.nodes.remove(node);
    }
    fn direct(&mut self, from: &Uuid, to: &Uuid) -> bool {
        if let Some(f) = self.nodes.get_mut(from) {
            f.direct(to,None)
        }
        else { false }
    }
    fn undirect(&mut self, from: &Uuid, to: &Uuid) -> bool {
        if let Some(f) = self.nodes.get_mut(from) {
            f.undirect(to,None)
        }
        else { false }
    }


    // search functions
    // todo: consider weights between nodes, breadth/depth first, cycle-detection
    fn get_path(&self, s: GraphSearch) -> Vec<&Uuid> {
        vec!()
    }

    fn get_cycle(&self, s: GraphSearch) -> Vec<&Uuid> {
        vec!()
    }

    fn get_next(&self, s:GraphSearch) {
        match s {
            GraphSearch::Depth(from,to) => {},
            _ => {},
        }
    }
}

// todo: impl as trait instead?
pub enum GraphSearch {
    Depth(Uuid,Uuid),
    Breadth(Uuid,Uuid),
}

//impl<I:PartialEq> GraphBackend<I> for Graph {
//}



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
        let nodes = &[graph.add();5];
        graph.get_mut(&nodes[0]).unwrap().direct(&nodes[1],None); //note there is no verification that b exists when doing this manually

        graph.direct(&nodes[1],&nodes[4]);
        graph.remove(&nodes[2]);
        assert!(!graph.direct(&nodes[2],&nodes[3]));

        
    }
        
}
