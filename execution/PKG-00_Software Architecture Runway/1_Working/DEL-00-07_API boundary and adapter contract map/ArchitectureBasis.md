<!-- chirality-architecture-basis/v1 -->
# DEL-00-07 — API boundary and adapter contract map — Architecture Basis

Classification: `ARCHITECTURE_BASIS_REFERENCE`
Consolidates: `Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md` (four-document kit, consolidated 2026-07-15 per piping decision D-43; source bytes hash-bound in the package `CONSOLIDATION_MANIFEST.md` and preserved in git history)
Authority basis: `execution/_Decomposition/SOFTWARE_DECOMP.md` revision 0.9, architecture-basis row AB-00-07
Dependency direction: one-way — project packages depend on this member; this member depends on no package deliverable (HUMAN-STEER-PKG00-EXCLUSION-001)

## Purpose

Internal/public API boundary map and adapter/plugin governance constraints for the software architecture runway (scope SOW-062, objective OBJ-013, type API_CONTRACT). This member defines API and adapter contracts only; it does not implement APIs, plugins, importers, exporters, schemas, or integration formats. Per the PDU-054 current declaration (D-41 R5 T7), the API, plugin, and adapter boundary declarations and their gates now exist; runtime adapter/plugin breadth is a separate implementation scope and is not implied by this architecture basis. Architecture roles covered: internal API boundary, public API boundary, adapter contract, plugin governance map, private-library boundary.

## Normative requirements

| ID | Requirement | Evidence |
|---|---|---|
| REQ-07-01 | Define which boundaries are internal implementation contracts and which may become public extension APIs. | Acceptance review |
| REQ-07-02 | Require adapters to validate units, provenance, redistribution status, diagnostics, and private/public data boundaries before data enters core workflows. | Acceptance review |
| REQ-07-03 | Prevent plugins and adapters from bypassing domain validation, rule-pack sandboxing, result envelopes, or report boundary controls. | Acceptance review |
| REQ-07-04 | Record import/export format choices as TBD unless a human ruling is cited. | Acceptance review |
| REQ-07-05 | Define handoff obligations for storage, reports, private libraries, local FEA export, and external automation without implementing them. | Human review |

### REQ-07-05 handoff obligations (D-41 R5 T2B, 2026-07-12 — current)

- **Storage:** adapters use the owning application-service/persistence contract, preserve schema version, hashes, provenance, diagnostics, and unit metadata, and do not treat direct SQL or sidecars as a public domain contract.
- **Reports:** adapters consume governed result/report envelopes and preserve affected-object references, assumptions, warnings, privacy classification, and professional-boundary flags; they do not create approval or code-compliance states.
- **Private libraries:** references remain local/private by default, carry redistribution and provenance metadata, and require explicit grants before filesystem, network, or publication access.
- **Local FEA:** the boundary is a governed handoff package with stable IDs, units/dimensions, coordinate basis, source hashes, diagnostics, and a loss report; it is not a solver-compatibility or validation claim.
- **External automation:** automation uses schema-first command/query/job-result envelopes, cannot mutate domain state directly, preserves idempotency/correlation and diagnostics, and requires explicit runtime grants for filesystem, network, process, or private-data access.

Five-boundary handoff map (D-41 R5 T2B, 2026-07-12):

| Handoff | Required preserved contract | Prohibited bypass |
|---|---|---|
| Storage | Schema version, canonical payload/hash metadata, provenance, diagnostics, units | Direct SQL/domain mutation; authoritative sidecars |
| Reports | Governed result/report refs, assumptions, warnings, privacy and professional boundaries | Approval, compliance, or reliance states |
| Private libraries | Local/private default, provenance/redistribution metadata, explicit grants | Implicit filesystem/network/publication access |
| Local FEA | Stable IDs, units/dimensions, coordinate basis, source hashes, diagnostics, loss report | Compatibility or solver-validation claim |
| External automation | Schema-first command/query/job envelopes, correlation/idempotency, explicit runtime grants | Direct mutation or implicit process/network/private-data access |

