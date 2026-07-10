# Agent guidance — signal-orchestrator-message

Read `ARCHITECTURE.md` before editing.

This is a payload contract repo. Keep it pure: typed records, binary rkyv wire
shape, NOTA projection for edges, and tests. Do not add routing, delivery,
threading, addressing, or daemon logic. Do not add a `Question` kind or a
`HardInterruption` kind; both are deliberately absent.

Run `cargo fmt`, `cargo test`, and `nix flake check` after Rust changes.
