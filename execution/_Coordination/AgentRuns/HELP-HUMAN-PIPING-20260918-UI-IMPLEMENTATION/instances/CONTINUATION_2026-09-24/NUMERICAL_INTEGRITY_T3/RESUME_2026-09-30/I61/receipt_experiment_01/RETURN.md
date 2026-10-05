# I61 RETURN: the first real retained-precision receipts (disposable projection experiment)

**Status: STOPPED on escalated contract tensions.** This meets the assignment's second end
condition: only escalated tensions remain.
- **Wire shape and hashes:** the producer emits real receipts for RF-SKEW-T-CANT-OFF-122-r1e-04 in both modes. All three draft readers accept their wire shape and every hash (G0–G3).
- **The main line stops at G4 in all three readers,** on contract tension T1. T1 needs a ROOT reading.
- **Under two labelled counterfactuals the milestone passes G0–G8 everywhere.** Probe B removes the legacy source-unavailable diagnostic, for T1; probe C makes the request's model schema 0.2.0, for T2. Under both, the milestone receipts pass G0–G8 in all three readers, in both modes, with standing `needs_recompute`. The three readers' classifications are identical, and each equals the producer's own certificate verdict on every row.
- **The refusal receipt** has a valid wire shape (G0–G2) but is unpublishable by contract (G3, T3).

This is wire-contract evidence only. It is not the public milestone (ruling, point 3), not
facade custody, admission or M, and not acceptance.

TASK Type 2 under ROOT's standing assignment ("The first real receipt: a disposable projection
experiment", NUM `c817a86cb1`), with no descendants.
- **Time:** 2026-10-04T00:49Z to 01:07Z, well inside the 5 h budget.
- **Host:** the M5 Max, with the memory guard (PID 5387) running throughout.
- **Toolchain:** Cargo on the default toolchain (no `DEVELOPER_DIR`), `--locked --offline`, `CARGO_BUILD_JOBS=4`, one Cargo job of mine at a time.
- **Python:** VENV, with the I52 helper binaries.
- **TypeScript:** the `node_modules` link and READER's prebuilt `public/` WASM copied into the archive.
- **Git:** no Git writes; Git reads used `GIT_OPTIONAL_LOCKS=0`.
- **Writes:** only inside the fence: WT/scratch/i61_receipt_experiment_01/, WT/targets/i61-receipt/{product_physics,result_export} and this folder.
- **Archive copies:**
  - `prod/` = `git archive` of NUM `c817a86cb1`, P/core and P/fixtures;
  - `reader/` = `git archive` of READER `b36739112a`; READER's working tree was not used.
- **Archive edits:** one `#[cfg(test)] mod` line plus the emitter file in prod's lib.rs (`_run_records/archive_lib_rs.diff`), and the three disposable harness files in reader.
- **Checks in the harnesses:** none was removed or weakened. The readers ran unchanged: Python `_validate_draft`, Rust `validate`, TypeScript `validateRetainedPrecision`. The Python harness additionally lists jsonschema violations, as a diagnostic aid only.

## Results: first gate and code per receipt, per iteration (PY / RS / TS)

| Iteration | Receipt | Python | Rust | TypeScript |
|---|---|---|---|---|
| iter01 (base) | milestone sparse, dense | G4 DIAGNOSTIC_MISMATCH | G4 | G4 |
| iter02 (base + refusal) | milestone sparse, dense | G4 | G4 | G4 |
| iter02 | refusal (preparation, sparse) | G3 COVERAGE_MISMATCH | G3 | G3 |
| probeB_iter01 (counterfactual B) | milestone sparse, dense | G5c CLASSIFICATION_MISMATCH | G5c | G5c |
| probeB_iter02 (B + E1 fix) | milestone sparse, dense | G8 INVOCATION_MISMATCH | G8 | G8 |
| probeC_iter01 (B + C + E1) | milestone sparse, dense | **PASS** needs_recompute (98 / 99 classes) | **PASS** | **PASS** |
| **iter03 (base + E1, final)** | milestone sparse, dense | **G4 DIAGNOSTIC_MISMATCH** | **G4** | **G4** |
| **iter03** | refusal | **G3 COVERAGE_MISMATCH** | **G3** | **G3** |
| probeC_iter02 (rerun with class dumps) | milestone sparse, dense | PASS | PASS | PASS |

