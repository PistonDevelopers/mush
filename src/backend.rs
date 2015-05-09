/// mush graph backend and abstraction to swap out with custom backend

use uuid::Uuid;
use std::collections::{HashMap,HashSet,VecDeque};


/// unidirectional edge, use two edges for bidirectional/undirected graph
/*
struct GraphEdge {
    to: Uuid,
    maybe_weight: Option<f64>,
}
impl GraphEdge {
    // this should be private, this should be called by a graph method
    fn new (uuid: Uuid, weight: Option<f64>) -> GraphEdge {
        GraphEdge { to: uuid,
                    maybe_weight:weight }
    }
    pub fn get_directed (&self) -> &Uuid {
        &self.to
    }
    pub fn get_weight (&self) -> Option<f64> {
        self.maybe_weight
    }
}*/

#[derive(Debug)]
struct GraphNode {
    uuid: Uuid,
    edges: HashMap<Uuid,Option<f64>>, //to,weight
}
impl GraphNode {
    fn new (uuid: Uuid) -> GraphNode {
        GraphNode { uuid: uuid,
                    edges: HashMap::new() }
    }
    /// direct the node towards another node
    // todo: rename me! sounds too similar to unidirectional
    fn direct (&mut self, to:&Uuid, weight: Option<f64>) -> bool {
        self.edges.insert(*to,weight).is_some()
    }
    fn undirect (&mut self, to:&Uuid) -> bool {
        self.edges.remove(to).is_some()
    }
}

//----

pub struct Graph {
    nodes: HashMap<Uuid,GraphNode>,

    // todo: as traits
    is_weighted: bool,
    is_directed: bool,
}
impl Graph {
    pub fn default() -> Graph {
        Graph { nodes: HashMap::new(),
                is_weighted: false,
                is_directed: true, }
    }

    /// manual accessors
    fn get_mut(&mut self, uuid: &Uuid) -> Option<&mut GraphNode> {
        self.nodes.get_mut(uuid)
    }
    fn get(&self, uuid: &Uuid) -> Option<&GraphNode> {
        self.nodes.get(uuid)
    }

    pub fn add (&mut self) -> Uuid { //todo: maybe_edges fn arg
        let uuid = Uuid::new_v4();
        let n = GraphNode::new(uuid);
        self.nodes.insert(uuid,n);
        uuid
    }
    pub fn remove(&mut self, node: &Uuid) { //-> Option<GraphNode> {
        self.nodes.remove(node);
    }

    pub fn direct(&mut self, from: &Uuid, to: &Uuid) -> bool {
        let mut r = false;
        if let Some(f) = self.nodes.get_mut(from) {
            r = f.direct(to,None);
        }

        if self.is_directed { return r }
        else if r {
            if let Some(t) = self.nodes.get_mut(to) {
                r = t.direct(from,None);
            }
        }

        return r
    }
    pub fn undirect(&mut self, from: &Uuid, to: &Uuid) -> bool {
        let mut r = false;
        if let Some(f) = self.nodes.get_mut(from) {
            r = f.undirect(to);
        }

        if self.is_directed { return r }
        else if r {
            if let Some(t) = self.nodes.get_mut(to) {
                r = t.undirect(from);
            }
        }

        return r
    }

    // search functions
    // todo: consider weights between nodes to direct search, cycle-detection
    pub fn get_path(&self, s: GraphSearch) -> Option<Vec<Uuid>> {
        let mut visited: HashSet<Uuid> = HashSet::new();
        let mut result = vec!();
        
        match s {
            GraphSearch::Depth(from,to) => {
                let mut stack = vec!();

                stack.push(from);
                visited.insert(from);
                result.push(from);

                let mut cursor = Some(from);

                while cursor.is_some() {
                    if let Some(ref node) = self.get(&cursor.unwrap()) {
                        //get first unvisited node
                        let not_visited = node.edges.iter().find(|&(&n,v)| !visited.contains(&n));
                        
                        if let Some((&n,w)) = not_visited {
                            stack.push(n);
                            visited.insert(n);
                            result.push(n);

                            if let Some(to_node) = to {
                                if n == to_node { break; }
                            }

                            cursor = Some(n);
                        }
                        else { cursor = stack.pop(); }
                    }
                    else { cursor = stack.pop(); }
                }

                if let Some(to_node) = to {
                    if result.contains(&to_node) {
                        return Some(result)
                    }
                }

                return None
            },
            GraphSearch::Breadth(from,to) => { // breadth first search, uses a queue
                let mut queue = VecDeque::new();

                queue.push_back(from);
                visited.insert(from);
                result.push(from);


                let mut cursor = Some(from);

                while cursor.is_some() {
                    if let Some(ref node) = self.get(&cursor.unwrap()) {
                        //get unvisted nodes to queue up
                        let not_visited: Vec<Option<(Uuid,Option<f64>)>> = node.edges.iter().map(|(&n,&v)| {
                            if !visited.contains(&n) {
                                Some((n,v))
                            }
                            else { None }
                        }).collect();

                        for maybe_node in not_visited {
                            if let Some((n,w)) = maybe_node {
                                queue.push_back(n);
                                visited.insert(n);
                                result.push(n);

                                if let Some(to_node) = to {
                                    if n == to_node { break; }
                                }

                                cursor = Some(n);
                            }
                        }

                        cursor = queue.pop_front();
                    }
                    else { cursor = queue.pop_front(); }
                }

                if let Some(to_node) = to {
                    if result.contains(&to_node) {
                        return Some(result)
                    }
                }

                return None
            },
            _ => None, // todo: djk algo
        }

        
        
    }

