---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-14-04
package_id: PKG-14
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-073, SOW-072]
package_objective_refs: [OBJ-016]
---

# Scope of Work — DEL-14-04 Analysis-run comparison engine

## Purpose and Objective Traceability

An engineer who changes a design and solves again can see what the change did to the results. For every result both runs report at the same place and on the same basis, the comparison gives the difference in a common unit and, where the engineer supplied a tolerance profile, whether the difference lies within it. Beside the results it shows what differs between the two runs themselves: solver, settings, load cases and diagnostics. The same two runs always give the same answer, and the answer is a diagnostic aid: it validates neither run.

It serves SOW-073 and SOW-072 under OBJ-016 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-14, row DEL-14-04), and `docs/PRD.md` §8.8, §10.5, §15.3 (the analysis outputs a comparison covers), §15.4, §15.5, FR-CMP-004 (Must: result deltas for mapped nodes, elements, supports, terminals and stress and result locations), FR-CMP-006 (Must: stable identifiers are the default mapping) and, for runs, FR-CMP-007 and FR-CMP-008 (Should: manual mappings; tolerance profiles). No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names it.

- **OUT-001** — The analysis-run comparison: the comparison of two analysis runs and its result.

## Deliverable Definition — Ontology

**Terms the product implements.** Comparison, side, mapping hierarchy, mapping record, unmatched record, tolerance profile, tolerance rule and the common comparison-result interface are defined by DEL-14-05 and used here as defined.

- *Result identity.* What must agree for two result rows to be the same result in two runs: the result identifier, the result family, the object it belongs to (a node, element, support, terminal or stress or result location), the load case or combination it is for, its dimension, and the semantic contract it was produced under, where the run record states one.
- *Automatic mapping.* A result identifier that occurs once on each side, with the rest of the result identity equal, is mapped automatically. Every other correspondence, between different identifiers or where an identifier repeats or its identity disagrees, exists only as an explicit manual mapping record.
- *Delta.* For a mapped pair, right minus left in a common unit. It is given only where both rows carry a unit and a dimension, the dimensions agree, and both values are finite numbers. A value in another unit of the same dimension is converted through the unit catalog. The two source values are kept with the delta, in their own units.
- *Not compared.* A mapped pair for which no delta is given, with every reason that applies.
- *Unmatched row.* A result row of one run with no counterpart in the other, listed on its side with its reason.
- *Tolerance classification.* Where a tolerance profile is supplied and one of its rules applies to the pair's result category and dimension, the delta is within the profile or exceeds it. Otherwise it is not classified, with the reason. The classification is derived from the delta and never replaces it.
- *Run context.* What the result says of the two runs themselves: each run's identity, model state, solver version, settings, units, load cases and combinations, rule-pack and library references and hashes, with the differences between the two in solver, settings and load basis.
- *Diagnostic difference.* A solve diagnostic present in one run and absent from the other.

**Undecided.**

- Whether a delta is given for a result to which the product reader gave a standing other than numerically eligible.

**Boundary.**

- **CLM-001** — Run records, the received results they bind and their verified read are DEL-14-02's. What a result row means, and whether a result may be relied on numerically, come from its producers (DEL-04-07) and the result readers (DEL-05-02 for an ordinary result, DEL-04-08 for a retained-precision or retained-source result), as the run record carries them.
- **CLM-002** — Mapping records, tolerance profiles and the common comparison-result interface are DEL-14-05's. This deliverable owns the arithmetic of a delta and the execution of the comparison, and it defines no tolerance value. The unit catalog and its conversions are DEL-02-02's.
- **CLM-003** — Comparing the entities of two model states is DEL-14-03's. The comparison view and the selection of two runs in the desktop are DEL-07-08's; the CSV file of the comparison table and the mapping of a comparison into the result export are DEL-08-04's; the comparison sections of a report are DEL-08-06's.
- **CLM-004** — Outside the product: comprehensive ingestion of a commercial prover's results, external validation, and any statement that a run is acceptable.

## Completion and Reliance Basis — Epistemology

A delta says how far two runs differ at one result. A delta within a tolerance profile says the runs agree to the tolerance the user chose; it says nothing about whether either run is right, and nothing about whether the tolerance is suitable. The comparison's own arithmetic is examined against unit definitions and hand-worked differences; agreement with another calculation of the product, a section-property calculation among them, is no basis for it or for any tolerance (also cited as DEL-14-04:4).

### Requirements

