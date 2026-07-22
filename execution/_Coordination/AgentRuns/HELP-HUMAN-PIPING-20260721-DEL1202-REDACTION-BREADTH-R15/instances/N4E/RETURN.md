# N4E Return — Exact N5D remediation attempt 5

**Status:** `SUCCESS`  
**Blockers:** none within N4E  
**Fresh verifier required:** yes (`N5E`)  
**State/receipt/lifecycle/release/Git effect:** none

## Implemented behavior

1. Python and TypeScript no longer treat broad safe-looking key names, tokens,
   suffixes, units, dimensions, diagnostics, or professional-language fields as
   public authority. Unmetadataed scalars under opaque objects and collections
   remain unknown outside local-private use. Explicit leaf metadata,
   complete exact-record-local metadata, and the route/document/deliverable/root
   exact PCF/MBF structural basis remain the only authorized public paths.
2. The CAEPIPE Parser CSV member now has its own adjacent explicit
   local-private intent. Harness JSON intent cannot authorize it. Before CSV
   intent there is no href, handoff, write side effect, or raw CSV exposure;
   after CSV intent the exact payload is materialized with warning/decision
   evidence and without mutating the source.
3. Controlled export UI now renders sanitized per-decision and per-finding
   evidence before an enabled link: path, classification/action, reason, and
   severity are inspectable without rendering raw private values. Blocked-link
   and accessible-name behavior is preserved.
4. Versioned amendments A1–A3 changed expectations/fixtures only for the
   direct full-suite and H4 consequences. They did not weaken product behavior,
   widen public authority, or alter unrelated coverage.

Product/test paths changed by N4E are exactly:

- `projects/chirality-piping/core/security/redaction/route_control.py`
- `projects/chirality-piping/tests/security/test_redaction_export_controls.py`
- `projects/chirality-piping/tests/test_handoff_export_workflow.py`
- `projects/chirality-piping/apps/desktop/src/features/redaction-controls/redactionExportControls.ts`
- `projects/chirality-piping/apps/desktop/src/features/redaction-controls/redactionExportControls.test.ts`
- `projects/chirality-piping/apps/desktop/src/features/redaction-controls/ControlledExportLink.tsx`
- `projects/chirality-piping/apps/desktop/src/features/redaction-controls/ControlledExportLink.test.tsx`
- `projects/chirality-piping/apps/desktop/src/features/local-fea-handoff/LocalFeaHandoffPanel.test.tsx`
- `projects/chirality-piping/apps/desktop/src/App.test.tsx`
- `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts`

The authorized CAEPIPE panel path did not require an edit.

## Verification

- focused Python redaction controls: `14 passed`;
- focused TypeScript redaction/control-link then App/combined lanes: `42`,
  `58`, and `100` passed respectively;
- amended Python security plus handoff lane: `21 passed`;
- amended local-FEA plus controls lane: `45 passed`;
- registered full piping pytest terminal: `523 passed`;
- registered full desktop Vitest terminal: `24 files / 492 passed`;
- registered desktop production build: `PASS`;
- H4 source terminal: `2 passed`; H4 production-dist: `1 passed`, both with
  externally directed output;
- headless Rust crate: `44 passed`;
- practitioner harness terminal: `311 passed`; self-check: `PASS`;
- final claims-language: `PASS`, `264` files;
- final path anchors: `PASS`, `835` live surfaces and zero findings;
- receipt validator: `PASS`, frozen through `Receipt-44`;
- final dirty JSON/JSONL parse: `PASS`, `70` JSON files, `1` JSONL file,
  `41` JSONL records;
- complete containment and `git diff --check`: `PASS`.

Failed intermediate evidence remains recorded honestly:

- `CHECK_piping-pytest.json`: `516 passed / 7 failed` before A1 expectation
  alignment; terminal result is `CHECK_piping-pytest_final.json`.
- `CHECK_desktop-test.json`: `490 passed / 2 failed` before A2 expectation
  alignment; terminal result is `CHECK_desktop-test_final.json`.
- `CHECK_h4_source.json`: `0 passed / 2 failed` before A3 adjacent H4 payload
  expectation alignment; terminal result is `CHECK_h4_source_final.json`.
- `CHECK_harness-pytest.json`: `310 passed / 1 failed` because three new H4
  evidence records contained literal external temporary paths. Only those
  evidence path strings were changed to `{EXTERNAL_TMP}`; terminal
  `CHECK_harness-pytest_final.json` passed `311` tests.

## Attempt-5 sweep

Exactly one late attempt-5 DEC-025 sweep was invoked after WORKING_ITEMS
acknowledged the terminal pre-sweep PASS checkpoint:

- registered result: `PASS`, exit `0`, duration `237.191s`;
- artifact:
  `projects/chirality-piping/validation/evidence/sweeps/SWEEP_20260722T083801Z_0c066652cd52-dirty.json`;
- artifact status/duration: `pass`, `234.123s`;
- SHA-256:
  `3838165eae833faf04a5b2612f42e102224911582580bcab06bef4aa5757a12b`;
- sweep capture: `78` dirty paths and zero `test-results/` paths;
- disposition: attempt 5 is acceptance-eligible only for fresh N5E
  verification. N4E makes no acceptance or closeout claim.

Attempts 1–4 remain byte-identical and are superseded/non-acceptance:

- attempt 1:
  `10fbc3c4e54b69df2856cb5dd42240dc87b35d4c3762df9664e6036cf7b3cd63`;
- attempt 2:
  `d1620f2f25264400be6a68e6931982c13b7c17fe52a14381fe70c673f045ca72`;
- attempt 3:
  `b2e89383c1e8a61ebfea8d52809358be53d23c56160315497f17d1d01844f4b6`;
- attempt 4:
  `67fe4d2042469ba2ec2950c717b823bb4ad2a6ad66324889a13e6e354be2a29d`.

The sweep generated ignored
`projects/chirality-piping/apps/desktop/test-results/.last-run.json` after its
capture. Its content reported `passed`; N4E deleted only that generated file.
Final repository `test-results/` file and dirty-path counts are zero.

## Containment and residual boundary

`CHECK_change-scope_pre_sweep.json` records `203` complete tracked/untracked
dirty paths, zero fence violations, and zero `test-results/` paths before the
sweep. `CHECK_change-scope_final.json` records `210` paths, zero violations,
and zero `test-results/` paths after the sweep and generated-output cleanup;
the difference consists only of authorized N4E evidence and the one attempt-5
sweep artifact. Protected-content/release exporters, DEL-12-02 `_STATUS.md`,
`MEMORY.md`, final deliverable run records, loop receipts, lifecycle state, and
Git state remain unchanged from frozen base
`0c066652cd527eb1559f715e914262d2bda42602`. The branch remains
`codex/piping-pkg12-redaction`, HEAD remains the frozen base, and no path is
staged.

The pre-existing native-GUI automation limitation remains: mocked-Tauri tests
cover pre-IPC/native behavior and H4 covers browser-visible controls and
downloads; no real native save/print automation claim is made. Fresh N5E
verification is mandatory before any state, receipt, lifecycle, release, or
Git effect.
