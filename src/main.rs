use dotenv::dotenv;
use std::path::Path;

use mrot_core::{file_io::read_orchestration_file, Orchestration, OrchestrationYml};

pub mod mrot_core;

///Runs the setup for the program this includes
/// --Reading from an orchestration.yml file
///
fn setup() -> OrchestrationYml {
    //read file
    let path = Path::new("orchestration.yml");

    read_orchestration_file::<OrchestrationYml>(path)
        .expect("Failed to read the orchestration file")
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let orchestrator = setup();
    let orchestration = orchestrator.generate_orchestrations();
    match orchestration {
        Ok(sorted_indices) => {
            orchestrator
                .run_orchestration_dangerously(&sorted_indices)
                .await
                .expect("Failed to run orchestration");
            return ();
        }
        Err(e) => {
            eprintln!("Failed to generate because a cycle was detected at {:?}", e);
            return ();
        }
    }
}
