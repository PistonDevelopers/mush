extern crate petgraph;

use conrod::{Ui, UiId, Button, Positionable, Sizeable, Labelable,Widget};
use opengl_graphics::glyph_cache::GlyphCache;
use petgraph::Graph;

use graph::{UiGraph, EditableNode, EditableEdge};

pub struct ToolPane<N: EditableNode, E: EditableEdge, F> {
    ui_graph: UiGraph<N, E>,
    ui_id_offset: UiId,
    maybe_on_save: Option<F>
}

impl<N: EditableNode, E: EditableEdge, F: Fn(Graph<N, E>)> ToolPane<N, E, F> {

    pub fn new(offset: UiId, source_graph: &Graph<N, E>) -> ToolPane<N, E, F>
    {
        ToolPane {
            ui_graph: UiGraph::new(source_graph, offset + 2),
            ui_id_offset: offset,
            maybe_on_save: None,
        }
    }

    pub fn on_save(&mut self, on_save: F) {
        self.maybe_on_save = Some(on_save);
    }

    pub fn build_ui(&mut self, ui: &mut Ui<GlyphCache>) {
        let id_offset = self.ui_id_offset;

        // we should use a canvas to place this appropriately
        Button::new()
            .xy(-1.0*ui.win_w/2.0+50.0,ui.win_h/2.0-20.0)
            .label("Save")
            .dimensions(100.0, 40.0)
            .react(|| self.save())
            .set(id_offset, ui);

        Button::new()
            .xy(-1.0*ui.win_w/2.0+150.0, ui.win_h/2.0-20.0)
            .label("New Node")
            .dimensions(100.0, 40.0)
            .react(|| self.ui_graph.add_node())
            .set(id_offset + 1, ui);

        self.ui_graph.build_ui(ui)
    }

    pub fn save(&self) where F: Fn(Graph<N, E>) {
        if let Some(ref on_save) = self.maybe_on_save {
            on_save(self.ui_graph.as_source_graph());
        }
    }
}
