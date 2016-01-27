use conrod::color::{Color, Colorable};
use conrod::FontSize;
use conrod::{Depth, Dimensions, Point, Position, Positionable,Sizeable};
use conrod::Theme;
use conrod::{GlyphCache, UserInput};
use conrod::{Widget,WidgetState};


/// Displays some given text centred within a rectangle.
#[derive(Clone, Debug)]
pub struct UiEdge<'a> {
    text: &'a str,
    from: Position,
    to: Position,
    style: Style,
    //maybe_h_align: Option<HorizontalAlign>,
    //maybe_v_align: Option<VerticalAlign>,
}

/// The styling for a Label's renderable Element.
#[allow(missing_docs, missing_copy_implementations)]
#[derive(Clone, Debug, PartialEq, RustcEncodable, RustcDecodable)]
pub struct Style {
    maybe_font_size: Option<FontSize>,
    maybe_color: Option<Color>,
}

/// The state to be stored between updates for the Label.
#[derive(Clone, Debug, PartialEq)]
pub struct EdgeState(String);


impl<'a> UiEdge<'a> {

    /// Construct a new widget.
    pub fn new(text: &'a str,from: Position, to: Position) -> UiEdge<'a> {
        UiEdge {
            text: text,
            from: from,
            to: to,
            style: Style::new(),
            //maybe_h_align: None,
            //maybe_v_align: None,
        }
    }

    /// Set the font size for the label.
    #[inline]
    pub fn font_size(mut self, size: FontSize) -> UiEdge<'a> {
        self.style.maybe_font_size = Some(size);
        self
    }

}


impl<'a> Widget for UiEdge<'a> {
    type State = EdgeState;
    type Style = Style;
    fn unique_kind(&self) -> &'static str { "Label" }
    fn init_state(&self) -> Self::State { EdgeState(String::new()) }
    fn style(&self) -> Style { self.style.clone() }
    fn canvas_id(&self) -> Option<CanvasId> { None }

    /// Update the state of the Label.
    fn update<'b, C>(self,
                     prev_state: &WidgetState<EdgeState>,
                     _xy: Point,
                     _dim: Dimensions,
                     _input: UserInput<'b>,
                     _style: &Style,
                     _theme: &Theme,
                     _glyph_cache: &GlyphCache<C>) -> Option<EdgeState>
        where
            C: CharacterCache,
    {
        let WidgetState { state: EdgeState(ref string), .. } = *prev_state;
        if &string[..] != self.text { Some(EdgeState(self.text.to_string())) } else { None }
    }

    /// Construct an Element for the Label.
    fn draw<C>(new_state: &WidgetState<EdgeState>,
               style: &Style,
               theme: &Theme,
               _glyph_cache: &GlyphCache<C>) -> Element
        where
            C: CharacterCache,
    {
        let WidgetState { state: EdgeState(ref string), dim, xy, .. } = *new_state;
        let size = style.font_size(theme);
        let color = style.color(theme);
        let form = text(Text::from_string(string.clone())
                            .color(color)
                            .height(size as f64)).shift(xy[0].floor(), xy[1].floor());
        collage(dim[0] as i32, dim[1] as i32, vec![form])
    }
    
}


impl Style {

    /// Construct the default Style.
    pub fn new() -> Style {
        Style {
            maybe_color: None,
            maybe_font_size: None,
        }
    }

    /// Get the Color for an Element.
    pub fn color(&self, theme: &Theme) -> Color {
        self.maybe_color.unwrap_or(theme.label_color)
    }

    /// Get the label font size for an Element.
    pub fn font_size(&self, theme: &Theme) -> FontSize {
        self.maybe_font_size.unwrap_or(theme.font_size_medium)
    }

}


impl<'a> Colorable for UiEdge<'a> {
    fn color(mut self, color: Color) -> Self {
        self.style.maybe_color = Some(color);
        self
    }
}

impl<'a> Positionable for UiEdge<'a> {
    fn position(mut self, pos: Position) -> Self {
        //self.from = pos;
        self
    }
    fn get_position(&self) -> Position { self.from }
    #[inline]
    fn horizontal_align(self, h_align: HorizontalAlign) -> Self {
        UiEdge { maybe_h_align: Some(h_align), ..self }
    }
    #[inline]
    fn vertical_align(self, v_align: VerticalAlign) -> Self {
        UiEdge { maybe_v_align: Some(v_align), ..self }
    }
    fn get_horizontal_align(&self, theme: &Theme) -> HorizontalAlign {
        self.maybe_h_align.unwrap_or(theme.align.horizontal)
    }
    fn get_vertical_align(&self, theme: &Theme) -> VerticalAlign {
        self.maybe_v_align.unwrap_or(theme.align.vertical)
    }
    fn depth(mut self, depth: Depth) -> Self {
        self
    }
    fn get_depth(&self) -> Depth { 0.0 }
}

impl<'a> Sizeable for UiEdge<'a> {
    fn width(self, _w: f64) -> Self { self }
    fn height(mut self, h: f64) -> Self {
        self.style.maybe_font_size = Some(h as FontSize);
        self
    }
    fn get_width<C: CharacterCache>(&self, theme: &Theme, glyph_cache: &GlyphCache<C>) -> f64 {
        glyph_cache.width(self.style.font_size(theme), self.text)
    }
    fn get_height(&self, theme: &Theme) -> f64 { self.style.font_size(theme) as f64 }
}

