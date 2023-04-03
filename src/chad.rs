use anyhow::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::{
    pages::{Page, CONTROLLERS, VIEWS, COW_PATH},
    template::{ROUTES, COW, COW_TEMPLATE},
};

pub fn chad() -> Result<()> {
    // set_watcher()?;

    let pages = PathBuf::from("pages");

    let htmlers = WalkDir::new(&pages)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|dir| dir.file_name() == "index.html");

    let mut routes: Vec<String> = vec![];

    _ = std::fs::create_dir_all(CONTROLLERS);
    _ = std::fs::create_dir_all(VIEWS);
    _ = std::fs::remove_file(COW_PATH);

    for (idx, path) in htmlers.into_iter().enumerate() {
        let page: Page = path.path().try_into()?;

        let component = page.controller();

        std::fs::write(page.controller_path(), component)?;
        std::fs::write(page.view_path(), page.view())?;

        routes.push(page.route(idx + 1));
    }

    let route_template = ROUTES
        .replace("__ROUTE_COUNT__", routes.len().to_string().as_str())
        .replace("__ROUTES__", routes.join("\n").as_str());

    std::fs::write("./config.cbl", route_template)?;
    std::fs::write(COW_PATH, COW)?;
    std::fs::write(COW_TEMPLATE_PATH, COW_TEMPLATE)?;

    return Ok(());
}
