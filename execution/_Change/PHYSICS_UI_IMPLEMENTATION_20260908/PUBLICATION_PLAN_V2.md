# CHANGE publication plan V2

Status: **PREPARED; HELD FOR ROOT ACCEPTANCE OF THE FRESH NATIVE PACKET**.

This successor preserves `PUBLICATION_PLAN_V1.md` (SHA-256 `0a3a009ca3529b93b08e8243283a1879475ad9022c6e4c4ea838b0af5ca41195`) as historical. It binds the reviewed post-G0 corrections and the accepted native build while leaving execution held. The active role is CHANGE Agent 1, task `/root/implementation_change`, using `gpt-5.6-sol` with high reasoning and no descendants. The Owner's standing authorization covers scoped commit, push, branch publication, pull-request opening, and exact-head green-CI merge. It excludes force, bypass, history rewrite, and destructive cleanup.

## Current state and immutable inputs

- Branch: `codex/piping-physics-ui-implementation-20260908`.
- Current HEAD: `7b73460c5e2d85a9f050344069d211fea4af7b3e`.
- Historical source-freeze commit: `831bd21d4b62202662629db010bff9a8722bbc43`.
- The latest read-only GitHub observation records `main` at `781b478176db46ca66ebd38991607612e0ce9338`, identical to the integration parent already contained in HEAD, with zero commits and zero changed paths since that SHA. The observation is `GITHUB_MAIN_OBSERVATION_V2.json` (SHA-256 `a55db0ee4c8f7e44f1eab7536e158ddfdd810f75745c70e7afcb5cbc58ed828d`). Refresh at execution.
- The original local handoff must remain SHA-256 `ebbed866266cc961344151519b2792c6cfc5889eb18143a7809be69700c56035`.
- The accepted 17-member production/source/test/fixture cut is `instances/U7/post_g0_browser_flow/COMPLETE_BINDING_V1.json` (SHA-256 `7eaf4e22e3ed32b00b980938f1da5398907eb4dcbb7c93a2764c2446187ec5e2`), aggregate `f2dd0992528f9d4f00b36f97e845e0041e2b78189bbc7d37730f8cf8466f52dc`.
- The final U7 browser-flow manifest is SHA-256 `dc40d93d94dd806111645f38dcf66c62dce2163cd76eddafa41ba4a001e0edb9`; its 4/4 browser log is SHA-256 `875968a90feebaf7019f47a3cedb231d7f62e26a3b2e34827135492654979f68`.
- The RF final applied-correction packet binds MANIFEST `97fff5dc92c4b056bd120aba828ba91e91e2695207726380a0cbd110878fc641`, REVIEW `4a6fb4191140f762658e49e4a608d537d5f54c35280b8a59dfe5b1845ca88d7c`, RETURN `bb1aa244a1966cd27eb57d79a9a53a92e0e61312f8aad363b1da14c5f9999056`, and STATUS `8363a4c49384c1aed3edb9377d54f4d6f6a62184eae6b63aa3a1c0a1b086a0d9`.

The six reviewed correction postimages are exact:

| Path | SHA-256 |
| --- | --- |
| `projects/chirality-piping/apps/desktop/e2e/r2-smoke.spec.ts` | `da2efafca0594a417cfd3c03aa99c143325048e6419a7d3a1c4ff4434a5b82b6` |
| `projects/chirality-piping/apps/desktop/src/App.test.tsx` | `5fa1e1e0439690db4692033eda0ae66c98c79fb050ce2cd1214fb74c417d855c` |
| `projects/chirality-piping/apps/desktop/src/services/previewService.test.ts` | `02a773947223e27b4c01d3c7e208a2806e6b532a791423d025293dac03675c2f` |
| `projects/chirality-piping/core/product_physics/src/lib.rs` | `f1ae3322a752d92e2125274e7c5ccc0f7ca2c26a00f19d8d5cd3d6c4234a3ed5` |
| `projects/chirality-piping/fixtures/product_preview/invented_mechanics_result.json` | `e246338e5accd330c4fb7a7602510f15d8fac42cc4cee5c09fecbab227b0df13` |
| `projects/chirality-piping/tests/product_preview/test_product_preview_service.py` | `8598a69f44e915e4ba724a30fe5fd7e6381cac959dcec059a13666906e2a8735` |

The accepted native build is bound by `NATIVE_SOURCE_BINDING_FINAL_V2.json` SHA-256 `0f415e24a67127bbbaed95b25adfd8015c7f2013b0c885483d1d9c90ab6daef4` and `NATIVE_BUILD_RESULT_V2.json` SHA-256 `d2a7f2b6c90609bba75121977f053402fdd33e37062226d424503c5f550234e4`. Its 21-file dist inventory is SHA-256 `b121ba76522039d5bee7ad505c1116f7c308524d152e1ec1f44e8a5d4b709ed2`, and its three-file bundle inventory is SHA-256 `0b210e1337d8767e9abe940146d37313e3d20d471be05ed0cf3ab3679964c5f0`. The ignored bundle is build output and must not be staged.

