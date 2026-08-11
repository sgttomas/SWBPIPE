# ORCHESTRATION PLAN AMENDMENT V2 — bounded author remediation

- RunID: `HELP-HUMAN-PIPING-20260810-DEL0904-OWNER-GATES`
- Amends: frozen plan version 1
- AmendmentAuthority: WORKING_ITEMS fan-in disposition `REMEDIATION`
- Trigger: independent verifier `A2-VERIFY` material-defect notice; Agent 0 fan-in concern
- AcceptedBase: unchanged `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3`
- Scope/authority/write fence: unchanged; preparation only; run-root-only writes

## Confirmed defects

1. The DEC-046 author packet correctly separates convergence from verification but does not fully answer the owner's explicit four-suite release-comparison value request. Its owner interface needs (a) concrete evidence-derived comparison-policy options and admission/exclusion matrices where defensible, and (b) an explicit authority-resolution choice among DEC-046 convergence-only, a separately authorized DEC-026-derived public-comparison policy, and deferral. No cross-axis value may be silently promoted.
2. The MAINTAINER_REVIEWED packet reverses the exact Git blobs of DEL-09-04 `_REVIEW.md` and `Review_Findings.csv`.
3. Its `MR-W1` truth-correction phase omits aggregate authority staleness: the manual index and `docs/VALIDATION_STRATEGY.md` still call DAG-007 current while the live `_DAG/_LATEST.md` pointer names DAG-009.

R14 has no material verifier defect and remains accepted into V2 fan-in.

## Added nodes and ownership

| Node | Agent | Dependencies | Writes |
|---|---|---|---|
| `A2-DEC046-REMEDIATION` | fresh non-delegating ephemeral Agent 2 | failed verifier findings + original author output | `instances/A2_DEC046_REMEDIATION/{PACKET_V2.md,RETURN.md}` |
| `A2-MAINTAINER-REVIEW-REMEDIATION` | fresh non-delegating ephemeral Agent 2 | failed verifier findings + original author output | `instances/A2_MAINTAINER_REVIEW_REMEDIATION/{PACKET_V2.md,RETURN.md}` |
| `A2-VERIFY-V2` | fresh non-delegating ephemeral Agent 2 | both remediation returns + accepted R14 return | `instances/A2_VERIFY_V2/RETURN.md` |

The two remediations are concurrent and disjoint. Original packets/returns remain immutable failure-history evidence. V2 verification is serialized and reviews the two replacement packets plus the accepted R14 packet. Manager fan-in depends on `A2-VERIFY-V2=PASS`.

## Retry classification

- Detection layer: independent Agent 2 evidence review plus Agent 0 fan-in.
- Failure class: `INCOMPLETE_OWNER_INTERFACE` for DEC; `SOURCE_IDENTITY_ERROR` and `INCOMPLETE_CURRENCY_SCOPE` for maintainer review.
- Attempt: author remediation attempt 2.
- Disposition: preserve originals; generate replacement V2 packets; no project-truth repair or ruling application.
