// use rfd::FileDialog;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File Select type (file || folder)
    #[arg(short, long, get_possible_values= &["file", "folder"])]
    select: String,
}
fn main() {
    let args = Args::parse();

    for _ in 0..10 {
        println!("Hello {}!", args.select)
    } // let files = FileDialog::new().pick_folder();
      // if files.is_none() {
      //     print!("User did not choose")
      // } else {
      //     println!("The user choose: {:#?}", files);
      // }
}
