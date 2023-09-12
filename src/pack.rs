use crate::structs::ProblemSummary;
use crate::wrapped::{PartialProblem, Problem};
use futures::future::join_all;
use std::fs;
use std::path::PathBuf;
use tokio::task;
use url::{ParseError, Url};

pub async fn pack(dir: &PathBuf, output: &PathBuf) {
    fs::create_dir_all(output).expect("Failed to create output directory");

    let entries = fs::read_dir(dir).expect("Directory not found");

    let mut entries: Vec<_> = entries
        .map(|entry| entry.expect("Failed to read directory entry"))
        .collect();
    entries.sort_by_key(|a| a.file_name());

    let mut tasks = Vec::new();
    let mut summaries: Vec<ProblemSummary> = Vec::new();

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
        let output_spec = output_dir.join("spec.json");

        let content = fs::read_to_string(&problem_path).expect("Failed to read problem.toml");
        let raw_problem: PartialProblem =
            toml::from_str(&content).expect("Failed to parse problem.toml");

        let summary = ProblemSummary {
            id: subdir_name.clone(),
            name: raw_problem.name.clone(),
            tags: raw_problem.tags.clone().unwrap_or(Vec::new()),
        };
        summaries.push(summary);

        fs::create_dir_all(output_dir).expect("Failed to create output directory");

        let mut problem = Problem::from(raw_problem);
        problem.id = subdir_name;

        // for all testcases, if the input is not an url, add "txt:" prefix
        for testcase in problem.testcases.iter_mut() {
            let layers = vec!["prep".to_owned(), "eval".to_owned(), "final".to_owned()];
            for layer in layers.iter() {
                if let Some(layer) = testcase.fs.get_mut(layer) {
                    for (_key, value) in layer.iter_mut() {
                        let parsed = Url::parse(value);
                        if let Err(e) = parsed {
                            if e == ParseError::RelativeUrlWithoutBase {
                                *value = format!("txt:{}", value);
                            }
                        }
                        if let Ok(parsed) = parsed {
                            if parsed.scheme() == "rfile" {
                                let relative_path = parsed.path();
                                let content = fs::read_to_string(path.join(
                                    relative_path.strip_prefix('/').unwrap_or(relative_path),
                                ))
                                .unwrap_or_else(|_| {
                                    panic!("Failed to read file {}", relative_path)
                                });
                                *value = format!("txt:{}", content);
                            }
                        }
                    }
                }
            }
        }

        let task = task::spawn(async move {
            fs::write(output_spec, serde_json::to_string(&problem).unwrap())
                .expect("Failed to write spec.json");
        });

        tasks.push(task);
    }

    join_all(tasks).await;

    let index_json = serde_json::to_string(&summaries).expect("Failed to convert to JSON");
    let index_path = output.join("index.json");
    fs::write(index_path, index_json).expect("Failed to write index.json");

    println!("Successfully wrapped problem box to {}", output.display());
}
