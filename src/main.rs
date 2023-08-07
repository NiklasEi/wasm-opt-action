use glob::glob;
use std::io::Write;
use std::process::Command;
use std::{env, io};

fn main() {
    // Github passes empty strings for not defined optional parameters.
    // We have multiple optional params, so we need to keep their order.
    let mut args: Vec<Option<String>> = env::args()
        .map(|arg| if !arg.is_empty() { Some(arg) } else { None })
        .collect();
    if args.iter().filter(|arg| arg.is_some()).count() < 2 {
        panic!("You need to at least define a file to optimize");
    }
    args.remove(0);
    println!("Got arguments: {:?}", args);
    args.reverse();

    let glob_input = format!(
        "/github/workspace/{}",
        args.pop().unwrap().unwrap().trim_start_matches('/')
    );
    println!("Searching for file matching '{glob_input}'");
    let inputs = glob(&glob_input).expect("Failed to read glob pattern");
    let optimize_all = args.pop().unwrap().unwrap_or("false".to_owned());

    for path in inputs {
        let input = path
            .expect("Failed to read path")
            .to_str()
            .expect("Path should be string")
            .to_owned();
        println!("Optimizing '{input}'");

        let output = if let Some(output) = args.pop().unwrap() {
            format!("/github/workspace/{}", output.trim_start_matches('/'))
        } else {
            input.clone()
        };
        println!("Writing optimized wasm file to '{output}'");

        let options = args.pop().unwrap().unwrap_or("-Os".to_owned());
        println!("Executing 'wasm-opt {input} -o {output} {options}'");
        let output = Command::new("wasm-opt")
            .args([input, "-o".to_owned(), output])
            .args(shell_words::split(&options).expect("Failed to parse options"))
            .output()
            .expect("failed to execute command");
        io::stdout()
            .write_all(&output.stdout)
            .expect("failed to write to stdout");
        io::stderr()
            .write_all(&output.stderr)
            .expect("failed to write to stderr");
        if optimize_all != "true" {
            break;
        }
    }
}
