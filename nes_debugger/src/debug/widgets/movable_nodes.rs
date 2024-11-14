use iced::{Element, Renderer};
use iced_graphics::geometry::Renderer as GeometryRenderer;

pub struct MovableNodes<'a, Message, Theme, Renderer> {
    pub nodes: Vec<Node<'a, Message, Theme, Renderer>>,
    dragging: Option<usize>,
}

pub struct Node<'a, Message, Theme, Renderer> {
    position: iced::Point,
    size: iced::Size,
    color: iced::Color,
    child: iced::Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme, Renderer> Node<'a, Message, Theme, Renderer> {
    pub fn new(
        position: iced::Point,
        size: iced::Size,
        color: iced::Color,
        child: Element<'a, Message, Theme, Renderer>,
    ) -> Self {
        Self {
            position,
            size,
            color,
            child,
        }
    }
}

impl<'a, Message, Theme> MovableNodes<'a, Message, Theme, Renderer> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            dragging: None,
        }
    }

    pub fn push(mut self, node: Node<'a, Message, Theme, Renderer>) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn new_node_at(
        position: iced::Point,
        child: iced::Element<'a, Message, Theme, Renderer>,
    ) -> Node<'a, Message, Theme, Renderer> {
        Node {
            position,
            size: iced::Size::new(50.0, 50.0),
            color: iced::Color::new(0.5, 0.5, 0.5, 1.0),
            child,
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

impl<'a, Message, Theme> iced::advanced::Widget<Message, Theme, iced::Renderer>
    for MovableNodes<'a, Message, Theme, iced::Renderer>
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
                            let rel_offset = iced::Point {
                                x: (cursor_position.x - node.position.x).abs(),
                                y: (cursor_position.y - node.position.y).abs(),
                            };
                            // let node_abs = position - rel_offset;
                            // node.position = position - node_abs;

                            node.position = position;
                            // Trigger a redraw
                            shell.invalidate_layout();
                        }
                        return iced::advanced::graphics::core::event::Status::Captured;
                    }
                }

                iced::Event::Keyboard(e) => {
                    println!("{e:?}");
                    return iced::advanced::graphics::core::event::Status::Ignored;
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
        // let mut root_node = iced::advanced::layout::Node::new(limits.max());
        // for node in &self.nodes {
        //     let child_node = node.child.as_widget().layout(
        //         &mut iced::advanced::widget::Tree::empty(),
        //         _renderer,
        //         &limits.shrink(node.size),
        //     );
        //     let child_layout = child_node
        //         .move_to(iced::Point::new(node.position.x, node.position.y));
        //     root_node = iced::advanced::layout::Node::with_children(
        //         root_node.size(),
        //         vec![child_layout],
        //     );
        // }

        let l = iced::Pixels(1.0f32);
        let size = limits
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .resolve(l, l, iced::Size::new(0.0f32, 0.0f32));

        iced::advanced::layout::Node::new(size)
        // root_node
    }
}

impl<'a, Message: 'a, Theme: 'a>
    Into<iced::Element<'a, Message, Theme, iced::Renderer>>
    for MovableNodes<'a, Message, Theme, iced::Renderer>
{
    fn into(self) -> iced::Element<'a, Message, Theme, iced::Renderer> {
        iced::Element::<'a, Message, Theme, iced::Renderer>::new(self)
    }
}
