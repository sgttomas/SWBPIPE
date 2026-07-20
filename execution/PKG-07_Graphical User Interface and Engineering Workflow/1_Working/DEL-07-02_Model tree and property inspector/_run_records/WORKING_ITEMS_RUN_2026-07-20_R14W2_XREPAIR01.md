# WORKING_ITEMS Run Record - R14-W2-XREPAIR-01 Wind Schema-Mirror Test Repair

Date: 2026-07-20
Agent: W2-XR executor (bounded Agent 2, non-delegating; campaign
`HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14`)
Deliverable: DEL-07-02 - Model tree and property inspector
Package: PKG-07 - Graphical User Interface and Engineering Workflow
Disposition: `R14-W2-XREPAIR-01` (recorded at
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/W2_CROSS_PACKAGE_REPAIR_DISPOSITION.md`)

## What

R14-W2 T4 (DEL-05-01 sub-span wind exposure, landed `a854d43a1`) lawfully
changed `schemas/model.schema.yaml` `WindEquivalentStaticInput`: `required`
is now `[pressure, shape_factor, direction]`, with `exposed_element_refs`
and the new `exposed_spans` as alternative exposure properties under an
`anyOf`. This deliverable's schema-mirror test —
`apps/desktop/src/features/model-tree/schemaSlotEmission.test.tsx`, the
wind required-set assertion cluster inside the "covers the canonical
seismic/wind generation parameter vocabulary one-to-one" test — still
asserted the pre-T4 four-key required set
(`[pressure, shape_factor, direction, exposed_element_refs]`) and
truthfully failed the DEC-025 evidence sweep, blocking the pre-push gate
for the landed in-lane work.

This run applied the one bounded repair the disposition authorizes: the
wind assertion cluster now asserts the landed schema truth
equal-or-stronger —

- `wind.required` equals exactly `["pressure", "shape_factor", "direction"]`;
- both `exposed_element_refs` and `exposed_spans` exist in
  `wind.properties`;
- the preview-key mapping comment is updated truthfully (preview
  `exposed_pipe_refs` maps to the canonical `exposed_element_refs`
  exposure surface; `exposed_spans` is the sub-span alternative exposure
  surface landed by R14-W2 T4, with no preview-surface counterpart);
- the shared-parameter check now compares the preview triple against the
  full `wind.required` array (previously a `slice(0, 3)` of the four-key
  set), which is strictly stronger.

No other line in the test file changed. No product code, schema,
other-deliverable file, dependency, DAG, or register changed.

## Why

Selection-principle basis per the disposition: loop plan Step 1 (a)/(b) —
repair failing validation for already-landed work; a gate prerequisite
beats new scope. The live schema is the single source of truth; the repair
asserts already-landed truth and creates no new vocabulary or criterion.

## Checks

- Focused desktop vitest (from `apps/desktop`, local node_modules,
  offline): `npx vitest run
  src/features/model-tree/schemaSlotEmission.test.tsx` — 1 file passed,
  10/10 tests passed, exit 0.
- Registered evidence sweep (from repo root):
  `run_registered_checks.py … --check evidence-sweep` — status PASS,
  exit 0; exactly one new sweep artifact,
  `validation/evidence/sweeps/SWEEP_20260720T055048Z_27110b28074a-dirty.json`
  (before/after directory snapshots recorded in the instance folder).
- `validate_claims_language.py`, `validate_path_anchors.py --text`,
  `git diff --check`, and changed-path containment
  (`validate_change_scope.py` against the disposition write fence) are
  recorded with exit codes in
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W2-XR/RETURN.md`.

## Boundaries And Residuals

- The W2-reported sub-span GUI-emit follow-on (surfacing `exposed_spans`
  in the desktop authoring UI, TP-PMM-GUIEMIT-001 surface) is OUT of this
  repair and is recorded on the campaign slate for separate lawful
  selection.
- `_STATUS.md` is untouched: this repair closes no Remaining row and makes
  no lifecycle, review, release-readiness, professional, certification,
  sealing, authentication, or code-compliance change.
- No commit, stage, push, PR, merge, or network action by this executor;
  fan-in review, commit, and merge occur separately per the disposition.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
