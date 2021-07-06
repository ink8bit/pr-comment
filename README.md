# PR comment

![Rust](https://github.com/ink8bit/pr-comment/workflows/Rust/badge.svg)

**comment** is a CLI utility to create a *pull request* comment with the following structure:

```md
**PULL REQUEST**
<current_branch_name>

**LINKS**
- repo_name_1 pull_request_url_1
- repo_name_2 pull_request_url_2

**REVIEW**
@reviewer_nickname

**CHANGES**
TODO: what you've changed

**TESTING**
TODO: how to test changes you've made
```

## Usage

1. Create JSON config file `.commentrc` in your `$HOME` folder.

This file should have the following structure:

```json
{
  "defaultReviewer": "reviewer_nickname_without_at_sign",
  "links": {
    "p": {
      "repoName": "pr-comment",
      "url": "https://github.com/ink8bit/pr-comment/pull"
    }
  }
}
```

`links` section can have as many keys as you wish. Two keys for example:

```json
{
  "defaultReviewer": "reviewer_nickname_without_at_sign",
  "links": {
    "p": {
      "repoName": "pr-comment",
      "url": "https://github.com/ink8bit/pr-comment/pull"
    },
    "a": {
      "repoName": "awesome-pr-comment",
      "url": "https://github.com/ink8bit/awesome-pr-comment/pull"
    }
  }
}
```


2. Run this command in your Terminal:

```sh
comment -l p/1
```

3. You should see the following output:

```md
**PR**
<current_branch_name>

**LINKS**
- [pr-comment](https://github.com/ink8bit/pr-comment/pull/1)

**REVIEW**
@reviewer_nickname_without_at_sign

**CHANGES**
TODO: what you've changed

**TESTING**
TODO: how to test changes you've made
```

## Help message

> Copy to clipboard works on macOS for now

```
USAGE:
    comment [FLAGS] [OPTIONS] --link <string>

FLAGS:
    -c, --copy       Copies comment to clipboard
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --link <string>        Sets PR links values, use comma for multiple values
    -r, --reviewer <string>    Sets a reviewer or reviewers, use comma for multiple values
```


## Example

- you created pull request with ID `35`
- your reviewer has a nickname `@awesome_reviewer`
- and you have the following config:

```json
{
  "defaultReviewer": "reviewer_nickname_without_at_sign",
  "links": {
    "p": {
      "repoName": "pr-comment",
      "url": "https://github.com/ink8bit/pr-comment/pull"
    }
  }
}
```

```sh
comment -l p/35 -r awesome_reviewer
```

Then you will get in your terminal:

```md
**PULL REQUEST**
<current_branch_name>

**LINKS**
- pr-comment https://github.com/ink8bit/pr-comment/pull/35

**REVIEW**
@awesome_reviewer

**CHANGES**
TODO: what you've changed

**TESTING**
TODO: how to test changes you've made
```

## Development

1. clone this repo
2. run in your Terminal:

```sh
cargo build
```

## Contributing

Please refer to [CONTRIBUTING.md](/.github/CONTRIBUTING.md).
