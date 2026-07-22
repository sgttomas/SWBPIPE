# N4F Return — Exact N5E remediation attempt 6

**Status:** `SUCCESS`  
**Blockers:** none within N4F  
**Fresh verifier required:** yes (`N5F`)  
**State/receipt/lifecycle/release/Git effect:** none

## Implemented behavior

1. `REXC-CORE-001` now passes its source workflow diagnostics to the route
   controller as a distinct input. The controller evaluates blocking severity
   before sanitization and composes source blockers additively with redaction
   and lossless-materialization blockers.
2. A source blocker now returns `blocked=true`, `payload=None`, and never
   calls the materializer. Redaction decisions/findings remain observable, and
   the summary reports distinct source, redaction, and total exposure-blocker
   counts without exposing raw diagnostic code or severity values.
3. Exact workflow tests restore and strengthen blocking, payload withholding,
   no-materialization, sanitized decision/finding visibility, and summary
   assertions for `EXP-UNIT-METADATA-MISSING`; the other blocking diagnostic
   cases also assert payload withholding and additive counts.
4. The attempt-5-added `rendered-report-render` click and its post-click-only
   assertions were removed from the H4 controlled-export sequence. The
   pre-A3 interaction flow and the expectation-only redacted-leaf assertions
   reachable in that flow remain.

N4F changed exactly these product/test paths:

- `projects/chirality-piping/core/handoff/exporter/workflow.py`
- `projects/chirality-piping/core/security/redaction/route_control.py`
- `projects/chirality-piping/tests/test_handoff_export_workflow.py`
- `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts`

## Verification

- focused workflow: `7 passed`;
- registered full piping pytest terminal: `523 passed`;
- registered desktop Vitest: `24 files / 492 passed`;
- registered desktop production build: `PASS`;
- H4 source terminal: `2 passed`; H4 production-dist: `1 passed`, both with
  externally directed output;
- headless Rust crate: `44 passed`;
- practitioner harness: `311 passed`; self-check: `PASS`;
- claims-language: `PASS`, `264` files;
- path anchors: `PASS`, `855` live surfaces and zero findings;
- receipt validator: `PASS`, frozen through `Receipt-44`;
- dirty JSON/JSONL parse: `PASS`, `83` JSON files, `1` JSONL file, `46`
  JSONL records;
- candidate containment and `git diff --check`: `PASS`.

Failed intermediate evidence is preserved honestly:

- `CHECK_h4_source_initial.json`: `0 passed / 2 failed` after removing the
  prohibited click while its A3-dependent assertions remained. Those
  post-click-only assertions were removed; terminal H4 source then passed.
- The first read-only dirty-JSON command had a shell-quoting `SyntaxError`.
  The corrected invocation parsed every dirty JSON/JSONL file successfully;
  both outcomes are recorded in `CHECK_validators_pre_sweep.json`.

## Attempt-6 sweep

WORKING_ITEMS acknowledged the terminal pre-sweep PASS, complete dirty-path
count, zero `test-results/`, exact prior hashes, zero attempt-6 count, and the
exact registered command. N4F then invoked that command exactly once and never
reran it.

- registered result: `PASS`, exit `0`, duration `230.089s`;
- artifact:
  `projects/chirality-piping/validation/evidence/sweeps/SWEEP_20260722T091524Z_0c066652cd52-dirty.json`;
- artifact status/duration: `pass`, `227.172s`;
- SHA-256:
  `d2e4a79447a9cd04c6ae03061be6c291dd1864a7c240a15af8210ab8a5c208c5`;
- sweep capture: `79` dirty paths and zero `test-results/` paths;
- disposition: attempt 6 is acceptance-eligible only for fresh N5F
  verification. N4F makes no acceptance or closeout claim.

Attempts 1–5 remain byte-identical:

- attempt 1: `10fbc3c4e54b69df2856cb5dd42240dc87b35d4c3762df9664e6036cf7b3cd63`;
- attempt 2: `d1620f2f25264400be6a68e6931982c13b7c17fe52a14381fe70c673f045ca72`;
- attempt 3: `b2e89383c1e8a61ebfea8d52809358be53d23c56160315497f17d1d01844f4b6`;
- attempt 4: `67fe4d2042469ba2ec2950c717b823bb4ad2a6ad66324889a13e6e354be2a29d`;
- attempt 5: `3838165eae833faf04a5b2612f42e102224911582580bcab06bef4aa5757a12b`.

The sweep generated ignored
`projects/chirality-piping/apps/desktop/test-results/.last-run.json`; its
content reported `passed`. N4F deleted only that known generated file.

## Containment and residual boundary

The terminal pre-sweep repository inventory contained `232` complete
tracked/untracked dirty paths with zero candidate-fence violations and zero
`test-results/` files. The final inventory after the sole sweep, its evidence,
and this return contains `235` paths, with zero violations and zero
`test-results/` files. No path is staged.

Protected-content/release exporters, DEL-12-02 `_STATUS.md`, `MEMORY.md`, final
deliverable run records, loop receipts, lifecycle state, and Git state remain
unchanged from frozen base `0c066652cd527eb1559f715e914262d2bda42602`.
The branch remains `codex/piping-pkg12-redaction` and HEAD remains the frozen
base.

The pre-existing native-GUI automation limitation remains: mocked-Tauri tests
cover pre-IPC/native behavior and H4 covers browser-visible controls and
downloads; no real native save/print automation claim is made. Fresh N5F
verification is mandatory before any state, receipt, lifecycle, release, or
Git effect.
