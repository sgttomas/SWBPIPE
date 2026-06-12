# DRAFT — H4 evidence-posture amendments to `_COORDINATION.md`

**Status:** APPROVED and APPLIED — the human project authority approved
this wording on 2026-06-11 ("I approve the 'H4' amendments"); the insertion
landed in `execution/_Coordination/_COORDINATION.md` step 8 the same day
(tranche `TP-H4-EVIDENCE-POSTURE-001`). This file is retained as the draft
basis and approval pointer; the live coordination record is authoritative.

## Proposed insertion

Location: `_COORDINATION.md` → "Application Integration And Issuance Loop"
→ step 8 ("Fan-in and validation"), appended after the existing expected
evidence sentence.

> UI evidence posture: when a tranche changes user-visible desktop
> behavior, the default evidence is an extension of the Playwright e2e
> spec(s) exercising the changed behavior in a real browser; a manual
> live-browser smoke note in `apps/desktop/SMOKE.md` without a spec
> extension is the exception and must record why automation was not
> extended. New React components land with unit tests (Vitest) at or above
> the slice's existing coverage pattern; a component with no unit test is a
> recorded evidence gap, not a silent omission. Homogeneous UI slices
> (several near-identical forms, rows, or panels produced from one
> template) may record one template-level test plus per-instance smoke
> assertions instead of duplicating the full suite per instance, provided
> the run record names every instance covered by the template rule.

## Why (one paragraph)

The seam plan's closure review (§9.2) found UI behavior changes carried by
manual SMOKE notes while the Playwright harness already exists (A8), and
the F-1 repair showed manual smoke does not protect against regression
budgets. Making spec extension the default, unit tests a floor, and the
template-batch rule explicit converts current good practice into the
recorded loop contract without adding a new evidence surface.

## Decision requested

Approve, amend, or reject the insertion above. On approval, the edit lands
as a small coordination tranche citing this draft and the H4 row; the H4
row is then compressed per the plan-maintenance rule.
