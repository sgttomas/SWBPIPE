**PASS — all three CI findings are closed. No additional actionable defect found.**

Reviewed frozen `7174f78b7bda3497ab9007d3aaef5850f1fd3b54` against `097decd6d4c89bacb0909c9c798933835ed0f27b`. Current integration `065485671c7bf1c9b100643316bb263dd39421d8` has no further CI or product-source changes.

- **Scope fallback:** directory-wide instrument reduction is removed. Shared helpers/fixtures require full coverage; source-spec-only reduction remains. Retired `instruments` mode cannot pass aggregation.
- **Candidate identity:** plans separately retain the event target base, require its ancestry in the tested head, and validate PR number/head/base against the hosted event. Missing, unavailable, unintegrated, or relabelled identities fail before browser execution.
- **Collected coverage:** each runner validates actual collected IDs before execution. Required titles occur exactly once per profile; selected files are nonempty in both profiles; full barrier/shard coverage is exact once. Missing, duplicate, unexpected, or empty coverage fails. Structured artifacts distinguish collection from execution and retain omitted IDs.

Strategy documentation and the receiving notice match these rules. Exact reproduction-file allowances do not create a generic executable exemption. Pinned setup, test configurations, existing assertions, and product bytes remain unchanged.

Independent read-only checks:

- Verified all **3 maintained-output hashes**, **3 preserved-file hashes**, and **48 evidence hashes**.
- Ran **8 pure collection/aggregation unit tests**, all passing.
- Revalidated retained raw collections: **436 full**, partitioned **30 + 147 + 59 + 144 + 56**; focused coverage **68 selected / 368 omitted**.
- Exercised hosted-identity and missing/nonancestor-base rejection through in-memory probes.
- Validated the current checkout’s PR825 plan against target `21175b5d3668f29acd408812d95a33ed4f14bfc8`: `pr825-repair`, explicitly partial.
- Scoped `git diff --check` passes.

The backcheck brief hash matches `df03482897c9dbe487f6ec3c36c9498738f6fc00941191a47e5c619064bfb2a4`; repair evidence index hashes to `9daab9b30efed9a6913c7930a40a3f2730a6dbd8cad2b2b8e8798fe4db0b052e`. The worker’s **28-test pass** remains retained evidence.

Suitable for CI-strategy fan-in. Final candidate selection must be regenerated after further changes; live-main revalidation, clean sweep, and required hosted execution remain owed. No browser/native/hosted run, writes, Git mutation, or delegation occurred here.

Independent TASK, parent ROOT; existing Astra/xhigh allocation and telemetry limits apply. Standard F-PIP-2 / DEC-081 claim fence applies.
