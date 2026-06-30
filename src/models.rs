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
    pub battery_data: Option<BatteryData>,

    pub raw_dump: String,
    pub wallpaper: Option<iced::widget::image::Handle>,
}
// BatteryData
#[derive(Debug, Clone, Default)]
pub struct BatteryData {
    pub absolute_capacity: Option<u64>,
    pub apple_raw_max_capacity: Option<u64>,
    pub cycle_count: Option<u64>,
    pub design_capacity: Option<u64>,
    pub date_of_first_use: Option<u64>,
    pub manufacture_date: Option<u64>,
    pub serial: Option<String>,
    pub raw_dump: Option<String>,
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
    pub battery_data: Option<BatteryData>,
    pub wallpaper: Option<iced::widget::image::Handle>,
}
