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
    pub disable_logtypes: Option<Vec<LogOutput>>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    Play {
        #[arg(long)]
        headless: bool,

        #[arg(action = clap::ArgAction::Append)]
        file: PathBuf,
    },

    Debug {
        #[arg(short, long, value_parser = maybe_hex::<u16>)]
        breakpoint: Option<u16>,

        #[arg(action = clap::ArgAction::Append)]
        file: PathBuf,
    },
}
