# Phase B work graph draft — held for Phase A parent acceptance

Authorizing direction: parent relay 2026-09-05. Scope PKG07 implementation attributed to DEL0701/02/03, not DEL0709. D58 gate and roadmap held. This plan is derivative to accepted rev0.12/SCA009/DAG010.

B0 UI contract and palette: one Agent2 owns new toolkit capability catalog/palette plus App.tsx integration and viewport tool labels. Organize routing/materials/supports/loads/review/display in familiar language; use structured metadata internally, never raw implementation field paths in primary UI. Capability catalog has supported/partial/unavailable/gated states from exact executable engine and UI surfaces. Existing supported commands focus/open actual forms. Unavailable actions disabled with specific reasons. App.tsx and src/types.ts serialized single integration owner.

B1 existing entity edits/deletion: after PKG16 reviewed B1 return, one Agent2 owns PropertyInspector and ModelTree integration, deletion intent helper and focused tests. Exact operation contract: Section set_field name, section_type (pipe only), provenance, properties.outside_diameter.value, properties.wall_thickness.value; quantity metadata preserved. Deletes op_kind delete, change_kind delete_material/delete_section/delete_component, targets Material/Section/Component, field_path materials/sections/components, before RFC8785 canonical JSON entire entity, after deleted, unit none/dimension dimensionless. Hash guards and inbound-ref blockers/no cascade. Build before-state using shared canonical helper; do not approximate JSON.stringify ordering. Section assignment remains held.

B2 support/material/wind existing-capability authoring: after PKG16 frozen fields, detached modules can be produced by disjoint children, but PropertyInspector wiring stays one integration owner after module fan-in. Explicit engineering inputs, typed units/provenance, queue/validate/preview/apply, no defaults. Hangers/nonlinear within inspector; no new deliverable or dedicated panel.

B3 Tier3: after existing-capability wiring accepted, compose frozen PKG02/03/05/16 contracts. Geometry initially straight unreferenced runs, explicit IDs/axes/angles/origin, attached-entity blockers. Hanger manual user-imported record selection no sizing. Self-weight explicit selected pipes and gravity value/unit/axis emitting standard ordered changes. Display units storage-neutral. Exact contracts awaited.

Review: every source tranche frozen and fresh read-only TASK/software-code-review before sweep; parent owns dependency provisioning and integrated check/publish lane. Child tests focus behavior and shared-engine route; no concurrent root dependency installs or Git mutation. Each child gets immutable brief, status/return, start/finish telemetry; manager accepts source/evidence before dependants.

## Parent contract relay — B2 and section reference

Support.configuration canonical projection of existing family/restraints/stiffness/hanger/nonlinear/provenance, omit absent; after explicitly replaces keys and omitted optionals clear. create_support uses same rich payload. Material.temperature_points before canonical current array or not_present if absent, after array. Wind exposure before/after canonical {exposed_pipe_refs:[],exposed_spans:[]} with both arrays required. All envelopes none/dimensionless; nested quantities preserve units. Structured forms only, no JSON-only solution.

N3 section_ref V2 accepted: optional pipe.section_ref string, model.sections default empty; inline OD/wall is materialized cache while bound; shared section edits update all refs atomically; direct bound dimensional edits block until explicit detach; supplemental per-span fields preserved. UI labels must clearly distinguish shared referenced section from local dimensions. Wait N2 assign/detach resolver implementation and review.

Atomic operation batch required for self-weight and applicable geometry, not sequential UI commits. N2 implementation awaited. Existing hashService.canonicalJsonString async delegates to Rust/Wasm; new entity delete must capture immutable draft and render busy/error states across canonicalization; never substitute JS canonicalization.

Environment parent-provisioned: npm available; CARGO_TARGET_DIR={TEMP_ROOT}/piping-ui-parity-target; Python PATH={TEMP_ROOT}/piping-ui-parity-py313/bin:$PATH. Wasm rebuild only after N2 source completion, coordinated with parent.

## Parent relay — reviewed shared section model and frozen B3 wire

N3 snapshot instances/N3_WI_PKG02/SECTION_ACCEPTED_SNAPSHOT_V1.json; 108 tests and fresh review PASS. Operation wire N2: operation_kind modify, change_kind assign_section/detach_section, target Element pipe ID, field_path section_ref, before canonical full pipe, assign after {section_ref:id}, detach after {source_section_ref:currentId}, envelope none/dimensionless. Shared section scalar edits update all bound caches atomically. Await N2 reviewed implementation before UI activation.
