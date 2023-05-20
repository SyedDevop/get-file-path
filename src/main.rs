use std::process::exit;

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
    let parsed_args = MyArgs::parse();

    let config = ConfigArgs::new(&parsed_args.select);

    if parsed_args.out {
        println!("{:}", config.str_path);
        exit(0);
    }

    let mut cmd = parsed_args.cmd;
    if config.path.is_file() {
        cmd.push(config.str_path);
    }
    ConfigArgs::run_cmd(&cmd)
    // process::exit(0);
}
