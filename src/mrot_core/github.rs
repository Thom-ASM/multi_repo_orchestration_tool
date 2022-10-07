pub struct GithubWorkflow {
    name: String,
    repo: String,
    owner: String,
}

use core::panic;
use std::task::Poll;

use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use tokio::time::{sleep, Duration};

#[derive(Serialize)]
struct Body {
    r#ref: String,
}

#[derive(Deserialize, Debug)]
struct GithubWorkflowResponse {
    total_count: usize,
    workflow_runs: Vec<workflowRun>,
}

#[derive(Deserialize, Debug)]
struct workflowRun {
    status: GithubWorkflowResponseStatus,
}

#[derive(Deserialize, Debug)]
enum GithubWorkflowResponseStatus {
    completed,
    action_required,
    cancelled,
    failure,
    neutral,
    skipped,
    stale,
    success,
    timed_out,
    in_progress,
    queued,
    requested,
    waiting,
}

pub enum PollResponse {
    Success,
    Failure(String),
}
impl GithubWorkflow {
    ///Create a new github workflow struct
    pub fn new(name: String, repo: String, owner: String) -> Self {
        GithubWorkflow { name, owner, repo }
    }

    /// Runs a workflow by running
    pub async fn run_workflow(
        &mut self,
        http_client: &reqwest::Client,
    ) -> Result<&mut Self, reqwest::Error> {
        println!("Running workflow {} on repo {} ", self.name, self.repo);

        //Need to create a new request body to include any args we want to pass the workflow
        let body = Body {
            r#ref: "master".to_string(),
        };

        let url = format!(
            "
            https://api.github.com/repos/{}/{}/actions/workflows/{}/dispatches",
            self.owner, self.repo, self.name
        );

        //TODO: move user-agent in to config file
        let resp = http_client
            .post(url)
            .bearer_auth(std::env::var("GITHUB_PAT_TOKEN").unwrap())
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "Sir-Martin-Esq-III")
            .json(&body)
            .send()
            .await;

        match resp {
            Ok(resp) => {
                let status = resp.status();
                println!("{:?}", status);
                match status {
                    StatusCode::NO_CONTENT => Ok(self),
                    _ => panic!(),
                }
            }
            Err(e) => Err(e),
        }
    }

    //FIXME: Github takes some time to spin up a action runner, this means
    // we need to check if we are not comparing against a stale workflow
    pub async fn poll_workflow(&mut self, http_client: &Client) -> PollResponse {
        println!("POLLING");

        //TODO: move this in to the config file
        let mut tries_left = 5;

        while tries_left > 0 {
            let resp = http_client
                .get(format!(
                    "
                https://api.github.com/repos/{}/{}/actions/workflows/{}/runs",
                    self.owner, self.repo, self.name
                ))
                .header("User-Agent", "Sir-Martin-Esq-III")
                .send()
                .await;

            match resp {
                Ok(resp) => {
                    let js = resp
                        .json::<GithubWorkflowResponse>()
                        .await
                        .expect("failed to deserialize workflow response ");

                    println!("{:?}", js.workflow_runs[0].status);

                    //FIXME: this match doesn't seem to be working quite well
                    match js.workflow_runs[0].status {
                        GithubWorkflowResponseStatus::success => return PollResponse::Success,
                        GithubWorkflowResponseStatus::failure => {
                            return PollResponse::Failure(String::from("Workflow failed"));
                        }
                        GithubWorkflowResponseStatus::timed_out => {
                            return PollResponse::Failure(String::from("Workflow timed out"));
                        }
                        GithubWorkflowResponseStatus::cancelled => {
                            return PollResponse::Failure(String::from("Workflow cancelled"));
                        }
                        _ => {
                            println!("Workflow still in progress waiting 10 seconds to try again");
                            tries_left -= 1;
                            //TODO: Move this in to the config file
                            sleep(Duration::from_secs(10)).await;
                        }
                    }
                }

                Err(_) => {
                    return PollResponse::Failure(String::from(
                        "Error trying to contact github api endpoint",
                    ))
                }
            }
        }

        PollResponse::Failure(String::from("Exceeded tries"))
    }
}
