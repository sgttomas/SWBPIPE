# Sealed brief — B1-REVIEW: independent code review of slice B1 (design tokens)

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-18 before launch. Role: TASK (Type 2) with the `software-code-review` skill, read-only, fresh context, working alone; Type 2 does not delegate. Model requested: Claude Opus 5, deliberately a different model from the implementer. Mechanism: Claude Code `Agent` tool, general-purpose type, background. You have no write target: your return is your final message, which ROOT retains at `{RUN}/instances/B1-REVIEW/RETURN.md`.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel`; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`; `{DESIGN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260916-UI-DESIGN-PROGRAM`.

Load the skill body first: `{REPO_ROOT}/.agents/skills/software-code-review/SKILL.md` (SHA-256 `06c27b1be5cfbd9e638570918a8f837d8439c8073c40d3ef708e53874f95570a` at sealing; report the hash you read). Where it and this brief differ, this brief's limits (read-only, no delegation) win.

## Candidate

Commit `bf1ad5662e7e21ee3ce065940ed29bbe50511f1c` on branch `codex/swbpipe-b1-tokens-20260918`, against `origin/main` (`7866f0a3c2c846cf071f735ea9b263a44fb00ca9`). Review 100 % of the frozen diff under `apps/desktop/**`: `git diff origin/main...bf1ad5662e7e21ee3ce065940ed29bbe50511f1c -- projects/chirality-piping/apps/desktop`. The run's own records in the same range are ROOT's and are outside the code review, except check 6. The project's rule requires your PASS with no actionable finding before the evidence sweep and before push.

## Authority to check the change against

The sealed brief `{RUN}/briefs/B1-TOKENS_design_tokens.md` and the design system V1.3 (`{DESIGN}/instances/DESIGN-SYSTEM/DESIGN_SYSTEM_V1.md` §1 and §2, `tokens.json`, `tools/gen.mjs`). Judge the code against those, not against the child's return (`{RUN}/instances/B1-TOKENS/RETURN.md` and `CORRECTION_1_RETURN.md` beside it), which you read last.

## What to look for

1. **The token file** in the product is byte-identical to the design system's; the recorded SHA-256 is right; the product build reads nothing under `execution/**`.
2. **The generator** reproduces `cssFrom` faithfully: same variable names, same theme blocks; `src/tokens.css` equals its output; the drift test really fails on drift (reason about it or run it).
3. **Theme resolution.** Light, dark and the "system" preference each resolve to the right token block, including when the operating system's scheme and the explicit preference disagree. No flash or mismatch between the token blocks and the existing `.app-shell[data-theme]` rules. Existing preference behaviour and its tests are preserved.
4. **The variable mapping.** 21 of the 25 `--ui-*` variables are defined from tokens and four keep their values (the two canvas colours, the control border, and, after ROOT's correction 1, the disabled ink). Each mapped variable maps to the token the design system's §2 gives that role; none is mapped to a token of a different role for convenience; judge the child's four stated judgment calls (`surface.header`, `text.secondary`, `selection.bar`, the status inks) and each kept variable's reason. Contrast: for text and control roles, check the mapped pairs against the design system's §2.9 contrast table; report any pair that falls below WCAG 2.2 AA that did not before.
5. **Nothing else changed.** No layout, structure, behaviour, operation, gate, picking or viewport file; no test weakened; `e2e/ui-foundation/**` untouched; no status label outside `statusLabels.ts`; copy rules kept.
6. **Records.** No authored file carries an absolute machine path.

You may run read-only and test commands that write nothing outside build caches (`npx vitest run <file>` from `{WORKING_ROOT}/apps/desktop`, `node scripts/generate-tokens.mjs` to a temporary directory if it takes one). Do not run Playwright, the evidence sweep or a dev server. Run no state-changing git command.

## Return

Verdict **PASS** (no actionable finding) or **FINDINGS**. For each finding: severity (blocking, major, minor, trivial), whether it is actionable before merge, file and line, what is wrong, the evidence, and the smallest correction. State what you did not check and which model you are.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
