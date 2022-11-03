pub mod mrot_core;
pub mod term_ui;

use dotenv::dotenv;
use std::{io, path::Path, time::Duration};
use tui::{
    backend::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
    Terminal,
};

use tokio::time::sleep;

use mrot_core::{file_io::read_orchestration_file, Orchestration, OrchestrationYml};

use term_ui::{run_tui_until_user_exit, setup_terminal, TuiApp};

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
async fn main() -> Result<(), io::Error> {
    dotenv().ok();

    let app = TuiApp::new("Multi repo Orchestration tool");
    let tick_rate = Duration::from_millis(250);
    let mut terminal = setup_terminal().expect("Failed to create terminal");
    run_tui_until_user_exit(&mut terminal, app, tick_rate);

    Ok(())

    // let orchestrator = setup();
    // let orchestration = orchestrator.generate_orchestrations();
    // match orchestration {
    //     Ok(sorted_indices) => {
    //         orchestrator
    //             .run_orchestration_dangerously(&sorted_indices)
    //             .await
    //             .expect("Failed to run orchestration");

    //     }
    //     Err(e) => {
    //         eprintln!("Failed to generate because a cycle was detected at {:?}", e);

    //     }
    // }
}
