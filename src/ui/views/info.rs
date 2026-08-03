use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Element, Length};

use crate::message::Message;
use crate::ui::app::LibraApp;

pub fn view(app: &LibraApp) -> Element<'_, Message> {
    if let Some(udid) = &app.selected_device_udid {
        if let Some(dev) = app.devices.iter().find(|d| d.udid == *udid) {
            let mut info_content = column![
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

            if let Some(battery_data) = &dev.battery_data {
                if let (Some(max), Some(design), Some(cycles)) = (battery_data.apple_raw_max_capacity, battery_data.design_capacity, battery_data.cycle_count) {
                    let wear = if design > 0 {
                        (max as f64 / design as f64) * 100.0
                    } else {
                        0.0
                    };
                    info_content = info_content.push(labeled_content("Battery Health", format!("{:.2}% - {} cycles", wear, cycles)));
                }
            }

            let power_buttons = row![
                button(text("Restart")).style(button::text).on_press(Message::RestartDevice(dev.udid.clone())),
                button(text("Shutdown")).style(button::text).on_press(Message::ShutdownDevice(dev.udid.clone())),
                button(text("Sleep")).style(button::text).on_press(Message::SleepDevice(dev.udid.clone()))
            ].spacing(10);

            let row_content = if let Some(handle) = &dev.wallpaper {
                row![
                    info_content,
                    Space::with_width(100),
                    column![
                        iced::widget::image(handle.clone())
                            .height(Length::Fixed(400.0)),
                        Space::with_height(15),
                        container(power_buttons)
                    ].align_x(iced::alignment::Horizontal::Center)
                ]
            } else {
                row![
                    info_content,
                    Space::with_width(100),
                    container(power_buttons).width(Length::Fill).center_x(Length::Fill)
                ]
            };

            let copy_buttons = row![
                button("Copy raw Device Info").on_press(Message::CopyToClipboard(dev.raw_dump.clone())).style(button::text),
                button("Copy raw Battery Data").on_press(Message::CopyToClipboard(
                    dev.battery_data.as_ref().and_then(|b| b.raw_dump.clone()).unwrap_or_else(|| String::from("No battery data"))
                )).style(button::text)
            ].spacing(10);

            let centered_content = container(
                column![
                    scrollable(row_content),
                    Space::with_height(20),
                    copy_buttons,
                ]
            )
            .width(Length::Fill)
            .center_x(Length::Fill);

            return scrollable(
                column![
                    text("Device Info").size(40),
                    text(format!("{}", dev.name)).size(30),
                    Space::with_height(20),
                    centered_content
                ]
            ).into();
        } else {
            return scrollable(
                column![
                    text("Device Info").size(40),
                    Space::with_height(20),
                    container(text("Device disconnected").size(20)).width(Length::Fill).center_x(Length::Fill),
                ]
            ).into();
        }
    } else {
        column![
            text("Device Info").size(40),
            Space::with_height(20),
            container(text("select a device from device tab").size(20)).width(Length::Fill).center_x(Length::Fill),
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