# WORKING_ITEMS — compatibility review repair V10

Continue the existing compatibility manager and sole TASK writer, both gpt-5.6-sol / high. Resolve REPO_ROOT in the supplied compatibility checkout; WORKING_ROOT is its projects/chirality-piping. RUN is WORKING_ROOT/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260914-RESULT-COMPATIBILITY-PRESSURE. Preserve the frozen V10 source/evidence and prior failures.

The fresh independent reviewer of Root candidate e244a2479207fa32db23e150d6cb41544ddc829f identified F01: the canonicalization crate auto-discovers the checked CLI binary, but crate-level cfg(feature = checked-cli) removes main during ordinary feature-off builds. Cargo.toml has no required-features declaration.

Prepare the minimal repair in the compatibility checkout only. Exact product write scope: core/serialization/canonical_json/Cargo.toml relative to WORKING_ROOT; add the named binary target and required-features = [checked-cli]. Do not broaden behavior or touch other product files. Root integration stays frozen while review and native qualification proceed. The manager may dispatch the existing sole Type 2 writer with this sealed purpose; the writer does not delegate. Record actual child dispatch and hash this brief.

No builds, Cargo tests, browser or native operations are allowed until Root releases the shared native lease and explicitly grants the repair verification slot. Read-only static Cargo target inspection is allowed. Preserve a pre-repair reproduction expectation, a patch/source hash, and pending feature-off and enabled checked-cli acceptance commands under RUN/instances/COMPATIBILITY/REVIEW_REPAIR_V10/**. Put actual host bindings under _run_records. Do not invent an observed E0601; it remains a static source finding until reproduced.

Wait for any additional Root-routed reviewer findings before declaring the entire review cycle complete. Do not edit unrelated files or perform Git. Return the minimal prepared repair and binding to Root; runtime verification and integration remain gated by the shared lease, with routine engineering authority already granted.
