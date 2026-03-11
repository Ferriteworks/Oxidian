# Conventions

This document outlines the conventions and style guidelines for this project. Following these conventions helps keep the codebase consistent, maintainable, and easier to read for everyone involved. These are not hard rules, but it's best to follow them unless you have a good reason not to

## Code style

General speaking, we use standard coding conventions for Rust, as enforced by `rustfmt`. The `rustfmt.toml` file in the root of the repository contains our specific formatting settings. The settings are an 88 character line width, 4-space line indent and the 2021 edition.

To format your code, run `cargo fmt` in the root of the repository. You can also set up your editor to run `cargo fmt` on save.

## Module structure

Module structure is organized into modules based on functionality. For example, all code related to command handling goes in the `commands` module, all code related to event handling goes in the `events` module, and so on. This helps keep related code together and makes it easier to find what you're looking for.
