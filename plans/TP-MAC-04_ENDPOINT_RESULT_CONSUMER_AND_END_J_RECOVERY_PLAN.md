---
doc_id: TP-MAC-04
doc_kind: implementation.plan
status: implemented
created: 2026-05-10
closed: 2026-05-10
---

# TP-MAC-04 Endpoint Result Consumer And End-J Recovery

## Purpose

Complete the first endpoint-result tranche after TP-MAC-03 by making both
straight-pipe element ends inspectable in the product preview workflow.

This tranche adds endpoint-j local force/moment results from the existing
12-DOF straight-pipe recovery vector and gives the desktop result detail panel
a concrete endpoint-pair consumer. It preserves existing end-i result IDs and
does not start intermediate station recovery, shear recovery, pressure-to-frame
load conversion, rule checks, or professional approval workflows.

## Scope

- Add end-j local force and moment preview result items for axial force,
  torsional moment, bending moment about local y, and bending moment about
  local z.
- Preserve existing end-i result IDs such as
  `result:force:pipe-P-120:axial`.
- Add stable end-j IDs such as
  `result:force:pipe-P-120:axial:end-j`.
- Use existing `ResultItem.metadata` semantics with `location: end_j`,
  `coordinate_system: element_local`, and explicit j-end sign-convention text.
- Extend the desktop result interpretation detail to show paired end-i/end-j
  values when a local endpoint force or moment result is selected.
- Surface end-j endpoint context in report-packet selected result refs and
  DEL-14-02 analysis-run hashes through the existing result envelope path.
- Update the mechanics gap ledger so endpoint-j local force/moment preview
  recovery is implemented while intermediate station recovery remains deferred.

## Boundaries

- No schema enum change is required; `schemas/results.schema.yaml` already
  permits `metadata.location: end_j`.
- No existing result ID is renamed or removed.
- No shear components are added in this tranche.
- No intermediate station sweep or stress-station expansion is added.
- No protected standards content, allowables, SIF/flexibility tables, private
  criteria, compliance result, certification, sealing, approval, release, or
  production-readiness claim is introduced.

## Acceptance Criteria

- The computed preview result envelope contains end-j local force/moment result
  IDs for each straight pipe.
- Selecting `result:force:pipe-P-120:axial:end-j` in the desktop results panel
  shows `axial_force`, `element_local`, `end_j`, j-end sign-convention text,
  `pipe:P-120`, and the paired end-i axial result.
- The mechanics gap ledger lists endpoint-j local force/moment preview recovery
  as implemented, not as a compliance failure.
- The report packet includes the deterministic end-j axial result ref when it
  exists.
- DEL-14-02 analysis-run records include result-value hashes for the new end-j
  result items.

## Closeout

TP-MAC-04 is implemented for the current product preview workflow.

Delivered:

- end-j local force/moment result emission in `core/product_physics`;
- regenerated computed fallback fixture with end-j endpoint result items;
- desktop endpoint-pair interpretation and display for selected local endpoint
  force/moment rows;
- endpoint-j selected-result refs in desktop and service report-packet context;
- analysis-run, schema, service, Rust, and desktop test coverage for end-j
  result metadata and hashing;
- coordination handoff updates pointing the next agent instance at this closed
  TP-MAC-04 state.

Final verification was run on 2026-05-10:

- `npm run generate:product-preview-mechanics`
- `cargo fmt --manifest-path core/product_physics/Cargo.toml`
- `cargo test --manifest-path core/product_physics/Cargo.toml`
- `python3 -m pytest tests/product_preview/test_product_preview_service.py tests/test_analysis_run_records.py tests/test_results_schema.py`
- `python3 tests/test_results_schema.py`
- `npm test --workspace apps/desktop`
- `npm run build --workspace apps/desktop`
- browser smoke against `http://127.0.0.1:5173/`
- `git diff --check`

Intermediate station recovery, shear recovery, endpoint stress-station
expansion, pressure-to-frame load conversion, governed calculation-report
generation, and protected rule/code checks remain deferred for future governed
plans.
