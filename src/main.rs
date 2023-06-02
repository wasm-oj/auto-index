use std::path::PathBuf;

mod cli;
mod convert;
mod index;
mod pack;
mod structs;

fn main() {
    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("index", args)) => {
            let dir: &PathBuf = args
                .get_one("directory")
                .expect("problem directory path should be provided");
            let output: &PathBuf = args
                .get_one("output")
                .expect("output file path should be provided");

            index::index(dir, output);
        }
        Some(("pack", args)) => {
            let dir: &PathBuf = args
                .get_one("directory")
                .expect("problem directory path should be provided");
            let output: &PathBuf = args
                .get_one("output")
                .expect("output directory path should be provided");
            let spec: &String = args.get_one("spec").expect("spec should be provided");

            match spec.as_str() {
                "io-fast" => {
                    pack::pack(dir, output, &convert::io_fast::IOFastConverter {});
                }
                _ => {
                    let _ = cli::cli().print_help();
                }
            }
        }
        Some(_) | None => {
            let _ = cli::cli().print_help();
        }
    }
}
