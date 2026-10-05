# HELP_HUMAN's rulings: piping LOOP_INIT in the binding form (2026-10-05)

Run `PIPING-LOOP-INIT-20261005`. The owner's direction is in [OWNER_DECISIONS.md](OWNER_DECISIONS.md).

## LM's return

LM, a Type 2 TASK, was dispatched under [BRIEF_LM.md](BRIEF_LM.md) as a Claude Code subagent (general-purpose, Claude Opus 5.5). It made no Git writes and ran no tests. It returned three files:
- `LOOP_INIT_MAPPING.md`, with sha256 `12c34aa8…`;
- `LOOP_INIT_PROPOSED.md`, with sha256 `072f6d36…`;
- `CONSISTENCY_EDITS.md`, with sha256 `3021790…`.

**The mapping** has 92 rows:
- 77 in class (a), carried elsewhere with verbatim quotes;
- 11 in class (b), piping-specific;
- 0 in class (c);
- 4 in class (d), obsolete.

It checked 194 destination quotes with `grep -F`, every draft path with `test -e`, and the eight workflow names against `workflows/index.json`. HELP_HUMAN checked these:
- the draft against the owner's three-part test;
- the stage-gate source: `_COORDINATION.md` "Current Target Stage (ruled record)", and PRD §24 "Release Milestones";
- each "before" text in A1–A7 occurring exactly once.

## Rulings on LM's open questions

1. **The MEMORY creation grant is kept.** It is piping-specific. Construct §3 leaves MEMORY writes to the project's write fences, and App v4 carries the same grant.
2. **`coordinated-knowledge-work` is kept.** Piping's T3 runs under it by owner delegation, and it is the general coordination method App v4 binds.
3. **"If you are unsure whether something matters, ask the human." is replaced by "If you are unsure whether a section matters, read it."** The replacement keeps the reading decision with the agent, as the owner asked ("the agent decides when to visit it"). It also avoids contradicting Root `AGENTS.md`: "uncertainty alone does not require an extra prompt".
4. **A6 and A7 are applied.**
   - **A6:** DEL-11-05 is IN_PROGRESS, and `RECON_2026-09-21_WHOLE_CORPUS` last changed at its closeout (2026-09-22). An earlier instruction tranche added the banner. The independent reviewer is asked to confirm that no active concordance scope freezes it.
   - **A7:** the WORKPLAN says its navigation "follows the subsequently adopted shared loop instructions". The owner-adopted retirement text is unchanged.
5. **Section B's edits are not made.** They were stale before this change, so they are recorded and left for their own correction. Section C, the manual citations and construct's introduction sentence, is outside this tranche. The manifest's notice rationale records it.
6. **"When to read further" in two loops is accepted.** Each LOOP_INIT binds its own project, and moving the paragraph into a manual is an owner-authored manual revision.

## Adoption

