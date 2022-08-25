use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use serde_derive::{Deserialize, Serialize};
use git2::Repository;

#[derive(Serialize, Deserialize, Clone)]
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

        match &self.command {
            Some(command) => Some(Command::new(command)
                .current_dir(path.canonicalize().unwrap())
                .output()
                .expect(&format!("Failed to execute start command for {}", &self.title))),
            None => None
        };
    }
}

pub fn load_options() -> Option<Vec<RepoSource>> {
    let mut file = File::open("projects.json").ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok();

    let parsed: Vec<RepoSource> = serde_json::from_str(&contents).ok()?;

    Some(parsed)
}


