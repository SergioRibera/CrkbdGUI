#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::{thread, time::Duration};

use clap::Parser;
use humantime::parse_duration;

mod cmd;
mod hid;
use cmd::{Arguments, TypeColor};
use hid::{
    color::{get_current_color, send_color},
    is_my_device, VENDOR_ID, PRODUCT_ID,
};
use hidapi_rusb::{HidApi, HidDevice};

fn main() {
    let args = Arguments::parse();
    if !args.no_gui {
        tauri::Builder::default()
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }

    let api = HidApi::new().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let devices: Vec<&hidapi_rusb::DeviceInfo> = api
        .device_list()
        .filter(|device| {
            if args.show_devices {
                println!(
                    "{} - {}, {:#?}, {}, {}, {}, {}",
                    device.manufacturer_string().unwrap_or_default(),
                    device.product_string().unwrap_or_default(),
                    device.path(),
                    device.vendor_id(),
                    device.product_id(),
                    device.usage_page(),
                    device.usage()
                );
            }
            is_my_device(device)
        })
        .collect();

    let time = args.time.unwrap_or(parse_duration("10s").unwrap());

    if let Some(raw) = args.color {
        let color = raw.get_data();
        if devices.len() == 0 {
            eprintln!("Could not find keyboard");
        } else {
            for d in devices {
                let type_color = args.type_color.clone();
                let device = d.open_device(&api).expect(
                    format!(
                        "{}: {}",
                        "Could not open HID device",
                        d.product_string().unwrap_or("")
                    )
                    .as_str(),
                );
                // let device = &api.open(VENDOR_ID, PRODUCT_ID).unwrap();
                let color = color.clone();
                change_color_and_restore(&device, color, time.clone(), type_color);
            }
        }
    }
}

fn change_color_and_restore(
    device: &HidDevice,
    color: Vec<u8>,
    delay: Duration,
    type_color: Option<TypeColor>,
) {
    let last_color = get_current_color(&device, 500).unwrap();
    println!("Data Received: {:?}", last_color);
    match send_color(&device, color.clone(), type_color.clone()) {
        Ok(w) => println!("Data Sended: {w}"),
        Err(e) => println!("Fail to send data: {e}"),
    }
    thread::sleep(delay);
    match send_color(&device, last_color.0.clone(), type_color) {
        Ok(w) => println!("Data Sended: {w}"),
        Err(e) => println!("Fail to send data: {e}"),
    }
}
