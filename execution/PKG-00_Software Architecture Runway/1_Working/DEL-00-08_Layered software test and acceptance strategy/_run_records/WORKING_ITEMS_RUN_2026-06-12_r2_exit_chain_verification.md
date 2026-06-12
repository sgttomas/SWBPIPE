---
run-id: WORKING_ITEMS_RUN_2026-06-12_r2_exit_chain_verification
timestamp: 2026-06-12T08:20:00-06:00
persona: WORKING_ITEMS
tranche-id: TP-INTEGRATED-VERIFY-002
package-id: PKG-00
deliverable-id: DEL-00-08
run-status: SUCCESS
authority:
  - docs/PRD.md §22.3 (R2 exit criterion)
  - plans/PLAN_2026-06-10_prd_completion.md Phase A exit-evidence row
  - execution/_Coordination/_COORDINATION.md (Working Desktop Application Standard, stage gate)
write-scope:
  - plans/VERIFICATION_2026-06-12_r2_exit_chain.md
  - apps/desktop/SMOKE.md (TP-MAC-140 entry)
  - execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/MEMORY.md
  - execution/PKG-00_Software Architecture Runway/1_Working/DEL-00-08_Layered software test and acceptance strategy/_run_records/**
  - plans/PLAN_2026-06-10_prd_completion.md (Phase A exit-evidence annotation)
  - plans/PLAN_COMPLETION_LOG.md
---

# WORKING_ITEMS Run - TP-INTEGRATED-VERIFY-002

## Scope

Independent verification session (human-instructed) producing the derivative
verification snapshot the completion plan's Phase A exit-evidence row requires
(TP-INTEGRATED-VERIFY successor), after the executing session stopped at the
human stage-advancement gate. Read-only with respect to product code; the two
repair tranches it spawned (`TP-R2VERIFY-FIX-001`, `TP-R2VERIFY-FIX-002`) have
their own run records.

## Re-run evidence

- Tauri backend suite: 32/32, including
  `r2_from_blank_rehearsal_authors_solves_and_renders_report` and
  `r2_from_blank_saved_project_opens_solves_and_renders_report`.
- `core/product_physics`: 25/25 (pre-repair baseline), including the B2
  mixed-unit equivalence witness.
- `core/units`: 13/13 including the schema/crate vocabulary parity test.
- Playwright e2e (real Chrome, wasm engine rebuilt): 2/2 including the
  from-blank GUI journey.
- Packaged desktop binary: `tauri build --debug --no-bundle` built in 1m05s;
  binary launched and registered a Foreground window with WebKit renderer and
  networking processes; zero bytes of error output; terminated cleanly. A
  throwaway `.app` wrapper in `/tmp` (not committed) was used solely to
  register the unbundled binary with macOS launch services.
- Five-surface DEC-025 sweep run at session closeout HEAD (summary committed
  with the closeout evidence commit).

## Findings

Recorded as F-1..F-4 in
`plans/VERIFICATION_2026-06-12_r2_exit_chain.md`: F-1 silent load-category
coercion (repaired same session, `TP-R2VERIFY-FIX-001`); F-2 sweep git-state
false-clean path (repaired same session, `TP-R2VERIFY-FIX-002`); F-3
export-schema compatibility stance unrecorded (residual to the next PKG-17
tranche); F-4 packaged-runtime GUI journey gap with named macOS automation
blockers (stays in the A5/A8 rows).

## Boundary review

No lifecycle state change, stage advancement, release readiness, professional
approval, certification, sealing, authentication, protected-content,
private-data, or code-compliance claim. The snapshot is input to the D-14
human ruling; agents do not advance the stage.

## Handoffs

- D-14 decision packet
  (`execution/_Coordination/_DECISIONS/D-14_r2_stage_advancement.md`) carries
  this snapshot as its evidence basis and awaits the human ruling.
- F-3 and F-4 residuals routed as above.
