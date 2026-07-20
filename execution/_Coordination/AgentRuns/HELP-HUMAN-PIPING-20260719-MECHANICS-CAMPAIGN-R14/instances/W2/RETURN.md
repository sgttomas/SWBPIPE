# W2 MANAGER RETURN — PKG-05 Loads Wave (R14)

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W2
**Role:** WORKING_ITEMS (Agent 1, PKG-05 package manager), reporting to HELP_HUMAN
**Branch:** `claude/piping-r14-pkg05-loads` (base `581a15b1c`, post-wave-1 main / PR #292 / Receipt-61)
**Date:** 2026-07-20 (UTC)
**Status:** WAVE COMPLETE WITH ONE BLOCKER FOR FAN-IN — T4 LANDED with its full
COMMIT-SAFE chain and all per-tranche checks passing; the wave-level DEC-025
evidence sweep FAILS on exactly one fence-external PKG-07 desktop test
(diagnosed below), so the branch is NOT PUSHABLE until HELP_HUMAN routes the
bounded DEL-07-02 cure. No push, PR, merge, rebase, or receipt was performed
(HELP_HUMAN fan-in acts).

## 1. Per-Tranche Dispositions

| Tranche | Scope | Disposition | Commit | Chain |
|---|---|---|---|---|
| T4 | DEL-05-01 sole Remaining — sub-span (partial-extent) wind exposure in occasional-load generation | **LANDED** | `a854d43a1` | brief v2 (v1 verifier BLOCK: the "additive only / no assertion weakened" test-edit constraint was unsatisfiable with the live `tests/test_model_schema.py` wind required-set assertion; cured by a recorded amendment authorizing ONE named equal-or-stronger replacement) → verifier v2 COMMIT-SAFE (16/16 claims; independent ten-class re-screen; lever-rule statics independently re-derived) → executor PASS (11/11 predicates; 17-step check sequence) → impl verifier COMMIT-SAFE (all checks independently re-run with matching counts; physics recomputed; containment independently re-swept) |
| T5 | DEL-05-04 conditional row — PDU-037 runtime stale-hash acceptance-reuse negative "when an owning acceptance runtime exists" | **CANNOT_ADVANCE — CONDITION_NOT_MET** | none (no execution, no brief) | condition verified against the live tree at `581a15b1c`: no runtime stores, loads, reuses, or invalidates human acceptance records (persistence envelope emits a literal empty `human_acceptance_refs`; desktop store has no acceptance surface; every `HUMAN_APPROVED_FOR_PROJECT` code site is a negative declaration; release gates do not yet exist per the DEL-09-05 emitter and the PDU-037/DEL-05-04-REQ-014 concordance rows). Evidence: `instances/W2/T5/CONDITION_VERIFICATION.md`. The Remaining row stays open unchanged and self-arms when an owning runtime lands |

Verified-parked rows (no Remaining work executable):

- **DEL-05-02** — sole Remaining row gated on D-45; register row confirmed
  `AWAITING_RULING` (packet `D-45_temperature_indexed_shear_modulus.md`).
  PARKED; untouched.
- **DEL-05-03, DEL-05-05** — `## Remaining` sections confirmed empty; no
  work exists to select.

Every verifier/executor BLOCK is preserved unsoftened in
`instances/W2/T4/` with its curing amendment recorded in the brief.

## 2. Wave Check Tally (registered checks, run once for the branch)

Executed at wave closeout from `REPO_ROOT` via
`tools/software_workflow/run_registered_checks.py`, each as its own halting
step; normalized JSON persisted beside this return:

| Check | Result | JSON |
|---|---|---|
| `piping-pytest` | PASS | `instances/W2/CHECK_piping-pytest.json` |
| `evidence-sweep` (DEC-025 pre-push gate) | **FAIL** (see §3) | `instances/W2/CHECK_evidence-sweep.json` |
| `harness-pytest` | PASS | `instances/W2/CHECK_harness-pytest.json` |
| `harness-self-check` | PASS | `instances/W2/CHECK_harness-self-check.json` |

The single evidence sweep produced exactly one new artifact
(`validation/evidence/sweeps/SWEEP_20260720T054243Z_a854d43a15d1-dirty.json`;
sweep inventory 287→288, delta exactly one file; the `-dirty` suffix
reflects the then-untracked closeout files committed with this return).
Within the sweep, the rust, python (507 passed), and wasm-build surfaces
passed; the `desktop_vitest` surface failed 1 of 476 tests (475 passed).
Per-tranche checks (cargo fmt/test per touched crate, read-only
regressions, focused pytest, contract guard, del1005 five-case
byte-identity, claims-language, path-anchors, `git diff --check`, JSON
parse, change-scope containment with persisted JSON) were run by the T4
executor and independently re-run by the T4 implementation verifier with
matching counts — tallies in `instances/W2/T4/EXECUTE_RETURN.md` and
`instances/W2/T4/VERIFY_IMPL.md`.

## 3. The DEC-025 Gate Blocker — Diagnosis and Routed Cure (for HELP_HUMAN)

- **Failing test (the only one):**
  `apps/desktop/src/features/model-tree/schemaSlotEmission.test.tsx`,
  case "covers the canonical seismic/wind generation parameter vocabulary
  one-to-one", line 381:
  `expect(wind.required).toEqual(["pressure", "shape_factor", "direction", "exposed_element_refs"])`.
- **Cause:** the governed T4 schema change relaxed
  `WindEquivalentStaticInput`'s required set (the `anyOf`
  at-least-one-marking-form rule) so `exposed_element_refs` is no longer
  unconditionally required. This desktop test is the PKG-07/DEL-07-02
  vocabulary-tracking mirror of exactly the assertion class the T4 v1
  brief verifier caught on the piping test surface (where the brief's v2
  amendment authorized the one equal-or-stronger replacement). The
  desktop surface runs only inside the DEC-025 sweep's `desktop_vitest`
  step, not in any per-tranche or piping-pytest check, so it surfaced at
  wave closeout.
