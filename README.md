# GitHub LOC counter: `ghloc`

[Blog post](https://matthewtrent.me/articles/ghloc) covering its creation 📚

### Example output

This scans all your repositories and aggregates your commit data (`ghloc stats`):

<img src="https://github.com/mattrltrent/ghloc/blob/main/assets/demo_1.png?raw=true" width="" height="" style="display: inline"/>


### Install via Homebrew

To install, run:

```sh
brew tap mattrltrent/tap ; brew install ghloc
```

To verify installation, run:

```sh
ghloc --version
```

### Quick start

**Get your GitHub token**

1. Go to [GitHub's token manager](https://github.com/settings/tokens?type=beta) (you may need to login) and click the **Generate new token** button in the top-right.

2. Give your new token a name, an optional description, and set its expiration. The specifics of what you enter don't matter. However, if your token expires, you'll have to re-enter a new one in the tool.

3. Under **Repository access**, select **All repositories**.

4. Under the **Permissions** > **Repository permissions** dropdown, grant your token **Access: Read-only** for:

    - **Contents**
    - **Commit statuses**

**Use the CLI tool**

Run this to set your credentials with your GitHub username and newfound token:

```sh
ghloc set --username <YOUR_NAME> --token <YOUR_TOKEN>
```

You can check they were added and saved correctly by running:

```sh
ghloc creds
```

And if needed, clear them by running:

```sh
ghloc clear
```

Then run this to check your stats (it will take a while):

```sh
ghloc stats
```

If you're ever lost, try:

```sh
ghloc help
```

or:

```sh
ghloc example
```
