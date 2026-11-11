---
schema: chirality-deliverable-sow/v1
deliverable_id: DEL-08-05
package_id: PKG-08
decomposition_basis: execution/_Decomposition/SOFTWARE_DECOMP.md
project_scope_refs: [SOW-043]
package_objective_refs: [OBJ-002, OBJ-007]
---

# Scope of Work — DEL-08-05 Report protected-content linter

## Purpose and Objective Traceability

Nothing the project publishes, whether a report template, an example, a fixture or a release candidate, carries protected standards content, someone's private data or a claim the software may not make without a person having been told to look at it. This deliverable contributes the two instruments of that: a lint engine that screens report text and sends what it suspects to human review, and the release-candidate scan, whose owner-signed record is a precondition of publication.

Neither instrument decides anything. The engine is a heuristic and can miss; the scan record states what was scanned and found. Legal and provenance judgment, and the release gate, are a person's.

It serves SOW-043 under OBJ-002 and OBJ-007 (`execution/_Decomposition/SOFTWARE_DECOMP.md`, PKG-08, row DEL-08-05, and the decomposition's ruling DEC-058 on scan ownership and procedure), and PRD FR-REP-005, §19.4 and §20.1. In `docs/RELEASE_TARGET_v1.0.md` it is the owning deliverable of exit criterion 14: the release-candidate protected-content scan is signed off by the owner.

- **OUT-001** — The lint engine: its targets and surface kinds, its finding record and schema, its command-line entry, and the desktop's lint view of a session's report content (REQ-001 to REQ-013).
- **OUT-002** — The release-candidate scan: the scan runner, the scan record, and the signed scan of each release candidate (REQ-014).

## Deliverable Definition — Ontology

A **target** is a piece of text with its surface kind and its provenance. The **surface kinds** are public report template, public report example, public fixture, private user template and private project export. The first three are public surfaces.

A **finding** carries a code, a class, a severity, the target and best-effort location, a message, a remediation and a **review route**: human IP review, human privacy review, human professional-boundary review or maintainer provenance review. The finding categories are protected-content risk, private-data risk, proprietary-source risk, unknown provenance (review required), prohibited professional claim, and safe metadata (allowed, no action). A **blocking** finding is one that must be dispositioned by a person before the content it concerns is published.

**Safe metadata** is a rule pack's identifier, version, checksum, source notice, redistribution status and required-input status. It is allowed wherever it does not reproduce a protected formula or table.

A **release candidate's artifact classes** are six: (1) the native project package and public container members; (2) the canonical calculation-report artifact and report templates; (3) installers, platform packages and embedded resources; (4) the public repository tree; (5) public examples, fixtures, validation benchmarks and corpora; (6) release notes, gate records and release-evidence documents.

The **scan record** is the machine-readable account of one scan of one candidate, written to `validation/evidence/releases/` and named by candidate, time and commit. It carries an owner sign-off block. A record the tool writes is unsigned.

Boundary:

- **CLM-001** — What counts as protected content, the provenance record and the quarantine rule are DEL-01-02's policy; the prohibited claims and the required notice are DEL-01-04's. The linter applies them and defines neither.
- **CLM-002** — The owner owns the scan act and is the sole signatory of the legal and protected-data release gate. This deliverable owns the scanner tooling. The release-readiness checks for private data and secrets that the scan calls, and the release gate checklist, are DEL-09-05's; the scan runner, its procedure and its record are this deliverable's. The repository scan of exit criterion 16 is DEL-10-04's criterion and uses this tooling.
- **CLM-003** — Redaction and export control of a user's own data are DEL-12-02's. A user's private report templates are the user's responsibility; the linter looks at them only when the user asks.
- **CLM-004** — Outside the product: a legal opinion or clearance, automatic quarantine or rewriting of content, and transmission of scanned content without the user's explicit action.

## Completion and Reliance Basis — Epistemology

Not decided: the severity policy of a continuous-integration guard that runs the linter outside the release scan. The release scan's policy is REQ-014's.

- **REQ-001** — By default the linter scans public surfaces only. A private user template or private project export is scanned only when the user explicitly asks for it (also cited as DEL-08-05-REQ-001). Source: SOW-043; PRD §20.3.
- **REQ-002** — The linter flags, in public templates and examples, suspected protected code text, copied standards tables, protected figures and examples, copied code formulas, material allowables, SIF and flexibility tables, protected dimensional tables and proprietary commercial data. Table-shaped content with the signature of a standards table is flagged as unknown provenance, review required (also cited as DEL-08-05-REQ-002). Source: SOW-043; PRD §19.4, §20.1; DEC-058.
- **REQ-003** — A scan with no findings is never presented as legal clearance or professional approval (also cited as DEL-08-05-REQ-003). Source: decomposition row DEL-08-05; DEC-058.
- **REQ-004** — Suspected protected content is routed to review and quarantine. It is never accepted, paraphrased or normalized into a public artifact (also cited as DEL-08-05-REQ-004). Source: PRD §20.2; `docs/IP_AND_DATA_BOUNDARY.md` §5.
- **REQ-005** — Safe metadata is permitted where it reproduces no protected formula or table (also cited as DEL-08-05-REQ-005). Source: PRD §17.4, §19.2.
- **REQ-006** — The linter flags public report wording that says the software certifies, seals, approves, authenticates or declares code compliance for reliance (also cited as DEL-08-05-REQ-006). Source: PRD §21.2.
- **REQ-007** — Findings are diagnostics compatible with the product's warning contract, with the IP-boundary warning class where it applies (also cited as DEL-08-05-REQ-007). Source: decomposition AB-00-06.
- **REQ-008** — The public examples and fixtures that exercise the linter use invented values or synthetic markers and copy nothing from a standards body or vendor (also cited as DEL-08-05-REQ-008). Source: PRD §20.1.
- **REQ-009** — For the same targets, configuration and version the linter's output is identical (also cited as DEL-08-05-REQ-009). Source: OBJ-007.
- **REQ-010** — A guard that runs the linter fails or warns by a documented severity policy, and suspected protected content blocks review until a human disposition is recorded (also cited as DEL-08-05-REQ-010). Source: PRD §20.2.
- **REQ-011** — By default the linter transmits no private project, material, component or rule-pack data, and it transmits none without an explicit user action (also cited as DEL-08-05-REQ-011). Source: PRD §20.4, §20.5.
- **REQ-012** — Where the linter's output joins a report, an API or a guard, it goes through the product's result envelopes, and no adapter or plugin route reaches a public output around it (also cited as DEL-08-05-REQ-012). Source: decomposition AB-00-03, AB-00-07.
- **REQ-013** — A report or export produced through an adapter or plugin passes the same lint gate as one the product produces itself. An adapter's declaration does not stand in for the gate (also cited as DEL-08-05:1). Source: decomposition AB-00-07.
- **REQ-014** — Every release candidate is scanned over the six artifact classes as it ships them; a class it does not ship is recorded `not_applicable`, never left out. The scan makes five checks: an inventory of each artifact's members with checksums; the protected-content lint over every public text and JSON member; a provenance check that every public data record carries the policy's provenance fields with a public-permissive redistribution status and an accepted review status, where an unknown or undecided status or a missing certification blocks; the private-data and secret checks; and a check that no quarantined material is in any artifact. Each scan writes its record with the source revision, tool versions, commands, per-artifact checksums, per-finding code, severity, review route and disposition, and its known limits. The signed record is a precondition of any publication act; a blocking finding halts publication until the item is removed or quarantined and the scan is run again; and no waiver authorises copying protected content (also cited as DEL-08-05:2). Source: DEC-058; release target exit criterion 14.

- **AC-001** — Each synthetic risk fixture produces a finding of its category with a review route: a protected-content marker, a private-data marker, a proprietary-source marker, a block shaped like a standards table, a target with unknown or missing provenance, and each claim REQ-006 names. The safe-metadata fixture and the PRD §19.3 notice produce no blocking finding. Verify with VER-001.
- **AC-002** — Under the default configuration a private user template and a private project export among the targets are not scanned and are reported as skipped; under an explicit request they are scanned. In neither case does the linter open a network connection or write outside the output its caller names. Verify with VER-002.
- **AC-003** — Every finding carries its code, class, severity, target, location, message, remediation and review route and validates against the linter's schema; each class is one of the product's six warning classes or a local class whose mapping to one of the six is stated in the linter's schema; a protected-content finding has the IP-boundary warning class; and two runs over the same targets, configuration and version give byte-identical output. Verify with VER-003.
- **AC-004** — The output of a run with no findings states that the scan is heuristic and is not a clearance or an approval. No output field states that the scanned content is cleared, approved or accepted by the linter; a finding's disposition is left for a person to record; and the linter moves, rewrites or deletes nothing it scans. Verify with VER-003.
- **AC-005** — Every fixture and example that exercises the linter is invented or synthetic, carries its provenance note, and contains no protected or private content. Verify with VER-004.
- **AC-006** — For every adapter or plugin route that can produce a report or export, a request whose content carries a blocking finding is refused with that finding named as the reason, exactly as on the product's own route, whatever the adapter declares. Verify with VER-005.
- **AC-007** — A guard run under the documented severity policy fails on a blocking finding and reports a review-required finding without clearing it. Verify with VER-005.
- **AC-008** — For a candidate tree built for the purpose, the scan record lists each of the six classes with its members and checksums or `not_applicable`; gives each of the five checks its result; lists every finding with code, severity, review route and an empty disposition; blocks on a public data record whose provenance is unknown, undecided or uncertified and on quarantined material; and is unsigned. Verify with VER-006.
- **AC-009** — For the v1.0 release candidate there is a scan record at the candidate's revision in which every finding has the owner's disposition, no blocking finding is open, and the sign-off block carries the owner's signature. Verify with VER-007.

## Production and Verification Method — Praxeology

Choices that define the product:

- The engine evaluates text its caller supplies. It reads no project file on its own initiative and has no default path into a user's private material.
- Detection is by structure, metadata and synthetic markers. No protected text is held as a pattern or a fixture, since that would put in the repository the material the linter exists to keep out.
- Doubt fails toward review. Unknown provenance is a finding, and a false positive is the accepted cost.
- The scan tool documents and never clears: it has no signing path, and the owner's signature is made outside it.

- **VER-001** — Executable check (observes the finding category and review route for each invented fixture, and the absence of a blocking finding for safe metadata and the required notice): the expected category of each fixture is fixed by the policy's prohibited classes and PRD §21.2's list, and the fixtures are written for the check.
- **VER-002** — Executable check (observes which targets are scanned and which skipped under the default and the opt-in configuration, and that a run with the network disabled and a read-only tree completes with output only where named).
- **VER-003** — Executable check (validates a run's output against the linter's schema with a JSON Schema validator that is not the product's, compares two runs byte for byte, reads the no-findings statement, and compares the scanned tree before and after).
- **VER-004** — Judgment by a reviewer who did not write the fixtures (reads each lint fixture and example and its provenance note for protected or private content).
- **VER-005** — Executable check (observes, on each adapter or plugin route that can produce a report or export, that the refusal of a request whose content carries a blocking marker names the blocking lint finding, and not a dispatch refusal, although the declaration is valid; and the exit state of a guard run over a blocking and a review-required fixture).
- **VER-006** — Executable check (runs the scan over an invented candidate tree seeded with one case per check and observes the record's classes, checksums, check results, findings, blocks and unsigned state): the record's required content is DEC-058's list, and the seeded cases are written for the check.
- **VER-007** — Judgment by the owner (performs the scan act on the release candidate, dispositions each finding and signs the record), with an executable check that observes only that the record names the candidate's revision, has no open blocking finding and has a filled sign-off block.

## Governing Values and Decisions — Axiology

- **AX-001** — *Send it to a person.* Better is fewer pieces of protected or private content that reach a public artifact without a person having been asked. More false positives are an acceptable price; a quieter linter is never bought by passing unknown provenance in silence.
- **AX-002** — *Claim nothing.* Better is a scan whose record says more exactly what was looked at and what was not. That may never become a statement that content is cleared, lawful or approved, and no result of the tool substitutes for the owner's signature.
- **AX-003** — *Do not hold what you hunt.* Better detection is welcome only by structure, metadata and invented markers. It is never gained by committing protected text, tables or formulas as patterns or fixtures.
- **AX-004** — *Private by default.* Better is a linter more useful to a user on their own material when they ask. It never scans private surfaces unasked and never sends anything without the user's explicit action.

## Output and Evaluation Matrix

| Output | Objective refs | Requirement/claim refs | Acceptance refs | Verification refs | Evidence expectation |
|---|---|---|---|---|---|
| OUT-001 | SOW-043 OBJ-002 | REQ-002, REQ-004, REQ-005, REQ-006 | AC-001 | VER-001 | The categories to flag come from PRD §19.4, §20.1 and §21.2 and the policy of `docs/IP_AND_DATA_BOUNDARY.md` §3, owner rulings. Each fixture is invented for the check with its expected category stated beforehand. The check shows the linter catches these markers; it does not show it catches real protected content, which only human review can. |
| OUT-001 | SOW-043 OBJ-002 | REQ-001, REQ-011, CLM-003 | AC-002 | VER-002 | The default scope comes from SOW-043's note that private templates are the user's responsibility and PRD §20.3 to §20.5, owner rulings; network and write behaviour are observed from outside the process. |
| OUT-001 | SOW-043 OBJ-007 | REQ-007, REQ-009, REQ-012 | AC-003 | VER-003 | The diagnostic fields come from the decomposition's architecture basis AB-00-06, an owner ruling, and DEL-00-06's six warning classes and mapping rule are the reference for the classes; conformance is decided by a JSON Schema validator the product did not write, and determinism by comparing two runs. |
| OUT-001 | SOW-043 OBJ-002 | REQ-003, REQ-004, CLM-004 | AC-004 | VER-003 | Owner ruling: DEC-058 states that a clean scan is not a legal clearance. The unchanged tree is observed by comparing it before and after. |
| OUT-001 | SOW-043 OBJ-002 | REQ-008 | AC-005 | VER-004 | The rule that fixtures are invented comes from PRD §20.1, an owner ruling; the fixtures are read by someone who did not write them. |
| OUT-001 | SOW-043 OBJ-002 | REQ-012, REQ-013 | AC-006 | VER-005 | The no-bypass rule comes from the decomposition's architecture basis AB-00-07, an owner ruling; the blocking content is seeded by the check. |
| OUT-001 | SOW-043 OBJ-002 | REQ-010 | AC-007 | VER-005 | The guard's behaviour is fixed by its documented severity policy. Owner ruling DEC-058 gives that policy for the release scan; outside the release scan the policy is not decided, and there the criterion has no ground beyond the guard's own documentation. |
| OUT-002 | SOW-043 OBJ-002 OBJ-007 | REQ-014, CLM-002 | AC-008 | VER-006 | Owner ruling: DEC-058 fixes the classes, the checks, the record's content and the gate. The candidate tree and its seeded cases are invented for the check. |
| OUT-002 | SOW-043 OBJ-002 | REQ-014, CLM-002 | AC-009 | VER-007 | Release target exit criterion 14 says what must exist. The dispositions and the signature are the owner's own acts on the real candidate; the tool supplies neither. |
