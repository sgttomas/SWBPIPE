---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-01-02
package_id: PKG-01
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-003, SOW-028]
package_objective_refs: [OBJ-002]
---

# Scope of Work — DEL-01-02 Copyright and protected-data boundary policy

## Purpose and Objective Traceability

This deliverable draws the line between what the public project may publish and what it may not. It says which content is open mechanics and may be public, which content belongs to standards bodies, vendors or users and may not, what record a piece of public data must carry before it is accepted, and what happens to content that looks protected. Every schema, importer, linter, example, report template and release scan in the product takes its content boundary, its provenance record and its quarantine rule from here.

It serves SOW-003 and SOW-028 under OBJ-002 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-01, row DEL-01-02), and PRD §5.4, §17.4, §19.4 and §20.1 to §20.3. No exit criterion of `docs/RELEASE_TARGET_v1.0.md` names this deliverable. It reaches the release target through criterion 14, where the release-candidate protected-content scan applies this policy's content boundary and provenance record, and through criterion 16, where the public repository is scanned before release.

- **OUT-001** — The IP and data boundary policy, `docs/IP_AND_DATA_BOUNDARY.md`: allowed public content, prohibited public content, private user data, the provenance record, the quarantine rule, the report boundary and the rule for evaluating an external prover.
- **OUT-002** — The contribution review checklist, `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`: the reviewer's record of one public contribution, field by field, ending in a disposition.

## Deliverable Definition — Ontology

**Terms the policy implements.**

- *Open mechanics.* Solver algorithms, schemas for geometry, units, loads, stress recovery and reports, blank templates, private-library import mechanisms, invented example data with a clear non-engineering notice, public-domain or permissively licensed data with documented provenance, and validation benchmarks from lawful sources. This is what the public repository may contain.
- *Protected content.* Standards-body text, tables, figures, examples and commentary; material allowable tables copied from standards; stress-intensification and flexibility-factor content copied from standards; protected dimensional and rating tables; proprietary vendor catalogues, and commercial software examples, report templates and benchmark files, without permission. Code-specific load combinations, allowables, factors and acceptance criteria are protected unless invented or cleared for public redistribution.
- *Private user data.* A user's rule packs, material libraries, component catalogues, owner standards and project design bases. It stays in user-controlled paths.
- *Provenance record.* The record every public data record carries, with these fields: `source_name`, `source_location`, `source_license`, `contributor`, `contributor_certification`, `redistribution_status` and `review_status`.
- *Redistribution status.* One of `public_permissive`, `private_only`, `unknown`, `protected_suspected`.
- *Review status.* One of `pending`, `accepted`, `rejected`, `quarantined`.
- *Quarantine.* Removal of suspected protected content from every public use until a human or legal review decides it. The default location is `quarantine/protected-content/`, or another non-example path the maintainer approves. Quarantine is a protective step, not a finding of infringement.
- *Redistribution* and *runtime handling.* Redistribution is committing, publishing or exporting content to others. Runtime handling is the product reading a user's own private data on that user's machine, including sending it to a model provider the user configured. The policy's prohibitions govern redistribution.

**Boundary.** The policy states the rules. The following enforce or apply them and belong elsewhere:

- who has authority to accept a contribution: DEL-01-01;
- the contributor's attestation, intake routing and whether intake is open: DEL-01-03;
- the provenance check on library import: DEL-03-07;
- the protected-content linter for reports and the release-candidate scan: DEL-08-05, with the owner as the scan's signatory;
- separation of public and private storage paths: DEL-12-01;
- telemetry content: DEL-12-03.

This deliverable implements no diagnostic, schema, test or gate.

**Not decided.**

- The owner's contributor rules. OUT-002 is written from them and is not drafted without them.
- Who gives the legal review that the quarantine rule and uncertain redistribution rights call for.
- Who keeps the quarantine location and who may read it.

## Completion and Reliance Basis — Epistemology

The policy is a project governance control. It is adequate when a maintainer, a contributor or an implementer can tell from it alone whether a given piece of content may be public, what record it needs and what to do when unsure.

### Requirements

