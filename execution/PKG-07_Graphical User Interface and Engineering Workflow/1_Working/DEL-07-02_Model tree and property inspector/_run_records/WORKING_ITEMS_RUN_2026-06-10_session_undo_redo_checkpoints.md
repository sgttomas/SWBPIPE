# WORKING_ITEMS Run Record — TP-APP-R2-UNDOREDO-001

Date: 2026-06-10

Deliverable: DEL-07-02 — Model tree and property inspector

Scope: Completion-plan Phase A3 sub-slice for local-session undo/redo
checkpoints exposed through the Apply Operations panel.

Changes:

- Added Undo/Redo controls and a local-session history summary to the Apply
  Operations panel.
- Applying a structured operation stores the prior session model and
  selection on an undo stack and clears the redo stack.
- Undo restores the prior model/selection and moves the current model to the
  redo stack; redo restores the undone model/selection. Both paths clear
  stale solve results and keep saved project data untouched.
- Opening a different local project clears session history so checkpoints
  cannot cross project boundaries.

Validation:

- `npm test --workspace apps/desktop` passed, 28/28, including apply → undo
  → redo of explicit viewport node creation.
- `npm run build --workspace apps/desktop` passed.
- Browser smoke on `http://127.0.0.1:5174/` created `node:N-155`, undid it
  so the tree row disappeared with `undo=0; redo=1`, then redid it so the
  active tree row and inspector position `9.1, 2.6, 3.3 m` returned with
  `undo=1; redo=0`. Browser warnings/errors after final reload: none.

Boundary:

- Undo/redo is an in-memory session checkpoint surface only. It does not
  persist changes, rewrite saved project snapshots, infer engineering values,
  perform unit conversion, access protected/private data, or make release,
  professional approval, certification, sealing, authentication, or
  code-compliance claims.
