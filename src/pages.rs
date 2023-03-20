use std::path::{Path, PathBuf};

use anyhow::anyhow;

use crate::template::{self, COMPONENT};

#[derive(Debug, Clone)]
pub enum Chunk {
    Text(String),
    Variable(String),
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
        if value.starts_with(":") {
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
}

impl Page {
    pub fn controller(&self) -> String {
        let vars = self.path
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

        let script_name: String = self.script_name.clone().into();

        return COMPONENT
            .replace("__SCRIPT_NAME__", &script_name)
            .replace("__VARIABLES__", &vars);
    }

    pub fn controller_path(&self) -> PathBuf {
        let script_name: String = self.script_name.clone().into();
        return PathBuf::from(format!("controllers/{}.cbl", &script_name));
    }

    pub fn view_path(&self) -> PathBuf {
        let script_name: String = self.script_name.clone().into();
        return PathBuf::from(format!("views/{}.cow", &script_name));
    }

    pub fn to_route(&self) -> String {
        todo!("me daddy");
    }
}

impl From<Page> for PathBuf {
    fn from(value: Page) -> Self {
        let script_name: String = value.script_name.into();
        let mut out = PathBuf::from(script_name);

        for chunk in value.path {
            let chunk: String = chunk.into();
            out.push(chunk);
        }

        let html: String = value.html.into();

        return out;
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

        if string_pieces.len() < 2 {
            return Err(anyhow!("requires a script name and an html file"));
        }

        // todo: i hate this
        let script_name = string_pieces.remove(0);
        let html = string_pieces.pop().expect("an html file");

        if !html.ends_with(".html") {
            return Err(anyhow!("requires an html file to generate the path"));
        }

        return Ok(Page {
            script_name: script_name.into(),
            html: html.into(),
            path: string_pieces.into_iter().map(|x| x.into()).collect(),
            contents: std::fs::read_to_string(value)?,
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
