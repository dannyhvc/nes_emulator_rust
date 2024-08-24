use iced::Renderer;
use iced_graphics::geometry::Renderer as GeometryRenderer;

#[derive(Clone)]
pub struct MovableNodes {
    pub nodes: Vec<Node>,
    dragging: Option<usize>,
}

#[derive(Clone)]
pub struct Node {
    position: iced::Point,
    size: iced::Size,
    color: iced::Color,
}

impl Node {
    pub fn new(
        position: iced::Point,
        size: iced::Size,
        color: iced::Color,
    ) -> Self {
        Self {
            position,
            size,
            color,
        }
    }
}

impl MovableNodes {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            dragging: None,
        }
    }

    pub fn new_node_at(position: iced::Point) -> Node {
        Node {
            position,
            size: iced::Size::new(50.0, 50.0),
            color: iced::Color::new(0.5, 0.5, 0.5, 1.0),
        }
    }

    fn node_at(&self, cursor_position: iced::Point) -> Option<usize> {
        self.nodes.iter().position(|node| {
            let node_bounds = iced::Rectangle {
                x: node.position.x,
                y: node.position.y,
                width: node.size.width,
                height: node.size.height,
            };
            node_bounds.contains(cursor_position)
        })
    }
}

// pub trait MoveableNodeRenderer {}

impl<Message, Theme> iced::advanced::Widget<Message, Theme, Renderer>
    for MovableNodes
{
    fn size(&self) -> iced::Size<iced::Length> {
        iced::Size::new(iced::Length::Fill, iced::Length::Fill)
    }

    fn draw(
        &self,
        _tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        _layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let mut frame =
            iced::widget::canvas::Frame::new(renderer, viewport.size());

        for node in &self.nodes {
            frame.fill_rectangle(node.position, node.size, node.color);
        }

        let geometry = vec![frame.into_geometry()];
        renderer.draw(geometry);
    }

    fn on_event(
        &mut self,
        _state: &mut iced::advanced::widget::Tree,
        event: iced::Event,
        _layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        if let Some(cursor_position) = cursor.position_in(*viewport) {
            match event {
                // Handle mouse button press
                iced::Event::Mouse(iced::mouse::Event::ButtonPressed(
                    iced::mouse::Button::Left,
                )) => {
                    if let Some(index) = self.node_at(cursor_position) {
                        // Start dragging the node
                        self.dragging = Some(index);
                        return iced::advanced::graphics::core::event::Status::Captured;
                    }
                }

                // Handle mouse button release
                iced::Event::Mouse(iced::mouse::Event::ButtonReleased(
                    iced::mouse::Button::Left,
                )) => {
                    // Stop dragging the node
                    self.dragging = None;
                    return iced::advanced::graphics::core::event::Status::Captured;
                }

                // Handle mouse movement
                iced::Event::Mouse(iced::mouse::Event::CursorMoved {
                    position,
                }) => {
                    if let Some(dragging_index) = self.dragging {
                        // Update the position of the dragged node
                        if let Some(node) = self.nodes.get_mut(dragging_index) {
                            node.position = position;
                            // Trigger a redraw
                            shell.invalidate_layout();
                        }
                        return iced::advanced::graphics::core::event::Status::Captured;
                    }
                }

                _ => {}
            }
        }

        iced::advanced::graphics::core::event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        _state: &iced::advanced::widget::Tree,
        _layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        // Check if the cursor is over any node
        if let Some(cursor_position) = cursor.position_in(*viewport) {
            if self.node_at(cursor_position).is_some() {
                // If the cursor is over a node, indicate a grab interaction
                return iced::advanced::mouse::Interaction::Grab;
            }
        }

        // Default to idle interaction if not over any node
        iced::advanced::mouse::Interaction::Idle
    }

    fn layout(
        &self,
        _tree: &mut iced::advanced::widget::Tree,
        _renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let l = iced::Pixels(0.0f32);
        let size = limits
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .resolve(l, l, iced::Size::new(0.0f32, 0.0f32));

        iced::advanced::layout::Node::new(size)
    }
}

impl<'a, Message, Theme> Into<iced::Element<'a, Message, Theme>>
    for MovableNodes
where
    Renderer: iced::advanced::Renderer,
{
    fn into(self) -> iced::Element<'a, Message, Theme, iced::Renderer> {
        iced::Element::new(self)
    }
}
