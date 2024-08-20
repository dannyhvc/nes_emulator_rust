use iced::Renderer;
use iced_graphics::geometry::Renderer as GeometryRenderer;

#[derive(Clone)]
pub struct MovableNodes {
    nodes: Vec<Node>,
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
// impl IcedGraphicsGeometryRenderer for iced::Renderer {
//     type Geometry = iced_graphics::geometry;

//     fn draw(&mut self, layers: Vec<Self::Geometry>) {
//         todo!()
//     }
// }

impl MovableNodes {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            dragging: None,
        }
    }

    fn new_node_at(position: iced::Point) -> Node {
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
// where
//     Renderer: iced::advanced::renderer::Renderer
//         + iced::advanced::graphics::geometry::Renderer,
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
        _event: iced::Event,
        _layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn iced::advanced::Clipboard,
        _shell: &mut iced::advanced::Shell<'_, Message>,
        _viewport: &iced::Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        iced::advanced::graphics::core::event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        _state: &iced::advanced::widget::Tree,
        _layout: iced::advanced::Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
        _renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        iced::advanced::mouse::Interaction::Idle
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        todo!()
    }
}
