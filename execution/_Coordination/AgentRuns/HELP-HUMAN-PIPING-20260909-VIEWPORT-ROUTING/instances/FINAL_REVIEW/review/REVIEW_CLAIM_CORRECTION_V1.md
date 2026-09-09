# Combined independent source-review claim correction V1

Status: `ACTIVE_ADDITIVE_CORRECTION`

This correction applies to frozen `REVIEW.md` SHA-256 `a4b67dffa819b133034455f1b939e2f7a8140bfe7582dac351fa334dab178712`. It changes two factual descriptions and leaves the source review, findings count, and `PASS` verdict unchanged.

1. In the F4 section, read “Exact zero/negative normal contact leaves sliding” as: **Exact zero or negative normal contact selects `Inactive`, thereby leaving the `Sliding` state.**
2. In the residual-risk section, replace the claim that U7 browser and native checks were supplied with: **The supplied browser checks validate the selected U7 interaction paths. Candidate native-build identity evidence was available, but the fresh native witness was still running and was neither supplied as passing evidence nor assessed by this code review.**

The pending native witness remains a separate downstream validation activity. No source changed, no review was repeated, and no claim is made about that witness’s outcome.
