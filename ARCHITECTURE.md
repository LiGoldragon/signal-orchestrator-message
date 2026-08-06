# signal-orchestrator-message — architecture

`signal-orchestrator-message` owns the semantic payload that an
orchestrator-addressed message carries inside the router's opaque body.

## Boundary

Owned here:

- `OrchestratorMessage` — kind, subject, content;
- `OrchestratorMessageKind` — `Guidance(GuidanceMagnitude)`, `Interruption`,
  `Report`;
- `GuidanceMagnitude` — `Soft`, `Standard`, `Hard`;
- non-empty subject and content text records;
- rkyv-compatible wire records and DOTOS projection for edges.

Not owned here:

- transport, routing, and delivery, which belong to the router;
- thread minting and thread identity, which are transport-level and belong to
  the message daemon;
- addressing and reachability, which belong to `orchestrate`.

## Deliberate absences

- No `Question` kind: a question is an ordinary thread message whose body asks
  something.
- No `HardInterruption`: a hard interruption would be a transport-level
  interrupt, which this semantic payload does not model. Hard urgency is
  `Guidance(Hard)` or `Interruption`.
- No thread field: threading is transport-level.
