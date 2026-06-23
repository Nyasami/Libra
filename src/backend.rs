use crate::models::{DeviceInfo, DeviceInfoState};
use crate::message::Message;
use iced::stream;
use futures::stream::StreamExt;
use futures::SinkExt;
use idevice::lockdown::LockdownClient;
use idevice::IdeviceService;
use idevice::usbmuxd::{UsbmuxdAddr, UsbmuxdConnection};
use idevice::provider::IdeviceProvider;

pub fn listen_for_devices() -> iced::Subscription<Message> {
    struct Listen;
    iced::Subscription::run_with_id(
        std::any::TypeId::of::<Listen>(),
        stream::channel(100, |output| async move {
            std::thread::spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap();

                rt.block_on(async move {
                    let mut output = output;
                    loop {
                        let mut connection = match UsbmuxdConnection::default().await {
                            Ok(c) => c,
                            Err(_) => {
                                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                                continue;
                            }
                        };
                        
                        let mut stream = match connection.listen().await {
                            Ok(s) => s,
                            Err(_) => {
                                tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                                continue;
                            }
                        };

                        while let Some(_event) = stream.next().await {
                            if output.send(Message::LoadDevices).await.is_err() {
                                return;
                            }
                        }

                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    }
                });
            });

            // loop alive
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
            }
        })
    )
}

/// fetch device info from lockdown
pub async fn fetch_devices() -> Result<Vec<DeviceInfo>, String> {
    let addr = UsbmuxdAddr::from_env_var().map_err(|e| e.to_string())?;
    let mut usbmuxd = UsbmuxdConnection::default().await.map_err(|e| e.to_string())?;
    let found_devices = usbmuxd.get_devices().await.map_err(|e| e.to_string())?;

    let mut result = Vec::new();

    for device in found_devices {
        let provider = device.to_provider(addr.clone(), "libra");
        
        let mut name: String = String::from("Unknown");
        let mut ios_version: String = String::from("Unknown");
        let mut model: String = String::from("Unknown");
        let mut product_type: String = String::from("Unknown");
        let mut activation_state: String = String::from("Unknown");
        let mut build_version: String = String::from("Unknown");
        let mut cpu_architecture: String = String::from("Unknown");
        let mut device_class: String = String::from("Unknown");
        let mut hardware_model: String = String::from("Unknown");
        let mut model_number: String = String::from("Unknown");
        let mut serial_number: String = String::from("Unknown");
        let mut imei: String = String::from("Unknown");
        let mut ecid: String = String::from("Unknown");
        let mut region_info: String = String::from("Unknown");
        let mut storage_total: String = String::from("Unknown");
        let mut storage_free: String = String::from("Unknown");
        let mut raw_dump: String = String::from("(Not available)");
        let mut wallpaper: Option<iced::widget::image::Handle> = None;

        let mut battery_capacity: u8 = 0;
        let mut battery_is_charging: bool = false;
        
        if let Ok(mut lockdown) = LockdownClient::connect(&provider).await {
            if let Ok(pairing_file) = provider.get_pairing_file().await {
                let _ = lockdown.start_session(&pairing_file).await;
            }
            
            // lets get the whole dict instead
            if let Ok(val) = lockdown.get_value(None, None).await {
                if let Some(dict) = val.as_dictionary() {
                    let get_str = |key: &str| -> Option<String> {
                        dict.get(key).and_then(|v| v.as_string().map(String::from))
                    };

                    let get_int = |key: &str| -> Option<u64> {
                        dict.get(key).and_then(|v| v.as_unsigned_integer())
                    };

                    if let Some(v) = get_str("ActivationState") { activation_state = v; }
                    if let Some(v) = get_str("BuildVersion") { build_version = v; }
                    if let Some(v) = get_str("CPUArchitecture") { cpu_architecture = v; }
                    if let Some(v) = get_str("DeviceClass") { device_class = v; }
                    if let Some(v) = get_str("DeviceName") { name = v; }
                    if let Some(v) = get_str("HardwareModel") { hardware_model = v; }
                    if let Some(v) = get_str("InternationalMobileEquipmentIdentity") { imei = v; }
                    if let Some(v) = get_str("RegulatoryModelNumbers") { model_number = v; }
                    if let Some(v) = get_str("ProductVersion") { ios_version = v; }
                    if let Some(v) = get_str("RegionInfo") { region_info = v; }
                    if let Some(v) = get_str("SerialNumber") { serial_number = v; }
                    if let Some(v) = get_int("UniqueChipID") { ecid = format!("0x{:X}", v); }
                    if let Some(v) = get_str("ProductType") {
                        product_type = v.clone();
                        model = crate::utils::human_readable_model(&product_type);
                    }
                }

                raw_dump = format!("{:#?}", val);
            }

            // nessesary keys i need apart from the whole dict dump
            // https://theapplewiki.com/wiki/List_of_MobileGestalt_keys
            if let Ok(Some(val)) = lockdown.get_value(Some("RegulatoryModelNumber"), None).await.map(|v| v.as_string().map(String::from)) {
                model_number = val;
            }
        }

        result.push(DeviceInfo {
            activation_state,
            battery_capacity,
            battery_is_charging,
            build_version,
            cpu_architecture,
            connection_type: format!("{:?}", device.connection_type),
            device_class,
            hardware_model,
            imei,
            ecid,
            ios_version,
            model,
            model_number,
            name,
            product_type,
            region_info,
            serial_number,
            storage_total,
            storage_free,
            udid: device.udid.clone(),
            raw_dump,
            wallpaper,
        });
    }

    Ok(result)
}

