use iced::widget::{Rule, Space, button, column, text};
use iced::Element;

use crate::message::Message;

pub fn view() -> Element<'static, Message> {
    column![
        text("Credits").size(40),
        Space::with_height(20),
        button(
            column![
                text("Nyasami").size(20),
                text("Developer"),
                Rule::horizontal(20)
            ]).style(button::text).on_press(Message::OpenUrl("https://github.com/nyasami".to_string())),
        button(
            column![
                text("jkcoxson").size(20),
                text("idevice"),
                Rule::horizontal(20)
            ]
        ).style(button::text).on_press(Message::OpenUrl("https://github.com/jkcoxson".to_string())),
    ].into()
}
