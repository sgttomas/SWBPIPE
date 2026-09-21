# Brief — R0 calibration worker (one deliverable, two sealed passes)

Parent: HELP_HUMAN Agent 0, run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`
(activation D-73 / DEC-110). Role: TASK (Type 2). You own exactly one
deliverable, named in your launch message. Do not delegate. Model and effort
are set by the parent.

The launch message supplies three values:
- `{DEL}`: your deliverable ID;
- `{FREEZE}`: absolute path of a read-only checkout of the frozen state
  `00115c71931bcae79909602d653740d3bb72dfa1`;
- `{REPO}`: absolute path of the repository checkout where the run folder
  lives.

Paths below are relative to `{REPO}` unless stated.

## Purpose

This is calibration. Your ledger tests whether the candidate conventions work
on a real deliverable. Your judgments matter, and so does your account of
where the conventions did not fit. You audit what the deliverable says
against what the frozen code, tests and records show. Code is evidence, not
authority.

## Boundary

- **Write only** these files under
  `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/R0_CALIBRATION/{DEL}/`:
  - `{DEL}_forward.csv`
  - `{DEL}_reverse.csv`
  - `{DEL}_notes.md`
  - scratch files you name `_scratch_*` (delete them before your final return)
- Read code, tests and documents **only from `{FREEZE}`**. Never write there.
- No builds, installs, test runs, git writes, or network access other than
  read-only `gh pr view` / `gh pr list` / `gh api` for merged pull requests.
- Do not read:
  - earlier reconciliation ledgers or summaries: in
    `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/DELIVERABLE_CONCORDANCE_2026-07-11_1305/`,
    the files `CLAIM_CONCORDANCE.csv`, `IMPLEMENTATION_SURFACES.csv`,
    `UNMAPPED_IMPLEMENTATION.csv`, `CONFLICTS_AND_UNKNOWNS.csv`,
    `PROPOSED_*`, `WAVES/**` and `PACKAGE_SUMMARIES/**` (other files there, and
    records a deliverable cites, are allowed);
  - other workers' folders under `R0_CALIBRATION/`;
  - `R0_CALIBRATION/PILOT_CAPABILITIES.csv`, until the parent sends it to you
    after you seal.
- Fences: never present unreviewed equation artifacts from the external
  piping-design corpus as evidence (DEC-043); never quote protected
  standards, vendor or private data; never state or imply a release,
  approval, compliance or certification claim; your dispositions are never
  owner rulings. Standard claim fence applies (F-PIP-2; claims taxonomy per
  DEC-081).

## Read first

1. `projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/R0_CALIBRATION/CANDIDATE_CONVENTIONS.md`.
   This file is your rules. Apply it exactly.
2. `projects/chirality-piping/docs/RECONCILIATION_PROFILE.md`.
3. `.../RECON_2026-09-21_WHOLE_CORPUS/RUN_BASIS.md` (authorities, gate
   evidence).
4. The rows of `.../RECON_2026-09-21_WHOLE_CORPUS/CLAIM_KEYS.csv` whose
   `DeliverableID` is `{DEL}`. These are your coverage denominator; each gives
   the source path and line range at the frozen state.
5. Your deliverable's folder in `{FREEZE}` (`projects/chirality-piping/execution/PKG-*/1_Working/{DEL}_*/`).

Then read whatever evidence you need:
- code, tests, schemas and fixtures in `{FREEZE}`;
- governing authorities and rulings;
- `GATE_EVIDENCE/` for suite-level pass status;
- merged PRs and AgentRuns records, as context.

## Pass 1 — forward ledger (sealed)

1. Write `{DEL}_forward.csv` to the Part D schema. It has exactly one row per
   issued key, plus any `.sNN` sub-claims under containers, and ends with the
   `#END` sentinel.
2. Run the structural validator until it passes:
   `PYTHONDONTWRITEBYTECODE=1 python3 projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS/tools/validate_ledger.py --run-dir projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS --repo-root . --deliverable {DEL} --forward <path to your forward.csv>`
   (run from `{REPO}`).
3. Compute its SHA-256 (`shasum -a 256`).
4. **Return** one message and stop:

   ```
   SEALED {DEL} forward <sha256> rows=<n> validator=PASS
   ```

   Add at most five lines on anything the parent must know now. From this
   point the forward file is sealed. Never edit it again.

## Pass 2 — reverse answers (after the parent's message)

The parent will send the path of `PILOT_CAPABILITIES.csv`. For **every** row in
it, answer in `{DEL}_reverse.csv` (Part D reverse schema, `#END` sentinel):
`CLAIMED_BY <key>`, `PARTIAL <key>` or `NOT_MINE`, each with a one-line reason.
Most rows are expected to be `NOT_MINE`; answer honestly. Validate with the
same command, adding
`--reverse <path to reverse.csv> --inventory <path to PILOT_CAPABILITIES.csv>`.

Then write `{DEL}_notes.md` covering:
- path aliases you used;
- judgment calls;
- **convention friction**: each place a candidate convention was unclear,
  missing, or produced a misleading row, with the smallest fix you would
  propose;
- unit-grain observations: was the claim-key grain right for this
  deliverable?;
- the smallest next check for every `UNKNOWN` row;
- whether the reverse pass changed your view of anything you sealed. Do not
  edit the sealed file; say it here.

Delete your `_scratch_*` files. Then return:

```
DONE {DEL} forward=<sha256> reverse=<sha256> notes=<sha256> validator=PASS
```

Follow it with at most ten lines summarizing:
- disposition counts;
- the top three cause tags;
- the most important convention friction.
