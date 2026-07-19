BLOCK

- **Material wrong-object/mode activation bypass.** In an isolated committed
  post-landing simulation, I replaced D-54 with a mode-`120000` symlink named
  `TARGET` and supplied its target as an untracked regular file whose exact
  bytes were `TARGET`. Git then reported the same object ID
  (`c270d5403553c7afa5f68471fbc42b8c8f13caf5`) for `HEAD:<D-54>`, the index
  entry, and `git hash-object <D-54>`. The guard does not require regular-file
  mode `100644`; `git show HEAD:<D-54>` produced only the symlink payload, so
  the D-54 section extractor silently produced the empty string. With an
  otherwise exact S5 return binding the SHA-256 of that empty semantic value
  (`e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`),
  the exact current guard returned `DEC087_GUARD_ACTIVE`. Thus index/HEAD/
  worktree object-ID equality does not establish that guarded D-54 is the
  required regular governed artifact, and the named wrong-object case is not
  fail-closed.

- **The v4 index-only bypass is closed.** A valid regular-file post-landing
  simulation returned `DEC087_GUARD_ACTIVE` with candidate-plan blob
  `d9d87b7339e064766edaf94223e7cd019bd71310`, D-54 semantic SHA-256
  `2b76c43e453d67821c3e35c3cfc5ba1b2d41e123dfbc9192a0089f37e9763b4b`,
  and DEC-087 semantic SHA-256
  `c28ad72450159d2658a99fb15d415e2feff4dd0ffe13c6e5b6bd4fa5a62305d4`.
  For each of the four guarded artifacts, append/stage/restore-to-HEAD produced
  `MM`, index blob unequal to HEAD, and worktree blob equal to HEAD; all four
  now returned the exact fallback. Unstaged, staged, removed-index,
  absent-worktree, unmerged-index, and wrong-index-object mutations also fell
  back for all four artifacts (24/24).

- **Field and remaining structural cases pass but do not cure the blocker.**
  All 52 missing, wrong, duplicate, contradictory, leading-space/tab,
  Markdown bullet/blockquote/bold/code, spaced-colon, case-variant, and
  embedded-prose field cases fell back. Duplicate DEC-087, absent-at-HEAD,
  unborn-HEAD, and outside-Git cases fell back. The exact live pre-landing
  guard returned
  `DEC087_GUARD_FALLBACK: use WORKPLAN_2026-07-17_piping_loop.md; DEC-087 prohibited`.
  The binding contains no future commit SHA.

- **Carry-forward and preservation checks pass.** The old/new diff is confined
  to the activation guard, successor/re-mint metadata, and DEC-087 Step 2.
  Owner intent through Step 1 is byte-identical (4,128 bytes,
  `72b822061b11ad1743c4685c1c99fce0c37ad374940f686f9f2b740c7e12619c`);
  the common Step-2 prefix is byte-identical (1,066 bytes,
  `fa169b6616710b9eb757c51ccdb2b89d2803147377ab5fcc07cf3d9bf8965720`);
  Step 3 is byte-identical (830 bytes,
  `b351d3aacdecb67a595f9450f100b29bac2d39ff4ab954df0333613fc84ad1d9`);
  Step 4 through EOF is byte-identical (5,448 bytes,
  `27a10f316520356929693f5af43923f9b8a131c3eac6889bfd5b934e274ddeac`);
  and fences through EOF are byte-identical (3,515 bytes,
  `aad0baef3bd24d73aa81a471700564482864582c36b230c07441e10b385da86f`).
  The old plan remains byte-equal to HEAD (14,203 bytes,
  `78be027f3ea39296b8b9fe7509e6d183b231c74847eb451cec2693b592c554da`).

- **Graph/history checks pass.** `WORK_GRAPH.json` has 20 unique node IDs and
  no unresolved dependency. The v1 returns, actual S5 BLOCK, v2 semantic
  BLOCK, v2 carry interruption plus late preserved BLOCK, v3 semantic BLOCK,
  v3 carry interruption without return, v4 semantic interruption without
  return, and v4 carry BLOCK are all present and truthfully reflected in the
  current graph/status surfaces. `LOOP_INIT.md` remains byte-equal to HEAD and
  selects the newest filename; current fallback directs the prior plan.
  Shared-Block v1 remains byte-identical across piping and app-dev at 5,108
  bytes / SHA-256
  `76438ab0e00dc70e5f6db751a32d0ff07b681c7b7fb12eeda338157c5ebe7668`.
  App-dev, old plans, receipts, D-49 through D-53, and DEL-09-04 have no diff
  from HEAD. Instruction-entrypoint and receipt validators passed, JSON
  parsed, the index was empty, `S5_REVIEW_RETURN_02.md` remained absent, and
  `git diff --check` passed before this return.

No repair, S5 return, DEC-087 effect, receipt, DEL-09-04 action, stage,
commit, merge, push, or external effect was performed. This verifier's only
repository write is this return.