- **REQ-001** — The engine compares two analysis runs deterministically, on stable identifiers and on manual mappings where they are required. Automatic mapping is as the ontology defines it, and nothing is matched by heuristic (also cited as R-14-04-001). Source: SOW-073; PRD FR-CMP-006, §15.4.
- **REQ-002** — A comparison treats each run as bound to its exact model state, solver version, settings, units, load cases, diagnostics, results, rule-pack references, library references and result hashes, and carries that context in its result (also cited as R-14-04-002). Source: SOW-072.
- **REQ-003** — A delta is computed in a common unit, and only where unit and dimension metadata allow a valid comparison. Missing, ambiguous or incompatible unit metadata is reported and never defaulted (also cited as R-14-04-003). Source: SOW-073; PRD §5.6.
- **REQ-004** — The comparison covers the mapped results of nodes, elements, supports, terminals and stress and result locations, and the diagnostics and settings of the two runs, wherever the run records expose them: node displacements and rotations, element forces and moments, support reactions, equipment terminal loads, stress quantities, governing locations, user-rule check outputs and solve diagnostics (also cited as R-14-04-004). Source: decomposition row DEL-14-04; PRD §15.3.
- **REQ-005** — Classification of a delta uses a tolerance profile the caller supplies. Without a profile or an applicable rule the delta is given unclassified, and no tolerance is assumed (also cited as R-14-04-005). Source: SOW-073; PRD §15.5.
- **REQ-006** — Comparison output is diagnostic evidence. It claims no external validation, engineering acceptance, approval or compliance (also cited as R-14-04-006). Source: SOW-073; PRD §21.2.
- **REQ-007** — The result is a schema-first document and no ad hoc result shape (also cited as R-14-04-007). Source: decomposition AB-00-03.
- **REQ-008** — No fixture, example or output introduces protected standards text or tables, proprietary values, a private rule pack's payload or an invented engineering default (also cited as R-14-04-008). Source: PRD §20.1, §5.5.
- **REQ-009** — Result deltas are examined on constructed runs (also cited as R-14-04-009). Source: decomposition row DEL-14-04; PRD §22.4.
- **REQ-010** — The result is produced against DEL-14-05's common comparison-result interface. The arithmetic of unit normalization and the execution of the comparison stay here (also cited as DEL-14-04-SCA011-REQ-001, DEL-14-04-SCA011-VER-001, DEL-14-04:2 and DEL-14-04:3). Source: decomposition row DEL-07-08 (the comparison interface is DEL-14-05's).
- **REQ-011** — In the product, a user compares two recorded runs and receives this comparison from the runs' own records and received results: unit-aware deltas with their source values, the reasons for every pair not compared, the differences in settings and diagnostics, and a diagnostic for missing data (also cited as DEL-14-04:1). Source: PRD §10.5, FR-CMP-004.
- **REQ-012** — The source values and the delta are kept as computed whatever tolerance profile is applied, so that a profile can change without changing the evidence. Source: SOW-073.
- **REQ-013** — A recorded run is compared only after its stored record and received result verify against their recorded hashes; a recorded run that does not verify gives no delta and a named reason. Source: SOW-072 (result hashes).

### Acceptance criteria

- **AC-001** — For two invented runs constructed with known values at the same results, each delta equals right minus left as computed by hand from the constructed values, and the two source values appear with it in their own units.
- **AC-002** — Where the two rows of a pair are in different units of one dimension, the delta equals the difference formed from the units' public definitions. Where a row has no unit or dimension, the dimensions disagree, or a value is not a finite number, the pair is reported not compared with its reason and no number.
- **AC-003** — A result identifier that occurs once on each side with equal result identity is mapped automatically. A different identifier, a repeated identifier and an identifier whose identity disagrees are each mapped only when a manual mapping record names both rows, and are otherwise listed unmatched with the reason. A mapping survives a round trip through JSON with its identifier and both result identifiers unchanged.
- **AC-004** — With a constructed tolerance profile, a delta inside the rule's bound is classified within and one outside it exceeds, as computed by hand from the rule. With no profile, no applicable rule or a rule without a value, the delta is reported unclassified with the reason. The delta and source values are identical under every profile.
- **AC-005** — For runs constructed to differ in solver version, one setting, one load case and one diagnostic, the result reports exactly those differences, and it carries each run's model state, solver version, settings, units, load basis, rule-pack and library references and hashes as the run records hold them.
- **AC-006** — Repeating a comparison gives a byte-identical result, and the order in which results or mappings are supplied changes nothing.
- **AC-007** — Each result kind of REQ-004 that a run record exposes is compared. A pair of such a kind that fails a condition under which a delta is given (REQ-003, AC-002) is reported not compared with its reason; no kind is omitted, or reported not compared because the comparison does not support it.
- **AC-008** — A comparison result validates against the accepted version of the common comparison-result interface, and a result made invalid in each required member is refused.
- **AC-009** — The product's comparison of two recorded runs is made from the runs' stored records and received results. A run whose stored record or result does not verify gives no delta and a named reason, and a row lacking required data appears with a diagnostic.
- **AC-010** — Every implementation of the comparison in the product gives the same mappings and deltas for the same pairs of runs.
- **AC-011** — No result, label or message states or implies validation, acceptance or compliance, and every fixture and example is invented, carries its provenance and holds no engineering default.

## Production and Verification Method — Praxeology

**Method choices that define the product.**

- Mapping is exact. An identifier with equal identity, or the user's explicit mapping, pairs two rows, and nothing else does.
- Evidence comes before interpretation: source values and delta first, tolerance classification derived from them.
- Units are converted by the unit catalog and by nothing of the comparison's own.
- A pair that cannot be compared honestly is reported not compared with its reasons. No delta is produced to fill a row.
- Results are produced in a fixed order that does not depend on the order of the input.

