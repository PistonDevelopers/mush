use ::{EditableNode,EditableEdge};

#[derive(Debug)]
pub struct NodeState {
    position: [f64; 2]
}
impl NodeState {
    pub fn new(pos:[f64;2]) -> Self {
        NodeState { position:pos }
    }
}

#[derive(Debug)]
pub struct Edge;
impl EditableEdge for Edge {
    fn default() -> Self { Edge }
}

impl EditableNode for NodeState {
    fn get_position(&self) -> [f64; 2] {
        self.position
    }

    fn set_position(&mut self, position: [f64; 2]) {
        self.position = position;
    }

    fn default() -> Self {
        NodeState { position: [0.0, 0.0] }
    }
}

/// instrinsics to get base nodestate from custom node
pub trait Intrinsics {
    fn get_base(&self) -> &NodeState;
    fn get_base_mut(&mut self) -> &mut NodeState;
    fn default() -> Self;
}
