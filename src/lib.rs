use clap::ValueEnum;

use rfd::FileDialog;
use std::path::PathBuf;
use std::process::Command;
use std::{env, iter, process};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum SelectFile {
    /// Select a single File form the file browser.
    File,
    /// Select a Folder from the file browser.
    Folder,
}

/// A struct that represents configuration arguments.
///
/// This struct contains two fields: `path` and `str_path`.
pub struct ConfigArgs {
    /// The `path` field is a `PathBuf` that represents a file path.
    pub path: PathBuf,
    /// The `str_path` field is a `String` that represents the same path as a string.
    pub str_path: String,
}

impl ConfigArgs {
    pub fn new(selectfile: &SelectFile) -> Self {
        let file = match selectfile {
            SelectFile::Folder => FileDialog::new().pick_folder(),
            SelectFile::File => FileDialog::new().pick_file(),
        };
        if let Some(filepath) = file {
            Self::set_directory(&filepath);
            ConfigArgs {
                str_path: filepath.display().to_string(),
                path: filepath,
            }
        } else {
            process::exit(0);
        }
    }
    /// Set the current directory to give Path.
    pub fn set_directory(path: &PathBuf) {
        let set_path = if path.is_dir() {
            path
        } else {
            path.parent().expect("Failed to get parent directory")
        };
        env::set_current_dir(set_path).expect("Failed to change current directory");
    }
    pub fn run_cmd(cmd_list: &[String]) {
        Command::new("cmd")
            .args(Self::parse_cmd_args(cmd_list))
            .spawn()
            .expect("failed to execute process");
    }
    fn parse_cmd_args(cmds: &[String]) -> Vec<&str> {
        iter::once("/C")
            .chain(cmds.iter().map(|e| e.as_str()))
            .collect()
    }
}