- **`projects/chirality-piping/loop/LOOP_INIT.md`** is replaced by the proposed text, with ruling 3 applied (sha256 `5c20a16fef96…`).
- **A1–A7 are applied** as written in `CONSISTENCY_EDITS.md`.
- **The tranche manifest** is `docs/governance_harness/tranche_manifests/PIPING-LOOP-INIT-20261005.yaml` (G4 PASS in CI mode).
- **Checks at adoption:**
  - `tools/validation/validate_instruction_entrypoints.py` PASS;
  - `projects/chirality-piping/tests/test_ci_e2e_plan.py` 41 passed (it reads piping's LOOP_INIT).
- **The gates for the PR,** under the owner's "appropriate level of CI":
  - an independent review, LR (`reviews/`), with any repairs confirmed by the same reviewer;
  - GEN-8 on the exact head;
  - the PR's automatic CI, which includes G4.
  
  No product code changes, so DEC-025 does not apply (project `AGENTS.md`, "Software checks").

## Addendum A: LR's review and the repairs (2026-10-05)

**LR's review** is `reviews/LR-LOOPINIT.md` (sha256 `4de888bc…`). Verdict: **REPAIR**, with 0 BLOCKING, 1 MAJOR, 3 MINOR and 9 NOTE.

**The mapping and edits LR confirmed:**
- 123 class (a) quotes, verbatim;
- every class (b) and (d) row;
- the paths and workflow names;
- A1–A7 as minimal and true. DEL-11-05 is not frozen: RECON_2026-09-21 closed on 2026-09-22.
- the validators.

**The repairs:**
- **MAJOR-1, adopted as LR proposed. Erratum to mapping row N13 and to this file's "checked the stage-gate source".** The standing constraint now points to the target stage and exit criteria recorded under `_COORDINATION.md` "Current Target Stage". It no longer points to `docs/PRD.md` §24 by milestone label, because the recorded target "PRD R5" uses the old v0.1 numbering, and its criteria are §24 R6's reproduction criterion plus §20.1.
- **MINOR-1, adopted as LR proposed.** "Its revision notes record each amendment that changed it; `execution/_ScopeChange/` holds every amendment, and its `_LATEST.md` selects the latest accepted one."
- **MINOR-2, repaired.**
  - A notice to Piping's loops, `execution/_Coordination/NOTICE_2026-10-05_PIPING_LOOP_INIT_BINDING.md`, is written and routed.
  - The rationale now records the two consequences handled elsewhere: construct's introduction (tranche ROOT-CONSTRUCT-LOOPINIT-WORDING-20261005, PR #1093, approved by the owner) and the manual citations (tranche ROOT-LOOPINIT-AUM-ALIGNMENT-20261005).
  - **Erratum to ruling 5:** its claim that the rationale already recorded construct's sentence was not true at the first head.
- **MINOR-3, repaired.** T3's rulings gain "T3's gate set and Git rules, consolidated after the handoff was made ephemeral". The work graph's T3 section lists it, and the steer names the product-PR gates.

**The notes:**
- **NOTE-1 and NOTE-2.** NUM absorbed main (`01809013ae`), and the PR is re-cut from that main, so its tree equals NUM's again. Main's move was App v4 only.
- **NOTE-3.** App v4's sentence is changed in tranche ROOT-LOOPINIT-AUM-ALIGNMENT-20261005, by the owner's direction.
- **NOTE-4, NOTE-5 and NOTE-9** are accepted as recorded.
- **NOTE-6.** Section B, and the contributor guide's row 8, are routed to tranche ROOT-LOOPINIT-AUM-ALIGNMENT-20261005, under the owner's "any other minimal consistency edits".
- **NOTE-7.** The handoff now says "little swap (about 1 GiB, dynamic)", and names the two standing worktrees.
- **NOTE-8.** The graph's owner decisions now include "all T3 scratch lives in `WT/scratch`". The steer's claim of "basis in the T3 rulings" holds, now that the consolidated ruling exists.
- **LR's `test_ci_e2e_plan.py` run** left about 79 MB of ignored `target/` folders in the PR worktree. They are removed with that worktree after the merge.

## Addendum B: LR's Addendum A at H3 (2026-10-05)

**LR's Addendum A** is appended to `reviews/LR-LOOPINIT.md` (full file sha256 `787b8443…`; the original review is a byte prefix). At H3 `32b7802403`: 0 BLOCKING, 0 MAJOR, 1 MINOR, 4 NOTE.
- **The repairs confirmed:** MAJOR-1, MINOR-2 and MINOR-3, and NOTE-1/2/7/8.
- **The consolidated T3 ruling** makes no new rule.
- **RR** is append-only.
- **G4, the entry validator and GEN-8** pass on H3.

`reviews/SHA256SUMS` keeps the original line for `LR-LOOPINIT.md` and appends the new one. Verify the newest line; the original line records the first review's bytes, which are the file's prefix.

**The findings:**
- **MINOR-A1, adopted as LR proposed.** The clause "`execution/_ScopeChange/` holds every amendment" was false: SCA-006 has no folder there, and is recorded in the decomposition's v0.9 note and D-43. The sentence now reads: "Its revision notes record each amendment that changed it, and `execution/_ScopeChange/_LATEST.md` selects the latest accepted one." **Erratum to Addendum A's MINOR-1 wording.**
- **NOTE-A1.** Addendum A's "re-cut from that main" means that main was merged into the PR branch, twice: after #1090/#1091, then after #1093.
- **NOTE-A2.** Section B (B1–B3) and the contributor guide's row 8 are carried into tranche ROOT-LOOPINIT-AUM-ALIGNMENT-20261005, as routed.
- **NOTE-A3 and NOTE-A4** are accepted. The merge waits for `harness` on the final head, and `WT/loop-init-pr` is removed after the merge.
