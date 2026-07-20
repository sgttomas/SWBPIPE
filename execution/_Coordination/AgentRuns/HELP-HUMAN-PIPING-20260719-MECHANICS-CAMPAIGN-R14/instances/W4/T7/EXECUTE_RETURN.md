# EXECUTE_RETURN — T7 DEL-09-04 Reproduction-Manual Stale-Text Refresh

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W4 / T7
**Role:** T7 executor (governed Agent 2, serialized, non-delegating)
**Brief:** `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_T7_DEL-09-04_VALMANUAL_STALE_TEXT.md`
(`CB-2026-07-20-T7-DEL-09-04-VALMANUAL-STALE-001`, EFFECTIVE, verifier
COMMIT-SAFE per `instances/W4/T7/VERIFY_BRIEF.md`)
**Date:** 2026-07-20
**Base commit:** `db9197a5dfb250eaf1f454be865b388e75497364` (branch
`claude/piping-r14-pkg09-evidence`, post-T6 head; freeze-check §4.1
verified the case-1 cell text, the frozen input's `supports[6]`
`restraints: []`, the `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` emission in
`core/product_physics/src/lib.rs` (lines 839 / 7563 / 7574), the T6
bundle tallies (24 cases, 11 matched + 13 blocked, exit 1), and the
`fixtures.len() == 24` readiness assertion — all as the brief recorded;
no material drift)
**Checkout state at start:** clean apart from the lawful pre-existing
untracked W4 T7 state (the T7 instance directory containing
`CURRENT_CANDIDATE_RATIONALE.md` and `VERIFY_BRIEF.md`, and the candidate
brief). No commit, push, branch, or merge was performed by this executor.

Paths relative to `projects/chirality-piping/` unless noted.

## OVERALL STATUS: PASS

## Per-Predicate Table (brief §3.1–§3.8)

