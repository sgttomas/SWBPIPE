# N4D Return — Exact N5C remediation attempt 4

**Status:** `SUCCESS`  
**Blockers:** none within N4D  
**Fresh verifier required:** yes  
**W3/state/receipt/Git:** held and unchanged

## Implemented behavior

1. Python and TypeScript public basis is now exact-record-local. A record's
   directly owned scalar leaves may use that record's own complete privacy and
   redistribution metadata; nested records and collections never inherit it
   and must declare their own basis.
2. TypeScript PCF (`DEL-17-07`) and CAEPIPE MBF (`DEL-17-04`) format projection
   now requires the exact `DOTH-FORMAT-003` route, exact document/deliverable
   identity, required root container shape, and exact normalized leaf paths.
   Matching-looking keys in opaque descendants remain unknown.
3. Report component and spring-hanger projection no longer infers invented,
   cleared, or protected posture from user-controlled provenance text. In the
   absence of explicit accepted metadata, the accepted user-local basis is
   `private_project_data / private_only / pending`.
4. Deleted only the generated untracked
   `projects/chirality-piping/test-results/.last-run.json`. H4 source and dist
   output was directed to `{EXTERNAL_TMP}/chirality-n4d-h4.sgJMrD/**`; no
   repository `test-results/` dirty path remains or appears in the attempt-4
   sweep capture.

Product/test changes made by N4D are exactly:

- `core/security/redaction/route_control.py`
- `tests/security/test_redaction_export_controls.py`
- `apps/desktop/src/features/redaction-controls/redactionExportControls.ts`
- `apps/desktop/src/features/redaction-controls/redactionExportControls.test.ts`
- `apps/desktop/src/features/report/renderableReportInput.ts`
- `apps/desktop/src/features/report/renderedReport.test.tsx`

## Verification

- focused Python redaction controls: `13 passed`;
- focused TypeScript redaction/report: `2 files / 44 passed`;
- registered full piping pytest: `522 passed`;
- registered full desktop Vitest: `24 files / 487 passed`;
- registered desktop production build: `PASS`;
- headless Rust crate: `44 passed`;
- H4 source: `2 passed`; H4 production-dist: `1 passed`, both with external
  output and no repository test-results effect;
- practitioner harness terminal: `311 passed`; self-check: `PASS`;
- claims, path-anchor, receipt, dirty JSON/JSONL, protected/release/state,
  containment, and `git diff --check` validators: `PASS`.

The first N4D harness run is preserved as an intermediate failure: all 310
completed tests passed except the live portability invariant, which rejected
literal `/tmp` spellings in the new H4 evidence. The evidence was corrected to
the symbolic `{EXTERNAL_TMP}` anchor and the terminal identical harness run
passed all 311 tests. A receipt-validator invocation also encountered a
transient Python library-load denial from macOS system policy; its immediate
identical retry passed and the failure is recorded in validator evidence.

## Attempt-4 sweep

Exactly one late attempt-4 DEC-025 sweep was invoked after every pre-sweep gate
passed:

- registered result: `PASS`, exit `0`, duration `224.379s`;
- artifact:
  `validation/evidence/sweeps/SWEEP_20260722T073143Z_0c066652cd52-dirty.json`;
- artifact status/duration: `pass`, `221.193s`;
- SHA-256:
  `67fe4d2042469ba2ec2950c717b823bb4ad2a6ad66324889a13e6e354be2a29d`;
- sweep capture: `77` dirty paths, zero `test-results/` paths;
- disposition: attempt-4 acceptance-eligible only for fresh N5D verification;
  no acceptance or closeout claim is made by N4D.

Attempts 1-3 remain byte-identical and superseded/non-acceptance:

- attempt 1:
  `10fbc3c4e54b69df2856cb5dd42240dc87b35d4c3762df9664e6036cf7b3cd63`;
- attempt 2:
  `d1620f2f25264400be6a68e6931982c13b7c17fe52a14381fe70c673f045ca72`;
- attempt 3:
  `b2e89383c1e8a61ebfea8d52809358be53d23c56160315497f17d1d01844f4b6`.

The immutable N4B H4-dist evidence also remains byte-identical at
`26abe58c3f5eb6e021112d606984b9deda52069b42e9dbf385468afbed01f6af`.

## Containment and residual boundary

The complete final tracked/untracked dirty inventory is recorded in
`CHECK_change-scope_final.json`; it has zero fence violations and zero
`test-results/` paths across all `180` enumerated dirty paths.
Protected-content/release exporters, DEL-12-02 state and
memory, final deliverable run records, loop receipts, and Git state remain
unchanged from frozen base `0c066652cd527eb1559f715e914262d2bda42602`.

The pre-existing native-GUI automation limitation remains: mocked-Tauri tests
cover pre-IPC/native behavior and H4 covers browser-visible controls and
downloads; no real native save/print automation claim is made. Fresh N5D
verification remains required before any W3 effect.