Every receipt in every iteration had **0** jsonschema violations against READER's schema
(`07951edacfed…`). That covers the D9b explicit `source_ref: null` and the unavailable branch.

**Classification parity, probe C** (`_run_records/class_parity_probeC_iter02.txt`):
- the Python, Rust and TypeScript classification lists are identical (sparse 98 rows, dense 99);
- they equal the producer's `CertifiedProductProof::verdicts()` row for row (98/98, 99/99): 69 absolute-verified, 25 relative-verified, 3 input-derived, and 1 or 2 non-quantity.

**The final receipts** (WT/scratch/i61_receipt_experiment_01/out/):

| Receipt | sha256 | bytes |
|---|---|---|
| iter03/milestone_sparse_interactive.json | 1f85ce7d20cec5c2dee3cf31d163e73edccae6304fba537181dcb5b98c725b97 | 202242 |
| iter03/milestone_dense_scrutiny.json | 03085ff509db20d5a4f9b650ba91bf80e4a7e01f13581f4b6bf430dfc4c9df6b | 203591 |
| iter03/refusal_preparation_sparse_interactive.json | 6fb7883c6539e652bca1564223f8c962cdf77d3e99b5ec7847972f73231d76df | 104987 |
| probeC_iter02/milestone_sparse_interactive.json (counterfactual) | 507013a6da36ec57de0ccc9014c6e1cb800723b89b344f7b75c9d381784db447 | 201616 |
| probeC_iter02/milestone_dense_scrutiny.json (counterfactual) | 8168dc3e064d20f6a594e1ce82ef5408fc67e8df4bcd727d93a1a6c0b73ff6dd | 202965 |

## Ledger summary (details and citations in LEDGER.md; provenance in PROVENANCE.md)

**Contract tensions, escalated.** D19–D30 address none of them.
- **T1 (blocks the main line at G4).** The actual ordinary route attempts the legacy source-block method for this case. It fails at source closure ("not a signed permutation", 46,628 charged), and the route emits `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` naming the case. C1's G4 row (WIRE_CONTRACT:147) forbids that diagnostic on a retained-selected case. C2's `legacy_source` (CONTRACT_DELTA:160) keeps it as the disclosure. ROOT chooses between:
  - suppressing it in the successor and carrying the work by `work_ref`;
  - amending G4;
  - a routing rule that declines the legacy attempt.
- **T2 (G8, reached under B).** All three readers admit only model `schema_version` 0.2.0 or 0.3.0 (PY:1353, RS:3278, TS). The milestone request fixture is 0.1.0, and the producer accepts it. I found no basis for the rule in C1–C3. ROOT chooses between re-authoring the milestone request at 0.2.0 (its digest changes, so it needs owner identity) and admitting 0.1.0.
- **T3 (refusal receipt, G3).** A one-case invocation with only an unavailable case has no successor publication (C1; PY:1572). The private prepared driver admits exactly one load case (PP/retained_product.rs:3030). So D9b, D19 and the unavailable branch cannot yet be exercised with real producer output, although their wire shape passes G0–G2. ROOT decides whether multi-case prepared support enters the receipt transaction's scope.

**Emitter bugs, fixed.**
- **E1:** `selection.absolute_verified` must come from the certificate's published-row verdicts, not the native `RetainedEvidence` lists. Fixing it cleared G5c.
- **E0:** `legacy_source` is now mapped from the actual source-block diagnostic.

**Producer gaps: no reader failure, but no typed producer fact.** These are the real serializer's work:
- the envelope transformation and its `RETAINED_PRECISION_*` diagnostic text;
- the Ordinary `initial`, `w2`, `formation` and `legacy_source` members, which are read back from the envelope, with `work_ref` null;
- the D6a attribution of invocation-level diagnostics (A2);
- derived `support_indices`;
- `not_covered`, which has no producer class;
- the READER-only fixtures (A4);
- `retained_state_sha256` (A1), which no reader checks;
- C1 §2's no-wrap premise;
- the unimplemented closed translations of unreached error paths;
- one-case indexing;
- custody-only owner binding (RV77-N4).

**Confirmed matches.**
- The invocation digest matches G8.
- The receipt, publication, source-identity and preparation hashes computed by the producer's canonicalizer match all three readers.
- The C1 §1 logical-attempt projection and the work equations pass G5 WORK.
- The typed C3 seam (stages, checks, proof trace and summary coverage) passes G5 PRODUCT_ATTEMPT and G5a.
- The section and geometry facts pass G5b and G8 under probe C.

