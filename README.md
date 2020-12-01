# PR comment

**comment** is a CLI app to create a *pull request* comment with the following structure:

```md
**PULL REQUEST**
feature/id

**LINKS**
- repo_name_1 pull_request_url_1
- repo_name_2 pull_request_url_2

**REVIEW**
@mario

**CHANGES**
_TODO:_ what you've changed

**TESTING**
_TODO:_ how to test changes you've made
```

## How to use

1. Create a config file `.commentrc` in your `$HOME` folder.

This file should be JSON with the following structure:

```json
{
  "defaultReviewer": "mario",
  "links": {
    "p": {
      "repoName": "pr-comment",
      "url": "https://github.com/ink8bit/pr-comment/pull"
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
@mario


**CHANGES**
_TODO:_ what you've changed

**TESTING**
_TODO:_ how to test changes you've made
```

## Commands

*TODO*

## How to install

- via Makefile
- via binary

## Development

### Run with params

```sh
cargo run -- --id 100 --link a --reviewer mario

# short options names
cargo run -- -i 100 -l a -r mario
```

## Build release version

```
cargo build --release
```

## Contributing

*TODO*
