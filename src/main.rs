use std::path::Path;

use notify::{RecursiveMode, Watcher};
use walkdir::WalkDir;
use anyhow::Result;

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

fn main() -> Result<()> {
    set_watcher()?;

    let mut pwd = std::env::current_dir()?;
    pwd.push("pages");

    let htmlers = WalkDir::new(&pwd)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|dir| dir.file_name() == "index.html");

    for path in htmlers {
        println!("{:?}", path);
    }

    return Ok(());
}


