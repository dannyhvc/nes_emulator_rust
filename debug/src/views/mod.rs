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

use super::types::DebuggerMsg;
use crate::types::UiContext;

fn menu_drop_down<'a>() -> Element<'a, DebuggerMsg> {
    menu_bar!((
        Button::new("View"),
        Menu::new(menu_items!((Button::new("RAM")
            .on_press(DebuggerMsg::RefreshContext(UiContext::ShowRAM))
            .width(Length::Fill))(
            Button::new("CPU")
                .on_press(DebuggerMsg::RefreshContext(UiContext::ShowCPU))
                .width(Length::Fill)
        )(
            Button::new("PPU")
                .on_press(DebuggerMsg::RefreshContext(UiContext::ShowPPU))
                .width(Length::Fill)
        )(
            Button::new("APU")
                .on_press(DebuggerMsg::RefreshContext(UiContext::ShowAPU))
                .width(Length::Fill)
        )))
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
