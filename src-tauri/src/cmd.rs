use std::time::Duration;

use clap::{Parser, Subcommand};
use humantime::parse_duration;

use crate::hid::color::Rgb;

const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[derive(Parser, Default, Debug)]
#[clap(author = AUTHORS, version, about=DESCRIPTION)]
pub struct Arguments {
    #[clap(short, long)]
    pub no_gui: bool,
    #[clap(short, long)]
    pub show_devices: bool,
    #[clap(subcommand)]
    pub color: Option<ColorArg>,
    #[clap(short, long, parse(try_from_str = parse_duration))]
    pub time: Option<Duration>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ColorArg {
    Color {
        a: HexData,
        b: Option<HexData>,
        #[clap(subcommand)]
        type_color: Option<TypeColor>,
    },
}

#[derive(Clone, Debug)]
pub struct HexData(Vec<u8>);

impl HexData {
    pub fn get_data(self) -> Vec<u8> {
        self.0
    }
}

impl std::str::FromStr for HexData {
    type Err = String;

    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        let hex_code = if hex.starts_with('#') { &hex[1..] } else { hex };
        match hex_code.len() {
            3 => {
                let red =
                    u8::from_str_radix(&hex_code[..1], 16).expect("Fail to get data from hex");
                let green =
                    u8::from_str_radix(&hex_code[1..2], 16).expect("Fail to get data from hex");
                let blue =
                    u8::from_str_radix(&hex_code[2..3], 16).expect("Fail to get data from hex");
                Ok(HexData(vec![red * 17, green * 17, blue * 17]))
            }
            6 => {
                let red =
                    u8::from_str_radix(&hex_code[..2], 16).expect("Fail to get data from hex");
                let green =
                    u8::from_str_radix(&hex_code[2..4], 16).expect("Fail to get data from hex");
                let blue =
                    u8::from_str_radix(&hex_code[4..6], 16).expect("Fail to get data from hex");
                Ok(HexData(vec![red * 17, green * 17, blue * 17]))
            }
            _ => Err("invalid hex code format".to_string()),
        }
    }
}

#[derive(Subcommand, Default, Debug, Clone)]
pub enum TypeColor {
    #[default]
    Full,
    Row {
        index: u8,
    },
    Single {
        index: u8,
    },
}
