# U7 R3 Seven-Row Consumer Proof

Basis: R3 source hashes in `R3_SUCCESSOR_MANIFEST_V3.json`. This is consumer evidence for same-RU backcheck, not formal dependency-row acceptance. `DAG-002-E0482` through `E0485` remain untouched/nonconsumed; N7 accepted history is not substituted for these seven rows.

| Row | R3 behavior and evidence |
| --- | --- |
| `DAG-002-E0486` | Canonical node/pipe/load/component IDs and schema paths cross the existing service boundary; new-end creation remains ordered `[create_node, connect_pipe_run]` in one batch and one checkpoint. Exact payload/order, receipt-binding, atomic publication, and typed-inspector forwarding tests pass. |
| `DAG-002-E0487` | Revision/epoch/frozen hash and complete producer receipt gate publication. R3 additionally distinguishes the engine-created hash from the exact echoed submitted hash and permits continuation only for the accepted direct-Apply commit token. Malformed receipts and coincident external open publish no model, receipt, retained context, or checkpoint; undo/redo and other commits clear continuation. |
| `DAG-002-E0488` | Pipe payload preserves the existing material ID, OD, wall, explicit length unit, y-reference, and entered provenance without inventing a section. Route/App tests and the bounded browser journey pass. |
| `DEP-007-02-004` | Explicit units and dimensions remain required while zero coordinates remain valid. Route tests verify unit-bearing geometry; App diffs retain `m`; inspector Apply preserves exact entered force intent. |
| `DEP-007-02-005` | Node/pipe provenance must be explicitly entered and material must resolve to an existing ID. Blank or pointer-only provenance keeps Add disabled; route/App/browser tests prove entered provenance reaches Apply, tree/inspector, and persistence. |
| `DEP-007-02-006` | Inspector Apply forwards the exact public user rule field without synthesizing a value or mutating the source model before service application. Typed-inspector rule-field and full-App required-input/reference assertions pass unchanged. |
| `DEP-007-02-007` | Apply does not copy or invent private rule-pack content or checksums. Existing App reference-only/redaction assertions and typed-inspector public-modifier forwarding pass unchanged. |

Final-cut checks: route **17/17**, App **162/162**, desktop build **PASS**, Chromium desktop **1/1 PASS** at 1024×768. Native GUI evidence remains a separate root gate.