| Predicate | Status | Evidence |
|---|---|---|
| §3.1 Case-1 dated note, precedent form + corrected cell | PASS | `docs/validation_manual/headless_runner_reproduction.md` Part 1 gains a dated 2026-07-20 note directly after the existing dated 2026-07-19 case-3 note, in the same form: byte expectation historical for sources at or after R14-W1 T2 (`faee4faed`, on main at `581a15b1c`, PR #292, Receipt-61); still exit 0 / `COMPLETED` / empty request-and-result validation diagnostics / non-empty `result_refs`; one non-blocking `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` warning for `support:CE-120` (frozen `restraints: []`, deliberately non-consuming) plus updated sign-convention text on the two `constant_effort_user_input_review` rows; no byte-match with the committed witness, which remains truthful for its pinned pre-T2 commits and is not edited. The "Expected evidence" cell was corrected in place: all three prior predicates retained verbatim in substance, one true post-T2 envelope-warning statement added, referencing the dated note. Warning framed as review evidence about a non-consuming user-data shape — not a defect, not a solve error, not new acceptance criteria (VERIFY_BRIEF D1 heeded: the run record uses the currency-lapse framing, not "contradicted"). |
| §3.2 Head-anchored live verification BEFORE editing | PASS | Live offline run executed first (§ Live-Run Evidence below); every observed value matched the §3.1 note content; page edited only afterward. Build prerequisite present (offline cargo run succeeded). |
| §3.3 Part 2 dated currency note | PASS | "Per-Case Reporting and Fail-Closed Semantics" gains a dated 2026-07-20 note: pinned R12-head figures remain truthful for `60841413a`; R14 head (`e315fb840` base) mechanics suite carries 24 fixtures (`fixtures.len() == 24`, `validation/benchmarks/mechanics/src/lib.rs:6057`; the three R14 additions named: `MECH-CONSTANT-EFFORT-SUPPORT-APPLIED-LOAD`, `MECH-CURVED-BEND-PRESSURE-THRUST-ARC`, `MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE`); committed T6 bundle `BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4/` cited with its recorded tallies (24 cases, 11 `executed_and_matched` + 13 `blocked`) and exit 1; regression-evidence-only framing; TBD owner gates restated; stress (12/15 + 3) and nonlinear (5/5) figures noted unchanged. |
| §3.4 No other page content changes in meaning | PASS | The page diff vs `db9197a5d` touches exactly three surfaces: the case-1 cell, the inserted case-1 dated note, and the inserted Part 2 currency note. Frontmatter (`OPS-VALIDATION-MANUAL-HEADLESS-RUNNER-REPRODUCTION`, `draft_evidence`), Scope, Authority table, case-2/case-3 rows, the case-3 dated note, both procedures, Bound Fixture Set, Rerun Consequence, Review Checks, Recorded Reproduction Deltas, and Remaining E2 Work are byte-identical. |
| §3.5 Frozen surfaces byte-identical | PASS | `git status --porcelain` empty over `validation/witness/**`, `validation/evidence/**`, `fixtures/**`, `validation/benchmarks/**`, `validation/hand_calcs/**` (check 3 below) — covers the seven tp_runner_015 surfaces, the eleven del1005 surfaces, all reproduction bundles, sweeps, and the T6 bundle. |
| §3.6 Claims calibration | PASS | All new text is regression/verification-evidence posture (DEC-081); no new tolerance, threshold, acceptance criterion, or normative content; both notes restate the TBD owner gates and the not-a-defect framing; the page's Scope statement that acceptance and professional judgment remain with the responsible engineer is untouched. `validate_claims_language.py` PASS after all durable writes (check 5). |
| §3.7 Bounded state update | PASS | `_STATUS.md`: `## Remaining` byte-identical to base (verified by section extraction against `git show db9197a5d`, SHA-equal both bullets); exactly one new History entry (bullet count 15→16, newest-first); `Last Updated` 2026-07-20; `Current State: IN_PROGRESS` unchanged. `MEMORY.md`: exactly one new newest-first entry recording the two dated notes, the three no-edit dispositions (§2.3–§2.5), and the rerun-consequence restatement. One new `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W4_T7_VALMANUAL_STALE_TEXT.md`. |
| §3.8 Non-acts | PASS | No reproduction run bundle; no witness/fixture/code/schema/test/tool/`validation/**` write; no lifecycle, stage, promotion, acceptance, or receipt act; no DEL-10-05 write; §6 plan passed (tally below). No Remaining bullet closed; nothing promoted. |

## Live-Run Evidence (brief §3.2 / §6 first check)

Command (offline; ephemeral outputs to the session scratchpad, outside all
durable paths):

```bash
CARGO_NET_OFFLINE=true cargo run --offline \
  --manifest-path core/runner/headless/Cargo.toml \
  --bin openpipestress-runner -- solve \
  --input validation/witness/inputs/tp_runner_015_final_cli_solve_input.json \
  --output <scratch>/t7_case1_head.json
```

- Exit code: **0**.
- `runner_result.job.state`: **`COMPLETED`**; `runner_result.diagnostics`
  `[]`; `request_validation.diagnostics` `[]`;
  `result_validation.diagnostics` `[]`; `runner_result.result_refs`
  length **830** (non-empty).
- Warning diagnostic: exactly one `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED`
  in the whole output, located at `mechanics_envelope.diagnostics[4]`
  (the solve result envelope), severity `"warning"`, id
  `diagnostic:constant-effort-support:support-CE-120:not-consumed`,
  `affected_refs` `["support:CE-120", "node:N-120"]`.
- SHA-256, regenerated `--output` file:
  `b3cd85af85655eadb827f366457494387ba4b58807fd5608c676958b37168613`
  (byte-identical to the stdout capture, and identical to the W1 T2
  executor's recorded post-change stdout digest —
  `instances/W1/T2/EXECUTE_RETURN.md`, "Pinned-Case And del1005 Results").
- SHA-256, committed witness
  `validation/witness/generated/tp_runner_015_final_cli_solve.json`:
  `c406d9c2d8b6e739cd8faf86fcd67ff8f685342f9ee056b5544685a769705188`.
- Byte-match regenerated-vs-witness: **False** (as the dated note states;
  the committed witness carries 822 `result_refs` and the pre-T2
  review-row text, consistent with the page's 2026-07-10 Recorded
  Reproduction Delta and the W1 T2 chain).
- No observed value contradicted the brief §3.1 note content.

## Check Tally (brief §6, in order; all offline; every step halting)

| # | Check | Outcome |
|---|---|---|
| 1 | §3.2 live offline case-1 verification (command above, run BEFORE any page edit) | PASS — exit 0, `COMPLETED`, empty request/result validation diagnostics, 830 `result_refs`, one envelope warning for `support:CE-120`, no witness byte-match |
| 2 | Page-assertion cross-check | PASS — every asserted value in the two notes and the corrected cell anchored: live run (exit/state/diagnostics/refs/SHAs); frozen input bytes (`supports[6]` `support:CE-120`, `constant_effort_support`, `restraints: []`, 375 N); `core/product_physics/src/lib.rs` emission site (lines 839/7563/7574); T6 bundle `SUITE_RUN_MECHANICS.json` (24 entries, 11 matched + 13 blocked, 0 mismatched, `requested_case_count` 24) and `MANIFEST.json` (`recorded_exit_code` 1, derivative/non-authoritative label); `validation/benchmarks/mechanics/src/lib.rs:6057` `assert_eq!(fixtures.len(), 24)`; the three R14 fixture additions from the `723c95b0f..e315fb840` README inventory delta and wave commits `faee4faed`/`6326b2f93`/`a854d43a1`; wave provenance and stress/nonlinear currency from preserved W1/W2 chain evidence (W1-C6, W2-C7). No asserted value originates in this tranche |
| 3 | Frozen-surface guard: `git status --porcelain` over `validation/witness/**`, `validation/evidence/**`, `fixtures/**`, `validation/benchmarks/**`, `validation/hand_calcs/**` | PASS — empty |
| 4 | DEL-09-04 `## Remaining` byte-identity vs `db9197a5d` (section extraction, SHA-256 compare) | PASS — byte-identical, both bullets |
| 5 | `python3 tools/validation/validate_claims_language.py --repo-root .` (REPO_ROOT) | PASS — 262 files scanned, DEC-081 registry taxonomy satisfied (re-run after all durable writes including this file) |
| 6 | `python3 tools/validation/validate_path_anchors.py . --text` (REPO_ROOT) | PASS — no literal home-dir absolute paths in live path-anchor surfaces (re-run after all durable writes) |
| 7 | `git diff --check` (REPO_ROOT) | PASS (re-run after all durable writes) |
| 8 | JSON parse of new/changed `.json` | PASS — only `instances/W4/T7/CHANGE_SCOPE_CONTAINMENT.json` is new; parses; no project `.json` changed |
| 9 | `python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT" --base db9197a5d --allowed <each §5 fence path> --path <each tranche write>` (mirroring the W1 T2 check-13 convention) | PASS — status PASS, 0 violations; JSON stdout persisted to `instances/W4/T7/CHANGE_SCOPE_CONTAINMENT.json`; the working tree's remaining untracked files are the pre-existing lawful W4 state (the candidate brief and the T7 verifier artifacts), not writes of this run |

Branch-level registered checks (`piping-pytest`, `evidence-sweep`,
`harness-pytest`, `harness-self-check`) run once at W4 wave closeout per
the controlling dispatch; no sweep artifact was created by this tranche.

## Changed Paths (6 durable writes, all inside the §5 fence)

1. `docs/validation_manual/headless_runner_reproduction.md` (modified —
   case-1 cell corrected; case-1 dated note; Part 2 currency note)
2. `execution/PKG-09_Verification, Validation, and Quality Oracles/1_Working/DEL-09-04_Validation manual skeleton/_STATUS.md`
   (modified — one History entry; `Last Updated`; Remaining byte-identical)
3. `.../DEL-09-04_Validation manual skeleton/MEMORY.md` (modified — one
   new newest-first entry)
4. `.../DEL-09-04_Validation manual skeleton/_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W4_T7_VALMANUAL_STALE_TEXT.md`
   (NEW)
5. `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W4/T7/EXECUTE_RETURN.md`
   (NEW — this file)
6. `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W4/T7/CHANGE_SCOPE_CONTAINMENT.json`
   (NEW)

The candidate brief was not modified by this executor (its §10 status
record was already EFFECTIVE when execution was released).

## Recorded Follow-Ons (routed, not fixed here)

- **Fallback fixture (brief §2.3):**
  `fixtures/product_preview/invented_mechanics_result.json` still carries
  the superseded "no global constant-effort load…" review-row text (W1 T2
  VERIFY_IMPL D1). Not referenced by the manual; pinned
  earlier-generation committed output outside the docs lane. Remains
  routed for a future code/fixture-lane selection.
- **`validation.rs` info-text (brief §2.4):** the
  `CONSTANT_EFFORT_USER_DATA_REVIEWED` clause "no global constant-effort
  solve behavior … is claimed" remains literally true of the validation
  acceptance itself; code surface outside the docs lane; observation
  recorded only.
- **Wind schema text (brief §2.5):** no surface on this page (verified
  absent); no action.

## Enumerated Refutable Claims (for the fresh implementation verifier)

- E1. The page diff vs `db9197a5d` consists of exactly three insertions/
  changes: (a) the case-1 "Expected evidence" cell now ends with "; on
  sources at or after R14-W1 T2, the solve result envelope additionally
  carries one non-blocking `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` warning
  for `support:CE-120`, per the dated 2026-07-20 note below." with the
  three prior predicates retained verbatim; (b) one dated 2026-07-20 note
  inserted between the case-3 dated note's final paragraph and "The
  committed generated witness outputs are:"; (c) one dated 2026-07-20
  currency note appended to the "Per-Case Reporting and Fail-Closed
  Semantics" section. Nothing else on the page changed (frontmatter
  byte-identical).
