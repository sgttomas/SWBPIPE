# WORKING_ITEMS Fan-in — N4D immutable remediation attempt 4

**Verdict:** `ACCEPT_FOR_FRESH_N5D_REVIEW`

- N4D return and TASK record are terminal `SUCCESS`.
- The four N5C findings were addressed within the exact authorized fence:
  record-local public basis no longer propagates to nested children; PCF/MBF
  structural projection uses exact document, deliverable, container, and path
  allowlists; report projection no longer infers claim state from user text;
  and the generated root `test-results/.last-run.json` was removed.
- Focused Python 13 and TypeScript 44 pass; piping 522, desktop 487 plus build,
  Rust 44, H4 source 2/2 and dist 1/1, harness 311 plus self-check all pass.
- Final claims, 808 path anchors, receipts, JSON/JSONL, protected/release/state,
  containment, and `git diff --check` validation pass. The complete final
  containment inventory covers 180 dirty paths with zero violations and zero
  dirty `test-results/` paths.
- Exactly one attempt-4 late sweep ran after all pre-sweep gates and passed:
  `SWEEP_20260722T073143Z_0c066652cd52-dirty.json`, SHA-256
  `67fe4d2042469ba2ec2950c717b823bb4ad2a6ad66324889a13e6e354be2a29d`.
- Attempts 1–3 remain immutable superseded/non-acceptance evidence. The N4B H4
  artifact remains hash-bound by the validated historical-evidence override.
- No DEL-12-02 state, memory, final deliverable run record, receipt, or Git
  closeout effect occurred.

Fresh N5D must independently test the exact four N5C closures and revalidate
the complete adopted implementation, prior finding closures, route/register
coverage, scope, evidence selection, policy classification, and all four sweep
dispositions. It must not edit or run another sweep. W3 remains held.

