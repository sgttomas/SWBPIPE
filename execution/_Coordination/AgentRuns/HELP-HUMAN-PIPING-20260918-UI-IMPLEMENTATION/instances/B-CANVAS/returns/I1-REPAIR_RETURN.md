# I1-REPAIR return, retained (model that ran: Claude Fable 5.1, claude-fable-5-1, by its own statement)

Retained by B-CANVAS (ROOT's script). What follows is the agent's final message as the host stored it, with the machine's repository path replaced by `{REPO_ROOT}`; nothing else is changed. The message exactly as the host stored it has SHA-256 `b7ec8bf8d985479355adee6bf127b178a188a7fe0d94513939f73b775a72de31` (196 lines), by this script.

---

# Return: I1-REPAIR (first profile of the benchmark instrument)

**Model:** Claude Fable 5.1 (`claude-fable-5-1`), TASK (Type 2). I worked alone, launched no agent, and ran no git command that changes state.

Of the four defects, defect 2 is deliberate and I did not repair it; the verifier is byte-identical. Defect 4's nine files are not in the repository. Defects 1 and 3 were missing statements, now made. No limit, tolerance, oracle or recorded value moved. Both Playwright lanes pass, and every file under `fixtures/` and `samples/` plus `fixture-manifest.json` is byte-identical.

**Hashes verified before reading:**
- Brief: `590e1557cc913ed1b313bd1571360ea90ae146bf3df4d2e9bb6958b1307e8805`, matches.
- Proposal P4: `c7acfb609fb0800fcbe156bdd064326a642b42d9fbe352c2e4a071b6e14b0f09`, matches.
- Worktree at `7300e30c7` on `codex/swbpipe-b-canvas-20260918`, clean at start.

**What I read:**
- The brief and P4, whole.
- The owner's record `OWNER_DIRECTION_2026-09-19_SECTION7_DECISIONS.md`, through `git show` from `origin/codex/swbpipe-records-20260919`.
- `{UIF}/README.md`, whole (689 lines).
- The relevant parts of `benchmark-harness.ts`, `freeze-candidate-point-oracle.mjs`, `verify-winner-cue-plan.mjs` (whole), `characterization-observations.mjs` and its `.d.mts`, `verify-characterization-observations.mjs` (whole), `full-cohort-controller.ts`, `full-cohort-controller.spec.ts`, `generate-fixtures.mjs`, `verify-fixtures.mjs` (whole).
- Read-only, the 2026-09-15 production-UI run's records: `FINAL_REVIEW/ORACLE_PIN_AND_LAUNCH_V57`, `RUNTIME_ORACLE_PIN_V57_BACKCHECK`, `COMPLETE_SOURCE_V1/COVERAGE.csv`, `RAW_ARCHIVE_V15`, `CLOSEOUT_PREPARATION_V3/MANIFEST.md`, and the nine `D70_BASELINE` method manifests.

Below, `{PUI}` is `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/instances`.

## The four, each named as a change

### Defect 1: geometry-source value. A statement was missing; no behaviour changed.
**Found.** `benchmark-harness.ts:264` holds `c0c09ebd…5f8e`. The same value stands at `freeze-candidate-point-oracle.mjs:39`, `README.md:686` and `full-cohort-controller.spec.ts:566`. The four places agree.

**Evidence for what the value is.**
- `git show 8468a33c86adb622b25e98f98b0eaf28c7e9fa0e:…/viewportSelection.ts | shasum -a 256` gives `c0c09ebd…`.
- `8468a33c…` is `CHARACTERIZATION_PRODUCT_REVISION`, the merge of PR #789.
- `fixtures/frozen-oracle-geometry.ts.txt` hashes to the same value, so the fixture is those bytes.
- The file's history: `5e22bd14a` is `c0c09ebd…`; `683cd5ab2`, merged by PR #794 as `362dcffc0`, is `fdf3eaa4…`.
- The V57 backcheck return states the constant "now equals accepted c0c09e…".

**Before.** The constant's only comment was the V20S sentence.

**After.**
- A comment at the constant names the revision, the preserving fixture, that PR #794 moved the product past it, and that a run against a current source root stops by design.
- A two-line comment stands at the drift throw.
- The README has a new section, "First-profile repair of 2026-09".
- The value, the comparison and the error text are unchanged.

**No limit moved.** In `git diff benchmark-harness.ts` every changed line is an added `//` comment (counted: 0 non-comment lines).

### Defect 2: `verify-winner-cue-plan.mjs` pins `97b18c96…`. Deliberate; not repaired.
The file is byte-identical (`c43489ff…`).

**Evidence.**
- `97b18c96…` is the hash of `viewportSelection.ts` at no commit of any ref. Five distinct hashes exist across all refs: `47dd6062`, `bdbf4261`, `215d8192`, `c0c09ebd`, `fdf3eaa4`.
- It is a pre-merge working state. `{PUI}/RAW_ARCHIVE_V15/INPUTS.json` records it as `FUNCTIONAL/BOX_GESTURE_REPAIR_V30/preimages/src/features/viewport/viewportSelection.ts`, 42,792 bytes. `COMPLETE_SOURCE_V1/COVERAGE.csv` row 121 lists it as the then-current source.
- `{PUI}/FINAL_REVIEW/ORACLE_PIN_AND_LAUNCH_V57/RETURN.json`, field `historicalHelperBoundary`, says: "verify-winner-cue-plan.mjs is unchanged, uninvoked, and still pinned97b18c … do not claim the old helper passed".
- The same return calls the harness "a runtime consumer, separate from the intentionally historical offline helper".
- The backcheck return repeats "unchanged/uninvoked/unclaimed".
- The verifier's own logic is a one-transition control. It requires that component and support visual geometry changed between `--old-dir` and `--new-dir`, so it would reject two identical current generations whatever hash it pinned.
- Neither input directory is in the repository, so it cannot be run here, before or after.

**Proposal, not done.** Either add a header comment naming the generation it checks, or retire it to history. Give the second profile's plan verifier the shared-value module the brief expected. Sharing the harness's current value with this verifier would make it reject the only plans it was written to check.

### Defect 3: the two `34`s. Outcome (a): the literal is right; its statement was missing.
**Evidence.**
- `characterization-observations.mjs:196` (`oneSuccessHistory`) and `:278` (`verifyContinuationFiles`) are reached only through the D-70 continuation. The second hard-codes product revision `8468a33c…`.
- `requiredMethodFiles` had 34 entries at `84bb2a1f9`, `1d1d65541`, `b69bc4bcb` and `70ce64ee2`.
- It has had 38 since `683cd5ab2`, which added `fresh-demo-policy.mjs`, `.d.mts`, `.spec.ts` and `fixtures/frozen-oracle-geometry.ts.txt`.
- All nine recorded method manifests under `D70_BASELINE` list exactly 34 files. Their path set is today's list minus those four.
- Today's inventory is already checked from the one list at `full-cohort-controller.ts:410` (`method.files.length !== requiredMethodFiles.length`).

**Before.** Two bare `34`s.

**After.**
- An exported, commented constant `D70_CONTINUATION_METHOD_FILE_COUNT = 34`.
- A function `validateD70ContinuationMethodInventory(method)`, declared in the `.d.mts`.
- Line 196 uses the constant; line 278 calls the function.
- Acceptance and refusal are the same as before. The one behavioural difference is the refusal's text: it still begins "complete34 method inventory required" and now continues "(D-70 continuation generation; …)".

**Recorded artifacts.** A scratch script applied the original rule, then the new function, to every `*METHOD*.json` with a `files` array under `D70_BASELINE`. 9 of 9 verify before and 9 of 9 after. `verify-characterization-observations.mjs` passes with identical output before and after.

**No limit moved.** 34 is still 34. A 38-entry inventory is still refused by the D-70 path and still required by the controller. I did not derive the 34 from today's list, because that would make the recorded manifests unverifiable.

### Defect 4: the generator's `protocol-history/`.
**Found.** `generate-fixtures.mjs:663` read nine files that do not exist. The read came after the eleven fixture writes and before the manifest write, so a run rewrote every fixture and then threw `ENOENT`.

**Where the nine files live: not in the repository.**
- `git log --all -- '*protocol-history*'` is empty, and no path of any ref matches.
- I took the SHA-256 of all 126,709 blobs in the object store (`git cat-file --batch-all-objects`). None matches any of the nine pinned hashes.
- `CLOSEOUT_PREPARATION_V3/MANIFEST.md` gives the reason: "Keep protocol-history snapshots as evidence rather than current method source."
- On this host a separate, older checkout of the verification lane, outside this repository, still holds them as untracked files. All nine match their pinned hashes, and a README there explains each supersession. Nothing in the repair depends on that checkout.

**Change, with the nine pinned hashes unchanged.**
- The history is verified before the first write. Before, a missing history left the fixtures rewritten and no manifest; after, the run refuses with nothing written.
- The history comes from `--protocol-history-dir <absolute directory>`. The old default directory is still honoured if it exists.
- A new `--check` mode regenerates everything in memory and compares the 12 inventoried files and the manifest with the frozen bytes, by SHA-256 and byte count. It writes nothing.
- Without a history, `--check` reports `NOT_SUPPLIED_NINE_FILES_NOT_IN_REPOSITORY`.
- Unknown arguments are refused.
- `verify-fixtures.mjs` is not that check: it compares the frozen bytes with the manifest and never regenerates. It is unchanged, and I built no second copy.

**Result.**
- `--check` gives `PASS_CHECK_REPRODUCES_FROZEN_BYTES_NOTHING_WRITTEN`: 13 compared, 0 differing, manifest `6e7fba8f…8739`.
- With the external directory supplied it also gives `VERIFIED_NINE_PINNED_HASHES`.
- A negative control in a scratch copy, with three bytes appended to one fixture, gives `FAIL_CHECK_…` and exit 1.
- I never ran the generator in writing mode in the tree.

**Proposal, not done.** ROOT decides whether the nine files and their README are preserved in the repository as evidence.

### Tests (failing, then passing)
Three tests titled "first profile repair: …" now end `full-cohort-controller.spec.ts`. The ordinary source lane runs them (`npm run test:e2e`, root `playwright.config.ts`, both projects). There is no new file, so `requiredMethodFiles` and `full-cohort-controller.ts` are untouched.

**Before the repairs (focused run: 4 failed, 2 passed).**
- Test 1 failed: the constant's comment lacked `8468a33c…`.
- Test 2 failed: `D70_CONTINUATION_METHOD_FILE_COUNT` was `undefined`, not 34.
- Test 3 passed, on purpose. I repaired the generator before that test ever ran in the tree, because the unrepaired generator ignores `--check` and writes. Its failing evidence is a scratch copy outside the repository: exit 1, `ENOENT …protocol-history/fixture-manifest-v1-…`, with the scratch fixtures already written.

**After.** All 6 pass.

## What the first profile binds once repaired
The first profile is the recorded demonstration of product revision `8468a33c86adb622b25e98f98b0eaf28c7e9fa0e`. It binds five things:
- That revision's `viewportSelection.ts` (`c0c09ebd…`), preserved as data in `fixtures/frozen-oracle-geometry.ts.txt`.
- The cue source `viewportSelectionPresentation.ts` (`00384d28…`).
- Fixture manifest `6e7fba8f…`, which the generator's check mode reproduces by hash.
- For the D-70 continuation, 34-file method manifests.
- For the fresh N10000 demonstration, the 38-file list.

PR #794 moved the geometry source to `fdf3eaa4…`, and slice C1b changed the selected colour. The first profile therefore cannot be run to a pass on today's product. A current source root stops at "winner product geometry/cue source drift" by design, and the profile is not rebound. `verify-winner-cue-plan.mjs` is a historical helper of an earlier generation, not a check of a current plan.

## Fifth-defect trace (read-only; nothing fixed)
I found 71 distinct 64-hex literals outside `fixtures/` and `samples/`.
- 21 are the hash of a file in the repository today.
- 9 are the protocol history (defect 4), and 1 is `97b18c96…` (defect 2).
- 23 are the spec's frozen tree-query table: `d61b6dd8` names ROOT's manifest, and 22 are id lists the test recomputes on every run, one of them the hash of `[]`.
- 17 bind artifacts that were never committed:
  - Policy documents: `bc0cb199`, `8195cd97`, `ef009d11`, `12ea9947`.
  - Visual tokens V4: `009b27db`.
  - Chromium and Perfetto sources: `dc3a3b53`, `6a5fa3cf`, `f8065edb`, `2491cb64`, `ff28d260`.
  - Causal-method diagnosis records: `f87fd653`, `15102056`, `dce9bc5a`, `bc21adf3`.
  - The original extractor: `6fbef3d3`.
  - A rejected snapshot: `9d7b0d55`.
  - The focused box ordering: `1b1eced5`.

Every literal stated in more than one place is stated consistently. I found no fifth stale or contradictory literal. One observation for the second profile: most policy preimages are outside the repository, so their hashes can be compared here but not re-derived.

## Files (SHA-256 after)
| File | SHA-256 after |
|---|---|
| `README.md` | `95075ceb96257f2c5c3e6056e8c0c2d593c14bc5e1c17e3628ab1e129dd13fe1` |
| `benchmark-harness.ts` | `fd4e463b2b2c9b644f415824106b5da501d6d027b95b314a09431f788228946a` |
| `characterization-observations.mjs` | `4728148dbe80bd2e8faff25f098ed6e535cf882f8f90498d5b54361b85be7bb1` |
| `characterization-observations.d.mts` | `e0b20eac7dfe880665849e06329389a09a8f2a76f342dae094ee0393c10da013` |
| `full-cohort-controller.spec.ts` | `8da4857fb3a2139db0972ce84c0b561e37a846900b2e7507589fc519d5334b6e` |
| `generate-fixtures.mjs` | `046e9e1205c9ede6cc5269bcf61a8be72e983036364f73538c6204de7f73db21` |
| `REPAIR_2026-09_FIRST_PROFILE.md` (new) | `e9af8dd80fd81f492b11923f1882c2090f360618b50bd07be078bcf09a7ce118` |

All seven are under `{UIF}`.

Unchanged:
- `verify-winner-cue-plan.mjs`: `c43489ff…`
- `freeze-candidate-point-oracle.mjs`: `d8c7a0cd…`
- `full-cohort-controller.ts`: `356dae65…`
- `viewportSelection.ts`: `fdf3eaa4…46d5`
- `viewportSelectionPresentation.ts`: `00384d28…5931`
- `fixture-manifest.json` and the 12 files under `fixtures/` and `samples/` (13 hashes before, equal after).

`git status --short` shows only the six modified files and the one new file, all in `{UIF}`.

## Commands, exit status and counts
| Command | Before | After |
|---|---|---|
| `npx vitest run src/features/viewport/viewportSelection.test.ts` | exit 0, 69 passed | exit 0, 69 passed (run twice at the end) |
| `verify-fixtures.mjs` | exit 0, PASS, 12 bound files | exit 0, output identical |
| `verify-point-hit-oracle.mjs`, `verify-box-selection-oracle.mjs`, `verify-characterization-observations.mjs`, `verify-causal-presentation-extractor.mjs` | exit 0 each | exit 0 each, output byte-identical |
| `verify-winner-cue-plan.mjs` | exit 1, "absolute new/old oracle directories required"; inputs not in the repository | not runnable, as before |
| `generate-fixtures.mjs --check` | (scratch copy) exit 1, ENOENT, wrote files | exit 0, PASS, 13 compared, 0 differing |
| `npx tsc -b` | exit 0 | exit 0 |
| `npx tsc -p e2e/ui-foundation/tsconfig.causal-method.json` | exit 0 | exit 0 |
| `python3 tools/validation/validate_claims_language.py` | exit 0, 338 files VALID | exit 0, 338 files VALID |
| `npm run test:e2e` (through the lock, 1 worker) | exit 0, 374 passed, 20 skipped | exit 0, 380 passed, 20 skipped |
| `npm run test:e2e:dist` (through the lock, 1 worker) | exit 0, 53 passed | 53 passed |

- **Source lane.** The count rose by 6. A name-by-name diff shows exactly the three new tests in each of the two projects, nothing removed, and the same skipped tests.
- **Dist lane.** After the repair it ended "53 passed"; the second focused run shared its shell command, so no separate exit status was captured.
- **Focused runs.** Both went through the lock. The first waited about ten minutes for another lane's lock and let it go first.
- **Host state at the end.** Ports 5174 and 5175 are not listening and the lock is released.
- **Opt-in attempt.** I ran the skipped D-70 continuation tests with `D70_WRITER_ORIGINAL_RETURN` pointing at the recorded `RUNNER/RETURN.json` (`b9cd6955…`).
  - The 14 continuation tests fail at `validateContinuationPolicy` with "canonical original seed path required". The recorded seed binds the original machine's absolute paths. That failure is in unchanged code and is reached before the lines I touched, so it is evidence neither way.
  - 4 passed, among them the internal120 transition test.

## Judged not to repair, with proposals
- Defect 2, the verifier: a header comment or retirement, plus a shared-value module for the second profile's verifier.
- The preservation of the nine protocol-history files in the repository, which is ROOT's decision.
- The "34 entries" and "34-member" sentences in the README's D-70 sections are left as the historical statements they are. The new README section explains them.

## Not done
- No benchmark (`*.benchmark.ts`) was run.
- No writing run of the generator was made in the tree.
- Nothing was edited outside `{UIF}`. No product, fixture, sample, policy, oracle, target or evidence file was touched. No git state was changed.
- No control was touched, so the "contrast not yet checked" list is empty.
- No semantic change was made to the product.
- I claim no performance, usability or conformance acceptance.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
