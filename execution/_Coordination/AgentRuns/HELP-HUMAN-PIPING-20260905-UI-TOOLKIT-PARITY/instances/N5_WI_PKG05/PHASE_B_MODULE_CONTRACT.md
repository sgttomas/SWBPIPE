# Phase B module contract v1 — held for parent wave release

Parent design acceptance: parent message 2026-09-05, accepting independent PASS_DESIGN_ONLY in instances/N_CROSS_CONTRACT_REFUTE/RETURN.md. This accepted design dependency is derivative evidence; original accepted decomposition is revision 0.12/SCA-009 and DAG-010. Source implementation remains held until parent accepts existing-capability B1/B2/row17. No implementation implied.

## Frozen core shape

New module core/product_physics/src/self_weight.rs, single source writer Agent2. Only named parent/N3 integration owner adds `pub mod self_weight;` to lib.rs. No dependencies, solver changes, Cargo manifest, shared lib.rs edits, UI, operation_applier, or authority updates by this child. Focused unit tests live in new module. Parent must provide module export before normal cargo tests can discover it.

Pure public generator: `generate_self_weight_operations(model: &PreviewModel, request: &SelfWeightRequest) -> Result<SelfWeightOperationPlan, Vec<Diagnostic>>`.

SelfWeightRequest is Deserialize and contains:
- case_id: String, label: String, pipe_refs: Vec<String>;
- gravity: {value: f64, unit: String, axis: String}; axis global_x/global_y/global_z only, value finite/nonzero/signed;
- provenance: String (explicit request source);
- source_model_hash: String (caller-supplied canonical SHA256 lowercase hex; generator preserves but cannot authenticate typed-view hash).

Plan is Serialize and contains source_model_hash, ordered changes, source_evidence, scope_label='selected_pipe_mass_only'. Every change has object_type='Load', target_ref=case_id, operation_kind='create', change_kind, field_label, field_path, before='not_present', after=serialized JSON string, unit, dimension, source_note. This is an operation draft, not a full accepted/applied operation envelope. Caller supplies operation/change IDs, author_type and user/session source metadata under PKG16; generated content never impersonates a user or reports approval. Emit case shell followed by loads sorted by exact pipe ID. No timestamps or ambient counters.

Case after: {id:case_id,label,kind:'primitive_user_load',status:'draft',provenance:request.provenance,primitive_loads:[]} with change_kind=create_load_case,path=load_cases,unit=none,dimension=dimensionless.
Load after: {id:generated_id,category:'distributed_force',target:{type:'element',pipe:pipe_id},direction:gravity.axis,magnitude:{value:mass_kg_per_m*normalized_gravity,unit:'N/m'},dimension:'force_per_length',provenance:serialized evidence} with change_kind=create_primitive_load,path=primitive_loads,unit=N/m,dimension=force_per_length. Do not emit category weight.

Generated id `load:self-weight:<case byte length>:<case_id>:<pipe byte length>:<pipe_id>` is injective without lossy sanitization. Validate exact id uniqueness against all existing primitive load ids and case IDs. Reject empty/whitespace identifiers, duplicate selection, ambiguous selected pipe IDs, existing case, any generated collision. Preserve exact nonempty IDs rather than trimming into a different reference.

Source evidence for each load and source_note includes method='pipe_mass_per_length_times_explicit_axis_acceleration/v1', original request gravity/value/unit/axis, request provenance, pipe ID/provenance, optional section_ref and referenced section source data/provenance, all original mass input quantities, normalized mass_kg_per_m, and explicit absence indicators for optional contents and insulation. Evidence is serialized into supported provenance/source_note strings, not unsupported JSON keys that consumers discard. Full original source document stays in caller; never reconstruct it from PreviewModel. Parent batch transport preserves draft source_note and payload provenance.

## Referenced sections and normalization

Clone the typed model and project its pipe_segments to uniquely resolved selected pipes only, sorted. Preserve the original section table so duplicate referenced section IDs can be detected. Call existing `resolve_shared_sections` on that selected-pipe clone before weight generation. Any diagnostics block the whole plan, including stale inline caches; do not bypass the N3 reference policy or silently substitute newer section dimensions. Retain original typed pipe inputs for evidence before resolution. Validate missing/duplicate endpoint refs against original node collection, same endpoint references, finite endpoint positions and zero geometric span; no inference of connectivity.

For each selected resolved pipe, normalize cloned OD/wall/mill tolerance/insulation thickness as Length, material/contents/insulation densities as Density using existing normalize_quantity; gravity as Acceleration. Explicitly validate finite values, positive material density and OD/wall/effective wall, nonnegative supplied mill tolerance/contents density/insulation thickness/density, paired insulation presence, nonnegative bore. Require explicit nonblank pipe provenance and request provenance; referenced section provenance must also be retained/required when the source model supports it. Missing mass-required values fail closed. Optional absent contents/insulation remain absent and are honestly enumerated; no material catalog lookup or substitutions. Reuse compute_pipe_mass_per_length, then reject nonfinite/nonpositive mass or nonfinite resultant intensity. Unrelated invalid supports/materials/loads must not block this pure planning stage; normal operation engine still validates original model boundary.

## Integration dependencies

PKG16 applies all drafts to the original untyped document via its same-engine atomic batch: validate original canonical hash once, sequentially validate/apply each operation to private cloned state, publish final model only when all succeed. Stale/midbatch failure yields no session mutation/checkpoint. PKG07 consumes plan, shows selected-pipe-only mass scope and absent optional sources, requires explicit gravity/axis/source entry, and submits one batch with one undo checkpoint. These are dependencies, not implemented by module child.

## Acceptance test matrix

1. Valid invented metal-only case: explicit positive and negative acceleration on each axis yields analytically computed line load; no default g.
2. SI and alternate supported entered unit equivalent inputs yield same N/m within documented numeric tolerance; stored model/request untouched.
3. Explicit contents, insulation and mill tolerance contribute according to existing helper; optional absence enumerated.
4. Missing density/unit/provenance, incompatible dimension, nonfinite/overflow/negative density or tolerance, incomplete insulation, invalid effective wall/bore and zero gravity reject with no partial plan.
5. Empty/duplicate/missing/ambiguous selected pipes or nodes, zero span, case/load ID collisions reject; punctuation/Unicode references retain injective generated IDs; reordered selection yields deterministic exact output.
6. Bound section equivalent dimensions produce same result; missing/duplicate/unsupported reference and stale inline cache block. Unselected broken section does not block.
7. Exact output shape matches current create_load_case/create_primitive_load; no weight token; source evidence survives serialization and is complete.
8. No changes to support or existing loads/source document; operation drafts do not embed approval or user impersonation.
Parent PKG16 integration tests additionally prove successful batch, failure after shell/later load rollback, stale hash and one checkpoint. These cannot be claimed by pure module tests.

Fresh Agent2 source review over 100% frozen diff follows implementation and local tests. Parent runs registered piping-pytest/evidence-sweep and harness-self-check with broader tranche. Rerun design/source review if normalization/reference or batch contract changes.
