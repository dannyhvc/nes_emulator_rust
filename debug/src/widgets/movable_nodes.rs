use iced::Renderer;
use iced_graphics::geometry::Renderer as GeometryRenderer;
use node_def::MovableNodeContainer;

pub type Element<'a, Message> =
    iced::Element<'a, Message, iced::Theme, iced::Renderer>;

pub mod node_def {
    use iced::advanced::widget::{
        tree::{State, Tag},
        Tree,
    };

    use super::{fallback::NullWidget, Element};

    /// Internal Container
    pub struct MovableNodeContainer<'a, Message> {
        pub nodes: Vec<Node<'a, Message>>,
        pub dragging: Option<usize>,
        pub drag_offset: iced::Vector,
        pub tree: Tree,
    }

    pub struct Node<'a, Message> {
        pub position: iced::Point,
        pub size: iced::Size,
        pub color: iced::Color,
        pub element: Element<'a, Message>,
        pub tree: iced::advanced::widget::Tree,
    }

    impl<'a, Message> Node<'a, Message> {
        pub fn new(
            position: iced::Point,
            size: iced::Size,
            color: iced::Color,
        ) -> Self {
            Self {
                position,
                size,
                color,
                element: Element::new(NullWidget),
                tree: Tree::empty(),
            }
        }
    }

    impl<'a, Message> MovableNodeContainer<'a, Message> {
        pub fn new() -> Self {
            let nodes = Vec::new();
            let tree = Tree {
                tag: Tag::stateless(),
                state: State::None,
                children: Vec::with_capacity(nodes.len()),
            };

            Self {
                nodes,
                dragging: None,
                drag_offset: iced::Vector::default(),
                tree,
            }
        }

        pub fn add_node(&mut self, node: Node<'a, Message>) {
            self.nodes.push(node);
            self.tree.children.push(Tree::empty());
        }

        pub fn new_node_from_widget(
            position: iced::Point,
            element: Element<'a, Message>,
        ) -> Node<'a, Message> {
            Node {
                position,
                size: iced::Size::new(100.0, 100.0),
                color: iced::Color::new(0.5, 0.5, 0.5, 1.0),
                element,
                tree: Tree::empty(),
            }
        }

        pub fn new_node_at(position: iced::Point) -> Node<'a, Message> {
            Node {
                position,
                size: iced::Size::new(50.0, 50.0),
                color: iced::Color::new(0.5, 0.5, 0.5, 1.0),
                element: Element::new(NullWidget),
                tree: Tree::empty(),
            }
        }

        pub fn node_at(&self, cursor_position: iced::Point) -> Option<usize> {
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
}

pub mod fallback {
    use iced::Renderer;

    #[derive(Default)]
    pub struct NullWidget;
    impl NullWidget {
        pub fn new() -> Self {
            NullWidget
        }
    }

    impl<Message, Theme> iced::advanced::Widget<Message, Theme, Renderer>
        for NullWidget
    {
        fn size(&self) -> iced::Size<iced::Length> {
            iced::Size::new(iced::Length::Shrink, iced::Length::Shrink)
        }

        fn layout(
            &self,
            _tree: &mut iced::advanced::widget::Tree,
            _renderer: &Renderer,
            _limits: &iced::advanced::layout::Limits,
        ) -> iced::advanced::layout::Node {
            iced::advanced::layout::Node::new(iced::Size::new(0.0, 0.0))
        }

        fn draw(
            &self,
            _tree: &iced::advanced::widget::Tree,
            _renderer: &mut Renderer,
            _theme: &Theme,
            _style: &iced::advanced::renderer::Style,
            _layout: iced::advanced::Layout<'_>,
            _cursor: iced::advanced::mouse::Cursor,
            _viewport: &iced::Rectangle,
        ) {
            // No drawing required
        }
    }
}

impl<'a, Message> iced::advanced::Widget<Message, iced::Theme, Renderer>
    for MovableNodeContainer<'a, Message>
{
    fn size(&self) -> iced::Size<iced::Length> {
        iced::Size::new(iced::Length::Fill, iced::Length::Fill)
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &iced::Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let mut frame =
            iced::widget::canvas::Frame::new(renderer, viewport.size());

        for (i, node) in self.nodes.iter().enumerate() {
            let node_tree = &tree.children[i];
            frame.fill_rectangle(node.position, node.size, node.color);
            node.element.as_widget().draw(
                node_tree, renderer, theme, style, layout, cursor, viewport,
            );
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
                        self.drag_offset = iced::Vector::new(
                            cursor_position.x.wrapping_sub(self.nodes[index].position.x),
                            cursor_position.y.wrapping_sub(self.nodes[index].position.y),
                        );
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
                            node.position = iced::Point::new(
                                position.x.wrapping_sub(self.drag_offset.x),
                                position.y.wrapping_sub(self.drag_offset.y),
                            );
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
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        let l = iced::Pixels(0.0f32);
        let size = limits
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .resolve(l, l, iced::Size::new(0.0f32, 0.0f32));

        let mut node_layout = Vec::with_capacity(self.nodes.len());

        for (i, node) in self.nodes.iter().enumerate() {
            let node_tree = &mut tree.children[i];
            let layout =
                node.element.as_widget().layout(node_tree, renderer, limits);
            node_layout.push(layout);
        }

        iced::advanced::layout::Node::with_children(size, node_layout)
    }
}

impl<'a, Message: 'a> Into<iced::Element<'a, Message, iced::Theme>>
    for MovableNodeContainer<'a, Message>
where
    Renderer: iced::advanced::Renderer,
{
    fn into(self) -> iced::Element<'a, Message, iced::Theme, iced::Renderer> {
        iced::Element::new(self)
    }
}
