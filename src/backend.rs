/// mush graph backend and abstraction to swap out with custom backend

use uuid::Uuid;
use std::collections::{HashMap,HashSet,VecDeque};

pub type Nid = Uuid; // graph node id
pub type Eid = (Nid,Nid); // graph edge id, (From,To)


/// unidirectional edge, use two edges for bidirectional/undirected graph
pub trait GraphEdge: Copy+Clone {
    fn default () -> Self;
}

pub trait GraphNode: Clone+EdgeGuard {
    fn default () -> Self;
    
    fn get_base(&self) -> &NodeBase;
    fn get_base_mut(&mut self) -> &mut NodeBase;
    
    fn get_name(&self) -> &str;
    fn get_position(&self) -> &[f64;2];

    fn set_name(&mut self, s: &str);
    fn set_position(&mut self, p: [f64;2]);
}

/// trait specifying node connection requirements
pub trait EdgeGuard: PartialEq {
    fn guard(&self, node: &Self) -> bool;
}
//--

#[derive(Debug,Clone,PartialEq)]
pub struct NodeBase {
    nid: Nid,
    edges_to: HashSet<Nid>,
    edges_from: HashSet<Nid>,
}
impl NodeBase {
    fn new () -> NodeBase {
        NodeBase { nid: Uuid::new_v4(),
                   edges_to: HashSet::new(),
                   edges_from: HashSet::new(), }
    }
    /// direct the node towards another node
    // todo: rename me! sounds too similar to unidirectional
    fn direct (&mut self, to:&Nid) {
        self.edges_to.insert(*to);
    }
    fn direct_from (&mut self, from:&Nid) {
        self.edges_from.insert(*from);
    }
    fn undirect (&mut self, to:&Nid) {
        self.edges_to.remove(to);
    }
    fn undirect_from (&mut self, from:&Nid) {
        self.edges_from.remove(from);
    }
   /* pub fn has_edge_from(&self,from:&Nid) -> bool {
        self.edges_from.contains((from,self.nid))
    }
    pub fn get_edge(&self,to:&Nid) -> bool {
        self.edges.contains((self.nid,to))
    }*/
}

pub trait Backend {
    type N;
    type I; //node id
    
    fn default() -> Self;
    
}

//----
#[derive(Debug)]
pub struct Graph<E:GraphEdge, N:GraphNode> {
    nodes: HashMap<Nid,N>,
    edges: HashMap<Eid,E>,
    
    // todo: as traits
    is_weighted: bool,
    is_directed: bool,
    is_tracking: bool,  // tracking from-edges
}

// todo: work on getting this to work along side of regular graph direct method
/*impl<E:GraphEdge, N:GraphNode> Graph<E,N> where N: EdgeGuard {
    pub fn direct(&mut self, from: &Nid, to: &Nid, e: E) -> bool {
        if let Some(f) = self.nodes.get(from) {
            if let Some(t) = self.nodes.get(to) {
                if !f.guard(t) { return false }
            }
        }

        true
    }
}*/

impl<E:GraphEdge, N:GraphNode> Backend for Graph<E,N> {
    type N = NodeBase;
    type I = Nid;
    fn default() -> Graph<E,N> {
        Graph { nodes: HashMap::new(),
                edges: HashMap::new(),
                is_weighted: false,
                is_directed: true,
                is_tracking: false, }
    }
}

//todo: turn many of these methods into a trait
impl<E:GraphEdge, N:GraphNode> Graph<E,N> {
    /*pub fn default() -> Graph<E,N> {
        Graph { nodes: HashMap::new(),
                edges: HashMap::new(),
                is_weighted: false,
                is_directed: true,
                is_tracking: false, }
    }*/

    /// manual accessors
    fn get_node_mut(&mut self, n: &Nid) -> Option<&mut N> {
        self.nodes.get_mut(n)
    }
    fn get_node(&self, n: &Nid) -> Option<&N> {
        self.nodes.get(n)
    }

