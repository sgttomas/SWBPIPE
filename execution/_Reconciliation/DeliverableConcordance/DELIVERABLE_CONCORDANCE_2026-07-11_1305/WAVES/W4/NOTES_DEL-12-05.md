# NOTES — DEL-12-05 Security threat model (R2 wave W4)

Frozen source tree: `main` @ `551f84ef6be656f1603ce0acfa5e3935aa9683c7`.
Deliverable: PKG-12 / DEL-12-05, status IN_PROGRESS (DOC_UPDATE).
Discovery pilot: **highest-available-capability GPT-5**, with adversarial care for
SECURITY/privacy and F-PIP-1 fence adjacency, as assigned.

All implementation and verification paths are relative to `WORKING_ROOT` =
`projects/chirality-piping`. Native requirement IDs `STM-REQ-001` through
`STM-REQ-012` are preserved in `NormativeSource`; fixed run ClaimIDs use the
`DEL-12-05-REQ-NNN` form.

## 1. Histograms (recount from the CSV; must reproduce exactly)

Disposition histogram (24 rows):

| Disposition | Count |
|---|---:|
| ALIGNED | 20 |
| PARTIALLY_IMPLEMENTED | 1 |
| STALE_SETUP_SPECIFICATION | 3 |
| **Total** | **24** |

ClaimType histogram (24 rows):

| ClaimType | Count |
|---|---:|
| REQUIREMENT | 12 |
| EXCLUSION | 6 |
| DECLARED_STATE | 6 |
| **Total** | **24** |

ACCEPTANCE = 0, REMAINING_WORK = 0, IMPLEMENTED_UNMAPPED = 0.

## 2. Self-flagged rows

- **DEL-12-05-REQ-009 — PARTIALLY_IMPLEMENTED.** The product threat model
  explicitly names `IP_BOUNDARY_WARNING` and generically requires diagnostic /
  result-envelope warning classes, while provenance, assumptions, and warnings
  are also present throughout the report/export controls. It does not explicitly
  bind provenance and assumption signals to named warning classes. The focused
  test does not cover this native requirement. The narrow missing documentation
  is therefore preserved rather than inferred from cross-deliverable diagnostic
  contracts.
- **DEL-12-05-DECL-003 (Guidance.md) — STALE_SETUP_SPECIFICATION, MEDIUM.** The
  product artifact exists and has been hardened, while Guidance still frames
  itself as setup content exposing risks “before implementation details harden.”
  Its conservative substance remains reflected in the product artifact;
  ALIGNED-with-note was plausible. W1–W3 calibration on setup-era future-tense
  prose supports the selected stale declaration disposition.
- **DEL-12-05-DECL-004 (Procedure.md) — ALIGNED.** Unlike the three stale kit
  declarations, Procedure explicitly defines a bounded setup workflow. Its
  prohibition on publishing the product artifact is scoped to that historical
  task and is evidenced by the setup run records; it is not read as a false
  assertion that no product artifact exists now.

## 3. Evidence-execution log

Re-executed from the frozen evidence worktree with
`PYTHONDONTWRITEBYTECODE=1`, an external `PYTHONPYCACHEPREFIX`, and pytest
`-p no:cacheprovider`:

- `python3 -m pytest -p no:cacheprovider -q tests/test_security_threat_model.py`
  → **3 passed**.
- `python3 tools/validation/validate_dependencies_schema.py <DEL-12-05>/Dependencies.csv`
  → **VALID**, 29 required columns, 23 data rows.

Ignored-aware porcelain was checked before and after. It contained exactly the
six addendum-9 allow-listed ignored artifact sets and no additional path:
`.pytest_cache/`, two reporting `Cargo.lock` files, two `__pycache__/` trees,
and `validation/benchmarks/nonlinear/target/`. The checks created no new frozen
artifact. The recorded R1 verification row `PY-67` also reports the focused
pytest surface as passing at the content-identical sweep commit; this pilot
re-executed it at the pinned frozen SHA.

## 4. Convention and security-boundary notes

- **Product-document grain.** These requirements ask whether the threat model
  identifies risks, controls, boundaries, and explicit TBDs. They do not claim
  the controls are implemented or security-sufficient. Consequently the
  requirement rows use `ValidationEvidence=NOT_APPLICABLE` with explicit
  no-assurance wording; a missing security-sufficiency review was not
  manufactured into a disposition gap.
- **Convention-6 marker use: zero rows.** The exact marker
  `NONE_FOUND — sufficiency review deferred, owner-gated` is intentionally not
  used. No row's accepted scope promises a later owner-gated sufficiency review
  as its validation basis. Legal, professional, cryptographic, permission-model,
  and protected-content boundaries are exclusions or implementation TBDs, not a
  blanket reason to stamp every SECURITY row. This is the narrow application
  required by the W3 DEL-08-05 spot-check calibration.
- **TBDs are not DEL-12-05 status residuals.** Encryption, secret storage,
  plugin permission mechanics, API transport, redaction workflow, telemetry
  schema, signing, reproducible builds, project packaging, and target admission
  remain explicit product-level implementation decisions. The DEL-12-05 claim
  is to record them honestly. Candidate ownership spans PKG-09, PKG-10, PKG-12,
  and PKG-17 surfaces; no unsupported rehoming into this status file is made.
- **No acceptance rows.** The Specification's Verification section restates
  coverage checks already embedded in the twelve requirement rows and adds no
  independent acceptance contract at addendum-12 grain.
- **No remaining-work row.** `_STATUS.md ## Remaining` contains only the seeded
  D-41 bootstrap item, excluded byte-exact under addendum 2. It is represented
  within DECL-005 without being treated as a residual, gate, or selectable item.
- **No implemented-unmapped row.** The material product artifact and focused
  test are explicitly stamped to DEL-12-05 in front matter, docstring, and R1
  inventory/verification records. June 18 desktop unit-policy evidence is a
  cross-deliverable supporting surface and does not warrant scope absorption.
- **Declaration census.** The four-document kit, `_STATUS.md`, and `MEMORY.md`
  yield six declarations; no deliverable-owned README exists. Specification,
  Datasheet, and Guidance retain overtaken setup-era framing. Procedure is
  historical-task scoped; Status and dated Memory are current/accurate at their
  respective grains.
- **Source reliability.** Requirement/exclusion rows are `UNVERIFIED` because
  their load-bearing product prose and focused tests are project-original,
  agent-generated evidence without a human validation ruling on claim content.
  Declaration prose rows are `NOT_APPLICABLE` per addendum 6. All dispositions
  remain agent judgments.

## 5. Boundary-compliance statement

All fences held. Discovery was read-only outside the two assigned W4 outputs.
No product file, lifecycle/status file, register, DAG, dependency record, or
review finding was edited; no lifecycle transition or R4/R5 repair was applied.
No release-readiness, security-assurance, certification, sealing, professional-
approval, or code-compliance claim is made. The product artifact's negative
guardrails were audited as document content only. Frozen-tree ignored-aware
porcelain remained restricted to the six addendum-9 allow-listed paths at close.
