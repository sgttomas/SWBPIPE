# D-25 - R4 Exit Scope and Residual Disposition

**Date prepared:** 2026-06-22
**Prepared by:** WORKING_ITEMS, per the Application Integration And Issuance
Loop decision-escalation step.
**Epistemic status:** RULED by human project authority on 2026-06-22. The
proposal basis is retained below. The ruling authorizes preparing the final R4
exit-chain verification packet, but it does not change lifecycle state, issue
deliverables, advance the current target stage, create release readiness,
professional approval, certification, sealing, authentication, code-compliance
acceptance, or assert that R4 is complete.

---

## 1. Decision Statement And Scope

Decide the **R4 exit-scope bar** for the PRD §22.5 milestone after the landed
Phase D component and nonlinear-support evidence.

PRD §22.5 defines R4 as "Piping Components and Nonlinear Supports" with these
exit criteria:

- nonlinear support validation cases converge;
- component provenance appears in reports.

This packet does not reopen `DEC-044` (assembled nonlinear solve ownership),
`DEC-045` (component macro-element realization), `DEC-046` (class-tiered
convergence policy), `DEC-049` (minimal spring-hanger user-data scope), or
`DEC-050` (R4 sparse evidence lane with dense remaining default). It asks
whether the current remaining Phase D residuals are R4 exit blockers, explicit
post-R4/R5 residuals, or a mixed set.

## 2. Current Evidence Basis

R4 component-provenance evidence is landed for the current invented preview
surface:

- bend and branch invented paths: `TP-R4-D1-BENDVIS-001`,
  `TP-R4-D1-BENDSTRESS-001`, and `TP-R4-D2-BRANCHSTRESS-001`;
- rigid/semi-rigid component invented path: `TP-R4-D3-RIGIDVIS-001`;
- expansion-joint invented path, user-stiffness macro-element, and load-side
  pressure-thrust evidence: `TP-R4-D4-EJSTIFF-001`,
  `TP-R4-D4-EJMACRO-001`, and `TP-R4-D4-EJTHRUST-001`;
- minimal spring-hanger user-data path under `DEC-049`:
  `TP-R4-D5-HANGERDATA-001`;
- rendered-report component provenance path:
  `TP-R4-D8-COMPPROVREPORT-001`.

R4 nonlinear-support evidence is landed for bounded current surfaces:

- assembled dense loop and product-preview sidecar under `DEC-044`/`DEC-046`:
  `TP-R4-D6-LOOPCORE-001` and `TP-R4-D6-PHYSINTEG-001`;
- explicit and derived friction normal evidence, product live-loop coverage,
  branch-assembly benchmark, convergence observations, active-set policy,
  force/moment residual policy, work residual policy, general-energy policy,
  and displacement/reaction-delta policy for current seed/product/fixture
  surfaces;
- thirteen accepted public-original non-seed multi-support companion fixtures
  carrying active-set, free-DOF force/moment, free-DOF work, general-energy,
  and displacement/reaction-delta policies;
- sparse evidence lane and generated-grid sparse suitability thresholds under
  `DEC-050`, with dense still default.

The current R4 gap packet remains
`plans/VERIFICATION_2026-06-21_r4_exit_gap.md`; it records **not ready**, not
closure.

## 3. Residuals Requiring Human Disposition

The following residuals are explicitly recorded in the active coordination and
completion surfaces:

1. Non-seed force/displacement threshold promotion beyond the accepted
   thirteen-fixture multi-support set.
2. Broader displacement/reaction-delta thresholds beyond the accepted
   current-seed, thirteen-fixture, and product-preview surfaces.
3. Broader energy thresholds outside the accepted current-seed, thirteen-fixture,
   and product-preview surfaces.
4. Broader multi-DOF / multi-support nonlinear acceptance thresholds beyond the
   accepted thirteen-fixture set.
5. Sparse default promotion plus sparse timing, memory/allocator/RSS, CI,
   hardware-normalized, and conditioning evidence beyond the bounded
   generated-grid pivot-ratio policy.
6. Deeper spring-hanger catalog sizing or constant-effort solve behavior beyond
   the minimal user-entered model ruled by `DEC-049`.
