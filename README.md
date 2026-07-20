# Residue

A real-time Windows install monitor that tracks filesystem, registry,
and firewall changes as an application installs, attributing each
change back to the process that made it.

## Table of Contents
- [Motivation](#motivation)
- [What it does](#what-it-does)
- [Status](#status)
- [How it works](#how-it-works)
- [Example output](#example-output)
- [Getting started](#getting-started)
- [Usage](#usage)
- [Architecture](#architecture)
- [Contributing](#contributing)
- [Credits](#credits)
- [License](#license)

## Motivation

Most uninstallers only remove what an installer *registered* with
Windows — they don't know about anything the application changed
after the fact, or anything the installer didn't declare. Residue was
built to answer a simpler question: what did this program actually do
to my system, observed directly, not inferred after the fact?

## What it does

- Monitors filesystem changes under user-specified watch root(s)
  during an installer run
- Monitors registry changes under `HKCU` and `HKLM\Software` during
  the same run
- Diffs firewall rules before and after the run
- Attributes each event to the process that triggered it, using
  direct process lineage from the original installer

See [SCOPE.md](SCOPE.md) for the precise boundary of what v1 does and
does not cover.

## Status

Early development. Core engine (directory + registry monitoring) is
in progress. Not yet ready for general use — track progress in
[SCOPE.md](SCOPE.md) and [CHANGELOG.md](CHANGELOG.md).

## How it works

Residue subscribes to Windows ETW providers for file, registry, and
process events, and takes a before/after snapshot of firewall rules.
Every event is tagged with the process ID that triggered it. Output is
a JSON diff you can inspect, script against, or feed into the
(planned) GUI.

## Example output

*(add a real sample JSON diff here once v1 produces one)*

## Getting started

*(installation instructions once there's a build to install)*

## Usage
