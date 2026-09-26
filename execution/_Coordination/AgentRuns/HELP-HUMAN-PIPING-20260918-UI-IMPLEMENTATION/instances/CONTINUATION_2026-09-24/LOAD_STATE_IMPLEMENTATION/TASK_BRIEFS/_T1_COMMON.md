# Common terms for T1 wave-1 TASKs

These terms apply to every T1 wave-1 brief (`T1_*.md`) in this folder.

- **Role.** TASK (Type 2). You do not delegate.
- **Manager.** The T1 WORKING_ITEMS manager (load/reference states) requested you. Report to it by `SendMessage`; a final text answer alone does not reach it.
- **Checkout.** The load-state worktree, on branch `codex/piping-load-states-20260925`. Start from the commit named in the spawn request or later. ROOT gives the concrete locations of the worktree, venv and scratch space in the spawn request.
- **Paths.** Relative to WORKING_ROOT = `projects/chirality-piping/`. `LSI` is `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/LOAD_STATE_IMPLEMENTATION`.
- **Git.** Make no Git writes: no staging, committing, pushing or branch changes. The manager integrates and commits.

## Authority and basis

- ROOT and the owner decided D1–D4 on 2026-09-26 (`OWNER_T1_DECISIONS_2026-09-26.md`, landing on main via ROOT's records PR):
  - **D1:** activate `openpipestress.result_semantics/0.3.0/load-reference-source-1` (profile `resolved_straight_load_state_source_v1`, receipt policy `LOAD-REFERENCE-SOURCE-1`) in the T1 PR, once its readers pass independent review;
  - **D2:** plain input fields; typed operations first;
  - **D3:** no backwards compatibility. 0.4.0 is for new models. There is no upgrade operation and no legacy `imposed_displacement` mapping.
  - **D4:** write scopes as granted in each brief.
- The wire of record:
  - `LSI/CP2_WIRE.md`;
  - `CP2_WIRE_ADDENDUM_1.md`;
  - `CP2_WIRE_ADDENDUM_2.md` (§5 is the joined method);
  - `CP3_WIRE_ADDENDUM.md`;
  - `CP4_WIRE_ADDENDUM.md`, as corrected by `CP4_REVIEW_DISPOSITION.md`.
- `LSI/T1_PLAN.md` gives context, and `LSI/CHECKPOINT_3.md`/`CHECKPOINT_4.md` give the current state.

## Constraints

- Every pre-existing contract, table, schema branch, raw fixture and carrier keeps its bytes and meaning. Changes are additive.
- Never populate a material or component library or a code rule, and never add an agent-invented default. All test inputs are invented and marked as such.
- Never weaken a protected test, tolerance, oracle or reference to get a pass. Never change a reference or a producer fixture to match an observation. A mismatch goes to the manager.
- Do not write `core/product_physics/**`, `apps/desktop/**`, `core/runner/headless/**` or another TASK's files. If you need a producer seam, ask the manager.
- These are reserved for another tranche (T0R); do not touch them:
  - in the product: `straight_summary_extrema`, `open_formula_summary_mpa`, headline/maxima selection, and the reaction-resultant publication;
  - the source-blocks readers, `core/reporting/result_export/src/source_blocks.rs` and `core/analysis_runs/source_blocks.py`;
  - the Current/rule/export standing functions in `semantic_contract.rs` and their Python peers. A T1 change in `semantic_contract.rs` may only add T1 identities to its enumerations and dispatch.
  (Added 2026-09-26 at ROOT's direction. The wave-1 briefs predate it; the manager relayed it to WP1_JOINED_READERS by message.)
- **After T0R.** T0R adds a static rule, in all three languages, that precision-1 is never a fresh identity. When T1 rebases after T0R, T1's own identities are added to that set.

## Build and disk

- Toolchain: `cargo +1.97.1 … --locked --offline -j 2`, with your own `CARGO_TARGET_DIR` under the given scratch space. Delete it when you finish.
- Python: the given venv, with `PYTHONDONTWRITEBYTECODE=1` and `-p no:cacheprovider`.
- Disk is shared, with about 15 GB free. Keep at most one scratch copy and one target at a time.
- Run mutants on a scratch copy (`git archive`), never in place.

## Return

- Write `LSI/<YOUR_FOLDER>/RETURN.md`, with logs and scripts under `LSI/<YOUR_FOLDER>/_run_records/`.
- Keep absolute machine paths out of every file you write.
- Include:
  - files changed, with sha256 before and after;
  - what each change does;
  - check commands and counts;
  - mutation evidence;
  - anything not done;
  - design questions.
- Then `SendMessage` the manager a short summary and the path.
