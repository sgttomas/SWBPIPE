# W5 MANAGER RETURN — DEL-09-04 Clean-Checkout Reproduction Wave (R14)

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W5
**Role:** WORKING_ITEMS (Agent 1, reproduction-wave manager), reporting to HELP_HUMAN
**Branch:** `claude/piping-r14-w5-clean-repro` (base `a5235340a`, post-wave-3 merged main / PR #294)
**Date:** 2026-07-20 (UTC)
**Status:** WAVE COMPLETE — one tranche LANDED, reproduction `PASS`; no
push, PR, merge, rebase, or receipt performed (HELP_HUMAN fan-in acts).
One post-commit near-miss cure recorded for fan-in (§3: portability
redaction in `VERIFY_IMPL.md` after a first-run `harness-pytest` FAIL;
failed check JSON preserved).

## 1. Tranche Disposition

| Item | Value |
|---|---|
| Brief | `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_DEL-09-04_CLEAN_REPRO_R14.md` (`CB-2026-07-20-DEL-09-04-CLEAN-REPRO-R14-001`), NEW brief per the R11 brief §8 rerun clause (procedure + runner + solver changed after `23eeaabc9`); R11 pinned predicates stale, R11 bundle immutable and untouched |
| Adoption | owner standing approval D-52/`DEC-085` as refined by D-54/`DEC-087`, SHA-bound at `f14fa77518a06f112ae72a8fcce4de0fab958d47`; `OwnerCaseSelection: NONE`; D-54 rationale at `instances/W5/CURRENT_CANDIDATE_RATIONALE.md` (ten-class screen no hit) |
| SOURCE_COMMIT | `a5235340aae3c41cf227f5617e593b268936f6b3` |
| Bundle RUN_ID | `REPRO_DEL0904_20260720T074714Z_a5235340aae3` |
| overall_status | **PASS** — evidence label `INTERNALLY_VERIFIED` (only); README states adoption basis and that NO owner acceptance occurred |
| Bundle | `validation/evidence/reproduction/REPRO_DEL0904_20260720T074714Z_a5235340aae3/` — 75 files; `SHA256SUMS.txt` verified over 74 entries (re-verified independently by the impl verifier: 0 missing, 0 extra) |
| Cases | all eight documented cases: frozen E1 `0/1/1` under CURRENT dated expectations (case 1 exit 0 `COMPLETED`, 830 `result_refs`, exactly one non-blocking `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` warning at `mechanics_envelope.diagnostics[4]` for `support:CE-120` — NOT diagnostic-clean, as documented; case 2 exit 1 `HEADLESS_RUNNER_LOAD_BASIS_MISSING`, no solver result; case 3 exit 1 `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` per the dated note) plus the five bound del1005 cases `0/0/0/1/1`, each output byte-identical to its committed witness |
| Predicates | P1–P16 all PASS (16/16); 17/17 commands matched expected exits; both generators byte-deterministic; offline local-only clone detached at `SOURCE_COMMIT`, clean before/after; §4.4 lockfile contingency NOT used (42 crates compiled from local cache); tmp root removed after hashing |
| Evidence commit | `4ff617ae1` (`evidence(piping): DEL-09-04 clean reproduction at R14 head (R14-W5)`, 85 files: brief, bundle, one sweep artifact, three DEL-09-04 state files, W5 chain artifacts) |
| Chain | manager-authored brief + D-54 rationale + dispatch transcript → fresh-context adversarial brief verifier **COMMIT-SAFE** (`instances/W5/VERIFY_BRIEF.md`; C1–C10; two Low defects cured in place pre-effect — prior-bundle-count and Rerun-Consequence quotation; three INFO recorded) → §10 EFFECTIVE → executor **PASS** (`instances/W5/EXECUTE_RETURN.md`; 7/7 executor checks incl. single evidence-sweep with proven one-file delta) → fresh-context implementation verifier **COMMIT-SAFE** (`instances/W5/VERIFY_IMPL.md`; V1–V10 independently recomputed; one Low digest-prefix typo in the run record cured pre-commit; two INFO) → manager commit |

Strike-no-rows honored: DEL-09-04 `## Remaining` byte-identical to base
(verified by executor and impl verifier against `git show HEAD`); both
owner-gated bullets untouched; lifecycle `IN_PROGRESS` unchanged;
progress recorded only in one `_STATUS.md` History entry, one `MEMORY.md`
entry, and
`_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W5_CLEAN_REPRO.md`.

## 2. Wave Check Tally

Executor-run (persisted in the bundle `checks/`): the eight-case
predicate validation, witness byte-comparison, evidence-sweep
(`SWEEP_20260720T075521Z_a5235340aae3-dirty.json`, exactly-one-file delta
proven by sweeps-before/after/delta), claims-language, path-anchors,
receipts consistency (untouched, cursor `Receipt-63`), `git diff
--check`, JSON/JSONL parse (15 files), and change-scope containment (80
paths, zero violations, exactly the seven brief §6 allowed entries) — all
PASS; independently re-run or re-derived by the implementation verifier.

Manager-run at wave closeout from `REPO_ROOT` via
`tools/software_workflow/run_registered_checks.py`, each as its own
halting step; normalized JSONs beside this return:

| Check | Result | JSON |
|---|---|---|
| `piping-pytest` | PASS (507 passed) | `instances/W5/CHECK_piping-pytest.json` |
| `harness-pytest` | FAIL then PASS on re-run after the §3 cure | `instances/W5/CHECK_harness-pytest_attempt1_FAIL.json`, `instances/W5/CHECK_harness-pytest.json` (311 passed) |
| `harness-self-check` | PASS | `instances/W5/CHECK_harness-self-check.json` |

Post-cure validator re-run (claims-language, path-anchors, receipt
validator, `git diff --check`): all PASS on the closeout tree.

## 3. Near-Miss Cure Record — Portability Finding After the Evidence Commit

First-run `harness-pytest` failed one test
(`test_live_gen8_semantic_portability_invariants`): the W5 implementation
verifier's `VERIFY_IMPL.md` line 199 contained a machine-absolute
temporary-root path (`/var/folders/…` prefix), an
`ABS_PATH_IN_UNCLASSIFIED_SURFACE` portability finding (SPEC-0.2.4). The
finding is fence-internal (a W5 chain artifact), narrative-only, and
post-dates the impl-verifier verdict it appears in. Cure: the
machine-absolute prefix was redacted in place with an explicit
manager-cure note (basename and meaning preserved); the failed check JSON
is preserved unsoftened as
`CHECK_harness-pytest_attempt1_FAIL.json`; the re-run passed (311
passed). Because the evidence commit `4ff617ae1` had already landed, the
cure and both check JSONs land in the closeout `chore` commit — recorded
here for HELP_HUMAN fan-in as a truthful post-commit delta to one chain
artifact. No bundle, witness, state, or brief byte changed in the cure.

## 4. Run-Record and Artifact Pointers

- Bundle: `validation/evidence/reproduction/REPRO_DEL0904_20260720T074714Z_a5235340aae3/`
- Sweep: `validation/evidence/sweeps/SWEEP_20260720T075521Z_a5235340aae3-dirty.json`
- DEL-09-04: `_STATUS.md` (History), `MEMORY.md`,
  `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W5_CLEAN_REPRO.md`
  (in the DEL-09-04 deliverable folder)
- Brief: `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_DEL-09-04_CLEAN_REPRO_R14.md`
- Chain: `instances/W5/` — `W5_DISPATCH_TRANSCRIPT.md`,
  `CURRENT_CANDIDATE_RATIONALE.md`, `VERIFY_BRIEF.md`,
  `EXECUTE_RETURN.md`, `VERIFY_IMPL.md`, the four `CHECK_*.json` files

## 5. Parked / Owner-Gated Rows (named, never executed)

- DEL-09-04: both Remaining bullets byte-identical (E2 residuals with
  owner-gated `MAINTAINER_REVIEWED` promotion + GUI-workflow evidence;
  DEC-046 tolerance promotion). Reproduction-result ACCEPTANCE remains an
  owner gate (D-46 protocol); this wave produced evidence only.
- Campaign-level parked cluster (unchanged): D-45; DEC-046
  tolerance/threshold promotions; DEL-04-04 friction path-history D-XX;
  PDU-035 REVIEW dispositions; DEL-04-04 G1/G2/G4+M2/M3 re-disposition;
  DEL-04-05 stage-gated/D-05b rows; PKG-09 human dispositions;
  `export-results` binding (DEL-10-05); W4-routed follow-ons (desktop
  flaky test, fallback-fixture text, `validation.rs` info-text).

## 6. Model Attribution

All W5 children (fresh-context brief verifier, executor, fresh-context
implementation verifier) and this manager ran on `claude-fable-5`
(Claude Fable 5) via the session harness Agent facility, no override or
mid-task substitution. Commits carry
`Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>`.

## 7. Enumerated Wave-Level Claims (for HELP_HUMAN fan-in refutation)

- W5-C1. Commit `4ff617ae1` exists on `claude/piping-r14-w5-clean-repro`
  on base `a5235340a` and contains only §5-fence paths (brief, one new
  bundle, one new sweep artifact, three DEL-09-04 files, `instances/W5/`
  chain artifacts); the closeout `chore` commit adds only the three
  closeout check JSONs, the preserved failed check JSON, the §3 one-line
  redaction in `VERIFY_IMPL.md`, and this return; the tree is clean after
  it.
- W5-C2. The tranche passed a fresh-context adversarial brief
  verification and a fresh-context implementation verification, both
  terminal COMMIT-SAFE, before the evidence commit; all verifier defects
  (two Low pre-effect, one Low pre-commit, one post-commit portability
  finding) are cured with the original findings preserved unsoftened.
- W5-C3. The bundle records all eight documented cases with the CURRENT
  manual expectations: E1 exits `0/1/1` with case-1 explicitly
  non-diagnostic-clean (one non-blocking constant-effort non-consumption
  warning) and case-3 on the post-#287 diagnostic; del1005 exits
  `0/0/0/1/1` with all five outputs byte-identical to their committed
  witnesses; expected witness mismatches for E1 cases 1 and 3 recorded
  per the dated notes; case-2 comparison recorded as observation.
- W5-C4. NO Remaining row anywhere was struck, edited, added, or
  reworded; DEL-09-04 lifecycle stays `IN_PROGRESS`; no acceptance,
  promotion, threshold/tolerance content, release/label/CI-gate effect,
  or normative content was created; the bundle is labeled
  `INTERNALLY_VERIFIED` only and its README states that no owner
  acceptance occurred.
- W5-C5. All frozen surfaces are byte-identical at the wave head: the
  seven tp_runner_015 surfaces, the eleven del1005 surfaces, the manual
  page, all prior reproduction bundles (incl. the immutable R11 bundle),
  prior sweeps, suite crates, `core/**`, `tests/**`, `tools/**`, and
  `fixtures/**` (verified by executor guards, containment, and the impl
  verifier's independent hash recomputation against the brief §3 pins).
- W5-C6. Execution was offline and local-only: mktemp clone from
  `REPO_ROOT` only, detached at `SOURCE_COMMIT`, `CARGO_NET_OFFLINE=true`
  + `--offline` on every cargo command, no install/fetch/network by any
  node, §4.4 contingency unused, temporary root removed after evidence
  hashing.
- W5-C7. Exactly one new sweep artifact exists for this wave
  (`SWEEP_20260720T075521Z_a5235340aae3-dirty.json`), proven as a
  one-file before/after delta; `loop/LOOP_RECEIPTS.md` is untouched
  (cursor `Receipt-63`); no push, pull, fetch, PR, merge, rebase, or
  receipt append was performed by W5 or any child; all work stayed inside
  `projects/chirality-piping/**`; `_DomainEngines/**`, app-dev, and PEC
  untouched.
- W5-C8. The R11 brief §8 rerun obligation is discharged by evidence at
  the R14 head (current-head R6-criterion reproduction restored); the R11
  brief and bundle are unedited; a future change to the procedure,
  runner, solver, fixtures, witnesses, or profile re-triggers per the new
  brief's §8.
- W5-C9. Every durable narrative artifact of this wave carries the
  standard claim fence sentence; no `PROVER_CORRELATED` or
  `ENGINEER_ACCEPTED` language exists anywhere in the wave's writes.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
