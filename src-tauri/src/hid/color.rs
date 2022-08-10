use std::fmt::Display;

use prisma::{Rgb, Hsv, Color};
use hidapi::HidDevice;

use super::CommandType;

const HASH: u8 = b'#';

pub enum RgblightType {
    Full,
    OneRow(u8),
    OnlyOne(u8)
}

pub fn send_color(device: &HidDevice, color: Rgb<u8>, rgb_type: RgblightType) {
    let (r, g, b) = color.to_tuple();

    let rgb_type_u8: (u8, u8) = match rgb_type {
        RgblightType::Full => (2, 0),
        RgblightType::OneRow(index) => (3, index),
        RgblightType::OnlyOne(index) => (1, index),
    };


    // [ command, action, index, r, g, b ]
    device.write(&[CommandType::ChangeColor as u8, rgb_type_u8.0, rgb_type_u8.1, r, g, b]).unwrap();
}

pub fn hex_to_rgb(hex: &str) -> Result<Rgb<u8>, ()> {
    let s = if hex.replace("#", "").len() > 6 {
        let mut b = hex.as_bytes().to_vec();
        b.iter().position(|&n| n > 6).map(|e| b.remove(e));
        b
    } else {
        hex.as_bytes().to_vec()
    };

    let mut buff: [u8; 6] = [0; 6];
    let mut buff_len = 0;

    for b in s {
        if !b.is_ascii() || buff_len == 6 {
            return Err(());
        }

        let bl = b.to_ascii_lowercase();
        if bl == HASH {
            continue;
        }
        if bl.is_ascii_hexdigit() {
            buff[buff_len] = bl;
            buff_len += 1;
        }
    }

    if buff_len == 3 {
        buff = [buff[0], buff[0], buff[1], buff[1], buff[2], buff[2]];
    }

    let hex_str = core::str::from_utf8(&buff).map_err(|_| ()).unwrap();
    let hex_digit = u32::from_str_radix(hex_str, 16).map_err(|_| ()).unwrap();

    let r = hex_digit >> 16;
    let g = (hex_digit >> 8) & 0x00FF;
    let b = hex_digit & 0x0000_00FF;

    Ok(Rgb::new(r as u8, g as u8, b as u8))
}

#[allow(dead_code)]
pub fn rgb_to_hex(rgb: RGB) -> String {
    format!("#{:02X}{:02X}{:02X}", rgb.0, rgb.1, rgb.2)
}
