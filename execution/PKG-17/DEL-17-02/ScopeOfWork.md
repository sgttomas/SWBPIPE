---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-17-02
package_id: PKG-17
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-030, SOW-074]
package_objective_refs: [OBJ-009, OBJ-017]
---

# Scope of Work — DEL-17-02 Export package, profile, and stable ID map contracts

## Purpose and Objective Traceability

This deliverable is the common export contract: what an export package is, what its profile declares, how every model entity is traced into the target, what the manifest records and how loss is reported. Every target exporter meets it before it writes anything in its own format. The result it contributes is that an export from SWBPIPE, in any format, is one auditable package: a reader can tell what was written, from which model, under which declared assumptions, where each model entity went, and what was omitted, approximated, delegated or could not be supported. An export may be partial; it is never silent about it.

It serves SOW-030 and SOW-074 under OBJ-009 and OBJ-017 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-17, row DEL-17-02), with `docs/PRD.md` §5.8, §11.7 (FR-HAND-002 to FR-HAND-004) and §16.1. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it; the loss report that exit criterion 6 reads is defined here. The native project package (`.opsproj`, DEL-02-05) takes its member inventory, manifest and per-member hashes from these contracts.

- **OUT-001** — The common export contract: the definitions below of export package, export profile, stable ID map, manifest and loss report, which every target schema under `schemas/` and every exporter realises, and the conformance of each target's package to them.

## Deliverable Definition — Ontology

**Terms the contract implements.**

- *Export package.* The set of members one export produces: target files, JSON payloads, the manifest, the stable ID map, the loss report, diagnostics and any sidecar. A single visible target file is still a package: its manifest, ID map and loss report travel with it.
- *Export profile.* The one place where an exporter's target assumptions are declared. Its fields are: `profile_id`; `profile_version`; `target_family` (native JSON, CAEPIPE MBF, stress-neutral CSV/JSON, PCF, GLB/glTF, or an adapter target); `target_version_basis` (a cited target version, or undecided); `source_basis_ids` (source IDs of the DEL-17-01 register and project references); `unit_policy` (output units and who converts); `coordinate_policy` (frame, axis convention and origin); `entity_coverage` (each entity family classed by loss category); `stable_id_policy`; `loss_report_policy` (required categories and which diagnostics block); `external_execution_policy` (optional, user-owned harness metadata only); and `boundary_notes`.
- *Stable ID map.* The record that links each canonical SWBPIPE identifier to what represents it in the target. The identifier families are project and model; line, node, element, component, support, material, section, load case, result case, analysis run and export; and on the target side file, record, metadata key, sidecar row and omitted-target placeholder.
- *Carrier mode.* How a target carries a canonical identifier, one of four:
  - `direct_target_carried`: the target artifact holds the identifier in a field, name or slot that an admitted source confirms;
  - `metadata_carried`: the target holds it in a documented metadata surface;
  - `sidecar_mapping`: the target does not hold it, and a separate ID-map member named by the manifest does. This is the mode whenever direct or metadata carriage is unsupported, ambiguous or undecided;
  - `omitted_target_entry`: the identifier has no representation in the target, and the ID map and the loss report both say so.
- *Manifest.* The package's own account of itself: the source model reference and its version or hash basis, the package and profile identifiers, the generation context, every member with its path and hash, the source-basis references, diagnostics and boundary notes.
- *Loss report.* The record, written for every export including a complete one, of how each part of the model fared. Its categories are six:
  - `exported`: represented directly in the target;
  - `omitted`: not written, with the reason and the affected identifiers;
  - `approximated`: written through a declared approximation;
  - `delegated`: passed to target configuration or to a user-owned external workflow without local interpretation;
  - `unsupported`: known target limitation;
  - `tbd`: behaviour no admitted source confirms, never presented as supported.
- *Loss entry.* Affected canonical identifiers, category, severity, target artifact if any, reason, source-basis reference and downstream implication.
- *Unit disclosure.* The export-unit contract: what every package states about the units of what it writes. It names the unit system it refers to and the source model, and gives the model's units as entered, the units of any result values carried, the units of the target file, the conversion policy, whether a conversion was performed and what it covered.

**Boundary.** This deliverable defines the contract; it writes no target format.

