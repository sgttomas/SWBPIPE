# WORKING_ITEMS Run Record — TP-APP-R2-CREATENODE-001

Date: 2026-06-10

Deliverable: DEL-07-02 — Model tree and property inspector

Scope: Completion-plan Phase A3 sub-slice proving that explicit viewport node
creation binds back into the model tree and property inspector.

Changes:

- Applied operation outcomes now select the affected entity in the shared app
  selection state. For `Node` targets, the new node becomes the active
  `node` selection after apply.
- The model tree renders the created node row from the returned session model
  document, and the property inspector displays the created node id, label,
  position, and provenance.
- Operation panel layout was adjusted so queued operations remain actionable
  in the browser viewport.

Validation:

- `npm test --workspace apps/desktop` passed, 28/28, including the full
  viewport queue/apply flow asserting `tree-row-node:N-150`, active tree
  selection, inspector position `8.4, 2.4, 2.8 m`, and
  `applied_operations=1`.
- `npm run build --workspace apps/desktop` passed.
- Browser smoke on `http://127.0.0.1:5174/` confirmed the created node was
  active in the tree and visible in the property inspector. Browser
  warnings/errors after the final reload: none.

Boundary:

- The model tree and inspector consume the returned session model only; this
  run does not persist the node, broaden field editing semantics, perform unit
  conversion, infer engineering values, access protected/private data, or make
  release, professional, certification, sealing, authentication, approval, or
  code-compliance claims.
