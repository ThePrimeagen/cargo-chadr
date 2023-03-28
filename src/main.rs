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
    // TODO: This is bull shit and just use the clap cargo parser babe
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
