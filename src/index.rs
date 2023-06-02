use std::fs;
use std::{io::Write, path::PathBuf};

use crate::structs::{Problem, ProblemSummary};

pub fn index(dir: &PathBuf, output: &PathBuf) {
    // Vector to hold all problem summaries.
    let mut summaries: Vec<ProblemSummary> = Vec::new();

    // Read all problem directories.
    let entries = fs::read_dir(dir).expect("Directory not found");

    // Sort the entries by name.
    let mut entries: Vec<_> = entries
        .map(|entry| entry.expect("Failed to read directory entry"))
        .collect();
    entries.sort_by_key(|a| a.file_name());

    // Iterate over all problem directories.
    for entry in entries {
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

    // Write JSON to file.
    let parent = output.parent().expect("Failed to get parent directory");
    fs::create_dir_all(parent).expect("Failed to create parent directory");
    let mut file = fs::File::create(output).expect("Failed to create file");
    file.write_all(json.as_bytes())
        .expect("Failed to write to file");

    println!("Successfully wrote summaries to {}", output.display());
}
