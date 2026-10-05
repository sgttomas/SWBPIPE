# I61 receipt experiment: mismatch ledger

Each entry is classified as an **emitter bug** (fixed in the experiment), a **producer gap** (a
missing typed fact) or a **contract tension** (escalated to ROOT). Every entry also says whether
decisions D19–D30 address it; none do.

**Paths:**
- PP = P/core/product_physics/src at NUM `c817a86cb1`.
- PY, RS and TS are the readers at READER `b36739112a`: PY is `P/core/analysis_runs/retained_precision.py`; RS and TS are the Rust and TypeScript reader sources.

## Contract tensions (escalated; ROOT decides)

### T1. A retained-selected case also carries the legacy source-unavailable disclosure (G4)

**Where it fails:** the milestone receipts in both modes, at G4 `RETAINED_PRECISION_DIAGNOSTIC_MISMATCH`, in Python (PY:1617), Rust and TypeScript alike. This is the first failure of the experiment's main line.

- **The producer fact.** The actual ordinary route attempts the legacy bounded source-block method for this request. It fails at source closure, `RecoveryFailure { stage: "source closure", helper_stage: SourceClosure, error: Unsupported("actual transform is not a signed permutation"), work: WorkReport { charged: 46628, rejected: 0, limit: 4000000 } }`. The route emits `diagnostic:source-recovery:case` (code `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`, affected_refs `["case"]`).
- **The reader rule.** C1's G4 row (I32 WIRE_CONTRACT:147) forbids any "source-unavailable naming a retained-selected case".
- **The conflicting contract text.** C2 (I32 CONTRACT_DELTA:160) gives `legacy_source = {disposition: …|unavailable, diagnostic_ref, work_ref}` and says "the original diagnostic_ref preserves original source failure disclosure". It also adds `legacy_source_work[]` for the WorkReport. A selected successor case whose ordinary route attempted and lost the legacy method therefore cannot satisfy both texts.
- **Counterfactual (probe B; not a reading).** Probe B removes that one diagnostic from the selected case and emits `legacy_source {unavailable, diagnostic_ref: null, work_ref: null}`. All three readers then accept G4–G7 once the emitter bug E1 below is fixed. They first fail at G8 (T2).
- **The options for ROOT:**
  - (a) the successor facade suppresses the legacy disclosure on a retained-selected case, and `legacy_source` carries the work through `work_ref` → `legacy_source_work[]`;
  - (b) amend C1's G4 so that a source-unavailable diagnostic may name a retained-selected case;
  - (c) a routing rule under which the legacy method is `declined_without_attempt` when W1 is eligible. This changes ordinary bytes, which the coexistence rule may forbid.
- **D19–D30:** not addressed.

### T2. The milestone request is model schema 0.1.0, which reader G8 refuses

**Where it fails:** G8 `RETAINED_PRECISION_INVOCATION_MISMATCH` in all three readers, reached only under probe B.

- **The reader check.** PY:1353, RS:3278 and TS alike require `model.schema_version ∈ {"0.2.0","0.3.0"}`.
- **The input.** The milestone input `P/fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json` is `"schema_version":"0.1.0"`, and the producer accepts it and certifies it in both modes.
- **The basis.** I found no basis for the 0.2.0/0.3.0 rule in C1, C2 or C3 by text search.
- **Counterfactual (probe C; not a reading).** Probe C sets `schema_version` to `"0.2.0"` before parsing, on top of probe B. All three readers then **pass G0–G8** in both modes with standing `needs_recompute`.
- **The options for ROOT:**
  - re-author the milestone request at 0.2.0, which changes its bytes and invocation digest and needs the owner's milestone identity;
  - admit 0.1.0 at G8, giving its basis.
- **D19–D30:** not addressed.

### T3. A one-case unavailable invocation has no successor publication, and the producer is one-case only

**Where it fails:** the refusal receipt (preparation refusal, sparse mode) fails at G3 `RETAINED_PRECISION_COVERAGE_MISMATCH` in all three readers (PY:1572: "any selected").

