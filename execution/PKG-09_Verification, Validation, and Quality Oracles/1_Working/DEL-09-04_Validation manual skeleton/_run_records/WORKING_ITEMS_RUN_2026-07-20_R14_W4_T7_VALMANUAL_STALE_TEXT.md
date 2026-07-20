# WORKING_ITEMS RUN — DEL-09-04 Reproduction-Manual Stale-Text Refresh (R14-W4 T7)

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14`, wave W4, tranche T7
**Executor:** T7 executor (governed Agent 2, serialized, non-delegating)
**Date:** 2026-07-20
**Adopted brief:** `CB-2026-07-20-T7-DEL-09-04-VALMANUAL-STALE-001`
(`execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_T7_DEL-09-04_VALMANUAL_STALE_TEXT.md`;
EFFECTIVE — execution released by the W4 manager under the R14 campaign
chain after the fresh-context brief verifier returned `COMMIT-SAFE`,
`instances/W4/T7/VERIFY_BRIEF.md`)
**Base commit:** `db9197a5dfb250eaf1f454be865b388e75497364`
(branch `claude/piping-r14-pkg09-evidence`, post-T6 head on post-wave-2
main base `e315fb840`; tree clean before execution apart from this run's
lawful pre-existing untracked state: the W4 T7 instance directory with the
verifier artifacts and the candidate brief). No commit, push, branch, or
merge was performed by this executor.

Paths relative to `projects/chirality-piping/` unless noted.

## 1. Scope executed (brief §3.1, §3.3, §3.7, inside the §5 fence)

1. **`docs/validation_manual/headless_runner_reproduction.md`** — three
   bounded edits, R13 precedent form, nothing else changed in meaning
   (frontmatter untouched):
   - dated 2026-07-20 historical note in Part 1 (parallel to the existing
     dated 2026-07-19 case-3 note): case 1's committed-witness byte
     expectation is historical for sources at or after R14-W1 T2
     (implementation commit `faee4faed`, on main at `581a15b1c`, PR #292,
     Receipt-61); the same frozen solve command still exits 0 /
     `COMPLETED` with empty request/result validation diagnostics and
     non-empty `result_refs`, and the solve result envelope additionally
     carries one non-blocking `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED`
     warning for `support:CE-120` (frozen `restraints: []`, a deliberately
     non-consuming shape) plus updated sign-convention disclosure text on
     its two `constant_effort_user_input_review` rows; the committed
     witness remains truthful for its pinned pre-T2 commits and is not
     edited; the warning is review evidence, not a defect and not a new
     acceptance criterion;
   - case-1 "Expected evidence" cell corrected in place: all prior
     predicates retained (`COMPLETED`; empty request/result validation
     diagnostics; non-empty `result_refs`) with the post-T2
     envelope-warning statement added, referencing the dated note;
   - dated 2026-07-20 currency note in Part 2 "Per-Case Reporting and
     Fail-Closed Semantics": pinned R12-head figures remain truthful for
     `60841413a`; at the R14 head (`e315fb840` base) the mechanics suite
     carries 24 fixtures (readiness assertion `fixtures.len() == 24`; R14
     additions `MECH-CONSTANT-EFFORT-SUPPORT-APPLIED-LOAD` (W1 T2),
     `MECH-CURVED-BEND-PRESSURE-THRUST-ARC` (W1 T3),
     `MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE` (W2 T4)); the committed T6
     bundle
     `validation/evidence/benchmarks/BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4/`
     records the head whole-suite `run-benchmark` result 24 cases, 11
     `executed_and_matched` + 13 `blocked`, exit 1 (`DEC-065` fail-closed)
     — regression evidence only, no release judgment, thresholds/
     tolerances/CI-gate policy `TBD` owner-gated; stress (12/15 + 3) and
     nonlinear (5/5) pinned figures unchanged by the R14 waves.
2. **`_STATUS.md`** — exactly one new History entry (newest-first) and
   `Last Updated` 2026-07-20; `Current State: IN_PROGRESS` unchanged;
   `## Remaining` byte-identical (both owner-gated bullets, W3 rows 7–8).
3. **`MEMORY.md`** — exactly one new newest-first entry recording the two
   dated notes, the three no-edit dispositions (brief §2.3–§2.5), and the
   R13 rerun-consequence restatement.
4. **This run record** — the one new file authorized under §5 item 3.
5. **Instance artifacts** — `instances/W4/T7/EXECUTE_RETURN.md` and
   `instances/W4/T7/CHANGE_SCOPE_CONTAINMENT.json` (§5 item 4).

No receipt append; no evidence-sweep artifact; no other file.

## 2. Live-run evidence (brief §3.2 mandatory head anchor, run BEFORE editing)

Command (offline, from `WORKING_ROOT`; outputs ephemeral, session
scratchpad only, outside durable paths):

```bash
CARGO_NET_OFFLINE=true cargo run --offline \
  --manifest-path core/runner/headless/Cargo.toml \
  --bin openpipestress-runner -- solve \
  --input validation/witness/inputs/tp_runner_015_final_cli_solve_input.json \
  --output <scratch>/t7_case1_head.json
```

Observed at base `db9197a5d`:

- exit code **0**; `runner_result.job.state` = **`COMPLETED`**;
  `runner_result.diagnostics` = `[]`; `request_validation.diagnostics` =
  `[]`; `result_validation.diagnostics` = `[]`;
  `runner_result.result_refs` length **830** (non-empty);
