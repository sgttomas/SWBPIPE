# PRESSURE_ORACLE implementation-refutation return V1

FAIL — three actionable arithmetic defects reproduced against the exact frozen V1 source. See FINDINGS.md for smallest inputs, independently derived expected values, observed outputs/errors, verified source lines and repair directions.

The original 114 valid cases (1,990 scalar comparisons) and 44 error fixtures (47 operations) all pass. All six required actual implementation mutation classes are killed in isolated evidence copies. Five added independent numeric cases identify 22 wrong/missing final outputs: loss through rounded areas/forces; upward-rounding amplified by selecting the largest multiplication association; and avoidable overflow before force cancellation.

No product or original oracle bytes changed. The new diagnostic expectations use the unchanged original arithmetic policy and independent oracle. Source and original freeze hashes reverified after execution. Root has routed a V2 repair; this V1 package remains immutable failure evidence. Root retains integration acceptance. A separately sealed follow-up must name frozen V2 source before successor refutation.
