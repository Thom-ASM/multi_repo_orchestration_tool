pub struct GithubWorkflow {
    name: String,
    repo: String,
    owner: String,
    polling: bool,
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
        httpClient: &reqwest::Client,
    ) -> Result<&mut Self, reqwest::Error> {
        let url = format!(
            "
            https://api.github.com//repos/{}/{}/actions/workflows/{}/dispatches",
            self.owner, self.repo, self.name
        );
        let res = httpClient.post(url).send().await;
        //run workflow
        println!("Running workflow {} on repo {} ", self.name, self.repo);

        Ok(self)
    }

    pub fn poll_workflow(&mut self) {
        while self.polling {
            //check if the workflow is done?
            //if it is break
            //else thread sleep for 10 seconds
            self.polling = false;
        }
    }
}
