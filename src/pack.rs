use crate::convert::{self, Converter};
use crate::structs::Problem;
use futures::future::join_all;
use std::fs;
use std::path::PathBuf;
use tokio::task;

pub async fn pack(dir: &PathBuf, output: &PathBuf, target: &str) {
    fs::create_dir_all(output).expect("Failed to create output directory");

    let entries = fs::read_dir(dir).expect("Directory not found");

    let mut entries: Vec<_> = entries
        .map(|entry| entry.expect("Failed to read directory entry"))
        .collect();
    entries.sort_by_key(|a| a.file_name());

    let mut tasks = Vec::new();

    for entry in entries {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let problem_path = path.join("problem.toml");
        if !problem_path.exists() {
            continue;
        }

        let subdir_name = path
            .file_name()
            .expect("Failed to get subdir name")
            .to_str()
            .expect("Failed to convert subdir name to string")
            .to_owned();

        let output_dir = output.join(&subdir_name);
        let output_spec = output_dir.join("specs.json");

        let content = fs::read_to_string(&problem_path).expect("Failed to read problem.toml");
        let problem: Problem = toml::from_str(&content).expect("Failed to parse problem.toml");

        fs::create_dir_all(output_dir).expect("Failed to create output directory");

        let target = target.to_owned();
        let dir = dir.join(&subdir_name);

        let task = task::spawn(async move {
            let specs = match target.as_str() {
                "io-fast" => {
                    let converter = convert::io_fast::IOFastConverter {};
                    converter.convert(&problem, &dir).await
                }
                _ => panic!("Unknown target: {}", target),
            };

            let specs_json = serde_json::to_string(&specs).expect("Failed to convert to JSON");

            fs::write(output_spec, specs_json).expect("Failed to write to file");
        });

        tasks.push(task);
    }

    join_all(tasks).await;
}
