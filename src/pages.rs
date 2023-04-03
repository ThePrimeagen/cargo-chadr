// NOTE: To all future readers.
// I am sorry for what i have done with the index file.  Special casing always makes me emotional,
// and in this case, i just went for it.  I just decided to throw away years of good practice for
// one sweet moment of victory.  Victory, so near so dear, so ready to be penetrated into the lands
// of success.
//
// Your truly
//
//
// ThePrimeagen
use std::{
    fmt::Display,
    path::{Path, PathBuf},
};

use anyhow::anyhow;

use crate::template::{self, CONTROLLER, ROUTE, VIEW};

pub const CONTROLLERS: &str = "controllers";
pub const CGI_BIN: &str = "cgi-bin";
pub const VIEWS: &str = "views";
pub const COW_PATH: &str = "cow.cbl";
pub const COW_TEMPLATE_PATH: &str = "cowtemplate.cbl";

#[derive(Debug, Clone)]
pub enum Chunk {
    Text(String),
    Variable(String),
}

impl Chunk {
    pub fn to_path_string(&self) -> String {
        match self {
            Chunk::Text(value) => return value.clone(),
            Chunk::Variable(value) => return format!("%{}", value),
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Chunk::Text(value) => write!(f, "{}", value),
            Chunk::Variable(value) => write!(f, "%{}", value),
        }
    }
}

impl From<Chunk> for String {
    fn from(value: Chunk) -> Self {
        match value {
            Chunk::Text(value) => return value,
            Chunk::Variable(value) => return value,
        }
    }
}

impl From<String> for Chunk {
    fn from(value: String) -> Self {
        if value.starts_with("%") {
            return Chunk::Variable(value[1..].to_string());
        }
        return Chunk::Text(value);
    }
}

#[derive(Debug)]
pub struct Page {
    pub script_name: Chunk,
    pub html: Chunk,
    pub path: Vec<Chunk>,
    pub contents: String,
    pub is_index: bool,
}

impl Page {
    fn route_pattern(&self, script_name: String) -> String {
        if script_name == "/" {
            return String::from("/");
        }

        let mut path = self
            .path
            .iter()
            .map(|chunk| chunk.to_path_string())
            .collect::<Vec<String>>();

        path.insert(0, self.script_name.to_path_string());

        let mut route_path = path.join("/");
        if !route_path.starts_with("/") {
            route_path.insert_str(0, "/");
        }
        return route_path;
    }
    pub fn controller(&self) -> String {
        let vars = self
            .path
            .iter()
            .filter(|x| match x {
                Chunk::Variable(_) => true,
                _ => false,
            })
            .enumerate()
            .map(|(idx, chunk)| {
                let chunk: String = chunk.clone().into();
                let var = template::VARIABLE
                    .replace("__NAME__", &chunk)
                    .replace("__INDEX__", &(idx + 1).to_string());

                return var;
            })
            .collect::<String>();

        let mut script_name: String = self.script_name.clone().into();

        return CONTROLLER
            .replace("__SCRIPT_NAME__", &script_name)
            .replace("__VARIABLES__", &vars);
    }

    pub fn view(&self) -> String {
        let vars = self
            .path
            .iter()
            .filter(|x| match x {
                Chunk::Variable(_) => true,
                _ => false,
            })
            .map(|chunk| {
                let name: String = chunk.clone().into();
                return template::VIEW_VARIABLES.replace("__NAME__", &name);
            })
            .collect::<String>();

        return VIEW
            .replace("__TITLE__", &format!("{} - CHADstack", &self.script_name))
            .replace("__BODY__", &self.contents)
            .replace("__VARIABLES__", &vars);
    }

    pub fn route(&self, idx: usize) -> String {
        let mut script_name: String = self.script_name.clone().into();
        if script_name == "index" {
            script_name = String::from("/");
        }

        return ROUTE
            .replace("__DESTINY__", &script_name)
            .replace("__INDEX__", &idx.to_string())
            .replace("__PATTERN__", &self.route_pattern(script_name));
    }

    pub fn controller_path(&self) -> PathBuf {
        let script_name: String = self.script_name.clone().into();
        return PathBuf::from(format!("{}/{}.cbl", CONTROLLERS, &script_name));
    }

    pub fn view_path(&self) -> PathBuf {
        let script_name: String = self.script_name.clone().into();
        return PathBuf::from(format!("{}/{}.cow", VIEWS, &script_name));
    }
}

impl TryFrom<&Path> for Page {
    type Error = anyhow::Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        let mut string_pieces = value
            .iter()
            .skip(1)
            .map(|x| x.to_string_lossy().to_string())
            .collect::<Vec<String>>();

        let is_index = string_pieces
            .last()
            .unwrap_or(&String::from(""))
            .ends_with("index.html")
            && string_pieces.len() == 1;

        if string_pieces.len() < 2 && !is_index {
            return Err(anyhow!("requires a script name and an html file"));
        }
        let contents = std::fs::read_to_string(value)?;

        if is_index {
            return Ok(Page {
                is_index,
                html: Chunk::Text("index.html".into()),
                contents,
                path: vec![Chunk::Text("/".into())],
                script_name: Chunk::Text("index".into()),
            });
        }

        let script_name = string_pieces.remove(0);
        let html = string_pieces.pop().expect("an html file");

        if !html.ends_with(".html") {
            return Err(anyhow!("requires an html file to generate the path"));
        }

        return Ok(Page {
            script_name: script_name.into(),
            html: html.into(),
            path: string_pieces.into_iter().map(|x| x.into()).collect(),
            contents,
            is_index,
        });
    }
}

#[cfg(test)]
mod test {

    use anyhow::Result;

    #[test]
    fn happy_path() -> Result<()> {
        return Ok(());
    }
}
