# Datasheet: DEL-12-03 Telemetry off-by-default design

## Identification

| Field | Value |
|---|---|
| Deliverable ID | DEL-12-03 |
| Deliverable name | Telemetry off-by-default design |
| Package ID | PKG-12 |
| Package name | Security, Privacy, and Private Data Handling |
| Deliverable type | SECURITY_CONTROL |
| Scope item | SOW-037 |
| Objective | OBJ-010 |
| Context envelope | S |
| Lifecycle target | SEMANTIC_READY, not ISSUED |

## Attributes

| Attribute | Value | Source |
|---|---|---|
| MVP telemetry posture | No telemetry by default; telemetry may be a no-op in MVP. | `_CONTEXT.md`; `docs/_Decomposition/SOFTWARE_DECOMP.md` SOW-037 and OI-008 |
| User consent posture | Any telemetry, if later implemented, is opt-in only and requires explicit human approval before design activation. | `docs/_Decomposition/SOFTWARE_DECOMP.md` OI-008; `docs/CONTRACT.md` OPS-K-PRIV-2 |
| Forbidden telemetry content | Private project data, code-specific data, private rule-pack data, private material/component data, secrets, paths, report content, and protected standards content. | `docs/CONTRACT.md` OPS-K-IP-1/2/3, OPS-K-DATA-1/2/3, OPS-K-PRIV-1/2 |
| Local-first boundary | No cloud operation is included unless separately authorized. | `_CONTEXT.md`; `docs/DIRECTIVE.md` Section 4.2 |
| Anticipated artifacts | Telemetry policy, config defaults, tests. | `_CONTEXT.md`; `docs/_Registers/Deliverables.csv` row DEL-12-03 |

## Conditions

| Condition | Required handling |
|---|---|
| Configuration key is absent | Treat telemetry as disabled. |
| Configuration value is malformed | Treat telemetry as disabled and emit a diagnostic or validation finding. |
| User has not explicitly opted in | Do not initialize telemetry transport, collection, background jobs, or upload queues. |
| Human approval for telemetry design is absent | Keep implementation as no-op/default-off and record telemetry details as TBD. |
| A payload field could expose private or protected data | Exclude the field and record an IP/privacy boundary finding. |

## Construction

This deliverable defines a design boundary rather than product code in this setup pass.

| Artifact | Construction intent | Current setup result |
|---|---|---|
| Telemetry policy | State the default-off, opt-in, privacy-preserving rule set. | Captured in `Specification.md` and `Guidance.md`. |
| Config defaults | Define default-disabled behavior for later implementation. | Captured as requirements and verification checks. |
| Tests | Define checks proving default-off and no private-data transmission behavior. | Captured in `Specification.md` and `Procedure.md`. |

## References

- `INIT.md` for project bootstrap and boundary rules.
- `AGENTS.md` for bounded Type 2 dispatch rules.
- `docs/CONTRACT.md` for OPS-K-IP, OPS-K-DATA, OPS-K-AUTH, OPS-K-PRIV, and OPS-K-AGENT invariants.
- `docs/DIRECTIVE.md` for founding boundaries and out-of-scope hidden cloud telemetry.
- `docs/IP_AND_DATA_BOUNDARY.md` for public/private data handling limits.
- `docs/SPEC.md` for architecture, diagnostics, reports, and acceptance semantics.
- `docs/_Decomposition/SOFTWARE_DECOMP.md` revision 0.4 for PKG-12, DEL-12-03, SOW-037, OBJ-010, OI-008, and AB-00-01/02/03/04/06/07/08.
- `docs/_Registers/Deliverables.csv` row DEL-12-03.
- `docs/_Registers/ScopeLedger.csv` row SOW-037.
- `docs/_Registers/ContextBudgetQA.csv` row DEL-12-03.
