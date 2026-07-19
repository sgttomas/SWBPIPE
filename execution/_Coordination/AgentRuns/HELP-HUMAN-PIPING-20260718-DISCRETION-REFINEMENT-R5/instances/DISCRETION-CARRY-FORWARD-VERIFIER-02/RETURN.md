# Verdict — BLOCK

The pre-repair activation guard tested by this verifier is not fail-closed for
ambiguous repeat-S5 fields. Its four `grep -Fqx` predicates establish only that
each expected line occurs at least once; they do not establish uniqueness or
exclude contradictory lines. In an otherwise valid post-landing simulation:

- the exact required four-line return passed;
- deleting or singly replacing the verdict, plan-blob binding, D-54 semantic
  hash, or DEC-087 semantic hash failed as intended;
- adding `Verdict: BLOCK` beside the required `Verdict: COMMIT-SAFE` still
  passed;
- adding `Basis-Plan-Blob: deadbeef` beside the expected plan binding still
  passed; and
- duplicating the exact `Verdict: COMMIT-SAFE` line still passed.

This contradicts both the plan's statement that any ambiguous condition falls
back and `S5_REVIEW_PACKET.md`'s rule that any ambiguous machine-readable line
keeps the guard in fallback. It also fails this brief's requirements for the
exact repeat-S5 verdict and failure on every absent or mismatched condition. An
ambiguous or contradictory return can therefore produce
`DEC087_GUARD_ACTIVE` once the structural Git conjuncts are true.

Other required checks completed without a separate blocker:

- Executing the guard's exact fenced shell block in the current pre-landing
  tree returned
  `DEC087_GUARD_FALLBACK: use WORKPLAN_2026-07-17_piping_loop.md; DEC-087 prohibited`.
  The candidate plan, D-54, and repeat-S5 return were absent from `HEAD`; the
  repeat-S5 return was also absent from the worktree. The tracked decomposition
  worktree blob differed from its `HEAD` blob, so pre-landing fallback had
  multiple independent causes.
- The post-landing structural design checks candidate plan, D-54, decomposition,
  and repeat-S5 return both through tracked/`HEAD:<path>` presence and
  worktree-to-`HEAD` blob equality. It binds the candidate-plan blob and stable
  D-54/DEC-087 semantic hashes. Missing or singly mismatched structural/hash
  conjuncts return failure, and DEC-087 row extraction rejects zero or multiple
  rows. The binding uses blob/semantic hashes, not a future commit SHA, so no
  circular future-SHA dependency was found.
- The old/new plan diff had exactly two diff hunks comprising the three governed
  categories: first-instruction activation guard plus re-mint/successor metadata,
  and the DEC-087 Step-2 refinement. All unaffected spans compared byte-identical:
  Owner intent through Step 1 was 4,128 bytes / SHA-256
  `72b822061b11ad1743c4685c1c99fce0c37ad374940f686f9f2b740c7e12619c`;
  the common Step-2 prefix was 1,066 bytes /
  `fa169b6616710b9eb757c51ccdb2b89d2803147377ab5fcc07cf3d9bf8965720`;
  Step 3 was 830 bytes /
  `b351d3aacdecb67a595f9450f100b29bac2d39ff4ab954df0333613fc84ad1d9`;
  and Step 4 through EOF was 5,448 bytes /
  `27a10f316520356929693f5af43923f9b8a131c3eac6889bfd5b934e274ddeac`.
  Standing fences through EOF were byte-identical at 3,515 bytes /
  `aad0baef3bd24d73aa81a471700564482864582c36b230c07441e10b385da86f`.
- The refined Step 2 retains the owner-act adoption boundary and expressly
  carries irreducible preference, purpose/scope, normative/acceptance,
  professional/safety/legal/fiduciary/residual-risk, spending/external,
  publication/release, protected-data, destructive/irreversible, and all other
  applicable D-54/D-52/D-49 fast rejects. Step 3 and the standing fences are
  byte-identical as recorded above.
- `WORKPLAN_2026-07-17_piping_loop.md` remained byte-equal to `HEAD` (SHA-256
  `78be027f3ea39296b8b9fe7509e6d183b231c74847eb451cec2693b592c554da`).
  `LOOP_INIT.md` also remained byte-equal to `HEAD` and still selects the newest
  `WORKPLAN_*.md`; filesystem ordering selects the 2026-07-18 candidate. The
  new first-instruction fallback neutralizes that pre-landing staging hazard,
  but its post-landing S5 ambiguity defect remains blocking.
- Piping D-50 and app-dev D-APP-60 Shared-Block v1 copies remained identical at
  5,108 bytes / SHA-256
  `76438ab0e00dc70e5f6db751a32d0ff07b681c7b7fb12eeda338157c5ebe7668`.
  Both source artifacts and the entire app-dev subtree compared clean to
  `HEAD`.

This verifier did not create `S5_REVIEW_RETURN_02.md`, apply DEC-087, modify
DEL-09-04, or repair the guard. Its only write is this return.
