use iced::border;
use iced::widget::container;

pub fn rounded_border(_theme: &iced::Theme) -> container::Style {
    // let palette = theme.extended_palette(); // no idea what this was for

    container::Style {
        // background: Some(palette.background.weak.color.into()),
        background: None,
        border: border::rounded(2),
        ..container::Style::default()
    }
}
