use crate::models::{DeviceInfo, InstalledApp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurrentView {
    Devices,
    Info,
    Apps,
    Credits,
}

/// messages
#[derive(Debug, Clone)]
pub enum Message {
    // init
    LoadDevices,
    DevicesLoaded(Result<Vec<DeviceInfo>, String>),
    InstalledAppsLoaded(String, Result<Vec<InstalledApp>, String>),
    DeviceInfoStateRefreshed(Result<Vec<crate::models::DeviceInfoState>, String>),
    // app
    SwitchView(CurrentView),
    SelectDevice(String),
    CopyToClipboard(String),
    // power
    RestartDevice(String),
    ShutdownDevice(String),
    SleepDevice(String),
    DeviceCommandResult(Result<(), String>),
    // misc
    OpenUrl(String),
}
