use super::super::types::DebuggerState;
use super::super::types::*;
use super::menu_drop_down;
use crate::traits::*;
use border::Radius;
use iced::widget::column as col;
use iced::widget::*;
use iced::*;
use iced_aw::{quad, widget::InnerBounds};

//
// ──────────────────────────────────────────────────────────────
//   Separator (Horizontal Grey Bar)
// ──────────────────────────────────────────────────────────────
//
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

//
// ──────────────────────────────────────────────────────────────
//   Main Entry Point for Debugger
// ──────────────────────────────────────────────────────────────
//
pub fn ram_base<'a>(state: &DebuggerState) -> Element<'a, DebuggerMsg> {
    let mb = menu_drop_down();

    col![mb]
        .push(debug_button_bar())
        .push(memory_scoller(state))
        .width(Length::Fill)
        .into()
}

//
// ──────────────────────────────────────────────────────────────
//   Debug Button Bar
// ──────────────────────────────────────────────────────────────
//
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
            .on_press(DebuggerMsg::CpuActions(CpuActions::Clock))
            .padding(padding),
        // button("show op").on_press(DebuggerMsg::Start).padding(padding),
        // button("show am").on_press(DebuggerMsg::Start).padding(padding),
        // button("show flags").on_press(DebuggerMsg::Start).padding(padding),
    ]
    .spacing(10)]]
    .align_x(Alignment::Center)
    .width(Length::Fill)
}

//
// ──────────────────────────────────────────────────────────────
//   Memory Scroller View
// ──────────────────────────────────────────────────────────────
//
fn memory_scoller<'a>(
    state: &DebuggerState,
) -> impl Into<Element<'a, DebuggerMsg>> {
    // Start memory view
    let start_page_view: Column<'_, DebuggerMsg> = {
        let start_bytes_view = state.editable_view(state.first_n_words());
        let mut mem_addr = DebuggerState::START;

        start_bytes_view
            .into_iter()
            .fold(Column::new(), |column, byte_row| {
                let row = addr_and_row(byte_row, &mut mem_addr);
                column.push(row).align_x(Alignment::Center)
            })
    };

    // End memory view
    let end_page_view: Column<'_, DebuggerMsg> = {
        let end_bytes_view = state.editable_view(state.last_n_words());
        let mut mem_addr: usize = DebuggerState::END;

        end_bytes_view
            .into_iter()
            .fold(Column::new(), |column, byte_row| {
                let row = addr_and_row(byte_row, &mut mem_addr);
                column.push(row).align_x(Alignment::Center)
            })
    };

    // Combined memory display
    const PADDING: u16 = 10;
    const SPACING: u16 = 10;

    let main = col![start_page_view, seperator(), end_page_view]
        .align_x(Alignment::Center)
        .padding(PADDING)
        .spacing(SPACING);

    scrollable(main)
}

//
// ──────────────────────────────────────────────────────────────
//   Helper: Address + Row Renderer
// ──────────────────────────────────────────────────────────────
//
fn addr_and_row<'a>(
    byte_row: Vec<String>,
    mem_addr: &mut usize,
) -> Row<'a, DebuggerMsg> {
    // Address label
    let line_addr =
        container(Text::new(format!("${mem_addr:04X}: "))).padding(Padding {
            top: 1.0,
            right: 5.0,
            bottom: 1.0,
            left: 1.0,
        });

    // Convert memory bytes into container elements
    let data: Vec<Element<'_, DebuggerMsg>> = byte_row
        .into_iter()
        .map(|byte| container(Text::new(byte)).padding(5).into())
        .collect();

    // Combine address + data row
    let row = row![line_addr].extend(data);

    *mem_addr += 0x10;
    row
}
