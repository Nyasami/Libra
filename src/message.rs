use crate::models::DeviceInfo;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurrentView {
    Devices,
    Info,
    Credits,
}

/// messages
#[derive(Debug, Clone)]
pub enum Message {
    /// Switch views
    SwitchView(CurrentView),
    /// Load Devices
    LoadDevices,
    /// Device Loaded Result
    DevicesLoaded(Result<Vec<DeviceInfo>, String>),
    /// Select Device
    SelectDevice(String),   
    /// Open Url
    OpenUrl(String),
}
