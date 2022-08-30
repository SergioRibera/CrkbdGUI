use hidapi_rusb::{HidDevice, HidResult};

use crate::{cmd::TypeColor, hid::RAW_EPSIZE};

use super::CommandType;

pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn get_current_color(device: &HidDevice, timeout: i32) -> Result<(Vec<u8>, usize), String> {
    let data = &mut [
        CommandType::GetCurrentColor as u8,
        CommandType::GetCurrentColor as u8,
        0,
        0,
        0,
        0,
    ];

    match device.write(data) {
        Ok(_) => {
            let buff = &mut [0u8; 3];

            match device.read_timeout(buff, timeout) {
                Ok(size) => Ok((buff.to_vec(), size)),
                Err(_) => Err("Read Failed".to_string()),
            }
        }
        Err(_) => Err("Not can send to device".to_string()),
    }
}

pub fn send_color(
    device: &HidDevice,
    color: Vec<u8>,
    rgb_type: Option<TypeColor>,
) -> HidResult<usize> {
    let rgb_type_u8: (u8, u8) = match rgb_type {
        Some(rgb) => match rgb {
            TypeColor::Full => (2, 0),
            TypeColor::Row { index } => (3, index),
            TypeColor::Single { index } => (1, index),
        },
        None => (2, 0), // Full
    };

    // [ command, action, index, r, g, b ]
    let data: &mut [u8; RAW_EPSIZE] = &mut [0u8; RAW_EPSIZE];
    data[0] = CommandType::ChangeColor as u8;
    data[1] = rgb_type_u8.0;
    data[2] = rgb_type_u8.1;
    data[3] = *color.first().unwrap();
    data[4] = *color.get(1).unwrap();
    data[5] = *color.get(2).unwrap();

    device.write(data)
}

#[allow(dead_code)]
pub fn rgb_to_hex(rgb: Rgb) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb.r, rgb.g, rgb.b)
}
