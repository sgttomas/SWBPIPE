# Specification: DEL-01-01 Project governance baseline

## Scope

This deliverable specifies a draft governance baseline for OpenPipeStress. It covers the intended public/free/open-source posture, maintainer and release policy skeleton needs, and required policy boundaries for protected content, professional authority, and agent-generated outputs.

This deliverable excludes product implementation, solver behavior, GUI behavior, legal advice, license selection, certification/approval language, and direct edits to the repo-level governance artifacts.

## Requirements

| ID | Requirement | Basis | Verification |
|---|---|---|---|
| REQ-01-01-01 | The governance baseline must preserve the intent that OpenPipeStress is a free and open-source piping stress analysis platform while keeping the exact license as `TBD` until the human project authority records it. | SOW-001; SOW-048; OPS-K-GOV-1 | Confirm `TBD` license language remains visible and no specific license is asserted. |
| REQ-01-01-02 | Governance language must distinguish project governance approval from professional engineering approval. | OPS-K-AUTH-1; docs/DIRECTIVE.md Section 6 | Check that maintainer/release approval is not described as certification, sealing, authentication, endorsement, or code-compliance approval. |
| REQ-01-01-03 | Public contribution and release policy language must include protected-content, provenance, redistribution-rights, privacy, and review gates. | OPS-K-IP-1, OPS-K-IP-2, OPS-K-IP-3, OPS-K-GOV-4 | Check for policy slots covering source/provenance, redistribution status, contributor certification, review disposition, quarantine/escalation, and private-data risk. |
| REQ-01-01-04 | Public release policy language must require disclosure of scope, validation status, known limitations, data-boundary constraints, and professional-responsibility limitations. | OPS-K-GOV-3 | Check release policy skeleton for each disclosure category or an explicit `TBD` slot. |
| REQ-01-01-05 | Maintainer authority, release authority, and public-governance decisions must be recorded in public governance artifacts before being treated as project policy. | OPS-K-GOV-2 | Check that draft language does not treat unrecorded roles, quorum, signing, or license choices as effective policy. |
| REQ-01-01-06 | Agent-produced governance content must remain draft/proposal material until accepted by the human gate. | OPS-K-AGENT-4; docs/AGENTIC_DEVELOPMENT_WORKFLOW.md | Check for draft status and absence of issued/final acceptance claims. |
| REQ-01-01-07 | Unknown governance values must be marked `TBD`; the deliverable must not invent legal conclusions, maintainer identities, quorum rules, signing process, or compliance claims. | OPS-K-AGENT-1; docs/DIRECTIVE.md Section 6 | Inspect all governance-value fields and open policy choices. |
| REQ-01-01-08 | The deliverable must preserve flat package/deliverable identity and stable IDs. | OPS-K-HIER-1; OPS-K-ID-1 | Confirm DEL-01-01, PKG-01, SOW, and OBJ IDs match registers. |

## Acceptance Criteria

| ID | Criterion | Evidence |
|---|---|---|
| AC-01-01-01 | License remains `TBD`; no specific license text or legal conclusion is asserted. | Datasheet decision surface; Guidance conflict C-01-01-001 |
| AC-01-01-02 | Contribution-review policy slots include source, provenance, redistribution status, contributor certification, review disposition, quarantine status, and private-data risk. | Specification REQ-01-01-03; future repo-level governance artifact review |
| AC-01-01-03 | Release policy slots include scope, validation status, known limitations, data-boundary constraints, professional-responsibility limitations, and release maturity wording marked `TBD` until decided. | Specification REQ-01-01-04; Guidance trade-offs |
| AC-01-01-04 | Maintainer/release authority values remain `TBD` until recorded by the human project authority. | Datasheet decision surface; Guidance conflict C-01-01-002 |
| AC-01-01-05 | Run evidence records that no repo-level artifacts were edited and no protected standards/code content was reproduced. | `_run_records/TASK_RUN_*.md` |

## Standards

| Standard or Authority | Status |
|---|---|
| `docs/CONTRACT.md` invariant catalog | Accessible draft governance authority |
| `docs/DIRECTIVE.md` founding directive | Accessible draft governance authority |
| `docs/TYPES.md` identity and vocabulary | Accessible draft governance authority |
| Legal open-source license text | TBD; no specific license selected |
| Protected standards/code requirements | Not reproduced; public data boundary only |

## Verification

Verification for this setup deliverable is document review:

- compare DEL-01-01 identity, scope items, objectives, and anticipated artifacts against `_CONTEXT.md` and the registers;
- confirm all applicable contract invariants are named or reflected;
- confirm repo-level artifacts were not modified;
- confirm no protected standards content, proprietary data, legal conclusion, or professional certification claim was introduced;
- confirm `TBD` is used for unresolved license, maintainer, release, quorum, signing, and human-authority choices;
- confirm `_SEMANTIC.md`, `_SEMANTIC_LENSING.md`, `Dependencies.csv`, `_DEPENDENCIES.md`, and `_run_records` evidence exist after the setup workflow completes.

## Documentation

Anticipated downstream artifacts from the register are:

- `docs/CONTRACT.md`
- `docs/DIRECTIVE.md`
- `governance/MAINTAINERS.md`

For this setup workflow, those artifacts are discussed only as targets for future human-approved repo-level edits.

The release maturity label taxonomy, validation-status wording, release signing process, and governance acceptance record format remain `TBD`.