- exactly one `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` diagnostic, located
  at `mechanics_envelope.diagnostics[4]`, severity `"warning"`, id
  `diagnostic:constant-effort-support:support-CE-120:not-consumed`,
  `affected_refs` `["support:CE-120", "node:N-120"]`;
- SHA-256 of the regenerated `--output` file:
  `b3cd85af85655eadb827f366457494387ba4b58807fd5608c676958b37168613`
  (identical to the stdout capture and to the W1 T2 executor's recorded
  post-change digest, `instances/W1/T2/EXECUTE_RETURN.md`);
- SHA-256 of the committed witness
  `validation/witness/generated/tp_runner_015_final_cli_solve.json`:
  `c406d9c2d8b6e739cd8faf86fcd67ff8f685342f9ee056b5544685a769705188`
  — **no byte-match**, as the dated note states (the committed witness
  also predates the recorded 822→830 result-surface growth, per the
  page's 2026-07-10 Recorded Reproduction Delta);
- the two `constant_effort_user_input_review` rows in the head output
  carry the post-T2 sign-convention disclosure text; the witness's rows
  carry the superseded "no global constant-effort load…" text.

No observed value contradicted the brief §3.1 note content; the page edit
proceeded only after this verification.

## 3. Source and derivative anchors (brief §6 page-assertion cross-check)

- Frozen input `validation/witness/inputs/tp_runner_015_final_cli_solve_input.json`:
  `supports[6]` = `support:CE-120`, family `constant_effort_support`,
  `restraints: []`, `hanger.constant_load` 375 N (7 supports total).
- `core/product_physics/src/lib.rs`: envelope-level
  `append_constant_effort_consumption_diagnostics` (line 7563, called at
  line 839) pushes code `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED`
  (line 7574), severity warning, via
  `classify_constant_effort_consumption` (line 7496).
- T6 bundle `SUITE_RUN_MECHANICS.json`: suite `mechanics`,
  `requested_case_count` 24, 24 case entries tallying 11
  `executed_and_matched` + 13 `blocked`, 0 mismatched; `MANIFEST.json`:
  `recorded_exit_code` 1, label `DERIVATIVE EVIDENCE — NON-AUTHORITATIVE`.
- `validation/benchmarks/mechanics/src/lib.rs:6057`
  `assert_eq!(fixtures.len(), 24)`; the three R14 additions confirmed by
  the `git diff 723c95b0f e315fb840` README inventory delta and the wave
  commit log (`faee4faed`, `6326b2f93`, `a854d43a1`).
- Wave provenance and stress/nonlinear currency: preserved W1/W2 chain
  evidence (`instances/W1/T2/EXECUTE_RETURN.md`; `instances/W1/RETURN.md`
  W1-C6; `instances/W2/RETURN.md` W2-C7); no asserted value originates in
  this tranche.

## 4. Recorded dispositions and rerun consequence

- **No page edit (brief §2.3):** `fixtures/product_preview/invented_mechanics_result.json`
  still carries the superseded "no global constant-effort load…"
  review-row text (W1 T2 VERIFY_IMPL D1); not referenced by the manual;
  remains routed for a future code/fixture-lane selection — not fixed here.
- **No page edit (brief §2.4):** `core/product_physics/src/validation.rs`
  `CONSTANT_EFFORT_USER_DATA_REVIEWED` info-text remains literally true of
  the validation acceptance itself; observation only.
- **No page edit (brief §2.5):** the W2 wind schema required-set text has
  no surface on this page (verified absent).
- **R13 rerun trigger honored:** this tranche is a "page through another
  lane" event under the R13 brief §8; consequence restated on the page and
  in MEMORY — any subsequent clean-checkout reproduction executes from a
  post-refresh source commit under a fresh run ID and a new immutable
  bundle; completed bundles are never edited.

## 5. Checks run (brief §6, in order; each a halting step)

See `instances/W4/T7/EXECUTE_RETURN.md` for the full tally with commands
and outcomes: live-run verification PASS; page-assertion cross-check PASS;
frozen-surface guard PASS (empty `git status --porcelain` over
`validation/witness/**`, `validation/evidence/**`, `fixtures/**`,
`validation/benchmarks/**`, `validation/hand_calcs/**`); DEL-09-04
`## Remaining` byte-identity PASS; claims-language validator PASS;
path-anchors validator PASS; `git diff --check` PASS; JSON parse PASS
(only the containment JSON is new); changed-path containment PASS
(`instances/W4/T7/CHANGE_SCOPE_CONTAINMENT.json`). Branch-level registered
checks run once at W4 wave closeout per the controlling dispatch.

## 6. Boundaries preserved

No reproduction run or bundle; no witness, fixture, code, schema, test,
tool, or `validation/**` write; no lifecycle, stage, promotion,
acceptance, threshold, tolerance, or receipt act; both Remaining bullets
byte-preserved (owner-gated per W3 rows 7–8); no MAINTAINER_REVIEWED
promotion; no GUI-workflow evidence; no DEL-10-05 work; no push, PR,
network, or external state change. The non-consumption warning is
described throughout as review evidence about a non-consuming user-data
shape — not a defect and not new acceptance criteria.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This
record documents bounded documentation-currency and deliverable-state work
only; it makes no validation-acceptance, release-readiness, reliance,
professional, certification, sealing, authentication, or code-compliance
claim.
