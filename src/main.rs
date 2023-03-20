mod template;
mod pages;

use std::path::{Path, PathBuf};

use notify::{RecursiveMode, Watcher};
use walkdir::WalkDir;
use anyhow::Result;

use crate::pages::Page;

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

    let pages = PathBuf::from("pages");

    let htmlers = WalkDir::new(&pages)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|dir| dir.file_name() == "index.html");

    for path in htmlers {
        let page: Page = path.path().try_into()?;
        let component = page.component();

        std::fs::write(page.component_path(), component)?;
        std::fs::write(page.view_path(), page.contents)?;
    }

    return Ok(());
}


