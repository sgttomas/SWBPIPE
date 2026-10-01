# AUD-T3-04 preservation — bounded TASK, reports directly to ROOT

Read agents/AGENT_TASK.md and COMMON.md. Initial allowance 30 minutes.
Use existing file/archive tools; no tool development, installs or Git mutations.
Read T3/AUDIT/REPORT.md finding 04 and KF2's _run_records/b/gate/uncommitted_sha256.txt.

Writes: R/evidence/** in numerics, and <wt>/preserved-evidence/t3-audit-20260930/**
only. Raw originals are read-only. No prune. No other scratch or source changes.
Find the six audited final-head DEC-025 suite-log sets (K4, KF1, VK, K6B, KF3,
KF2), distinguishing earlier attempts via their SWEEP head identities. Preserve
all needed original per-manifest logs and the two KF2 gate runs.jsonl files.
Copy rather than move; verify original and copied hashes and record byte sizes.
Preserve provenance/run-from bases. Sanitize machine paths and ANSI in committed
suite-log copies, retaining original hashes and the explicit transformation.
For large JSONL, use an ordinary compressed evidence copy outside Git and a
portable inventory/restore instruction; never commit a >100MB object. Determine
whether sanitized compressed copies fit the repository's existing limits before
proposing them. Do not upload elsewhere or claim remote durability.

Return a concise inventory and closure limits: which evidence is now durable
locally versus Git-recoverable, where original hashes match, and what still needs
a retention decision. Do not claim AUD-T3-04 fully closed merely because a local
copy exists. Keep the current sweep_kf2 baseline and all original paths intact.
Seal your completed packet only after verification. If blocked or over the support
allowance, return the preserved subset and exact remaining issue.
