# W4 MANAGER RETURN — PKG-09 Evidence Wave (R14)

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W4
**Role:** WORKING_ITEMS (Agent 1, PKG-09 package manager), reporting to HELP_HUMAN
**Branch:** `claude/piping-r14-pkg09-evidence` (base `e315fb840`, post-wave-2 main / PR #293)
**Date:** 2026-07-20 (UTC)
**Status:** WAVE COMPLETE — two tranches LANDED; no push, PR, merge,
rebase, or receipt performed (HELP_HUMAN fan-in acts). One near-miss
deviation recorded for fan-in: the wave carries TWO new sweep artifacts
(first sweep FAIL on one flaky fence-external desktop test, preserved;
immediate single re-run PASS as the DEC-025 pre-push gate) — see §3.

## 1. Per-Tranche Dispositions

| Tranche | Scope | Disposition | Commit | Chain |
|---|---|---|---|---|
| T6 | DEL-09-01 §16.2 residual row, W3-bounded agent-lawful slice — benchmark-evidence-system construction (derivative bundle `validation/evidence/benchmarks/BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4/`: whole-suite head capture 24 cases = 11 `executed_and_matched` + 13 `blocked`, exit 1 recorded as regression evidence; 24-fixture family/provenance/redistribution index, every anchor existence-verified; verification refresh 38 suite tests vs the R5-era 21-family/33-test point; deterministic bundle-local assembly; SHA-256 manifest with derivative/non-authoritative label) | **LANDED** | `db9197a5d` | brief verifier COMMIT-SAFE (C1–C10; MEDIUM D1: no durable W4 dispatch artifact — cured by `instances/W4/W4_DISPATCH_TRANSCRIPT.md`) → executor fail-closed halt (10/11 checks PASS; the single FAIL was `validate_path_anchors` on the manager's own transcript artifact, OUT of the executor fence — correct §7 behavior, preserved unsoftened) → manager cure (placeholder redaction + note in the transcript; claims-language/path-anchors/diff-check re-run PASS) → impl verifier COMMIT-SAFE (11/11 §6 checks independently re-run; all 11 E-claims confirmed; blocked→cured chain verified a–d; 3 INFO notes, none load-bearing) |
| T7 | DEL-09-04 reproduction-manual stale-text refresh (W1-routed follow-on): dated 2026-07-20 case-1 historical note (post-`faee4faed` non-consumption warning `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` for `support:CE-120`; committed witness pinned-truthful, unedited), corrected case-1 expected-evidence cell, dated Part 2 currency note (24 mechanics fixtures; T6 bundle cited as committed witness evidence); three no-edit dispositions recorded (fallback fixture, `validation.rs` info-text, wind schema absent from page) | **LANDED** | `ab3286316` | brief verifier COMMIT-SAFE (C1–C10; Low D1 shorthand defect recorded in the brief status block, operative text controlling) → executor PASS (9/9 checks; mandatory live offline case-1 anchor run BEFORE editing: exit 0, `COMPLETED`, 830 result_refs, one envelope warning at `mechanics_envelope.diagnostics[4]`, stdout SHA `b3cd85af…8613` ≠ committed witness SHA `c406d9c2…5188`) → impl verifier COMMIT-SAFE (20/20 re-run incl. independent live solve rerun; 12/12 E-claims confirmed; 0 defects) |

No tranche was parked or referred; no D-49/D-52/D-54 limit was hit.
Every verifier/executor halt is preserved unsoftened in the tranche
instance directories (`instances/W4/T6/`, `instances/W4/T7/`) with its
curing record.

Dispatch-currency note: the W4 dispatch's "23 mechanics fixtures" figure
was W1-current; the live post-wave-2 suite carries 24 (the W2 T4
sub-span wind fixture is the 24th). Recorded in
`instances/W4/W4_DISPATCH_TRANSCRIPT.md` manager notes and reflected in
both tranches' artifacts.

## 2. Wave Check Tally (registered checks, run once for the branch)

Executed at wave closeout from `REPO_ROOT` via
`tools/software_workflow/run_registered_checks.py`, each as its own
halting step; normalized JSON persisted beside this return:

| Check | Result | JSON |
|---|---|---|
| `piping-pytest` | PASS (507 passed) | `instances/W4/CHECK_piping-pytest.json` |
| `evidence-sweep` (DEC-025 pre-push gate) | FAIL then PASS on single re-run (§3) | `instances/W4/CHECK_evidence-sweep_attempt1_FAIL.json`, `instances/W4/CHECK_evidence-sweep.json` |
| `harness-pytest` | PASS | `instances/W4/CHECK_harness-pytest.json` |
| `harness-self-check` | PASS | `instances/W4/CHECK_harness-self-check.json` |

Per-tranche checks (suite cargo test, live-run anchors, byte-identity
guards, claims-language, path-anchors, `git diff --check`, JSON/CSV
parse, containment with persisted JSON) were run by each executor and
independently re-run by each implementation verifier — tallies in the
per-tranche `EXECUTE_RETURN.md` and `VERIFY_IMPL.md` artifacts.

## 3. Sweep Deviation — Near-Miss Record for Fan-In

The controlling dispatch requires a single wave sweep producing exactly
one new `SWEEP_*.json`. The wave head carries TWO new artifacts, both at
commit `ab3286316`:

- `validation/evidence/sweeps/SWEEP_20260720T071112Z_ab32863165f8-dirty.json`
  — overall `fail`. Sole failure: `desktop_vitest` 475/476, one test
  (`src/App.test.tsx` "picks straight pipe endpoints from viewport node
  targets before apply", an `aria-pressed` interaction assertion) —
  fence-external to W4 (no W4 write touches `apps/desktop/**`, `core/**`,
  or `schemas/**`). Rust, python (507 passed), and wasm-build surfaces
  passed.
- `validation/evidence/sweeps/SWEEP_20260720T071331Z_ab32863165f8-dirty.json`
  — overall `pass` (the DEC-025 pre-push gate for this branch).

Flake diagnosis, recorded before the re-run: the identical desktop
source passed the full sweep ~90 minutes earlier at `27110b280`
(`instances/W2-XR/CHECK_evidence-sweep.json`, sweep exit 0), W4 changed
no desktop-relevant surface, and the focused re-run of the single test
passed immediately (1 passed / 57 skipped, 2.1 s). The failure shows
`act(...)` warnings and a timing-sensitive `aria-pressed` assertion
under full-suite load — verification-infrastructure flakiness, not a
product regression detected by this wave.

Disposition: both sweep artifacts and both check JSONs are preserved and
committed (deleting the failed record to satisfy the exactly-one rule
would destroy evidence; the truthful sequence is the smaller deviation).
The `-dirty` suffixes reflect the then-untracked closeout files
committed with this return. Follow-on routed to HELP_HUMAN: a bounded
DEL-07-xx/desktop test-hardening selection for the flaky interaction
test (deterministic state settling around the pick-endpoint toggle).

## 4. Run-Record Pointers

- DEL-09-01: `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W4_T6_BENCH_EVIDENCE.md`
  (in the DEL-09-01 deliverable folder)
- DEL-09-04: `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W4_T7_VALMANUAL_STALE_TEXT.md`
  (in the DEL-09-04 deliverable folder)
- Briefs: `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_T6_DEL-09-01_BENCH_EVIDENCE_SYSTEM.md`,
  `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_T7_DEL-09-04_VALMANUAL_STALE_TEXT.md`
- Chain artifacts: `instances/W4/T6/`, `instances/W4/T7/` (rationales,
  verifier returns, executor returns, containment JSONs), plus
  `instances/W4/W4_DISPATCH_TRANSCRIPT.md` (verbatim dispatch with one
  recorded path redaction).

## 5. Parked / Owner-Gated Rows (named, never executed)

- DEL-09-01: Remaining rows 1 (PDU-037 keep-open — the index is BUILT as
  evidence in the T6 bundle; the ROW stays open, closure judged at the
  owner's gate), 2 (PDU-013 unit-catalog hold, ENGINEERING), 3 (§16.2
  residual row itself — progress recorded in History/MEMORY only;
  completion judged at the owner's R5-exit gate), 4 (PDU-060
  `PKG09-0901-PKG02-001` human disposition). All four byte-identical.
- DEL-09-04: both Remaining bullets (E2 residuals with owner-gated
  MAINTAINER_REVIEWED promotion + GUI-workflow evidence; DEC-046
  tolerance promotion). Byte-identical.
- DEL-09-02 keep-TBD row, DEL-09-03 PDU-060 dispositions, DEL-09-05
  rows 9–11 (per W3): untouched, owner-gated.
- Campaign-level parked cluster (unchanged): D-45; DEC-046
  tolerance/threshold promotions; DEL-04-04 friction path-history D-XX;
  PDU-035 REVIEW dispositions; DEL-04-04 G1/G2/G4+M2/M3 re-disposition;
  DEL-04-05 stage-gated/D-05b rows; PKG-09 human dispositions.

## 6. Follow-Ons Reported for HELP_HUMAN Routing (not executed in W4)

1. **Desktop flaky-test hardening** (§3): the `App.test.tsx`
   pick-endpoint `aria-pressed` interaction test is timing-flaky under
   full-suite load; a bounded DEL-07-xx selection should make it
   deterministic. Until then the DEC-025 sweep carries a known
   intermittent fence-external failure mode.
2. **Fallback fixture stale text** (carried from W1 T2 VERIFY_IMPL D1,
   verified still present): `fixtures/product_preview/invented_mechanics_result.json`
   retains the superseded "no global constant-effort load…" review-row
   text; a future code/fixture-lane selection may refresh it. Not a
   docs surface; recorded disposition in the T7 run record.
3. **`validation.rs` info-text wording** (carried from W1 T2): the
   `CONSTANT_EFFORT_USER_DATA_REVIEWED` clause remains literally true;
   optional wording touch for a future code-lane selection.

## 7. Model Attribution

All W4 children (two fresh-context brief verifiers, two executors, two
fresh-context implementation verifiers) and this manager ran on
`claude-fable-5` (Claude Fable 5). Commits carry the
`Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` trailer.

## 8. Enumerated Wave-Level Claims (for HELP_HUMAN fan-in refutation)

- W4-C1. Commits `db9197a5d` (T6) and `ab3286316` (T7) exist on
  `claude/piping-r14-pkg09-evidence` in that order on base `e315fb840`,
  each containing only its tranche's §5-fence paths plus its chain
  artifacts (T6 additionally carries the manager's dispatch transcript,
  a lawful W4 coordination artifact); the working tree after the
  closeout commit is clean of all wave-fence paths.
- W4-C2. Each tranche passed a fresh-context adversarial brief
  verification and a fresh-context implementation verification, both
  terminal COMMIT-SAFE, before its commit; the T6 executor's fail-closed
  halt (out-of-fence path-anchor finding) and both brief-verifier
  defects (T6 D1 MEDIUM dispatch provenance; T7 D1 Low shorthand) are
  preserved unsoftened with their curing records.
- W4-C3. Three of the four registered checks pass first-run at the wave
  head; `evidence-sweep` failed once on the single §3-diagnosed flaky
  fence-external desktop test and passed on the single re-run; exactly
  two new sweep artifacts exist for this branch (fail then pass, both
  committed with this return); the passing sweep is the DEC-025
  pre-push gate. No commit of this branch is pushed by W4.
- W4-C4. NO Remaining row anywhere was struck, edited, added, or
  reworded: DEL-09-01's four rows and DEL-09-04's two bullets are
  byte-identical to base (verified per-tranche by executor and
  implementation verifier); progress is recorded in History/MEMORY/run
  records only.
- W4-C5. No lifecycle state changed (DEL-09-01 and DEL-09-04 remain
  `IN_PROGRESS`); no threshold, tolerance, acceptance criterion,
  verification→validation promotion, release/label/CI-gate effect, or
  normative content was created or promoted; PDU-013, PDU-037 (row),
  PDU-060, DEC-046, D-45, and every named hold are untouched.
- W4-C6. The T6 bundle is derivative, labeled non-authoritative,
  hash-manifested, deterministic (assembly rerun byte-identical,
  independently reproduced), and cites its accepted upstream basis; its
  whole-suite capture records head regression evidence (11 matched + 13
  blocked, exit 1) exactly as emitted, with no defect or release claim.
- W4-C7. All frozen surfaces are byte-identical at the wave head: the
  seven tp_runner_015 surfaces, the eleven del1005 surfaces, all
  reproduction bundles, prior sweeps, coverage, gates, release
  artifacts, suite crates, hand-calcs, and `fixtures/**` (verified by
  per-tranche guards and containment runs; the only `validation/**`
  additions are the T6 bundle and the two closeout sweep artifacts).
- W4-C8. The T7 page edits are exactly three surfaces (corrected case-1
  cell; two dated 2026-07-20 notes); every asserted value is anchored to
  a live offline run at the head, frozen input bytes, the committed T6
  bundle, crate assertions, or preserved W1/W2 chain evidence; the
  committed solve witness was not edited and is described as truthful
  for its pinned commits.
- W4-C9. No push, pull, fetch, PR, merge, rebase, receipt append, or
  network action was performed by W4 or any child; `loop/LOOP_RECEIPTS.md`
  is untouched (cursor Receipt-62); all work stayed inside
  `projects/chirality-piping/**`; `_DomainEngines/**`, app-dev, and PEC
  are untouched; the offline constraint held (no install/provisioning by
  any child; cargo/npm ran offline against local state).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
