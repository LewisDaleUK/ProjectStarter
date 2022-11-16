use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::collections::HashMap;
use serde_derive::{Deserialize, Serialize};
use git2::Repository;
use dirs::home_dir;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RepoSource {
    pub title: String,
    pub language: String,
    pub description: String,
    source: String,
    command: Option<String>,
}

impl RepoSource {
    pub fn create(&self, directory: &str) {
        let path = Path::new(directory);

        let repo = match Repository::clone(&self.source, path) {
          Ok(repo) => repo,
          Err(e) => panic!("failed to clone: {}", e),
        };
        repo.remote_delete("origin").ok();

        self.command.as_ref().map(|command| Command::new(command)
                .current_dir(path.canonicalize().unwrap())
                .output()
                .unwrap_or_else(|_| panic!("Failed to execute start command for {}", &self.title)));
    }

    pub fn clone(&self) -> RepoSource {
        RepoSource {
            title: String::from(&self.title),
            language: String::from(&self.language),
            description: String::from(&self.description),
            source: String::from(&self.source),
            command: self.command.clone()
        }
    }

}

pub fn load_options() -> Option<Vec<RepoSource>> {
    let home = home_dir().unwrap();
    let home_path = home.to_str().unwrap();

    let files: Vec<PathBuf> = vec![
        Path::new(&format!("{}/{}", home_path, ".config/projects.json")),
        Path::new(&format!("{}/{}", home_path, ".projects.json")),
        Path::new("./projects.json")
    ].into_iter()
        .filter(|x| x.exists())
        .map(|x| x.canonicalize().unwrap())
        .collect();

    let mut parsed_map = HashMap::new();

    for source in &files {
        let mut file = File::open(source).ok()?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).ok();
        let parsed: Vec<RepoSource> = serde_json::from_str(&contents).ok()?;

        for opt in parsed {
            parsed_map.insert(String::from(&opt.title), opt.clone());
        }
    }

    Some(Vec::from_iter(parsed_map.values().cloned()))
}