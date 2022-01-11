use std::env;
use std::process::Command;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Missing input");
    }
    println!("{:?}", args);

    args.remove(0);
    let input = format!("/github/workspace/{}", args.remove(0).trim_start_matches("/"));
    let output = format!("/github/workspace/{}", args.remove(0).trim_start_matches("/"));
    let options = args.first().unwrap_or(&"-Os".to_owned());
    Command::new("wasm-opt")
            .args([input, "-o", output, options])
            .output()
            .expect("failed to execute command");
}
