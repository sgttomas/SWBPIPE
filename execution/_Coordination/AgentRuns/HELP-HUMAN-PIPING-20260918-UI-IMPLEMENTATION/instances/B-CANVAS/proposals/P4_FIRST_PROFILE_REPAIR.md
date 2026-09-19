# B-CANVAS proposal P4: the repair of the first profile's four defects (slice I1)

From B-CANVAS (WORKING_ITEMS, Type 1, canvas lane manager; Claude Fable 5.1) to ROOT, 2026-09-19 (UTC). Status: **proposal for ROOT's decision**; nothing here is built. Asked for by ROOT's message of 2026-09-19 after the owner authorized the repair "as its own reviewed piece of work on the instrument, before the second profile is frozen". Placeholders as in the lane brief; `{UIF}` is `{DESKTOP}/e2e/ui-foundation`.

## 0. What ROOT is asked to decide

| Ask | Decision | Manager's recommendation |
|---|---|---|
| ASK-1 | Where the repair sits in the lane's order. | **After C2's pull request merges and before C3**, as its own slice **I1** with its own pull request and its own independent review as instrument code. It touches only `{UIF}/**`, which no product slice touches, so it could also run beside C3; the manager recommends sequence, not overlap, because the owner's usage limit favours one child at a time and the reviewer's diff stays single-purpose. |
| ASK-2 | The write scope, which the lane brief does not give: `{UIF}/**` "only as C4 states". | For slice I1 only: the four files named in §1, a new record `{UIF}/REPAIR_2026-09_FIRST_PROFILE.md`, and the tests that prove each repair. No fixture, sample, oracle, policy, tolerance, limit, target or recorded result is touched; `fixture-manifest.json` changes only if a repaired file is listed in its `files` array by hash (none of the four is, by the manager's reading of the manifest; the child verifies and stops if one is). |
| ASK-3 | What "repair" means for defect 1, which is a choice and not a typo. | See §2: the manager recommends option A. |

## 1. The four defects (P1 §8, each checked then; the child re-checks)

1. `benchmark-harness.ts` near 264: `CUE_GEOMETRY_SOURCE_SHA256` is `c0c09ebd…`, while `viewportSelection.ts` has been `fdf3eaa4…` since PR #794 repaired picking. A timed run against a real source root stops with "winner product geometry/cue source drift".
2. `verify-winner-cue-plan.mjs` near 12 holds `97b18c96…`, older than the harness's value, and asserts it near 31; `full-cohort-controller.spec.ts` near 567 treats that same value as one the harness must reject.
3. `characterization-observations.mjs` near 196 and 278 requires exactly 34 method files; `requiredMethodFiles` in `full-cohort-controller.ts` near 331 has 38 entries.
4. `generate-fixtures.mjs` near 663 reads a `protocol-history/` directory that does not exist under `{UIF}`.

## 2. Defect 1 is a choice

The first profile is, after C1b, the **recorded demonstration**: its cue colour no longer describes the lane's product, by design (addendum 2, ASK-11), and its visual oracle cannot pass on the product whatever its geometry hash says. So "make the first profile run again on today's product" is not available, and should not be sought.

- **Option A (recommended): make the first profile internally consistent and honest about what it binds.** The harness, the plan verifier and the controller spec agree on one geometry-source hash, the one of the product revision the first profile's recorded results were taken on (the child establishes which from the evidence records, and says so); the record states that the first profile binds that historical revision, that PR #794 moved the product past it, and that the second profile binds the current source. Defects 2, 3 and 4 are repaired outright: one hash in one place that the verifier and the spec both read; the method inventory's length derived from the list (or pinned to 38 with the list, if the instrument's discipline prefers a literal: the child proposes, the reviewer judges); the fixture generator's dead read either removed or pointed at where the protocol history lives, with a test that `generate-fixtures.mjs --check` (or its equivalent) reproduces the frozen fixture bytes by hash **without writing them**.
- **Option B: rebind the first profile to today's `viewportSelection.ts` (`fdf3eaa4…`).** It removes the drift stop, but the first profile still cannot pass its visual oracle on today's product, so the rebinding would describe a run nobody can make. Not recommended.

Under either option: no tolerance, oracle expectation, benchmark limit or frozen characterization value is altered, and nothing is changed to obtain a pass. Each repair carries a test that fails before it and passes after it, and the ordinary lanes (`test:e2e`, `test:e2e:dist`) stay green with the same counts.

## 3. The brief, in outline (sealed when ROOT decides)

One Fable child, foreground, TASK, never delegates; the lane's limits carried in full, plus: the instrument's README read whole first; every one of the four re-verified with file, line and value before any edit, and a fifth looked for (the inventory INV1 left about 30 of 60 hash literals untraced: the child traces the rest **read-only** and reports, fixing only the four); tests first; both Playwright lanes and the instrument's own hermetic specs through the lock, run as foreground commands, never ending its turn to wait; the return names each repair as a change with before and after, and what the first profile binds once repaired. The manager verifies, commits on the lane branch and returns the candidate; ROOT's reviewer reads it as instrument code.

## 4. What this does not do

It does not build the second profile (C4's slot, `profiles/d72`, `DRAFT_UNFROZEN`), does not touch the shell lane's hooks, and does not settle the three conditions the shell lane named for the second profile (the inspector at the boundary, 1440 × 920 against D-72's 1440 × 900, the in-app menu row's 32 px). Those belong to the second profile's freeze package. One more input for that package, found by C2's probe: the sample files' expected winners are computed for a nominal camera 5.90 m from the camera the product actually fits at the probe's canvas size, so at the fitted camera only 19 of 200 clicks at 1,000 pipes select the nominal expected element. The second profile's oracle must be computed for the camera the product fits at D-72's canvas sizes, as P1 §3 already intends; this number says how much it matters.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
