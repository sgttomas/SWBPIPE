# I61 RETURN: the real-receipt experiment rerun on the accepted-candidate readers

**Status: PASS.** The milestone receipts for RF-SKEW-T-CANT-OFF-122-r1e-04, sparse and dense, pass G0–G8 in all three draft readers with standing `needs_recompute`.
- **What was emitted:** the receipts came from the actual private prepared producer, with T1 option (a) as the owner confirmed it.
- **What was not changed:** the request is the unchanged model-0.1.0 request, and no counterfactual is applied.
- **Classification parity:** every reader's row classifications are identical across the three languages, and each equals the producer's certificate verdict on every row (98/98 sparse, 99/99 dense).
- **The refusal receipt** passes G0–G2 and stops at G3, as T3 decided.
- **No mismatch needed a contract reading.**

This is wire-contract evidence through the test-only prepared driver. It is not the public milestone, which needs the captured facade, admission/M and the real serializer.

TASK Type 2 under ROOT's standing grant (NUM `83732c5677`, ruling "T1 confirmed by the owner"), with no descendants.
- **Time:** 2026-10-04T01:46Z to about 01:50Z.
- **Host:** the M5 Max, with the memory guard (PID 5387) running.
- **Toolchain:** the default toolchain (no `DEVELOPER_DIR`), `--locked --offline`, `CARGO_BUILD_JOBS=4`, one Cargo job at a time.
- **Python:** VENV, with the I52 helper binaries.
- **TypeScript:** the `node_modules` link and READER's prebuilt `public/` WASM copied into the archive.
- **Git:** no Git writes.
- **Writes:** only inside the fence: WT/scratch/i61_receipt_experiment_02/, WT/targets/i61-receipt/ and this folder.
- **Archives:**
  - `prod/` = `git archive` of NUM `83732c5677`, P/core and P/fixtures. Its producer core is identical to `c817a86cb1`;
  - `reader/` = `git archive` of READER `abcb16fd27d7c3ccd019f2533eb261d4c564fdc7`.
- **The readers ran unchanged:**
  - Python `_validate_draft(source, invocation)`;
  - Rust `retained_precision::validate(source, Some(invocation))`;
  - TypeScript `validateRetainedPrecision(source, invocation)`.
  
  The harnesses only load the files, call the reader and dump its classifications.

## The emitter change (`_run_records/emitter_delta_from_experiment_01.diff`)

- **T1 (a) on the retained-selected case:**
  - the successor envelope omits the legacy `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` naming the case, and `ordinary_attempts[0].diagnostic_refs` no longer lists it;
  - `legacy_source = {disposition:"unavailable", diagnostic_ref:null, work_ref:0}`;
  - `body.legacy_source_work[0] = {case_index:0, stage:"source closure", helper_stage:"source_closure", charged:46628, rejected:0, limit:4000000, settlement:"booked"}`. These are the actual WorkReport values, read back from the actual diagnostic's message (producer gap G-l).
- **Unselected case (the refusal receipt):** it keeps today's disclosure. `diagnostic_ref` names the diagnostic, and `work_ref` points to its work entry.
- **No counterfactuals:** the probe-B and probe-C code paths are removed, and the emitter asserts that the request's model `schema_version` is `"0.1.0"`.

## Results (`_run_records/reader_results_*.txt`, `checks_and_class_parity.txt`)

| Receipt | Python | Rust | TypeScript |
|---|---|---|---|
| milestone, sparse_interactive | **PASS G0–G8**, standing `needs_recompute`, invocation-bound, 98 classifications | **PASS G0–G8**, eligible=false, invocation-bound, 98 | **PASS G0–G8**, `needs_recompute`, 98 |
| milestone, dense_scrutiny | **PASS G0–G8**, `needs_recompute`, 99 | **PASS G0–G8**, 99 | **PASS G0–G8**, `needs_recompute`, 99 |
| refusal (preparation, sparse) | G0–G2 pass; first failure G3 `COVERAGE_MISMATCH` | same | same |

Every receipt has 0 jsonschema violations against READER's schema. Gate by gate:
- G0–G3: identity, policies, definition and table; the four hashes; encodings; coverage;
- G4: diagnostics, now passing under T1 (a);
- G5: native, ordinary (O4 and D6d over `legacy_source` and `legacy_source_work`), products and work;
- G5a, G5b, G5c;
- G6: row tokens;
- G7: base projection to preview-physics-1;
- G8: the invocation, the 0.1.0 model under D31, the maps, sections and materials.

All pass for both milestone receipts in all three readers. The readers' standing is `needs_recompute` because eligibility is held by the readers' implementation-complete flags; the draft paths still evaluate every gate.

**Classification parity:**

| Mode | Python = Rust = TypeScript | Readers = producer `CertifiedProductProof::verdicts()` | Classes | All certificate verdicts passed |
|---|---|---|---|---|
| sparse | yes, 98 rows | 98 / 98 | 69 absolute-verified, 25 relative-verified, 3 input-derived, 1 non-quantity | yes |
| dense | yes, 99 rows | 99 / 99 | 69 absolute-verified, 25 relative-verified, 3 input-derived, 2 non-quantity | yes |

**The receipts** (WT/scratch/i61_receipt_experiment_02/out/iter01/). A rerun was byte-identical for all three.

| Receipt | sha256 | bytes |
|---|---|---|
| milestone_sparse_interactive.json | bca4e9ca1bd81a43b7296edf7c5aaccc842d730170111a940dc0aa8637e32e59 | 201916 |
| milestone_dense_scrutiny.json | 064491534c9f5e587ddb4cf2542d30a747cda06f12dae5b55272bb9e6435f310 | 203265 |
| refusal_preparation_sparse_interactive.json | 123c370f4591efc5d65b0ef537997a03fd7b08579bd5449ed2e034b6e4760ee4 | 105287 |

