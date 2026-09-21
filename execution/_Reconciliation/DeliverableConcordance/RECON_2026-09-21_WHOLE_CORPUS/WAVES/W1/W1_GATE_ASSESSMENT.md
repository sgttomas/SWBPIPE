# Wave 1 gate assessment (Agent 0)

Inputs:
- `PKG-07/PKG-07_VERIFICATION.md` (sha256 `a2e8d065…`);
- `PKG-16/PKG-16_VERIFICATION.md` (sha256 `ab14a7e6…`);
- Agent 0's own rechecks (`RUN_STATE.jsonl`, `AGENT0_RECHECK`).

The gate is defined in `WAVE_PLAN.md`.

## Result against the gate as written: NOT MET

| Condition | PKG-07 | PKG-16 | Met? |
|---|---|---|---|
| 1. Verifier verdict ACCEPT or ACCEPT WITH CONTESTED ROWS | ACCEPT WITH CONTESTED ROWS | ACCEPT WITH CONTESTED ROWS | yes |
| 2. Package firm false alignment ≤ 5% | 4.1% (4 of 97) | 6.3% (1 of 16) | **no** (PKG-16) |
| 3. No unresolved shared-situation conflict | 1 conflict (DEL-07-06 R04); the verifier resolved it toward UNKNOWN, but the sealed row still reads ALIGNED | none | **no** (until recorded) |
| 4. Every deliverable validates | 9 of 9 | 4 of 4 | yes |

Across the wave, 5 of 113 = 4.4% (pooled). No deliverable reached the rerun
threshold. The 13 deliverables have 1,318 forward rows, and 542 were
sampled.

## One cause behind all five firm errors

Each of the five false alignments has the same shape:
- DEL-07-01 CLM-005.r05;
- DEL-07-02 CLM-025.r04, CLM-018.r06 and CLM-026;
- DEL-16-04 REQ-16-04-09.

In each, the row's own Notes or evidence record an unmet element of its claim
("not located", "partially evidenced", "gap carried on RQ-008"), and yet the
row is marked `ALIGNED`. No convention forbids this outright. C6(a) covers
only claims that are satisfied because the governed behaviour does not
exist.

A mechanical wording check over the Notes and evidence columns flags 95 of
the 715 wave 1 `ALIGNED` rows and catches 4 of the 5 firm errors.

## Proposed remedy (for owner ruling)

**R-1. New rule C6(i).** A row whose own evidence or Notes record an unmet
element of its claim is not `ALIGNED`, even when the gap is also recorded
on another row. It takes the disposition for the unmet element, usually
`PARTIALLY_IMPLEMENTED` or `UNKNOWN · EVIDENCE_NOT_LOCATED`. Rule C1
(common defects) may share the cause and the `FindingGroup`, but never
turns the row into `ALIGNED`.

**R-2. Remaining items that record an open action (the gap in A4).** This
applies when the text is accurate and the action is still open.
- Every Remaining unit is `ClaimType DECLARED_STATE`.
- When a governing claim row in the same ledger carries the open work, the
  Remaining row is `ALIGNED`, with `OPEN_ACTION: <that key>` in Notes. The
  declaration is accurate, and the gap lives on the governing row (A4:
  Remaining is not authority).
- When no governing row carries it, the Remaining row takes the gap
  disposition itself (`DOCUMENTED_UNIMPLEMENTED` or
  `PARTIALLY_IMPLEMENTED`), so that the open work is not lost.
- R-1 does not apply to the first case, because the claim there is "this is
  open", which is true.

**R-3. Clarify C6(c): the origin test for the two stale classes.** Text
first present at the initial migration (it predates the Scope of Work
migration, found with `git log -S` on the frozen history) is
`STALE_SETUP_SPECIFICATION`. This holds even when the Scope of Work
migration re-declared it. `STALE_REVIEW_OR_EVIDENCE` is only for text first
declared later.

**R-4. Verification and tooling.**
- The validator gains a `--notes-gap` report that lists `ALIGNED` rows whose
  Notes or evidence contain gap wording.
- Workers must clear each listed row before sealing, either by
  re-disposing it or by saying in Notes why the wording is not about this
  claim.
- Verifiers sample these rows at 100%.
- Batch mode accepts a resolutions file: verifier-resolved pairs recorded by
  Agent 0 in `WAVES/<W>/RESOLUTIONS.csv`.

**R-5. Specific NOT_MINE reasons where the paths overlap.** When a
capability's EntryPoints hit a path that the deliverable's own ledger cites,
the NOT_MINE reason must address that capability. A template reason is not
enough in that case.

**R-6. Wave 1 dispositions.**
- No rerun. No deliverable crossed the rerun threshold.
- Every firm, weak and field disagreement stays visible as a contested row,
  with the verifier's values, in `WAVES/W1/RESOLUTIONS.csv`. R3 synthesis
  reads the sealed ledger together with that file. Sealed files are never
  patched.

**Not in this remedy (goes to R4 as an authority item).** DEC-094 and the
SCA-009 annex disagree over where DEL-07-03's R-005 load-case editor
ownership lands. As a result, no PKG-07 deliverable claims the implemented
Load Case Manager. The four DEL-07-03 accepted-divergence rows stay
contested.

## Recommendation

1. Adopt R-1 to R-6.
2. Update the worker and verifier briefs and the validator.
3. Get independent review of those changes.
4. Scale out to the rolling queue at 16 live agents.

Wave 1 has one cause behind its firm errors, a pooled rate of 4.4%, and no
reruns. The rules above target that cause directly. A second gate wave
would cost time without testing anything that the first scale-out packages
will not test anyway. Each package still gets its own verifier.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
