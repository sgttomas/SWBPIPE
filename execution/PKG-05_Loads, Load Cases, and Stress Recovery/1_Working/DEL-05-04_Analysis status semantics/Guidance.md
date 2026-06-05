# Guidance: DEL-05-04 Analysis status semantics

## Purpose

This deliverable exists to prevent users, reports, APIs, and future agents from treating a mechanics solve, a user-rule-pack check, and professional acceptance as the same thing. The local kit should make implemented status evidence legible while preserving the boundary that lifecycle promotion and human/project acceptance require separate authority.

## Principles

| Principle | Guidance | Source |
|---|---|---|
| Separate computation from judgment | Mechanics and rule-pack statuses describe computations and findings; professional compliance remains human judgment. | OPS-K-MECH-2; OPS-K-AUTH-1 |
| Missing data is a result, not a default | Missing solve-required and rule-check-required values must be reported explicitly. Do not invent values or silently bypass a check. | OPS-K-DATA-2 |
| Human acceptance is external | Human acceptance, if later supported, should be modeled as a separate hash-bound record rather than a solver-emitted status. | OPS-K-AUTH-2; `docs/architecture/analysis_status_semantics.md` Human Acceptance Records |
| Rule checks use user data | User rule packs may evaluate results, but public software does not bundle protected standards formulas or private project values. | `docs/SPEC.md` section 6; OPS-K-IP-1 |
| Reports preserve limitation context | Report-facing statuses should carry warnings, assumptions, rule-pack references, provenance, and limitations. | OPS-K-REPORT-1 |
| Local kit is not lifecycle approval | This folder records evidence, acceptance criteria, review history, and residual TBDs. It may cite implemented schema/code/test evidence, but it does not promote lifecycle state, release readiness, or professional acceptance. | `_CONTEXT.md`; `_STATUS.md`; OPS-K-AGENT-4 |

## Considerations

- `MECHANICS_SOLVED` and `RULE_INPUTS_INCOMPLETE` can both be true for the same snapshot when mechanics outputs exist but user-rule-pack data is missing.
- `USER_RULE_CHECKED` and `USER_RULE_FAILED` are user-rule computation outcomes. They should not be renamed or displayed as automatic code compliance.
- `HUMAN_REVIEW_REQUIRED` is a software/report-facing boundary notice, not a failure.
- `HUMAN_APPROVED_FOR_PROJECT` is reserved for external human acceptance records. Implemented and future ordinary solve, rule-check, import, and report-generation code paths should prevent this value from appearing as automatic software status.
- `CODE_COMPLIANT` is not an allowed automatic status per `docs/TYPES.md`.
- Implemented schema/API/result surfaces should continue to expose only automatic software statuses in ordinary software output; external human acceptance remains a hash-bound reference or record.

## Trade-offs

| Trade-off | Guidance |
|---|---|
| Single enum vs status dimensions | A single enum is simple, but multiple statuses may truthfully coexist. Implemented and future surfaces should preserve coexistence where sources allow it. |
| Human approval label vs separate record | A label is easy to display, but a separate record is needed to bind acceptance to reviewed content hashes and prevent stale reliance. |
| Rule-pack checked vs rule-pack passed | The sources define checked and failed outcomes; pass semantics should be treated carefully and tied to user-defined rule outputs, not professional acceptance. |

## Vocabulary Notes

- Use `mechanics solved` for open-mechanics computation completion only.
- Use `rule inputs incomplete` when a rule pack cannot evaluate because user/code/project data is missing.
- Use `user rule checked` or `user rule failed` for user-rule-pack computation outcomes.
- Use `human review required` as a report/professional-boundary notice.
- Use `human approved for project` only as an external human acceptance record reference, never as an automatically emitted software result.
- Reports may reference rule-pack identity and checksums, but public artifacts must not reproduce protected formulas or private values.

## Examples

| Scenario | Appropriate status interpretation |
|---|---|
| Missing pipe section dimensions prevent solving. | `MODEL_INCOMPLETE`; do not invent defaults. |
| Displacements/reactions/stresses are computed, but a rule pack needs a missing owner allowable. | `MECHANICS_SOLVED` plus `RULE_INPUTS_INCOMPLETE`. |
| A user rule pack runs and returns at least one failing check. | `USER_RULE_FAILED` with rule-pack identity/version/checksum and diagnostics. |
| A calculation report is exported for professional use. | Include `HUMAN_REVIEW_REQUIRED` and professional-boundary notice. |
| A competent human later accepts a specific report package. | External hash-bound human acceptance record; not automatic software approval. |

## Conflict Table (for human ruling)

| Conflict ID | Conflict | Source A | Source B | Impacted sections | Proposed authority (PROPOSAL) | Human ruling |
|---|---|---|---|---|---|---|
| None | No source conflict identified during current evidence alignment. | N/A | N/A | N/A | N/A | TBD |

## Open Questions

| Open item | Status |
|---|---|
| Exact ownership of all schema/API/result integration points across downstream deliverables. | Partly implemented; integration ownership remains deliverable-specific |
| Storage owner and UI presentation for external human acceptance records. | TBD |
| Canonicalization edge cases for non-JSON payload hashes referenced by human records. | TBD |
| Whether a positive user-rule outcome needs an explicit `USER_RULE_PASSED` value or remains represented by `USER_RULE_CHECKED` plus check details. | TBD |
| Human acceptance workflow details remain TBD because automatic software authority is prohibited and any accepted record must bind to reviewed hashes. | TBD |
