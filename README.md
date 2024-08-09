# How to Build a Powerful GraphQL API with Rust

## Show installed rust versions

```sh
$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/zw/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu
nightly-2024-01-18-x86_64-unknown-linux-gnu
nightly-2024-01-19-x86_64-unknown-linux-gnu
nightly-2024-01-29-x86_64-unknown-linux-gnu
nightly-x86_64-unknown-linux-gnu (default)
1.72.0-x86_64-unknown-linux-gnu

active toolchain
----------------

1.72.0-x86_64-unknown-linux-gnu (overridden by '/home/zw/code/rust_programming/axum-graphql/rust-toolchain.toml')
rustc 1.72.0 (5680fa18f 2023-08-23)
```

## Update stable rust

```sh 
rustup update stable
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.78.0 (9b00956e5 2024-04-29)

info: checking for self-update
```

## Troubleshooting 

- `rustup update` failing with could not rename component file
  - Solution: `rustup toolchain uninstall stable && rustup toolchain install stable`


## How to set local git config 

To set Git configuration for your local project, you need to specify the configuration settings that should apply only to your project, rather than globally across all your projects. Here’s how you can do it:

### Viewing Local Configuration

To view the current configuration settings for your local project, use the following command:
```sh
git config --local --list
```

This command will display all configuration settings that are specific to your current project.

### Editing the Local Configuration File Directly

You can also directly edit the `.git/config` file in your project directory to add or modify settings. This file is in INI format. Here’s an example of what it might look like:

```ini
# ...
[user]
  name = zwpdbh
  email = hyperion_z@outlook.com
```

### Summary

By setting your Git configuration locally, you ensure that these settings apply only to your specific project, allowing you to customize behavior on a per-project basis. Use the `git config --local` command to set these configurations, or edit the `.git/config` file directly for more advanced setups.