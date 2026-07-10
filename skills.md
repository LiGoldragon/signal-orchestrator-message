# skills — signal-orchestrator-message

- Payload-only contract: no request/reply channel roots.
- Message kinds are semantic, never transport signals.
- Binary component traffic uses typed rkyv records. NOTA is only projection for
  clients, tests, and tools.
- `Question` and `HardInterruption` are deliberately absent; do not add them.
- No thread field: threading is transport-level.
- Run `cargo fmt`, `cargo test`, and `nix flake check` after Rust changes.
