pub struct GithubWorkflow {
    name: String,
    repo: String,
    owner: String,
    polling: bool,
}

use reqwest::{Client, StatusCode};
use serde::Serialize;

use tokio::time::{sleep, Duration};

#[derive(Serialize)]
struct Body {
    r#ref: String,
}

impl GithubWorkflow {
    ///Create a new github workflow struct
    pub fn new(name: String, repo: String, owner: String, polling: bool) -> Self {
        GithubWorkflow {
            name,
            owner,
            repo,
            polling,
        }
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

    pub async fn poll_workflow(&mut self, _http_client: &Client) {
        self.polling = true;
        println!("POLLING");
        while self.polling {
            sleep(Duration::from_secs(10)).await;
            self.polling = false;
        }
    }
}
