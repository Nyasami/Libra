/// DeviceInfo
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub udid: String,
    pub connection_type: String,
    pub name: String,
    pub ios_version: String,
    pub model: String,
    pub product_type: String,

    pub raw_dump: String,
}
