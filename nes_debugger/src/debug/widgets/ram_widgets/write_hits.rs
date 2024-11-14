use iced::widget::column;
use iced::widget::Text;

use crate::debug::types::dh_debuggee_message::DebuggeeMessage;

pub fn ram_write_hit_view<'a>() -> iced::Element<'a, DebuggeeMessage> {
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
