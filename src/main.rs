use glob::glob;
use std::env;
use std::process::Command;

fn main() {
    // Github passes empty strings for not defined optional parameters.
    // We have multiple optional params, so we need to keep their order.
    let mut args: Vec<Option<String>> = env::args()
        .map(|arg| if arg.len() > 0 { Some(arg) } else { None })
        .collect();
    if args.iter().filter(|arg| arg.is_some()).count() < 2 {
        panic!("You need to at least define a file to optimize");
    }
    args.remove(0);
    println!("Got arguments: {:?}", args);
    args.reverse();

    let glob_input = format!(
        "/github/workspace/{}",
        args.pop().unwrap().unwrap().trim_start_matches("/")
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

    let output = if let Some(output) = args.pop().unwrap() {
        format!("/github/workspace/{}", output.trim_start_matches("/"))
    } else {
        input.clone()
    };
    println!("Writing optimized wasm file to '{output}'");

    let options = args.pop().unwrap().unwrap_or("-Os".to_owned());
    println!("Executing 'wasm-opt {input} -o {output} {options}'");
    Command::new("wasm-opt")
        .args([input, "-o".to_owned(), output, options])
        .output()
        .expect("failed to execute command");
}
