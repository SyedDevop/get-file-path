use std::path::Path;
use std::process;
use std::process::Command;
use std::{env, iter};

use clap::{Parser, ValueEnum};
use rfd::FileDialog;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct MyArgs {
    /// File Select type (file || folder)
    #[arg(value_enum)]
    select: SelectFile,

    /// commands and it's  arguments to run after changing to the specified directory.
    /// For example, -- ls -al would run the `ls -al`.
    #[arg(last = true)]
    cmd: Vec<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum SelectFile {
    /// Select a single File form the file browser.
    File,
    /// Select a Folder from the file browser.
    Folder,
}

fn main() {
    let selectfile = MyArgs::parse();

    let my_path = pars_select_arg(&selectfile.select);
    set_directory(&my_path);

    let cmd_args = parse_cmd_args(&selectfile.cmd);

    Command::new("cmd")
        .args(cmd_args)
        .output()
        .expect("failed to execute process");
}

fn pars_select_arg(selectfile: &SelectFile) -> String {
    let file = match selectfile {
        SelectFile::Folder => FileDialog::new().pick_folder(),
        SelectFile::File => FileDialog::new().pick_file(),
    };
    if let Some(filepath) = file {
        filepath.display().to_string()
    } else {
        process::exit(0);
    }
}

/// Set the current directory to give Path.
fn set_directory(path: &str) {
    env::set_current_dir(Path::new(path)).expect("Failed to change current directory");
}

fn parse_cmd_args(cmds: &[String]) -> Vec<&str> {
    iter::once("/C")
        .chain(cmds.iter().map(|e| e.as_str()))
        .collect()
}
