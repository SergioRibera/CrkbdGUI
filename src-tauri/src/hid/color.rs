use hidapi::HidDevice;
use prisma::{Hsv, Rgb};

use super::CommandType;

const HASH: u8 = b'#';

pub enum RgblightType {
    Full,
    OneRow(u8),
    OnlyOne(u8),
}

pub fn send_color(device: &HidDevice, color: Hsv<f32>, rgb_type: RgblightType) {
    let rgb_type_u8: (u8, u8) = match rgb_type {
        RgblightType::Full => (2, 0),
        RgblightType::OneRow(index) => (3, index),
        RgblightType::OnlyOne(index) => (1, index),
    };

    // [ command, action, index, r, g, b ]
    device
        .write(&[
            CommandType::ChangeColor as u8,
            rgb_type_u8.0,
            rgb_type_u8.1,
            color.hue().0 as u8,
            color.saturation() as u8,
            color.value() as u8,
        ])
        .unwrap();
}

#[allow(dead_code)]
pub fn rgb_to_hex(rgb: Rgb<u8>) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb.red(), rgb.green(), rgb.blue())
}
