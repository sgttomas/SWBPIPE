# Independent KREV-01–05 repair backcheck

**Disposition: all five original findings are closed for this frozen module candidate; suitable for bounded manager fan-in.** No new blocking defect was identified in the complete repaired kernel/sparse diff. This permits integration work to continue; it does not qualify the complete M03 product, close N05/N06 physical accuracy, accept protected observations, or authorize release.

Same independent TASK executor `/root/solver_manager/kernel_review`, parent `/root/solver_manager`, delegated-harness-native, gpt-6-astra xhigh as dispatched. This is a resumed code-review assignment with the same full Root/TASK/Piping/LOOP_INIT and software-code-review basis. No role/model change, descendants, source/test edits, commits, Cargo/build/npm/native/browser execution. Only owned review evidence was written. The original [review](../RETURN.md), findings, failed logs and snapshots remain preserved.

## Exact candidate and executed evidence

[Source check and snapshots](_run_records/SOURCE_CHECK.json) bind all six files:

- frame common structural: `3a016181fc49092d2775cc365dd2956b4c137c1f6bb64046a77b925ef5f9cce0`.
- rigid-body geometry: `091d4069852c6033da38074af297c5eaa5a9469510c00d577f222e077aec2c15`.
- sparse structural: `89af5f9e72867072b0f9e2a54245592aa51745f1992ec20f118d20d1e13ae08f`.
- The two module-export files and sparse dependency manifest retain their original reviewed hashes.

Review covered every change from the preserved pre-repair snapshot, including the skyline algorithm moved into the common module, public API/lifetime changes, expansion arithmetic, intended-action gate and added tests. The unchanged source retains the original complete-diff review coverage. [Scoped validator](_run_records/scope_check.json) passes the six owned paths.

Parent execution, inspected in the actual raw logs:

| Run | Observed result | Source/chronology |
|---|---|---|
| Initial repair frame | 56 pass, 1 fail | Retained `KERNEL_INTEGRATION_CHECKS/_run_records/frame.log`; failure was analytical 0.5 versus computed 0.49999999999999994 asserted with bit equality. |
| Repaired frame | 57 pass, 0 fail; compile-fail doctest 1 pass | `frame_repaired.log`; parent explicitly confirmed this ran after the final comparison correction on 3a016181. |
| Sparse | 25 pass, 0 fail; no doctests | `sparse.log`; ran before the sole comparison correction. Production bytes are identical across that edit. |

The comparison correction preserves analytical reference 0.5, checks prescribed displacement exactly 1.0, and applies the existing 1e-9 relative analytical criterion to the free displacement. The rejection and intended-action checks remain unchanged. Replacing only those two comparison lines with the earlier exact-vector assertion reconstructs hash `6464e76112acbe9e674296e8665e38f3fc569a39e1cb4e889c7e295b5e3234ac`, independently proving there was no production change between the sparse run and final source. This is a correction to a new test's inappropriate exact floating-point equality, not a relaxation of protected numerical criteria or a changed expected result. [Input hashes and parent chronology](_run_records/INPUT_HASHES.json) bind the reviewed raw evidence.

My [independent repair probes](_run_records/independent_backcheck.py) ran using Python standard library only; [output](_run_records/independent_backcheck.json) is retained. They check source-arithmetic transcriptions against exact Fraction oracles and are **not Rust execution**:

- 165 addition sequences: every intermediate expansion sum equals the exact represented-input sum, including the reported lost tail and subnormal addition controls.
- 165 product cases: 162 accepted expansions equal exact represented products; three unsupported product-range cases reject explicitly.
- Six scaled false-witness cases are refuted by original-coordinate action; six valid oblique/origin/radix companions have zero exact action.
- Prescribed coupling controls include the consistent -1 case, a -1.125 mismatch below amplification 1, and the reported -2 mismatch; intended-action evidence detects both nonzero errors.
- The moved skyline factor/solve transcription recovers a nonconstant independently manufactured 8x8 NP-B solution in natural, reverse and even/odd order. Maximum absolute discrepancies are 0, approximately 4.45e-15 and 1.34e-15 respectively. This is bounded correctness evidence, not an executed Rust benchmark or a new acceptance tolerance.

