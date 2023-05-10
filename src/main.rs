// use rfd::FileDialog;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args)
    // let files = FileDialog::new().pick_folder();
    // if files.is_none() {
    //     print!("User did not choose")
    // } else {
    //     println!("The user choose: {:#?}", files);
    // }
    
}

fn parse_config(args: &[String]) -> &str {
    let query = &args[1];
    &query
}
