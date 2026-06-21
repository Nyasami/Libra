use iced::widget::{column, scrollable, text, Space};
use iced::Element;

use crate::message::Message;
use crate::ui::app::LibraApp;

pub fn view(app: &LibraApp) -> Element<'_, Message> {
    if let Some(udid) = &app.selected_device_udid {
        if let Some(dev) = app.devices.iter().find(|d| d.udid == *udid) {
            let content = column![
                text(format!("{}", dev.name)).size(30),
                Space::with_height(20),
                text(format!("UDID: {}", dev.udid)),
                text(format!("Model: {} - {}", dev.model, dev.product_type)),
                text(format!("iOS: {}", dev.ios_version)),
                text(format!("Connection: {}", dev.connection_type)),
                text(format!("Activation State: {}", dev.activation_state)),
                text(format!("CPU Architecture: {}", dev.cpu_architecture)),
                text(format!("Device Class: {}", dev.device_class)),
                text(format!("Hardware Model: {}", dev.hardware_model)),
                text(format!("Product Type: {}", dev.product_type)),
                text(format!("Region: {}", dev.region_info)),
                text(format!("Serial Number: {}", dev.serial_number)),
                text(format!("Raw Dump: {}", dev.raw_dump)),
            ].spacing(5);

            return column![
                text("Device Info").size(40),
                Space::with_height(20),
                scrollable(content)
            ].into();
        } else {
            return column![
                text("Device Info").size(40),
                Space::with_height(20),
                text("Device disconnected").size(20),
            ].into();
        }
    } else {
        column![
            text("Device Info").size(40),
            Space::with_height(20),
            text("select a device from device tab").size(20),
        ].into()
    }
}
