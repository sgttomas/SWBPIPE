# Specification: DEL-01-03 Contributor certification workflow

## Scope

This deliverable defines the local draft requirements for a contributor certification workflow covering attestations, provenance fields, review routing, rejection rules, and quarantine handling for public data contributions.

It does not edit `CONTRIBUTING.md`, license files, maintainer policy files, or release artifacts. Those repo-level artifacts remain future implementation targets or human-governance outputs. It does not make legal conclusions, certify rights, or approve engineering reliance.

## Requirements

| Req ID | Requirement | Source |
|---|---|---|
| DEL-01-03-REQ-01 | The workflow shall require contributor-provided source, location, license or `TBD`, contributor identity, certification statement, redistribution status, and review status for public data records. | `docs/IP_AND_DATA_BOUNDARY.md` section 4; OPS-K-IP-2 |
| DEL-01-03-REQ-02 | The workflow shall reject or quarantine contributions with suspected protected standards text, tables, figures, examples, copied formulas, material allowables, SIF/flexibility tables, protected dimensional tables, proprietary vendor data without redistribution rights, or private user/project/rule-pack data. | `docs/IP_AND_DATA_BOUNDARY.md` sections 3 and 5; OPS-K-IP-1, OPS-K-IP-3 |
| DEL-01-03-REQ-03 | The workflow shall route suspected protected content to human/legal review and shall not paraphrase protected tables or values into public data. | `docs/IP_AND_DATA_BOUNDARY.md` section 5; OPS-K-IP-3; sealed brief hard stops |
| DEL-01-03-REQ-04 | The workflow shall preserve `TBD` for unresolved license, maintainer authority, release authority, quorum, and legal-review outcomes. | `docs/CONTRACT.md` OPS-K-GOV-1 and OPS-K-GOV-2; SOFTWARE_DECOMP OI-001, OI-003 |
| DEL-01-03-REQ-05 | The workflow shall separate contribution governance from professional engineering approval and shall not claim certification, sealing, code compliance, legal clearance, or professional reliance. | `docs/CONTRACT.md` OPS-K-AUTH-1, OPS-K-AGENT-4; `docs/DIRECTIVE.md` section 6 |
| DEL-01-03-REQ-06 | The workflow shall record reviewer disposition as a project governance review only, with status values that distinguish pending, accepted, rejected, and quarantined contributions. | `docs/IP_AND_DATA_BOUNDARY.md` section 4; `docs/AGENTIC_DEVELOPMENT_WORKFLOW.md` section 5 |
| DEL-01-03-REQ-07 | The workflow shall keep public/private data boundaries visible for future adapters, imports, rule packs, reports, and release gates. | AB-00-02, AB-00-06, AB-00-08; `docs/SPEC.md` sections 1, 7, 8 |
| DEL-01-03-REQ-08 | The workflow shall provide a contributor certification template suitable for later inclusion in a `CONTRIBUTING.md` section, but this setup deliverable shall keep that template local. | `_CONTEXT.md` Anticipated Artifacts; sealed brief |

## Standards

| Reference | Applicability | Status |
|---|---|---|
| `docs/CONTRACT.md` | Binding invariant catalog for IP, governance, authority, and agent behavior. | Accessible |
| `docs/IP_AND_DATA_BOUNDARY.md` | Public/private data boundary and provenance/quarantine rules. | Accessible |
| `docs/DIRECTIVE.md` | Stop rules and governance baseline. | Accessible |
| `docs/SPEC.md` | Architecture, provenance, diagnostics, reports, and acceptance semantics. | Accessible |
| External legal standards or licenses | Exact license and legal sufficiency of certification language. | TBD; human/legal review required |

## Verification

| Requirement | Verification approach |
|---|---|
| DEL-01-03-REQ-01 | Inspect template fields for all provenance and review fields from `docs/IP_AND_DATA_BOUNDARY.md` section 4. |
| DEL-01-03-REQ-02 | Inspect rejection/quarantine rules for all public-repository exclusions from `docs/IP_AND_DATA_BOUNDARY.md` section 3. |
| DEL-01-03-REQ-03 | Confirm suspected protected content routes to quarantine and human/legal review with no copied or paraphrased protected content. |
| DEL-01-03-REQ-04 | Confirm unresolved license/governance decisions remain `TBD`. |
| DEL-01-03-REQ-05 | Search the workflow for prohibited reliance claims such as certify, seal, approve, authenticate, legal clearance, or code compliant, except where listed as prohibited terms. |
| DEL-01-03-REQ-06 | Confirm disposition statuses are present and reviewer authority is framed as governance review only. |
| DEL-01-03-REQ-07 | Confirm protected-content/provenance warning paths are visible for downstream software gates. |
| DEL-01-03-REQ-08 | Confirm the repo-level `CONTRIBUTING.md` is not edited by this setup run. |

## Documentation

The deliverable-local documentation set consists of:

- `Datasheet.md` - structured fields and workflow states.
- `Specification.md` - requirements, verification hooks, and exclusions.
- `Guidance.md` - rationale, principles, trade-offs, assumptions, and human rulings needed.
- `Procedure.md` - operational steps for intake, review, quarantine, disposition, and records.
- `_SEMANTIC.md` and `_SEMANTIC_LENSING.md` - semantic lens artifacts used as enrichment aids only.
- `Dependencies.csv` and `_DEPENDENCIES.md` - local dependency register and index.

## Acceptance Criteria

- The local kit contains the required four documents and generated semantic/dependency artifacts.
- No repo-level artifacts are modified.
- Future repo-level `CONTRIBUTING.md` adoption requires recorded human project authority approval.
- No protected standards/code data, copied tables, proprietary values, or legal conclusions are introduced.
- Unknown license/governance/legal-review decisions remain `TBD`.
- Human rulings needed are visible.
