# Specification: DEL-12-01 Local-first storage and private data paths

## Scope

This deliverable specifies the local-first storage boundary and symbolic private data path conventions for project models, private rule packs, private material data, private component data, and related user-owned code/design-basis data.

This setup run is documentation production only. It does not implement storage, edit product source, choose a physical project package/container, create private paths, create real secrets or private data, add tests, or introduce cloud assumptions.

## Requirements

| Requirement ID | Requirement | Source Basis | Verification |
|---|---|---|---|
| LFSP-REQ-001 | The product storage posture shall be local-first by default for private project, rule-pack, material, component, and code/design-basis data. | SOW-029; OPS-K-PRIV-1; `docs/DIRECTIVE.md` §6 | Confirm the storage policy text states local-first default and private-data user control. |
| LFSP-REQ-002 | The public repository shall not be the default durable location for user-private project models, private rule packs, private material libraries, private component libraries, owner standards, credentials, or proprietary data. | OPS-K-IP-1; OPS-K-DATA-1; `docs/IP_AND_DATA_BOUNDARY.md` §§3,6 | Boundary review checks public/private path class separation. |
| LFSP-REQ-003 | Cloud storage, cloud sync, or cloud service transmission shall be out of MVP unless separately approved by the human project authority through governance or scope change. | SOW-029 notes; `docs/DIRECTIVE.md` §4.2 | Verify no cloud service is assumed in this deliverable's artifacts. |
| LFSP-REQ-004 | Storage conventions shall align with the architecture persistence baseline: versioned, unit-aware, provenance-preserving, schema-governed, migration-aware, round-trip testable persistence with canonical JSON/JCS-compatible hashes for JSON payloads. | AB-00-04; SOW-050; `docs/_Decomposition/SOFTWARE_DECOMP.md` §8 | Cross-check private path language against the deterministic persistence baseline. |
| LFSP-REQ-005 | The physical project package/container remains TBD and shall not be selected by this deliverable without human approval. | Acceptance/risk note; AB-00-04 | Verify every container/package reference preserves `TBD`. |
| LFSP-REQ-006 | Path conventions shall use symbolic path classes until implementation chooses OS-specific roots, application data directories, or project package structure. | Acceptance/risk note; setup write-scope constraint | Confirm no real user path, secret path, or environment-specific private data location is created. |
| LFSP-REQ-007 | Private rule-pack references shall preserve identity, version, checksum, source notice, and redistribution status without bundling protected formulas, code text, allowables, or proprietary values into public artifacts. | OPS-K-RULE-3; `docs/SPEC.md` §6; `docs/IP_AND_DATA_BOUNDARY.md` §§3,6 | Review rule-pack path references for metadata-only public handling. |
| LFSP-REQ-008 | Material, component, section, rule-pack, report, and project data crossing storage/import/export boundaries shall preserve provenance and redistribution/privacy status. | OPS-K-IP-2; OPS-K-DATA-3; OPS-K-UNIT-1; `docs/TYPES.md` §§7-8 | Future schema and adapter tests check provenance fields and unit-bearing values. |
| LFSP-REQ-009 | Storage diagnostics and result/report envelopes shall surface private/public boundary warnings without claiming certification, sealing, approval, authentication, or automatic code compliance. | OPS-K-AUTH-1; AB-00-06; `docs/SPEC.md` §§7-8 | Future diagnostics/report tests check warnings and professional-boundary notices. |
| LFSP-REQ-010 | Adapters, imports, exports, plugins, and private-library access shall not bypass unit checks, provenance checks, diagnostics, sandboxing, or public/private data-boundary controls. | AB-00-02; AB-00-07; OPS-K-PRIV-1 | Future adapter/plugin tests must exercise no-bypass behavior. |
| LFSP-REQ-011 | Tests for implemented storage behavior shall include private-path resolution, repository-leakage prevention, deterministic round-trip serialization, migration status handling, provenance preservation, and report/export boundary checks. | AB-00-08; `docs/SPEC.md` §§9,11 | Test files are not created in this setup run; future implementation must add them. |
| LFSP-REQ-012 | Missing storage choices, unresolved roots, and incomplete private-data handling shall be explicit `TBD`, warning, or finding states rather than silent defaults. | OPS-K-DATA-2; OPS-K-AGENT-1; `docs/DIRECTIVE.md` §3 | Review the deliverable for visible TBD/open issue entries. |

## Standards

No external engineering code, code clause, standards table, protected formula, material allowable, SIF/flexibility table, protected dimensional table, or proprietary catalog source is used or reproduced by this deliverable.

The controlling project sources for this setup run are the OpenPipeStress governance and decomposition artifacts listed in `Datasheet.md` and `_REFERENCES.md`.

## Verification

| Verification ID | Check | Expected Result |
|---|---|---|
| LFSP-VER-001 | Confirm `Datasheet.md`, `Specification.md`, `Guidance.md`, and `Procedure.md` exist. | Four-document kit is present. |
| LFSP-VER-002 | Validate `Dependencies.csv` with `tools/validation/validate_dependencies_schema.py`. | Schema valid with all 29 v3.1 columns. |
| LFSP-VER-003 | Confirm `_SEMANTIC.md` audit result is PASS and `_SEMANTIC_LENSING.md` has complete matrix coverage for A, B, C, F, D, X, and E. | Semantic setup gates pass. |
| LFSP-VER-004 | Search deliverable artifacts for disallowed claims or real secret/path examples. | No storage implementation, real secret, cloud assumption, protected data, or certification claim is introduced. |
| LFSP-VER-005 | Confirm `_STATUS.md` is `SEMANTIC_READY` only after the setup sequence and validation pass. | Lifecycle state is consistent with setup evidence. |

## Documentation

Required setup artifacts for this run:

- `Datasheet.md`;
- `Specification.md`;
- `Guidance.md`;
- `Procedure.md`;
- `_SEMANTIC.md`;
- `_SEMANTIC_LENSING.md`;
- `Dependencies.csv`;
- `_DEPENDENCIES.md`;
- `_run_records/*`;
- `_STATUS.md`.

Implementation artifacts deferred by this run:

- storage policy files outside this deliverable folder;
- source code or schema edits;
- executable tests;
- physical project package/container;
- real private paths, secrets, credentials, or private data.
