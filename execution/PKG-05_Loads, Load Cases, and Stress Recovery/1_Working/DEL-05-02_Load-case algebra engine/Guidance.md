# Guidance: DEL-05-02 Load-case algebra engine

## Purpose

This deliverable creates the setup boundary for load-case algebra that combines mechanics result states in a unit-aware way while leaving code-specific combinations and acceptance decisions to user-owned rule packs and human authority.

## Principles

- Keep algebra mechanics-focused: addition, scaling, subtraction, and ranging describe load/result relationships, not code compliance.
- Treat units and dimensions as first-class constraints; an expression that mixes incompatible dimensions is a finding, not a repair opportunity.
- Keep code-specific combinations outside the product baseline unless supplied by lawful/private user rule packs.
- Keep rule-pack evaluation declarative and sandboxed; this setup does not authorize arbitrary executable rules.
- Preserve status separation between mechanics solved, rule-pack checked, and human-approved outputs.

## Considerations

| Topic | Guidance | Evidence |
|---|---|---|
| Expression grammar | TBD; select only under a future implementation brief with security and unit checks. | `_CONTEXT.md` Still TBD; OPS-K-RULE-2 |
| Primitive load dependencies | Combination algebra will need primitive load cases/result states from adjacent PKG-05 deliverables, but this setup does not implement them. | DEL-05-01, DEL-05-05 register context |
| Rule-pack combinations | User rule packs may supply code-specific combinations; this deliverable should expose a safe boundary for those inputs. | SOW-014; OBJ-005 |
| Diagnostics | Missing operands, dimensional mismatch, invalid references, and unsupported operations should become explicit findings. | OPS-K-DATA-2; AB-00-06 |

## Trade-offs

- A more expressive grammar can help users define combinations, but increases validation and sandboxing risk. Grammar choice remains TBD.
- Hard-coded combinations might appear convenient, but would violate the data boundary for code-specific and project-specific rule content.
- Early subtraction/ranging support should prefer deterministic, testable mechanics behavior over broad expression flexibility.

## Examples

Setup examples are intentionally abstract. No code-specific load combination, allowable, or protected standard formula is provided in this deliverable setup.

## Conflict Table (for human ruling)

| Conflict ID | Topic | Contenders | Human ruling |
|---|---|---|---|
| None | No source conflict identified during setup. | N/A | N/A |

