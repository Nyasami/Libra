use idevice::lockdown::LockdownClient;
use idevice::provider::IdeviceProvider;
use idevice::services::diagnostics_relay::DiagnosticsRelayClient;
use idevice::usbmuxd::{UsbmuxdAddr, UsbmuxdConnection};
use idevice::IdeviceService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = UsbmuxdAddr::from_env_var()?;
    let mut usbmuxd = UsbmuxdConnection::default().await?;

    let devices = usbmuxd.get_devices().await?;
    if devices.is_empty() {
        println!("No device");
        return Ok(());
    }

    let device = &devices[0];
    let provider = device.to_provider(addr.clone(), "libra-debugger");

    let mut lockdown = LockdownClient::connect(&provider).await?;
    if let Ok(pairing_file) = provider.get_pairing_file().await {
        lockdown.start_session(&pairing_file).await?;
    } else {
        println!("No pairing file");
    }

    let mut diag = DiagnosticsRelayClient::connect(&provider).await?;

    match diag
        .ioregistry(None, Some("AppleSmartBattery"), Some("AppleSmartBattery"))
        .await
    {
        Ok(Some(registry)) => {
            println!("AppleSmartBattery:\n{:#?}", registry);
        }
        Ok(None) => {
            println!("empty reg");
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }

    Ok(())
}
