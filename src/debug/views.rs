#![allow(clippy::unwrap_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]

use super::{data_preporation, traits::*, types::*};
use border::Radius;
use iced::widget::*;
use iced::*;
use iced_aw::{
    menu::{self, *},
    menu_bar, menu_items, quad,
    widget::InnerBounds,
};

use super::types::DebuggerState;
use iced::widget::column as col;
use widget::Scrollable;
/// # seperator
///
/// horizontal grey seperator component
fn seperator() -> quad::Quad {
    quad::Quad {
        quad_color: Color::from([0.5; 3]).into(),
        quad_border: Border {
            radius: Radius::new(4.0),
            ..Default::default()
        },
        inner_bounds: InnerBounds::Ratio(0.98, 0.2),
        height: Length::Fixed(20.0),
        ..Default::default()
    }
}

/// # menu_bar_example
///
/// The main entry point component for the debugger
pub fn base<'a>(state: &DebuggerState) -> impl Into<Element<'a, DebuggerMsg>> {
    // closure for making a small seperator line in the menu

    #[rustfmt::skip]
    let mb = menu_bar!((
        Button::new("View"),
        Menu::new(menu_items!(
            ( Button::new("CPU").on_press(DebuggerMsg::Start).width(Length::Fill) )
            ( Button::new("PPU").on_press(DebuggerMsg::Start).width(Length::Fill) )
            ( Button::new("APU").on_press(DebuggerMsg::Start).width(Length::Fill) )
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

    let mut main_col = col![mb.padding(10.0)];
    main_col = main_col
        .push(
            col![row![row![
                button("reset")
                    .on_press(DebuggerMsg::Start)
                    .padding(Padding {
                        top: 10.0,
                        right: 45.0,
                        bottom: 10.0,
                        left: 45.0,
                    }),
                button("clock")
                    .on_press(DebuggerMsg::Start)
                    .padding(Padding {
                        top: 10.0,
                        right: 45.0,
                        bottom: 10.0,
                        left: 45.0,
                    }),
                button("show op").on_press(DebuggerMsg::Start).padding(
                    Padding {
                        top: 10.0,
                        right: 45.0,
                        bottom: 10.0,
                        left: 45.0,
                    }
                ),
                button("show am").on_press(DebuggerMsg::Start).padding(
                    Padding {
                        top: 10.0,
                        right: 45.0,
                        bottom: 10.0,
                        left: 45.0,
                    }
                ),
                button("show flags").on_press(DebuggerMsg::Start).padding(
                    Padding {
                        top: 10.0,
                        right: 45.0,
                        bottom: 10.0,
                        left: 45.0,
                    }
                ),
            ]
            .spacing(10)]]
            .align_x(Alignment::Center)
            .width(Length::Fill),
        )
        .width(Length::Fill);

    let ram_as_label_widget: Vec<
        Element<'a, DebuggerMsg, iced::Theme, iced::Renderer>,
    > = state
        .bus
        .ram()
        .iter()
        .map(|b| Element::from(text("b")))
        .collect();

    state.first_n_words();

    Container::new(Scrollable::new(Column::from_vec(ram_as_label_widget)));

    // main_col = main_col.push(Scrollable::new(col![state.bus.ram()]));

    main_col
}
