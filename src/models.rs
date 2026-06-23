/// DeviceInfo
/// these are the static info from devices
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub activation_state: String,
    pub build_version: String,
    pub connection_type: String,
    pub cpu_architecture: String,
    pub device_class: String,
    pub hardware_model: String,
    pub imei: String,
    pub ecid: String,
    pub ios_version: String,
    pub model: String,
    pub model_number: String,
    pub name: String,
    pub product_type: String,
    pub region_info: String,
    pub serial_number: String,
    pub storage_total: String,
    pub storage_free: String,
    pub udid: String,

    // batteries
    pub battery_capacity: u8,
    pub battery_is_charging: bool,

    pub raw_dump: String,
    pub wallpaper: Option<iced::widget::image::Handle>,
}

/// DeviceInfoState
/// these are the info from devices that are not static
#[derive(Debug, Clone)]
pub struct DeviceInfoState {
    pub udid: String,
    pub storage_total: String,
    pub storage_free: String,
    pub battery_capacity: u8,
    pub battery_is_charging: bool,
    pub wallpaper: Option<iced::widget::image::Handle>,
}
