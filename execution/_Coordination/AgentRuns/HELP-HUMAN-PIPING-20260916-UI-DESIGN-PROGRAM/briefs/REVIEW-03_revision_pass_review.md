# Sealed brief — REVIEW-03: independent review of the revision pass and the handoff

Sealed by ROOT (HELP_HUMAN, successor) on 2026-09-18 before launch. Role: TASK, read-only reviewer, working alone; Type 2 does not delegate. Model requested: Claude Opus 5. Mechanism: Claude Code `Agent` tool, general-purpose type, background. You have no write target: your return is your final message, which ROOT retains at `{RUN}/instances/REVIEW/REVIEW-03_RETURN.md`.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel`; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`.

## Candidate

Commit `3e4cb3a1eeac12791498aec05fd912735d8f841b` on branch `codex/swb-ui-revision-pass`, against `origin/main`. Review the complete diff (`git diff origin/main...3e4cb3a1eeac12791498aec05fd912735d8f841b --stat`, then the files). This brief's own file and index row land in the next commit and are outside the candidate. The scope of the work is design only: no product source, test, schema or root file may have changed.

## Authority to check against

The owner's words, hash-bound, in `{WORKING_ROOT}/execution/_Coordination/_DECISIONS/D-71_RULING_2026-09-18.md`, its two addenda, `D-72_RULING_2026-09-18.md` and its addendum; `DEC-099` to `DEC-105` in `{WORKING_ROOT}/execution/_Decomposition/SOFTWARE_DECOMP.md` §12; the owner's handoff message in `{RUN}/instances/ROOT/SUCCESSOR_ACTIVATION_2026-09-18.md`; and the three sealed briefs `DESIGN-SYSTEM-03`, `UX-SPEC-02`, `MOCKS-03` under `{RUN}/briefs/`.

## Checks

1. **Scope.** Every changed path is under `{RUN}`. Nothing under `apps/`, `core/`, `schemas/`, `docs/`, `tools/` or the repository root changed.
2. **The rulings are carried, none invented.** In design system V1.2, specification V1.1 and the frames: the name is SWBPIPE alone; no maturity sentence; no acceptance sentence or variant; labels only from the one table with their domains; no other vendor's product named as product copy; the Checked mark's words as the addendum gives them; every action has a pointer control (the owner's amendment of Q-20). Run your own case-insensitive searches over `instances/DESIGN-SYSTEM`, `instances/UX-SPEC` and `instances/MOCKS/frames`; do not rely on the children's lints. Earlier passes' records (`MOCKS_V1.md`, `MOCKS_V2.md`) are history and keep old strings. Report any statement that attributes to the owner something the owner's messages do not say.
3. **ROOT's two decisions are labelled as ROOT's.** ROOT settled that a run stops being the current solve basis after a model change and that a Stale run is held on a Historical record's terms. Check that `briefs/_INDEX.md`, the specification and the handoff present these as ROOT's decisions for the pass, not as owner rulings, and check ROOT's factual basis in the product source (`{WORKING_ROOT}/apps/desktop/src/App.tsx`, the function `commitModelAfterSolveInvalidation` and its call site; the specification's §12 item 4 cites what is cleared).
4. **The handoff** (`{RUN}/instances/ROOT/IMPLEMENTATION_HANDOFF_2026-09-18.md`): recompute every SHA-256 in its §1 table; check that it starts and authorizes no implementation, that its ten constraints carry every implementation-handoff constraint of the owner's handoff message without weakening one, that its D-72 summary matches the D-72 ruling and addendum, that its gap classes match the operations map's gap list, and that §8 asks exactly one decision and does not presume the answer.
5. **Records.** `briefs/_INDEX.md` records each sealed brief with a hash that matches the file, the model, the mechanism and the return; `WORK_GRAPH.json` parses and its statuses agree with the index; history in the index, the handoff state and the preparation record is preserved, not rewritten. No authored file carries an absolute machine path. Governed records end with the claim fence line.
6. **Sample the specification's citations.** Pick twenty path-and-line citations from the operations map at random and confirm each names the identifier it claims at `HEAD`.
7. **Copy rules.** "Accept", never "Approve", as a control; none of certify, seal, approve, authenticate, comply, compliant or sign-off as a control ("Review/signoff block" is a registered section name); Canadian spelling.

## Return

Verdict PASS or FINDINGS. For each finding: severity (blocking, major, minor, trivial), file and line, what is wrong, and the evidence. State what you did not check. Run no build, test or dev server and no state-changing git command.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
