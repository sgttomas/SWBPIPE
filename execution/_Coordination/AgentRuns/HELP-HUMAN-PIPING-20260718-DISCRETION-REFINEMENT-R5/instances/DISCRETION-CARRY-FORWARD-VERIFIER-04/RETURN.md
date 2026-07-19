BLOCK

- **Material staged/index-dirty activation remains.** I executed the exact
  current guard in isolated post-landing Git histories. The valid condition
  (candidate plan, D-54, `SOFTWARE_DECOMP.md`, and the exact four-line S5
  return committed at one HEAD with matching worktree bytes and content
  bindings) returned `DEC087_GUARD_ACTIVE`. For each of those four required
  artifacts independently, I then appended a sentinel, staged it with
  `git add`, and restored the worktree file to its HEAD bytes. Each artifact
  was dirty (`git status --short` reported `MM`), its index blob differed from
  HEAD, and its worktree blob still equaled HEAD. All four cases nevertheless
  returned `DEC087_GUARD_ACTIVE`. The guard proves only index membership with
  `git ls-files`; it never proves the indexed blob equals HEAD. This violates
  this brief's dirty-artifact failure case and the plan's claim that any dirty
  condition falls back, so the corrected candidate is not fail-closed.

- **The named field corrections otherwise pass.** In the same valid
  post-landing simulation, added or substituted leading-space, leading-tab,
  Markdown bullet, blockquote, bold/code wrapper, spaced-colon, case-variant,
  embedded-prose, exact contradictory, exact duplicate, and wrong-binding
  lines all fell back. Missing or wrong values for each of the four required
  fields also fell back. The helper now detects a canonicalized key anywhere
  in the line while accepting only one exact raw expected line.

- **Other structural cases otherwise pass.** Worktree-byte changes and absent
  worktree files for each of the four required artifacts fell back, as did
  removal from the index, absent/unborn HEAD, execution outside a Git
  worktree, and a duplicate DEC-087 row. The valid content binding is
  non-circular: it uses candidate blob `280509c835da09ae9803e0e683233bc324e2751d`,
  D-54 sections 2--3 SHA-256
  `2b76c43e453d67821c3e35c3cfc5ba1b2d41e123dfbc9192a0089f37e9763b4b`,
  and current DEC-087 field SHA-256
  `3c9e909686527e8e679c7462ddf79083fd7add546c4c2b9101c0019064fb30c0`,
  not a future commit SHA.

- **Current fallback and plan carry-forward pass.** Executing the exact fenced
  guard in the live pre-landing tree returned exactly
  `DEC087_GUARD_FALLBACK: use WORKPLAN_2026-07-17_piping_loop.md; DEC-087 prohibited`.
  The new plan differs only in the first activation guard,
  successor/re-mint metadata, and DEC-087 Step 2. Unaffected spans remain
  byte-identical: Owner intent through Step 1 is 4,128 bytes / SHA-256
  `72b822061b11ad1743c4685c1c99fce0c37ad374940f686f9f2b740c7e12619c`;
  the common Step-2 prefix is 1,066 bytes /
  `fa169b6616710b9eb757c51ccdb2b89d2803147377ab5fcc07cf3d9bf8965720`;
  Step 3 is 830 bytes /
  `b351d3aacdecb67a595f9450f100b29bac2d39ff4ab954df0333613fc84ad1d9`;
  Step 4 through EOF is 5,448 bytes /
  `27a10f316520356929693f5af43923f9b8a131c3eac6889bfd5b934e274ddeac`;
  and fences through EOF are 3,515 bytes /
  `aad0baef3bd24d73aa81a471700564482864582c36b230c07441e10b385da86f`.
  The old plan remains byte-equal to HEAD at 14,203 bytes / SHA-256
  `78be027f3ea39296b8b9fe7509e6d183b231c74847eb451cec2693b592c554da`.
  `LOOP_INIT.md` still selects the newest filename, and the exact current
  fallback neutralizes that ordinary pre-landing selection, but this does not
  cure the staged/index-dirty post-landing bypass.

- **Graph, history, and preservation checks pass.** `WORK_GRAPH.json` has 16
  unique node IDs and no unresolved dependency. It truthfully records the v2
  carry verifier's late preserved `BLOCK` outside the closed original fan-in
  and the v3 carry verifier's interruption without a return; D-54, the
  register, DEC-087, orchestration plan, review packet, top-level return,
  handoff, status, and interruption records agree. Shared-Block v1/app-dev,
  the prior workplan, receipts, and the DEL-09-04 subtree are unchanged from
  HEAD. Instruction-entrypoint and receipt validators passed, JSON parsed,
  the index was empty, and `git diff --check` passed before this return.

No repair, S5 return, DEC-087 effect, receipt, DEL-09-04 action, commit,
merge, or push was performed. This verifier's only write is this return.
