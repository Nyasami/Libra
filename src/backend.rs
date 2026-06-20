use crate::models::DeviceInfo;
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
        let mut raw_dump: String = String::from("(Not available)");

        if let Ok(mut lockdown) = LockdownClient::connect(&provider).await {
            if let Ok(pairing_file) = provider.get_pairing_file().await {
                let _ = lockdown.start_session(&pairing_file).await;
            }

            if let Ok(Some(val)) = lockdown.get_value(Some("DeviceName"), None).await.map(|v| v.as_string().map(String::from)) {
                name = val;
            }
            if let Ok(Some(val)) = lockdown.get_value(Some("ProductVersion"), None).await.map(|v| v.as_string().map(String::from)) {
                ios_version = val;
            }
            if let Ok(Some(val)) = lockdown.get_value(Some("ProductType"), None).await.map(|v| v.as_string().map(String::from)) {
                product_type = val;
                model = crate::utils::human_readable_model(&product_type);
            }
            if let Ok(val) = lockdown.get_value(None, None).await {
                raw_dump = format!("{:#?}", val);
            }
        }

        result.push(DeviceInfo {
            udid: device.udid.clone(),
            connection_type: format!("{:?}", device.connection_type),
            name,
            ios_version,
            model,
            product_type,
            raw_dump,
        });
    }

    Ok(result)
}
