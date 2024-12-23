use super::menu_drop_down;
use crate::components::types::CpuFlags;
use crate::debug::types::{DebuggerMsg, DebuggerState};
use iced::widget::column as col;
use iced::*;
use widget::{container, text, Column, Container, Row, Text};

pub fn cpu_base<'a>(state: &DebuggerState) -> Element<'a, DebuggerMsg> {
    let mb = menu_drop_down();
    let main = col![mb];

    let register_details = &state.cpu;
    let register_details: Vec<Element<'a, DebuggerMsg>> = [
        Text::new(format!("A: {:02X}", register_details.a)),
        Text::new(format!("X: {:02X}", register_details.x)),
        Text::new(format!("Y: {:02X}", register_details.y)),
        Text::new(format!("SP: {:02X}", register_details.sp)),
        Text::new(format!("PC: {:04X}", register_details.pc)),
        //
        Text::new(format!("FETCHED: {:02X}", register_details.fetched)),
        Text::new(format!("TEMP: {:04X}", register_details.temp)),
        Text::new(format!("ABS: {:04X}", register_details.abs)),
        Text::new(format!("REL: {:04X}", register_details.rel)),
        Text::new(format!("OPCODE: {:02X}", register_details.opcode)),
        Text::new(format!("CYCLES: {:02X}", register_details.cycles)),
        Text::new(format!(
            "CLOCK COUNT: {:08X}",
            register_details._clock_count
        )),
    ]
    .into_iter()
    .map(|txt| txt.into())
    .collect();

    // Create a

    let cpu_debug_status =
        status_register_component(&state).align_x(Alignment::End);

    let registers = Column::from_vec(register_details)
        .align_x(Alignment::Center)
        .width(Length::Fill);

    main.push(cpu_debug_status).push(registers).into()
}

fn status_register_component<'a>(
    state: &DebuggerState,
) -> Container<'a, DebuggerMsg> {
    // We can make this component show red character symbol when the flag
    // is off and green when the flag is on.

    let mut flag_vals: Vec<_> = vec![];

    const NUMBER_OF_FLAGS: usize = 8;
    const FLAG_SYMBOL: [&str; NUMBER_OF_FLAGS] =
        ["C", "Z", "I", "D", "B", "U", "V", "N"];
    let GREEN: iced::Color = iced::Color::from_rgb(0., 1., 0.);
    let RED: iced::Color = iced::Color::from_rgb(1., 0., 0.);

    for i in 0..u8::BITS {
        let flag_text_color: iced::Color =
            match state.cpu.get_flag(CpuFlags::try_from(1u8 << i).unwrap()) {
                1 => GREEN,
                _ => RED,
            };

        // Casting to Element so that we can use the from_iter to contruct a row
        let flag_text: Element<'a, DebuggerMsg> =
            container(text(FLAG_SYMBOL[i as usize]).color(flag_text_color))
                .padding(2)
                .into();

        flag_vals.push(flag_text);
    }

    let status_line: Row<'a, DebuggerMsg> = Row::from_iter(flag_vals);
    Container::new(status_line)
}
