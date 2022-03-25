use glob::glob;
use std::env;
use std::process::Command;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Missing input");
    }
    args.remove(0);
    println!("Got arguments: {:?}", args);

    let glob_input = format!(
        "/github/workspace/{}",
        args.remove(0).trim_start_matches("/")
    );
    let input = glob(&glob_input)
        .expect("Failed to read glob pattern")
        .next()
        .expect("No path found fr Glob")
        .expect("Failed to read path")
        .to_str()
        .expect("Path should be string")
        .to_owned();
    println!("Optimizing {input}");
    let output = format!(
        "/github/workspace/{}",
        args.remove(0).trim_start_matches("/")
    );
    let options = args.first().unwrap_or(&"-Os".to_owned()).to_owned();
    Command::new("wasm-opt")
        .args([input, "-o".to_owned(), output, options])
        .output()
        .expect("failed to execute command");
}
