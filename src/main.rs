mod pages;
mod chad;
mod link;
mod init;
mod template;

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
enum Command {
    // default command is build
    #[clap(name = "chad")]
    Chad,

    // default command is build
    #[clap(name = "link")]
    Link,

    #[clap(name = "init")]
    Init,
}

#[derive(Debug, Parser)]
struct Opts {

    // positional arguments
    #[clap(subcommand)]
    action: Command,
}

fn main() -> Result<()> {
    let opts: Opts = Opts::parse();

    match opts.action {
        Command::Chad => chad::chad(),
        Command::Link => link::link(),
        Command::Init => init::init(),
    }
}
