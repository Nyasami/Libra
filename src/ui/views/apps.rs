use crate::message::Message;
// use crate::ui::app::LibraApp;
use iced::widget::{column, text, Space};
use iced::Element;

pub fn view() -> Element<'static, Message> {
    column![
        text("Apps").size(20),
        Space::with_height(20),
        text("wipppppppp")
    ]
    .into()
}
