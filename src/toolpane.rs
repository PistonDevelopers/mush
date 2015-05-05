extern crate petgraph;

use conrod::{Ui, UiId, Button, Positionable, Sizeable, Labelable};
use opengl_graphics::glyph_cache::GlyphCache;
use petgraph::Graph;

use graph::{UiGraph, EditableNode, EditableEdge};

pub struct ToolPane<N: EditableNode, E: EditableEdge> {
    ui_graph: UiGraph<N, E>,
}

impl<N: EditableNode, E: EditableEdge> ToolPane<N, E> {

    pub fn new(offset: UiId, source_graph: &Graph<N, E>) -> ToolPane<N, E> {
        ToolPane {
            ui_graph: UiGraph::new(source_graph, offset)
        }
    }

    pub fn build_ui(&mut self, ui: &mut Ui<GlyphCache>) {
        // we should use a canvas to place this appropriately
        Button::new()
            .xy(-1.0*ui.win_w/2.0+50.0,ui.win_h/2.0-20.0)
            .label("New Node")
            .dimensions(100.0,40.0)
            .react(|| self.ui_graph.add_node())
            .set(0,ui);

        self.ui_graph.build_ui(ui)
    }
}
