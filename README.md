# signal-orchestrator-message

Semantic payload contract carried inside the opaque orchestrator message body.

Message kinds are semantic only: `Guidance` (with a magnitude), `Interruption`,
and `Report`. They say what a message means, not how it is transported.
Threading is transport-level and is not modelled here. `HardInterruption` is
reserved-absent.

The binary wire is rkyv-backed; NOTA projection is only for clients, tests, and
tools.
