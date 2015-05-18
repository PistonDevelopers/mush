extern crate mush;

use mush::{Graph,GraphSearch,GraphEdge,GraphNode,NodeBase,EdgeGuard,Backend};

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
    //type P = [f64;2];
    
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

#[test]
fn test_basic_iter() {
    let mut graph: Graph<MyEdge,MyNode> = Graph::default();
    let mut nodes = vec!();
    for _ in 0..5 {
        nodes.push(graph.add());
    }

    
    graph.with_nodes(|n| {let name = &n.name;} );
    graph.with_nodes_mut(|n| n.position[0] += 0.0);
}
