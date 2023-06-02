use crate::convert::Converter;
use crate::structs::Problem;
use std::fs;
use std::path::PathBuf;

pub fn pack(dir: &PathBuf, output: &PathBuf, converter: &dyn Converter) {
    fs::create_dir_all(output).expect("Failed to create output directory");

    let entries = fs::read_dir(dir).expect("Directory not found");

    let mut entries: Vec<_> = entries
        .map(|entry| entry.expect("Failed to read directory entry"))
        .collect();
    entries.sort_by_key(|a| a.file_name());

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

                fs::create_dir_all(output.join(&subdir_name))
                    .expect("Failed to create output directory");

                let specs = converter.convert(&problem);

                let specs_json = serde_json::to_string(&specs).expect("Failed to convert to JSON");

                fs::write(output.join(&subdir_name).join("specs.json"), specs_json)
                    .expect("Failed to write to file");
            }
        }
    }
}
