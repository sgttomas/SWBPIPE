# N4C Return — Leaf-explicit remediation attempt 3

**Status:** `SUCCESS`  
**Blockers:** none within N4C  
**Fresh verifier recommendation:** yes  
**W3/state/Git:** held and unchanged

## Implemented behavior

The Python and TypeScript route projectors no longer derive a payload-wide
public basis from root or sibling `privacy` / `provenance` envelopes. Public
treatment is now bounded to:

- a contract-native metadata-bearing scalar leaf;
- an exact record whose own direct metadata contains both an allowed privacy
  classification and an allowed redistribution status; or
- the existing exact structural-token projection, including document-bound
  PCF (`DEL-17-07`) and CAEPIPE MBF (`DEL-17-04`) format metadata in TypeScript.

Unmetadataed siblings remain `unknown / unknown / pending`. In
`public_report`, `shared_model`, and `downstream_tool` they redact with
`REDISTRIBUTION_STATUS_UNKNOWN`; in `local_private` they preserve the existing
`warning_only` outcome and remain explicitly unknown.

Changed product/test files are exactly:

- `core/security/redaction/route_control.py`
- `apps/desktop/src/features/redaction-controls/redactionExportControls.ts`
- `tests/security/test_redaction_export_controls.py`
- `apps/desktop/src/features/redaction-controls/redactionExportControls.test.ts`

## Verification

- focused Python: `12 passed`;
- focused TypeScript redaction controls: `2 files / 34 passed`;
- registered piping pytest: `521 passed`;
- registered desktop test: `24 files / 483 passed`;
- registered desktop production build: `PASS`;
- H4 source: `2 passed` (desktop + compact); production-dist: `1 passed`;
- practitioner harness: `311 passed`; self-check: `PASS`;
- claims language: `PASS`, 264 files;
- path anchors: `PASS`, 784 live surfaces post-sweep;
- containment: `PASS`, zero violations, including the attempt-3 sweep;
- JSON, receipt, protected/release no-change, and `git diff --check`: `PASS`.

Intermediate failing check evidence is preserved in N4C. It records the
progression from payload-wide compatibility assumptions to record/document-
bounded projection and the pre-policy harness portability finding. Terminal
evidence uses the `_final` / `_final2` files. The initial two-worker H4 source
invocation passed both assertions but encountered the repository-documented
slow teardown; the terminal serial rerun is recorded in `CHECK_h4_source.json`.

## Sweep disposition

Exactly one attempt-3 DEC-025 sweep ran after all prerequisite gates passed:

- artifact: `validation/evidence/sweeps/SWEEP_20260722T065603Z_0c066652cd52-dirty.json`
- SHA-256: `b2e89383c1e8a61ebfea8d52809358be53d23c56160315497f17d1d01844f4b6`
- registered result: `PASS`, exit `0`, duration `212.066s`
- disposition: acceptance-eligible only for fresh verification; no acceptance
  or closeout claim is made by N4C.

Prior sweeps remain byte-identical and are superseded/non-acceptance:

- attempt 1: `10fbc3c4e54b69df2856cb5dd42240dc87b35d4c3762df9664e6036cf7b3cd63`
- attempt 2: `d1620f2f25264400be6a68e6931982c13b7c17fe52a14381fe70c673f045ca72`

The immutable N4B H4-dist evidence also remains byte-identical at
`26abe58c3f5eb6e021112d606984b9deda52069b42e9dbf385468afbed01f6af`;
the validated hash-bound historical portability policy reports no issue.

## Residual boundary and handoff

The pre-existing native-GUI automation limitation remains: mocked-Tauri tests
cover pre-IPC/native behavior and H4 covers browser-visible controls and
downloads; no real native save/print automation claim is made.

Fresh independent verification remains required. N4C performed no deliverable
status, memory, receipt, final deliverable run record, commit, push, PR, merge,
lifecycle, release, legal/security-sufficiency, or professional-acceptance act.