**Measurements:**
- **Producer run** (debug test binary emitting all three receipts): 1.52 s real, maximum RSS 20.5 MB, peak footprint 7.8 MB.
- **Compile:** 13 s for the product_physics test crate in the new archive; 2.4 s for the Rust reader.
- **Host load:** none unexpected.

## Ledger (LEDGER.md)

- **No reader mismatch and no contract tension remain.**
- **Resolved since 01:** T1 by option (a), T2 by D31, and E1 carried over.
- **Recorded limit:** T3, the refusal receipt, stays a G0–G2 check.
- **Producer gaps for the serializer brief:** G-a to G-k as in experiment 01, plus a new **G-l**. The legacy WorkReport is untyped in the producer: the emitter reads it back from the diagnostic text, which C2 forbids for the real serializer. The serializer must capture the typed `RecoveryFailure` and its WorkReport at PP/lib.rs:3747–3760.

## Decisions needed

None for this run. For ROOT's reader closure condition, this is the real-receipt half on `abcb16fd27`; the confirmation round 04 remains. G-l should be added to the serializer brief alongside G-a to G-k and assumptions A1, A2 and A4.

## Records (see SHA256SUMS)

- **RETURN.md** and **LEDGER.md**.
- **_run_records/:**
  - the emitter and its delta from experiment 01;
  - the three harnesses and `run_iter.sh`;
  - the archive lib.rs diff;
  - the reader results;
  - the checks and class parity;
  - the producer RSS.

## Bulk (WT/scratch/i61_receipt_experiment_02/)

| sha256 | bytes | path (WT/scratch/i61_receipt_experiment_02/…) |
|---|---|---|
| 98881d67924729c4eccdf549d7f894b0b3779acbef7907613b14ab541f786960 | 2095 | logs/iter01_checks.txt |
| cdb5e12eb6eab3ecd3da0375d9112682b1a48297d1f24b399bdf410507947dd1 | 197753 | logs/iter01_emit.log |
| c17a81c198823afd66cd1f9e06b47b2a1da50c7baf36b4d44763451c8af3a2c2 | 483 | logs/iter01_py.log |
| a3f3a8327d593562f920db3bb73e5182a4d34eb0ecdec5ca15b9e703c1e522ac | 1740 | logs/iter01_rs.log |
| 32e568316d3fa0f39184b96122a89e8d775822951fc701e6b49b92653b19f506 | 455 | logs/iter01_ts.log |
| 3e5e0614ced28da5a2feebdd762e90367116b67edada8c781429b76dee16021b | 284 | logs/iter01_ts_results.txt |
| 58dc8c9a736a00f13a49de04a62dd7f38950be3abf935dbc8a246f60c270ccc8 | 940 | logs/rss_run.log |
| 064491534c9f5e587ddb4cf2542d30a747cda06f12dae5b55272bb9e6435f310 | 203265 | out/iter01/milestone_dense_scrutiny.json |
| 7aaf321c46743aa441c99b57f8ad44fdf3c52e97734120e1b384701456a78d3e | 8775 | out/iter01/milestone_dense_scrutiny.json.py_classes.txt |
| ca51f7087815feccf2b1c2809f5103872d54d80235847de96ae8cc9a59937bc0 | 11160 | out/iter01/milestone_dense_scrutiny.json.rs_classes.txt |
| 7aaf321c46743aa441c99b57f8ad44fdf3c52e97734120e1b384701456a78d3e | 8775 | out/iter01/milestone_dense_scrutiny.json.ts_classes.txt |
| faab802f997b461d585b187bce5c1e83af248d7a4806a85a0c3b3f161369e61c | 4126 | out/iter01/milestone_dense_scrutiny.provenance.json |
| 9dee8a36b0dbae8138b3c47f763f914df921a0ea6ddcb2c5cb3063242510e4ed | 10007 | out/iter01/milestone_dense_scrutiny.verdicts.txt |
| bca4e9ca1bd81a43b7296edf7c5aaccc842d730170111a940dc0aa8637e32e59 | 201916 | out/iter01/milestone_sparse_interactive.json |
| 6bd72ef19872bed60693625d090077678367e9d369014cfc4c165e18d7815bf3 | 8693 | out/iter01/milestone_sparse_interactive.json.py_classes.txt |
| 7cf0c9fb4391ed468a9b1ffd31ece63d9bff17bdcad05ba2246ace84368314ee | 11079 | out/iter01/milestone_sparse_interactive.json.rs_classes.txt |
| 6bd72ef19872bed60693625d090077678367e9d369014cfc4c165e18d7815bf3 | 8693 | out/iter01/milestone_sparse_interactive.json.ts_classes.txt |
| faab802f997b461d585b187bce5c1e83af248d7a4806a85a0c3b3f161369e61c | 4126 | out/iter01/milestone_sparse_interactive.provenance.json |
| ff702362f6c978f3a34cec7c1130643db331e68557d9315f09cd53b7f3ffa5cc | 9909 | out/iter01/milestone_sparse_interactive.verdicts.txt |
| 123c370f4591efc5d65b0ef537997a03fd7b08579bd5449ed2e034b6e4760ee4 | 105287 | out/iter01/refusal_preparation_sparse_interactive.json |
| 2cf63734740f1f354e7e8f2c82e929bf431e261e97f22c88ce076428379d7e30 | 2258 | out/iter01/refusal_preparation_sparse_interactive.provenance.json |
