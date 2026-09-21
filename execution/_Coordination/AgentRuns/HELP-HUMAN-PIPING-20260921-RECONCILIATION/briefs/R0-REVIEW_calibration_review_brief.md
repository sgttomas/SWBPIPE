# Brief — R0 calibration review (fresh, evidence-only)

Parent: HELP_HUMAN Agent 0, run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`
(D-73 / DEC-110). Role: TASK (Type 2), fresh context, **evidence-only**. You
never edit a ledger and never repair anything. Do not delegate.

The launch message supplies `{FREEZE}` (read-only checkout of the frozen state
`00115c71931bcae79909602d653740d3bb72dfa1`) and `{REPO}` (repository checkout
holding the run folder). `RUN` below means
`{REPO}/projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`.

## What you are reviewing

Eight calibration ledgers under `RUN/R0_CALIBRATION/<DEL>/`, for DEL-04-04,
DEL-07-02, DEL-00-05, DEL-07-09, DEL-01-01, DEL-12-03, DEL-11-01 and
DEL-17-05. Each deliverable has a sealed forward ledger, reverse answers
against `RUN/R0_CALIBRATION/PILOT_CAPABILITIES.csv`, and notes. They were
produced under the candidate rules in
`RUN/R0_CALIBRATION/CANDIDATE_CONVENTIONS.md`. The purpose of calibration is
to decide whether those rules are fit to scale to 102 deliverables, and what
must change first.

## Boundary

- Write only `RUN/R0_CALIBRATION/R0_REVIEW.md`, plus scratch files named
  `_review_scratch_*` in `RUN/R0_CALIBRATION/`. Delete the scratch files
  before you return.
- Read code and documents from `{FREEZE}` only.
- No builds, installs, test runs, git writes, or network access other than
  read-only `gh`.
- Do not read the July 2026 run's ledgers. In
  `.../DELIVERABLE_CONCORDANCE_2026-07-11_1305/`, that means
  `CLAIM_CONCORDANCE.csv`, `WAVES/**`, `PACKAGE_SUMMARIES/**`, `PROPOSED_*`,
  `CONFLICTS_AND_UNKNOWNS.csv`, `IMPLEMENTATION_SURFACES.csv` and
  `UNMAPPED_IMPLEMENTATION.csv`.
- The fences are the same as the workers': DEC-043, no protected data, no
  release/approval/compliance claims. Standard claim fence applies (F-PIP-2;
  claims taxonomy per DEC-081).

## Checks

1. **Structure.** Run `RUN/tools/validate_ledger.py` on each forward and
   reverse file, and report the results.
2. **Evidence spot-check.**
   - For each deliverable, pick at least eight rows, including every
     non-aligned row up to twelve and at least three `ALIGNED` rows.
   - Re-derive each picked row from `{FREEZE}`: does the cited implementation
     do what the row says? Do the cited tests exercise it? Is the disposition
     right?
   - Report every row you disagree with: key, what it says, what you found,
     and the right disposition.
3. **False alignment.**
   - Look for `ALIGNED` rows whose claim the code does not actually satisfy.
   - Look for `COVERED_BY_CHILDREN` or `NOT_ASSESSED` rows that hide a real
     claim.
   - Report the rate per deliverable.
4. **Cross-deliverable consistency.** Are the same situations given the same
   disposition, cause tag and tier across the eight? Examples: setup-era
   future tense, the D-41-era "current declaration" blocks, rename residue,
   stale Remaining items.
5. **Authority handling.**
   - Were governing and context sources kept apart (A3)?
   - Did any row let a merged PR or AgentRuns record change a disposition?
   - Is `ACCEPTED_DIVERGENCE` used only with a permitting ruling?
   - Were `INVARIANT` and `PROJECT_BASELINE` tiers assigned where the claim
     restates a contract, boundary or ruling?
   - Was the ISSUED deliverable handled correctly?
6. **Reverse pass.**
   - Compare the eight workers' answers per capability.
   - List capabilities claimed by two or more deliverables.
   - List capabilities claimed by none. Do they look genuinely unowned by
     these eight, or were they missed?
   - List answers that look anchored or careless.
7. **Unit grain.**
   - Is the claim-key grain right? Weigh the `_CONTEXT.md` blocks, the SoW
     section wrappers, and D-41 declaration blocks.
   - Estimate what fraction of rows carried no audit value.
   - Propose grain changes that would keep coverage provable.
8. **Convention friction.** Consolidate the workers' friction notes with
   your own findings. For each candidate convention (A1–A7, B1–B3, C1–C10,
   Part D), say: keep, amend (with exact proposed text), or drop.
9. **Cause-tag fitness.** Did the sixteen tags cover what was found? Were any
   overused or ambiguous? Propose merges or additions.

## Output — `R0_REVIEW.md`

1. **Verdict line:** `SCALE-READY`, `READY WITH NAMED AMENDMENTS`, or
   `NOT READY`.
2. **Per-deliverable table:**
   - structural result;
   - rows;
   - non-aligned count;
   - spot-checked rows;
   - disagreements;
   - false-alignment rate.
3. **Findings**, numbered, each with severity (`BLOCKING`, `MAJOR`, `MINOR`)
   and evidence.
4. **Convention dispositions (keep / amend / drop):**
   - one row per candidate convention;
   - exact proposed text for each amendment;
   - named repairs to specific calibration ledgers. These are to be carried
     into the waves, never patched in place.
5. **Grain recommendation.**
6. **Reverse-pass recommendation** for the R1 whole inventory.
7. **Scale-out recommendation** for R2: batch size, verifier sampling rate,
   and anything the manager briefs must add.

Keep the owner's time in mind. Put the decisions the owner must make first,
stated plainly. Detail follows.

## Return

```
DONE R0_REVIEW verdict=<verdict> sha256=<sha256>
```

Follow it with at most ten lines: the verdict reason, the blocking findings,
and the top amendments.
