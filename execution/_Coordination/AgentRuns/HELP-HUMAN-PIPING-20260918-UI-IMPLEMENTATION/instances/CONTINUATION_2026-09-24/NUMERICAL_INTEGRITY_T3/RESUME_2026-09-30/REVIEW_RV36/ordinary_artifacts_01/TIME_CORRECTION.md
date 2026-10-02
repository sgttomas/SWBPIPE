# RV36 timestamp correction

The sealed SESSION.json field `new_checks_stopped_utc=2026-10-02T04:32:15Z`
was entered during sealing without a sampled clock for that exact event.
Withdraw that timestamp's precision.

The directly sampled sealing/completion time, 2026-10-02T04:32:47.013533+00:00,
establishes that all review checks, including the Arc source hash check, were
complete **by** that time. It does not establish the exact last-check instant.
Receipt, scheduled cutoff and hard deadline are unchanged; completion remains
before both limits. The substantive review verdict and all check results are
unchanged.

The original 92-payload seal and its payload bytes are preserved. This additive
correction is included in FINAL_SHA256SUMS, which covers every current payload,
including the original SHA256SUMS and this correction. No new technical check
or execution was undertaken for this timestamp correction.
