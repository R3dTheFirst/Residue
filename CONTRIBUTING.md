# Contributing to Residue

## Before you start
Read [SCOPE.md](SCOPE.md) — it defines what v1 is and isn't. PRs outside
that scope will likely be deferred, not because they're bad ideas, but
because scope discipline is how this project ships at all.

## Development setup
- Rust toolchain: [version]
- Recommended: build and test inside a VM (Hyper-V or similar), since
  Residue watches real install-time system changes. Don't run test
  installs on your host.
- [Any windows-rs / SDK setup specifics]

## Build & test

`cargo build`

`cargo test`

## Running a test monitor
cargo run -- monitor <path-to-some-installer.exe>
Inspect the resulting JSON output in [location].

## Submitting changes
1. Open an issue first for anything beyond a small fix — avoids wasted
   work on something out of scope or already in progress.
2. Fork, branch, PR against `main`.
3. Keep PRs scoped to one subsystem/feature where possible.
4. Update CHANGELOG.md under "Unreleased."

## Good first issues
Labeled `good first issue` in the tracker — self-contained, don't
require deep familiarity with the whole engine.
