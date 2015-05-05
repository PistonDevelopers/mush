use conrod::{CustomWidget, CustomWidgetState};
use conrod::{Color, Colorable};
use conrod::Frameable;
use conrod::{FontSize, Labelable};
use conrod::{Mouse, MouseButtonState};
use conrod::{Depth, Dimensions, Position, Positionable, HorizontalAlign, VerticalAlign, Sizeable};
use conrod::{UiId, Ui};
use conrod::{WidgetUpdate};
use conrod::CharacterCache;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum NodeState {
    Normal, //expanded, static position; todo: rename to static?
    NormalCollapsed, // alternative to enum states for collapse, we can use collapse state as: Normal(true) or track a separate collapse state enum
    Dragging,
}

fn get_new_state(is_over: bool, prev: NodeState, mouse: Mouse) -> NodeState {
    use self::NodeState::{Normal,NormalCollapsed, Dragging};
    //buttonstate is either up or down, if there was a 'released' state we could track this easily
    match (is_over, prev, (mouse.left,mouse.right)) {
        (true,  _, (MouseButtonState::Down,_)) => Dragging,
        (true, Normal, (_,MouseButtonState::Down)) => NormalCollapsed, //this tracks bad because the state flips too quickly, a delay or mouse-release would fix this
        (true, NormalCollapsed, (_,MouseButtonState::Down)) => Normal,
        
        (false, Dragging, (MouseButtonState::Down,_)) => Dragging,
        
        (false, Dragging,(MouseButtonState::Up,_)) => Normal,
        (true, Dragging,(MouseButtonState::Up,_)) => Normal,
        _ => prev,
    }
}

impl NodeState {
    /// Alter the widget color depending on the state.
    fn color(&self, color: Color) -> Color {
        match *self {
            NodeState::Normal => color,
            NodeState::NormalCollapsed => color.complement(),
            NodeState::Dragging => color.highlighted(),
        }
    }
}

impl CustomWidgetState for NodeState {
    fn init() -> Self { NodeState::Normal }
    fn matches(&self, _other: &NodeState) -> bool { true }
}

#[derive(Clone, Debug)]
pub struct Node<'a> {
    pos: Position,
    dim: Dimensions,
    depth: Depth,
    maybe_label: Option<&'a str>,
    style: Style,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Style {
    pub maybe_color: Option<Color>,
    pub maybe_frame: Option<f64>,
    pub maybe_frame_color: Option<Color>,
    pub maybe_label_color: Option<Color>,
    pub maybe_label_font_size: Option<u32>,
}

impl Style {
    pub fn new() -> Style {
        Style {
            maybe_color: None,
            maybe_frame: None,
            maybe_frame_color: None,
            maybe_label_color: None,
            maybe_label_font_size: None,
        }
    }
}

impl<'a> Node<'a> {
    pub fn new() -> Node<'a> {
        Node {
            pos: Position::Absolute(0.0, 0.0),
            dim: [64.0, 64.0],
            depth: 0.0,
            maybe_label: None,
            style: Style::new(),
        }
    }
}

impl<'a> CustomWidget for Node<'a> {

    type State = NodeState;

    fn update<C>(self, state: NodeState, ui_id: UiId, ui: &mut Ui<C, Node<'a>>) -> WidgetUpdate<NodeState>
        //where C: CharacterCache //I had to comment this out, not sure why it wouldn't compile for me
    {

        use elmesque::form::{collage, rect, text};
        use conrod::utils::is_over_rect;

        let dim = self.dim;
        let mut xy = ui.get_xy(self.pos, dim, ui.theme.align.horizontal, ui.theme.align.vertical);

        let mouse = ui.get_mouse_state(ui_id).relative_to(xy);
        let is_over = is_over_rect([0.0, 0.0], mouse.xy, dim);
        let new_state = get_new_state(is_over, state, mouse);

        match new_state {
            NodeState::Dragging => xy = ui.mouse.xy, //this isn't exact, fixme!
            _ => (),
        }
        
        // TODO - use color, frame attributes from style
        // TODO - refactor element building into a 'new_element' function/method (see builtin Conrod widgets)

        let frame_form = rect(dim[0], dim[1]).filled(ui.theme.frame_color);

        let color = new_state.color(ui.theme.shape_color);

        let frame_w = ui.theme.frame_width;
        let (inner_w, inner_h) = (dim[0] - frame_w * 2.0, dim[1] - frame_w * 2.0);
        let inner_form = rect(inner_w, inner_h).filled(color);

        let maybe_label_form = self.maybe_label.map(|label_text| {
            use elmesque::text::Text;
            let text_color = ui.theme.label_color;
            let size = ui.theme.font_size_medium;
            text(Text::from_string(label_text.to_string()).color(text_color).height(size as f64))
                .shift(xy[0].floor(), xy[1].floor())
        });

        let form_chain = Some(frame_form).into_iter()
            .chain(Some(inner_form).into_iter())
            .map(|form| form.shift(xy[0], xy[1]))
            .chain(maybe_label_form.into_iter());

        let element = collage(dim[0] as i32, dim[1] as i32, form_chain.collect());

        WidgetUpdate {
            new_state: new_state,
            xy: xy,
            depth: self.depth,
            element: element
        }
    }
}

impl<'a> Labelable<'a> for Node<'a> {
    fn label(mut self, text: &'a str) -> Self {
        self.maybe_label = Some(text);
        self
    }

    fn label_color(mut self, color: Color) -> Self {
        self.style.maybe_label_color = Some(color);
        self
    }

    fn label_font_size(mut self, size: FontSize) -> Self {
        self.style.maybe_label_font_size = Some(size);
        self
    }
}

// TODO - determine if it even makes sense to implement Positionable,
// Seems to me that we would only ever need to support positioning relative to some container (ie 'toolpane'?)
impl<'a> Positionable for Node<'a> {
    fn position(mut self, pos: Position) -> Node<'a> {
        self.pos = pos;
        self
    }

    #[inline]
    fn horizontal_align(self, _h_align: HorizontalAlign) -> Self {
        self
    }

    #[inline]
    fn vertical_align(self, _v_align: VerticalAlign) -> Self {
        self
    }
}

impl<'a> Sizeable for Node<'a> {
    #[inline]
    fn width(self, w: f64) -> Self {
        let h = self.dim[1];
        Node { dim: [w, h], ..self }
    }
    #[inline]
    fn height(self, h: f64) -> Self {
        let w = self.dim[0];
        Node { dim: [w, h], ..self }
    }
}

impl<'a> Colorable for Node<'a> {
    fn color(mut self, color: Color) -> Self {
        self.style.maybe_color = Some(color);
        self
    }
}

impl<'a> Frameable for Node<'a> {
    fn frame(mut self, width: f64) -> Self {
        self.style.maybe_frame = Some(width);
        self
    }
    fn frame_color(mut self, color: Color) -> Self {
        self.style.maybe_frame_color = Some(color);
        self
    }
}
