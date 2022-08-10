#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use clap::{Arg, Command};

mod hid;
use hid::{
    color::{hex_to_rgb, send_color, RgblightType},
    is_my_device,
};
use hidapi::HidApi;

use crate::hid::{PRODUCT_ID, USAGE_ID, USAGE_PAGE, VENDOR_ID};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let matches = Command::new("Crkbd Manager")
        .about(DESCRIPTION)
        .version(VERSION)
        .author(AUTHORS)
        .args(&[
            Arg::new("NoGui")
                .long("no-gui")
                .help("Excecute in batch mode")
                .takes_value(false),
            Arg::new("Color")
                .long("color")
                .short('c')
                .help("Change all color of the keyboard (this use the HEX color, example #9ad0ff)")
                .takes_value(true),
            Arg::new("Restore")
                .long("restore")
                .short('r')
                .help("Restore keyboard color in time")
                .takes_value(true),
            Arg::new("Device_List")
                .long("list")
                .short('l')
                .help("Show list Devices")
                .takes_value(false),
        ])
        .get_matches();

    let show_devices = matches.is_present("Device_List");
    if !matches.is_present("NoGui") {
        tauri::Builder::default()
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }

    let api = HidApi::new().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    let device = api
        .device_list()
        .find(|device| {
            if show_devices {
                println!(
                    "{}, {}, {}, {}, {}",
                    device.product_string().unwrap_or_default(),
                    device.vendor_id(),
                    device.product_id(),
                    device.usage_page(),
                    device.usage()
                );
            }
            is_my_device(device)
        })
        .unwrap_or_else(|| {
            eprintln!("Could not find keyboard");
            std::process::exit(1);
        })
        .open_device(&api)
        .unwrap_or_else(|_| {
            eprintln!("Could not open HID device");
            std::process::exit(1);
        });

    if let Some(hex) = matches.value_of("Color") {
        let color = hex_to_rgb(hex).unwrap();
        send_color(&device, color, RgblightType::Full);
    }
}
