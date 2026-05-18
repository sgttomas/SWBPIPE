# CAEPIPE Developer-Team Question Dossier: DEL-17-01

## Purpose

This dossier records clarification questions for the CAEPIPE developer/support team or equivalent public support channels. It is designed to close target-behavior gaps without asking for proprietary internals, protected standards data, commercial examples, or license-bypass guidance.

## Question Register

| Question ID | Question | Current Public Basis | Affected Deliverables | Gating Impact | Status |
|---|---|---|---|---|---|
| CQ-17-01-001 | Which CAEPIPE version should OpenPipeStress target first for a documented MBF export/import smoke profile? | Public CAEPIPE MBF and batch documentation exists; supported version/profile is not project-selected. | DEL-17-04, DEL-17-05 | Blocks version-specific support claims. | TBD |
| CQ-17-01-002 | Which MBF record families are recommended for a minimal straight-pipe/support/load-case smoke model? | MBF is documented as a text model data path; exact first-subset coverage remains project-defined. | DEL-17-04 | Blocks first writer subset acceptance. | TBD |
| CQ-17-01-003 | Are comments or user fields in MBF appropriate for preserving stable external IDs, or should stable IDs be sidecar-only? | PKG-17 requires stable ID maps; target-carried identity capacity is not yet confirmed. | DEL-17-02, DEL-17-04 | Blocks ID-map placement claims. | TBD |
| CQ-17-01-004 | Which command-line invocation pattern is recommended for a user-owned, licensed local executable smoke run? | Public docs describe command-line/batch behavior, but harness profile details remain environment-sensitive. | DEL-17-05 | Blocks harness profile wording. | TBD |
| CQ-17-01-005 | Which CSV output sections are stable enough for automated regression parsing in a first harness? | Public docs identify model/result export surfaces; stable parser subset is not yet selected. | DEL-17-05, DEL-17-06 | Blocks parser coverage claims. | TBD |
| CQ-17-01-006 | Which PCF translation defaults should OpenPipeStress warn about or avoid in a conservative PCF subset? | Public PCF translator documentation identifies mapping/default behavior; subset policy remains project-defined. | DEL-17-07 | Blocks PCF subset acceptance. | TBD |
| CQ-17-01-007 | Are there public documentation pages preferred for citation in open-source handoff documentation? | Current admitted URLs are public and official; preferred citation targets may vary by version. | DEL-17-01 through DEL-17-09 | Blocks documentation citation hardening only. | TBD |

## Question Boundary

Questions must not ask for:

- proprietary solver algorithms;
- protected standards content;
- commercial example files;
- copied report templates;
- hidden binary format behavior;
- license-bypass procedures;
- code-compliance calculations or professional acceptance criteria.

## Closure Rule

When a question is answered, record:

- answer source and date;
- whether the answer is public-citable or private/support-only;
- affected TBD IDs;
- affected downstream deliverables;
- any boundary that prevents public repository use.

