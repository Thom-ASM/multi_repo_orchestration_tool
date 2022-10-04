pub struct GithubWorkflow {
    name: String,
    repo: String,
    polling: bool,
}

impl GithubWorkflow {
    ///Create a new github workflow struct
    pub fn new(name: String, repo: String, polling: bool) -> Self {
        GithubWorkflow {
            name,
            repo,
            polling,
        }
    }

    /// Runs a workflow by running
    pub async fn run_workflow(&mut self) -> Result<&mut Self, reqwest::Error> {
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
