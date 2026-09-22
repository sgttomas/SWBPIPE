# Brief — R3/R4 independent review (fresh, read-only)

Parent: HELP_HUMAN Agent 0, run `HELP-HUMAN-PIPING-20260921-RECONCILIATION`
(D-73 / DEC-110). Role: TASK (Type 2), fresh context, **read-only**. Do not
edit anything in the repository, do not use git to change state, and do not
delegate. You did not author any of the material under review.

The launch message supplies:
- `{REPO}`: the repository checkout;
- `{FREEZE}`: a read-only checkout of the frozen state
  `00115c71931bcae79909602d653740d3bb72dfa1`;
- `{SCOPE}`: your review scope (one of RV1–RV7 below);
- `{SCRATCH}`: your scratch path.

`RUN` means
`projects/chirality-piping/execution/_Reconciliation/DeliverableConcordance/RECON_2026-09-21_WHOLE_CORPUS`.

## Candidate

The diff `dfcce81e4..HEAD` on branch `claude/piping-recon-r3-20260922`, as
it stands when you start. Record the HEAD you reviewed. Seven reviewers run
in parallel, one per scope. Stay in your scope, but report anything outside
it that you trip over in one line.

The R3 phase:
- the deterministic tables in `RUN/R3/`;
- 14 task analyses in `RUN/R3/TASKS/`, which Agent 0 checked for coverage at
  each return;
- `R3_SYNTHESIS.md` and `COVERAGE_AND_QA.md`.

The R4 phase:
- 29 decision packets in `RUN/R4/DECISION_PACKETS/P1..P3/`;
- four handoffs:
  - H1 `RUN/R3/SCOPE_CHANGE_HANDOFF/`;
  - H2 `RUN/R3/CODE_FIX_BRIEF_CANDIDATES/`;
  - H3 `RUN/R3/ENGINEERING_AUTHORITY/`;
  - H4 `RUN/R4/R5_TRANCHE_PROPOSAL/`;
- Agent 0's integration record `RUN/R4/R4_GATE_INDEX.md`, with
  `tools/check_r4_coverage.py`.

The drafters' contract is
`{REPO}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260921-RECONCILIATION/briefs/R3-INTEGRATION_brief.md`.
Their topic lists and boundaries are in `RUN/R3_INTEGRATION_TOPICS.md`.
Their return records are in `RUN/RUN_STATE.jsonl`, in the `RETURN` events for
`R3-INT-*`.

## Scopes

- **RV1: deterministic layer and reports.**
  - Run `tools/synthesize_r3.py --check`, `tools/index_r3_classes.py --check`
    and `tools/check_r4_coverage.py --check`, all with
    `PYTHONDONTWRITEBYTECODE=1`.
  - Read the three tools for logic errors. For `check_r4_coverage.py`, is
    each `PORTIONS` predicate what the packet actually states in its §5? Look
    for a vacuous check.
  - Check every figure in `R3_SYNTHESIS.md`, `COVERAGE_AND_QA.md` and
    `R4_GATE_INDEX.md` §1, §2 and §6 against the tables.
- **RV2: packets A1–A10.**
- **RV3: packets B1–B12.**
- **RV4: packets C1–C7 and `R4_GATE_INDEX.md` §3–§5.** Are the placements of
  U1–U10 and the other gaps sensible? Is anything on no list still missing?
- **RV5: H1 and H3.**
- **RV6: H2 briefs CFB-01 to CFB-27, and `CODE_FIX_CANDIDATES.md`.**
- **RV7: H2 briefs CFB-28 to CFB-54, and H4.**

## Check (packets and handoffs, RV2–RV7)

1. **Fidelity to the evidence.**
   - For at least a third of the items in scope, and every recommendation,
     open the cited ledger rows, resolutions, register rows and `{FREEZE}`
     `path:line`. Does the citation say what the item claims?
   - Flag any misquoted ruling, wrong DEC number, or code claim the frozen
     tree does not bear out.
2. **Authority.**
   - Effective values are accepted results; task outputs are proposals.
   - Where tasks disagree, are both views shown without choosing?
   - Is each holder right under the brief (OWNER, ENGINEERING, WORKING_ITEMS
     review or scope-change, HELPS_HUMANS, EXTERNAL)?
   - Does any item decide, repair or rule something itself?
3. **Options and recommendations.**
   - Are the options balanced, with consequences stated for deliverables,
     code and other packets?
   - Is each recommendation supported by the evidence? Where it is not, the
     item must say "no recommendation; owner's call".
   - Is the on-ruling mechanism an existing change path? Does it wrongly
     imply that R5, code, lifecycle or DAG work happens without separate
     authorization?
4. **Coverage and keys.**
   - Do the stated portions and filters reproduce the claimed keys over
     `RUN/R3/CLASS_ASSIGNMENTS.csv` (or the task CSV)?
   - Does every cited key exist in `RUN/R3/CORPUS_CLAIMS.csv`?
   - In H2 and H4, does every row whose class or T8 reading needs an owner or
     review decision carry a `BlockedOnPacket`? Does each resolve to a real
     packet ID, an H3 item ID, or a token in an `H3_TOKEN_MAP.csv`?
5. **Fences.**
   - No certification, code-compliance, professional-approval or
     engineering-acceptance claim (F-PIP-2; DEC-081).
   - No equation sources (DEC-043).
   - No quoted protected, private or third-party copyrighted content.
   - Repository-relative paths in committed records.
   - The claim fence is present.
6. **Clarity for the owner.** Could the owner rule from the packet alone? Flag
   any question that is ambiguous, compound in a way that forces an unwanted
   bundle, or missing a consequence the owner needs.

Run no builds or tests. Write scratch files only under `{SCRATCH}/review/`.

## Return

A single message:

1. first line `VERDICT: PASS` (no actionable finding) or `VERDICT: FINDINGS`;
2. the scope and the reviewed HEAD;
3. the items you checked in depth (IDs);
4. numbered findings, each with file and line, severity (`BLOCKING`,
   `ACTIONABLE`, `MINOR`), evidence and the smallest fix;
5. a separate list of items that need the **owner** rather than a repair;
6. last line `END-OF-RETURN`.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
