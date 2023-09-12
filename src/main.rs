use std::path::PathBuf;

mod cli;
mod pack;
mod structs;
mod wrapped;

#[tokio::main]
async fn main() {
    let matches = cli::cli().get_matches();

    match matches.subcommand() {
        Some(("pack", args)) => {
            let dir: &PathBuf = args
                .get_one("directory")
                .expect("problem directory path should be provided");
            let output: &PathBuf = args
                .get_one("output")
                .expect("output directory path should be provided");

            pack::pack(dir, output).await;
        }
        Some(_) | None => {
            let _ = cli::cli().print_help();
        }
    }
}
