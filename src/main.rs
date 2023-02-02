use glob::glob;
use std::env;
use std::process::Command;

fn main() {
    // Github passes empty strings for not defined optional parameters.
    // We have multiple optional params, so we need to keep their order.
    let args = env::args()
        .map(|arg| if arg.len() > 0 { Some(arg) } else { None })
        .collect();
    let config = parse(args, "/github/workspace/".to_owned());
    run(config);
}

fn parse(mut args: Vec<Option<String>>, workspace: String) -> ActionConfig {
    if args.iter().filter(|arg| arg.is_some()).count() < 2 {
        panic!("You need to at least define a file to optimize");
    }
    args.remove(0);
    println!("Got arguments: {:?}", args);
    args.reverse();
    let glob_input = format!(
        "{workspace}{}",
        args.pop().unwrap().unwrap().trim_start_matches("/")
    );
    println!("Searching for file matching '{glob_input}'");
    let mut files: Vec<String> = glob(&glob_input)
        .expect("Failed to read glob pattern")
        .map(|path| {
            path.expect("Failed to read path")
                .to_str()
                .expect("Path should be string")
                .to_owned()
        })
        .collect();

    if files.is_empty() {
        panic!("No file found matching '{glob_input}'");
    }
    let output = args
        .pop()
        .unwrap()
        .map(|output| format!("/github/workspace/{}", output.trim_start_matches("/")));
    let optimize_all = args.pop().unwrap().unwrap_or("false".to_owned());
    if optimize_all != "true" {
        files.resize(1, String::new())
    }
    let options = args.pop().unwrap().unwrap_or("-Os".to_owned());

    ActionConfig {
        files,
        output,
        options,
    }
}

fn run(action: ActionConfig) {
    let options = action.options;

    for input in action.files {
        let output = action.output.clone().unwrap_or(input.clone());
        println!("Optimizing '{input}'");
        println!("Writing optimized wasm file to '{output}'");
        println!("Executing 'wasm-opt {input} -o {output} {options}'");
        Command::new("wasm-opt")
            .args([input, "-o".to_owned(), output.clone(), options.clone()])
            .output()
            .expect("failed to execute command");
    }
}

#[derive(PartialEq, Debug)]
struct ActionConfig {
    files: Vec<String>,
    output: Option<String>,
    options: String,
}

#[cfg(test)]
mod tests {
    use crate::{parse, ActionConfig};

    #[test]
    fn single_file() {
        assert_eq!(
            parse(
                vec![
                    Some("executable".to_owned()),
                    Some("*.md".to_owned()),
                    None,
                    None,
                    None
                ],
                "./".to_owned()
            ),
            ActionConfig {
                files: vec!["Deployment.md".to_owned()],
                output: None,
                options: "-Os".to_owned()
            }
        )
    }

    #[test]
    fn multiple_files() {
        assert_eq!(
            parse(
                vec![
                    Some("executable".to_owned()),
                    Some("*.md".to_owned()),
                    None,
                    Some("true".to_owned()),
                    None
                ],
                "./".to_owned()
            ),
            ActionConfig {
                files: vec!["Deployment.md".to_owned(), "Readme.md".to_owned()],
                output: None,
                options: "-Os".to_owned()
            }
        )
    }

    #[test]
    fn custom_options() {
        assert_eq!(
            parse(
                vec![
                    Some("executable".to_owned()),
                    Some("*.md".to_owned()),
                    None,
                    None,
                    Some("custom".to_owned())
                ],
                "./".to_owned()
            ),
            ActionConfig {
                files: vec!["Deployment.md".to_owned()],
                output: None,
                options: "custom".to_owned()
            }
        )
    }
}
