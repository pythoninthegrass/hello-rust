# hello-rust

[![CI](https://github.com/pythoninthegrass/hello-rust/workflows/CI/badge.svg)](https://github.com/pythoninthegrass/hello-rust/actions)

## Setup

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* Install `hello-rust` via
    ```bash
    # remote
    cargo install --git https://github.com/pythoninthegrass/hello-rust

    # local
    cargo install --path .
    ```

## Development

### Setup

* Follow the [setup instructions](#setup) above to set up your environment
* Install [devbox](https://www.jetpack.io/devbox/docs/quickstart/)
    ```bash
    curl -fsSL https://get.jetpack.io/devbox | bash
    ```

### Usage

* Basic commands
    ```bash
    # install dependencies
    devbox install

    # enter devbox
    devbox shell

    # refresh devbox after making changes to devbox.json
    refresh

    # deactivate devbox
    exit
    ```

## Further reading

[Introduction | Rust GitHub Template](https://rust-github.github.io/)

[Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)

[Rust | Devbox](https://www.jetpack.io/devbox/docs/devbox_examples/languages/rust/)

[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
