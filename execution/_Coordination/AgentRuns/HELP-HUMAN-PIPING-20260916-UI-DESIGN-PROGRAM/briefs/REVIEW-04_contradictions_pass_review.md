# Sealed brief — REVIEW-04: independent review of the contradictions pass and the refreshed handoff

Sealed by ROOT (HELP_HUMAN, successor) on 2026-09-18 before launch. Role: TASK, read-only reviewer, working alone; Type 2 does not delegate. Model requested: Claude Opus 5. Mechanism: Claude Code `Agent` tool, general-purpose type, background. You have no write target: your return is your final message, which ROOT retains at `{RUN}/instances/REVIEW/REVIEW-04_RETURN.md`.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel`; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`.

## Candidate

Commit `80f122300bff3101570beb96b001231350589cd4` on branch `codex/swb-ui-contradictions`, against `origin/main`. Review the complete diff (`git diff origin/main...80f122300bff3101570beb96b001231350589cd4 --stat`, then the files). This brief's own file and index row land in the next commit and are outside the candidate. The scope of the work is design only: no product source, test, schema or root file may have changed.

## Authority to check against

The owner's three messages, recorded verbatim with hashes in `{RUN}/instances/ROOT/OWNER_DIRECTION_2026-09-18_CONTRADICTIONS.md`. That record also holds ROOT's preceding advice, which the first message adopts by reference, and an item-by-item effect table in which ROOT's readings are labelled as ROOT's. The three sealed briefs `DESIGN-SYSTEM-04`, `UX-SPEC-03` and `MOCKS-04` under `{RUN}/briefs/`, and the supplement and corrections recorded in `{RUN}/briefs/_INDEX.md`. The earlier authority of REVIEW-03 (the D-71 and D-72 rulings and addenda, `DEC-099` to `DEC-105`, the owner's handoff message) still binds.

## Checks

1. **Scope.** Every changed path is under `{RUN}`.
2. **The direction record is faithful.** It quotes the owner and separates the owner's words from ROOT's readings. Report any statement, there or in the design system, the specification, the frames' record, the index or the handoff, that attributes to the owner something the three messages do not say. In particular: "Apply" is ROOT's word, not the owner's; the copy of the six additions is the design system child's.
3. **Every one of the twenty-five items is carried as directed, and nothing else changed in meaning.** For each of items 1 to 16 and C-17 to C-25, find where design system V1.3 (change-log rows 91 to 111) and specification V1.2 (§13 rows 35 onward) apply it, and check that the two documents now agree with each other and with the direction. The three owner rulings: item 15 variant A (a tooltip names the control, then the key in parentheses), C-20 variant A (the paste band's buttons carry names only), C-23 variant B (three status chips on the Review page). C-22: a new project opens in Both view. C-25: Run 03. Item 5: no control reads "Commit". Earlier change-log rows and earlier §13 rows must be byte-identical to `origin/main`; check by diff.
4. **The two documents against each other.** Look for a new contradiction the pass introduced. Check the specification's references to design system sections against V1.3's actual section numbers. The frames child reported one defect, C-26 (the specification's §13 rows 36 and 37 crossed their citations); check that the correction is recorded without rewriting those rows.
5. **The frames.** Run your own case-insensitive searches over `{RUN}/instances/MOCKS/frames`: "Commit" as a control's text or tooltip, a key written outside parentheses in a `title` attribute, a key on a paste button's face, "Run 3", the retired maturity and acceptance sentences, a machine path, an external URL loaded as a resource. `s1_table_light` is gone and `s1_both_light` exists, in the frames, the shots and the index. Both `s9` frames carry three status chips and every other frame at most two. The hanger table in `s8_table_light` has no State column. Recompute the hashes in `{RUN}/instances/MOCKS/RETURN.md` §2. You may open screenshots under `shots/`. `MOCKS_V1.md` to `MOCKS_V3.md` are history and keep old strings.
6. **The handoff** (`{RUN}/instances/ROOT/IMPLEMENTATION_HANDOFF_2026-09-18.md`): recompute every SHA-256 in §1; check that the refresh changed only the status paragraph, §1 and §7; that §3's ten constraints still carry the owner's words, including "unless separately justified and authorized" in constraint 5; that §7 states the open items accurately against `MOCKS_V4.md` and the two returns; that it still starts and authorizes no implementation and that §8 asks exactly one decision.
7. **Records.** `briefs/_INDEX.md` records each of the three sealed briefs with a hash that matches the file, the model, the mechanism and the return, and the recorded return hashes match the files at the candidate; `WORK_GRAPH.json` parses, its statuses agree with the index, and no earlier frontier was dropped or altered; history is preserved, not rewritten. No authored file carries an absolute machine path. Governed records end with the claim fence line.
8. **Copy rules.** "Accept", never "Approve", as a control; none of certify, seal, approve, authenticate, comply, compliant or sign-off as a control ("Review/signoff block" is a registered section name); Canadian spelling.
9. **The design system's agreement check.** You may run `node tools/agree.mjs DESIGN_SYSTEM_V1.md tokens.json specimen.html` from `{RUN}/instances/DESIGN-SYSTEM`; it reads files and writes nothing. It should report no problems.

## Return

Verdict PASS or FINDINGS. For each finding: severity (blocking, major, minor, trivial), file and line, what is wrong, and the evidence. State what you did not check. Run no build, test or dev server and no state-changing git command.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
