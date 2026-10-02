# RV38 maintained coverage inventory

All 28 changed files are covered at `81c03849033f3ce745668f581f446530789397b8` against `3a0251874d6ab38008173a22ef09d651a0f20d9e`. H = `projects/chirality-piping/core/solver/performance_harness`; VR = `projects/chirality-piping/validation/benchmarks/numerical_robustness`. Full paths, pre/post SHA256, candidate blob IDs and line counts are in `_run_records/SOURCE_AND_RECORD_CHECKS.json`. Inspection and metadata checks are complete; no runtime test pass is claimed by RV38.

| # | Path | Coverage |
|---|---|---|
| 1 | `H/observations/k6b/SHA256SUMS` | Checked exact one-line counts-hash replacement; other manifest entries unchanged and new counts hash verifies. |
| 2 | `H/observations/k6b/counts.jsonl` | All 33 rows compared: same order, keys, identity/non-estimate fields; 22 estimate fields per row only. Values match H19-derived literal fixtures. |
| 3 | `H/runner/k6_runner.py` | Complete delta and surrounding schedule/admission/launch/ratio paths inspected; holds before subprocess binding, no stale fallback, same fresh denominator; inherited catalogue/no-op/seed limits recorded. |
| 4 | `H/runner/test_k6_runner.py` | All added tests inspected: actual argv equality, fresh denominator, no-fallback, plan, hold ordering and non-success recorded-ascent semantics. Mocked tests do not qualify full seed input. |
| 5 | `H/src/bin/k6_observe/main.rs` | Complete delta plus parsing/count/source/solve/prefix windows traced: true constructor origin, six actual Args strings, computed/file one-result flow and unchanged numeric backstop. |
| 6 | `H/src/bin/k6_observe/w1.rs` | Doc correction matches KF3 partial-stage behavior and newly explicit inside-solve test; no behavior changed. |
| 7 | `H/src/k6/canonical.rs` | New wrapper calls unchanged parser and returns actual Canonical origin. Existing exact reserves, final lengths/roundtrip and temporary drops read. |
| 8 | `H/src/k6/counts.rs` | Explicit complete-W1 Result interface; four other mode formula branches unchanged; parsed seed37 fields traced. |
| 9 | `H/src/k6/models.rs` | All dispatch branches traced to Builder/Fixture origin; model wrapper/representations and actual recipes retained. |
| 10 | `H/src/k6/w1/counts.rs` | Capture is within existing source/count path; actual IDs before drop and distinct source refusal; old estimator removed, compatibility exports separate. |
| 11 | `H/src/k6/w1/envelope.rs` | Complete 1,233-line source read against accepted phase/owner/metric equations; checked scalar/profile/phase APIs, all helper and schedule expressions, source-refusal extraction. |
| 12 | `H/src/k6/w1/h_envelope.rs` | Complete 621-line source read: raw success/refusal and constructor facts, actual launch contexts, finite format grammar, stage windows, diagnostics and explicit reference pair. |
| 13 | `H/src/k6/w1/mod.rs` | Exports and scope documentation accurately distinguish conditional arithmetic from observations and caller qualification. |
| 14 | `H/tests/k6b_bin.rs` | Full new integration test logic read: exact prepass/file-normal context, changed unused UTF-8 path, half-cap boundary, computed/file refusal and no solve. |
| 15 | `H/tests/k6b_w1.rs` | All changed logic/literal rows inspected; 726 literal fields independently match H19 projection. Original semantic/storage checks preserved; inside-solve partial work meaningful. |
| 16 | `H/tests/k6c_envelope.rs` | Complete test file read; source-valid Uc witness, exact/upper populations, named phase and ownership regressions, full/short containment, widths/overflow and missing inputs. |
| 17 | `H/tests/k6c_h_envelope.rs` | Complete tests read; actual retained Strings/repeat digits, real constructor token and refused raw population/digit behavior. |
| 18 | `VR/Cargo.lock` | All delta inspected: seven local packages added, no registry source/checksum/version edits. |
| 19 | `VR/Cargo.toml` | Single local H dependency; existing default/seeded feature boundaries unchanged and no allocator registered in H library. |
| 20 | `VR/examples/vk_scale.rs` | All delta and live sequence read: full argv history, original loader/hash identity, capture/source drop, one shared estimator, checked JSON widths, counts continuation and unchanged backstop. |
| 21 | `VR/runner/test_vk_scale_runner.py` | All new tests read: binding caps/paths and fresh denominator, process-free plan, no fallback, approval/ascent held rows and positive controls. |
| 22 | `VR/runner/vk_scale_runner.py` | All delta/surrounding tier/admission flow inspected: pure holds precede capped binding; fresh numeric denominator; existing baseline and scale policies retained. |
| 23 | `VR/src/cases.rs` | Complete changed wrappers/parse factor and relevant source_parts read: original iterator item/content/order, fixed summaries, same external hash overlap and no added owned model graph. |
| 24 | `VR/src/envelope.rs` | Complete 1,725-line source read: checked signed arithmetic, loader/argv/context qualifications, Exact cost intervals, 15 caller phases, five joins, shared kernel consumption and local helper tests. |
| 25 | `VR/src/lib.rs` | Only additive envelope export; no solver or test-feature contract changed. |
| 26 | `VR/src/scale.rs` | Complete removal/re-export delta inspected; stale port absent, Counts/Sizes intact and separate from new math. |
| 27 | `VR/tests/k6c_envelope.rs` | All executable test logic read; complete 193+12 literal tables parsed as data and all 10,250 cells checked against retained sources, including path lengths and source-drop lifetimes. |
| 28 | `VR/tests/scale.rs` | Complete delta inspected; original storage equalities, strict field ordering, membership/cardinality193 preserved with explicit reference context. |

Supporting unchanged source was consulted only as needed for constructor/parser provenance, retained-kernel lifetimes, sparse validation, serialization and allocator registration. The semantic and whole-body proof of every unchanged dependency was not repeated. Complete historical acceptance/measurement records are outside this 28-file source review; the explicitly selected sealed evidence remains conditional and separately attributed.

