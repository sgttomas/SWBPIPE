# Piping session handoff — 2026-09-25 (second session)

The owner directed this session to bring its work to completion and pause, then approved three follow-ons: merge PR905, independently review load-state checkpoint 3, and apply the stress-neutral edit. All three are done. This is a pause checkpoint, not undertaking completion, a final receipt, engineering acceptance or release.

## Landed

- **[PR905](https://github.com/sgttomas/chirality/pull/905) merged** into main at `23aad15d6fd08638c6fa6a017b8aa128dd36d02a`. Verified: the PR head `0d7805565` is in main's history, and the Piping tree is identical to it.
  - Qualification of head `6d9c0915f`:
    - hosted CI green;
    - the manual full dual-viewport dispatch, run 36190876950, as the surface-4 binding;
    - a clean five-surface DEC-025 sweep;
    - the D-GOV-45 leak scan, with no credentials found;
    - a final independent readiness review, READY.
  - The final records-only head then passed hosted CI. Evidence is in main's `ENGINE_INTEGRATION/CI_REPAIR/`: `DEC025_SWEEP`, `FINAL_READINESS`, and the earlier repair and review folders.
  - The current graph is main's `execution/_Coordination/WorkGraphs/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/WORK_GRAPH.md`; see its section "PR905 qualification — resumed session". Its "Merge needs green hosted CI" line is now satisfied by that final run.
- Original finding closure remains **4 of 38** (M04/M09/M24/M35). PR905 does not close a further whole group by itself. The next session should reassess M01/M03/M05/M08/M14/M33/M34 against merged main before updating any closure.

## Load/reference state (branch `codex/piping-load-states-20260925`)

- **Current state.** Branch head `41163f7f9` carries checkpoints 1–3 (M10/M16/M29 resolved-case facade, retained-source join, readers, schemas and stress-neutral support) and the independent checkpoint-3 review.
  - The branch is based on c278, not on main. Merge main (which now contains PR905) before further work. That brings in the upstream fixes for the numerical_integrity lock and the stress-neutral test.
- **Next work.** Implement the ROOT dispositions in `LOAD_STATE_IMPLEMENTATION/REVIEW_CHECKPOINT_3/ROOT_DISPOSITION.md`:
  - SF-1: fall back to ordinary publication when a selected join cannot finalize, and reserve the replay budget before selecting;
  - notes N-1 to N-5.
  - Then: connected desktop types/authoring/persistence, headless/native adapters and native witnesses, a mergeable PR with its own qualification, and reassessment of the M10/M16/M29 closure.
- **Reserved identifiers.** `openpipestress.result_semantics/0.3.0/load-reference-source-1` and profile `resolved_straight_load_state_source_v1` stay reserved and inactive until the join candidate passes review in a mergeable slice.

## Carried items

- **B6:**
  - native WebKit layout of the wrapped result-filter row;
  - `.result-page-row` narrow-column overlap;
  - the dist 1024x768 focus-outline observation, which also occurs on main.
- **Witness inputs:** the DEL-10-05 witness inputs and their generator still carry solver identity `product_physics` 0.1.0. Regenerating them needs its own scope.
- **Legacy fixture:** the precision-1 fixture pair predates the legacy-pressure refusal; regenerate or annotate it in the next Results/fixture slice.
- **Composite failure:** the inherited composite `SOURCE_BLOCKS_FINALIZATION_FAILED` fail-closed behaviour of physics-source-1 is a connected open finding.
- **Preserved elsewhere:** general UI/B4/C4/live checkpoints, the six deferred UI files and the untracked-record archive on the owner's machine, the resolved C4 decisions, and the unperformed actual-human live witnesses all stay as recorded in the previous handoff (`codex/piping-pipes-20260924` at `94a1cbb`).

## Execution notes for the next session

- **Delegation:** subagents in this host cannot spawn subagents. ROOT spawns each TASK with the full role, a named manager and SendMessage reporting. Only ROOT widens a TASK's write boundary.
- **Host permissions:** the classifier may refuse writes it considers shared resources. Take such a refusal to the owner; do not route around it.
- **Surface-4 binding:** after the CI burden reduction, a PR-event run is not a DEC-093 binding. Dispatch `piping-desktop-e2e.yml` with a full 40-character `target_base`.
- **Local environment:** Node 24 and Rust 1.97.1 are needed, and `zsh` for the governance tools tests. Fetch `archive/*` tags before the live archive test. The shallow initial clone must be unshallowed for the self-check.
