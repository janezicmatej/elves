# Elves

An Advent Of Code utils crate

Please read the [API documentation here](https://docs.rs/elves/).

![ci](https://github.com/janezicmatej/elves/actions/workflows/ci.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/elves.svg)](https://crates.io/crates/elves)

How to use with Cargo:

```toml
[dependencies]
elves = "0.1.0"
```

## Project goals

- minimal dependency tree
- provide a place to store reusable code from past years to reuse in next runs and keep solution repositories clean
- provide a way to learn how to integrate with crates.io
- provide a way to experiment with rust dx tool

## Workflow

During month of december there will be a `develop` branch active where I will add code I deem reusable from solutions I write every day. After christmas I will make a new release (pretty much a direct merge from `develop` into `master`) so I suggest version locking this library if you somehow find it.
