# Cross-contract design refutation

Role: direct ephemeral Agent 2, parent HELP_HUMAN. Actual inherited model identifier unavailable. No delegation, source edits, or tests. This is derivative design-review evidence against parent-declared accepted SCA-009/revision 0.12/DAG-010 and captured source hashes, not decomposition truth or implementation acceptance.

## Verdict

PKG03: REVISE before integrated selection implementation. One concrete blocking contract omission.
PKG05: PASS for the proposed bounded interface; no design blocker found. Existing transactionality and source-binding integration obligations remain mandatory, not satisfied by this verdict.

## C1 — Existing generic stiffness can override the selected hanger

Evidence: core/product_physics/src/lib.rs:7550, support_stiffness_input, resolves support.stiffness before support.hanger.stiffness. PreviewSupport at line 313 permits both. Contract points 5–6 allow selection onto an existing support but only prohibit merging missing values from previous selections; they do not define the conflicting top-level generic stiffness disposition.

Counterexample: existing support.stiffness is ux / 2000 N/m; selected record.hanger.stiffness is uy / 1000 N/m. Updating only hanger preserves the old generic stiffness, so actual solve consumption uses ux / 2000 despite selected imported uy / 1000. Changing hanger families can similarly retain incompatible generic/nonlinear data.

Minimum correction: declare an explicit existing-support compatibility preflight. If top-level stiffness or incompatible family/nonlinear context is present, either reject selection with an actionable conflict diagnostic, or require explicit user-authorized replacement clearing the conflicting fields atomically with installing the chosen hanger. Do not silently alter restraints or overwrite unrelated support context. Pin a regression proving the consumed DOF/value corresponds to selected record or the entire operation is rejected unchanged.

## Mandatory implementation checks retained from the proposals

- PKG03 quantities: restrict hanger stiffness DOF to ux/uy/uz for the stated force_per_length contract; product validator expects rotational stiffness for rotational DOFs (validation.rs:904). ImportedQuantity magnitude/value adapter is necessary because product Quantity is {value,unit}, not the library wrapper (lib.rs:177).
- PKG03 provenance: imported records and each numeric value need all seven provenance fields and nested disposition checks. Existing generic validate_nested_values only checks metadata keys and existence of a provenance object; it does not ensure nested fields or quarantine/public-private decisions. The proposal explicitly requires tightening the hanger path, so reusing the old traversal without those additions would violate it.
- PKG03 durable provenance integration remains explicitly unresolved in proposal point 6. Product Quantity has no per-value provenance field and PreviewSupport provenance is a string; a frozen operation/persistence evidence location must retain library+record identity, complete selected payload, and per-value provenance before end-to-end selection is accepted. Unsupported keys cannot serve as evidence since consuming deserializers can ignore them. This is a declared prerequisite for PKG16, not evidence that it already works.
- PKG05 distributed_force/element/global-axis/force_per_length payload agrees with resolve_create_primitive_load at operation_applier/src/lib.rs:4141. The resolver accepts compatible N/m even when the project force/length basis differs (line 4418), as long as outer change.unit and payload magnitude.unit match and project unit metadata exists.
- PKG05 selected-pipe normalization must call normalize_quantity for each relevant explicit section quantity before compute_pipe_mass_per_length (product_physics lib.rs:4535 and 4743). Existing normalizer covers densities as Density and mill tolerance as Length, consistent with the proposal. Full-model normalization is unnecessary for generation.
- PKG05 derived weights preserve request/pipe source provenance and absence summary. Do not regenerate the whole source document from PreviewModel: that typed consumer is lossy and its Project only retains id. PKG16 must apply output operations to the original document, retaining all unmodified data and references.
- PKG05 caller-bound model snapshot/hash, unique case/load IDs, sorted pipes and all-or-nothing private working-copy application are appropriate. Current engine exposes individual operations; whole-plan application remains a required integration dependency. Test a failure after shell creation and at a later load, proving zero session/checkpoint mutation, along with stale-plan rejection and unit equivalence.

## Handoff

Design review complete; PKG03 C1 remains open for manager amendment and focused backcheck. PKG05 may proceed after parent interface/write-fence acceptance with the obligations above. This does not replace the required fresh frozen-source software-code-review or profile tests. Rerun on amended proposals, consumer/source drift, or changed transaction/evidence contracts. No authority pointer, lifecycle state, scope, or external-agent runtime changed.
