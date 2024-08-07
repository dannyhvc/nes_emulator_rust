use iced::widget::column;
use iced::widget::row;
use iced::widget::text;
use iced::widget::Text;
use iced_aw::{grid, grid_row};

use super::super::DebuggeeMessage;

use super::super::Debuggees;

pub(crate) fn debug_cpu_view(
    app: &Debuggees,
) -> iced::Element<DebuggeeMessage> {
    let res = iced::widget::responsive(|_s| {
        let cpu_col = iced::widget::Column::<DebuggeeMessage>::new()
            .push(Text::new("CPU DATA").size(30))
            .push(row!(Text::new(format!("{}", app.cpu)).size(20)))
            .push(Text::new("BUS DATA").size(30))
            .push(row![Text::new(format!(
                "RAM: {}KB",
                app.bus.ram().len() / crate::components::KB(1)
            )),])
            .padding(100);
        iced::widget::Scrollable::new(cpu_col).into()
    });

    res.into()
}

pub struct BorderedContainerStyle;

impl Into<iced::theme::Container> for BorderedContainerStyle {
    fn into(self) -> iced::theme::Container {
        iced::theme::Container::Custom(Box::new(self))
    }
}

impl iced::widget::container::StyleSheet for BorderedContainerStyle {
    type Style = iced::Theme;

    fn appearance(
        &self,
        _style: &Self::Style,
    ) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            border: iced::Border {
                color: iced::Color::from_rgb(0.0, 0.0, 0.0),
                width: 2.0,
                radius: iced::border::Radius::from([1.0; 4]),
            },

            ..iced::widget::container::Appearance::default()
        }
    }
}

pub(crate) fn bordered_container<'a, T: 'a>(
    content: T,
) -> iced::widget::Container<'a, DebuggeeMessage>
where
    T: Into<iced::Element<'a, DebuggeeMessage>>,
{
    iced::widget::Container::new(content).style(BorderedContainerStyle)
}

pub(crate) fn ram_read_hit_view<'a>() -> iced::Element<'a, DebuggeeMessage> {
    iced::widget::responsive(|_s| {
        // title
        let mut read_col = column!(Text::new("Read").size(30)).padding(10);
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
        read_col = read_col.push(grid);
        iced::widget::Scrollable::new(read_col).into()
    })
    .into()
}

pub(crate) fn ram_write_hit_view<'a>() -> iced::Element<'a, DebuggeeMessage> {
    iced::widget::responsive(|_s| {
        let mut write_col: iced::widget::Column<DebuggeeMessage> =
            column!(Text::new("Write")).padding(10);
        let mut w: Vec<_> =
            crate::components::dh_bus::ram_stats::write_access_hits()
                .into_iter()
                .filter(|&(_k, v)| v > 1)
                .collect();
        w.sort_by_key(|&(key, _)| key);
        for &(k_write, v_write) in w.iter() {
            write_col = write_col
                .push(column!(Text::new(format!(
                    "@{:x} #{}",
                    k_write, v_write
                ))))
                .padding(5);
        }
        iced::widget::Scrollable::new(write_col).into()
    })
    .into()
}
