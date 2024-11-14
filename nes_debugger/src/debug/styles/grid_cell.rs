pub struct BorderedContainerStyle;

impl Into<iced::theme::Container> for BorderedContainerStyle {
    fn into(self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(self))
    }
}

impl iced::widget::container::StyleSheet for BorderedContainerStyle {
    type Style = iced::Theme;

    fn appearance(
        &self,
        _style: &Self::Style,
    ) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            border: iced::Border {
                color: iced::Color::from_rgb(0.0, 0.0, 0.0),
                width: 2.0,
                radius: iced::border::Radius::from([1.0; 4]),
            },

            ..iced::widget::container::Appearance::default()
        }
    }
}
