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

## How to use

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


1. Run this command in your Terminal:

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

## Commands

### Help

```sh
comment -h
```

### Version

```sh
comment -V
```

### Create a comment to your pull request:

For example,
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

### Copy to clipboard

If you want to copy your comment to clipboard add `-c` or `--copy` flag to your command:

```sh
comment -i 150 -l p/35 -c
```

## How to install

1. Go to [releases page](https://github.com/ink8bit/pr-comment/releases)
2. Grab the latest binary
3. Put binary into `/usr/local/bin/`
4. Type `comment -h` in Terminal and you should see a help message

## Development

### Run with params

```sh
cargo run -- --id 100 --link p/1 --reviewer reviewer_nickname

# short options names
cargo run -- -i 100 -l p/1 -r reviewer_nickname
```

## Build

### Build debug version

```sh
cargo build
```

### Build release version

```sh
cargo build --release
```

## Contributing

Please refer to [CONTRIBUTING.md](/.github/CONTRIBUTING.md).
