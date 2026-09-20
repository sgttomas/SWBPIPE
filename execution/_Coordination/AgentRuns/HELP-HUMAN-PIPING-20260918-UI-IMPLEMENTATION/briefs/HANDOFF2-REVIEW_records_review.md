# Sealed brief — HANDOFF2-REVIEW: independent read-only check of the second session handoff of 2026-09-19

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-19 before launch. Role: TASK (Type 2), read-only, fresh context, working alone; Type 2 does not delegate. Model requested: Claude Opus 5. Mechanism: Claude Code `Agent` tool, general-purpose type. No write target: your return is your final message, which ROOT retains at `{RUN}/instances/HANDOFF2-REVIEW/RETURN.md`. The owner is near a usage limit: keep this narrow and do not read product code.

Path placeholders: `{REPO_ROOT}` is `git rev-parse --show-toplevel`; `{RUN}` is `{REPO_ROOT}/projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`. Write no absolute machine path in your return.

## Candidate

Branch `codex/swbpipe-records-20260919` against `origin/main`: `git diff origin/main...HEAD --stat`. Records only. The files that matter: `{RUN}/instances/ROOT/SESSION_HANDOFF_2026-09-19B.md`, `{RUN}/instances/ROOT/SUCCESSOR_PROMPT_2026-09-19B.md`, the two owner-direction records of this session beside them, `{RUN}/HANDOFF_STATE.md` (this session's entries at its end), `{RUN}/briefs/_INDEX.md` (its last section) and the two launch briefs in `{RUN}/briefs/`.

## What to check

1. **Every checkable fact in the handoff is true:** each commit, branch head, merge commit, pull-request number and head (`gh pr view <n> --json headRefOid,mergeCommit,state`), SHA-256 and path it states. The lane branches are `origin/codex/swbpipe-b-shell-20260918` and `origin/codex/swbpipe-b-canvas-20260918`; read files on them with `git show <ref>:<path>`. Check that the files the handoff tells a successor to read exist where it says, on the branch it says.
2. **A successor could act from it.** Reading only what the successor prompt names, would a new session know the state, the order of work, what blocks, and its first steps? Name anything a successor would have to guess. Check the handoff against the two lanes' own pause records (`{RUN}/instances/B-SHELL/LANE_LOG.md` and `{RUN}/instances/B-CANVAS/LANE_LOG.md` on their branches, last sections only) for contradiction.
3. **The owner's words.** In the two owner-direction records and wherever the handoff or the prompt attributes a decision to the owner: is each attribution supported by the quoted words, and is ROOT's reading labelled as ROOT's? The successor prompt is a draft of the owner's message: flag any line in it that states as the owner's direction something the records show only as ROOT's decision.
4. **Records hygiene:** no absolute machine path in an authored file of the range; each governed record ends with the claim fence line; no claim of usability, conformance, performance or owner acceptance.

You may run read-only git and `gh` commands and `shasum`. No tests, no builds, no state-changing git, no file writes.

## Return

Begin with `# HANDOFF2-REVIEW return`. Verdict **PASS** (no actionable finding) or **FINDINGS**; for each finding: severity (blocking, minor, trivial), the file and line, what is wrong, the evidence, the exact corrected wording. State what you did not check and which model you are.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