**How each criterion is examined.**

- **VER-001** — Executable check (observes the deltas and source values of invented runs against differences worked by hand from the constructed values).
- **VER-002** — Comparison with a named reference (the public definitions of the units involved) with an executable check (observes pairs constructed with a missing unit, a missing dimension, disagreeing dimensions and a non-numeric value).
- **VER-003** — Executable check (observes automatic and manual mapping on runs constructed with unique, different, repeated and identity-disagreeing identifiers, and a JSON round trip of each mapping).
- **VER-004** — Executable check (observes classification under constructed profiles against bounds worked by hand from each rule, with no profile, no applicable rule and a rule without a value, and compares deltas across profiles).
- **VER-005** — Executable check (observes the run context and its differences for runs constructed to differ in known members, against the two run records).
- **VER-006** — Executable check (observes repeated comparisons byte for byte and comparisons with the inputs supplied in another order).
- **VER-007** — Executable check (observes, for each result kind of REQ-004, a constructed pair of runs exposing it).
- **VER-008** — Comparison with a named reference (the accepted version of the comparison-result schema, under a validator that imports no product code) with an executable check (observes constructed invalid results).
- **VER-009** — Executable check (observes the product's comparison of two recorded runs, of a run with an altered stored result, and of a run with a row lacking required data).
- **VER-010** — Executable check (observes each implementation on one committed set of invented runs and compares their mappings and deltas; it shows that the implementations agree and nothing more).
- **VER-011** — Judgment by a reviewer who did not write the comparison, who examines its outputs, wording and fixtures against PRD §20.1 and §21.2 with the protected-content scan.

## Governing Values and Decisions — Axiology

- **AX-001** — *Evidence before interpretation.* Better is more of the comparison readable without a tolerance: source values, units and deltas. A classification never replaces or rounds the numbers it was derived from.
- **AX-002** — *Comparable or not compared.* Better is more result kinds compared in a common unit. It may never be gained by comparing values whose units, dimensions or identity do not establish that they are the same quantity.
- **AX-003** — *Identity over resemblance.* Better is more rows paired by identifier or by the user's mapping, never by a guess.
- **AX-004** — *The user's tolerance, or none.* A missing tolerance leaves a delta unclassified. Better is fewer unclassified deltas because the user defined more rules, never because the product assumed one.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-073 SOW-072 OBJ-016 | REQ-003 REQ-009 REQ-012 | AC-001 | VER-001 | Closed form: a difference of two constructed values, worked by hand and not taken from the engine's output |
| OUT-001 | SOW-073 OBJ-016 | REQ-003 CLM-002 | AC-002 | VER-002 | Outside source: the public definitions of the units, which fix each conversion factor exactly. The defective pairs are constructed, so the criterion states the expected reason |
| OUT-001 | SOW-073 OBJ-016 | REQ-001 | AC-003 | VER-003 | The runs are constructed, so the criterion states the expected mapping under the mapping hierarchy of PRD §15.4, as DEL-14-05 states it |
| OUT-001 | SOW-073 OBJ-016 | REQ-005 REQ-012 | AC-004 | VER-004 | Closed form: the bound is worked by hand from the constructed rule. The rule's own values are the user's and are not examined for suitability |
| OUT-001 | SOW-072 OBJ-016 | REQ-002 REQ-004 | AC-005 | VER-005 | The runs are constructed with known differences, and the expected context is the two run records, which DEL-14-02 produces and the comparison did not write |
| OUT-001 | SOW-073 OBJ-016 | REQ-001 | AC-006 | VER-006 | The criterion fixes the expected result: the second run equals the first whatever the order of the input |
| OUT-001 | SOW-073 SOW-072 OBJ-016 | REQ-004 | AC-007 | VER-007 | Each kind of PRD §15.3 and the decomposition row is constructed, so the criterion states that it is compared |
| OUT-001 | SOW-073 OBJ-016 | REQ-007 REQ-010 | AC-008 | VER-008 | The reference is the accepted version of DEL-14-05's comparison-result schema, which this deliverable does not own; DEL-14-05 states its version as undecided |
| OUT-001 | SOW-073 SOW-072 OBJ-016 | REQ-011 REQ-013 CLM-001 | AC-009 | VER-009 | The altered result and the defective row are constructed, so the criterion states the refusal and the diagnostic; the stored hashes are recomputed with RFC 8785 and SHA-256 |
| OUT-001 | SOW-073 OBJ-016 | REQ-001 | AC-010 | VER-010 | The product's own output on committed runs, compared between its implementations. This guards against the implementations drifting apart and is not grounds for correctness; AC-001 to AC-004 carry those |
| OUT-001 | SOW-073 OBJ-016 | REQ-006 REQ-008 CLM-004 | AC-011 | VER-011 | Judged by a reviewer against PRD §20.1, §21.2 and §5.5 |
