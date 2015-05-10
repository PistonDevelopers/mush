/// mush graph backend and abstraction to swap out with custom backend

use uuid::Uuid;
use std::collections::{HashMap,HashSet,VecDeque};



/// unidirectional edge, use two edges for bidirectional/undirected graph
//#[derive(Hash, Eq, PartialEq, Debug)]
#[derive(Debug)]
struct GraphEdge<K> {
    to: Uuid,
    maybe_weight: Option<f64>,
    maybe_custom: Option<K>,
}
impl<K> GraphEdge<K> {
    // this should be private, this should be called by a graph method
    fn new (uuid: Uuid, weight: Option<f64>, custom: Option<K>) -> GraphEdge<K> {
        GraphEdge { to: uuid,
                    maybe_weight:weight,
                    maybe_custom: custom }
    }
    pub fn get_directed (&self) -> &Uuid {
        &self.to
    }
    
    pub fn get_weight (&self) -> Option<&f64> {
        if let Some(ref w) = self.maybe_weight { Some(w) }
        else { None }
    }
    pub fn get_weight_mut (&mut self) -> Option<&mut f64> {
        if let Some(ref mut w) = self.maybe_weight { Some(w) }
        else { None }
    }
    
    pub fn get_custom (&self) -> Option<&K> {
        if let Some(ref k) = self.maybe_custom { Some(k) }
        else { None }
    }
    pub fn get_custom_mut (&mut self) -> Option<&mut K> {
        if let Some(ref mut k) = self.maybe_custom { Some(k) }
        else { None }
    }
}

//--

#[derive(Debug)]
struct GraphNode {
    uuid: Uuid,
    edges_to: HashMap<Uuid,Option<f64>>, //to,weight //todo: turn to hashset if using graphedge struct
    edges_from: HashMap<Uuid,Option<f64>>,
}
impl GraphNode {
    fn new (uuid: Uuid) -> GraphNode {
        GraphNode { uuid: uuid,
                    edges_to: HashMap::new(),
                    edges_from: HashMap::new(), }
    }
    /// direct the node towards another node
    // todo: rename me! sounds too similar to unidirectional
    fn direct (&mut self, to:&Uuid, weight: Option<f64>) {
        self.edges_to.insert(*to,weight);
    }
    fn undirect (&mut self, to:&Uuid) {
        self.edges_to.remove(to);
    }
}

//----
#[derive(Debug)]
pub struct Graph {
    nodes: HashMap<Uuid,GraphNode>,

    // todo: as traits
    is_weighted: bool,
    is_directed: bool,
    is_tracking: bool,  // tracking from-edges
}

//todo: turn many of these methods into a trait
impl Graph {
    pub fn default() -> Graph {
        Graph { nodes: HashMap::new(),
                is_weighted: false,
                is_directed: true,
                is_tracking: false, }
    }

    /// manual accessors
    fn get_node_mut(&mut self, uuid: &Uuid) -> Option<&mut GraphNode> {
        self.nodes.get_mut(uuid)
    }
    fn get_node(&self, uuid: &Uuid) -> Option<&GraphNode> {
        self.nodes.get(uuid)
    }

    pub fn add (&mut self) -> Uuid { //todo: maybe_edges fn arg
        let uuid = Uuid::new_v4();
        let n = GraphNode::new(uuid);
        self.nodes.insert(uuid,n);
        uuid
    }
    pub fn remove(&mut self, node: &Uuid) -> bool { //Option<GraphNode> {
        self.nodes.remove(node).is_some()
    }

