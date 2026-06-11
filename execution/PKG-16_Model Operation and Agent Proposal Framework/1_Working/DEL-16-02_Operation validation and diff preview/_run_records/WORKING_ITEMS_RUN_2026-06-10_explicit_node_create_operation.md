# WORKING_ITEMS Run Record — TP-APP-R2-CREATENODE-001

Date: 2026-06-10

Deliverable: DEL-16-02 — Operation validation and diff preview

Scope: Completion-plan Phase A3 sub-slice extending the structured operation
validation/apply seam to explicit node creation.

Changes:

- Browser local operation engine and Rust `operation_applier` now resolve
  `create_node` only when the intent targets `object_type=Node` with
  `field_path=nodes`, `before=not_present`, matching project length unit,
  `dimension=length`, no duplicate node id, and a JSON payload containing
  matching id, non-empty label/provenance, and finite numeric x/y/z.
- Generated diff preview rows for accepted explicit create-node intents.
- Applied explicit create-node intents by returning a new session model
  document with the node appended; input model mutation remains false.
- Underspecified legacy viewport node gestures now block with
  `OP-CREATE-NODE-SHAPE-INVALID` instead of inventing coordinates.

Validation:

- `cargo test --manifest-path core/model_operations/operation_applier/Cargo.toml`
  passed, 20/20, including explicit create-node apply and underspecified
  gesture blocking.
- `npm test --workspace apps/desktop` passed, 28/28, including browser-engine
  explicit create-node apply without mutating the input model.
- `npm run build --workspace apps/desktop` passed.
- Targeted `git diff --check` on touched source/evidence files passed.

Boundary:

- No unit conversion, default coordinate inference, direct model mutation,
  durable persistence, protected/private data, telemetry/network path,
  release readiness, professional approval, certification, sealing,
  authentication, or code-compliance claim was introduced.
