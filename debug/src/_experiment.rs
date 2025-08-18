use iced::{
    widget::{button, column, row, text}, Settings,
};
use log::info;

use crate::widgets::modal::Modal;



pub fn start_debugger() -> iced::Result {
    let app = DebuggerApp {
        backend: MockBackend,
        show_modal: false,
    };

    iced::application("NES Debugger (Iced 0.13.1)", update, view)
        .settings(Settings {
            id: Some("main".into()),
            ..Settings::default()
        })
        .theme(|_| iced::Theme::Light)
        .executor::<iced::executor::Default>()
        .exit_on_close_request(true)
        .run_with(|| (app, iced::Task::none()))
}

// ===== Debugger Backend Trait =====
#[derive(Debug, Clone)]
pub struct CpuState {
    pub pc: u16,
    pub sp: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub cycles: u64,
}

pub trait DebuggerBackend {
    fn cpu_state(&self) -> CpuState;
    fn read_mem(&self, addr: u16, len: usize) -> Vec<u8>;
    fn write_mem(&mut self, addr: u16, data: &[u8]);
    fn step_instruction(&mut self);
    fn reset(&mut self);
}

// ===== Mock Backend for UI Testing =====
#[derive(Default, Clone)]
pub struct MockBackend;
impl DebuggerBackend for MockBackend {
    fn cpu_state(&self) -> CpuState {
        CpuState {
            pc: 0xC000,
            sp: 0xFD,
            a: 0x01,
            x: 0x02,
            y: 0x03,
            p: 0b0010_0100,
            cycles: 123456,
        }
    }
    fn read_mem(&self, addr: u16, len: usize) -> Vec<u8> {
        (0..len).map(|i| ((addr + i as u16) & 0xFF) as u8).collect()
    }
    fn write_mem(&mut self, _addr: u16, _data: &[u8]) {}
    fn step_instruction(&mut self) {}
    fn reset(&mut self) {}
}

// ============================== App State ==============================
#[derive(Default, Clone)]
struct DebuggerApp {
    backend: MockBackend,
    show_modal: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Step,
    Reset,
    ToggleModal,
    CloseModal,
    _None,
}

fn update(state: &mut DebuggerApp, message: Message) -> iced::Task<Message> {
    match message {
        Message::Step => {
            state.backend.step_instruction();
            info!("Stepped one instruction");
        }
        Message::Reset => {
            state.backend.reset();
            info!("Reset CPU");
        }
        Message::ToggleModal => {
            state.show_modal = !state.show_modal;
        }
        Message::CloseModal => {
            state.show_modal = false;
        }
        Message::_None => {}
    }
    iced::Task::none()
}

fn view<'a>(state: &'a DebuggerApp) -> iced::Element<'a, Message> {
    let cpu = state.backend.cpu_state();

    let main_view = column![
        text("NES Debugger").size(30),
        row![
            button("Step").on_press(Message::Step),
            button("Reset").on_press(Message::Reset),
            button("Memory Editor").on_press(Message::ToggleModal),
        ]
        .spacing(10),
        row![
            text(format!("PC: ${:04X}", cpu.pc)),
            text(format!("A: ${:02X}", cpu.a)),
            text(format!("X: ${:02X}", cpu.x)),
            text(format!("Y: ${:02X}", cpu.y)),
            text(format!("SP: ${:02X}", cpu.sp)),
        ]
        .spacing(15),
        text(format!("Status: {:08b}", cpu.p)),
        text(format!("Cycles: {}", cpu.cycles)),
    ]
    .spacing(20)
    .padding(20);

    if state.show_modal {
        Modal::new(true, main_view, || {
            column![
                text("Memory Editor").size(24),
                text("Here you can edit memory..."),
                button("Close").on_press(Message::CloseModal),
            ]
            .spacing(1)
            .padding(10)
            .into()
        })
        .view()
    } else {
        main_view.into()
    }
}
