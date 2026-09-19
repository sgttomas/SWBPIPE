# First-profile repair of 2026-09

Record of slice I1 of the canvas lane (run `HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`, instance
`B-CANVAS`, child `I1-REPAIR`, TASK, Claude Fable 5.1), 2026-09-19. Authority: the owner's direction
of 2026-09-19, section 7 decisions, group 3(b): the repair of four defects of this instrument's first
profile "as its own reviewed piece of work on the instrument, before the second profile is frozen";
ROOT accepted the lane's proposal P4 and decided defect 1 as option A (the value stays).

**The rule this work was done under.** No tolerance, oracle expectation, benchmark limit, target,
fixture, sample, policy or recorded result is altered; nothing is changed to obtain a pass;
`fixture-manifest.json`, `fixtures/**` and `samples/**` are byte-identical before and after (thirteen
files, SHA-256 compared). This record claims no performance, usability or conformance acceptance.

Paths below are relative to `apps/desktop/e2e/ui-foundation/` unless they start with `src/`,
`execution/` or `apps/`; `{D70}` is
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/instances/ROOT/D70_BASELINE`
and `{PUI}` is `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260915-PRODUCTION-UI/instances`.

## What the first profile binds

The first profile is the **recorded demonstration**. It binds product revision
`8468a33c86adb622b25e98f98b0eaf28c7e9fa0e` (`CHARACTERIZATION_PRODUCT_REVISION`, the merge of PR #789):
the geometry source `src/features/viewport/viewportSelection.ts` of that revision
(`c0c09ebdb05a49565bf61e576ff0b391037916c614f5add8f05f4270539f5f8e`, preserved byte for byte as data in
`fixtures/frozen-oracle-geometry.ts.txt`), the cue source `viewportSelectionPresentation.ts`
(`00384d2831797e5cba8acff21e82e36f21a853c398e8c2a801ccfed0f3a55931`, still the product's bytes today),
the fixture manifest `6e7fba8fdba11853ad7aa558c7e7c29ffb1a82e79c11a1b7633827335f458739`, and, for the
D-70 continuation, 34-file method manifests. PR #794 moved the product's geometry source to
`fdf3eaa4...`, and the lane's slice C1b changed the selected colour, so the first profile cannot be run
to a pass on today's product, **by design**: a current source root stops at "winner product geometry/cue
source drift". It is not rebound. A later product is bound by a later profile, which this work does not build.

## Defect 1: the geometry-source value (a statement was missing; no behaviour changed)

Found: `benchmark-harness.ts` `CUE_GEOMETRY_SOURCE_SHA256 = "c0c09ebd...5f8e"`;
`freeze-candidate-point-oracle.mjs` line 39, the README's last paragraph and
`full-cohort-controller.spec.ts` line 566 state the same value. **No inconsistency between the four places.**

Evidence for what the value is:

- `git show 8468a33c86adb622b25e98f98b0eaf28c7e9fa0e:projects/chirality-piping/apps/desktop/src/features/viewport/viewportSelection.ts | shasum -a 256` gives `c0c09ebd...5f8e`.
- `shasum -a 256 fixtures/frozen-oracle-geometry.ts.txt` gives the same value: the fixture is those bytes.
- The file's history on this branch: `5e22bd14a` (2026-09-16) `c0c09ebd...`; `683cd5ab2` (2026-09-17,
  merged by PR #794, `362dcffc0`) `fdf3eaa4...`. `8468a33c` (PR #789) lies between them.
- `{PUI}/FINAL_REVIEW/RUNTIME_ORACLE_PIN_V57_BACKCHECK/RETURN.json`: "CUE_GEOMETRY_SOURCE_SHA256 now equals
  accepted c0c09e... used by the generator, actual source and all new plans."

Change: a comment at the constant and one at the drift stop say which revision the value is, that the
fixture holds the bytes, that PR #794 moved the product past it, and that the stop is by design; the
README says the same. Before: the constant's only comment was "V20S verified gesture-lifetime additions
preserve primitive geometry; admit that exact source." After: as above. The value, the comparison and the
error text are unchanged.

No limit moved: the constant, the freezer's literal, the spec's expectation and the fixture are
byte-for-byte what they were (`git diff` shows only added comment lines in `benchmark-harness.ts`).

## Defect 2: the offline plan verifier's older value (deliberate; NOT repaired)

Found: `verify-winner-cue-plan.mjs` line 12 `geometryHash = "97b18c96...6aee"`, asserted at line 31 for
every plan of `--new-dir`; the harness binds `c0c09ebd...` and the spec names `97b18c96...` as a value the
harness must reject. The two cannot accept one plan.

The history shows this is deliberate, so the file is **byte-identical** and nothing was "fixed":

- `97b18c96...` is not the hash of `viewportSelection.ts` at any commit of any ref (five distinct hashes
  exist in the object store: `47dd6062`, `bdbf4261`, `215d8192`, `c0c09ebd`, `fdf3eaa4`). It is a
  pre-merge working state: `{PUI}/RAW_ARCHIVE_V15/INPUTS.json` records it as
  `FUNCTIONAL/BOX_GESTURE_REPAIR_V30/preimages/src/features/viewport/viewportSelection.ts`, 42,792 bytes,
  and `{PUI}/FINAL_REVIEW/COMPLETE_SOURCE_V1/COVERAGE.csv` row 121 lists it as the then-current source.
- `{PUI}/FINAL_REVIEW/ORACLE_PIN_AND_LAUNCH_V57/RETURN.json`, `historicalHelperBoundary`:
  "verify-winner-cue-plan.mjs is unchanged, uninvoked, and still pinned97b18c. It also asserts that
  component/support visual geometry changes relative to a historical oracle, so it is not the appropriate
  identical-geometry control for this supplement. ... do not claim the old helper passed." The same return
  calls the harness constant "a runtime consumer, separate from the intentionally historical offline helper",
  and the backcheck return repeats "Historical verify-winner-cue-plan helper remains unchanged/uninvoked/unclaimed."
- The verifier's own logic is a one-transition control: it requires `changed === (type is component or
  support)` between `--old-dir` and `--new-dir`, that is, it checks the one generation in which the
  attachment centres were corrected. It would reject two identical current generations whatever hash it pinned.
- Neither input exists in the repository (`git ls-files` has no `*candidate-runtime-point-oracle-v3.json`),
  so it cannot be run here, before or after.

Proposal (for the lane and ROOT, not done here): either add a header comment to the verifier naming the
generation it checks (pre-V30 preimage `97b18c96...`, attachment-centre correction) and that it is not a
check of current plans, or retire it to history; and, when the second profile gets a plan verifier, have
that new verifier read the one bound value from the module the harness reads. Sharing the harness's
current value with this verifier would make it reject the only plans it was written to check.

## Defect 3: the 34-file method inventory (the literal is right; its statement was missing)

Found: `characterization-observations.mjs` line 196 (`oneSuccessHistory`: old and new method manifests
across the owner's one-success transition) and line 278 (`verifyContinuationFiles`: the continuation
policy's method manifest) require exactly 34 files; `requiredMethodFiles` in `full-cohort-controller.ts`
has 38 entries. Outcome **(a)**: both checks validate the closed D-70 continuation generation, not today's list.

- Both functions are reached only through the D-70 continuation (`launch-slot`, `close-one-success`,
  `validateClaimedContinuation`); the second hard-codes product revision `8468a33c...` a few lines on.
- `requiredMethodFiles` had 34 entries at `84bb2a1f9`, `1d1d65541`, `b69bc4bcb` and `70ce64ee2` (the
  commits that built the continuation) and 38 from `683cd5ab2`, which added `fresh-demo-policy.mjs`,
  `fresh-demo-policy.d.mts`, `fresh-demo-policy.spec.ts` and `fixtures/frozen-oracle-geometry.ts.txt`.
- All nine recorded method manifests under `{D70}` list exactly 34 files, and their path set is exactly
  today's list minus those four (checked for `MANAGER/CONTINUATION/INTERNAL120/METHOD_MANIFEST_SUCCESSOR.json`
  and `WRITER/METHOD_MANIFEST_CANDIDATE.json`).
- Today's inventory is checked in `full-cohort-controller.ts` (the run preflight) with
  `method.files.length !== requiredMethodFiles.length`: already derived from the one list.

Deriving 34 from today's list would have made every recorded 34-file manifest, claim and transition
unverifiable, so it was not done. Change: the literal is now the exported, commented constant
`D70_CONTINUATION_METHOD_FILE_COUNT = 34` with `validateD70ContinuationMethodInventory(method)`; line 196
uses the constant and line 278 calls the function. Before: two bare `34`s and the error "complete34 method
inventory required". After: the same value, the same acceptance and refusal, the same error text followed
by "(D-70 continuation generation; ...)". A manifest with no `files` array is refused by both, with that
error, as before.

Recorded artifacts, before and after: a scratch script applied the original literal rule, and then the
new function, to every `*METHOD*.json` with a `files` array under `{D70}`: **9 of 9 verify before, 9 of 9
after**. `verify-characterization-observations.mjs` passes before and after (it exercises the trace
observations, not the continuation). The spec's existing one-success test builds a 34-file manifest and is
unchanged (it is one of the 20 `D70_WRITER_*` skips in the ordinary lane).

No limit moved: 34 is still 34, 38 is still refused by the D-70 path and still required by the controller.

## Defect 4: the fixture generator's protocol history (the bytes are not in the repository)

Found: `generate-fixtures.mjs` line 663 read nine files from `protocol-history/` beside it; the directory
does not exist. Because the read came after the eleven `emit` writes and before the manifest write, a run
rewrote every fixture and sample and then threw `ENOENT`.

Where the nine files live: **not in the repository.** `git log --all -- '*protocol-history*'` is empty;
no path of any ref contains `protocol-history`, `superseded-before` or `label-proxy`; and a SHA-256 of all
126,709 blobs in the object store (`git cat-file --batch-all-objects`) matches none of the nine pinned
hashes. `{PUI}/CLOSEOUT_PREPARATION_V3/MANIFEST.md` says why: "Keep protocol-history snapshots as evidence
rather than current method source." On the host this repair was made on, a separate, older local checkout
of the verification lane still holds the directory as untracked files, and all nine match their pinned
hashes (with a tenth file, a README that explains each supersession); that checkout is outside this
repository and nothing here depends on it.

Change, with the nine pinned hashes unchanged:

- The history is verified **before the first write**. Before: a missing history left eleven files
  rewritten and no manifest. After: it refuses with nothing written.
- The directory is `--protocol-history-dir <absolute directory>`, or the old default if it exists.
- `--check` regenerates everything in memory, compares each of the 12 inventoried files and the manifest
  with the frozen bytes by SHA-256 and byte count, prints a report and exits non-zero on any difference.
  It writes nothing. Without a history it reports `NOT_SUPPLIED_NINE_FILES_NOT_IN_REPOSITORY` rather than
  claiming it; with one it holds it to the nine hashes.
- Unknown arguments are refused.

`verify-fixtures.mjs` is not that check (it compares the frozen bytes with the manifest and never
regenerates), so it is unchanged and no second copy of it was built.

Result: `node generate-fixtures.mjs --check` gives `PASS_CHECK_REPRODUCES_FROZEN_BYTES_NOTHING_WRITTEN`,
13 compared, 0 differing, manifest `6e7fba8f...8739`; with the external directory supplied it also gives
`VERIFIED_NINE_PINNED_HASHES`. A negative control in a scratch copy (three bytes appended to one frozen
fixture) gives `FAIL_CHECK_...` and exit 1. The generator was never run in its writing mode in the tree.

Proposal (not done here): ROOT decides whether the nine files and their README are preserved in the
repository as evidence (for instance under the production-UI run's records), so the writing mode can be
satisfied from the repository.

## The controls

Three tests at the end of `full-cohort-controller.spec.ts`, titled "first profile repair: ...", run by the
ordinary source lane (`npm run test:e2e`, the root `playwright.config.ts`, both projects). Before the
repairs of defects 1 and 3 the first two failed (the constant's comment did not name the revision;
`D70_CONTINUATION_METHOD_FILE_COUNT` was undefined); after, they pass. The third passed on first run
because the generator was repaired before that test was ever executed in the tree, on purpose: the
unrepaired generator ignores `--check` and writes. Its failing evidence was taken from a scratch copy
outside the repository: exit 1, `ENOENT ... protocol-history/fixture-manifest-v1-superseded-before-timed-run.json`,
after the scratch fixtures had been written.

## The fifth-defect trace (read-only; nothing fixed)

71 distinct 64-hex literals in this folder outside `fixtures/` and `samples/`. 21 are the hash of a file
in the repository today. Of the other 50: 9 are the protocol history (defect 4); 1 is `97b18c96...`
(defect 2); 23 are in the spec's frozen tree-query table (`d61b6dd8...` names ROOT's manifest; 22 are
hashes of computed id lists the test recomputes on every run, one of them the hash of `[]`); and the
other 17 bind artifacts that were never committed: the point-hit, box-selection, observability and UI-test
policy documents (`bc0cb199`, `8195cd97`, `ef009d11`, `12ea9947`), the visual tokens V4 (`009b27db`), the
Chromium and Perfetto sources (`dc3a3b53`, `6a5fa3cf`, `f8065edb`, `2491cb64`, `ff28d260`), the causal
method's diagnosis records (`f87fd653`, `15102056`, `dce9bc5a`, `bc21adf3`), the original extractor
(`6fbef3d3`), a rejected continuation snapshot (`9d7b0d55`) and the focused box ordering (`1b1eced5`).
Every literal stated in more than one place is stated consistently. **No fifth stale or contradictory
literal was found.** One observation for the second profile: most of the instrument's policy preimages
are outside the repository, so their hashes can be compared but not re-derived here.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
