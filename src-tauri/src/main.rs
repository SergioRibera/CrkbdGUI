#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::{thread, time::Duration};

use clap::Parser;

mod cmd;
mod hid;
use cmd::{Arguments, ColorArg};
use hid::{
    color::{get_current_color, send_color},
    is_my_device,
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

    let mut device_list = api.device_list();

    if args.show_devices {
        for device in api.device_list() {
            println!(
                "{} - {}, {:#?}, {}, {}, {}, {}, {}",
                device.manufacturer_string().unwrap_or_default(),
                device.product_string().unwrap_or_default(),
                device.path(),
                device.vendor_id(),
                device.product_id(),
                device.usage_page(),
                device.usage(),
                device.interface_number()
            );
        }
    }

    if let Some(d) = &device_list.find(|device| is_my_device(device)) {
        if let Ok(device) = d.open_device(&api) {
            if let Some(raw) = args.color {
                change_color_and_restore(&device, raw, args.time);
            }
        }
    }
}

fn change_color_and_restore(device: &HidDevice, color: ColorArg, time: Option<Duration>) {
    match color {
        ColorArg::Color { a, b, type_color } => {
            let last_color = if let Some(b_color) = b {
                b_color.get_data()
            } else {
                get_current_color(device, 500)
                    .unwrap_or((vec![0xBF, 0xFF, 0x00], 3))
                    .0
            };
            match send_color(device, a.get_data(), type_color.clone()) {
                Ok(_) => {}
                Err(e) => println!("Fail to send data: {e}"),
            }

            if let Some(delay) = time {
                thread::sleep(delay);
                match send_color(device, last_color, type_color) {
                    Ok(_) => {}
                    Err(e) => println!("Fail to send data: {e}"),
                }
            }
        }
    }
}
