mod pages;
mod opts;
mod chad;
mod link;
mod init;
mod template;

use anyhow::Result;
use clap::Parser;
use opts::{Command, Opts};

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match opts.action {
        Command::Chad => chad::chad(),
        Command::Link => link::link(opts),
        Command::Init => init::init(),
    }
}