- Each target's schema, writer, fixtures and field layout belong to its deliverable: DEL-17-03 (native JSON), DEL-17-04 (CAEPIPE MBF), DEL-17-05 (external-run records), DEL-17-06 (stress-neutral), DEL-17-07 (PCF), DEL-17-08 (review geometry), DEL-17-09 (adapter targets).
- Which target facts are admitted is DEL-17-01's. The canonical identifiers are the model document's (DEL-02-01). The hash basis and the one-current-version rule for persisted documents are DEL-00-04's. The provenance record and the data boundary are DEL-01-02's. The diagnostic record is DEL-00-06's.
- The handoff package and its manifest are DEL-15-01's, and target mapping records are DEL-15-02's. An export operation offered through the API uses the envelopes of DEL-00-03 and DEL-10-01; this contract defines no envelope, transport or endpoint.
- This deliverable owns the export-unit contract and no code that applies it. The desktop helper that forms a unit disclosure for every export panel is shared desktop code under DEL-07-12. The refusal of an export whose project unit is missing, and the exact conversions an export performs, are DEL-02-02's.

**Not decided.** Whether a CAEPIPE MBF field can carry a canonical identifier directly (DEL-17-01 `TBD-17-01-003`). Until it is, MBF identity is sidecar-only.

## Completion and Reliance Basis — Epistemology

### Requirements

*Source and architecture basis.*

- **REQ-001** — A target-format statement in a profile rests on a source ID of the DEL-17-01 register.
- **REQ-002** — Target behaviour that the register and the dossier leave open stays undecided in the profile.
- **REQ-003** — The contract defines no identity scheme, hash basis, provenance record or data-boundary rule of its own; it uses those of the model document, the persistence rules and the data boundary policy.
- **REQ-004** — A profile states no target behaviour that the admitted source basis does not support.
- **REQ-005** — A profile records the source IDs it consumed, or marks the source as undecided.
- **REQ-006** — Every export contract schema is JSON Schema 2020-12.
- **REQ-007** — A hash recorded for a JSON member or a manifest is computed on the project's canonical JSON hash basis.
- **REQ-008** — The contract defines no API envelope; an export operation exposed through the API uses the command, query and job envelopes.

*Export package.*

- **REQ-010** — A package declares a package identifier, the source model's identity and its version or hash basis, the export profile identifier and the generation context.
- **REQ-011** — A package lists every member it emits: target files, JSON payloads, ID maps, manifests, loss reports, diagnostics and sidecars.
- **REQ-012** — Members have stable paths or a declared rule that generates them.
- **REQ-013** — A hash is recorded for every deterministic text or JSON member; a binary or target-owned member records a hash or the reason none is available.
- **REQ-014** — Every timestamp field is declared as deterministic, normalized, omitted or runtime-dependent.

*Export profile.*

- **REQ-020** — A profile declares target family, profile version, target version basis, source-basis IDs and boundary notes.
- **REQ-021** — A profile declares unit policy, coordinate policy, stable-ID policy and loss-report policy.
- **REQ-022** — A profile classes every entity family as exported, omitted, approximated, delegated, unsupported or undecided.
- **REQ-023** — A profile presents no open question of the DEL-17-01 register as support.
- **REQ-024** — External execution is optional and user-owned; a profile may record harness metadata and requires no bundled executable.
- **REQ-025** — Target code and check options are recorded only as pass-through target configuration.
- **REQ-026** — A reference to the export plan (`PLAN-EXPORT-INTEROP`) supports strategy and target order only, never a target-format field.

*Stable ID map.*

- **REQ-030** — The map links each canonical identifier to the target record, file, metadata field or sidecar row that represents it.
- **REQ-031** — The map records each unmapped canonical identifier with its reason.
- **REQ-032** — The map is the same on re-export when the source model and the profile are unchanged.
- **REQ-033** — A target file that cannot carry canonical identifiers is paired with a sidecar map that the manifest names.
- **REQ-034** — A map entry keeps the canonical identity and any target-generated identity apart.
- **REQ-035** — A map entry states its carrier mode, one of the four.
- **REQ-036** — A profile claims `direct_target_carried` only where an admitted source confirms the carrier for that profile.
- **REQ-037** — CAEPIPE MBF identity uses `sidecar_mapping` while `TBD-17-01-003` is open.

*Manifest.*

- **REQ-040** — The manifest records the source model reference, the package and profile identifiers, the member inventory and the source-basis references.
- **REQ-041** — The manifest carries diagnostics and boundary notes sufficient to audit unsupported behaviour without opening every member.
- **REQ-042** — The manifest says of each target-specific field whether it is source-confirmed, delegated, unsupported or undecided.
- **REQ-043** — The manifest names the sidecar ID map and the loss report whenever the target artifact cannot carry identity or loss categories itself.