- **REQ-001** — The policy states that the public repository contains no protected content, naming each class (SOW-003; PRD §20.1; `docs/CONTRACT.md` OPS-K-IP-1).
- **REQ-002** — The policy requires every public data record to carry the full provenance record, with a review disposition (PRD §20.2; `docs/CONTRACT.md` OPS-K-IP-2).
- **REQ-003** — The policy defines the handling of suspected protected content: stop ingestion, mark the record `protected_suspected`, quarantine it, record the issue and request human or legal review. Quarantined material is not used as fixture data, copied into documentation, summarized into public tables or turned into public defaults while its status is unresolved (PRD §20.2; `docs/CONTRACT.md` OPS-K-IP-3).
- **REQ-004** — The policy separates open mechanics from code-specific, proprietary and private user data: code-specific values are supplied by the user, and importers, examples, reports, telemetry, issue bundles and issue forms exclude private user data unless the user exports or contributes it with documented redistribution rights (PRD §5.4, §20.3; `docs/CONTRACT.md` OPS-K-DATA-1).
- **REQ-005** — A missing, unknown or undocumented licence, provenance or redistribution status blocks public acceptance until it is resolved. Rights are never inferred from a source being public, useful or easy to find (`docs/CONTRACT.md` OPS-K-IP-2, OPS-K-DATA-2).
- **REQ-006** — Public example rule packs use invented, non-code values only and carry a clear non-engineering notice (PRD §17.4; `docs/CONTRACT.md` OPS-K-RULE-1). Public report templates and examples use invented or permissively sourced data only and contain no protected standards text, protected tables, proprietary formulas, private rule-pack content or private project data (PRD §19.4; `docs/CONTRACT.md` OPS-K-REPORT-2).
- **REQ-007** — Review of a public contribution for protected content, provenance, redistribution rights and private data is a maintainer gate that precedes acceptance. The checklist records the reviewer and the disposition (SOW-028; `docs/CONTRACT.md` OPS-K-GOV-4).
- **REQ-008** — The policy gives no legal advice and draws no legal conclusion about any content or contribution. Accepting content under it is project governance, not professional approval of engineering use (PRD §21.2).
- **REQ-010** — The policy states rules only. Enforcement is by the deliverables named in the boundary: DEL-03-07 on library import, DEL-08-05 in report linting and the release-candidate scan, DEL-12-01 in storage paths and DEL-12-03 in telemetry.
- **REQ-011** — The checklist is at `governance/CONTRIBUTION_REVIEW_CHECKLIST.md`. It is the reviewer's side of the two-sided record of a contribution, used in the sole-maintainer review; the contributor's side is DEL-01-03's certification.
- **REQ-012** — The policy carries the rule for the external prover CAEPIPE, interoperate openly and evaluate privately: public text may name CAEPIPE and state the file-format and interface facts the code needs to interoperate with it; anything that evaluates CAEPIPE (result values, differences, agreement counts, measured tolerances, defects, limits, screenshots) stays in the user's private exchange folder and is never committed; and no public text claims agreement with CAEPIPE. When the owner has checked the CAEPIPE licence for benchmark clauses, the rule is amended so that benchmark agreement is published and other evaluation stays private (`docs/RELEASE_TARGET_v1.0.md`, benchmark method).
- **REQ-013** — The policy distinguishes redistribution from runtime handling. A user's own private data read at runtime, or sent to a model provider that user configured, is not redistribution and creates no exception to REQ-001, REQ-003 or the rights of third parties (`docs/CONTRACT.md` OPS-K-PRIV-1).

### Acceptance criteria

- **AC-001** — The policy's prohibited list names every class in PRD §20.1's "must not contain" list and in `docs/CONTRACT.md` OPS-K-IP-1, and its allowed list names each class PRD §20.1 permits and nothing those sources prohibit.
- **AC-002** — The policy names all seven provenance fields with the two value sets above, and states that a record with an undecided licence, an `unknown` redistribution status or no contributor certification is not acceptable as public data.
- **AC-003** — The policy gives the five quarantine steps in order, names the default quarantine location, and lists the four uses quarantined material is barred from.
- **AC-004** — The checklist has a field for each of: the contribution's identity and scope; source name and location; licence or redistribution basis; contributor identity and certification; redistribution status; protected-content risk; private-data risk; test, validation and documentation evidence; the reviewer and the disposition; and the quarantine or escalation record. A reviewer holding only the policy and the checklist reaches the disposition the policy determines for each of three invented contributions: original data with complete provenance, vendor data with unclear redistribution terms, and data that appears copied from a standards table.
- **AC-005** — The policy states that public example rule packs use invented, non-code values only and carry a clear non-engineering notice, and that public report templates and examples use invented or permissively sourced data and contain no protected standards text, protected table, proprietary formula, private rule-pack content or private project data.
- **AC-006** — The policy states where private user data lives, the default exclusion of that data from importers, examples, reports, telemetry, issue bundles and issue forms, and that runtime handling of a user's own data is not redistribution and waives no third party's rights.
- **AC-007** — The policy states the CAEPIPE rule with its three parts: what public text may say, what stays private, and that no public text claims agreement.
- **AC-008** — Neither document states a legal conclusion, calls quarantine a finding of infringement, or describes acceptance of content as approval of engineering use.

