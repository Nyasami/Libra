use iced::widget::{button, column, container, row, text, Rule, Space};
use iced::{Element, Length, Theme, Task};

use crate::models::DeviceInfo;
use crate::message::{Message, CurrentView};
use crate::backend::{fetch_devices, listen_for_devices, poll_device_info_state};
use crate::ui::views;
pub struct LibraApp {
    pub devices: Vec<DeviceInfo>,
    pub error: Option<String>,
    pub selected_device_udid: Option<String>,
    pub current_view: CurrentView,
}

impl LibraApp {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                devices: Vec::new(),
                error: None,
                selected_device_udid: None,
                current_view: CurrentView::Devices,
            },
            Task::perform(fetch_devices(), Message::DevicesLoaded),
        )
    }

    pub fn title(&self) -> String {
        String::from("Libra")
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::LoadDevices => {
                Task::perform(fetch_devices(), Message::DevicesLoaded)
            }
            Message::DevicesLoaded(Ok(devices)) => {
                self.devices = devices;
                Task::none()
            }
            Message::DevicesLoaded(Err(error)) => {
                self.error = Some(error);
                Task::none()
            }
            Message::SelectDevice(udid) => {
                self.selected_device_udid = Some(udid);
                self.current_view = CurrentView::Info;
                Task::none()
            }
            Message::SwitchView(view) => {
                self.current_view = view;
                Task::none()
            }
            Message::OpenUrl(url) => {
                let _ = open::that(url);
                Task::none()
            }
            Message::DeviceInfoStateRefreshed(Ok(states)) => {
                for state in states {
                    if let Some(device) = self.devices.iter_mut().find(|d| d.udid == state.udid) {
                        device.storage_total = state.storage_total;
                        device.storage_free = state.storage_free;
                        device.battery_capacity = state.battery_capacity;
                        device.battery_is_charging = state.battery_is_charging;
                        device.wallpaper = state.wallpaper;
                    }
                }
                Task::none()
            }
            Message::DeviceInfoStateRefreshed(Err(error)) => {
                self.error = Some(error);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let sidebar = container(
            column![
                text("Libra").size(32),
                Space::with_height(40),
                nav_button("Devices", CurrentView::Devices, &self.current_view),
                Space::with_height(10),
                nav_button("Info", CurrentView::Info, &self.current_view),
                Space::with_height(10),
                nav_button("Credits", CurrentView::Credits, &self.current_view),
            ]
            .width(Length::Fill)
        )
        .padding(30)
        .width(Length::Fixed(250.0))
        .height(Length::Fill);

        let content: Element<'_, Message> = match self.current_view {
            CurrentView::Devices => views::devices::view(self),
            CurrentView::Info => views::info::view(self),
            CurrentView::Credits => views::credits::view(),
        };

        row![
            sidebar,
            Rule::vertical(1),
            container(content)
                .padding(40)
                .width(Length::Fill)
                .height(Length::Fill)
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        let mut subs = vec![listen_for_devices()];
        for dev in &self.devices {
            subs.push(poll_device_info_state(dev.udid.clone()));
        }
        iced::Subscription::batch(subs)
    }

    pub fn theme(&self) -> Theme {
        Theme::CatppuccinFrappe
    }
}

fn nav_button<'a>(label: &'a str, target: CurrentView, current: &CurrentView) -> Element<'a, Message> {
    let mut btn = button(text(label).size(18))
        .style(button::text)
        .width(Length::Fill)
        .padding(10)
        .on_press(Message::SwitchView(target.clone()));

    if *current == target {
        btn = btn.style(|theme: &Theme, status| {
            let mut style = button::secondary(theme, status);
            style.border = iced::Border {
                radius: 4.0.into(),
                ..style.border
            };
            style
        });
    }

    btn.into()
}
