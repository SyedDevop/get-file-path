use std::path::Path;
use std::process::{exit, Command};
use std::{env, iter};

use clap::Parser;

use gopa::{ConfigArgs, SelectFile};
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct MyArgs {
    /// File Select type (file || folder)
    #[arg(value_enum)]
    select: SelectFile,

    /// If out it returns the path with out executing
    #[arg(short, long, default_value_t = false)]
    out: bool,
    /// commands and it's  arguments to run after changing to the specified directory.
    /// For example, -- ls -al would run the `ls -al`.
    #[arg(last = true)]
    cmd: Vec<String>,
}
fn main() {
    let selectfile = MyArgs::parse();

    let my_path = ConfigArgs::new(&selectfile.select);

    if selectfile.out {
        println!("{:}", my_path.path);
        exit(0);
    }
    if my_path.is_folder {
        set_directory(&my_path.path);
        run_cmd(&selectfile.cmd);
    } else {
        set_directory(&my_path.parent_path);
        let mut new_cmd = selectfile.cmd;
        new_cmd.push(my_path.path);
        run_cmd(&new_cmd);
    }
    // process::exit(0);
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

fn run_cmd(cmd_list: &[String]) {
    Command::new("cmd")
        .args(parse_cmd_args(cmd_list))
        .spawn()
        .expect("failed to execute process");
}
