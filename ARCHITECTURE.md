# Architecture

## Overview

Residue is split into two independent pieces:

**Engine** — does the actual monitoring, diffing, and attribution. No UI
dependency. Fully usable from the CLI alone. This is the source of truth.

**GUI** (post-v1) — a thin, separate process that reads the engine's
output and displays it. Communicates via [local socket / named pipe /
polling JSON file — decide when you get there]. The GUI never touches
monitoring logic directly.

## Why this split

Keeping the engine UI-independent means it's scriptable, testable, and
usable in headless/CI contexts, and means GUI work never blocks or
complicates the core monitoring logic. It also means a service/daemon
mode later is a wrapper around the existing engine loop, not a rewrite.

## Engine internals (as built)

*(fill in as each subsystem lands — one section per subsystem, e.g.:)*

### Directory monitoring
- API/provider used:
- Known limitations:

### Registry monitoring
- API/provider used:
- Known limitations:

### Firewall diffing
- API used: `INetFwPolicy2` (COM)
- Known limitations:

### Attribution
- Method: PID lineage, direct-child-of-installer only
- Known limitations: no coverage of detached processes, scheduled tasks
