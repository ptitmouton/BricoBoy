use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_num::maybe_hex;

use crate::logging::log::LogOutput;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub name: Option<String>,

    #[command(subcommand, flatten = true)]
    pub command: Option<Commands>,

    #[arg(long)]
    pub headless: bool,

    #[arg(long)]
    pub disable_logtypes: Option<Vec<LogOutput>>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Device {
        #[arg(long, value_parser=maybe_hex::<u16>)]
        breakpoint: Option<u16>,

        #[arg(short, long)]
        file: Option<PathBuf>,
    },
}
