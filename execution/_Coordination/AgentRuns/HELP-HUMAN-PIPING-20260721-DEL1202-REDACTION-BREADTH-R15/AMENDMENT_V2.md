# Amendment v2 — cure of N3 candidate-brief BLOCK

**Authority:** WORKING_ITEMS pre-effect remediation under the adopted plan;
product/state effect remains held.

| N3 defect | V2 cure |
|---|---|
| Missed CLI surfaces | Added `REXC-CLI-002` for `headless_preview_runner`; added `REXC-LINT-001` as an exact separately governed DEC-058/DEC-059 diagnostic exclusion with no-change/no-product-use proof. |
| Caller-selected writer context | Each Python writer now hard-codes its matrix context. Only the fixed-local-private CAEPIPE external writer receives an explicit-intent boolean. Both runner binaries are fixed local-private. |
| Cross-package ownership | Added `AFFECTED_OWNER_MAP.csv`; exact owner decision must explicitly authorize N4's bounded integration scope. Non-PKG-12 payload semantics and all non-PKG-12 state remain read-only. |
| Ambiguous exclusions | Added `ROUTE_VERIFICATION_DISPOSITIONS.csv`; all 30 unique RouteIDs are covered exactly once. |
| Premature closeout | N4 cannot write deliverable state/run record/receipt. N5 is read-only. W3 may close only DEL-12-02 state after N5 `COMMIT-SAFE`; HELP_HUMAN/CHANGE retains receipt/Git closeout. |

N3's v1 `BLOCK` remains immutable history. N3B is a fresh verifier and is the
only v2 refutation that can support an owner-adoption request.

