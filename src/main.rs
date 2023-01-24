use dotenv::dotenv;
use std::path::Path;

use mrot_core::{file_io::read_orchestration_file, Orchestration, OrchestrationYml};

use clap::Parser;

pub mod mrot_core;

///Runs the setup for the program this includes
/// --Reading from an orchestration.yml file
///
fn setup(orchestration_name: &str) -> OrchestrationYml {
    //read file
    let path = Path::new(orchestration_name);

    read_orchestration_file::<OrchestrationYml>(path)
        .expect("Failed to read the orchestration file")
}

/// MROT
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///Name of the orchestration file
    #[arg(short, long)]
    name: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let args = Args::parse();

    let orchestrator = setup(&args.name);
    let orchestration = orchestrator.generate_orchestrations();
    match orchestration {
        Ok(sorted_indices) => {
            orchestrator
                .run_orchestration_dangerously(&sorted_indices)
                .await
                .expect("Failed to run orchestration");
        }
        Err(e) => {
            eprintln!("Failed to generate because a cycle was detected at {:?}", e);
        }
    }
}
