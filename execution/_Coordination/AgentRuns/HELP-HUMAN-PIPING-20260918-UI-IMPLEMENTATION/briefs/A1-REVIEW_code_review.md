# Sealed brief — A1-REVIEW: independent code review of Tranche A1

Sealed by ROOT (HELP_HUMAN, Agent 0) on 2026-09-18 before launch. Role: TASK (Type 2) with the `software-code-review` skill, read-only, fresh context, working alone; Type 2 does not delegate. Model requested: Claude Opus 5, deliberately a different model from the implementers. Mechanism: Claude Code `Agent` tool, general-purpose type, background. You have no write target: your return is your final message, which ROOT retains at `{RUN}/instances/A1-REVIEW/RETURN.md`.

Path placeholders: resolve `{REPO_ROOT}` with `git rev-parse --show-toplevel`; `{WORKING_ROOT}` is `{REPO_ROOT}/projects/chirality-piping`; `{RUN}` is `{WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION`.

Load the skill body first: `{REPO_ROOT}/.agents/skills/software-code-review/SKILL.md` (SHA-256 `06c27b1be5cfbd9e638570918a8f837d8439c8073c40d3ef708e53874f95570a` at sealing; report the hash you read). Follow it; where it and this brief differ, this brief's limits (read-only, no delegation) win.

## Candidate

Commit `8143645eacff25e56d685259301795f0c3a482cb` on branch `codex/swbpipe-a1-visible-rename-20260918`, against `origin/main` (`04aefa9803a6a9d26dae39ad6bbc06c470963fad`). Review 100 % of the frozen diff: `git diff origin/main...8143645eacff25e56d685259301795f0c3a482cb`, 105 files. The project's rule (`{WORKING_ROOT}/AGENTS.md`, "Agent Posture And Delegation") requires your PASS with no actionable finding before the evidence sweep and before push.

## Authority to check the change against

The owner's authorization and three planning rulings: `{RUN}/instances/ROOT/ACTIVATION_2026-09-18.md`. The acts: `DEC-100`, `DEC-101`, `DEC-102`, `DEC-103`, `DEC-105` in `{WORKING_ROOT}/execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (read the rows whole; `DEC-099` is superseded and `DEC-104` is not in this tranche), and the scope change bundle `{WORKING_ROOT}/execution/_ScopeChange/SCA-010_2026-09-18_1400/`. The three sealed briefs under `{RUN}/briefs/` and the children's returns under `{RUN}/instances/`. Judge the code against the rulings and the briefs, not against the children's or ROOT's account of it.

## What to look for

1. **Correctness and regressions** in every changed source file, as the skill directs. This tranche is meant to change displayed text, packaging identity and one persistence fallback, and nothing else in behaviour. Report any behavioural change beyond that.
2. **Identity left alone.** No document-kind or artifact-kind string, schema `$id`, package, crate, binary or library name, event name, window global, environment variable, test id, `openpipestress_jcs_ijson_v1`, `openpipestress_result_semantics_v0_2`, `.opsproj`, or the store filename changed. No string that is compared by code, stored in a saved document, hashed or pinned by a schema `const` changed. `schemas/**` and `fixtures/**` are untouched.
3. **The rulings are fully carried on live surfaces in scope.** Run your own case-insensitive searches over `apps/desktop` (not `node_modules`, `public/wasm-engine`, `SMOKE.md`), `core`, `docs` (not `_history`, `_ScopeChange`) and `tools/release`: the maturity sentence; each of the five acceptance texts and close paraphrases ("acceptance … responsible engineer"); "Technical Preview" as part of the name; the curated labels "Review required" and "Inputs needed"; the vendor name rendered to a user. Judge each survivor: identity or data contract (acceptable, should be listed in a return), deferred by the brief (governance documents, validation-manual cases), or a miss.
4. **The status labels** (`apps/desktop/src/features/workspace/statusLabels.ts`): exactly `DEC-102`'s eight forms; token reachable in place; domain always shown; no label outside the table; the handling of values that are not among the eight tokens is honest; the two legacy-enum mappings preserve prior behaviour.
5. **The lint** (`{REPO_ROOT}/tools/validation/validate_claims_language.py` and its test): `REGISTERED_TEXTS` matches `{WORKING_ROOT}/docs/claims_registry.md` exactly; the two new guards are sound, with firing and clean cases; nothing previously enforced is weakened (`RETIRED_PHRASE`, the litany rule, the PRD and renderer anchors). The registry's acts are recorded as amendments and ruled history is not rewritten.
6. **The store copy-forward** (`apps/desktop/src-tauri/src/lib.rs`, `carry_forward_legacy_project_store` and `legacy_project_store_source`): never moves, deletes or overwrites; safe against partial copies and races; cannot block startup; SQLite sidecar handling is coherent (consider a WAL copied without its main file, or a main file copied while the legacy store is open elsewhere); tests cover what they claim.
7. **Tests not weakened.** Every changed assertion moved to new text or to an equivalent behavioural check; none was deleted to obtain a pass. Judge specifically the change in `apps/desktop/e2e/ui-foundation.spec.ts` that sets one test's window to 1440 × 899 after the footer was removed: is it an honest preservation of the geometry the frozen endpoints were characterized against, or does it mask a regression? No picking tolerance, oracle or benchmark limit may change anywhere in the diff (`apps/desktop/e2e/ui-foundation/**` should be untouched; say if it is not).
8. **SCA-010 exactly:** `docs/PRD.md` has 23 replacements, the historical path on the excepted line is unchanged, the header follows A004; `docs/report_notice_template.md` three lines; the renderer's emitted notice agrees with both and keeps "decision-support software"; `_LATEST.md` points to SCA-010.
9. **Scope and records.** Every changed path is inside one child's write scope or is ROOT's (the run directory and the tranche manifest). The manifest `{REPO_ROOT}/docs/governance_harness/tranche_manifests/PIPING-SWBPIPE-A1-CLAIMS-LINT-20260918.yaml` states the authorization truthfully. No authored file carries an absolute machine path. Copy rules: "Accept", never "Approve", as a control; none of certify, seal, approve, authenticate, comply, compliant or sign-off as a control; Canadian spelling in new prose.

You may run read-only and test commands that write nothing outside build caches: `python3 -m pytest -q tools/validation/test_validate_claims_language.py`, `python3 tools/validation/validate_claims_language.py`, `npx vitest run <file>` from `{WORKING_ROOT}/apps/desktop`, `cargo test --lib` offline in a changed crate. Do not run Playwright, the evidence sweep or a dev server; ROOT runs those after your verdict. Run no state-changing git command.

## Return

Verdict **PASS** (no actionable finding) or **FINDINGS**. For each finding: severity (blocking, major, minor, trivial), whether it is actionable before merge, file and line, what is wrong, the evidence, and the smallest correction. State what you did not check and which model you are.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
