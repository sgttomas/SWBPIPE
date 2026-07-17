# AgentRuns Record — D-48 Claims-Language Alignment (2026-07-16)

**Parent:** session agent (direct human invocation; owner in-session)  
**Selection authority:** `HUMAN` ("Plan and execute with subagents, as
before. Merge PRs as before." — D-48 → O-A, packet §9)  
**Posture:** `MIXED` (Wave-1 fan-out with one serial test-update stage after
fan-in; Wave-2 fan-out)  
**Authority:** D-48 O-A / `DEC-081`; actions pre-accepted (packet §9);
ruling merged to `main` via PR #257 (`d171339b8`) before dispatch.  
**Harness:** Claude Code session subagents; briefs frozen in the dispatch
prompts (session transcript); returns summarized at fan-in below.

## Pre-dispatch exploration (2 read-only nodes, during planning)

- **E1 — product-surface inventory:** ~54 UI display strings across ~40
  panels + App.tsx banner; ReportLintPanel expectation corpus (~44 strings);
  2 emitted core notices (report_renderer:719 — not §19.3-equivalent-text;
  rule_check_runner:78); ~20 doc passages / 8 files; 1 e2e + ~13 Vitest
  literal assertions; DEC-025 sweep mechanics confirmed (5 fixed surfaces).
- **E2 — governance inventory:** 45 SOWs / 77 occurrences in 5 shapes
  (A:36 table cells, B:19 prose tails, C:13 bullets, D:7 steps, E:2 ACs);
  all 16 `_STATUS` occurrences under `## History` (ruled history → zero
  edits); litany is emergent from OPS-K-AUTH-1 + F-PIP-2 (no template);
  lint precedent = receipt-validator wrapper + GEN-13 self-check slot;
  DEC-025 surface list fixed (not modified — pytest shim joins the
  existing `python_pytest` surface).

## Wave 1 — Phase A fan-out (6 nodes, concurrent, disjoint writes)

| Node | Objective | Write scope | Expected return |
|---|---|---|---|
| W1-LINT | Author `tools/validation/validate_claims_language.py` + colocated test + piping `tests/` shim + GEN-13 in `cmd_self_check.py` (repo-root files = DEC-081 scope grant) | the 4 named files only | files + test results + live finding sample |
| W1-UI-1 | Registry alignment, 16 feature dirs (accessibility-baseline…model-tree) | those dirs, non-test | old→new string map |
| W1-UI-2 | Registry alignment, 19 feature dirs + App.tsx banner + services | those files, non-test | old→new string map (banner exact) |
| W1-LINTPANEL | ReportLintPanel display strings + 44-entry expectation corpus + its own tests | `features/report-lint/**` | corpus-usage finding + map |
| W1-CORE | Emitted Rust notices → §19.3 canonical / BS-ACCEPT; colocated Rust test updates; cargo green | `core/**` | map + cargo results |
| W1-DOCS | 10 user-facing docs aligned; §19.3 template & anchor docs strengthened | the 10 named files | map |

Dependencies: none among the six (registry authored by parent first).
Fan-in gate: parent merges the old→new maps, reconciles against the E1
inventory, then dispatches the serial stage.

## Wave 1 — Phase B (serial, after fan-in)

| Node | Objective | Write scope |
|---|---|---|
| W1-TESTS | Update all cross-cutting test assertions from the merged maps (`App.test.tsx`, `RulePackManagerPanel.test.tsx`, `e2e/r2-smoke.spec.ts`) | those test files only |

Then parent: lint run, evidence battery (DEC-025 sweep, self-check incl.
GEN-13, harness pytest, receipt validator w/ Receipt-50), `_STATUS.md`
history lines, PR, session-authorized merge.

## Human decision points

All owner acts taken before dispatch (O-A ruling, pre-acceptance, merge
authority — packet §9). Behavioral (non-text) surprises return to the owner.

## Returns (fan-in log)

**Phase A (all six nodes COMPLETED; returns valid at fan-in):**

- **W1-LINT:** validator + 13-test colocated suite + piping shim + GEN-13
  wiring delivered; harness suite 265/266 with the live-baseline severity pin
  issue anticipated and resolved by the parent's wave gating (below).
  Disclosed deviation: one read-only `wc`-class Bash call — accepted.
- **W1-UI-1:** 23 string edits / 17 files, registry-verbatim, complete
  old→new map returned; scope statements preserved.
- **W1-UI-2:** 39 edits incl. the banner (BS-MATURITY + BS-ACCEPT short),
  8 sweep additions, full map returned; flagged the rulePackService draft
  template checksum coupling for the test stage.
- **W1-LINTPANEL:** corpus verified self-contained (no renderer coupling);
  42/44 entries aligned by boundary class; count/testid assertions
  unaffected; flagged a historical SMOKE.md quote (dispositioned: dated
  evidence-ledger entry, left).
- **W1-CORE:** renderer notice → PRD §19.3 verbatim; runner const →
  BS-ACCEPT; 4 further emitted litanies aligned; 11 crates cargo-green; the
  only core test asserting old text updated. Kept-list returned.
- **W1-DOCS:** 10 docs aligned with kept-with-reason list; §19.3 template
  re-anchored verbatim; anchor docs cross-referenced.

**Parent integration (fan-in edits, serialized):** validator wave-gating
(`WAVE2_SURFACES_ACTIVE=False`: Wave-1 docs list + SOW scan deferred to the
Wave-2 tranche per the plan — resolves the live-baseline pin without churn);
authority-citation (§21.2) and enforcement-vocabulary suppressions with
tests (17/17 green); BS-ACCEPT standalone-second-sentence variant added to
registry + validator; gap file `ResultExportPanel.tsx:352` aligned (fell
between the two UI group splits); three comment litanies aligned; RELEASE_
NOTES_TEMPLATE §21.2 citation added; two doc passages re-wrapped/aligned.
**Live lint result after fan-in: VALID, 80 files scanned, exit 0.** Sweep
shim test passes.

**Phase B:** W1-TESTS dispatched with the merged old→new maps (in flight).

## Wave 2 — dispatch plan (2026-07-16, post-Wave-1 merge `c99e36963`)

Live-tree enumeration (lint with Wave-2 scope active): 494 findings — 374
across 82 `ScopeOfWork.md` files (larger than the E2 estimate of 45/77
because the deterministic litany threshold catches more than literal
restatements; same shapes, same rules, within the ruled O-A scope), 63
identical validation-manual case-page boilerplate sentences (parent-fixed
mechanically before dispatch: BS-VALID-derived replacement across 64 files),
and 57 judgment-requiring doc findings in 21 files.

Fan-out (7 nodes, concurrent, disjoint writes, file-tools-only, sealed
briefs with exact `path:line:text` inventories frozen in the dispatch
prompts):

| Node | Scope | Findings/files |
|---|---|---|
| W2-SOW-1 | PKG-01..04 SOWs + own `_STATUS` History appends | 74/18 |
| W2-SOW-2 | PKG-05..08 SOWs + appends | 86/22 |
| W2-SOW-3 | PKG-09..12 SOWs + appends | 97/20 |
| W2-SOW-4 | PKG-13..15 SOWs + appends | 73/13 |
| W2-SOW-5 | PKG-16..17 SOWs + appends | 44/9 |
| W2-DOCS-1 | SPEC, TYPES, DIRECTIVE, INTENT, RELEASE_QUALITY_GATES, AGENTIC_DEVELOPMENT_WORKFLOW, VALIDATION_STRATEGY | 37/7 |
| W2-DOCS-2 | architecture/, security/, adr, contributor_guide, local_analysis, _Examples | 20/14 |

Disposition rules (all briefs): (a) litany table cells / whole bullets →
GF-TOKEN sentence; (b) claims-tails in prose/steps/ACs → surgical excision
preserving deliverable-specific scope exclusions, GF-TOKEN or registry short
variant where a boundary statement is genuinely needed; (c) structured
metadata identifiers and enum values never renamed — prose around them
aligned; legitimate prohibited-vocabulary enumerations keep their text with
a "(PRD §21.2)" authority citation; (d) `## History` in `_STATUS.md` never
edited; one dated History append per edited deliverable citing DEC-081.

Parent (serialized, after fan-in): validator `WAVE2_SURFACES_ACTIVE=True`
flip + test updates; authoring-directive note beside F-PIP-2 in the loop
workplan (fence text unchanged); cross-project residuals recorded
(`agents/AGENT_DOMAIN_ENGINE.md`, `skills/software-bounded-implementation/`
`QA_CHECKS.md` — outside the piping fence, not edited); lint green over the
full live tree; self-check; harness pytest; Receipt-51; PR; session-
authorized merge. Docs-only wave → no DEC-025 sweep (per plan).