/// those info from device are updated every now and then, so we polling it every 5s
pub fn poll_device_info_state(udid: String) -> iced::Subscription<Message> {
    struct PollInfoState;
    iced::Subscription::run_with_id(
        (std::any::TypeId::of::<PollInfoState>(), udid.clone()),
        stream::channel(1, move |mut output| async move {
            let Ok(addr) = UsbmuxdAddr::from_env_var() else { return; };
            let Ok(mut usbmuxd) = UsbmuxdConnection::default().await else { return; };
            let Ok(found_devices) = usbmuxd.get_devices().await else { return; };
            let Some(device) = found_devices.into_iter().find(|d| d.udid == udid) else { return; };
            
            let provider = device.to_provider(addr.clone(), "libra");

            let Ok(mut lockdown) = LockdownClient::connect(&provider).await else { return; };
            
            if let Ok(pairing_file) = provider.get_pairing_file().await {
                let _ = lockdown.start_session(&pairing_file).await;
            }

            let mut afc = idevice::services::afc::AfcClient::connect(&provider).await.ok();
            let mut sbs = idevice::services::springboardservices::SpringBoardServicesClient::connect(&provider).await.ok();

            loop {
                let (battery_capacity, battery_is_charging) = get_battery(&mut lockdown).await;
                
                let (storage_total, storage_free) = if let Some(afc) = &mut afc {
                    get_storage(afc).await
                } else {
                    (String::from("Unknown"), String::from("Unknown"))
                };
                // still not sure polling wallpaper every 5s is a good idea =.=
                let wallpaper = if let Some(sbs) = &mut sbs {
                    get_wallpaper(sbs).await
                } else {
                    None
                };

                let state = DeviceInfoState {
                    udid: udid.clone(),
                    storage_total,
                    storage_free,
                    battery_capacity,
                    battery_is_charging,
                    wallpaper,
                };

                if output.send(Message::DeviceInfoStateRefreshed(Ok(vec![state]))).await.is_err() {
                    break;
                }

                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        })
    )
}

async fn get_wallpaper(sbs: &mut idevice::services::springboardservices::SpringBoardServicesClient) -> Option<iced::widget::image::Handle> {
    if let Ok(png) = sbs.get_home_screen_wallpaper_preview_pngdata().await {
        return Some(iced::widget::image::Handle::from_bytes(png));
    }
    None
}

async fn get_storage(afc: &mut idevice::services::afc::AfcClient) -> (String, String) {
    if let Ok(info) = afc.get_device_info().await {
        let total_gb = info.total_bytes as f64 / 1_000_000_000.0;
        let free_gb = info.free_bytes as f64 / 1_000_000_000.0;
        return (format!("{:.2} GB", total_gb), format!("{:.2} GB", free_gb));
    }
    (String::from("Unknown"), String::from("Unknown"))
}

async fn get_battery(lockdown: &mut LockdownClient) -> (u8, bool) {
    if let Ok(val) = lockdown.get_value(None, Some("com.apple.mobile.battery")).await {
        if let Some(dict) = val.as_dictionary() {
            let capacity = dict.get("BatteryCurrentCapacity").and_then(|v| v.as_unsigned_integer()).unwrap_or(0);
            let is_charging = dict.get("BatteryIsCharging").and_then(|v| v.as_boolean()).unwrap_or(false);
            return (capacity as u8, is_charging);
        }
    }
    (0, false)
}