*Loss report.*

- **REQ-050** — Every target exporter writes a loss report, the native project-owned exports included.
- **REQ-051** — A loss entry has the affected canonical identifiers, category, severity, target artifact if any, reason, source-basis reference and downstream implication.
- **REQ-052** — An approximation entry describes the approximation and hides no target limitation.
- **REQ-053** — An unsupported or undecided entry blocks when it affects solver-ready or compatibility-sensitive output.
- **REQ-054** — The categories are `exported`, `omitted`, `approximated`, `delegated`, `unsupported` and `tbd`, and no other.
- **REQ-055** — A delegated entry names the target configuration or external workflow that receives the behaviour, and claims no local interpretation.
- **REQ-056** — An undecided entry names its open question where one of `TBD-17-01-001` to `TBD-17-01-006` governs it.

*Units.*

- **REQ-060** — Every package carries a unit disclosure, and what it states equals what the writer did: the model's units as entered, the units of the file written, and any conversion performed with its scope.

*Boundary.*

- **REQ-090** — The contract and its examples contain no protected standards value, proprietary target example, private client data or reverse-engineered target behaviour.
- **REQ-091** — No profile, manifest, loss report or package member states compatibility with a target tool, completeness of a format, solver validity of review geometry, release readiness, code compliance, validation or professional acceptance.
- **REQ-092** — Every target profile carries each open question of the DEL-17-01 register that touches it until the question closes.
- **REQ-093** — A package is complete and valid with no target executor configured.
- **REQ-094** — Public fixtures and examples of every target contain no protected standards text or table, commercial example, proprietary model file, private owner criterion or licence-bypass guidance.

### Acceptance criteria

- **AC-001** — For an invented model exported through each target exporter the product has, the package carries a profile, a manifest, a stable ID map, a loss report and diagnostics; the manifest lists every member written, each with its path and its hash or the stated reason for none; and no file is written that the manifest does not list.
- **AC-002** — Every profile the product ships has all twelve profile fields. Each field that states target behaviour cites a register source ID or is marked undecided, and each open question that touches the target appears in it.
- **AC-003** — For each target, every canonical identifier of the exported model is accounted for in the ID map: each target representation it has is an entry with its carrier mode, or, when it has none, it has an omitted-target entry and also appears in the loss report; no identifier is both mapped and omitted. Canonical and target-generated identities are separate fields. `direct_target_carried` appears only under a profile that cites the admitted source for the carrier, and never in a CAEPIPE MBF map.
- **AC-004** — Every export writes a loss report, one with nothing lost included. It uses only the six categories, and every entry has the seven members of a loss entry. A model seeded with an entity the target cannot represent yields an entry for that entity and never a silent drop; a delegated entry names its recipient; an undecided entry names its open question; and an unsupported or undecided entry that affects solver-ready or compatibility-sensitive output blocks the export.
- **AC-005** — Exporting the same model under the same profile twice gives an identical ID map, equal hashes, and members that are byte-identical apart from fields declared runtime-dependent. Each timestamp field carries its declaration, and no runtime-dependent value is an input to any hash.
- **AC-006** — Each recorded hash of a JSON member or manifest equals SHA-256 over that member's RFC 8785 canonical form, as computed by an implementation that imports no product code. Each hash of a non-JSON member names the normalization it was computed over.
- **AC-007** — Every target schema is valid JSON Schema 2020-12, and the loss categories, carrier modes and loss-entry members it admits are those defined here, by the same names.
- **AC-008** — A package of each target validates and is complete with no executor configured. No contract text, schema, fixture, profile or package member states a claim REQ-091 names or contains content REQ-090 and REQ-094 exclude.
- **AC-009** — For each target, the package of an invented model carries a unit disclosure whose model units equal the model's, whose target units equal those of the file written and those the profile's unit policy declares, and whose conversion statement matches the values in the file.

## Production and Verification Method — Praxeology

**Method choices that define the contract.**

