# D-14 — R2 Stage Advancement for the Working Desktop Application Standard

**Date prepared:** 2026-06-12
**Prepared by:** WORKING_ITEMS (verification session), per the Application Integration And Issuance Loop decision-escalation step.
**Epistemic status:** PROPOSAL (non-governing). Only the human project authority rules. Nothing here changes lifecycle state, creates release readiness, professional approval, certification, sealing, authentication, or code-compliance claims, or asserts that any PRD milestone "is met" — milestone judgments are the human's.

---

## 1. Decision statement and scope

Whether to advance the **current target stage** recorded in
[`execution/_Coordination/_COORDINATION.md`](../_COORDINATION.md) (Working
Desktop Application Standard) from the **PRD R2 exit criterion** to the **PRD
R3 exit criterion** (rule packs and private libraries in the GUI, PRD §22.4 —
completion-plan Phase C), now that the R2-chain evidence exists and the
executing session has halted at this gate.

The coordination record states: "Agents propose stage advancement with
evidence; only a human-approved coordination update advances the target stage
recorded here." This packet is that proposal. The decision governs **ordinary
tranche selection scope only**: with the target stage at R2, Phase C work is
out-of-stage and the loop halts; advancing makes Phase C ordinary in-stage
work. It does not issue deliverables (all 100 remain `CHECKING` + 1 `ISSUED`),
does not close Phase A residuals, and does not constitute the R2 milestone
acceptance in PRD §22 terms.

## 2. Evidence basis

**Primary artifact:** [`plans/VERIFICATION_2026-06-12_r2_exit_chain.md`](../../../plans/VERIFICATION_2026-06-12_r2_exit_chain.md)
(`TP-INTEGRATED-VERIFY-002`) — independent re-run and audit of the landed
A9–A12/A8 chain against PRD §22.3 verbatim.

Summary of the verified state:

- **Create from blank without raw-file edits** — verified: blank-project path
  (A9), support/material/section creation (A10), full deletion coverage with
  refusal semantics (A11), and the Playwright GUI journey authoring the
  complete A12 script through visible controls (2/2 e2e, re-run).
- **Solve the authored model** — verified: A12 Tauri regression and
  saved-project regression (save → reopen → solve), 32/32 re-run; browser mode
  honestly refuses with named diagnostics.
- **Report the authored model** — verified: hash-bound A7 HTML rendered from
  the authored model in both backend regressions.
- **Packaged runtime** — the `tauri build` binary builds and boots cleanly
  (SMOKE `TP-MAC-140`); the GUI journey **inside the packaged webview** is the
  one unexercised seam (verification finding **F-4**; `tauri-driver` does not
  support macOS, so in-repo automation cannot currently close it).
- Verification findings F-1 (silent load-category coercion) and F-2 (sweep
  git-state false-clean path) were repaired same-session
  (`TP-R2VERIFY-FIX-001`/`-002`); F-3 (export-schema compatibility stance) is
  routed to the next PKG-17 tranche.
- Phase A row residuals remain recorded in the plan (A3 canvas
  gestures/component authoring, A4 subtraction/range expressions, A5
  packaged-GUI smoke and diagnostics polish, A6 directional deformed shape,
  A8 full SMOKE-checklist parity); Phase B is mid-flight (B1 landed; 12 B2
  slices landed; B2/B3 remainder enumerated in the plan rows).

## 3. Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Advance to R3 now, residuals non-blocking.** Phase C becomes ordinary in-stage work; F-4 and the A-row residuals stay recorded selection candidates with no gate attached. | Fastest; risks the packaged-runtime gap decaying into permanent "recorded residual" status. |
| **O-B** | **Hold at R2 until the packaged-runtime GUI journey evidence exists.** Requires resolving the macOS automation gap first — realistically a human-performed manual smoke of the packaged binary against the A12 script (recordable in `SMOKE.md` as a human entry), since `tauri-driver` has no macOS support. | Strongest evidence before any new scope; halts all Phase C work for a gap that is runtime-binding, not functional. |
| **O-C** | **Advance to R3 now with F-4 carried as a named blocking residual at the next stage-exit review.** Phase C opens immediately; the coordination text records that no R3 exit review may be entered while F-4 remains open. | Unblocks the program and keeps teeth on the gap. |

**Recommendation (PROPOSAL): O-C.** Every element of the R2 criterion is
proven on at least one automated surface and independently re-run; the
remaining gap binds proven parts together in the packaged runtime rather than
questioning any of them. Holding all Phase C work for it (O-B) buys little —
while attaching it as a blocking residual at the next gate (O-C) prevents the
decay O-A would invite. The cheapest unconditional closure of F-4 remains
available at any time: one human manual pass of the packaged binary through
the A12 script, recorded in `SMOKE.md`.

## 4. Proposed coordination edit (applied only on acceptance)

Replace the "**Current target stage — PRD R2 exit criterion.**" paragraph of
`_COORDINATION.md` (Working Desktop Application Standard) with:

> **Current target stage — PRD R3 exit criterion (advanced 2026-06-12 by
> `DEC-0XX`).** A user can define a private, non-code rule pack and run checks
> from the GUI, with pass/fail blocked on missing inputs (PRD §22.4), reached
> through the completion plan's Phase C items. R2-stage evidence was reviewed
> per `plans/VERIFICATION_2026-06-12_r2_exit_chain.md`; the packaged-runtime
> GUI journey (verification finding F-4) remains a named blocking residual
> that must close before any R3 exit review. All boundary prohibitions below
> continue unchanged.

The existing boundary-prohibition bullets and the "Later stages" paragraph
remain; "Later stages" drops R3 from its list. The executing agent applies
this edit as its first coordination action after the ruling, mirroring the
seam-plan acceptance pattern.

## 5. Companion dispositions (same sitting, separate small rulings — not bundled into D-14)

1. **F-3 contract-corpus fixture review** —
   `fixtures/model_operations/contract_corpus/README.md` still records
   `review_status: pending human review` (T1 residual, 2026-06-11).
2. **C1 grammar ASSUMPTIONs** — three labeled ASSUMPTIONs from
   `TP-C1-GRAMMAR-001` await disposition (dimension-product table contents,
   derived unit-ref convention, Select branch-compatibility rule); see
   `execution/PKG-06_Rule Packs and User-Supplied Code Check Engine/1_Working/DEL-06-02_Sandboxed unit-aware expression evaluator/_run_records/TASK_RUN_2026-06-11_TP-C1-GRAMMAR-001.md`.
   These gate parts of Phase C formal review, so ruling them at the same
   sitting is efficient if the stage advances.

## 6. Downstream impact if ruled to advance

- First next-stage decision action: prepare the **D-02b** packet (writable
  rule-expression text syntax, deferred by `DEC-022` to the C2 lead-up).
- C2/C3 consume A3's authoring shell; B2/B3 continue as the parallel units
  lane; the CAEPIPE oracle plan's implementation tranches remain gated on
  their own §9 sequencing.
- F-4 closure path stays open in the A5/A8 rows regardless of option chosen.

## 7. Ruling mechanism

Per existing practice (`DEC-018`..`DEC-028`): the human project authority
selects an option (or rules directly); the ruling is appended to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as the next `DEC` entry
citing this packet; the coordination edit in §4 is applied; this register row
moves to `RULED` with the record pointer. Agents never apply the stage change
without the recorded ruling.
