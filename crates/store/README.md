# rxchef-store

Global and project-scoped storage for [rx-chef](https://rx-chef.readthedocs.io/):
recipes, variables, pipelines, and bounded execution history.

This crate is an implementation detail consumed by `rxchef-cli` and
`rxchef-tui`. It is published separately because both binaries depend on it
independently; most users should install one of those instead of depending
on this crate directly.

See the [full documentation](https://rx-chef.readthedocs.io/) for the store
layout, precedence rules, and file formats.

License: Apache-2.0