7. External validation threshold evidence.
8. The final R4 exit-chain verification packet and human R4 exit ruling.

## 4. Options

| ID | Option | Consequence |
|---|---|---|
| **O-A** | **Strict R4 closure: every listed residual must be closed before R4 exit review.** | R4 remains open until sparse default/timing/memory/CI/hardware-normalized/remaining-conditioning, external validation thresholds, broader nonlinear thresholds, and deeper spring-hanger behavior are implemented and evidenced. This maximizes rigor but likely pulls R5/release-style evidence into R4. |
| **O-B** | **Bounded R4 exit envelope: current seed/product/thirteen-fixture nonlinear evidence and current invented component-provenance evidence can be sufficient for R4 exit review if the final exit-chain packet is green and all residuals are explicitly deferred.** | R4 exit review can proceed after a successor `VERIFICATION_<date>_r4_exit_chain.md` proves the bounded PRD §22.5 evidence, records full validation, and lists sparse default, external validation, deeper spring-hanger behavior, broader thresholds, and release-quality axes as non-R4 blockers. No release or professional claim is created. |
| **O-C** | **Targeted additional R4 blockers: name a smaller must-close subset, then defer the rest.** | The human names which residuals are R4 blockers, for example one more nonlinear threshold surface, a spring-hanger sufficiency statement, or a sparse threshold subset. Agents close only that subset before preparing the exit-chain packet; all other residuals remain explicit post-R4/R5 work. |

## 5. Advisory Recommended Disposition (PROPOSAL)

Recommend **O-B**.

Rationale: PRD §22.5 is narrow. It requires nonlinear support validation cases
to converge and component provenance to appear in reports. The already-landed
Phase D evidence demonstrates those properties for current invented,
public-original, bounded surfaces, while the remaining residuals are largely
scale, release, external-reproducibility, or beyond-current-envelope hardening
axes. Those axes matter, but they align more naturally with the R5 engineering
beta and release-quality gates unless the human project authority wants R4 to
absorb them.

O-B does not declare R4 complete. It authorizes preparing a final R4 exit-chain
packet only after current bounded evidence is revalidated, residuals are
explicitly named, and the human project authority separately rules the R4 exit.

## 6. Proposed Follow-Up On Ruling

If **O-A** is ruled:

- keep `plans/VERIFICATION_2026-06-21_r4_exit_gap.md` as the active R4 state;
- select the next implementation tranche from the listed residuals until all
  are closed;
- do not prepare a final R4 exit-chain packet until those residuals are
  evidenced.

If **O-B** is ruled:

- prepare a successor `VERIFICATION_<date>_r4_exit_chain.md` for human R4 exit
  review;
- run the DEC-025 evidence sweep on a clean committed head for that packet;
- record the residuals above as explicit post-R4/R5 or non-blocking hardening
  items;
- update coordination only after the human separately approves the R4 exit.

If **O-C** is ruled:

- update the completion plan and coordination state to name the human-selected
  must-close subset;
- execute that subset as ordinary bounded R4 work;
- then prepare the R4 exit-chain packet.

## 7. Human Ruling And Disposition

Human project authority ruled on 2026-06-22:

> regarding `D-25` my ruling is `O-B` R4 can move to a final exit-chain
> verification packet.

Recorded ruling: `DEC-052` in
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12.

Disposition:

- The current bounded seed/product/thirteen-fixture nonlinear evidence and
  current invented component-provenance evidence may be assembled into
  `plans/VERIFICATION_2026-06-22_r4_exit_chain.md` for human R4 exit review.
- The residuals listed in §3 are not blockers to preparing that packet under
  this ruling. They remain explicit post-R4/R5 or non-blocking hardening
  residuals unless a later human gate selects one as blocking.
- R4 exit itself remains a separate human review/approval step. This ruling
  does not approve R4 exit, advance the target stage to R5, issue deliverables,
  create release readiness, professional approval, certification, sealing,
  authentication, or code-compliance acceptance.

## 8. Ruling Mechanism

Per existing practice, the human project authority selects an option or rules
directly. The ruling is appended to
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12 as the next `DEC` entry
citing this packet. This register row moved from `AWAITING_RULING` to `RULED`
with the `DEC-052` decision pointer.
