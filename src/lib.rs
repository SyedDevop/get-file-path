use clap::{Parser, ValueEnum};

use rfd::FileDialog;
use std::process;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct MyArgs {
    /// File Select type (file || folder)
    #[arg(value_enum)]
    pub select: SelectFile,

    /// commands and it's  arguments to run after changing to the specified directory.
    /// For example, -- ls -al would run the `ls -al`.
    #[arg(last = true)]
    pub cmd: Vec<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum SelectFile {
    /// Select a single File form the file browser.
    File,
    /// Select a Folder from the file browser.
    Folder,
}
pub struct ConfigArgs {
    pub is_folder: bool,
    pub path: String,
    pub parent_path: String,
}

impl ConfigArgs {
    pub fn new(selectfile: &SelectFile) -> ConfigArgs {
        let file = match selectfile {
            SelectFile::Folder => FileDialog::new().pick_folder(),
            SelectFile::File => FileDialog::new().pick_file(),
        };
        if let Some(filepath) = file {
            let p_path: String = filepath.parent().unwrap().display().to_string();
            ConfigArgs {
                is_folder: filepath.is_dir(),
                path: filepath.display().to_string(),
                parent_path: p_path,
            }
        } else {
            process::exit(0);
        }
    }
}