- **Why not cured in W2:** `apps/desktop/**` is outside the T4 §5 fence
  and outside this manager's PKG-05 queue mandate; the W1 precedent for a
  fence-external consequence (the T1 DEL-08-04 vocabulary gap) is to
  report and route, not to fix cross-package from inside the wave. The
  T4 brief §7 directs exactly this disposition.
- **Bounded cure for routing:** a DEL-07-02 (PKG-07) selection updating
  the one desktop assertion to the same equal-or-stronger form the piping
  test now carries (three-key required subset + the exact `anyOf` shape +
  `exposed_spans` shape/minItems + `additionalProperties: false`),
  naturally combined with the already-reported GUI-emit follow-on (§5
  item 1). Until it lands and the sweep is re-run clean, no commit of
  this branch should be pushed (the sweep is the pre-push gate; W1
  refinement, applied wave-wide).
- The failure is a verification-coverage drift on a derivative GUI
  surface, not a product-solve defect: the canonical schema change it
  detects is exactly the governed, twice-verified T4 change.

## 4. Run-Record Pointers

- DEL-05-01: `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W2_T4_SUBSPAN_WIND.md`
  (in the DEL-05-01 deliverable folder)
- T5 verification: `instances/W2/T5/CONDITION_VERIFICATION.md`
- Brief: `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_T4_PKG05_SUBSPAN_WIND.md`
- Chain artifacts: `instances/W2/T4/` (rationale with v2 amendment record,
  VERIFY_BRIEF.md v1 BLOCK preserved, VERIFY_BRIEF_V2.md COMMIT-SAFE,
  EXECUTE_RETURN.md, VERIFY_IMPL.md, CHANGE_SCOPE_CONTAINMENT.json).

## 5. Follow-Ons Reported for HELP_HUMAN Routing (not executed in W2)

1. **DEL-07-02 sub-span GUI surface** (from T4, two connected parts):
   (a) the §3 blocker — the one-line schema-vocabulary assertion update
   in `schemaSlotEmission.test.tsx` (required to clear the DEC-025
   pre-push gate); (b) GUI/operation-applier emission of sub-span
   exposure marking (`exposed_spans`), the natural extension of the
   whole-span TP-PMM-GUIEMIT-001 emit surface.
