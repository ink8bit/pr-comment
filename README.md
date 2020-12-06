# PR comment

![Rust](https://github.com/ink8bit/pr-comment/workflows/Rust/badge.svg)

**comment** is a CLI app to create a *pull request* comment with the following structure:

```md
**PULL REQUEST**
feature/id

**LINKS**
- repo_name_1 pull_request_url_1
- repo_name_2 pull_request_url_2

**REVIEW**
@reviewer_nickname

**CHANGES**
_TODO:_ what you've changed

**TESTING**
_TODO:_ how to test changes you've made
```

## Installation

1. Go to [releases page](https://github.com/ink8bit/pr-comment/releases)
2. Grab the latest binary
3. Put binary into `/usr/local/bin/`
4. Type `comment -h` in Terminal and you should see a help message

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
comment -i 5 -l b/1
```

3. You should see the following output:

```
**PR**
`feature/5`

**LINKS**
- [pr-comment](https://github.com/ink8bit/pr-comment/pull/1)

**REVIEW**
@reviewer_nickname_without_at_sign

**CHANGES**
_TODO:_ what you've changed

**TESTING**
_TODO:_ how to test changes you've made
```

## Help message

```
USAGE:
    comment [FLAGS] [OPTIONS] --id <int> --link <string>

FLAGS:
    -b, --bug        Sets a bug value to true
    -c, --copy       Copies comment to clipboard
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --id <int>             Sets Task ID value
    -l, --link <string>        Sets PR links values, use comma for multiple values
    -r, --reviewer <string>    Sets a reviewer or reviewers, use comma for multiple values
```


## Example

- you have a task ID `150` in your task tracker
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
comment -i 150 -l p/35 -r awesome_reviewer
```

Then you will get in your terminal:

```
**PULL REQUEST**
feature/150

**LINKS**
- pr-comment https://github.com/ink8bit/pr-comment/pull/35

**REVIEW**
@awesome_reviewer

**CHANGES**
_TODO:_ what you've changed

**TESTING**
_TODO:_ how to test changes you've made
```

## Development

1. clone this repo
2. run in your Terminal:

```sh
cargo build
```

## Contributing

Please refer to [CONTRIBUTING.md](/.github/CONTRIBUTING.md).