    //this is virtually the same as get_path dfs, should abstract dfs somehow to use it for this
    pub fn get_cycle(&self, from: Uuid, to: Option<Uuid>) -> Option<Uuid> { //Option<Vec<Uuid>> {
        let mut stack = vec!();
        let mut visited: HashSet<Uuid> = HashSet::new();
        
        stack.push(from);
        visited.insert(from);

        let mut cursor = Some(from);

        while cursor.is_some() {
            if let Some(ref node) = self.get(&cursor.unwrap()) {
                //get first unvisited node
                let not_visited = node.edges.iter().find(|&(&n,v)| !visited.contains(&n));

                if let Some(cycle) = node.edges.iter().find(|&(&n,v)| stack.contains(&n)) {
                    return Some(*cycle.0);
                    // should consider returning stack
                }
                
                if let Some((&n,w)) = not_visited {
                    stack.push(n);
                    visited.insert(n);

                    if let Some(to_node) = to {
                        if n == to_node { break; }
                    }

                    cursor = Some(n);
                }
                else { cursor = stack.pop(); }
            }
            else { cursor = stack.pop(); }
        }

        return None
    }

    /// get immediate next node from list of connected nodes for the current node
    pub fn get_next(&self, from: &Uuid) -> Option<Uuid> {
        if let Some(n) = self.nodes.get(from) {
            if let Some(next_id) = n.edges.iter().next() {
                return Some(*next_id.0) // grab uuid key
            }
        }

        return None
        
    }

    fn is_connected() -> bool { false }
    fn is_complete() -> bool { false }
    
    fn get_path_shortest(&self) -> bool {
        if !self.is_weighted { false } //must be weighted
        else { false } //todo: use bfs
    }
}

// todo: impl as trait instead?
pub enum GraphSearch {
    Depth(Uuid,Option<Uuid>), // used on part of graph for reachability, and all of graph for cycle-detection
    Breadth(Uuid,Option<Uuid>), // used on part of graph for reachability, and (unweighted) for shortest path
    Dijkstra(Uuid,Uuid), // used on part of graph (weighted) for shortest path
}


pub struct GraphBuilder(Graph);

impl GraphBuilder {
    pub fn new() -> GraphBuilder {
        GraphBuilder(Graph::default())
    }

    pub fn directed(mut self, d: bool) -> GraphBuilder {
        self.0.is_directed = d;
        self
    }
    pub fn weighted(mut self, w: bool) -> GraphBuilder {
        self.0.is_weighted = w;
        self
    }
    pub fn build(mut self) -> Graph {
        self.0
    }
}

// ----
// tests
// ----
#[cfg(test)]
mod tests {
    extern crate test;

    use ::{Graph,GraphSearch};
    
    #[test]
    fn test_basic() {
        let mut graph = Graph::default();
        let mut nodes = vec!();
        for i in 0..5 {
            nodes.push(graph.add());
        }
        
        graph.direct(&nodes[0],&nodes[1]);

        graph.direct(&nodes[3],&nodes[0]);
        graph.direct(&nodes[4],&nodes[3]);
        

        let r = graph.get_path(GraphSearch::Depth(nodes[0],Some(nodes[4])));
        assert!(!r.is_some());

        let r = graph.get_path(GraphSearch::Depth(nodes[4],Some(nodes[0])));
        assert!(r.is_some());


        {let r = graph.get_cycle(nodes[4],Some(nodes[1]));
         assert!(!r.is_some());}
        
        let n2 = graph.add();
        graph.direct(&n2,&nodes[3]);
        graph.direct(&nodes[0],&n2);
        
        let r = graph.get_cycle(nodes[4],Some(nodes[1]));
        assert!(r.is_some());
    }
        
}
