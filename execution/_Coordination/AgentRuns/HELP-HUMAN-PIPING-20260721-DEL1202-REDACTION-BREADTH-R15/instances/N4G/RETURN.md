# N4G Return — Exact N5F remediation attempt 7

**Status:** `SUCCESS`  
**Blockers:** none within N4G  
**Fresh verifier required:** yes (`N5G`)  
**State/receipt/lifecycle/release/Git effect:** none

## Implemented behavior

1. `REXC-CORE-001` now guards `target_mapping_contract` and `target_fixture`
   before any mapping dereference. Omitted, explicit-null, and non-Mapping
   values reach the existing controlled source-diagnostic gate as
   `EXP-TARGET-MAPPING-MISSING` or `EXP-TARGET-FIXTURE-MISSING` rather than
   raising.
2. A missing, null, or non-Mapping manifest `units_manifest` now emits the
   existing blocking `EXP-HANDOFF-MANIFEST-FIELD-MISSING` diagnostic. The
   mapping comparison no longer dereferences an invalid units value or emits a
   derived units mismatch for that invalid source.
3. Every invalid-source case returns `blocked=true`, `payload=None`, sanitized
   decisions/findings/summary evidence, and zero materializer calls. Exact
   tests cover all nine source/value combinations.
4. Lossless-only withholding now records `lossless_blocking_count=1` and adds
   it to `exposure_blocking_count`; the real withholding blocker is therefore
   nonzero and consistent with `blocked=true` without changing redaction
   findings or lossless semantics.

N4G changed exactly these product/test paths:

- `projects/chirality-piping/core/handoff/exporter/workflow.py`
- `projects/chirality-piping/core/security/redaction/route_control.py`
- `projects/chirality-piping/tests/test_handoff_export_workflow.py`

## Verification

- focused workflow: `17 passed`;
- registered full piping pytest: `533 passed`;
- registered desktop Vitest: `24 files / 492 passed`;
- registered desktop production build: `PASS`;
- H4 source: `2 passed`; H4 production-dist: `1 passed`, both with external
  output;
- headless Rust crate: `44 passed`;
- practitioner harness: `311 passed`; self-check: `PASS`;
- claims-language: `PASS`, `264` files;
- path anchors: `PASS`, `878` live surfaces and zero findings;
- receipt validator: `PASS`, frozen through `Receipt-44`;
- terminal dirty JSON/JSONL parse: `PASS`, `100` JSON files, `1` JSONL file,
  `51` JSONL records;
- N4G write-fence validation and `git diff --check`: `PASS`.

## Attempt-7 sweep

WORKING_ITEMS acknowledged the terminal pre-sweep PASS, complete 254-path
inventory, zero `test-results/`, exact prior hashes, zero attempt-7 count, and
the exact registered command. N4G invoked that command exactly once and never
reran it.

- registered result: `PASS`, exit `0`, duration `258.922s`;
- artifact:
  `projects/chirality-piping/validation/evidence/sweeps/SWEEP_20260722T094654Z_0c066652cd52-dirty.json`;
- artifact status/duration: `pass`, `255.821s`;
- SHA-256:
  `6b6a99dfe79186a3b6d25b6fa192eba24485584bd82694db40782b6c89641ade`;
- sweep capture: `80` dirty paths;
- disposition: attempt 7 is acceptance-eligible only for fresh N5G
  verification. N4G makes no acceptance or closeout claim.

Attempts 1–6 remain byte-identical:

- attempt 1: `10fbc3c4e54b69df2856cb5dd42240dc87b35d4c3762df9664e6036cf7b3cd63`;
- attempt 2: `d1620f2f25264400be6a68e6931982c13b7c17fe52a14381fe70c673f045ca72`;
- attempt 3: `b2e89383c1e8a61ebfea8d52809358be53d23c56160315497f17d1d01844f4b6`;
- attempt 4: `67fe4d2042469ba2ec2950c717b823bb4ad2a6ad66324889a13e6e354be2a29d`;
- attempt 5: `3838165eae833faf04a5b2612f42e102224911582580bcab06bef4aa5757a12b`;
- attempt 6: `d2e4a79447a9cd04c6ae03061be6c291dd1864a7c240a15af8210ab8a5c208c5`.

The sweep generated ignored
`projects/chirality-piping/apps/desktop/test-results/.last-run.json`; its
content reported `passed`. N4G deleted only that known generated file.

## Containment and residual boundary

The terminal pre-sweep repository inventory contained `254` complete
tracked/untracked dirty paths with zero N4G write-fence violations and zero
`test-results/` files. The final inventory after the sole sweep, its evidence,
this return, and TASK completion contains `257` paths, with zero violations
and zero `test-results/` files. No path is staged.

Protected-content/release exporters, DEL-12-02 `_STATUS.md`, `MEMORY.md`, final
deliverable run records, loop receipts, lifecycle state, and Git state remain
unchanged from frozen base `0c066652cd527eb1559f715e914262d2bda42602`.
The branch remains `codex/piping-pkg12-redaction` and HEAD remains the frozen
base.

The pre-existing native-GUI automation limitation remains: mocked-Tauri tests
cover pre-IPC/native behavior and H4 covers browser-visible controls and
downloads; no real native save/print automation claim is made. Fresh N5G
verification is mandatory before any state, receipt, lifecycle, release, or
Git effect.
