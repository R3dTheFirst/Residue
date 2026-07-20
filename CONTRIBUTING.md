# Contributing to Residue

First off, thank you for considering contributing — Residue is early
and every bit of help, from code to bug reports to documentation
fixes, moves it forward.

## Table of Contents
- [Before you start](#before-you-start)
- [Development environment](#development-environment)
- [Build & test](#build--test)
- [Running a test monitor](#running-a-test-monitor)
- [Submitting changes](#submitting-changes)
- [What to expect](#what-to-expect)
- [Other ways to contribute](#other-ways-to-contribute)
- [Recognition](#recognition)

## Before you start

Read [SCOPE.md](SCOPE.md) — it defines what v1 is and isn't. PRs
outside that scope will likely be deferred, not because they're bad
ideas, but because scope discipline is how this project ships at all.
If you're unsure whether something fits, open an issue first.

## Development environment

- Rust toolchain: [version]
- Recommended: build and test inside a VM (Hyper-V or similar), since
  Residue watches real install-time system changes. Don't run test
  installs on your host machine.
- [windows-rs / Windows SDK setup specifics]

## Build & test
`cargo build`

`cargo test`

## Running a test monitor
cargo run -- monitor <path-to-some-installer.exe>

Inspect the resulting JSON output at [location].

## Submitting changes

1. For anything beyond a small fix, open an issue first — this avoids
   wasted work on something out of scope or already in progress.
2. Fork the repo, branch off `main`.
3. Keep PRs focused — one subsystem or fix per PR.
4. Add/update tests for your change where applicable.
5. Update `CHANGELOG.md` under "Unreleased."
6. Open the PR against `main` with a clear description of what and why.

## What to expect

This is a young project maintained part-time, so response times won't
be instant — but every issue and PR will get a response. If a PR sits
untouched for more than two weeks, feel free to ping it.

## Other ways to contribute

Not every contribution is code — reporting bugs with clear repro
steps, improving documentation, or testing against installers Residue
hasn't seen yet are all genuinely useful.

## Recognition

Contributors are credited in the README. Thank you for helping build
this.
