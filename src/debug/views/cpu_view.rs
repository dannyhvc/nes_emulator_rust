use super::menu_drop_down;
use crate::debug::types::{DebuggerMsg, DebuggerState};
use iced::widget::column as col;
use iced::widget::*;
use iced::*;

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

    let status_txt =
        Container::new(Text::new(format!("STATUS: {:08b}", state.cpu.status)))
            .align_right(Length::Fill)
            .padding(5);

    let registers = Column::from_vec(register_details)
        .align_x(Alignment::Center)
        .width(Length::Fill);

    main.push(status_txt).push(registers).into()
}
