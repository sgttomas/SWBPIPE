# N5H Return — Fresh attempt-8 terminal verification

**Verdict:** `COMMIT-SAFE`

## Mandatory attempt-8 probe

- A fresh public-workflow probe set `units_manifest={}` and the copied
  `source_context.units_manifest_ref=None`. It emitted exactly
  `EXP-HANDOFF-MANIFEST-FIELD-MISSING`, returned `blocked=true` with no
  payload, and made zero materializer calls.
- The controlled result retained sanitized decisions/findings without the raw
  diagnostic code and reported truthful source/source-blocking/redaction/
  lossless/exposure counts of `1/1/0/0/1`.
- The exact regression is present in
  `tests/test_handoff_export_workflow.py`; the retained N4G non-Mapping guard
  and valid non-empty Mapping behavior remain intact.

## Complete adversarial reconciliation

- Reprobed all 18 N5G invalid-input variants: omitted, null, list, string,
  integer, and boolean values for each mapping contract, target fixture, and
  units manifest. Every case emitted exactly its existing blocker, returned
  blocked/no payload, made zero materializer calls, and retained sanitized
  evidence with internally consistent counts.
- Exercised all 15 declared workflow diagnostic codes through the public
  workflow. Ordinary blockers gated additively; both warnings remained
  nonblocking. A combined blocker-plus-warning reported two source findings,
  one source blocker, and one total exposure blocker. Lossless-only
  withholding reported `0/0/1/1`, blocked with no payload, and set
  `materialization_withheld=true`.
- Fresh focused checks passed: workflow/redaction/runner Python `40 passed`;
  shared desktop projector/link/report Vitest `52 passed`. Source and test
  review reconfirmed all N5–N5G closures: native downstream context, no broad
  public/safe-key or value-text inference, exact record-local and PCF/MBF
  authority, unknown-before-private Rust handling, renderer iframe/save/print
  suppression, subprocess runner coverage, independent CAEPIPE CSV intent,
  sanitized per-decision/finding observability, and the pre-A3 H4 flow.
- Amendments A1–A3 reconcile to the final tests: A1 again asserts exact
  source-block/no-payload/no-materializer behavior; A2 remains expectation-
  only; the prohibited A3 render click and post-click-only assertions remain
  absent.

## Routes, evidence, and containment

- `ROUTE_MATRIX.csv` contains 31 unique RouteIDs. The five disposition rows
  assign all 31 exactly once (`22/2/4/1/2`); 54 owner rows provide 87 unique
  `(RouteID, Path)` associations with no missing route or duplicate.
- All eight immutable sweep artifacts exist, report `pass`, and match the
  recorded SHA-256 chain. Attempts 1–7 remain superseded/non-acceptance;
  attempt 8 is the sole acceptance-eligible artifact, SHA-256
  `3b08deb6b9a87dbeb3c7715266a072a3d1a3ebb97898dab0ff4556922013076c`.
  N5H ran no sweep. The N4B H4 historical artifact also remains byte-identical
  at its authorized hash.
- N4H's failed focused, H4, and harness intermediates remain preserved and
  distinguishable from terminal passing evidence. Its terminal records cover
  focused `18`, piping `534`, desktop `492`, build, H4 source/dist, Rust `44`,
  harness `311`, self-check, validators, and the sole attempt-8 sweep.
- Candidate-v6 containment plus the separately authorized, hash-bound
  `validation/portability_policy.json` repair passed for all 284 pre-N5H-
  record tracked/untracked paths: N4H's final 282 plus `N4H_FAN_IN.md` and the
  runtime-owned N5H brief. Final verifier containment is 286 paths after this
  return and TASK record, with zero violations, zero staged paths, zero dirty
  or ignored `test-results/` paths, and a clean `git diff --check`.
- The underlying redaction contract and parity corpus, protected-content and
  release tools, DEL-12-02 `_STATUS.md`, `MEMORY.md`, final deliverable run
  records, loop receipts, lifecycle state, branch, and HEAD remain unchanged
  from `0c066652cd527eb1559f715e914262d2bda42602`. The branch remains
  `codex/piping-pkg12-redaction`.

No repair, evidence sweep, deliverable-state/lifecycle/receipt/release, or Git
effect was performed. `COMMIT-SAFE` releases only DEL-12-02 W3 closeout; it
does not authorize merge, lifecycle transition, release, issuance,
professional acceptance, or security/legal sufficiency.