## Production and Verification Method — Praxeology

**Method choices that define the policy.**

- *Lists, not judgment calls.* Allowed and prohibited content are enumerated by class, so that an implementer can turn each class into a check.
- *One vocabulary.* The provenance field names and the two status value sets are defined here once. Schemas, importers, the linter and the release scan use these names and values.
- *Conservative default.* When status is unclear the outcome is pending, rejected or quarantined, never accepted.
- *Two-sided record.* With a quorum of one there is no second reviewer; the record of a contribution is the contributor's certification and the reviewer's checklist, holding the same evidence fields.

**How each criterion is examined.**

- **VER-001** — Judgment by a named role (an independent reviewer who did not write the text): read the policy against PRD §5.4, §17.4, §19.4 and §20.1 to §20.3, `docs/CONTRACT.md` OPS-K-IP-1 to OPS-K-IP-3, OPS-K-RULE-1, OPS-K-REPORT-2 and OPS-K-PRIV-1, and the benchmark method of `docs/RELEASE_TARGET_v1.0.md`; report each class, field, value, step or rule that is missing, added or weakened, and each legal conclusion or approval-like statement.
- **VER-002** — Judgment by a named role (an independent reviewer who did not write the checklist): apply the policy and the checklist to the three invented contributions of AC-004 and record the disposition reached for each. The expected dispositions are the policy's: eligible for acceptance; pending or rejected; quarantined and escalated.

## Governing Values and Decisions — Axiology

- **AX-001** — *Keep protected content out.* Better is fewer ways protected or private content can reach a public artifact, measured by the content classes and paths the policy leaves unaddressed. Usefulness of an example or a dataset is never a reason to accept content whose rights are unclear.
- **AX-002** — *Make the lawful path easy.* Better is a contributor or implementer reaching the right disposition with fewer fields and fewer steps. This is never bought by dropping a provenance field, by inferring rights, or by letting an automated check stand in for the human or legal review that suspected content requires.
- **AX-003** — *Record, do not expose.* Review outcomes are recorded where that is safe. A public record never reproduces the suspected content, a private rule pack, an owner standard or project data.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-003 OBJ-002 | REQ-001 REQ-004 REQ-010 | AC-001 | VER-001 | PRD §20.1 lists the permitted and the prohibited classes and `docs/CONTRACT.md` OPS-K-IP-1 the prohibited ones; the reviewer compares the policy with them |
| OUT-001 | SOW-028 OBJ-002 | REQ-002 REQ-005 | AC-002 | VER-001 | PRD §20.2 lists the fields. The two value sets are defined by this scope |
| OUT-001 | SOW-003 SOW-028 OBJ-002 | REQ-003 | AC-003 | VER-001 | PRD §20.2 and `docs/CONTRACT.md` OPS-K-IP-3 require quarantine and escalation; the steps and the default location are defined by this scope |
| OUT-002 | SOW-028 OBJ-002 | REQ-007 REQ-011 | AC-004 | VER-002 | The policy says what must happen to each invented contribution; the dispositions are reached by a reviewer who wrote neither document. The checklist's content comes from the owner's contributor rules |
| OUT-001 | SOW-003 OBJ-002 | REQ-006 | AC-005 | VER-001 | PRD §17.4 and §19.4 and `docs/CONTRACT.md` OPS-K-RULE-1 and OPS-K-REPORT-2 |
| OUT-001 | SOW-003 OBJ-002 | REQ-004 REQ-013 | AC-006 | VER-001 | PRD §20.3 and `docs/CONTRACT.md` OPS-K-PRIV-1 |
| OUT-001 | SOW-003 OBJ-002 | REQ-012 | AC-007 | VER-001 | The release target's benchmark method, which says what is published about CAEPIPE and what stays private |
| OUT-001 | SOW-003 SOW-028 OBJ-002 | REQ-008 | AC-008 | VER-001 | PRD §21.2 lists the prohibited claims |
