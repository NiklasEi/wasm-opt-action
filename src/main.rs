use glob::glob;
use std::env;
use std::process::Command;

fn main() {
    let mut args: Vec<String> = env::args().filter(|arg| arg.len() > 0).collect();
    if args.len() < 2 {
        panic!("You need to at least define a file to optimize");
    }
    args.remove(0);
    println!("Got arguments: {:?}", args);
    args.reverse();

    let glob_input = format!(
        "/github/workspace/{}",
        args.pop().unwrap().trim_start_matches("/")
    );
    println!("Searching for file matching '{glob_input}'");
    let input = glob(&glob_input)
        .expect("Failed to read glob pattern")
        .next()
        .expect("No path found for Glob")
        .expect("Failed to read path")
        .to_str()
        .expect("Path should be string")
        .to_owned();
    println!("Optimizing '{input}'");

    let output = if let Some(output) = args.pop() {
        format!("/github/workspace/{}", output.trim_start_matches("/"))
    } else {
        input.clone()
    };
    println!("Writing optimized wasm file to '{output}'");

    let options = args.first().unwrap_or(&"-Os".to_owned()).to_owned();
    Command::new("wasm-opt")
        .args([input, "-o".to_owned(), output, options])
        .output()
        .expect("failed to execute command");
}