    pub fn direct(&mut self, from: &Uuid, to: &Uuid) -> bool {
        let mut r = true;

        if !self.get_node(to).is_some() { return false } // todo: expand on this, and impl for undirect?
        
        if let Some(f) = self.nodes.get_mut(from) {
            f.direct(to,None);
        }
        else { r = false; }

        if self.is_directed { return r }
        else if r {
            if let Some(t) = self.nodes.get_mut(to) {
                t.direct(from,None);
            }
            else { r = false; }
        }

        return r
    }
    pub fn undirect(&mut self, from: &Uuid, to: &Uuid) -> bool {
        let mut r = true;
        if let Some(f) = self.nodes.get_mut(from) {
            f.undirect(to);
        }
        else { r = false; }

        if self.is_directed { return r }
        else if r {
            if let Some(t) = self.nodes.get_mut(to) {
                t.undirect(from);
            }
            else { r = false; }
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
                    if let Some(ref node) = self.get_node(&cursor.unwrap()) {
                        //get first unvisited node
                        let not_visited = node.edges_to.iter().find(|&(&n,v)| !visited.contains(&n));
                        
                        if let Some((&n,w)) = not_visited {
                            if !self.is_tracking || self.nodes.contains_key(&n) { //node exists?
                                stack.push(n);
                                visited.insert(n);
                                result.push(n);

                                if let Some(to_node) = to {
                                    if n == to_node { break; }
                                }

                                cursor = Some(n);
                            }
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
                    if let Some(ref node) = self.get_node(&cursor.unwrap()) {
                        //get unvisted nodes to queue up
                        let not_visited: Vec<Option<(Uuid,Option<f64>)>> = node.edges_to.iter().map(|(&n,&v)| {
                            if !visited.contains(&n) {
                                Some((n,v))
                            }
                            else { None }
                        }).collect();

                        for maybe_node in not_visited {
                            if let Some((n,w)) = maybe_node {
                                if !self.is_tracking || self.nodes.contains_key(&n) { //node exists?
                                    queue.push_back(n);
                                    visited.insert(n);
                                    result.push(n);

                                    if let Some(to_node) = to {
                                        if n == to_node { break; }
                                    }

                                    cursor = Some(n);
                                }
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
    pub fn get_cycle(&self, from: Uuid) -> HashSet<(Uuid,Uuid)> {
        let mut stack = vec!();
        let mut visited: HashSet<Uuid> = HashSet::new();
        let mut r = HashSet::new(); //Vec::new();

        stack.push(from);
        
        while stack.len() > 0 {
            let cursor = *stack.last().unwrap();
            visited.insert(cursor);
            
            if let Some(ref node) = self.get_node(&cursor) {
                
                //does the cursor point to a node on stack
                for (n,_) in node.edges_to.iter() {
                    if stack.contains(&n) {
                        r.insert((*n,cursor));
                    }
                }

                //get first unvisited node
                let not_visited = node.edges_to.iter().find(|&(n,_)| !visited.contains(n));
                
                if let Some((&n,_)) = not_visited {
                    if !stack.contains(&n) {
                        stack.push(n); //add node to check
                    }
                }
                else { stack.pop(); } //nothing left, pop off and head back a node
            }
            else { stack.pop(); } //invalid node?
        }

        return r
    }

    /// get immediate next node from list of connected nodes for the current node
    pub fn get_next(&self, from: &Uuid) -> Option<Uuid> {
        if let Some(n) = self.nodes.get(from) {
            if let Some(next_id) = n.edges_to.iter().next() {
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
    pub fn track_from_nodes(mut self, t: bool) -> GraphBuilder {
        self.0.is_tracking = t;
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

    use uuid::Uuid;
    use std::collections::{HashMap};

    #[test]
    fn test_basic_direct() {
        let mut graph = Graph::default();
        let mut nodes = vec!();
        for _ in 0..5 {
            nodes.push(graph.add());
        }
        
        assert!(graph.direct(&nodes[0],&nodes[1]));

        assert!(graph.direct(&nodes[3],&nodes[0]));
        assert!(graph.direct(&nodes[4],&nodes[3]));

        graph.remove(&nodes[2]);
        assert!(!graph.direct(&nodes[4],&nodes[2]));
        
        let n6 = graph.add();
        assert!(graph.direct(&n6,&nodes[3]));
        assert!(graph.direct(&nodes[0],&n6));
    }


    
    #[test]
    fn test_basic_paths() {
        let mut graph = Graph::default();
        let mut nodes = vec!();
        for _ in 0..5 {
            nodes.push(graph.add());
        }
        
        graph.direct(&nodes[0],&nodes[1]);

        graph.direct(&nodes[3],&nodes[0]);
        graph.direct(&nodes[4],&nodes[3]);

        graph.remove(&nodes[2]);

        let r = graph.get_path(GraphSearch::Depth(nodes[0],Some(nodes[4])));
        assert!(!r.is_some());

        let r = graph.get_path(GraphSearch::Depth(nodes[4],Some(nodes[0])));
        assert!(r.is_some());

        let r = graph.get_path(GraphSearch::Breadth(nodes[4],Some(nodes[0])));
        assert!(r.is_some());
    }



    #[test]
    fn test_basic_cycle() {
        let mut graph = Graph::default();
        let mut nodes = vec!();
        for _ in 0..5 {
            nodes.push(graph.add());
        }
        
        graph.direct(&nodes[0],&nodes[1]);

        graph.direct(&nodes[3],&nodes[0]);
        graph.direct(&nodes[4],&nodes[3]);

        graph.remove(&nodes[2]);


        let r = graph.get_cycle(nodes[4]);
        assert_eq!(r.len(),0);
        
        let n6 = graph.add();
        graph.direct(&n6,&nodes[3]);
        graph.direct(&nodes[0],&n6);

        let n7 = graph.add();
        graph.direct(&n7,&nodes[3]);
        graph.direct(&nodes[0],&n7);
        
        let r = graph.get_cycle(nodes[4]);
        println!("{:?}",r);
        assert_eq!(r.len(),2);
    }
}
