# DEL-08-05 Report protected-content linter — worker notes (W3, PKG-08, worker G2)

Forward ledger sealed: `DEL-08-05_SEAL.txt`. Evidence read from the freeze
checkout at `00115c719` only. Agent judgments, not owner rulings.

## Path aliases

- Engine: `core/reporting/protected_content_linter/src/lib.rs` (crate
  `open_pipe_stress_protected_content_linter`), CLI
  `…/src/bin/protected_content_lint_cli.rs`.
- Release scan tool cited as the project copy
  `projects/chirality-piping/tools/release/run_release_candidate_scan.py`
  (a repository-root copy of `tools/release/` also exists; the validator
  requires the explicit project form).
- Product callers: `core/reporting/report_renderer` (pre/post-render lint) and
  `core/reporting/pdf_emitter`, reached through the Tauri report-package
  command; the desktop `ReportLintPanel.tsx` is a separate TypeScript lint.

## Judgment calls

- **FG-DEL-08-05-02 — possible defect (REQ-006, CLM-006.r03, CLM-013.r04,
  AC-001 note).** The engine's `prohibited_claim_phrases` matches
  "certified/sealed/approved/authenticated by openpipestress" plus "code
  compliant" and one generic phrase. After the SCA-010 rename, the same claims
  naming SWBPIPE are not matched by the engine the renderer and PDF emitter
  use. Judged PARTIALLY_IMPLEMENTED · POSSIBLE_DEFECT · INVARIANT · CLAIMS,
  AuthorityNeeded REVIEW. This is rename residue with a functional
  professional-boundary consequence; the verifier and owner should see it.
- **FG-DEL-08-05-01 (REQ-002, CLM-006.r01, AC-001):** detection is limited to
  planted synthetic markers and the DEC-058 standards-table signature; copied
  prose, formulas, figures and proprietary text without a numeric grid have no
  heuristic → PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · IP_DATA
  (MEDIUM; the heuristic nature is acknowledged by the SOW itself).
- **FG-DEL-08-05-03 (REQ-010, CLM-004.r04, CLM-013.r07, OUT-001,
  CONTEXT#anticipated-artifacts):** no CI guard runs the linter. DEC-058 set
  the owner-run release-scan procedure and DEC-059 deferred hosted CI →
  DEFERRED_BY_RULING · PROJECT_BASELINE · RULED_CRITERION (OUT-001 kept
  LOCAL_DESIGN · PARTIAL_SLICE because its gap is the CI boundary element of a
  local output claim).
- **REQ-012:** report integration holds; adapter/plugin runtime does not exist
  to bind into (CP-11) → PARTIALLY_IMPLEMENTED; Remaining R01 is ALIGNED with
  `OPEN_ACTION` to REQ-012.
- **Remaining R02** (owner release-candidate scan act): accurate; ALIGNED with
  `OPEN_ACTION` to REQ-010 (F2).
- **OQ-003 / OQ-005:** partly or fully settled by DEC-058 →
  STALE_SETUP_SPECIFICATION · SCOPE_REDIRECTED_BY_RULING · PROJECT_BASELINE ·
  RULED_CRITERION. OQ-001/002/004 settled by implementation →
  STALE_SETUP_SPECIFICATION · DOC_BEHIND_CODE.
- **CLM-007.r01:** `INIT.md` does not exist at repository or project root →
  CP-02.
- **CLM-010:** judged as setup-framed text; its in-scope list restates REQ
  rows and is judged there.
- **Tables split:** CLM-003, CLM-004, CLM-006, CLM-007, CLM-013, CLM-019,
  CLM-024. Unsplit: CLM-005, CLM-028, CLM-030, CLM-032.

## Canonical departures

None. CP-03 blocks carry CP-02 fields.

## Convention friction

- A test that exists but keys on the former product name (CLM-013.r04) sits
  between "check exists" and "rename residue"; judged with FG-02.
- The desktop lint panel and the Rust engine are separate implementations with
  different phrase lists; the ledger judges the engine for the claims about the
  linter and notes the panel.

## Rename residue observed (for R3)

SOW lines 183, 388; crate name `open_pipe_stress_protected_content_linter`;
desktop filename prefix `openpipestress-preview-report-lint-`; engine
prohibited-claim phrases (functional; FG-02).

## UNKNOWN rows — smallest next check

- `DEL-08-05:SOW#CLM-013.r08` (privacy check): search `tests/` and the
  linter crate tests for an assertion that lint and scan runs make no network
  or telemetry transfer; if none exists, record the zero-dependency crate
  (`Cargo.toml` `[dependencies]` empty) as the only evidence and re-dispose.

## Reverse pass

Read the routing file only after all three seals. No change to sealed views.
Overlapping capabilities answered specifically (F5): RC-08-0034, 0035, 0043,
0093, 0102, 0112, 0124, 0151, 0157, 0160, 0168, 0175, 0219, 0244, 0254, 0275,
0297, 0300, 0316. RC-08-0093 (renderer/PDF lint gate) answered PARTIAL.

## Batch consistency

`validate_ledger_v2.py --batch` over DEL-08-04/05/06: PASS, 0 findings.

## Selectability

`SelectableUnderCurrentLoop` is `NOT_APPLICABLE` on every row (C9).

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
