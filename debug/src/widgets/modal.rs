use iced::widget::{container, Column, Space, Stack};
use iced::{Alignment, Color, Element, Length, Shadow, Theme, Vector};

pub struct Modal<'a, Message> {
    is_visible: bool,
    content: Element<'a, Message>,
    overlay: Element<'a, Message>,
}

impl<'a, Message: Clone + 'a> Modal<'a, Message> {
    pub fn new<F>(
        is_visible: bool,
        content: impl Into<Element<'a, Message>>,
        overlay_builder: F,
    ) -> Self
    where
        F: FnOnce() -> Element<'a, Message>,
    {
        Self {
            is_visible,
            content: content.into(),
            overlay: overlay_builder(),
        }
    }

    pub fn view(self) -> Element<'a, Message> {
        if self.is_visible {
            let Modal {
                is_visible: _,
                content,
                overlay,
            } = self;
            let overlay = Self::overlay_layer(overlay);
            Stack::new().push(content).push(overlay).into()
        } else {
            self.content
        }
    }

    /// Builds the semi-transparent background + centered overlay
    fn overlay_layer(overlay: Element<'a, Message>) -> Element<'a, Message> {
        container(
            Column::new()
                .push(Space::with_height(Length::Fill)) // top spacing
                .push(Self::overlay_card(overlay)) // the modal card
                .push(Space::with_height(Length::Fill)) // bottom spacing
                .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(Self::background_style)
        .into()
    }

    /// Builds the card holding your overlay content
    fn overlay_card(overlay: Element<'a, Message>) -> Element<'a, Message> {
        container(overlay)
            .padding(20)
            .style(Self::card_style)
            .into()
    }

    /// Style for the dark semi-transparent background
    fn background_style(_theme: &Theme) -> container::Style {
        container::Style {
            background: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.5).into()),
            ..Default::default()
        }
    }

    /// Style for the actual modal content card
    fn card_style(_theme: &Theme) -> container::Style {
        container::Style {
            background: Some(Color::from_rgb(1.0, 1.0, 1.0).into()),
            shadow: Shadow {
                color: Color::WHITE,
                blur_radius: 5.0,
                offset: Vector::new(0.0, 1.0),
            },
            ..Default::default()
        }
    }
}
