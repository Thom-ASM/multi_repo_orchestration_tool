use async_trait::async_trait;
use github::GithubWorkflow;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Stderr};

use self::github::PollResponse;
use petgraph::{
    algo::{toposort, Cycle},
    graph::{DiGraph, NodeIndex},
};

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
    dependsOn: Option<Vec<String>>,
}

#[async_trait]
pub trait Orchestration {
    /// runs the orchestration but checks if all repos and workflows
    /// are safe to run therefore making it slower
    async fn run_orchestration_safe(&self, sorted_indices: &Vec<NodeIndex>) -> Result<(), Stderr>;

    //FIXME: this link to the safe function doesn't actually work..
    /// runs the orchestration but without the safety checks of
    /// [run_orchestration_safe]
    ///
    ///
    fn run_orchestration_dangerously(&self) -> Result<(), Stderr>;

    /// Generates the orchestrations for the workflows
    /// This function should create a digraph and then topographically
    /// sort the workflows
    fn generate_orchestrations(&self) -> Result<Vec<NodeIndex>, Cycle<NodeIndex>>;
}

#[async_trait]
impl Orchestration for OrchestrationYml {
    async fn run_orchestration_safe(&self, sorted_indices: &Vec<NodeIndex>) -> Result<(), Stderr> {
        println!("Running {}", self.name);

        let client = Client::new();

        for idx in sorted_indices {
            let current_step = &self.steps[idx.index()];

            //FIXME: I shouldn't need to clone this
            let resp = GithubWorkflow::new(
                current_step.workflowName.clone(),
                current_step.repo.clone(),
                current_step.owner.clone(),
            )
            .run_workflow(&client)
            .await
            .unwrap()
            .poll_workflow(&client)
            .await;

            match resp {
                PollResponse::Success => println!("Successfully ran workflow"),
                PollResponse::Failure(msg) => eprintln!("{}", msg),
            }
        }

        Ok(())
    }

    fn run_orchestration_dangerously(&self) -> Result<(), Stderr> {
        unimplemented!()
    }

    fn generate_orchestrations(&self) -> Result<Vec<NodeIndex>, Cycle<NodeIndex>> {
        //Create a graph
        let mut step_deps_graph = DiGraph::new();

        // Hash map to make the graph vertex generation more efficient
        // as I wouldn't have to run .position() each iteration
        // as worst case for finding the position would be O(n+n-1+n-2....n-n)
        // but now its O(n) to create the hashmap and O(1) to query the key
        // then also making it O(1) to query the workflow step array
        let mut hm: HashMap<String, usize> = HashMap::with_capacity(self.steps.len());

        //adds the value for the hashmap and adds the matching node to the graph
        for (idx, val) in self.steps.iter().enumerate() {
            hm.entry(val.name.clone()).or_insert(idx);
            step_deps_graph.add_node(idx);
        }

        //iterate over each step and create an edge between 2 nodes if it exists
        for (idx, val) in self.steps.iter().enumerate() {
            let current_step_deps = &val.dependsOn;
            if let Some(deps) = current_step_deps {
                for dep in deps {
                    let x = NodeIndex::new(*hm.get_key_value(dep).unwrap().1);
                    let y = NodeIndex::new(idx);

                    step_deps_graph.add_edge(x, y, 1);
                }
            }
        }

        toposort(&step_deps_graph, None)
    }
}
