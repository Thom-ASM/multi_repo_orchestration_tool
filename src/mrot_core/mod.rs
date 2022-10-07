use async_trait::async_trait;
use github::GithubWorkflow;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::Stderr;

pub mod file_io;
pub mod github;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct OrchestrationYml {
    name: String,
    description: String,
    steps: Vec<OrchestrationStep>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct OrchestrationStep {
    name: String,
    description: Option<String>,
    owner: String,
    repo: String,
    workflowName: String,
    workflowArgs: Option<Vec<String>>,
    dependsOn: Option<String>,
}

#[async_trait]
pub trait Orchestration {
    /// runs the orchestration but checks if all repos and workflows
    /// are safe to run therefore making it slower
    async fn run_orchestration_safe(&self) -> Result<(), Stderr>;

    //FIXME: this link to the safe function doesn't actually work..
    /// runs the orchestration but without the safety checks of
    /// [run_orchestration_safe]
    ///
    ///
    fn run_orchestration_dangerously(&self) -> Result<(), Stderr>;

    /// Generates the orchestrations for the workflows
    /// This function should have 2 parts to this,
    /// - grouping all of the steps that don't have any value for the
    /// `depends_on` key in the yml
    /// - building dependency trees for the remaining steps
    fn generate_orchestrations();
}

#[async_trait]
impl Orchestration for OrchestrationYml {
    async fn run_orchestration_safe(&self) -> Result<(), Stderr> {
        println!("Running {}", self.name);

        let client = Client::new();

        //FIXME: I shouldn't need to clone this :(
        for step in &self.steps {
            let resp = GithubWorkflow::new(
                step.workflowName.clone(),
                step.repo.clone(),
                step.owner.clone(),
                false,
            )
            .run_workflow(&client)
            .await
            .unwrap()
            .poll_workflow(&client)
            .await;
        }

        Ok(())
    }

    fn run_orchestration_dangerously(&self) -> Result<(), Stderr> {
        unimplemented!()
    }

    fn generate_orchestrations() {
        todo!()
    }
}
