# Guidance: DEL-08-06 State, comparison, and handoff report sections

## Purpose

This deliverable exists to make state/run records, deterministic comparisons, and handoff manifests visible in auditable calculation reports while preserving the project boundary that software output is decision support, not professional validation. The source basis is `_CONTEXT.md`, SOW-024, OBJ-007, OBJ-016, OBJ-017, OBJ-018, `docs/SPEC.md` section 9, and the local approved DAG-002 dependency mirror.

## Principles

| Principle | Guidance | Source |
|---|---|---|
| Report from records, not hidden state | Treat model states, analysis runs, comparisons, and handoff manifests as source records referenced by stable IDs, hashes, checksums, provenance, review state, and privacy classification where available. | `docs/SPEC.md` section 9; `docs/TYPES.md`; `Dependencies.csv` |
| Keep missing data visible | Missing solve-required or rule-check-required values should become explicit findings, warnings, limitations, unresolved TBDs, or human-review-needed findings. | `docs/CONTRACT.md` OPS-K-DATA-2; `docs/SPEC.md` sections 4.3 and 9 |
| Do not turn reports into approval records | State/run, comparison, and handoff sections may support professional review workflows but must not claim certification, sealing, approval, authentication, endorsement, code compliance, or professional reliance. | `docs/CONTRACT.md` OPS-K-AUTH-1; `docs/SPEC.md` sections 4.3 and 9 |
| Preserve source envelopes | Prefer references to source envelopes and manifests over copying private payloads or engineering values into public artifacts. | `docs/SPEC.md` section 9; `docs/IP_AND_DATA_BOUNDARY.md` sections 6 and 7 |
| Separate public examples from private data | Public fixtures should be original or invented. Private project values, private rule-pack payloads, private library content, protected standards text, and proprietary formulas should not be copied into public report artifacts. | `docs/IP_AND_DATA_BOUNDARY.md` sections 2, 3, and 7; `docs/SPEC.md` section 9 |
| Treat handoff as traceability, not acceptance | Handoff manifests can support downstream modeling and professional validation workflows, but unsupported-target flags and external-prover metadata do not create automatic professional acceptance. | `execution/_Decomposition/SOFTWARE_DECOMP.md` SOW-074 and SOW-075; `Dependencies.csv` rows DAG-002-E0866 through DAG-002-E0868 |

## Considerations

- The report generator, audit manifest, warnings/provenance sections, result export, protected-content linter, private redaction/export controls, state/run records, comparison records, handoff package records, export workflow records, and external-prover metadata are all upstream evidence surfaces in `Dependencies.csv`. This setup pass preserves those edges as ACTIVE and does not reclassify them.
- ASSUMPTION: the future implementation should assemble sections from schema-first records rather than from unstructured text scraping. This follows the architecture-basis injection and `docs/SPEC.md` section 9, but exact code interfaces are TBD.
- Any comparison wording should remain diagnostic/audit-oriented. A deterministic comparison can show changes and deltas; it does not prove external validation or acceptance.
- Any handoff wording should expose target mapping metadata and unsupported-target flags when available. It should not imply that a downstream external tool or human has approved the model.
- Boundary notices should be stable enough for tests, but exact wording remains TBD until the professional-claims policy deliverable and report templates are accepted.
- Protected-content linter output is review evidence only; a clean finding is not legal clearance or professional clearance.

## Trade-offs

| Trade-off | Conservative direction |
|---|---|
| Detail vs protected/private leakage | Prefer identifiers, checksums, source notes, review state, and privacy classification over copying formulas, tables, private values, or private payloads. |
| Human readability vs machine traceability | Preserve stable structured fields first; rendered prose can summarize but should not drop provenance or warning semantics. |
| Comparison completeness vs false authority | Present deterministic comparison findings and unresolved mappings without declaring acceptance, validation, or compliance. |
| Handoff usefulness vs unsupported-target risk | Expose unsupported or approximate target behavior instead of hiding loss of fidelity. |
| Early implementation speed vs report safety | Keep exact APIs and layout TBD until source contracts are available; do not infer product code from decomposition text alone. |

## Examples

No authoritative examples are available in the accessible source set for this deliverable. Public examples or fixtures for later implementation should be invented or otherwise permitted, include provenance/review notes, and avoid protected standards text, protected tables, proprietary formulas, proprietary engineering values, private project data, private rule-pack payloads, private library content, and real secrets.

## Conflict Table (for human ruling)

| Conflict ID | Conflict (short statement) | Source A (file + section) | Source B (file + section) | Impacted sections | Proposed authority (PROPOSAL) | Human ruling (TBD) |
|---|---|---|---|---|---|---|
| None | No source conflicts were identified during this P1/P2 setup pass. | N/A | N/A | N/A | N/A | TBD |
