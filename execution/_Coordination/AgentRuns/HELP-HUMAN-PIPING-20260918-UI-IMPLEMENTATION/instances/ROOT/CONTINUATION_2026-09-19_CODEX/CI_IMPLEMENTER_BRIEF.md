# Sealed brief — Piping CI selection and isolated full-run shards

TASK Type2, parent ROOT HELP_HUMAN, gpt-6-astra/low; no delegation. You are the
sole lent CI writer in swbpipe-wt2 on reviewed PR002dff0f. ROOT owns integration,
Git, tranche manifest and independent review. B3 manager writes tooltip repair
in wt3; no overlapping writes. Do not run browser/app/native/build/full suites.
No Git mutation. Tools/filesystem scopes are instructions, not host enforcement.

Read root/Piping AGENTS, agents/AGENT_TASK.md, relevant loop protected checks,
software-workflow.json, existing Piping workflow/configs and current
OWNER_RESUME_2026-09-20.md in ROOT's checkout. User asks to fix/merge PR825 and
resolve a more efficient CI strategy, then continue graph work. Full hosted run
35496887667 on002dff0f was408pass/20skip/2sameaccessibilityfail,35m55s. Existing
full local source410pass,20skip and dist53pass/sweep remain historical; no new pass.

Implement a small, transparent conservative selection policy and cheaper full
CI orchestration. Do not rewrite Playwright tests, configuration, oracle, timeout,
workers, retries, profile coverage or local DEC025. One worker per isolated runner.
No fullyParallel mutation. Preserve existing exact viewport projects and180000ms
CI timeout. Preserve all full-run test entries, including existing20skips.

Required behavior:
1. Every nonempty source CI run starts with b3-accessibility.spec.ts in both
profiles as an early barrier. If it fails, expensive remainder does not start.
For reduced modes, the barrier may include the complete selected subset in one
job instead of duplicating setup. --max-failures=1 is acceptable for failing fast,
not as a way to report an incomplete run as full; no assertions or skips change.
2. Full CI remains default for product/shared configuration, core/schema/fixture,
unknown or unavailable diff inputs and manual workflow_dispatch. After the fast
barrier, run the rest on four isolated runners via Playwright --shard, each with
one worker. Exclude already executed fast spec from remainder without losing new
specs. Do not globally enable parallel test mode. Aggregate a stable check named
Desktop E2E (source mode), failing on any required failure/cancel/selection error;
only explicitly absent remainder in reduced mode is allowed to be skipped.
3. Safe ordinary reductions: source-spec-only changes run all changed source
specs plus fast spec; e2e/ui-foundation/**-only changes run all instrument source
specs plus fast spec (no benchmark execution). Evidence/docs records may accompany
these changes without forcing broad checks. Any unclassified executable input
falls back to full. Deleted/renamed/unknown test inputs should conservatively
fall back rather than produce zero selected tests. Never use only the last commit
of a PR; route on the complete PR diff against its Git merge base.
4. Owner-authorized one-time PR825 exception: baseline002dff0f244976f98b36517d920b3761f6f88704
must be an ancestor of current head and PR number exactly825. If every subsequent
product change is in styles.css, b3-accessibility.spec.ts, or narrowly named reason
component files explicitly confirmed by ROOT, select all b3-accessibility.spec.ts,
workspace-layout.spec.ts and gui-workflow-validation.spec.ts in both profiles.
CI strategy files/manifest/docs/records may accompany this delta. All other input
changes fall back to full. This carries prior full-run evidence; it is explicitly
partial coverage on the repaired revision and must not emit a full-suite claim.
Ask ROOT for the exact final allowed component paths; do not broaden silently.
5. Emit a readable/machine-readable selection artifact and job summary with base,
head, selection basis, changed paths, selected specs/projects, reasons and whether
coverage is full. A reduced result cannot be used as DEC093 full surface4 evidence.
Keep workflow triggers covering the selector and its tests plus actual existing
inputs. No changes to governance-harness.yml or branch protections. Cancel stale
PR runs; do not add recurring schedules or provider permissions.
6. Avoid repeated setup definitions if a small composite action improves clarity;
copy the existing pinned Node/Rust/WASM/Playwright/dependency setup faithfully.
Do not add third-party routing packages. Prefer Python standard library for routing
with deterministic unit tests, and safe argument arrays for command construction.
No unsafe shell expansion of PR-controlled file names/expressions. Missing selected
files and invalid plan data fail or fall back to full; never pass an empty selector
that could silently invert intended scope.

Write scope in wt2 only: .github/workflows/piping-desktop-e2e.yml; optional
.github/actions/setup-piping-e2e/action.yml; projects/chirality-piping/tools/ci/**;
projects/chirality-piping/tests/test_ci_e2e_plan.py; run evidence under existing
RUN/instances/CI-STRATEGY-CODEX/. Do not edit shared graph/Root records/receipts.
No other product, role, instruction or workflow writes. ROOT writes authorized
instruction-tranche manifest and receiving-loop notice in parallel.

Validate policy using meaningful temporary Git repositories/tests: complete diff,
known reduction, unknown-source fallback, failed/missing baseline, wrong PR,
renames/deletions, malformed plans, manual full, safe command arguments and
full fast+remainder set equality. Parse workflow/action YAML and check job success/
skip/failure/cancel aggregation. List real Playwright tests only if helpful; no
browser is launched by --list. Record actual commands, failures, counts, changed
files and hashes. Return a frozen working diff and brief/role/input hashes.
Do not commit or push. ROOT will review and integrate, then run hosted checks.

References for orchestration: https://playwright.dev/docs/test-sharding and
https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax
(use official current docs if you need syntax confirmation). Preserve old full
failure evidence; test selection is authorized efficiency, not a claim that the
original failure was harmless. No product/release acceptance.
