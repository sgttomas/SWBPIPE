# N5F Return — Fresh attempt-6 terminal verification

**Verdict:** `BLOCK`

## Material findings

1. **`REXC-CORE-001` still does not return a controlled block for every existing blocking source diagnostic.**
   `core/handoff/exporter/workflow.py:73-85` dereferences and materializes source
   inputs before `diagnostics_for_export_workflow` runs at line 113. Fresh
   adversarial calls reproduced:

   - `target_mapping_contract=None` raises `AttributeError` at line 73 before
     declared `EXP-TARGET-MAPPING-MISSING` can be emitted;
   - `target_fixture=None` raises `AttributeError` at line 74 (and would also
     fail at line 83) before declared `EXP-TARGET-FIXTURE-MISSING` can be
     emitted; and
   - a mapping-shaped handoff manifest with `units_manifest=None` reaches
     `workflow.py:407` and raises `AttributeError` before the already-declared
     `EXP-HANDOFF-MANIFEST-FIELD-MISSING` can be returned through the
     controlled gate.

   Each path performs zero route materialization/sink calls, but it returns an
   exception rather than the required `blocked=true`, `payload=None`, sanitized
   decisions/findings/summary envelope. This fails the N5F mandatory
   every-blocking-diagnostic probe and candidate §§3.5-3.6, 5.1, and acceptance
   predicates 2 and 5.

2. **The new “total exposure blocker” count omits the lossless-materialization blocker.**
   `core/security/redaction/route_control.py:117-121` correctly includes
   destructive lossless withholding in `blocked`, but lines 127-130 calculate
   `exposure_blocking_count` only as source blockers plus redaction
   `blocking_count`. A fresh lossless probe with one destructive redaction
   returned `blocked=true`, `payload=None`, and
   `materialization_withheld=true`, while `source_blocking_count=0`,
   `redaction_blocking_count=0`, and `exposure_blocking_count=0`. This conflicts
   with the attempt-6 requirement to compose source, redaction, and lossless
   blockers and with N4F's claim that the summary reports a total
   exposure-blocker count.

3. **The restored workflow tests do not cover the mandatory complete source-diagnostic gate.**
   `tests/test_handoff_export_workflow.py:452-534` now gives strong exact
   blocked/no-payload/no-materializer evidence for
   `EXP-UNIT-METADATA-MISSING` and blocked/payload/count evidence for the two
   mismatch diagnostics and authority-term diagnostics. All seven focused
   tests pass. They do not exercise `EXP-TARGET-MAPPING-MISSING`,
   `EXP-TARGET-FIXTURE-MISSING`, the null manifest-field failure above, or the
   lossless total-count inconsistency, so the terminal suite cannot detect the
   two product defects.

## Independently reconciled evidence

- For every reachable ordinary blocking diagnostic probed — handoff invalid,
  manifest field missing, mapping invalid/context missing, model-hash and unit
  mismatch, unsupported target kind, prohibited authority term, missing or
  blocked privacy, and `EXP-UNIT-METADATA-MISSING` — the attempt-6 gate did
  return `blocked=true`, `payload=None`, zero materializer calls, sanitized
  decisions/findings, no raw diagnostic code/severity, and distinct source vs
  redaction counts.
- Both existing warning diagnostics (`EXP-TARGET-KIND-MISMATCH` and
  `EXP-UNSUPPORTED-RECORDS-MISSING`) remained nonblocking, materialized a
  payload, and reported one source finding with zero source/exposure blockers.
- The A3-added `rendered-report-render` click and its post-click-only
  assertions are absent from the controlled-export sequence at
  `apps/desktop/e2e/r2-smoke.spec.ts:537-573`. The only remaining render click
  is the separate pre-existing from-blank flow at line 908. No replacement
  action was added to the repaired sequence.
- The route matrix contains 31 unique RouteIDs. The five disposition rows
  assign all 31 exactly once (22 controlled sinks, 2 caller traces, 4
  no-egress, 1 existing control, 2 separate-governance), and the 54 owner-map
  rows cover every RouteID with no duplicate `(RouteID, Path)` association.
- Scope validation against the adopted candidate fence passed. Before this
  return, the inventory was 238 dirty paths: N4F's final 235, its manager
  fan-in, the N5F brief, and the mandatory N5F TASK record. There are zero
  staged paths and zero ignored or dirty `test-results/` files.
- Exactly six attempt artifacts exist and their SHA-256 values match the
  recorded chain: attempts 1-5 remain `10fbc3c4...`, `d1620f2f...`,
  `b2e89383...`, `67fe4d20...`, and `3838165e...`; attempt 6 is
  `d2e4a79447a9cd04c6ae03061be6c291dd1864a7c240a15af8210ab8a5c208c5`.
  Attempt 6 is the sole artifact presented as acceptance-eligible, but this
  N5F `BLOCK` rejects it for W3 fan-in. No sweep was run by N5F.
- N4F's failed intermediate H4 evidence remains distinguishable from its
  terminal passing H4 evidence. Registered terminal evidence reports 523
  piping tests, 492 desktop tests, desktop build, source/dist H4, 44 headless
  Rust tests, 311 harness tests, self-check, validators, and the sole attempt-6
  sweep passing; the fresh adversarial probes above expose cases absent from
  that suite.
- Protected-content/release paths, DEL-12-02 state/memory/final run record,
  receipts, lifecycle state, branch, and HEAD remain unchanged from
  `0c066652cd527eb1559f715e914262d2bda42602`; `git diff --check` passes.

No repair, evidence sweep, state/lifecycle/receipt, release, or Git effect was
performed. W3 remains held. Remediation requires a new bounded implementation
attempt followed by a fresh verifier.
