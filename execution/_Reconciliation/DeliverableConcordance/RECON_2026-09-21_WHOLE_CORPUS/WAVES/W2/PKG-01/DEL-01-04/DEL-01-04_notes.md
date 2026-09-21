# DEL-01-04 notes — W2 PKG-01, worker G1

Forward ledger sealed (SHA-256 in `DEL-01-04_SEAL.txt`): 61 rows (59 required
keys, no `.rNN` rows, 2 `.sNN` sub-claims). Reverse: 199 routed capabilities.
These are agent judgments, not owner rulings.

## Path aliases

- Deliverable folder: `projects/chirality-piping/execution/PKG-01_Governance, IP Boundary, and Professional Responsibility/1_Working/DEL-01-04_Professional responsibility and product-claims policy/`. Its files are cited only in `ContextRefs`.
- `agents/AGENT_WORKING_ITEMS.md` is the repository-root agents file.
- `GATE:GATE_EVIDENCE/PR834_CI/harness-run.json` is suite-level evidence for CLM-012, not a per-check result, and was not rerun.

## Judgment calls

1. **No table was split.** Every row of CLM-002, 003, 004, 010, 011, 012, 019 and 024 to 028 takes the block's single disposition. CLM-010's Notes record each requirement's evidence.
2. **R05 (hash-bound human acceptance records) is `ALIGNED`, not "met by construction".** schemas/analysis_status.schema.yaml defines `human_acceptance_records` with `bound_hashes` and invalidation. PB section 7 and the notice template carry the same rule. The storage workflow is still an accurate open TBD (TBD-04; the registry reserves ENGINEER_ACCEPTED).
3. **AC-001 `PARTIALLY_IMPLEMENTED` (`FG-DEL-01-04-02`, MEDIUM).** PB section 2 still lists roster, quorum and release authority as TBD after DEC-027. Both artifacts pin 0.7 / DAG-007.
4. **CLM-006 references.** INIT.md and agents/AGENT_PREPARATION.md no longer resolve, and the DAG-006 entry is a pin. All are recorded as one CP-02 stale-pointer row.
5. **Architecture basis `.s02`.** The "current scope-change basis" stops at SCA-004, while the latest accepted amendment is SCA-010. SCA-007's validation-posture change bears on product claims. This is recorded as CP-02. This variant's still-TBD list is accurate, so it has no row.
6. **CONTEXT package reference (CS-06 DRIFT).** The assigned-scope list omits SOW-064, which is this deliverable's own scope item.
7. **Rename.** The CP-04 finding sits on the SOW surface and covers seven lines. PROFESSIONAL_BOUNDARY.md also carries the former name; that is noted as context only (governance document, DEC-101 next-amendment clause).
8. **CLM-028 ("no source conflict").** Accurate as a record of the setup review. DEC-100 and DEC-107 leave PROFESSIONAL_BOUNDARY.md untouched (claims registry).

## Canonical departures

None.

## Convention friction

- Deliverable-local evidence is limited to `ContextRefs`.

## UNKNOWN rows

None.

## Reverse pass

The reverse pass did not change my view of any sealed row. `CLAIMED_BY`: PROFESSIONAL_BOUNDARY.md and report_notice_template.md. `COVERS`: the analysis-status schema (implements R05), the claims registry (PB is the policy depth behind BS-ACCEPT; the registry's home is DEL-16-04) and the release notes template (R04). All F5 overlaps have capability-specific reasons.

## Batch consistency

PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