- E2. Running the documented frozen case-1 solve command offline at the
  implementation base yields exit 0, `runner_result.job.state`
  `COMPLETED`, empty `runner_result.diagnostics` /
  `request_validation.diagnostics` / `result_validation.diagnostics`, 830
  `result_refs`, and exactly one `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED`
  diagnostic in the entire output, located in
  `mechanics_envelope.diagnostics`, severity `warning`, subject
  `support:CE-120`, with output SHA-256
  `b3cd85af85655eadb827f366457494387ba4b58807fd5608c676958b37168613`.
- E3. The committed witness
  `validation/witness/generated/tp_runner_015_final_cli_solve.json` has
  SHA-256
  `c406d9c2d8b6e739cd8faf86fcd67ff8f685342f9ee056b5544685a769705188`,
  carries 822 `result_refs`, no `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED`
  diagnostic, and the pre-T2 review-row sign-convention text — and is
  byte-identical to its state at `db9197a5d` (untouched by this tranche).
- E4. The frozen solve input's `supports[6]` is
  `{id: support:CE-120, family: constant_effort_support, restraints: [],
  hanger.constant_load: 375 N}` (7 supports total), and
  `core/product_physics/src/lib.rs` emits the
  `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` warning at envelope level
  (`append_constant_effort_consumption_diagnostics`, called at line 839)
  for exactly this empty-restraints non-consuming shape.
