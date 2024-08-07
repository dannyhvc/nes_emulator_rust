use crate::debug::types::{
    dh_debuggee::debuggee::Debuggees, dh_debuggee_message::DebuggeeMessage,
};

use iced::widget::row;
use iced::widget::Text;

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
