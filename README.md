# authorizer-cli

CLI tool for [authorizer.dev](https://authorizer.dev), use this to perform various operations on your authorizer instance.

For detailed information about all the supporetd operations check [docs](https://docs.authorizer.dev/)

## Getting Started

Here is a quick guide on getting started with the `authorizer-cli` tool.

### Step 1 - Create Instance

Get Authorizer URL by instantiating [Authorizer instance](/deployment) and configuring it with necessary [environment variables](/core/env).

### Step 2 - Install Binary

```sh
cargo install authorizer-cli
```

### Step 3 - Use the cli tool from your terminal.

- Invite members by uploading a csv or txt file

```sh
authorizer-cli invite-members --file-path ./authorizer_sample.csv
```

- Download sample files for usage

```sh
authorizer-cli-test download-samples --file-path .
```

- Print help message

```sh
authorizer-cli --help
```

Output:

```sh
CLI tool for authorizer

Usage: authorizer-cli-test [OPTIONS] [COMMAND]

Commands:
  invite-members    Invite users to app [--file-path <PATH>]
  download-samples  Download samples [--file-path <PATH>]
  help              Print this message or the help of the given subcommand(s)

Options:
  -d, --debug...  Turn debugging information on
  -h, --help      Print help information
  -V, --version   Print version information
```

Note: pass `-d` flag after command to turn ON Debug mode.

## Development

- Clone this repo
- For development use

```sh
cargo build
```

- For production use

```sh
cargo build --release
```

## Support our work

Github Sponsorship: https://github.com/sponsors/authorizerdev
