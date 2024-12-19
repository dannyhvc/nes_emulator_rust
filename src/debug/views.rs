#![allow(clippy::unwrap_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]

use super::{traits::*, types::*};
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

    let main_pane = col![mb.padding(10.0)];
    let debug_buttons = debug_button_bar();

    let memory_scoller = memory_scoller(state);

    main_pane
        .push(debug_buttons)
        .push(memory_scoller)
        .width(Length::Fill)
}

/// # debug_button_bar
///
///
fn debug_button_bar<'a>() -> impl Into<Element<'a, DebuggerMsg>> {
    let padding = Padding {
        top: 10.0,
        right: 45.0,
        bottom: 10.0,
        left: 45.0,
    };

    col![row![row![
        button("reset")
            .on_press(DebuggerMsg::Start)
            .padding(padding),
        button("clock")
            .on_press(DebuggerMsg::Start)
            .padding(padding),
        button("show op")
            .on_press(DebuggerMsg::Start)
            .padding(padding),
        button("show am")
            .on_press(DebuggerMsg::Start)
            .padding(padding),
        button("show flags")
            .on_press(DebuggerMsg::Start)
            .padding(padding),
    ]
    .spacing(10)]]
    .align_x(Alignment::Center)
    .width(Length::Fill)
}

/// # memory_scoller
///
///
fn memory_scoller<'a>(
    state: &DebuggerState,
) -> impl Into<Element<'a, DebuggerMsg>> {
    // the first (0xF Bytes) x (0xF0 Bytes)
    let start_page_view: Column<'_, DebuggerMsg> = {
        let start_bytes_view = state.editable_view(state.first_n_words());
        let mut mem_addr = DebuggerState::START;

        start_bytes_view
            .into_iter() // Take ownership of the data
            .fold(Column::new(), |column, byte_row| {
                let row = addr_and_row(byte_row, &mut mem_addr);
                column.push(row).align_x(Alignment::Center)
            })
    };

    let end_page_view: Column<'_, DebuggerMsg> = {
        let end_bytes_view = state.editable_view(state.last_n_words());
        let mut mem_addr: usize = DebuggerState::END;

        end_bytes_view
            .into_iter() // Take ownership of the data
            .fold(Column::new(), |column, byte_row| {
                let row = addr_and_row(byte_row, &mut mem_addr);
                column.push(row).align_x(Alignment::Center)
            })
    };

    const PADDING: u16 = 10u16;
    const SPACING: u16 = 10u16;

    let main: Column<'_, DebuggerMsg> = {
        col![start_page_view, seperator(), end_page_view]
            .align_x(Alignment::Center)
            .padding(PADDING)
            .spacing(SPACING)
    };

    scrollable(main)
}

fn addr_and_row<'a>(
    byte_row: Vec<String>,
    mem_addr: &mut usize,
) -> Row<'a, DebuggerMsg> {
    // fmt for the address of a row
    let line_addr: Container<'_, DebuggerMsg> = Container::new(Text::new(
        format!("${mem_addr:04X}: "),
    ))
    .padding(Padding {
        top: 1f32,
        right: 5f32,
        bottom: 1f32,
        left: 1f32,
    });

    let data: Vec<Element<'_, DebuggerMsg>> = byte_row
        .into_iter()
        .map(|byte| {
            // converting since extend method on row only accepts `Element`
            Container::new(Text::new(byte)).padding(5).into()
        })
        .collect();

    // push the new remaining elements after the address s.t. they're to the right of the address
    let row = row![line_addr].extend(data);

    *mem_addr += 0x10;
    row
}
