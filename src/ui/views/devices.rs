use iced::widget::{Rule, Space, button, column, scrollable, text};
use iced::{Element, Length};

use crate::message::Message;
use crate::ui::app::LibraApp;

pub fn view(app: &LibraApp) -> Element<'_, Message> {
    if let Some(err) = &app.error {
        return column![
            text("Devices").size(40),
            Space::with_height(20),
            text(format!("Error: {}", err)).color(iced::Color::from_rgb(0.8, 0.2, 0.2))
        ].into();
    }
    
    if app.devices.is_empty() {
        return column![
            text("Devices").size(40),
            Space::with_height(20),
            text("No devices connected.").size(20)
        ].into();
    }

    let mut dev_list = column![];
    
    for (_i, dev) in app.devices.iter().enumerate() {
        let card = column![
            text(format!("{}", dev.name)).size(24),
            text(format!("{}", dev.model)),
            text(format!("iOS: {}", dev.ios_version)),
            text(format!("UDID: {}", dev.udid)),
            text(format!("Connection: {}", dev.connection_type)),
            Rule::horizontal(50)
        ].spacing(5);
        
        let btn = button(card)
            .width(Length::Fill)
            .style(button::text)
            .on_press(Message::SelectDevice(dev.udid.clone()));

        dev_list = dev_list.push(btn);
    }

    column![
        text("Devices").size(40),
        Space::with_height(20),
        scrollable(dev_list)
    ].into()
}
