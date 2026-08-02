---
doc_id: R22-N3-D45-CODIFICATION-LAUNCH
doc_kind: coordination.launch_brief
status: SEALED
created: 2026-08-01
---

# Launch brief — N3 D-45 Option O-B codification

- Parent: `HELP_HUMAN`
- Managed role: `SOFTWARE_DECOMP` (Agent 1)
- Run ID: `HELP-HUMAN-PIPING-20260801-D63-D45-TM-R22`
- Node: `N3_D45_CODIFICATION`
- Working root: `{REPO_ROOT}/projects/chirality-piping`
- Frozen Git basis: `3c2e816f1072295de15fdcdf924c19b4b66497bc`
- Active branch: `codex/piping-d63-d45-rulings`
- Dependency: `N2_TM_REGISTER_MIGRATION` completed
- Delegation: none

## Authority

The owner ruled verbatim `2) D-45 ruling: O-B.` The subject is the exact
pre-ruling D-45 proposal at SHA-256
`c7e2cb465df438c413e56beaaeb1e13fdbf85bb46e88b400c01ef1697590d371`.
The owner-intent record at
`execution/_Coordination/OWNER_INTENT_2026-07-31_DESIGN_TOOL_BOUNDARY.md` is
intent of record, not scope, and may not be expanded. The maturity survey is
candidate input only and is not an authority or finding for this node.

## Objective

Record the D-45 O-B ruling separately, append its pointer to the proposal,
change exactly the D-45 decision-register row to ruled, append the next-free
decomposition entry applying the `DEC-077` method to a new explicit
user-entered temperature-point G field, and replace DEL-05-02's former ruling
gate with truthful unimplemented Remaining work. Do not claim implementation
or lifecycle change.

## Read scope

- root and project agent/loop instructions
- the R22 plan, graph, and N2 return
- D-45, D-38, `DEC-077`, and the decision register
- live material/model schemas and product-physics modulus-basis implementation
- DEL-05-02 context, status, memory, scope declaration, and DEC-077 run record
- the owner design-tool intent record, as intent only

## Write scope

- `execution/_Coordination/_DECISIONS/D-45_RULING_2026-08-01.md`
- `execution/_Coordination/_DECISIONS/D-45_temperature_indexed_shear_modulus.md`
- exactly the D-45 row in `execution/_Coordination/_DECISIONS/_REGISTER.md`
- exactly one next-free decision row in
  `execution/_Decomposition/SOFTWARE_DECOMP.md`
- DEL-05-02 `_STATUS.md` and `MEMORY.md` only
- this `instances/N3_D45_CODIFICATION/` directory
- only the N3 and N4 `status` values in the R22 `WORK_GRAPH.json`, after the
  fan-in gate passes

No schema, core, app, test, fixture, benchmark, Task Management register,
D-62/D-63 surface, root register, notice, receipt, `LOOP_INIT.md`, product
direction, or Git state may be written.

## Codification contract

- O-B only: add explicit user-entered temperature-point G.
- Apply the existing `DEC-077` method to G: exact-id basis retained, linear
  interpolation only between qualifying adjacent user-entered points,
  two-source and method provenance, and blocking without extrapolation.
- Base G remains the no-temperature-basis behavior; selected temperature
  bases may not silently fall back to base G.
- The decision is subject to a later bounded implementation/evidence tranche.
  Current behavior remains unimplemented and DEL-05-02 remains `IN_PROGRESS`.

## Acceptance checks

Verify the exact owner bytes/hash and pre-ruling proposal hash; current live
temperature-point E/alpha-without-G shape; current base-G cloning; exact
`DEC-077` wording; `DEC-092` as next free; exactly one D-45 register-row
replacement; status/memory truth; no implementation or lifecycle claim; path
containment; R22 graph/status JSON; and `git diff --check`.