Rerun only my lightweight evidence with `python3 <this-directory>/_run_records/independent_backcheck.py`. The parent's unit/doc commands remain parent-owned.

The subsequently supplied parent AFTER manifest was also read and checked: all six reviewed source hashes and the three relevant raw log hashes match, and its explicit chronology confirms the final frame rerun. It does not extend this review to the concurrent product/nonlinear changes.

## Findings closed and consequence checks

| Finding | Repair and independent backcheck |
|---|---|
| KREV-01: false physical geometry witness | Centered/normalized rows now generate candidates only. Final actions expand subtraction of original represented coordinates and exact products with rigid parameters; nonzero original constraint action cannot become an exact zero witness. Recovered node motions must equal that physical expression exactly. Range loss/inexact scalar recovery withholds the witness. Both reported defects and valid companions are covered in the passing Rust tests and independent exact action controls. Full-rank SVD remains a numerical screen, not proof of the complete stiffness rank. |
| KREV-02: ignored K_fc fidelity | Audit now includes delta(K_fc)*u_c in the reduced load perturbation; a changed zero reduced RHS rejects. Completion separately checks intended free action from complete contribution expansions against original loads and imposed values. Dense and sparse regression controls cover the reported failure and consistent companion. The new source-level check also catches a meaningful intended-action mismatch when its amplification is below 1; it cannot be excused by a zero K_ff perturbation. |
| KREV-03: erased low tail | A grow-expansion representation replaces the ordinary single low accumulator. All terms are retained in report arrays and in perturbation/action evaluation; high/low scalar fields are explicitly descriptive. Mantissa/FMA product decomposition and reversible radix scaling preserve admitted represented terms, with explicit range errors. Independent exact probes confirm the reported sequence retains 1e-16 and exercise cancellation/subnormal/range consequences. |
| KREV-04: witness helper bounds panics | PreparedSystem fields are private and borrow one original StructuralSystem. Public helpers validate source/maps, dimensions, finiteness and prepared symmetry before indexing; no separate conflicting original system is accepted. Passing catch_unwind tests cover internally corrupted rows, exponents and source maps. Source inspection confirms external callers cannot construct these malformed prepared states through the safe public API. |
| KREV-05: LU completion bypass | PositiveFactor's backend, source and evidence fields are private. Only checked positive Cholesky/skyline constructors return it; finish_structural takes that factor alone. It borrows the same immutable prepared system, with no closure, arbitrary label, pivot-vector or second-source acceptance seam. Both constructors reject N07; the passing compile-fail test refuses closure completion. Private completion still performs original-equation, condition and contribution-action gates. |

The common skyline constructor validates permutation and profile dimensions, rejects omitted represented coefficients, takes coefficients only from the prepared matrix and performs one positive screened LDL factorization. The sparse wrapper supplies RCM/profile structure, so the API change does not add a dense Cholesky fallback or relabel legacy sparse reports. Profile arithmetic, forward/diagonal/back solves and original/global index mapping were traced after the move. No legacy generic solver or raw protected metric implementation changed.

Opaque borrowing prevents a successful factor for one prepared matrix from qualifying a second matrix or force/boundary source. Expansion operations admit exact subnormal terms only where reversibility/provenance is retained; unsupported ranges remain errors. The original normalized residual formula, row count, factor reserve, refinement cap and named condition estimate remain unchanged. The intended-action gate is additional evidence with its own count/allowance, not a replacement of the original stored-equation residual.

## Limits retained

The new test passes and this review cover the initial kernel/sparse module repair, not all product/nonlinear adapter changes. Those source files were outside this review's write/diff scope and require their own frozen review and actual execution.

Full maintained N/R/NP dense/sparse/fallback observations, actual selected-contact state, richer transformation/projection/range cases, performance/storage observations, protected analytical 1e-9 and DEC-050/053 checks, clean DEC-025, actual-candidate CI, native consumer witnesses and professional/release boundaries remain with the parent. No previous failed run is erased.

N05/NP-A can still satisfy represented/intended residual screening while missing the required intended physical answer. N06 remains assembly-resolution unresolved without stronger recovery. Contribution-preserving solve/recovery or another independently validated stronger method remains required; these findings' closure does not close that work or authorize warning-only acceptance. No certified SPD/inertia, guaranteed forward digits, native execution or model/OS diversity is claimed.

