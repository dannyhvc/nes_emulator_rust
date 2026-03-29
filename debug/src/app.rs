use iced::advanced::graphics::core::SmolStr;
//logging import
use log::{debug, info};

// iced imports
use iced::keyboard::key;
use iced::{Element, Subscription};

// debug imports
use crate::types::DebuggerMsg;
use crate::types::DebuggerState;

use super::types::UiContext;
use super::views::*;

fn match_key_press_char(key: SmolStr, state: &mut DebuggerState) {
    if key == "q" {
        info!("exiting...");
        std::process::exit(0);
    }

    if key == "r" {
        info!("reseting cpu state...");
        state.cpu.reset(&state.bus);
        state.disasm_idx = 0_usize;
    }

    if key == "i" {
        info!("calling irq...");
        state.cpu.irq(&mut state.bus);
    }

    if key == "n" {
        info!("calling nmi...");
        state.cpu.nmi(&mut state.bus);
    }
}

// Emits an updated DebuggerState after each tick
fn cpu_tick_stream(state: &mut DebuggerState) -> iced::Task<DebuggerMsg> {
    // Clock the CPU once
    state.cpu.clock(&mut state.bus);

    // Optionally: check breakpoints, etc.
    // Yield the new state and keep going
    iced::Task::done(DebuggerMsg::CpuActions(crate::types::CpuActions::Step))
}

fn match_key_press_special(
    key: iced::keyboard::key::Named,
    state: &mut DebuggerState,
) {
    match key {
        iced::keyboard::key::Named::Escape => {
            info!("exiting...");
            std::process::exit(0);
        }

        iced::keyboard::key::Named::Space => {
            _ = clock_cpu(state); // TODO: use this task for something
        }
        _ => (), // not implemented but not erroring
    }
}

fn clock_cpu(state: &mut DebuggerState) -> iced::Task<DebuggerMsg> {
    // clock the cpu until it is done once hit
    // log::info!("{}", state.disasm[state.disasm_idx].1);
    state.cpu.clock(&mut state.bus);
    while !state.cpu.complete() {
        state.cpu.clock(&mut state.bus);
    }
    let x = state.bus.ram();
    let x = &x[0x8000..0x800C];
    info!("\n\n{x:X?}");

    iced::Task::done(DebuggerMsg::CpuActions(crate::types::CpuActions::Step))
    // state.disasm_idx += 1_usize;
    // log::info!("{}", state.disasm[state.disasm_idx].1);
}

pub fn title(_state: &DebuggerState) -> String {
    "NES Debugger".into()
}

pub fn update(
    state: &mut DebuggerState,
    message: DebuggerMsg,
) -> impl Into<iced::Task<DebuggerMsg>> {
    match message {
        DebuggerMsg::Start => {
            debug!("Session Started");
            return cpu_tick_stream(state);
        }
        DebuggerMsg::End => debug!("Session Ended"),
        DebuggerMsg::KeyPressed(key) => match key {
            iced::keyboard::Key::Character(key) => {
                match_key_press_char(key, state)
            }
            iced::keyboard::Key::Named(key) => {
                match_key_press_special(key, state);
            }
            _ => unimplemented!(),
        },
        DebuggerMsg::RefreshContext(ctx) => state.context = ctx,
        DebuggerMsg::CpuActions(action) => match action {
            crate::types::CpuActions::Reset => state.cpu.reset(&mut state.bus),
            crate::types::CpuActions::Clock => return clock_cpu(state),
            crate::types::CpuActions::Step => info!("{}", state.cpu),
        },
    }
    // println!("{:?}", state.cpu);
    iced::Task::none()
}

pub fn view(
    state: &DebuggerState,
) -> Element<'_, DebuggerMsg, iced::Theme, iced::Renderer> {
    match state.context {
        UiContext::ShowRAM => ram_view::ram_base(state).into(),
        UiContext::ShowCPU => cpu_view::cpu_base(state).into(),
        UiContext::ShowPPU => ppu_view::ppu_base(state).into(),
        UiContext::ShowAPU => apu_view::apu_base(state).into(),
    }
}

pub fn subscription(_state: &DebuggerState) -> Subscription<DebuggerMsg> {
    iced::keyboard::on_key_press(|key, _modifier| match key {
        key::Key::Named(_) => Some(DebuggerMsg::KeyPressed(key)),
        key::Key::Character(_) => Some(DebuggerMsg::KeyPressed(key)),
        key::Key::Unidentified => None,
    })
}
