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
    workflow_runs: Vec<WorkflowRun>,
}

#[derive(Deserialize, Debug)]
struct WorkflowRun {
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
    Waiting,
}

pub enum PendingResponse {
    RateLimit,
    NonComplete,
}

pub enum PollResponse {
    Success,
    Failure(String),
    Pending(PendingResponse),
}

impl GithubWorkflow {
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

        let resp = http_client
            .post(url)
            .bearer_auth(std::env::var("GITHUB_PAT_TOKEN").unwrap())
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "MROT runner")
            .json(&body)
            .send()
            .await;

        match resp {
            Ok(resp) => {
                let status = resp.status();
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

        let resp = http_client
            .get(format!(
                "https://api.github.com/repos/{}/{}/actions/workflows/{}/runs",
                self.owner, self.repo, self.id
            ))
            .header("User-Agent", "testrunner")
            .send()
            .await;

        match resp {
            Ok(resp) => {
                if resp
                    .headers()
                    .get("x-ratelimit-remaining")
                    .expect("Failed to find the remaining rate limit header")
                    .as_bytes()
                    == b"0"
                {
                    return PollResponse::Pending(PendingResponse::RateLimit);
                }

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
                    _ => PollResponse::Pending(PendingResponse::NonComplete),
                }
            }

            Err(_) => {
                PollResponse::Failure(String::from("Error trying to contact github api endpoint"))
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
                sleep(Duration::from_secs(20)).await;
            },
            Err(_) => todo!(),
        }
    }

    pub async fn poll_workflow_until_complete(&self, http_client: &Client) -> PollResponse {
        let max_tries = 11;
        let mut counter = 0;
        let mut delay: u64 = 0x1;

        while counter < max_tries {
            let poll_resp = self.poll_workflow(http_client).await;

            if let PollResponse::Pending(pending_resp) = poll_resp {
                match pending_resp {
                    PendingResponse::RateLimit => {
                        //could check if the next tick will occour after the rate limit resets but for now,
                        // we'll just panic
                        panic!("Hit the github rate limit")
                    }
                    PendingResponse::NonComplete => {
                        println!("Workflow {:?} is still pending...", self.name);
                    }
                }

                delay = delay << 2;
                println!("..will retry in {:?} seconds", delay);
                sleep(Duration::from_secs(delay)).await;
                counter += 1;
            } else {
                return poll_resp;
            }
        }

        PollResponse::Failure(String::from("Exceeded amount of tries"))
    }
}
