use std::path::PathBuf;

use anyhow::Result;
use walkdir::WalkDir;

use crate::{pages::Page, opts::Opts};

pub fn link(opts: Opts) -> Result<()> {

    let pages = PathBuf::from("pages");

    let htmlers = WalkDir::new(&pages)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|dir| dir.file_name() == "index.html");

    let cgi_bin = PathBuf::from("cgi-bin");
    _ = std::fs::remove_dir_all(&cgi_bin);
    _ = std::fs::create_dir(&cgi_bin);

    let the_cow = PathBuf::from(opts.cow_path);

    for page in htmlers {
        let page: Page = page.path().try_into()?;

        let script_name: String = page.script_name.into();
        let mut the_moon = cgi_bin.clone();
        the_moon.push(script_name);

        std::os::unix::fs::symlink(&the_cow, the_moon)?;
    }

    return Ok(());
}
