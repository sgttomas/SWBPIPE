# DEL-01-03 notes — W2 PKG-01, worker G1

Forward ledger sealed (SHA-256 in `DEL-01-03_SEAL.txt`): 110 rows (82 required
keys, 23 `.rNN` rows for split blocks CLM-003, CLM-006 and CLM-022, and 5 `.sNN`
sub-claims). Reverse: 199 routed capabilities. These are agent judgments,
not owner rulings.

## Path aliases

- Deliverable folder: `projects/chirality-piping/execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-03_Contributor certification workflow/`. Its files are cited only in `ContextRefs`.
- `.github/ISSUE_TEMPLATE` refers to `projects/chirality-piping/.github/ISSUE_TEMPLATE/`.

## Judgment calls

1. **Authority TBDs (`FG-DEL-01-03-01`).** DEC-027 settles maintainer, reviewer and release authority, with intake closed. DEC-079 adopted the review skeleton and keeps the legal instrument and its sufficiency open. The stale class follows F3 origin: CLM-003.r05, CLM-022.r04 and CLM-034 were first present at 7bee9ae41. The others (CLM-006.r11, REQ-04, CLM-020.s02, CLM-021, C-002) were written on 2026-06-03/04.
2. **CLM-003.r04, license TBD and "free/open-source" (`FG-DEL-01-03-05`).** This is `STALE_SETUP_SPECIFICATION`. PolyForm-Noncommercial-1.0.0 was selected on 2026-06-03, and the 2026-06-04 status entry says the stale license wording was resolved, but this row was missed. The PRD-level posture conflict is recorded in DEL-01-01 CLM-009.s01.
3. **REQ-08 and AC-001 `PARTIALLY_IMPLEMENTED` (`FG-DEL-01-03-02`, MEDIUM).** CONTRIBUTING.md L18 reads as if contributions are accepted, and it never states that DEC-027 closed intake (the issue-template README does). Both artifacts keep the overtaken authority TBDs and pin 0.7 / DAG-007.
4. **Remaining R01, CP-07 `REMAINING_STATE_MISMATCH` · `RULED_CRITERION`.** The item was written on 2026-07-10 with the gate "D-07b". D-07b was ruled on 2026-07-15 (DEC-079).
5. **Remaining R02, `UNKNOWN` · `AUTHORITY_UNCLEAR`.** See the smallest check below.
6. **Remaining R03, `DOCUMENTED_UNIMPLEMENTED` · `NOT_STARTED`.** Under F2 no governing row of this SOW carries the work. The export tool has no ISSUE_TEMPLATE handling, and the public repository is prospective (DEC-059). The item names DEL-10-04.
7. **Closed intake and C6(a).** OUT-001, CLM-007 and CLM-023 describe the documented workflow, its states and its record types. The documents and templates exist, and DEC-079 adopted them as the activation skeleton, so I read these claims as satisfied by the documents. They are not claims that intake runs. A verifier may read CLM-023 (records) as met by construction.
8. **CLM-009, CLM-017 and CLM-024 (PDU-054, identical).** CLM-017 and CLM-024 carry `DUPLICATE_OF DEL-01-03:SOW#CLM-009` and take the same CP-02 disposition.

## Canonical departures

None.

## Convention friction

- Deliverable-local evidence is limited to `ContextRefs`.
- There is no disposition for a Remaining item whose basis became silent (R02). `UNKNOWN` · `AUTHORITY_UNCLEAR` (C6(f)) was the closest fit.

## Smallest check for UNKNOWN rows

- `STATUS#remaining/R02`: the owner confirms whether a pre-release legal review is still required. The item cites PRD v0.1 section 17.5, "Legal Review Requirement". The adopted PRD v0.4 has no legal-review requirement, and its section 17.5 is now Missing Data Behavior. If the review is still required, record its governing basis.

## Reverse pass

The reverse pass did not change my view of any sealed row. `CLAIMED_BY`: CONTRIBUTING.md and the certification template. `UNKEYED`: the E6 issue templates. They were built under this deliverable's run record, and the PDU-054 declaration states they exist, but no requirement key specifies them. `COVERS`: the review checklist and the IP policy. All F5 overlaps have capability-specific reasons.

## Batch consistency

PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
