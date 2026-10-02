# I24 runner hold repair return

Repaired the automatic pre-admission binding order in both runners. A pure H eligibility helper now checks existing named refusals, conditional holds and recorded ascent before either runner starts a model-specific counts or normal subprocess. The tier no-op baseline remains.

After eligibility passes, the actual planned context is rebound before numeric admission and denominator use. No stale estimate fallback or fake-count admission was added. Standalone explicit counts acquisition and CLI are unchanged, as are VR's 512MiB counts cap, H's existing normal-cap prepass, normal caps and numerical thresholds.

Legacy direct admission/plan reason priority and result fields are preserved. All 8,120 H and 864 VR pure admission comparisons against the base returned equal dictionaries. Preflight held run records intentionally lack a freshly bound estimate. Prior error, timeout or abort records still satisfy recorded ascent; None/not_run do not.

Final verification: 54 H runner tests and 15 VR runner tests pass. Stub-launch regressions cover H CHAIN100 ascent, conditional and named-never holds; VR 10000 approval/ascent holds; and permitted recorded cases with bind→numeric→normal ordering and matching denominators/caps. Held models launch no prepass or normal process; the tier baseline is explicitly permitted. No real 10000 run or build occurred.

Exactly four maintained files changed: H runner and its test, VR runner and its test. Full diff, postimage hashes/archive, supplied basis/native parentage, verbatim brief/clarifications, raw checks and exact timing are under [_run_records](_run_records/OWNED_POSTIMAGES.json). No sealed packet was edited. Return to ROOT for full-diff/hash verification, commit and same-reviewer backchecks; no independent review or acceptance is claimed.
