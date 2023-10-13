# authorizer-cli

*CLI tool for [Authorizer](https://authorizer.dev): Simplify Management of Your Authorizer Instance*

Welcome to the "authorizer-cli" project, a command-line tool designed to streamline the management of your Authorizer instance. Authorizer is a powerful platform for user management and access control in your applications.

For detailed information about all supported operations, please refer to our [documentation](https://docs.authorizer.dev/).

## Table of Contents

- [Getting Started](#getting-started)
  - [Step 1 - Create an Authorizer Instance](#step-1---create-an-authorizer-instance)
  - [Step 2 - Install the Binary](#step-2---install-the-binary)
  - [Step 3 - Use the CLI Tool](#step-3---use-the-cli-tool)
- [Commands and Usage](#commands-and-usage)
  - [Invite Members](#invite-members)
  - [Download Sample Files](#download-sample-files)
  - [Debug Mode](#debug-mode)
  - [Help Message](#help-message)
- [Development](#development)
- [Support Our Work](#support-our-work)

## Getting Started

Follow these steps to quickly get started with the "authorizer-cli" tool.

### Step 1 - Create an Authorizer Instance

1. Visit [Authorizer Deployment](https://github.com/authorizerdev/authorizer-cli/blob/main/deployment) to create your Authorizer instance.
2. Configure your instance with the necessary environment variables as specified in our [core/env documentation](https://github.com/authorizerdev/authorizer-cli/blob/main/core/env).

### Step 2 - Install the Binary

Use Rust's package manager, `cargo`, to install the *"authorizer-cli"* binary on your system. Ensure you have Rust and `cargo` installed.

```sh
cargo install authorizer-cli
```

This will download and install the *"authorizer-cli"* tool.

### Step 3 - Use the CLI Tool

You can now use the "authorizer-cli" from your terminal. Below are some example commands to get you started:

## Commands and Usage 

The "authorizer-cli" provides the following commands and detailed usage instructions:

### Invite Members

Invite users to your app by uploading a CSV or TXT file containing member information. This is particularly useful for onboarding new users efficiently.

#### Command :

```sh
authorizer-cli invite-members --file-path ./authorizer_sample.csv
```

#### Usage:

- `--file-path` (Required): Provide the file path to the CSV or TXT file containing the member information you want to upload.
- `-d` (Optional): You can add the -d flag to enable debug mode, which provides detailed debugging information.

### Download Sample Files 

Download sample files to your specified directory for various use cases. These files can be handy for testing and experimentation.

#### Command:

```sh
authorizer-cli download-samples --file-path .
```

#### Usage:

- `--file-path`(Required): Specify the directory where you want to download the sample files.

### Debug Mode

Enable debug mode to get more detailed information about what the tool is doing behind the scenes. This can be useful for troubleshooting and gaining insights into the tool's behavior.

#### Example with Debug Mode:

```sh
authorizer-cli invite-members --file-path ./authorizer_sample.csv -d
```

#### Usage:

Add `-d` flag after a commmand to enable debug mode.

### Help Message

For a full list of available commands and their respective options, you can use the `--help` command.

#### Command:

```sh
authorizer-cli --help
```

#### Usage:

This command provides a comprehensive overview of all available commands and their associated options.

## Development 

If you'd like to contribute to the development of the "authorizer-cli" tool, follow these steps:

1. Clone this repository.

```sh 
git clone <repository_url>
```

2. For development use, build the tool with the following command:

```sh
cargo build
```

3. For production use, build the tool in release mode:

```sh
cargo build --release
```

## Support Our Work

If you appreciate our work and would like to support the "authorizer-cli" project, please consider sponsoring us on GitHub:

[GitHub Sponsorship]( https://github.com/sponsors/authorizerdev)

Your sponsorship helps fund and maintain the development of this tool.

Please ensure that you have configured the required environment variables and files, such as the CSV file, as specified in the documentation to use the tool effectively.

Thank you for choosing "authorizer-cli." We hope it simplifies your Authorizer management tasks. If you have any questions or encounter issues, please don't hesitate to contact us.