Apply REQ-07-05 as a no-bypass checklist at each owning boundary: identify the owning application-service contract; verify schema/version, identity, unit, provenance, diagnostic, privacy, and grant metadata; reject direct domain/storage bypasses; record unsupported target behavior without inventing implementation. Do not infer a concrete format, target compatibility, runtime grant, validation outcome, or professional decision from this map. These are architecture obligations only; concrete target formats and runtime integrations remain with their owning deliverables.

### Required invariants

- `OPS-K-IP-1`: Public artifacts must not contain protected standards text, tables, figures, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, or proprietary commercial data.
- `OPS-K-DATA-2`: Missing solve-required or rule-check-required values remain explicit findings, never silent defaults.
- `OPS-K-AUTH-1`: Software and agents must not claim to certify, seal, approve, authenticate, or declare engineering code compliance for reliance.
- `OPS-K-MECH-1`: Global analysis architecture remains a 3D centerline/frame model; local FEA is a handoff path.
- `OPS-K-AGENT-1`: Unknown engineering or architecture facts become `TBD`.
- `OPS-K-AGENT-3`: Type 2 execution stays within sealed deliverable scope.

### Interpretation guidance

- Prefer explicit contracts over package-local assumptions; keep language concrete enough for implementation, abstract enough to avoid premature stack decisions.
- Treat diagnostics, provenance, units, and data-boundary checks as cross-cutting architecture obligations.
- Preserve the distinction between mechanical calculation, user rule checking, and professional approval; do not claim code compliance or professional approval.
- Record a choice as `TBD` when no cited human ruling exists; record `PROPOSAL` only when explicitly framed for review; downstream packages cite the accepted decision record or note the awaiting ruling.
- Boundary: this member scopes PKG-00 architecture only. It does not advance PKG-01 through PKG-17 work and introduces no protected or proprietary content.

## Resolved decisions (former TBD and human-ruling queue)

The kit's original TBD slots (public API protocol, plugin trust model, import/export formats, adapter packaging, permission model) have been partially resolved by cited human rulings recorded in `execution/_Coordination/_DECISIONS/_REGISTER.md` and codified in SOFTWARE_DECOMP.md §12:

- **Import/export container:** the native project package and public transport form is a multi-member archive package (zip/directory) per the PKG-17 manifest contracts — D-09 ruling, codified as `DEC-028`.
- **Container naming and release matrix:** `.opsproj` naming rider and the release matrix — D-06 ruling, codified as `DEC-057`.
- **Headless external-automation entrypoint:** stable local `openpipestress-runner` CLI with schema-first JSON input, stdout JSON default, explicit output path only, and a local foreground no-network/no-daemon/no-telemetry/no-hidden-mutation process policy — D-32 ruling (`DEC-064`, provisional proven-L2 entrypoint) and D-33 ruling (`DEC-065`, final CLI and process policy).

## Realized artifacts

| Anticipated (setup era) | Realized | Owner |
|---|---|---|
| `docs/architecture/api_boundary_map.md` | Never created under that name. Realized as `api/api_boundary_contract.yaml` (machine-checked by `tests/test_api_boundary_contract.py`) | DEL-10-01 |
| Adapter contract map | `docs/architecture/plugin_boundary.md` and `docs/architecture/extension_domain_contracts.md` | Architecture docs (this basis provides the boundary obligations) |

## Open holds and routed questions

Genuinely still open, confirmed as `TBD` in the implemented contract `api/api_boundary_contract.yaml` (see also `tests/test_api_boundary_contract.py`):

- Public API transport protocol and endpoint syntax (`public_transport_protocol`, `endpoint_syntax`, `openapi_transport_binding`).
- External format list and the per-format export contracts (`external_format_list` and related export-contract slots).
- Related contract slots also held TBD there include plugin runtime/loading/signing/isolation, permission grant persistence, API stability level, and code-generation tooling.

These remain routed to human ruling; they are not silently selected here.

## Currency and provenance

Consolidated 2026-07-15 per piping decision D-43 from the four-document kit (`Datasheet.md`, `Specification.md`, `Procedure.md`, `Guidance.md`) as reconciled by D-41 R5 T7 (PDU-054 currentness declarations). Prior wording, including setup-era framing, is preserved in git history; `MEMORY.md` is retained unchanged. Current upstream authority is SOFTWARE_DECOMP revision 0.9 with DAG-007 coordination.
