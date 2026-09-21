**Scoped PASS — no actionable findings.** Independent TASK `/root/ci_cache_review`, Codex harness-native Astra/xhigh, reviewed the four maintained paths and supporting `CI_HINT_REFRESH` evidence for `1251dc91981dd93688955d5cc19c6ee9afd37eb4..cafc84827a71be6172c1811dcae1c2aaa3e65d7c`.

Reviewed paths:

- `docs/governance_harness/tranche_manifests/PIPING-CI-CACHE-WARMING-20260921.yaml`
- `projects/chirality-piping/docs/CI_STRATEGY.md`
- `projects/chirality-piping/execution/_Coordination/NOTICE_2026-09-21_CI_CACHE_WARMING.md`
- `projects/chirality-piping/tools/ci/e2e_duration_hints.json`

Sorted four-path inventory SHA256, UTF-8 with final LF: `8fb6e0172457f892f78f2b3c3ac04e97ae2ba88cd4b7c1462647b65fde712583`. The launch brief and all eight supplied content bindings matched; working bytes matched the frozen candidate. Refreshed hints SHA256: `ec333f0b55433efc6f093f7b4635fb7fd8259ca00586fc3fdf6575207bd6b7fa`.

Independent in-memory calculations confirmed:

- Unique canonical mapping of 435 passes and 20 skips, forming the complete disjoint 455-identity union; all 23 tagged identities resolve through recorded metadata.
- All duration values and units match the raw log. Twenty newly measured identities total 106.1 seconds.
- All 23 prior-only entries, 20 existing `0.05` skip weights, and the 30-second fallback remain unchanged.
- The single zero-millisecond report retains its existing `0.001` scheduling floor, explicitly distinguished from measured time.
- Deterministic partitions preserve exact coverage, the 30-test accessibility barrier, and all 103 grouping boundaries. Old partitions also match actual corrected-run job membership.
- Recorded-duration shard sums reproduce exactly: old `251.476 / 236.551 / 375 / 476.945`; refreshed `334.727 / 334.772 / 335.231 / 335.242`.

**Provenance distinction:** the retained collection and plan originate at `d02bb990…`; successful run `35567286692`, its durations, and replayed source bodies bind to `80770322…`. I verified that the only intervening relevant source change replaces `Meta+z` with `ControlOrMeta+z` inside one test body. Test identities, tags, locations, selector, configuration, and grouping remain compatible. This supports the replay without representing the earlier collection as a fresh collection from `80770322…`.

The initial two policy errors and corrected 31-test pass are retained accurately. Reversing the supplied patch in memory recovered the exact recorded initial script hash; reconstructing the initial zero-weight data recovered its recorded hash. No validator or assertion was relaxed. Scope validation and bounded `git diff --check` passed.

No tests, builds, browser/native actions, network operations, file writes, Git mutations, or descendants were performed. `refresh.py` was not executed. Other Node/native records are outside this verdict.

The documentation appropriately limits the totals to historical recorded-duration replay. Final combined-candidate review, applicable G4/CI checks, actual main-cache creation, consumer hits, and future timing observations remain outstanding under ROOT ownership.
