# Development

Run the following command in the root of the project to start the Dioxus dev server:

```bash
dx serve --hot-reload
```

- Open the browser to http://localhost:8080

## Prerequisites

```sh
cargo install dioxus-cli
sudo apt install libssl-dev pkg-config
```

## How to config project's git configure

- Show current git config used by project

  ```sh
  git config --list
  ```

- Config by editing `.git/config`

  ```toml
  [user]
    name = zw
    email = hyperion_z@outlook.com
  ```

## Run

```sh
dx serve --port 3002
```

## References

- [dioxus-openai-qa-gui](https://github.com/fairjm/dioxus-openai-qa-gui)  