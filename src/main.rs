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
    let args = std::env::args()
        .skip_while(|x| {
            return x != "--";
        })
        .collect::<Vec<String>>();

    let args = if args.len() == 0 {
        std::env::args().collect::<Vec<String>>()
    } else {
        args
    };

    let opts: Opts = Opts::parse_from(args);

    match opts.action {
        Command::Chad => chad::chad()?,
        Command::Link => link::link(opts)?,
        Command::Init => init::init()?,
    }

    return Ok(());
}
