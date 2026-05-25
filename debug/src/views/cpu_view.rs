// This module renders the CPU view in the debugger UI.
// It displays CPU registers, the status register flags, and the menu bar.

use super::menu_drop_down; // Reuse the menu bar component from the parent module
use crate::styles; // Styling utilities (rounded borders, etc.)
use crate::types::{DebuggerMsg, DebuggerState}; // Shared debugger types & message enum
use components::types::CpuFlag; // Enum representing individual CPU flags
use iced::widget::{column as col, container, text, Column, Container, Row};
use iced::{Alignment, Color, Element, Length};

/// Renders the **CPU base component** for the debugger UI.
/// Shows:
/// - Menu bar
/// - CPU status register flags
/// - All CPU registers with their current values
pub fn cpu_base<'a>(state: &DebuggerState) -> Element<'a, DebuggerMsg> {
    let cpu = &state.cpu; // Shortcut reference to the CPU state

    // Build a vertical list of all register values (A, X, Y, etc.)
    let registers = Column::with_children(vec![
        reg("ACC", cpu.a),             // Accumulator
        reg("X", cpu.x),             // X index register
        reg("Y", cpu.y),             // Y index register
        reg("SP", cpu.sp),           // Stack pointer
        reg16("PC", cpu.pc),         // Program counter (16-bit)
        reg("FETCHED", cpu.fetched), // Last fetched byte
        reg16("TEMP", cpu.temp),     // Temp register (16-bit)
        reg16("ABS", cpu.abs),       // Absolute address (16-bit)
        reg16("REL", cpu.rel),       // Relative address (16-bit)
        reg("OPCODE", cpu.opcode),   // Last executed opcode
        reg("CYCLES", cpu.cycles),   // Remaining cycles
        reg32("CLOCK", cpu._clock_count), // Master clock (32-bit)
    ])
    .align_x(Alignment::Center) // Center horizontally in the column
    .width(Length::Fill);       // Take full available width

    // Compose the CPU view vertically:
    //  1. Menu bar
    //  2. Status register flags
    //  3. Registers column
    col![
        menu_drop_down(),                                   // Top menu bar
        status_register_component(state).align_x(Alignment::End), // Flags row
        container(registers).style(styles::rounded_border)  // Registers display
    ]
    .width(Length::Fill) // Full width
    .into()
}

/// Helper: formats an 8-bit register value as `NAME: 0xXX`
fn reg<'a>(name: &str, val: u8) -> Element<'a, DebuggerMsg> {
    register_text(format!("{name}: {:02X}", val))
}

/// Helper: formats a 16-bit register value as `NAME: 0xXXXX`
fn reg16<'a>(name: &str, val: u16) -> Element<'a, DebuggerMsg> {
    register_text(format!("{name}: {:04X}", val))
}

/// Helper: formats a 32-bit register value as `NAME: 0xXXXXXXXX`
fn reg32<'a>(name: &str, val: u32) -> Element<'a, DebuggerMsg> {
    register_text(format!("{name}: {:08X}", val))
}

/// Shared helper for drawing a register value with rounded border styling
fn register_text<'a>(text_val: String) -> Element<'a, DebuggerMsg> {
    container(text(text_val))
        .style(styles::rounded_border)
        .width(200) // Fixed width for consistent alignment
        .into()
}

/// Builds the **status register flags** UI.
/// Displays each CPU status flag (C, Z, I, D, B, U, V, N) in a row.
/// Flag color:
///   - GREEN = flag set
///   - RED   = flag cleared
fn status_register_component<'a>(
    state: &DebuggerState,
) -> Container<'a, DebuggerMsg> {
    // Order of status flags as displayed
    const SYMBOLS: [&str; 8] = ["C", "Z", "I", "D", "B", "U", "V", "N"];

    // Colors for active/inactive flags
    const GREEN: Color = Color::from_rgb(0., 1., 0.);
    const RED: Color = Color::from_rgb(1., 0., 0.);

    // Build a small colored text element for each flag
    let flags: Vec<Element<'_, DebuggerMsg>> = SYMBOLS
        .iter()
        .enumerate()
        .map(|(i, &sym)| {
            // Determine if this flag is set by checking the CPU state
            let flag_set =
                state.cpu.get_flag(CpuFlag::try_from(1u8 << i).unwrap()) == 1;

            // Render the flag label, applying a dynamic text color
            container(text(sym).style(move |_| iced::widget::text::Style {
                color: if flag_set { Some(GREEN) } else { Some(RED) },
            }))
            .padding(2) // Small spacing around the flag
            .into()
        })
        .collect();

    // Arrange all flags in a row with a rounded border
    container(Row::with_children(flags)).style(styles::rounded_border)
}