2. **DEL-05-04 conditional row remains armed** (from T5): the runtime
   stale-hash acceptance-reuse negative becomes selectable only when a
   future lawful selection lands an owning acceptance runtime
   (acceptance-record storage/reuse or actual release gates); named for
   the final slate, not executable now.
3. **Validation-manual currency note** (low priority): the whole-span
   occloadgen manual case
   (`docs/validation_manual/cases/mechanics/mech-tp-pmm-p3-occloadgen-equivalent-static.md`)
   remains truthful for whole-span behavior (unchanged by T4); a
   docs-lane touch adding the sub-span case is optional future work, not
   a staleness defect.

## 6. Parked Owner-Gated Rows (named, never executed)

- DEL-05-02 sole row: temperature-indexed shear modulus (D-45
  `AWAITING_RULING`).
- Campaign-level parked cluster (unchanged, per the campaign plan): D-45;
  DEC-046 tolerance/threshold promotions; DEL-04-04 friction path-history
  D-XX; PDU-035 REVIEW dispositions; DEL-04-04 G1/G2/G4+M2/M3
  re-disposition; DEL-04-05 stage-gated/D-05b rows; PKG-09 human
  dispositions.

## 7. Model Attribution

All W2 children (two fresh-context brief verifiers, one executor, one
implementation verifier) and this manager ran on `claude-fable-5`
(Claude Fable 5). Commits carry the
`Co-Authored-By: Claude Fable 5 <noreply@anthropic.com>` trailer.

## 8. Enumerated Wave-Level Claims (for HELP_HUMAN fan-in refutation)

- W2-C1. Commit `a854d43a1` exists on `claude/piping-r14-pkg05-loads` on
  base `581a15b1c`, containing only T4 §5-fence paths plus the T4 chain
  artifacts, and the working tree after the closeout commit is clean of
  all wave-fence paths.
- W2-C2. T4 passed a fresh-context adversarial brief verification (v2)
  and a fresh-context implementation verification, both terminal
  COMMIT-SAFE, before its commit; the v1 BLOCK is preserved unsoftened
  with its curing amendment recorded in the brief and rationale.
- W2-C3. Three of the four registered checks pass at the wave head;
  `evidence-sweep` fails solely on the single §3-diagnosed desktop test
  (475/476 desktop tests pass; rust/python/wasm sweep surfaces pass), and
  exactly one new sweep artifact exists for this branch, committed with
  this return. No commit of this branch is pushed (the sweep remains the
  unpassed pre-push gate).
- W2-C4. The struck Remaining rows are exactly: DEL-05-01 (sole item).
  DEL-05-02's gated row, DEL-05-04's conditional row, and every other
  Remaining row in the project are byte-unchanged; no new Remaining rows
  were added anywhere.
- W2-C5. T5 executed no writes outside `instances/W2/T5/`; its
  cannot-advance verdict cites only opened live-tree artifacts; DEL-05-04
  `_STATUS.md`, `MEMORY.md`, and run records are untouched by W2.
- W2-C6. No lifecycle state changed (all five PKG-05 deliverables remain
  in their prior states); no threshold, tolerance, acceptance criterion,
  or normative content was created or promoted; D-45, DEC-046, D-12,
  PDU-035, and every named hold are untouched.
- W2-C7. Whole-span wind generation behavior, ids, and diagnostics are
  preserved; the five committed `del1005_payload_binding_*` witnesses are
  byte-identical at the wave head (verified independently by executor and
  implementation verifier); no `validation/witness/**` file changed.
- W2-C8. No push, pull, fetch, PR, merge, rebase, receipt append, or
  network action was performed by W2 or any child; `loop/LOOP_RECEIPTS.md`
  is untouched; all work stayed inside `projects/chirality-piping/**`;
  `_DomainEngines/**`, app-dev, and PEC are untouched; the offline
  constraint held (no install/provisioning by any child).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
