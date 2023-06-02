use std::path::PathBuf;

use clap::{arg, value_parser, Command};

pub fn cli() -> Command {
    Command::new("box")
        .version(format!(
            "{} {}",
            env!("VERGEN_GIT_SHA"),
            env!("VERGEN_CARGO_TARGET_TRIPLE")
        ))
        .about("WASM OJ Problem Box Manager")
        .author("Jacob Lin <jacob@csie.cool>")
        .subcommand(
            Command::new("index")
                .about("make index for problems")
                .args(&[
                    arg!(-o --output <file> "a path to the output index file")
                        .default_value("index.json")
                        .value_parser(value_parser!(PathBuf)),
                    arg!([directory] "a path to the problem directory")
                        .default_value("problem")
                        .value_parser(value_parser!(PathBuf)),
                ]),
        )
        .subcommand(
            Command::new("pack")
                .about("pack problems into judge specs")
                .args(&[
                    arg!(-o --output <directory> "a path to the output directory")
                        .default_value("spec")
                        .value_parser(value_parser!(PathBuf)),
                    arg!(-s --spec <spec> "the target spec")
                        .default_value("io-fast")
                        .value_parser(value_parser!(String)),
                    arg!([directory] "a path to the problem directory")
                        .default_value("problem")
                        .value_parser(value_parser!(PathBuf)),
                ]),
        )
}
