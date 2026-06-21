use iced::widget::{column, row, scrollable, text, Space};
use iced::{Element, Length};

use crate::message::Message;
use crate::ui::app::LibraApp;

pub fn view(app: &LibraApp) -> Element<'_, Message> {
    if let Some(udid) = &app.selected_device_udid {
        if let Some(dev) = app.devices.iter().find(|d| d.udid == *udid) {
            let info_content = column![
                labeled_content("", format!("{} - {}%", if dev.battery_is_charging { "Charging" } else { "Not Charging" }, dev.battery_capacity)),
                labeled_content("UDID", dev.udid.clone()),
                labeled_content("Model", format!("{}", dev.model)),
                labeled_content("iOS", format!("{} ({})", dev.ios_version, dev.build_version)),
                labeled_content("Activation State", dev.activation_state.clone()),
                labeled_content("CPU Architecture", dev.cpu_architecture.clone()),
                labeled_content("Device Class", dev.device_class.clone()),
                labeled_content("Hardware Model", dev.hardware_model.clone()),
                labeled_content("Product Type", format!("{} ({})", dev.product_type, dev.model_number)),
                labeled_content("Region", dev.region_info.clone()),
                labeled_content("IMEI", dev.imei.clone()),
                labeled_content("Serial Number", dev.serial_number.clone()),
                labeled_content("ECID", dev.ecid.clone()),
                labeled_content("Storage", format!("{} free of {}", dev.storage_free, dev.storage_total)),
            ].spacing(10);

            let row_content = if let Some(handle) = &dev.wallpaper {
                row![
                    info_content,
                    Space::with_width(100),
                    iced::widget::image(handle.clone())
                        .height(Length::Fixed(400.0))
                ]
            } else {
                row![info_content]
            };

            return column![
                text("Device Info").size(40),
                text(format!("{}", dev.name)).size(30),
                Space::with_height(20),
                scrollable(row_content),
                // scrollable(
                //     text(format!("{}", dev.raw_dump))
                // )
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

fn labeled_content<'a>(label: &'a str, value: String) -> Element<'a, Message> {
    row![
        text(label).width(Length::Fixed(140.0)),
        text(value).font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..iced::Font::DEFAULT
        })
    ]
    .spacing(10)
    .into()
}