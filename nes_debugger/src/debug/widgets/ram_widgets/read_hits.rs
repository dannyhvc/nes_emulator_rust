use crate::debug::types::dh_debuggee_message::DebuggeeMessage;
use crate::debug::widgets::grid_cell::bordered_container;
use iced::widget::column;
use iced::widget::text;
use iced::widget::Text;
use iced_aw::{grid, grid_row};

pub fn ram_read_hit_view<'a>() -> iced::Element<'a, DebuggeeMessage> {
    iced::widget::responsive(|_s| {
        // title
        let read_col = column!(Text::new("Read").size(30)).padding(10);
        // map -> sorted set
        let mut r: Vec<_> =
            crate::components::dh_bus::ram_stats::read_access_hits()
                .into_iter()
                .filter(|&(_k, v)| v > 1)
                .collect();
        r.sort_by_key(|&(key, _)| key);

        // header part of the table
        let mut grid = grid![grid_row!(
            bordered_container(text("Address").size(18)),
            bordered_container(text("Data").size(18))
        )];
        // body part
        for &(k_read, v_read) in r.iter() {
            grid = grid
                .push(grid_row![
                    bordered_container(Text::new(format!("0x{:x}", k_read))),
                    bordered_container(Text::new(format!(
                        "0x{:x}(hex) | {}(dec)",
                        v_read, v_read
                    ))),
                ])
                .width(iced::Length::FillPortion(50))
                .spacing(0.0)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .vertical_alignment(iced::alignment::Vertical::Center);
        }

        // HACK: remove the explain
        let read_col = iced::Element::from(read_col.push(grid))
            .explain(iced::Color::BLACK);
        iced::widget::Scrollable::new(read_col).into()
    })
    .into()
}
