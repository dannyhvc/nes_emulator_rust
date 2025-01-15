//logging import
use log::{debug, info};

// iced imports
use iced::application::{Title, Update, View};
use iced::keyboard::key;
use iced::{Element, Subscription};

// debug imports
use crate::types::DebuggerApp;
use crate::types::DebuggerMsg;
use crate::types::DebuggerState;

use super::types::UiContext;
use super::views::*;

impl Title<DebuggerState> for DebuggerApp {
    fn title(&self, _state: &DebuggerState) -> String {
        "NES Debugger".into()
    }
}

impl Update<DebuggerState, DebuggerMsg> for DebuggerApp {
    fn update(
        &self,
        state: &mut DebuggerState,
        message: DebuggerMsg,
    ) -> impl Into<iced::Task<DebuggerMsg>> {
        match message {
            DebuggerMsg::Start => debug!("Session Started"),
            DebuggerMsg::End => debug!("Session Ended"),
            DebuggerMsg::KeyPressed(key) => match key {
                iced::keyboard::Key::Character(key) => {
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
                iced::keyboard::Key::Named(key) => {
                    if key == iced::keyboard::key::Named::Escape {
                        info!("exiting...");
                        std::process::exit(0);
                    }

                    if key == iced::keyboard::key::Named::Space {
                        // clock the cpu until it is done once hit
                        // log::info!("{}", state.disasm[state.disasm_idx].1);
                        state.cpu.clock(&mut state.bus);
                        while !state.cpu.complete() {
                            state.cpu.clock(&mut state.bus);
                        }
                        let x = state.bus.ram();
                        let x = &x[0x8000..0x800C];
                        info!("\n\n{x:X?}");
                        // state.disasm_idx += 1_usize;
                        // log::info!("{}", state.disasm[state.disasm_idx].1);
                    }
                }
                _ => unimplemented!(),
            },
            DebuggerMsg::RefreshContext(ctx) => state.context = ctx,
            DebuggerMsg::CpuActions(_) => todo!(),
        }
        // println!("{:?}", state.cpu);
        iced::Task::none()
    }
}

impl<'a> View<'a, DebuggerState, DebuggerMsg, iced::Theme, iced::Renderer>
    for DebuggerApp
{
    #[allow(refining_impl_trait)]
    fn view(
        &self,
        state: &'a DebuggerState,
    ) -> Element<'a, DebuggerMsg, iced::Theme, iced::Renderer> {
        match state.context {
            UiContext::ShowRAM => ram_view::ram_base(state).into(),
            UiContext::ShowCPU => cpu_view::cpu_base(state).into(),
            UiContext::ShowPPU => ppu_view::ppu_base(state).into(),
            UiContext::ShowAPU => apu_view::apu_base(state).into(),
        }
    }
}

pub fn subscription(_state: &DebuggerState) -> Subscription<DebuggerMsg> {
    iced::keyboard::on_key_press(|key, _modifier| match key {
        key::Key::Named(_) => Some(DebuggerMsg::KeyPressed(key)),
        key::Key::Character(_) => Some(DebuggerMsg::KeyPressed(key)),
        key::Key::Unidentified => None,
    })
}
