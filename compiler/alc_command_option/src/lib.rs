use std::path::PathBuf;
use structopt::{clap::arg_enum, StructOpt};

pub fn parse_args() -> CommandOptions {
    let command_options = CommandOptions::from_args();
    command_options
}

arg_enum! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum Gc {
        None,
        OwnRc,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "alc")]
pub struct CommandOptions {
    #[structopt(parse(from_os_str))]
    pub src: PathBuf,
    #[structopt(short = "o", long, parse(from_os_str), default_value = "out.ll")]
    pub out: PathBuf,
    #[structopt(long = "gc", default_value = "none")]
    pub gc: Gc,
    #[structopt(long = "debug")]
    pub debug: bool,
}

impl CommandOptions {
    pub fn src_file_name(&self) -> &str {
        self.src.to_str().unwrap_or("[FATAL]")
    }
}