## Measurements

- **Producer run** (the debug test binary emitting both milestone modes plus the refusal receipt, with serialization and hashing; `/usr/bin/time -l`, two runs): 1.24–1.36 s real, **maximum RSS 20.0–20.1 MB**, peak footprint 7.5–7.6 MB.
- **Through `cargo test`, including the rustc rebuild of the emitter:** 4.6 s, 939 MB maximum RSS. That figure is the compiler, not the producer.
- **Reader runs:** Python seconds; Rust 3 s (prebuilt target clone); vitest 0.4 s.
- **Host load:** no unexpected load; the memory guard log has no kills.

## Decisions ROOT needs

1. **T1:** the reading for a legacy source-unavailable disclosure on a retained-selected case. Probe B shows that suppressing it, with `legacy_source {unavailable, null, null}`, lets every later gate pass.
2. **T2:** the milestone request's model schema, 0.1.0 versus G8's 0.2.0/0.3.0, and where that rule comes from.
3. **T3:** whether multi-case prepared support, and hence a real unavailable-case receipt, is in the receipt transaction's scope, or whether D9b, D19 and the unavailable branch stay synthetic until then.
4. **Assumptions A1, A2 and A4,** and the producer gaps G-a to G-k, as inputs to the real serializer's design (ruling, point 4).
5. **The rerun:** on the accepted reader head after D19–D30, as the ruling requires.

## Records here (see SHA256SUMS)

- **RETURN.md:** this file.
- **LEDGER.md:** the mismatch ledger.
- **PROVENANCE.md:** the field provenance map.
- **PROGRESS_G0_G2.md:** the progress note.
- **_run_records/:**
  - the emitter;
  - the three harnesses;
  - `run_iter.sh`;
  - the archive lib.rs diff;
  - the class-parity output.

## Bulk (WT/scratch/i61_receipt_experiment_01/)

