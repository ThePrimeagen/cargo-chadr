use clap::Parser;

#[derive(Debug, Parser)]
pub enum Command {
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
#[command(version)]
pub struct Opts {

    // positional arguments
    #[clap(subcommand)]
    pub action: Command,

    #[clap(short = 'c', long = "cow", default_value = "/cow/the.cow")]
    pub cow_path: String,
}


