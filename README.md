# PR comment

**comment** is a CLI app to create a *pull request* comment with the following structure:

```md
**PR**
`feature/id`

**LINKS**
- [repo_name_1](pull_request_url_1)
- [repo_name_2](pull_request_url_2)

**REVIEW**
@mario

**CHANGES**
_TODO:_ what you've changed

**TESTING**
_TODO:_ how to test changes you've made
```

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
