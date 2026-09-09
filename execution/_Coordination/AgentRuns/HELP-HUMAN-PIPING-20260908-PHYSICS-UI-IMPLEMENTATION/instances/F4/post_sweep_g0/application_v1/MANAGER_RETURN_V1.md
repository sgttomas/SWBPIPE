# F4 six-path application manager return V1

Status: `BLOCKED_VALIDATED`

The Owner-approved V4 patch was applied exactly once. Its decoded/applied SHA-256 is `c31a705a39c4629aacf6a79871c1497a8b8d3976e389478f79e41f4f7752c334`; all six live post-images match the approved values, reverse application reconstructs all six preimages, and the scoped diff passes `git diff --check`.

The focused Rust witness passed 1/1; the complete product-physics crate passed 138/138 with zero doc-tests; Python passed 9/9; preview-service passed 16/16; each named App test passed 1/1; and the registered generator output is byte-identical to the live fixture at SHA-256 `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13`.

The prescribed browser smoke failed 0/2 before reaching the changed numeric assertion. Both configured browser projects timed out at `apps/desktop/e2e/r2-smoke.spec.ts:552` because `queue-explicit-node-intent` remained disabled. No retry, assertion change, or weakening occurred. Root-routed RU diagnosis identifies omitted draft provenance in the stale smoke flow; that separate candidate requires its own authority and review.

Exact failed-browser evidence is bound under `{DEL_RUN}/post_sweep_g0/application_v1/child_I3/**`:

- raw browser log preserved as member `VALIDATE_BROWSER_SMOKE.txt` in `LOSSLESS_TEXT_EVIDENCE.b64.json`, original SHA-256 `e80398806723565a79a1770dcb4d7e4279286daaa6237930c8865ded4f837482`; active pointer SHA-256 `9a1ee1a719b4f482fb243fd522116d749946a6b2ee663ee213ac91ec678e723d`
- compact error context original SHA-256 `0e786e835aaf65906ab4203ae13cc3bfc8d71237af112bd5657f39491952d394`, trace SHA-256 `3a60f634dbe86ce7cecfe975455ebd0890cb540f7907eda8d4995755ed4521eb`
- desktop error context original SHA-256 `0e786e835aaf65906ab4203ae13cc3bfc8d71237af112bd5657f39491952d394`, trace SHA-256 `fe6608ea5b1f482a7023f86c6aaa678365f629487ae42d1635b074e9f6fdd1df`

The 17-entry complete binding is `032d50ea69013be10f87257d8e30c9fc676704a0abfa77a4aabe7f1856555e63`, with aggregate `2eb48c1af8d0d6d13e64204fb0fefd70da6ed3c89493ee206d3ae2fb97c5af8e`. Child RETURN is `724a70361c770858e70c569cc5f4119259b5b613e511302c932a4ba1d9399c9f`; child STATUS is `b6719b950fdb15e6b8083440eaf75e5ed64359951f3129abac683e84a9d88afa`; manager validation is `e4e784552221baad8c193dad1c26c91f810bc924897f8f460ed0d38148250908`.

The three raw text files with generated trailing whitespace were preserved in the lossless bundle and replaced by transparent pointer successors under scope `5faa148be350c147214e30b4dac3e8420e7d6f26642245fe0dcdb11e36763e05`; correction manifest SHA-256 is `7b482b068f2416fe4adb4ae959745b4b40a939c5c03da2e400261935c4e3e5d0`. F4's application and evidence subtree has no path-anchor or candidate-whitespace finding. Global validators still report disjoint RU evidence findings outside F4 ownership.

The six applied post-images remain frozen. Acceptance, native build, full DEC-025, practitioner checks, Git acts, and publication remain held for root routing. F4 releases exclusive six-path source and test resources.

`{DEL_RUN}` means `projects/chirality-piping/execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-04_Nonlinear support active-set solver/_run_records/PHYSICS_UI_IMPLEMENTATION_20260908`.