## Entry release

CHANGE may execute only after root has read and accepted the fresh native default-fixture packet, supplied its exact paths and full hashes, confirmed the native Agent 2 stopped, and confirmed every writer stopped. The packet must truthfully bind this exact 17-member cut, the accepted V2 build, unique native identity, the bounded 830-row default-fixture solve, and retained-spring L-100 values `0.411203 N` friction and `41.120279 N` normal force. This plan does not predict the packet hashes or accept the native result.

At release, rehash the 17-member cut, six correction postimages, accepted build packet, fresh native packet, controlling C0/root records, and original handoff. Run the receipt validator before reading the latest receipt cursor. Require the current loop/DAG/status inventory, repository self-check, path-anchor validator, candidate-whitespace validator, JSON/LF/containment checks, exact allowlist, and stopped-writer state to pass. The failed sweep at `SWEEP_20260908T202715Z_7b73460c5e2d.json` remains historical failure evidence and cannot satisfy a release gate.

## Source freeze and current-main integration

1. Stage only the authorized 17-member implementation cut and its accepted run/control/review/native evidence. Exclude ignored dist, target, bundle, app-local store, temporary files, and unrelated paths. Commit the scoped clean pre-test container as the new **source-freeze commit**.
2. Fetch `origin` after the source-freeze commit. Record current `origin/main`, ancestry, branch divergence, and every upstream path since `781b478176db46ca66ebd38991607612e0ce9338`. Stop for root impact routing if upstream touches `projects/chirality-piping/` source, loop/control/receipt state, or accepted contract bindings.
3. If `origin/main` remains `781b478176db46ca66ebd38991607612e0ce9338`, no integration commit is needed because current HEAD already contains it. If main advances only through verified disjoint paths, integrate using the repository-established non-fast-forward merge method. No rebase, force, bypass, or silent conflict resolution.
4. Designate the resulting commit as the **tested source commit**. Require a clean tree and exact equality to the accepted 17-member manifest across `core/`, `apps/`, `fixtures/`, and `tests/`. Record any producer paths outside those four roots separately if the final accepted binding names them.

## Clean verification

From the clean tested source commit, execute the full unchanged DEC-025 five-surface sweep from `projects/chirality-piping/` using the established Python environment:

```sh
PYTHON_ENV/bin/python tools/release/run_evidence_sweep.py --execute
```

Require G0 through G4 all PASS and require the sweep summary's `commit_hash` to equal the tested source commit. Then, from repository root, run:

```sh
PYTHON_ENV/bin/python -m pytest -q tools/practitioner_harness
```

Preserve exact commands, exit codes, counts, durations, and lossless raw output under structural run records. Do not repeat a passing suite without a source change or concrete failure. Any failure is preserved and routed to root for bounded diagnosis and repair; it does not automatically produce an Owner-approval proposal. If a correction changes only tests or evidence, root determines from actual unchanged production bytes and the dependency chain whether the accepted native production binding remains valid; a native rebuild or repeat witness is not automatic.

## Receipt 137 and evidence-only closeout

Return the tested source SHA, full sweep evidence, practitioner result, exact source equality, and remaining dirty evidence paths to root. Root then releases C0 to append Receipt 137. Receipt 137 must name the validator-confirmed prior cursor and set `Examined-Through` to the tested source commit. It must bind the accepted corrections, RF/U7 review, fresh native packet, source freeze/current-main integration, 17-member equality, five-surface sweep, practitioner result, and continuing non-closure limits.

After root accepts the C0 handoff and Receipt 137, run the receipt validator before reading the new cursor. Then run the final G0-G4 record-binding checks, practitioner binding, path-anchor validation, candidate-whitespace validation, JSON/LF/containment checks, exact allowlist/status review, and staged-whitespace equivalent. Commit only the new sweep, Receipt 137, final C0 records, and concise CHANGE closeout as a separate **evidence-only commit**.

Require source equality after that commit:

```sh
git diff --exit-code TESTED_SOURCE_COMMIT..HEAD -- \
  projects/chirality-piping/core \
  projects/chirality-piping/apps \
  projects/chirality-piping/fixtures \
  projects/chirality-piping/tests
```

Also rehash every member of the accepted 17-member cut. A review-record hash change alone does not invalidate unchanged product bytes; root evaluates the actual affected dependency chain.

## PR and merge

Push `codex/piping-physics-ui-implementation-20260908` and open one pull request to `main`. Record the PR URL and exact head SHA. Read every required CI check and current mergeability for that exact head. Merge with the repository-established merge-commit method only when all required checks are green and the exact head is unchanged. No force, bypass, stale-head merge, or history rewrite. Fetch `origin/main`, verify the effective merge SHA, and record it in CHANGE closeout evidence.

This publication closes only the bounded implementation and evidence container accepted by the owning workflow. It does not mutate dependency rows, DAG/lifecycle/public-schema state, engineering compliance, professional reliance, or broader first-wave closure.
