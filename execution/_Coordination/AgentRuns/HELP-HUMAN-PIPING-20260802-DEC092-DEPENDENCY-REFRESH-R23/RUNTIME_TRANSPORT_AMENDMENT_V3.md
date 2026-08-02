# R23 bounded index-layout repair amendment V3

- Amendment ID: `R23-RTA-003`.
- Basis: V2 fan-in found that all five CSVs matched their sealed expected
  post-hashes, while each consumer `_DEPENDENCIES.md` differed from its sealed
  expected post-content only by one missing blank line between the pre-existing
  run-history item and the R23 run-history item.
- Scope: add exactly that one blank line in each of the five named consumer
  indexes. No semantic text, count, history content, order, or other whitespace
  may change. Each repair has one exact expected post-hash.
- CSV fence: all five `Dependencies.csv` files are read-only and must remain at
  their sealed Attempt-02 expected post-hashes.
- Runtime: five bounded Codex-native Agent 2 repair attempts, one disjoint index
  per child, normal native tools permitted only to verify/apply the exact hunk.
- Preservation: Attempt-01, Attempt-02, V1, and V2 remain immutable evidence.
- Acceptance: successful child patch is pending complete PROJECT_SETUP fan-in.
  Any context mismatch, unexpected hash, or additional delta stops the tranche
  without repair.
- All evidence, row/field/write fences, five holds, anchor scope, F-PIP-1..5,
  claim fence, and prior exclusions remain unchanged. No DAG, receipt, pointer,
  lifecycle/status/memory/product, decision/register, or Git closeout authority.

