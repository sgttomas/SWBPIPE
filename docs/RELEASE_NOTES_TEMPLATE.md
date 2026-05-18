---
doc_id: OPS-RELEASE-NOTES-TEMPLATE
doc_kind: engineering.release_template
status: draft
created: 2026-05-04
deliverable_id: DEL-10-04
refs:
  - rel: governed_by
    to: OPS-PROFESSIONAL-BOUNDARY
  - rel: governed_by
    to: OPS-RELEASE-QUALITY-GATES
---

# Release Notes Template

Use this template for release-candidate notes and public release notes. Replace
bracketed placeholders before publication. Keep unresolved decisions as `TBD`
with an owner or gate.

## Release Identity

| Field | Value |
|---|---|
| Release label | `[release label]` |
| Source revision | `[commit hash or working-tree evidence record]` |
| Release date | `[date]` |
| Release authority | `[human/project authority record or TBD]` |
| Evidence profile | `[skeleton, python, security, cargo, all, or custom]` |
| Artifact set | `[docs only, source archive, package candidates, installers, or TBD]` |

## Summary

`[Short summary of implemented software changes and review state.]`

This release evidence describes software behavior, validation status, known
limitations, and data-boundary controls. It is not professional engineering
approval, certification, sealing, endorsement, authentication, legal advice, or
a code-compliance determination for reliance.

## Changed Surfaces

| Area | Summary | Evidence |
|---|---|---|
| Solver/mechanics | `[TBD or summary]` | `[tests, benchmarks, or not applicable]` |
| Rule engine | `[TBD or summary]` | `[tests or not applicable]` |
| GUI/workflow | `[TBD or summary]` | `[tests/screenshots or not applicable]` |
| Reports/audit | `[TBD or summary]` | `[tests/lint or not applicable]` |
| API/interoperability | `[TBD or summary]` | `[schemas/tests or not applicable]` |
| Packaging/release | `[TBD or summary]` | `[readiness profile or not applicable]` |

## Validation Evidence

| Gate | Command or record | Result | Notes |
|---|---|---|---|
| Release skeleton | `python3 tools/release/check_release_readiness.py --profile skeleton --execute` | `[pass/fail/waived/TBD]` | `[notes]` |
| Python/schema contracts | `python3 tools/release/check_release_readiness.py --profile python --execute` | `[pass/fail/waived/TBD]` | `[notes]` |
| Security/privacy | `python3 tools/release/check_release_readiness.py --profile security --execute` | `[pass/fail/waived/TBD]` | `[notes]` |
| Rust crates | `python3 tools/release/check_release_readiness.py --profile cargo --execute` | `[pass/fail/waived/TBD]` | `[notes]` |
| Protected-content review | `[scan command or review record]` | `[pass/fail/waived/TBD]` | `[notes]` |

Waivers must reference a human governance record and must not authorize
protected-content copying, private-data exposure, signing-secret exposure,
professional approval claims, or code-compliance claims.

## Known Limitations And Open Risks

- `[limitation or risk]`
- `[TBD decision and owner]`

## Data, IP, And Privacy Boundary

- Public artifacts do not intentionally include protected standards text,
  protected tables, protected examples, proprietary commercial data, private
  project data, private rule packs, private material/component libraries, or
  real secrets.
- Public examples and fixtures are `[invented/public-domain/permissively
  licensed/not applicable/TBD]`.
- Telemetry and external data transfer status: `[off by default/not
  applicable/TBD]`.

## Professional Boundary Notice

OpenPipeStress is decision-support software. It may compute mechanics, evaluate
user-supplied rule packs, record diagnostics, and assemble auditable reports.
Competent human review remains required before professional reliance on any
project-specific piping calculation.

## Reproducibility Manifest

| Field | Value |
|---|---|
| Git revision | `[commit hash]` |
| Working tree state | `[clean or changed paths]` |
| Python version | `[version]` |
| Rust/Cargo version | `[version or not applicable]` |
| Node/package tooling | `[version or not applicable]` |
| Host OS | `[OS/version]` |
| Commands run | `[command list or attached record]` |
| Artifact checksums | `[checksums or TBD]` |

## Upgrade And Compatibility Notes

`[Migration steps, compatibility breaks, schema version changes, or none.]`

## Human Review Record

| Review item | Disposition |
|---|---|
| Release authority | `[accepted/waived/TBD]` |
| Data-boundary review | `[accepted/waived/TBD]` |
| Professional-boundary review | `[accepted/waived/TBD]` |
| Known-risk acceptance | `[accepted/waived/TBD]` |
