extern crate conrod;
extern crate uuid;
extern crate rustc_serialize;
extern crate find_folder;
extern crate piston_window;
    
pub use toolpane::{ToolPane};
pub use backend::{Backend,Graph,GraphSearch,GraphEdge,GraphNode,NodeBase,EdgeGuard,Eid,Nid};
pub use uigraph::{UiGraph,UiBase,UiNode};

pub mod toolpane;
pub mod backend;
pub mod uigraph;

pub const MAX_NODES: usize = 1000;
pub const MAX_CONN_OUT: usize = 100;
pub const MAX_CONN_IN: usize = 100;
