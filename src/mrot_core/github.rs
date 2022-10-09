pub struct GithubWorkflow {
    name: String,
    id: String,
    repo: String,
    owner: String,
}

use core::panic;

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
    Pending(String),
}

impl GithubWorkflow {
    ///Create a new github workflow struct
    pub fn new(name: String, repo: String, owner: String, id: String) -> Self {
        GithubWorkflow {
            name,
            owner,
            repo,
            id,
        }
    }

    /// Runs a workflow by running
    async fn trigger_workflow(&self, http_client: &reqwest::Client) -> Result<(), reqwest::Error> {
        println!("Running workflow: {} on repo {} ", self.name, self.repo);

        //Need to create a new request body to include any args we want to pass the workflow
        let body = Body {
            r#ref: "master".to_string(),
        };

        let url = format!(
            "
            https://api.github.com/repos/{}/{}/actions/workflows/{}/dispatches",
            self.owner, self.repo, self.id
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
                    StatusCode::NO_CONTENT => Ok(()),
                    _ => panic!(),
                }
            }
            Err(e) => Err(e),
        }
    }

    async fn poll_workflow(&self, http_client: &Client) -> PollResponse {
        println!("Polling workflow {:?}...", self.name);

        let t = format!(
            "https://api.github.com/repos/{}/{}/actions/workflows/{}/runs",
            self.owner, self.repo, self.id
        );

        println!("{:?}", t);

        let resp = http_client
            .get(format!(
                "https://api.github.com/repos/{}/{}/actions/workflows/{}/runs",
                self.owner, self.repo, self.id
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

                match js.workflow_runs[0].status {
                    GithubWorkflowResponseStatus::success => PollResponse::Success,
                    GithubWorkflowResponseStatus::completed => PollResponse::Success,
                    GithubWorkflowResponseStatus::failure => {
                        PollResponse::Failure(String::from("Workflow failed"))
                    }
                    GithubWorkflowResponseStatus::timed_out => {
                        PollResponse::Failure(String::from("Workflow timed out"))
                    }
                    GithubWorkflowResponseStatus::cancelled => {
                        PollResponse::Failure(String::from("Workflow cancelled"))
                    }
                    _ => {
                        PollResponse::Pending(String::from(
                            "Workflow still in progress waiting 10 seconds to try again",
                        ))
                    }
                }
            }

            Err(_) => {
                PollResponse::Failure(String::from(
                    "Error trying to contact github api endpoint",
                ))
            }
        }
    }

    pub async fn run_workflow(&self, http_client: &Client) -> Result<&Self, ()> {
        let trigger_workflow_resp = self.trigger_workflow(http_client).await;

        match trigger_workflow_resp {
            Ok(_) => loop {
                let poll_resp = self.poll_workflow(http_client).await;
                if let PollResponse::Pending(_e) = poll_resp {
                    println!("Workflow {:?} has successfully started!", self.name);
                    return Ok(self);
                }
                println!(
                    "Workflow runner for {:?} is still being created ",
                    self.name
                );
                sleep(Duration::from_secs(2)).await;
            },
            Err(_) => todo!(),
        }
    }

    pub async fn poll_workflow_until_complete(&self, http_client: &Client) -> PollResponse {
        let mut tries = 6;

        while tries >= 0 {
            let poll_resp = self.poll_workflow(http_client).await;

            if let PollResponse::Pending(_m) = poll_resp {
                println!("Workflow {:?} is still pending...", self.name);
                //TODO: move the sleep amount to the config file
                sleep(Duration::from_secs(10)).await;
                tries -= 1;
            } else {
                return poll_resp;
            }
        }

        PollResponse::Failure(String::from("Exceeded amount of tries"))
    }
}
