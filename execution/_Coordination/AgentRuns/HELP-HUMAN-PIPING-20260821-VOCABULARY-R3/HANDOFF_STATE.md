# Handoff state — vocabulary completion round 3

- State: `N1_ACCEPTED_N2_ACCEPTED_ROWS_14_15_CLOSED_DEC025_PASS_PR_PENDING`.
- Accepted upstream basis: `main@66efaf6b95605ef69f3e405b505f48506d3cbada` (PR #603 merge), DAG-010, Receipt-123, accepted SCA-009, and the regenerated G4 export.
- Authoritative/derivative boundary: landed product commits are implementation truth; the coverage ledger, AgentRuns package, receipt, and DEC-025 summaries are evidence/coordination derivatives and do not replace decomposition truth.

## Lineage and node returns

- Step A backup: remote branch `codex/piping-vocabulary-r3-wip-20260821`, commit `764a459a1ce3d23c777ec1d5610c9f7852c8131d`, no PR. Its five blobs match the frozen R2 review hashes. Working-branch pre-repair lineage is `2bee267300e571e4e8686f73aba6ad4ba8be4c54`.
- N1: `ACCEPTED_AND_COMMITTED` at `8ca1984db45a9a8f6f3111a905b07c7d3da47c33`. Failed `N1_REVIEW_V1.md` is preserved; repair cycle 1 closed its post-queue role-inference finding; fresh `N1_REVIEW_V2.md` passed. Tee, reducer, valve, and flange now create through the explicit seam, distinct tee roles are authoritative, and bend is preserved.
- N2: `ACCEPTED_AND_COMMITTED` at `d1a8e20ae413be040b82428cdd0bdbef0809e8de`. Failed `N2_REVIEW_V1.md` is preserved; repair cycle 1 closed its existing-kind transition finding; fresh `N2_REVIEW_V2.md` passed. Expansion-joint creation requires a deliberately selected incident span plus explicit geometry, four stiffness axes, units, references, sources, and provenance.

## Coverage, checks, and parked work

- Vocabulary rows 14 and 15 are `CLOSED / CLOSED / LANDED`; row 16 remains `CLOSED / CLOSED / LANDED` and unchanged.
- The first clean DEC-025 attempt is preserved at `SWEEP_20260822T002108Z_d1a8e20ae413.json`: all 36 Rust crates passed, then Python collection failed because the selected host interpreter lacked `jsonschema`; later surfaces did not run. This was an environment dependency failure, not a product finding.
- The repository-pinned development dependencies were provisioned in an isolated temporary path. The complete clean retry passed at commit `5a73d9f0d2b21506ccd6b16564b00167f98a3690`: 36 Rust crates, Python 902, desktop Vitest 573, Playwright dev 22 and dist 2, and production build. Summary `SWEEP_20260822T002452Z_5a73d9f0d2b2.json`, SHA-256 `db7111fc6a3ff629061ba879e2bc89ac9751668c28d570579527ac11495c136a`.
- AUDIT_DECOMP remains parked and not selectable; the owner will rule on a runner separately.
- Owner-held PKG-02 runtime/transport/permission choices, `.opsproj`, PDU-011/PDU-047, and `MAINTAINER_REVIEWED` promotion remain unchanged.

## Publication state

- Branch: `codex/piping-vocabulary-r3-20260821`.
- `origin/main` advanced before PR creation to `adf805e0d9ac55787e8ac815c3018467babb7f50`; merge-base remains the accepted post-externalization basis `66efaf6b95605ef69f3e405b505f48506d3cbada`. No mid-PR sync has occurred or been authorized.
- One PR against `main` remains to be opened after this non-self-referential fan-in evidence commit; no merge is authorized.
- No unsigned-artifact lane or `artifact-proof` label applies. No lifecycle, release, issuance, reliance, professional-approval, certification, sealing, authentication, or code-compliance effect is created.
- Next owner: CHANGE for scoped final evidence commit, push, one PR, and external check observation; merge remains owner-gated.
