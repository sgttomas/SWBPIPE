**PASS — the applied parity patch and bounded CI exception match the approved scope.**

Reviewed `7a37a8f2b2d4f55460ebeaa7d201bae4d840cf9b` against `efaef7e7fd50af3763c0e8a53bc23ee838ad6f32`.

- Committed dist SHA-256 is exactly the previously reviewed `f6def858ca800f0f65fa338a92847e6ad8e0b11052dc6063f56d31d809cca4f3`.
- Only the named post-wheel assertion changed. Product/source-suite inputs, other oracles, geometry, endpoints, timeouts, and skips remain unchanged.
- CI hashes the **committed blob** for that exact path. In-memory probes confirmed altered contents and wrong-PR cases fall back to full coverage.
- Binary-evidence allowance is confined to the named `B3_DIST_ROUNDOFF/` directory and permitted extensions. Scripts and adjacent directories remain excluded.
- The actual candidate validates as `pr825-repair`, explicitly partial, against integrated target `21175b5d3668f29acd408812d95a33ed4f14bfc8`.

The approval record explicitly covers both parity and PR825-only bounded reruns. Documentation preserves the failed065 sweep and labels combined verification accurately. The sealed brief matches; scoped `git diff --check` passes. Retained policy evidence reports **30 tests passed**, log SHA-256 `e67f47ea4f8d191a51c3d5ec84d12aa3f2b7edd71fd274de6435b2afdf1967cb`.

No browser/native/hosted execution, writes, Git mutation, or delegation occurred. Corrected full dist, remaining build, final unchanged-input bindings, candidate collection, and hosted CI remain pending; no pass is inferred from the running checks.

Independent TASK/Astra-xhigh attribution and existing telemetry limits apply. Standard F-PIP-2 / DEC-081 claim fence applies.
