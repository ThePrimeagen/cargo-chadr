mod pages;
mod template;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use pages::{CONTROLLERS, VIEWS, CGI_BIN};
use template::ROUTES;
use walkdir::WalkDir;

use crate::pages::Page;

/*
fn set_watcher() -> Result<()> {
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
           Ok(event) => println!("event: {:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    return Ok(());
}
*/

#[derive(Debug, Parser)]
enum Command {
    // default command is build
    #[clap(name = "chad")]
    Chad,

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
    // set_watcher()?;

    let pages = PathBuf::from("pages");

    let htmlers = WalkDir::new(&pages)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|dir| dir.file_name() == "index.html");

    let mut routes: Vec<String> = vec![];

    _ = std::fs::create_dir_all(CONTROLLERS);
    _ = std::fs::create_dir_all(VIEWS);

    for (idx, path) in htmlers.into_iter().enumerate() {
        let page: Page = path.path().try_into()?;
        let component = page.controller();
        let script_name: String = page.script_name.clone().into();

        std::fs::write(page.controller_path(), component)?;
        std::fs::write(page.view_path(), page.view())?;

        // unix is the only platform to run on
        std::os::unix::fs::symlink(script_name, CGI_BIN)?;

        routes.push(page.route(idx + 1));
    }

    let route_template = ROUTES
        .replace("__ROUTE_COUNT__", routes.len().to_string().as_str())
        .replace("__ROUTES__", routes.join("\n").as_str());

    std::fs::write("./config.cbl", route_template)?;

    return Ok(());
}
