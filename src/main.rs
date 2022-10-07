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
    println!("{:?}", orchestrator);
    orchestrator
        .run_orchestration_safe()
        .await
        .expect("failed to run orchestration");
}