| sha256 | bytes | path (WT/scratch/i61_receipt_experiment_01/…) |
|---|---|---|
| 4bd82e01d80383be2d0bb9db1070c18343cad467a0290aaa69018e232a4d2c25 | 7046 | logs/01_pp_build.log |
| eaf9d105095f19b8bc813a4625ff2d0cef6f665060a6f77053df571292e76812 | 195307 | logs/02_emit_iter01.log |
| c571fead891bc70532e170fa431ac5a5215b2b8198c4d2e248fd4622b91b5507 | 398 | logs/03_py_iter01.log |
| 9e93ab77d7ee552cdc820ed59ab363c5827e8aa7929cba3b8e8aadbfa93856ff | 1648 | logs/04_rs_iter01.log |
| c17fcacc038344733a5ddbe5d4ab769aa28a4894d62838619ec724269bf621b2 | 455 | logs/05_ts_iter01.log |
| 5b9b43ea7f13f96e4bc949384b10aded188f324d2dc9faaac7d1af8ffe6b834b | 940 | logs/06_rss_run1.log |
| 46e4e3c8040c1826c601c90ad69e216f7dcf102571c415a451a92f2e3721da8e | 940 | logs/06_rss_run2.log |
| 863289852759fc776cab044efd22cbd81d2391c52858164726b5dae1b427f9bb | 194836 | logs/iter02_emit.log |
| 9756fcff8b34a1bc844191155ab7d5ab07174e6390c916f91a8678863851ca1e | 607 | logs/iter02_py.log |
| 9ad11bf184698cb85db34136c2a95ad31d6a89e36213f14147ba952fe3ed6dd1 | 1075 | logs/iter02_rs.log |
| a78c0249de32a9f4b4e97b76c2ad7c9e09e5c04b6b1f4b87070d18a0f4faaaed | 456 | logs/iter02_ts.log |
| 38c381db4006c08010e98997edecf91a8d62e9ddb3cf879427537b154ec7c7bb | 292 | logs/iter02_ts_results.txt |
| f0dd7726a8038254b733a41fd35121b88b065295a76a53bf721d6cd5d78ce2d3 | 194611 | logs/iter03_emit.log |
| 9756fcff8b34a1bc844191155ab7d5ab07174e6390c916f91a8678863851ca1e | 607 | logs/iter03_py.log |
| fb41c769fb7a4df38125c2dddb2ada3ebc650ca184a7965ce57a12a551224ff6 | 1075 | logs/iter03_rs.log |
| efed39bb755bca6cb952ca5b98df2b2a76dac10dfca7f81b7540030cf7e6ab74 | 455 | logs/iter03_ts.log |
| 38c381db4006c08010e98997edecf91a8d62e9ddb3cf879427537b154ec7c7bb | 292 | logs/iter03_ts_results.txt |
| 5b9f2585b88221c184321d2dd1e82d9b8ee902c2b43840d12ddef95a9688e021 | 194231 | logs/probeB_iter01_emit.log |
| 73be7144d58642cf6f52eac03989f34cb843fdbb5b44e8e5c693d2331695969e | 468 | logs/probeB_iter01_py.log |
| 2fa44a06ffba7fbd85ec15d5a09db8629a5e01847252b68156d3cc59ecb73276 | 979 | logs/probeB_iter01_rs.log |
| 3af9cf5644f7c7bc1e51357e86abd10830edd30192b062334c80e531f506afa6 | 455 | logs/probeB_iter01_ts.log |
| c96792a031f96a94cf044dba7594fc1d9f3a7cb79969e0eb24e64be2104b438e | 198 | logs/probeB_iter01_ts_results.txt |
| ab7d719fb562c8fff3893b721af8b833bed6ad901512ffd22df1c57f174706d0 | 194456 | logs/probeB_iter02_emit.log |
| 92c183d5164e5e305636c3612c2985af6db3a3685afdc882fec5935161e24012 | 478 | logs/probeB_iter02_py.log |
| 95d93a5e6df8926f5021a8c440759c2ee0127529b4c95a3be3d905eb67778761 | 969 | logs/probeB_iter02_rs.log |
| d6ebd7db2d38cb01074c7cdec1b0533e4dfbd0ef832d2aa3234ce520cedad46d | 455 | logs/probeB_iter02_ts.log |
| 7c69d0ebdc0ea47b4b530190109dee56d8f2bdadaa9308291bc260512f0b4bb5 | 188 | logs/probeB_iter02_ts_results.txt |
| 327ba96123c78d9e80630c459529a90c974aa58dfaa80be65318e4f831a4824b | 194456 | logs/probeC_iter01_emit.log |
| 10332f899839547ff73d398e79e2ccd521456037fec607935bbf9250da3de523 | 274 | logs/probeC_iter01_py.log |
| 64248a96dcff096d35ce647a4b386e2e46bea9a75c62d24e6b0db8f25f0bd48f | 955 | logs/probeC_iter01_rs.log |
| 1037720169b2ed2223aa6ffbd7a41b0f5d6be0e1b9c13611e6bb64cb1956c31d | 455 | logs/probeC_iter01_ts.log |
| 77e72d637bcc0d8dcc12f95869121d715a3c1d1419fdd023c6680a4e192bf4a1 | 180 | logs/probeC_iter01_ts_results.txt |
| 89b192d3566c9f0d9da85c62cdd1f207122872d92e68d1c0308ae84ced865690 | 388 | logs/probeC_iter02_class_parity.txt |
| 63beac56ce40f9b16f6397e04202c5873e47f3a6260791635c708a559523d8d5 | 194456 | logs/probeC_iter02_emit.log |
| 10332f899839547ff73d398e79e2ccd521456037fec607935bbf9250da3de523 | 274 | logs/probeC_iter02_py.log |
| fcf3ced17c3415a1994bd976239e606c358440db61d33a5e1499d0f5c7207d28 | 1188 | logs/probeC_iter02_rs.log |
| a51b295aa3b5882a9f0c3aeec239b2436a8f3cdacba13156a4e6b6455515f081 | 455 | logs/probeC_iter02_ts.log |
| 77e72d637bcc0d8dcc12f95869121d715a3c1d1419fdd023c6680a4e192bf4a1 | 180 | logs/probeC_iter02_ts_results.txt |
| 6fb7883c6539e652bca1564223f8c962cdf77d3e99b5ec7847972f73231d76df | 104987 | out/iter02/refusal_preparation_sparse_interactive.json |
| e67526ce9169fbc0ae240d48fd5926af9ae6b1ab683317f27d323db1d66c778a | 2101 | out/iter02/refusal_preparation_sparse_interactive.provenance.json |
| 03085ff509db20d5a4f9b650ba91bf80e4a7e01f13581f4b6bf430dfc4c9df6b | 203591 | out/iter03/milestone_dense_scrutiny.json |
| d3d356220a476f5aad2ea4e61cb2fa0b711824dacb962ca5be3c685005c427bc | 3789 | out/iter03/milestone_dense_scrutiny.provenance.json |
| 1f85ce7d20cec5c2dee3cf31d163e73edccae6304fba537181dcb5b98c725b97 | 202242 | out/iter03/milestone_sparse_interactive.json |
| d3d356220a476f5aad2ea4e61cb2fa0b711824dacb962ca5be3c685005c427bc | 3789 | out/iter03/milestone_sparse_interactive.provenance.json |
| 6fb7883c6539e652bca1564223f8c962cdf77d3e99b5ec7847972f73231d76df | 104987 | out/iter03/refusal_preparation_sparse_interactive.json |
| e67526ce9169fbc0ae240d48fd5926af9ae6b1ab683317f27d323db1d66c778a | 2101 | out/iter03/refusal_preparation_sparse_interactive.provenance.json |
| 669c042f9af4f7502cec6f3153c465cedf6fe5e989a84f1528897b8827271b32 | 202965 | out/probeB_iter02/milestone_dense_scrutiny.json |
| 7318e3f989ba63be5cbf0aefa8bbc53a7e1c4c2910bfc26dd0df4822d3609c24 | 3927 | out/probeB_iter02/milestone_dense_scrutiny.provenance.json |
| 53c3cdad3384770c74c3a340854718e00d458a5597b4294909f2bb19eff15a6b | 201616 | out/probeB_iter02/milestone_sparse_interactive.json |
| 7318e3f989ba63be5cbf0aefa8bbc53a7e1c4c2910bfc26dd0df4822d3609c24 | 3927 | out/probeB_iter02/milestone_sparse_interactive.provenance.json |
| 8168dc3e064d20f6a594e1ce82ef5408fc67e8df4bcd727d93a1a6c0b73ff6dd | 202965 | out/probeC_iter02/milestone_dense_scrutiny.json |
| 7aaf321c46743aa441c99b57f8ad44fdf3c52e97734120e1b384701456a78d3e | 8775 | out/probeC_iter02/milestone_dense_scrutiny.json.py_classes.txt |
| ca51f7087815feccf2b1c2809f5103872d54d80235847de96ae8cc9a59937bc0 | 11160 | out/probeC_iter02/milestone_dense_scrutiny.json.rs_classes.txt |
| 7aaf321c46743aa441c99b57f8ad44fdf3c52e97734120e1b384701456a78d3e | 8775 | out/probeC_iter02/milestone_dense_scrutiny.json.ts_classes.txt |
| d411a6dc3064e0451427ecf3ea035f7c6c030dd0528ecef1b4e00b7aebbe37a5 | 4054 | out/probeC_iter02/milestone_dense_scrutiny.provenance.json |
| 9dee8a36b0dbae8138b3c47f763f914df921a0ea6ddcb2c5cb3063242510e4ed | 10007 | out/probeC_iter02/milestone_dense_scrutiny.verdicts.txt |
| 507013a6da36ec57de0ccc9014c6e1cb800723b89b344f7b75c9d381784db447 | 201616 | out/probeC_iter02/milestone_sparse_interactive.json |
| 6bd72ef19872bed60693625d090077678367e9d369014cfc4c165e18d7815bf3 | 8693 | out/probeC_iter02/milestone_sparse_interactive.json.py_classes.txt |
| 7cf0c9fb4391ed468a9b1ffd31ece63d9bff17bdcad05ba2246ace84368314ee | 11079 | out/probeC_iter02/milestone_sparse_interactive.json.rs_classes.txt |
| 6bd72ef19872bed60693625d090077678367e9d369014cfc4c165e18d7815bf3 | 8693 | out/probeC_iter02/milestone_sparse_interactive.json.ts_classes.txt |
| d411a6dc3064e0451427ecf3ea035f7c6c030dd0528ecef1b4e00b7aebbe37a5 | 4054 | out/probeC_iter02/milestone_sparse_interactive.provenance.json |
| ff702362f6c978f3a34cec7c1130643db331e68557d9315f09cd53b7f3ffa5cc | 9909 | out/probeC_iter02/milestone_sparse_interactive.verdicts.txt |
| 9656a124f54dc156754d9370f7d166cdf259ae1cd7c59dd2d55a91cbc52260b1 | 49131 | prod/projects/chirality-piping/core/product_physics/src/i61_receipt_probe.rs (emitter, archive copy) |
