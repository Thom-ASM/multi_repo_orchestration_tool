# Building an orchestration file
Currently this program only supports `.yml files` and even worse, it has to specifically be called `orchestration.yml` and placed in the root of the repo's directory.

## Setup

### Main
Here are the main keys you will need to add to your orchestration file

Each step looks something like this:
```yaml
name: orchestration
description: orchestration demo
steps:
- ... 
- ... 
- ... 
```
with each of the keys representing the following:

| Key      | Description | required|
| ----------- | ----------- | ----------- |
| name      | The name you want for the **whole** orchestration      |`true`|
| description      | The description of the orchestration      |`true`|
| steps      | Array of all the workflows you want to run      |`true`|



### Steps
The core of the orchestration file is the `steps` array which contains a list of all of the workflows you want to run

Each step looks something like this:
```yaml
  - step:
    name: deployRepo2
    depends_on:
      - deployRepo1
    description: Will deploy QA
    repo: multi_repo_orchestration_tool
    owner: Sir-Martin-Esq-III
    workflow_number: 36915094 
```
with each of the keys representing the following:

| Key      | Description | required|
| ----------- | ----------- | ----------- |
| name      | The name you want for the step       |`true`|
| depends_on   | A list of steps you want to run before this, i.e a list of other steps `names`        |`false`|
| description   | Description of the step        |`false`|
| repo   | Github Repo the workflow resides in        |`true`|
| owner   | Owner of the aforementioned Github repo        |`true`|
| workflow_number   | Number assigned to that repo     |`true`|



