# DEL-01-02 notes — W2 PKG-01, worker G1

Forward ledger sealed (SHA-256 in `DEL-01-02_SEAL.txt`): 72 rows (56 required
keys, 11 `.rNN` rows for the split requirements table CLM-010, 5 `.sNN`
sub-claims). Reverse: 199 routed capabilities. These are agent judgments,
not owner rulings.

## Path aliases

- Deliverable folder: `projects/chirality-piping/execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-02_Copyright and protected-data boundary policy/`. Because the path has spaces, its files are cited only in `ContextRefs`. `NormativeSource` lines are relative to that folder.
- Parity records are repository-root paths under `execution/_Coordination/AgentRuns/SOW-PACKAGE-BATCH-ADOPTION-20260714-01/`.
- `GATE:GATE_EVIDENCE/PR834_CI/harness-run.json` is cited once (CLM-012.s03) as suite-level evidence. Its self-check step, which registers GEN-13, passed on the PR #834 head. No per-check result is asserted, and nothing was rerun.

## Judgment calls

1. **Reviewer role TBD (`FG-DEL-01-02-01`, MEDIUM).** DEC-027 made the sole human authority the sole maintainer and release authority, with intake closed. DEC-079 adopted the D-07b sole-maintainer review process as the skeleton. I read the reviewer role as recorded and the legal instrument as open. The text was first present on 2026-06-03 (11c5497f6), so the rows are `STALE_REVIEW_OR_EVIDENCE` (F3): R11, CLM-012.s02, CLM-016, CLM-017, CLM-023 and C-003.
2. **R3 and CLM-011, "quarantine path TBD".** These are overtaken by the default `quarantine/protected-content/` convention (policy L80, checklist section 8), which this SOW's own C-001 also records. The text was first present at 7bee9ae41, so the rows are `STALE_SETUP_SPECIFICATION` · `DOC_BEHIND_CODE`.
3. **R5 `IMPLEMENTED_DIFFERENTLY` · `OTHER` (MEDIUM).** The verification clause names the TYPES label `UNKNOWN_SOURCE`, but the checklist spells the state `unknown`. The blocking rule itself holds.
4. **AC-001 `PARTIALLY_IMPLEMENTED` (`FG-DEL-01-02-02`).** The authority blocks of the policy and the checklist still list roster, quorum and release authority as TBD after DEC-027, and they pin 0.7 / DAG-007.
5. **C-002.** The ruling column still says TBD, although DEC-079 has since ruled the deferral. The row is `STALE_SETUP_SPECIFICATION` (MEDIUM).
6. **CLM-002 (PDU-055 declaration).** Assessed under CP-03 and disposed as a CP-02 pin. Its delegation of residuals to an empty `## Remaining` is not relied on (A4).
7. **Rename.** The CP-04 finding sits on the SOW surface (L96, L257). The governance documents IP_AND_DATA_BOUNDARY.md L18 and the checklist also carry the former name. That is noted as context only, under DEC-101's next-amendment clause for governance documents.

## Canonical departures

None. CS rows follow their assignments. CS-04 has `.s01` and `.s02`.

## Convention friction

- As for DEL-01-01: deliverable-local evidence can go only in `ContextRefs`.
- The TBD-overtaken-by-ruling rows split between `STALE_SETUP_SPECIFICATION` and `STALE_REVIEW_OR_EVIDENCE` purely by F3 origin, although they are one finding.

## UNKNOWN rows

None.

## Reverse pass

The reverse pass did not change my view of any sealed row. `CLAIMED_BY`: IP_AND_DATA_BOUNDARY.md and the contribution review checklist. `COVERS`: CONTRIBUTING.md, the certification template and the report notice template, each of which implements a requirement of this policy. All F5 overlaps have capability-specific reasons.

## Batch consistency

PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