    pub fn add (&mut self) -> Nid { //todo: maybe_edges fn arg
        let n: N = GraphNode::default();
        let nid = n.get_base().nid;
        self.nodes.insert(nid,n);
        nid
    }
   fn add_edge (&mut self, from: &Nid, to: &Nid, e: E) -> Eid {
       let eid = (*from,*to);
       self.edges.insert(eid,e);
       eid
    }
    
    pub fn remove(&mut self, n: &Nid) -> Option<N> {
        self.nodes.remove(n)
    }

    pub fn get_edge_mut (&mut self, e: &Eid) -> Option<&mut E> {
        self.edges.get_mut(e)
    }
    pub fn get_edge (&self, e: &Eid) -> Option<&E> {
        self.edges.get(e)
    }

    //todo: check for previous edge!
    pub fn direct(&mut self, from: &Nid, to: &Nid, e: E) -> bool {
        let mut r = false;

        if let Some(f) = self.nodes.get(from) {
            if let Some(t) = self.nodes.get(to) {
                if !f.guard(t) { return false }
            }
        }
        
        let eid = self.add_edge(from,to,e);

        if !self.get_node(to).is_some() { return false }
        
        if let Some(f) = self.nodes.get_mut(from) {
            f.get_base_mut().direct(to);
            r = true;
        }

        if r {
            
            
            if self.is_tracking { let t = self.nodes.get_mut(to).unwrap();
                                  t.get_base_mut().direct_from(from); }
            
            if !self.is_directed {
                if let Some(t) = self.nodes.get_mut(to) {
                    t.get_base_mut().direct(to);
                }
                else { r = false; }
            }
        }

        if !r { self.edges.remove(&eid); }
        
        r
    }
    
    pub fn undirect(&mut self, from: &Nid, to: &Nid) {
        let eid = (*from,*to);
        
        if let Some(f) = self.nodes.get_mut(from) {
            f.get_base_mut().undirect(to);
        }
        
        if self.is_tracking {  let t = self.nodes.get_mut(to).unwrap();
                               t.get_base_mut().undirect_from(from); }
        if !self.is_directed {
            if let Some(t) = self.nodes.get_mut(to) {
                t.get_base_mut().undirect(from);
            }
        }

        self.edges.remove(&eid);
    }

