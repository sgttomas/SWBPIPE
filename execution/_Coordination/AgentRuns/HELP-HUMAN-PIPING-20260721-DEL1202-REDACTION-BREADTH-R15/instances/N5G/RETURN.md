# N5G Return — Fresh attempt-7 terminal verification

**Verdict:** `BLOCK`

## Material finding

1. **Attempt 7 weakens the existing required-manifest-field gate for an empty
   `units_manifest`, allowing an invalid workflow payload to materialize.**
   `core/handoff/exporter/workflow.py:368-373` now considers every `Mapping`
   value present for `units_manifest`; the frozen implementation rejected all
   falsey required fields with `if not manifest.get(field)`. A fresh public-
   workflow probe set `units_manifest={}` and aligned
   `source_context.units_manifest_ref=None`. The result emitted no source
   diagnostic, returned `blocked=false` with a non-null payload, and invoked
   materialization. It therefore bypassed the already-declared
   `EXP-HANDOFF-MANIFEST-FIELD-MISSING` blocker. The new regression matrix at
   `tests/test_handoff_export_workflow.py:495-561` covers omission, null, and
   one non-Mapping value, but not the pre-existing empty-Mapping case; all 17
   focused tests pass while this regression remains. This violates the N4G
   no-unrelated-regression/preserve-prior-coverage boundary, candidate v6
   sections 3 and 5.1, and N5G's complete every-declared-source-blocker review.
   Remediation must preserve the new type guard while also treating an empty
   required units manifest as missing, with exact blocked/no-payload/no-
   materializer regression evidence, followed by a new immutable attempt and
   fresh verifier.

## Mandatory probe results

- Omitted, null, list, string, integer, and boolean values were probed for each
  of `target_mapping_contract`, `target_fixture`, and `units_manifest` (18
  cases). Every required attempt-7 case emitted exactly its existing blocker,
  returned blocked/no payload, made zero materializer calls, and retained
  sanitized decision/finding/summary evidence.
- All 15 declared workflow diagnostic codes were exercised through
  `build_handoff_export_workflow`. Ordinary blockers gated additively with no
  payload/materialization; both warnings remained nonblocking. A combined
  blocker-plus-warning retained two source findings, one source blocker, and
  one total exposure blocker.
- Lossless-only withholding returned `blocked=true`, no payload, zero
  materializer calls, one related finding, and internally consistent counts:
  source `0`, redaction `0`, lossless `1`, total exposure `1`, with
  `materialization_withheld=true`.
- Fresh focused checks passed: workflow Python `17 passed`; shared desktop
  projector/link/report Vitest `52 passed`. These suites do not cover the
  empty-Mapping regression above.

## Complete closure reconciliation

- Prior safe-key/public-basis, exact PCF/MBF authority, report-text inference,
  independent CAEPIPE CSV intent, sanitized decision/finding observability,
  native-package context, renderer iframe suppression, runner subprocess
  coverage, H4 pre-A3 flow, and source-diagnostic additive-gate closures remain
  present. The attempt-7 defect is a new narrowing regression in the same
  source-diagnostic gate.
- The route matrix has 31 unique RouteIDs; dispositions assign all 31 exactly
  once (`22/2/4/1/2`), and the 54 owner rows cover every route with 87 unique
  `(RouteID, Path)` associations and no duplicates.
- Candidate-scope validation passed the pre-return 260-path complete
  tracked/untracked inventory with zero violations. This reconciles N4G's
  final 257 paths plus `N4G_FAN_IN.md`, the N5G brief, and the mandatory N5G
  TASK record. There are zero staged or tracked/untracked/ignored
  `test-results/` paths; `git diff --check` passes.
- Exactly seven immutable sweep artifacts exist. Their hashes match the
  recorded chain through attempt 7
  (`6b6a99dfe79186a3b6d25b6fa192eba24485584bd82694db40782b6c89641ade`),
  all record passing surfaces, and only attempt 7 is presented as acceptance-
  eligible. No sweep was run by N5G.
- Protected-content/release files, DEL-12-02 state/memory/final run records,
  receipts, lifecycle state, branch, and HEAD remain unchanged from
  `0c066652cd527eb1559f715e914262d2bda42602`; the branch remains
  `codex/piping-pkg12-redaction`.

No repair, evidence sweep, state/lifecycle/receipt, release, or Git effect was
performed. W3 remains held.
