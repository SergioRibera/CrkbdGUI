#![allow(dead_code)]
use hidapi_rusb::DeviceInfo;

pub mod color;

// [ command, action, index, r, g, b ]

pub const VENDOR_ID: u16 = 0x4653;
pub const PRODUCT_ID: u16 = 0x0001;
pub const USAGE_PAGE: u16 = 0xFF60;
pub const USAGE_ID: u16 = 0x61;
pub const INTERFACE_ID: i32 = 1;

pub const RAW_EPSIZE: usize = 6;

pub fn is_my_device(device: &DeviceInfo) -> bool {
    device.vendor_id() == VENDOR_ID
        && device.product_id() == PRODUCT_ID
        // && device.usage_page() == USAGE_PAGE
        // && device.usage() == USAGE_ID
        && device.interface_number() == INTERFACE_ID
}

pub enum CommandType {
    GetKeyboardValue = 0x01,
    SetKeyboardValue = 0x02,
    GetCurrentColor = 0x03,
    ChangeColor = 0x04,

    GetCurrentColorFlag = 0x00,
}
