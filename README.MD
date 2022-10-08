# Multi Repo Orchestration Tool

This tool is designed to be used to simplify the management and deployment of projects which use multiple repos  

## How?
This program reads in your orchestration.yml file, it then creates a graph and runs a topographic sort to ensure the order of steps is correct based on the `dependsOn` value for each step.

Then it makes a request to the `workflow_dispatch` endpoint on the github API. If successfully created it will keep polling that workflow until it succeeds, fails, or times out.

## Setup
Currently there is no simple setup guide or any CLI (though this is planned), however if you wish to run the code locally you can do the following

### Pre-reqs
- Install [Rust](https://www.rust-lang.org/) and swear allegiance to ferris <img src="https://rustacean.net/assets/rustacean-flat-happy.png" width="50px">
- Generate a [github PAT token for the program to use](https://)
  
### Building and running this project
1. Create an `orchestration.yml` file using [this format](./orchestration.md)
2. Create a `.env` file with a `GITHUB_PAT_TOKEN=<PAT_FROM_PREVIOUS_STEP>` 
3. Run `cargo build`
4. Run `cargo run`
5. Sit back, relax and enjoy!


