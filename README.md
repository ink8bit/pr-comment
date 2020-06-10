# PR comment

A CLI app to compose a PR comment.

It creates comment with the following structure:

```md
**PR**

[Repository name](repo link)
**Branch:** `feature/id` or **Branch:** `hotfix/id`

**REVIEW**
@Shazam
@Mario

**CHANGES**
_TODO:_ what you've changed

**TESTING**
_TODO:_ how to test changes you've made
```

## How to install

via Makefile

## Development

cargo run

### Run with params

cargo run -- --id 100 --assignee Mario --repo awesome

## Build release version

cargo build --release

## Contributing
