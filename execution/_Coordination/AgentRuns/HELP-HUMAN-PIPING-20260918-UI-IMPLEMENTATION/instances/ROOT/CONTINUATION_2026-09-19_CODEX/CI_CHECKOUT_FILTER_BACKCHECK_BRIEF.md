ROOT to existing independent TASK b3_code_review (Astra/xhigh), read-only/no
writes/Git/UI/tests/delegation. Narrow backcheck ROOT's working change in ROOT
continuation checkout, based06548567: exactly three identical additions of
filter: blob:none after existing fetch-depth:0 in Piping CI's selection/barrier/
remainder checkouts, with explanatory comments. Same existing pattern is already
used by governance-harness.yml. Full history remains for ancestry; full working
checkout remains; only historical blobs become lazy-fetched. Aggregate shallow
checkout unchanged. Motivation: avoid downloading all historical run-evidence
blobs three/four times on isolated runners. No test, selector, source, timeout,
assertion, scope or job-state change. Assess supported v4syntax and completeness
of current-source/ancestry/history reads; return actionable finding or PASS.
ROOT writes this in its own checkout while wt2 runs clean065 sweep, so no sweep
source is modified. Final candidate will contain this reviewed workflow-only tail.
