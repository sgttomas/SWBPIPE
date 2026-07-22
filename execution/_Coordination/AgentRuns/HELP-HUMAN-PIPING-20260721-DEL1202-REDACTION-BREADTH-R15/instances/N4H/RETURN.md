# N4H Return — Empty units-manifest remediation attempt 8

**Status:** `SUCCESS`  
**Blockers:** none within N4H  
**Fresh verifier required:** yes (`N5H`)  
**State/receipt/lifecycle/release/Git effect:** none

## Implemented behavior

1. `REXC-CORE-001` again treats an empty `units_manifest` Mapping as a
   missing required manifest field while retaining N4G's type guard. Empty,
   null, omitted, and non-Mapping values emit the existing blocking
   `EXP-HANDOFF-MANIFEST-FIELD-MISSING` source diagnostic.
2. The exact regression sets `units_manifest={}` and the copied
   `source_context.units_manifest_ref=None`. It proves the public workflow
   returns blocked with no payload, makes zero materializer calls, retains
   sanitized decisions/findings, and reports truthful `1/1/0/0/1`
   source/source-blocking/redaction/lossless/exposure counts.
3. Non-empty valid mappings and every prior closure remain unchanged. N4H
   changed exactly these product/test paths:

   - `projects/chirality-piping/core/handoff/exporter/workflow.py`
   - `projects/chirality-piping/tests/test_handoff_export_workflow.py`

## Verification

- focused workflow: `18 passed`;
- registered full piping pytest: `534 passed`;
- registered desktop Vitest: `24 files / 492 passed`;
- registered desktop production build: `PASS`;
- H4 source: `2 passed`; H4 production-dist: `1 passed`, both with external
  output;
- headless Rust crate: `44 passed`;
- practitioner harness: `311 passed`; self-check: `PASS`;
- claims-language: `PASS`, `264` files;
- path anchors: `PASS`, `902` live surfaces and zero findings;
- receipt validator: `PASS`, frozen through `Receipt-44`;
- terminal dirty JSON/JSONL parse: `PASS`, `116` JSON files, `1` JSONL file,
  `56` JSONL records before this Markdown return;
- N4H write-fence validation and `git diff --check`: `PASS`.

Intermediate failures are preserved rather than overwritten:

- the first focused run was `17 passed / 1 failed` because the new test
  constructed its mapping fixture after emptying the manifest; fixture setup
  order was corrected before the public-workflow probe;
- the first H4 source run passed desktop and hit an environmental Chrome
  launch-close on compact before test execution; its same-command rerun passed
  both projects;
- the first harness run was `310 passed / 1 failed` because new H4 evidence
  recorded literal external temporary paths; evidence was normalized to
  `{EXTERNAL_TMP}` and the registered harness rerun passed.

## Attempt-8 sweep

WORKING_ITEMS acknowledged the terminal pre-sweep PASS, complete `279`-path
inventory, zero `test-results/`, exact seven prior hashes, zero attempt-8
count, and the exact registered command. N4H invoked that command exactly once
and never reran it.

- registered result: `PASS`, exit `0`, duration `236.034s`;
- artifact:
  `projects/chirality-piping/validation/evidence/sweeps/SWEEP_20260722T101717Z_0c066652cd52-dirty.json`;
- artifact status/duration: `pass`, `232.766s`;
- SHA-256:
  `3b08deb6b9a87dbeb3c7715266a072a3d1a3ebb97898dab0ff4556922013076c`;
- sweep capture: `81` dirty paths;
- disposition: attempt 8 is acceptance-eligible only for fresh N5H
  verification. N4H makes no acceptance or closeout claim.

Attempts 1–7 remain byte-identical:

- attempt 1: `10fbc3c4e54b69df2856cb5dd42240dc87b35d4c3762df9664e6036cf7b3cd63`;
- attempt 2: `d1620f2f25264400be6a68e6931982c13b7c17fe52a14381fe70c673f045ca72`;
- attempt 3: `b2e89383c1e8a61ebfea8d52809358be53d23c56160315497f17d1d01844f4b6`;
- attempt 4: `67fe4d2042469ba2ec2950c717b823bb4ad2a6ad66324889a13e6e354be2a29d`;
- attempt 5: `3838165eae833faf04a5b2612f42e102224911582580bcab06bef4aa5757a12b`;
- attempt 6: `d2e4a79447a9cd04c6ae03061be6c291dd1864a7c240a15af8210ab8a5c208c5`;
- attempt 7: `6b6a99dfe79186a3b6d25b6fa192eba24485584bd82694db40782b6c89641ade`.

The sweep generated ignored
`projects/chirality-piping/apps/desktop/test-results/.last-run.json`; its
content reported `passed`. N4H removed only that known generated file.

## Containment and residual boundary

The terminal pre-sweep repository inventory contained `279` complete
tracked/untracked dirty paths with zero N4H write-fence violations and zero
`test-results/` files. The final inventory after the sole sweep, its evidence,
this return, and TASK completion contains `282` paths, with zero violations
and zero `test-results/` files. No path is staged.

Protected-content/release exporters, DEL-12-02 `_STATUS.md`, `MEMORY.md`, final
deliverable run records, loop receipts, lifecycle state, and Git state remain
unchanged from frozen base `0c066652cd527eb1559f715e914262d2bda42602`.
The branch remains `codex/piping-pkg12-redaction` and HEAD remains the frozen
base.

The pre-existing native-GUI automation limitation remains: mocked-Tauri tests
cover pre-IPC/native behavior and H4 covers browser-visible controls and
downloads; no real native save/print automation claim is made. Fresh N5H
verification is mandatory before any state, receipt, lifecycle, release, or
Git effect.
