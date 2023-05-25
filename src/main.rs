use crate::structs::{Problem, ProblemSummary};
use std::env;
use std::fs;
use std::io::Write;


mod structs;

fn main() {
    // Fetch directory from command line arguments.
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <directory>", args[0]);
        return;
    }
    let dir = &args[1];

    // Vector to hold all problem summaries.
    let mut summaries: Vec<ProblemSummary> = Vec::new();

    // Recursively walk through the directory and its subdirectories.
    for entry in fs::read_dir(dir).expect("Directory not found") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.is_dir() {
            let subdir_name = path
                .file_name()
                .expect("Failed to get subdir name")
                .to_str()
                .expect("Failed to convert subdir name to string")
                .to_owned();

            let problem_path = path.join("problem.toml");
            if problem_path.exists() {
                let content =
                    fs::read_to_string(&problem_path).expect("Failed to read problem.toml");

                let problem: Problem =
                    toml::from_str(&content).expect("Failed to parse problem.toml");

                let summary = ProblemSummary {
                    id: subdir_name,
                    name: problem.name,
                    tags: problem.tags.unwrap_or(Vec::new()),
                };
                summaries.push(summary);
            }
        }
    }

    // Convert summaries to JSON.
    let json = serde_json::to_string(&summaries).expect("Failed to convert to JSON");

    let mut file = fs::File::create("index.json").expect("Failed to create file");
    file.write_all(json.as_bytes())
        .expect("Failed to write to file");

    println!("Successfully wrote summaries to index.json");
}
