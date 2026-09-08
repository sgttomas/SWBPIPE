# U7 Successor Seven-Row Consumer Proof V2

Basis: successor source hashes in `SUCCESSOR_MANIFEST_V2.json`. This is consumer evidence for same-RU backcheck, not formal dependency-row acceptance. `DAG-002-E0482` through `E0485` remain untouched/nonconsumed; N7 accepted history is not substituted for these seven rows.

| Row | Successor behavior and exact evidence |
| --- | --- |
| `DAG-002-E0486` | Canonical node/pipe/load/component IDs and schema paths cross the service boundary; a new endpoint remains ordered `[create_node, connect_pipe_run]`. Exact route payload/order tests pass, the complete response binder verifies every returned identity/diff, App atomic publication produces one checkpoint, and typed inspector forwards the exact selected field intent. |
| `DAG-002-E0487` | Frozen revision/epoch/hash plus complete returned model evidence gate publication. App tests at lines 15880 and 16059 prove delayed Add/Apply selection invalidation publishes nothing. The continuation test proves the component's own accepted model transition, while external revision, save/open, incomplete node, and malformed-context tests preserve existing persistence semantics. |
| `DAG-002-E0488` | Pipe payload preserves existing material ID, OD, wall, explicit length unit, y-reference, and explicit provenance without inventing a section. Route payload tests, straight-pipe/component App tests, and the bounded browser's separate shared-section assignment pass. |
| `DEP-007-02-004` | Explicit units/dimensions remain required and zero coordinates remain valid. Route tests verify unit-bearing geometry; App diffs retain `m`; inspector Apply preserves the exact `350` to `500 N` force intent. |
| `DEP-007-02-005` | Node/pipe provenance must be user-entered and material must resolve to an existing ID. Blank and pointer-only provenance keeps Add disabled; route/App/browser tests prove exact entered provenance reaches review, Apply, tree/inspector, and save/open. |
| `DEP-007-02-006` | Inspector Apply continues to forward the exact public user rule field without synthesizing a rule value or mutating the source model before service application. The typed inspector rule-field test and full App required-input/reference assertions pass. |
| `DEP-007-02-007` | Apply does not copy or invent private rule-pack content/checksums. Existing App reference-only/redaction assertions and typed inspector public modifier forwarding pass unchanged. |

Final checks: route 17/17, App 157/157, inspector 9/9, desktop build PASS, and 1024×768 Chromium journey 1/1. Packaged native evidence remains a separate root gate.
