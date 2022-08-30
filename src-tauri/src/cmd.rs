use std::time::Duration;

use clap::{Parser, Subcommand};
use humantime::parse_duration;

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
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        hex::decode(s).map(HexData)
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
