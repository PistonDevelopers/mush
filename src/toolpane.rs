extern crate petgraph;

use conrod::{Ui, UiId, Button, Positionable, Sizeable, Labelable,Widget,WidgetId};
use conrod::color::{blue, light_grey, orange, dark_grey, red, white};
use conrod::{Colorable, Label, Split, WidgetMatrix, Floating};

use opengl_graphics::glyph_cache::GlyphCache;
use petgraph::Graph;

use graph::{UiGraph, EditableNode, EditableEdge};

pub struct ToolPane<N: EditableNode, E: EditableEdge, F> {
    ui_graph: UiGraph<N, E>,
    ui_id_offset: WidgetId,
    maybe_on_save: Option<F>
}

impl<N: EditableNode, E: EditableEdge, F: Fn(Graph<N, E>)> ToolPane<N, E, F> {

    pub fn new(offset: WidgetId, source_graph: &Graph<N, E>) -> ToolPane<N, E, F>
    {
        ToolPane {
            ui_graph: UiGraph::new(source_graph, offset + 7),
            ui_id_offset: offset,
            maybe_on_save: None,
        }
    }

    pub fn on_save(&mut self, on_save: F) {
        self.maybe_on_save = Some(on_save);
    }

    pub fn build_ui(&mut self, ui: &mut Ui<GlyphCache>) {
        let id_offset: WidgetId = self.ui_id_offset;

        // Construct our Canvas tree.
        Split::new(id_offset).flow_right(&[
            //Split::new(id_offset+1).color(light_grey()).pad(100.0),
            Split::new(id_offset+1).color(dark_grey())
                ]).set(ui);


        
        // we should use a canvas to place this appropriately
        Button::new()
            .top_left_of(id_offset+1)
            .label("Save")
            .dimensions(100.0, 40.0)
            .react(|| self.save())
            .set(id_offset+3, ui);

        Button::new()
            .right(5.0)
            .label("New Node")
            .dimensions(100.0, 40.0)
            .react(|| self.ui_graph.add_node())
            .set(id_offset+4, ui);

        self.ui_graph.build_ui(ui)
    }

    pub fn save(&self) where F: Fn(Graph<N, E>) {
        if let Some(ref on_save) = self.maybe_on_save {
            on_save(self.ui_graph.as_source_graph());
        }
    }
}
