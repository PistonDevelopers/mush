extern crate petgraph;

use petgraph::{
    Graph,
    Bfs,
    Dfs,
    Incoming,
    Outgoing,
    Directed,
    Undirected,
};
use petgraph::graph::NodeIndex;

#[derive(Debug)]
struct State {
    flag: bool,
}

pub fn graphs () {
    let mut g = Graph::new();
    let a = g.add_node(State { flag: true });
    let b = g.add_node(State { flag: true });
    let c = g.add_node(State { flag: false });
    g.add_edge(a,b,1);
    g.add_edge(b,c,1);
    //g.add_edge(c,a,1);
    println!("{:?}",g.find_edge(a,b));
    println!("{:?}",g.find_edge(a,c));
    println!("{:?}",g.neighbors(a).collect::<Vec<_>>());
    assert!(!petgraph::algo::is_cyclic_directed(&g));
    //println!("{:?}",petgraph::algo::connected_components(&g));
    g[a].flag = false;
    println!("{:?}",g[a]);
}

