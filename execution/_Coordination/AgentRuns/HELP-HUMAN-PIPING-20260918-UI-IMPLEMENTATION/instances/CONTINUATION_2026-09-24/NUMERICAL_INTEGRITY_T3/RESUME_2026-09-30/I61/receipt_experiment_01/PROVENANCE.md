# I61 receipt experiment: field provenance map

**Sources:**
- **Emitter:** `_run_records/emitter_i61_receipt_probe.rs`, compiled in a NUM `c817a86cb1` archive copy as a `#[cfg(test)]` module (`_run_records/archive_lib_rs.diff`).
- **Producer route:** `PreparedCase::prepare_observed → solve_native → project_candidate → typed_trace`, for RF-SKEW-T-CANT-OFF-122-r1e-04.

**Key to the Class column:**
- **PRODUCER:** an actual producer fact.
- **DERIVED:** computed by the emitter from producer facts.
- **GAP:** read back from envelope text, because no typed producer fact exists.
- **CONTRACT:** a fixed contract constant.
- **ASSUMPTION:** a ROOT-accepted provisional reading.
- **TEXT:** emitter-chosen text.

| Receipt member | Class | Source |
|---|---|---|
| body.receipt_version … canonicalization | CONTRACT | constants of C1 §3/§4 (equal to the readers' G0) |
| body.invocation | PRODUCER | `source_receipt::CapturedInvocation::parse` digest (`source_blocks_invocation_v1` over `{request, solver_mode}`) |
| body.publication_sha256, receipt_sha256 | PRODUCER | `canonical_json_checked_v1_text` + sha2, over the final envelope without the member, and over the body |
| body.work | PRODUCER | `RecordedInvocation::new(60e9)`, `CaseLimit(20e9)` (PP/retained_product.rs:3282–3283); `charged` = `meter().charged()`; execution_order is the one case |
| cases[].run | PRODUCER + DERIVED | `RecordedInvocation.runs()[case.run]` (origin, cache snapshots, record build links, RunWork); physical records from `RetainedEvidence.attempts`. Logical attempts come from the emitter's C1 §1 projection (`RP-LOGICAL-ATTEMPTS-v1`) |
| cases[].selection | PRODUCER | `RetainedEvidence` (stop_rule, floor_ratio, body_scales, input_derived_dofs, pivot/rcond/residual, resolution_scale, estimate, charge, theta, certified_bound, floor); `ledger_sha256` = SHA256(K4LED) per C2:91 |
| cases[].selection.retained_state_sha256 | ASSUMPTION A1 | SHA256(`retained_state_encoding`), C1 §3 K4RST; no reader checks it |
| cases[].selection.section_terms | PRODUCER | `ProductCapture.facts` (area, Z) + new operational (length, axial, torsion) |
| cases[].selection.absolute_verified | PRODUCER | `CertifiedProductProof::verdicts()` AbsoluteVerified, in row order (E1 fix) |
| cases[].selection.not_covered | GAP | emitted `[]`; the producer has no NotCovered class |
| cases[].source_identity_sha256 | PRODUCER hash | H(`retained_precision_source_mp_v2`, CaseSource without `index`) |
| sources[].id_maps | PRODUCER | `ProductCapture` nodes, members, spring_map and supports, joined with `owner.source()` |
| sources[].body_membership, layout | PRODUCER | `PrimitiveSource` body maps; `recover::layout(owner.source())` |
| sources[].stations, supports | PRODUCER | `PrimitiveSource` |
| sources[].constraints[].support_indices | DERIVED | supports covering the constrained DOF |
| sources[].nodal_terms | PRODUCER | `PrimitiveSource` loads + `ProductCapture.terms` (canonical → original) |
| sources[].section_terms (+geometry) | PRODUCER | `ProductCapture.facts` + new operational |
| sources[].kernel_source_sha256, stiffness_sha256 | PRODUCER | SHA256 of `SourceOrigin.identity` (K4SRC) and `.stiffness` (K4STF) |
| sources[].preparation.sha256 | PRODUCER hash + ASSUMPTION A4 | H(`retained_precision_preparation_v1`, …); definition_sha256 from READER's definition fixture |
| material_bases[] | PRODUCER | `ProductCapture.materials` and `members[].material`; base selector from the actual request |
| calls, groups, builds | PRODUCER | `RecordedInvocation.calls()`, `groups()`, `builds()`; a group's source_refs are the runs naming it |
| ordinary_attempts[].initial | GAP | `numerical_quality.cases[].solve_quality` and `evidence_refs` |
| ordinary_attempts[].w2, formation | GAP | `not_triggered`/`null` after checking for the absence of W2 and K-D5 diagnostics |
| ordinary_attempts[].legacy_source | GAP | the actual `SOURCE_BLOCK_RECOVERY_UNAVAILABLE` diagnostic; `work_ref` null (no WorkReport capture) |
| ordinary_attempts[].diagnostic_refs | ASSUMPTION A2 | the diagnostics naming the case (D6a) |
| product_attempts[] | PRODUCER | `retained_receipt::PreparedAttemptView` (stages, checks, proof trace with summary_coverage, preparation members/work/conversions, adapter, operational old/new, overlay/g5a work); indices are the emitter's |
| refusal: product_attempts[].result.error | PRODUCER | `PreparedCaseFailure.capture.error` and `.preparation_error` |
| refusal: cases[].reason | CONTRACT | `(source_unavailable, preparation)`, `prepared_product_failure` (D19); `source_ref: null` (D9b) |
| envelope producer/profile/recovery_method | CONTRACT (table) | READER semantic table; no producer constant |
| envelope RETAINED_PRECISION_* diagnostic | TEXT | the id `diagnostic:retained-precision:<case>`, message and severity are the emitter's |
