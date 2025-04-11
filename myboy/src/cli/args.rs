use std::path::PathBuf;

use clap::{Parser, Subcommand};
use clap_num::maybe_hex;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub name: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long)]
    pub dbg_view: Option<bool>,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Output {
    Serial,
    CPUStates,
}

#[derive(Subcommand)]
pub enum Commands {
    Run {
        #[arg(short, long, value_parser=maybe_hex::<u16>)]
        breakpoint: Option<u16>,

        #[arg(short, long)]
        file: Option<PathBuf>,

        #[arg(short, long)]
        output: Option<Output>,
    },
}