    // search functions
    // todo: consider weights between nodes to direct search
    pub fn get_path(&self, s: GraphSearch) -> Option<Vec<Nid>> {
        let mut visited = HashSet::new();
        let mut result = vec!();
        
        match s {
            GraphSearch::Depth(from,to) => {
                let mut stack = vec!();

                stack.push(from);
                
                while stack.len() > 0 {
                    let cursor = *stack.last().unwrap();
                    visited.insert(cursor);
                    
                    if let Some(ref node) = self.get_node(&cursor) {

                        //get first unvisited node
                        let not_visited = node.get_base().edges_to.iter().find(|&n| !visited.contains(n));
                        
                        if let Some(&n) = not_visited {
                            if !self.is_tracking || self.nodes.contains_key(&n) { 
                                stack.push(n); //add node to check
                                result.push(n);

                                if let Some(to_node) = to {
                                    if n == to_node { break; }
                                }
                            }
                        }
                        else { stack.pop(); } //nothing left, pop off and head back a node
                    }
                    else { stack.pop(); } //invalid node?
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

                while queue.len() > 0 {
                    let cursor = *queue.front().unwrap();
                    if let Some(ref node) = self.get_node(&cursor) {
                        //get unvisted nodes to queue up
                        let not_visited: Vec<Option<Nid>> = node.get_base().edges_to.iter().map(|&n| {
                            if !visited.contains(&n) {
                                Some(n)
                            }
                            else { None }
                        }).collect();

                        for maybe_node in not_visited {
                            if let Some(n) = maybe_node {
                                if !self.is_tracking || self.nodes.contains_key(&n) { //node exists?
                                    queue.push_back(n);
                                    visited.insert(n);
                                    result.push(n);

                                    if let Some(to_node) = to {
                                        if n == to_node { break; }
                                    }
                                }
                            }
                        }

                        queue.pop_front();
                    }
                    else { queue.pop_front(); }
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
    pub fn get_cycle(&self, from: Nid) -> HashSet<(Nid,Nid)> {
        let mut stack = vec!();
        let mut visited = HashSet::new();
        let mut r = HashSet::new(); //Vec::new();

        stack.push(from);
        
        while stack.len() > 0 {
            let cursor = *stack.last().unwrap();
            visited.insert(cursor);
            
            if let Some(ref node) = self.get_node(&cursor) {
                
                //does the cursor point to a node on stack
                for n in node.get_base().edges_to.iter() {
                    if stack.contains(&n) {
                        r.insert((*n,cursor));
                    }
                }

                //get first unvisited node
                let not_visited = node.get_base().edges_to.iter().find(|n| !visited.contains(n));
                
                if let Some(&n) = not_visited {
                    if !stack.contains(&n) {
                        if !self.is_tracking || self.nodes.contains_key(&n) {
                            stack.push(n); //add node to check
                        }
                    }
                }
                else { stack.pop(); } //nothing left, pop off and head back a node
            }
            else { stack.pop(); } //invalid node?
        }

        return r
    }

    /// get all nodes in their graph layout (DFS)
    pub fn get_all_nodes(&self) -> Vec<Vec<Nid>> {
        let mut result = vec!();
        let mut visited = HashSet::new();
        
        for (n,d) in &self.nodes {
            if !visited.contains(n) {
                if let Some(r) = self.get_path(GraphSearch::Depth(*n,None)) {
                    for _n in r.iter() { visited.insert(*_n); }
                    result.push(r);
                }
            }
        }
        result
    }

    /// get immediate next node from list of connected nodes for the current node
    pub fn get_next(&self, from: &Nid) -> Option<Nid> {
        if let Some(n) = self.nodes.get(from) {
            if let Some(next_id) = n.get_base().edges_to.iter().next() {
                if !self.is_tracking || self.nodes.contains_key(&next_id) {
                    return Some(*next_id) // grab uuid key
                }
            }
        }
        None
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
    Depth(Nid,Option<Nid>), // used on part of graph for reachability, and all of graph for cycle-detection
    Breadth(Nid,Option<Nid>), // used on part of graph for reachability, and (unweighted) for shortest path
    Dijkstra(Nid,Nid), // used on part of graph (weighted) for shortest path
}


pub struct GraphBuilder<E:GraphEdge,N:GraphNode> (Graph<E,N>);

/// tracking specifies that you wish to track the from-node connections
impl<E:GraphEdge, N:GraphNode> GraphBuilder<E,N> {
    pub fn new() -> GraphBuilder<E,N> {
        GraphBuilder(Graph::default())
    }

    pub fn directed(mut self, d: bool) -> GraphBuilder<E,N> {
        self.0.is_directed = d;
        self
    }
    pub fn weighted(mut self, w: bool) -> GraphBuilder<E,N> {
        self.0.is_weighted = w;
        self
    }
    pub fn tracking(mut self, t: bool) -> GraphBuilder<E,N> {
        self.0.is_tracking = t;
        self
    }
    pub fn build(mut self) -> Graph<E,N> {
        self.0
    }
}

// ----
// tests
// ----
#[cfg(test)]
mod tests {
    extern crate test;

    use ::{Graph,GraphSearch,GraphEdge,GraphNode,NodeBase,EdgeGuard};

    use super::{Backend};
    
    #[derive(Copy,Clone,PartialEq)]
    struct MyEdge {
        factor: f64,
    }
    impl GraphEdge for MyEdge {
        fn default() -> MyEdge { MyEdge { factor:0.0f64, } }
    }


    #[derive(Copy,Clone,PartialEq)]
    enum MyGuard {
        In,
        Out,
        Root,
    }

    #[derive(Clone,PartialEq)]
    struct MyNode {
        name: String,
        position: [f64;2],
        base: NodeBase,
        guard: MyGuard,
        kind: MyGuard,
    }
    impl GraphNode for MyNode {
        fn default() -> MyNode { MyNode { name: "".to_string(),
                                             position: [0.0,0.0],
                                             base: NodeBase::new(),
                                             guard: MyGuard::In,
                                             kind: MyGuard::Out }}

        fn get_base(&self) -> &NodeBase { &self.base }
        fn get_base_mut(&mut self) -> &mut NodeBase { &mut self.base }
        
        fn get_name(&self) -> &str { &self.name }
        fn get_position(&self) -> &[f64;2] { &self.position }

        fn set_name(&mut self, s: &str) { self.name = s.to_string() }
        fn set_position(&mut self, p: [f64;2]) { self.position = p }
    }

    // setup node-edge guards
    impl EdgeGuard for MyNode {
        fn guard(&self, other: &Self) -> bool {
            match (self.guard,other.kind) {
                (MyGuard::In,MyGuard::Root) => true,
                (MyGuard::In,MyGuard::Out) => true,
                _ => false,
            }
        }
    }

    

    #[test]
    fn test_basic_direct() {
        let mut graph: Graph<MyEdge,MyNode> = Graph::default();
        let mut nodes = vec!();
        for _ in 0..5 {
            nodes.push(graph.add());
        }

        let edge_def = MyEdge::default();
        
        assert!(graph.direct(&nodes[0],&nodes[1],edge_def));

        assert!(graph.direct(&nodes[3],&nodes[0],edge_def));
        assert!(graph.direct(&nodes[4],&nodes[3],edge_def));

        graph.remove(&nodes[2]);
        assert!(!graph.direct(&nodes[4],&nodes[2],edge_def));
        
        let n6 = graph.add();
        assert!(graph.direct(&n6,&nodes[3],edge_def));
        assert!(graph.direct(&nodes[0],&n6,edge_def));
    }


    
    #[test]
    fn test_basic_paths() {
        let mut graph: Graph<MyEdge,MyNode> = Graph::default();
        let mut nodes = vec!();
        for _ in 0..5 {
            nodes.push(graph.add());
        }

        let edge_def = MyEdge::default();
        
        graph.direct(&nodes[0],&nodes[1],edge_def);

        graph.direct(&nodes[3],&nodes[0],edge_def);
        graph.direct(&nodes[4],&nodes[3],edge_def);

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
        let mut graph: Graph<MyEdge,MyNode> = Graph::default();
        let mut nodes = vec!();
        for _ in 0..5 {
            nodes.push(graph.add());
        }

        let edge_def = MyEdge::default();
        
        graph.direct(&nodes[0],&nodes[1],edge_def);

        graph.direct(&nodes[3],&nodes[0],edge_def);
        graph.direct(&nodes[4],&nodes[3],edge_def);

        graph.remove(&nodes[2]);


        let r = graph.get_cycle(nodes[4]);
        assert_eq!(r.len(),0);
        
        let n6 = graph.add();
        graph.direct(&n6,&nodes[3],edge_def);
        graph.direct(&nodes[0],&n6,edge_def);

        let n7 = graph.add();
        graph.direct(&n7,&nodes[3],edge_def);
        graph.direct(&nodes[0],&n7,edge_def);
        
        let r = graph.get_cycle(nodes[4]);
        assert_eq!(r.len(),2);
    }

    #[test]
    fn test_basic_guard() {
        let mut graph: Graph<MyEdge,MyNode> = Graph::default();
        let mut nodes = vec!();
        for _ in 0..5 {
            nodes.push(graph.add());
        }

        // for this test: node kind is 'out' by default, and guard is 'in'
        graph.get_node_mut(&nodes[0]).unwrap().kind = MyGuard::Root;
        graph.get_node_mut(&nodes[2]).unwrap().kind = MyGuard::In;

        let edge_def = MyEdge::default();
        
        assert!(graph.direct(&nodes[1],&nodes[0],edge_def)); // out to root
        assert!(graph.direct(&nodes[2],&nodes[1],edge_def)); // in to out
        assert!(!graph.direct(&nodes[1],&nodes[2],edge_def)); // out to in
    }
}
