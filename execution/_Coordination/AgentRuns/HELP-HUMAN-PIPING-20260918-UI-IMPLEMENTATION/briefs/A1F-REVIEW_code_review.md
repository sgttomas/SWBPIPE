# Sealed brief — A1F-REVIEW: independent code review of the Tranche A1 follow-ups

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-18T23:20Z before launch. Role: TASK (Type 2) with the `software-code-review` skill, read-only, fresh context, working alone; Type 2 does not delegate. Model requested: Claude Opus 5, deliberately a different model from the implementer. Mechanism: Claude Code `Agent` tool, general-purpose type, background. You have no write target: your return is your final message, which ROOT retains at `{RUN}/instances/A1F-REVIEW/RETURN.md`.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel` in the worktree you are started in; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`.

Load the skill body first: `{REPO_ROOT}/.agents/skills/software-code-review/SKILL.md` (SHA-256 `06c27b1be5cfbd9e638570918a8f837d8439c8073c40d3ef708e53874f95570a` at sealing; report the hash you read). Where it and this brief differ, this brief's limits (read-only, no delegation) win.

## Candidate

Commit `bb7c7a2e8e128c91b001d69734369f1b211c7122` on branch `codex/swbpipe-a1-followups-20260918`, against `origin/main` (`7866f0a3c2c846cf071f735ea9b263a44fb00ca9`). Review 100 % of the frozen diff outside the run's own records: `git diff origin/main...bb7c7a2e8e128c91b001d69734369f1b211c7122 -- . ':!projects/chirality-piping/execution/_Coordination'`. The `DEC-106` to `DEC-109` rows in `{WORKING_ROOT}/execution/_Decomposition/SOFTWARE_DECOMP.md` are inside the review. The project's rule requires your PASS with no actionable finding before the evidence sweep and before push.

## Authority to check the change against

The owner's words, with hashes: `{RUN}/instances/ROOT/OWNER_APP_ID_2026-09-18.md` and `{RUN}/instances/ROOT/OWNER_DIRECTION_2026-09-18_THREE_ITEMS.md`. The rows `DEC-100`, `DEC-103`, `DEC-106` to `DEC-109`. The sealed brief `{RUN}/briefs/A1F-FOLLOWUPS_identifier_notices_documents.md`. Judge the change against the owner's words and the brief, not against the child's return (`{RUN}/instances/A1F-FOLLOWUPS/RETURN.md`) or ROOT's account, which you read last.

## What to look for

1. **Attribution.** ROOT told the owner of two emitted notices and four documents; the change touches three notices and thirteen documents. Check that the records say exactly that, label ROOT's wider application as ROOT's reading, and attribute to the owner nothing beyond the owner's words. Check that the `DEC` rows state the facts truthfully (the schema's pinned sentence against the producer's, the commit where they diverged, what is left untouched). Report any overstatement.
2. **The three notices.** Only the acceptance clause left; the first clause and "Human review remains required." are intact; no new claims sentence was added; nothing else in the three crates changed. Does any consumer compare, hash-pin or parse the notice text (persistence read gates, the report renderer, comparison, the desktop services)? Would a document saved under the old wording still load? Is any golden fixture or stored hash invalidated?
3. **The schema.** `professional_boundary_notice` is a non-empty string and nothing else in `schemas/rule_check_run_result.schema.json` changed. Is loosening a `const` to a string safe for every consumer of this schema? Verify ROOT's claim that the old `const` was a sentence the producer had not emitted since commit `74902c6c6`. The Python test now asserts the type and minimum length and its comment is truthful.
4. **The documents.** For each of the thirteen: only the registered acceptance text (and a pointer that served only it) left; the rest of the sentence or paragraph survives and still reads correctly; no content-boundary, validation-posture or human-review statement was lost with it; no replacement claim was written. Run `python3 {RUN}/tools/find_acceptance_texts.py --scope-of-work` from `{WORKING_ROOT}` and judge each survivor. Confirm the PRD, `docs/CONTRACT.md`, `docs/PROFESSIONAL_BOUNDARY.md`, `docs/SPEC.md`, `docs/report_notice_template.md`, every `ScopeOfWork.md` and `docs/validation_manual/cases/**` are untouched.
5. **Tests not weakened.** The assertions that required the removed text moved with it; every guard against the text stayed; no behavioural assertion was deleted or loosened in `tests/test_user_guide_status_wording.py`, `tests/test_local_fea_handoff_contract.py`, `tests/test_operation_result_schemas.py`, `apps/desktop/src-tauri/src/lib.rs` or the desktop tests. No tolerance, oracle or benchmark limit changed; `apps/desktop/e2e/ui-foundation/**` is untouched.
6. **The registry.** `docs/claims_registry.md` records the act as a dated amendment, keeps the registered texts listed so the lint's list still matches, stops directing `BS-ACCEPT` onto new artifacts, and rewrites no ruled history. `python3 tools/validation/validate_claims_language.py` from `{REPO_ROOT}` reports VALID. The lint itself is unchanged.
7. **The identifier.** `com.chirality.swbpipe` everywhere it is configured or pinned; `com.swbpipe.desktop` survives nowhere outside `execution/**`; the store carry-forward still resolves the new directory from the configured identifier and still carries from `org.openpipestress.technical-preview` only. Judge ROOT's statement that no store needs carrying from `com.swbpipe.desktop`.
8. **The panel.** "External prover run" in the heading, the accessible name and the prose; both pins moved; no vendor named; identifiers and test ids unchanged.
9. **Scope and records.** Every changed path is inside the brief's write scope or is ROOT's. No identity-layer rename slipped in. No authored file carries an absolute machine path. Copy rules kept.

You may run read-only and test commands that write nothing outside build caches: `python3 -m pytest -q <file>`, `npx vitest run <file>` from `{WORKING_ROOT}/apps/desktop`, `cargo test --lib` offline in a changed crate, the two validators. Do not run Playwright, the evidence sweep or a dev server. Run no state-changing git command. Another change's evidence sweep is running on this host while you work, and its browser tests are sensitive to load: run one test command at a time, and no whole-suite run (`npm run test:desktop`, `python3 -m pytest -q tests`, a workspace-wide cargo build). The implementer's whole-suite results are in its return; the evidence sweep that follows your PASS runs every suite on the candidate.

## Return

Verdict **PASS** (no actionable finding) or **FINDINGS**. For each finding: severity (blocking, major, minor, trivial), whether it is actionable before merge, file and line, what is wrong, the evidence, and the smallest correction. State what you did not check and which model you are.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
