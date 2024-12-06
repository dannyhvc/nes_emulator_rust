#![allow(clippy::unwrap_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]

use super::types::*;
use border::Radius;
use iced::widget::*;
use iced::*;
use iced_aw::{
    menu::{self, *},
    menu_bar, menu_items, quad,
    widget::InnerBounds,
};

use iced::widget::column as col;

/// # menu_bar_example
///
/// Helps with conceptualizing how the menu tree works for iced.
pub fn base<'a>() -> impl Into<Element<'a, DebuggerMsg>> {
    // closure for making a small seperator line in the menu
    let seperator = || quad::Quad {
        quad_color: Color::from([0.5; 3]).into(),
        quad_border: Border {
            radius: Radius::new(4.0),
            ..Default::default()
        },
        inner_bounds: InnerBounds::Ratio(0.98, 0.2),
        height: Length::Fixed(20.0),
        ..Default::default()
    };

    #[rustfmt::skip]
    let mb = menu_bar!((
        Button::new("Widgets"),
        Menu::new(menu_items!(
            ( Button::new("You can use any widget").on_press(DebuggerMsg::Start).width(Length::Fill) )
            ( Button::new("as a menu item").on_press(DebuggerMsg::Start).width(Length::Fill) )
            ( seperator() )
            ( Button::new("Labeled Separator").on_press(DebuggerMsg::Start).width(Length::Fill) )
            ( Button::new("Dot Separator").on_press(DebuggerMsg::Start).width(Length::Fill) )
            ( Button::new("Item").on_press(DebuggerMsg::Start).width(Length::Fill) )
            ( Button::new("Item").on_press(DebuggerMsg::Start).width(Length::Fill) )
        ))
        .width(240.0)
    ))
    .draw_path(menu::DrawPath::Backdrop)
    .style(|theme, status| menu::Style {
        path_border: Border {
            radius: Radius::new(6.0),
            ..Default::default()
        },
        ..primary(theme, status)
    });

    #[rustfmt::skip]
    let main_col = col! {
        mb.padding(10.0),
        row![
            button("reset").on_press(DebuggerMsg::End).padding(Padding {
                top: 10.0,
                right: 100.0,
                bottom: 10.0,
                left: 100.0,
            }),
            button("clock").on_press(DebuggerMsg::End).padding(Padding {
                top: 10.0,
                right: 100.0,
                bottom: 10.0,
                left: 100.0,
            }),
        ]
    };

    let main_col = col!(main_col).align_x(Alignment::Center);

    // let main_row = row!(main_col);
    main_col
}