- E5. The Part 2 currency note's every figure matches its committed
  anchor: `SUITE_RUN_MECHANICS.json` in
  `validation/evidence/benchmarks/BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4/`
  contains 24 case entries tallying exactly 11 `executed_and_matched` +
  13 `blocked` (0 mismatched) with `requested_case_count` 24;
  `MANIFEST.json` records exit code 1 and the derivative/non-authoritative
  label; `validation/benchmarks/mechanics/src/lib.rs` line 6057 asserts
  `fixtures.len() == 24`; the three named R14 additions are exactly the
  three fixture ids added to `validation/benchmarks/mechanics/README.md`
  between `723c95b0f` and `e315fb840`.
- E6. The stress and nonlinear pinned figures are unchanged at head:
  `git log 60841413a..HEAD` over `validation/benchmarks/stress` and
  `validation/benchmarks/nonlinear` is empty, and the five
  `del1005_payload_binding_*` generated witnesses are byte-identical to
  their state at `db9197a5d`.
- E7. `_STATUS.md` `## Remaining` is byte-identical to `git show
  db9197a5d` (both bullets); exactly one History bullet was added
  (15→16); `Current State: IN_PROGRESS` and the frontmatter-free header
  form are unchanged; `Last Updated` is 2026-07-20.
- E8. `MEMORY.md` gained exactly one entry, inserted newest-first
  directly under the file title, recording the two dated notes, the three
  §2.3–§2.5 no-edit dispositions, and the rerun-consequence restatement;
  no existing entry was modified.
- E9. The changed-path set at head vs `db9197a5d` is exactly the 6 paths
  listed above, all inside the §5 fence (persisted containment JSON
  status PASS, 0 violations); the remaining untracked files (the
  candidate brief, `VERIFY_BRIEF.md`, `CURRENT_CANDIDATE_RATIONALE.md`)
  are pre-existing lawful W4 state, not writes of this run.
- E10. No file under `validation/**`, `fixtures/**`, `core/**`,
  `schemas/**`, `tests/**`, or `tools/**` is in the diff; no other
  `docs/**` file (including `docs/validation_manual/index.md` and the
  case pages) changed; `loop/LOOP_RECEIPTS.md` is untouched (cursor
  remains Receipt-62).
- E11. All new text passes `validate_claims_language.py` (DEC-081) and
  contains no new tolerance, threshold, acceptance criterion, or
  normative content; the warning is consistently described as review
  evidence about a non-consuming user-entered data shape, never as a
  defect or solve error.
- E12. All commands ran offline (`CARGO_NET_OFFLINE=true`, `--offline`);
  no network, install, provisioning, commit, push, branch, or merge
  action was performed by this executor; live-run outputs were ephemeral
  (session scratchpad only, outside all durable project paths).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
