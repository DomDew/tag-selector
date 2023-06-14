# Introduction <!-- omit from toc -->

Tag-selector is a nifty little command line tool to lets you checkout a tag for a git repository from a neat dropdown.

- [Roadmap 🏗️](#roadmap-️)
- [Installation 🧑‍🔧](#installation-)
  - [Cargo](#cargo)
- [Usage](#usage)

## Roadmap 🏗️

- Allow filters for tags

## Installation 🧑‍🔧

### Cargo

**Prerequisites:**

You have cargo and rust installed 🦀.

**How to install:**

1. Clone this repo.
2. Cd into this repo.
3. `cargo install --path .` - to install it in your path.

## Usage

1. Checkout any git repo
2. Run `tag-selector` in your terminal
3. Select any of the tags that will be listed

Takes an optional `--pattern=<any-string>` command line argument by which to filter the tags. Defaults to `"*"`.
