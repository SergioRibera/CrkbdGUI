use hidapi::DeviceInfo;

pub mod color;

// [ command, action, index, r, g, b ]

pub const VENDOR_ID: u16 = 0x4653;
pub const PRODUCT_ID: u16 = 0x4D4D;
pub const USAGE_PAGE: u16 = 0xFF60;
pub const USAGE_ID: u16 = 0x61;

pub fn is_my_device(device: &DeviceInfo) -> bool {
    device.vendor_id() == VENDOR_ID
        && device.product_id() == PRODUCT_ID
        // && device.usage_page() == USAGE_PAGE
        // && device.usage() == USAGE_ID
}

pub enum CommandType {
    ChangeColor = 0x04,
}
