# Scope

## One-line pitch
Residue watches what a Windows installer actually does — filesystem,
registry, and firewall changes — and attributes each change to the
process that made it.

## v1 definition
- Monitors filesystem changes under user-specified watch root(s) during
  a single installer run
- Monitors registry changes under HKCU and HKLM\Software during the
  same run
- Diffs firewall rules before/after the run
- Basic attribution: every event tagged with the PID that triggered it;
  attributed to the original installer process only if it is a direct
  child in the process tree. Anything outside that lineage is logged as
  "unattributed," not guessed at.
- Outputs a single JSON diff file per run
- Runs as a CLI tool, invoked as `residue monitor <installer.exe>`

## Explicitly NOT in v1
- GUI (comes after CLI/engine is solid — v1.x)
- Service/daemon mode (v1 runs in foreground, manually invoked)
- Static analysis of installer files (MSI table parsing, etc.) — v2 idea
- Uninstall execution (Residue reports, it does not remove — that may
  never be in scope, TBD)
- Attribution across detached/elevated child processes, scheduled tasks,
  or post-install background activity
- Driver-based monitoring (ETW/API-level only for v1)

## Subsystem build order
1. Directory monitoring — [ ]
2. Registry monitoring — [ ]
3. Firewall rule diffing — [ ]
4. PID-lineage attribution — [ ]
5. JSON diff output — [ ]

## Definition of shippable v1
All five items above working end-to-end against a real installer, one
worked example in the README, tagged as v1.0.0.

## Licensing note
MIT or Apache 2.0, decide before first external contribution.
