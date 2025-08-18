#![allow(clippy::unwrap_used)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::wildcard_imports)]
#![allow(clippy::enum_glob_use)]

pub mod apu_view;
pub mod cpu_view;
pub mod ppu_view;
pub mod ram_view;

use border::Radius;
use iced::widget::*;
use iced::*;
use iced_aw::{
    menu::{self, *},
    menu_bar, menu_items,
};

use super::types::{DebuggerMsg, UiContext};

fn ram_button<'a>() -> Button<'a, DebuggerMsg> {
    Button::new("RAM")
        .on_press(DebuggerMsg::RefreshContext(UiContext::ShowRAM))
        .width(Length::Fill)
}

fn cpu_button<'a>() -> Button<'a, DebuggerMsg> {
    Button::new("CPU")
        .on_press(DebuggerMsg::RefreshContext(UiContext::ShowCPU))
        .width(Length::Fill)
}

fn ppu_button<'a>() -> Button<'a, DebuggerMsg> {
    Button::new("PPU")
        .on_press(DebuggerMsg::RefreshContext(UiContext::ShowPPU))
        .width(Length::Fill)
}

fn apu_button<'a>() -> Button<'a, DebuggerMsg> {
    Button::new("APU")
        .on_press(DebuggerMsg::RefreshContext(UiContext::ShowAPU))
        .width(Length::Fill)
}

/// Creates the "View" dropdown menu for the debugger UI

#[rustfmt::skip]
fn menu_drop_down<'a>() -> Element<'a, DebuggerMsg> {
    menu_bar!((
        Button::new("View"),
        Menu::new(
            menu_items!(
                (ram_button())
                (cpu_button())
                (ppu_button())
                (apu_button())
            )
        )
        .width(240.0)
    ))
    .draw_path(menu::DrawPath::Backdrop)
    .style(|theme, status| menu::Style {
        path_border: Border {
            radius: Radius::new(6.0),
            ..Default::default()
        },
        ..primary(theme, status)
    })
    .padding(10.0)
    .into()
}
