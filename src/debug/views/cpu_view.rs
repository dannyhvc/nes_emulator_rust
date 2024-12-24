use super::super::styles;
use super::menu_drop_down;
use crate::components::types::CpuFlags;
use crate::debug::types::{DebuggerMsg, DebuggerState};
use iced::widget::column as col;
use iced::*;
use widget::{container, text, Column, Container, Row};

/// The entire CPU base component
/// This component displays all widgets and
pub fn cpu_base<'a>(state: &DebuggerState) -> Element<'a, DebuggerMsg> {
    let mb = menu_drop_down();
    let main = col![mb];
    let cpu = &state.cpu;

    let register_details: Vec<Element<'_, DebuggerMsg>> = [
        register_text(format!("A: {:02X}", cpu.a)),
        register_text(format!("X: {:02X}", cpu.x)),
        register_text(format!("Y: {:02X}", cpu.y)),
        register_text(format!("SP: {:02X}", cpu.sp)),
        register_text(format!("PC: {:04X}", cpu.pc)),
        register_text(format!("FETCHED: {:02X}", cpu.fetched)),
        register_text(format!("TEMP: {:04X}", cpu.temp)),
        register_text(format!("ABS: {:04X}", cpu.abs)),
        register_text(format!("REL: {:04X}", cpu.rel)),
        register_text(format!("OPCODE: {:02X}", cpu.opcode)),
        register_text(format!("CYCLES: {:02X}", cpu.cycles)),
        register_text(format!("CLOCK COUNT: {:08X}", cpu._clock_count)),
    ]
    .into_iter()
    .map(|txt| txt.into())
    .collect();

    let status_bar = status_register_component(&state).align_x(Alignment::End);
    let registers = container(
        Column::from_vec(register_details)
            .align_x(Alignment::Center)
            .width(Length::Fill),
    )
    .style(styles::rounded_border);

    main.push(status_bar)
        .push(registers)
        .width(Length::Fill)
        .into()
}

fn register_text<'a>(fmtd_string: String) -> Element<'a, DebuggerMsg> {
    let fmt_txt = text(fmtd_string);
    container(fmt_txt)
        .style(styles::rounded_border)
        .width(200)
        .into()
}

fn status_register_component<'a>(
    state: &DebuggerState,
) -> Container<'a, DebuggerMsg> {
    // We can make this component show red character symbol when the flag
    // is off and green when the flag is on.
    const NUMBER_OF_FLAGS: usize = 8;
    const FLAG_SYMBOL: [&str; NUMBER_OF_FLAGS] =
        ["C", "Z", "I", "D", "B", "U", "V", "N"];
    let Green: iced::Color = iced::Color::from_rgb(0., 1., 0.);
    let Red: iced::Color = iced::Color::from_rgb(1., 0., 0.);

    let flag_vals = (0..u8::BITS).map(|i| {
        let flag_text_color: iced::Color =
            match state.cpu.get_flag(CpuFlags::try_from(1u8 << i).unwrap()) {
                1 => Green,
                _ => Red, // will always be 0
            };

        // Casting to Element so that we can use the from_iter to contruct a row
        container(text(FLAG_SYMBOL[i as usize]).color(flag_text_color))
            .padding(2)
            .into()
    });

    let status_line = Row::from_iter(flag_vals);
    container(status_line).style(styles::rounded_border)
}