- **The readers are faithful here.** C1 makes a successor publication conditional on a selected case.
- **The producer limit.** The private prepared driver admits exactly one load case and no combinations (PP/retained_product.rs:3030, `prepared one-case/no-combination source scope`).
- **The consequence.** Real producer output cannot yet put an unavailable case into a publishable receipt. D9b's explicit null, the unavailable branch and D19 stay synthetic-only.
- **What the refusal receipt still shows.** Its wire shape is sound: 0 schema violations; it passes G0, G1 (the receipt and publication hashes; it has no CaseSource, so no source or preparation hash) and G2. It carries `source_ref: null`, D19's `prepared_product_failure`, and the reason `(source_unavailable, preparation)`.
- **The decision for ROOT:** whether multi-case prepared support enters the receipt transaction's scope.
- **D19–D30:** the receipt exercises D19's shape only; nothing in the round addresses publishability.

## Emitter bugs (fixed during the experiment)

- **E1.** `selection.absolute_verified` was sourced from the native `RetainedEvidence.absolute_verified`. Those are kernel QuantityIds, including `Reaction(Dof)` rows that have no one-to-one published row. The result was G5c `CLASSIFICATION_MISMATCH` in all three readers (probe B, iteration 1).
  - **Fix:** take it from the certificate's per-row `CertifiedProductProof::verdicts()` class `AbsoluteVerified{bound_bits}`, in published row order.
  - **Lesson for the serializer:** `absolute_verified` and `not_covered` are facade-level (published-row) facts.
  - The readers' classifications then match the producer's certificate verdicts for all 98 rows (sparse) and all 99 rows (dense).
- **E0.** Before any reader ran, the emitter's ordinary mapping assumed there was no source-block diagnostic. It now maps `legacy_source` from the actual `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` diagnostic, which is what surfaced T1.

## Producer gaps (no reader failure; the real serializer must fill them)

- **G-a. The envelope transformation has no producer code.** The emitter applies the successor semantic id and profile id, the `recovery_method` row token, and the `RETAINED_PRECISION_*` diagnostic, whose id, message and severity are emitter text. `formulation_basis.limitations` were left as produced and passed G0 and G7.
- **G-b. The Ordinary members have no typed capture.** `initial` is read back from `numerical_quality` (`solve_quality`, `evidence_refs`). `legacy_source` is read back from the diagnostic. `w2` and `formation` are emitted as `not_triggered` and `null` after checking that no W2 or K-D5 diagnostic exists. `legacy_source.work_ref` is null because the WorkReport is not captured; C2 expects `legacy_source_work[]`.
- **G-c. D6a attribution (assumption A2).** The emitter takes the diagnostics naming the case: three load-category diagnostics, the source-unavailable diagnostic and the integrity diagnostic. The invocation-level `RULE_CHECK_INPUTS_MISSING` (no affected_refs) is left unattributed. The readers accept this; they check uniqueness and resolution, not completeness.
- **G-d. `constraints[].support_indices` are derived by the emitter** from support node and restraint; the producer has no map.
- **G-e. `selection.not_covered` is emitted empty.** The producer has no NotCovered class. The readers agreed for this case.
- **G-f. The definition and semantic-table fixtures come from READER (assumption A4).** NUM does not carry them; the formation-definition hash still matches.
- **G-g. `retained_state_sha256` (assumption A1) is not checked by any reader.** `ledger_sha256` follows C2:91. Neither is rederivable by a reader.
- **G-h. C1 §2's upstream no-wrap admission premise is not established.** Counters were emitted as exact, and no reader can check saturation.
- **G-i. Several closed translations are unimplemented and unreached.** Reason, Stop, Refusal, BridgeError, ProductError, PublicError (other than preparation), the group-refused and bound-refusal paths have no translation; the receipts contain no `UNMAPPED` placeholder.
- **G-j. Indices are untested beyond one case.** The run, source, product and ordinary indices were assigned by the emitter for a one-case invocation.
- **G-k. The proof↔owner binding is by custody only** (RV77-N4), unchanged.
