# Decision log

| Date | Decision | Rationale |
|---|---|---|
| 2026-07-28 | Revalidate DAG-008; do not regenerate a successor. | SCA-008 changed no dependency or DAG path, so a rebuild would create churn without new topology. |
| 2026-07-28 | Preserve the 30 aggregate-only duplicate retirements. | A blind local-register aggregation would reactivate retired duplicate rows and falsely report 1,001 unique directed pairs before direction-aware deduplication. |
| 2026-07-28 | Treat `CURRENT_BY_REVALIDATION` as an evaluation disposition, not a new graph authority. | DAG-008 is already approved; this run demonstrates continuing compatibility. |
| 2026-07-28 | Update only the DepClosure pointer. | The DAG pointer remains on immutable DAG-008. |
| 2026-07-28 | Record the owner's standing direction as provenance but keep the disposition document non-authoritative until its governed application. | Audit evidence does not itself grant authority. |

## Human-direction provenance

> Finish out your plan now (attaining your goal) with self merge of PRs and auto approve for owners rulings, which should still be recorded in the usual manner with your recommendation standing as what I approved.

No human override of AUDIT_DEP_CLOSURE's read-only-on-deliverables boundary was
required.