- The profile is the only place a target assumption is declared; an exporter reads its coverage from the profile and blocks or warns from it.
- Sidecar identity is the default; direct carriage is the exception and needs a source.
- A loss report is written on success, not only on failure.
- A narrow profile with sourced behaviour is preferred to a broad one with unproven behaviour.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the package each target exporter writes for an invented model, and the files on disk): the five objects are present, the manifest's inventory equals the files written, and the ID map accounts for every canonical identifier of the model, none both mapped and omitted.
- **VER-002** — Executable check (observes each exporter's response to seeded faults): a direct-carrier claim without a source, an entity the target cannot represent, and an unsupported entry on solver-ready or compatibility-sensitive output each produce the named diagnostic or entry.
- **VER-003** — Executable check (observes the bytes of two exports of the same model and profile, and the inputs of each hash): the ID map and hashes are equal, members are equal apart from fields declared runtime-dependent, and a changed timestamp changes no hash.
- **VER-004** — Comparison with a named reference (an RFC 8785 canonicalizer that imports no product code): recomputes each recorded JSON hash from the member's bytes.
- **VER-005** — Executable check (observes every target schema): each validates as JSON Schema 2020-12, and its category, carrier-mode and loss-entry vocabularies equal those of this scope.
- **VER-006** — Executable check (the project's protected-content scan over the export schemas and fixtures) and judgment by a reviewer who did not write them: examines every schema, fixture and profile for the claims and content the boundary requirements exclude.
- **VER-007** — Executable check (observes each target's unit disclosure against the invented model's units and the units and values of the file written).
- **VER-008** — Executable check (observes that every profile the product ships has the twelve profile fields) and comparison with a named reference (the DEL-17-01 source register: each field that states target behaviour cites a source ID the register admits for that target or is marked undecided, and each open question of the register that touches the target appears in the profile).

## Governing Values and Decisions — Axiology

- **AX-001** — *Explicit loss.* Better is a larger share of each exported model's entities accounted for by a loss category and a smaller share reaching the target through behaviour the profile does not declare; the target is all and none. Coverage is never raised by classing an unsourced behaviour as exported.
- **AX-002** — *Identity survives the target.* Better is more target records that trace back to a canonical identifier, and more of them by a carrier the target itself preserves. A direct carrier is never claimed without its source, and target record order is never the only identity.
- **AX-003** — *One vocabulary.* Better is fewer target-local variants of category, carrier mode and manifest field. A target's convenience never justifies a new meaning for a shared term.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-010 REQ-011 REQ-012 REQ-013 REQ-040 REQ-041 REQ-043 REQ-050 | AC-001 | VER-001 | `docs/PRD.md` §16.1 names the package's contents; the comparison is against the files on disk and an invented model |
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-001 REQ-002 REQ-004 REQ-005 REQ-020 REQ-021 REQ-022 REQ-023 REQ-025 REQ-026 REQ-042 REQ-092 | AC-002 | VER-008 | The DEL-17-01 register is the reference for each cited source ID and open question; the profiles examined are those the product ships |
| OUT-001 | SOW-074 OBJ-017 | REQ-003 REQ-030 REQ-031 REQ-033 REQ-034 REQ-035 REQ-036 REQ-037 | AC-003 | VER-001 VER-002 | `docs/PRD.md` FR-HAND-002 and FR-HAND-003; the identifiers compared are the invented model's own, which no exporter wrote |
| OUT-001 | SOW-074 OBJ-017 | REQ-050 REQ-051 REQ-052 REQ-053 REQ-054 REQ-055 REQ-056 | AC-004 | VER-002 | `docs/PRD.md` FR-HAND-004; the seeded unsupported entity states what must be reported |
| OUT-001 | SOW-074 OBJ-017 | REQ-014 REQ-032 | AC-005 | VER-003 | The product's own earlier output, which guards against change only; determinism is the property examined |
| OUT-001 | SOW-074 OBJ-017 | REQ-007 REQ-013 | AC-006 | VER-004 | Outside source: RFC 8785 and SHA-256, through a canonicalizer that imports no product code |
| OUT-001 | SOW-030 OBJ-009 | REQ-006 REQ-008 | AC-007 | VER-005 | Outside source: the JSON Schema 2020-12 meta-schema; the vocabularies are this scope's definitions |
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-024 REQ-090 REQ-091 REQ-093 REQ-094 | AC-008 | VER-006 | `docs/IP_AND_DATA_BOUNDARY.md` and `docs/PRD.md` §21.2 state what is excluded. The scan's markers are synthetic, so the scan finds only the kinds of content it is seeded with; the reviewer's judgment covers the rest |
| OUT-001 | SOW-030 SOW-074 OBJ-009 OBJ-017 | REQ-021 REQ-060 | AC-009 | VER-007 | The comparison is against the invented model and the file written, neither of which the disclosure wrote |
