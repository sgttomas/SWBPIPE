# Proposed production seams and bounded validation

All paths below are repository-relative. P=projects/chirality-piping;
PP=P/core/product_physics; FK=P/core/solver/frame_kernel. Function locations refer
to frozen `8104a4fedd`. These are proposed implementation impacts, not a write
grant from this TASK. No maintained file was edited.

## One concrete dataflow

1. **Preserve entry and selection.** Run ordinary processing and its protected
   formation demotion as today. Actual capture authenticates invocation, normalized
   model, material selection, case and completed observations. Typed/no-capture
   requests stay ordinary. Exact-block selection suppresses all new W1 source,
   numeric and draft work and retains original bytes. The original built model
   remains the fallback and historical observed basis.
2. **Prepare a real retained section.** From the captured normalized D and actual
   effective-wall t, use B1C's geometry with existing Wide<16>/1024 outward helpers.
   Prepare A/I/J/Z/c once per member. For A/I/J/Z, require both rigorous interval
   endpoints to have identical finite RN64 conversions. That proves the rounded
   source property; no pi-series runtime or increasing precision is needed.
   Preserve existing validity and primitive-admission gates. Ambiguous rounding,
   range or arithmetic failure declines the candidate with spent evidence; it
   does not change model admission, typed behavior or availability obligations.
3. **Construct a new, owned K.** PP builds a RetainedPreparedCase from the actual
   preparation records, with a new PrimitiveSource and ProductMemberFacts using
   those same new properties. E/G, coordinates, y_reference, constraints, spring
   identities and individual load ledger remain identical to capture. Compute
   operational A/Z/L/ka/kt from this actual prepared producer. Bind original
   normalized source → preparation input/output → actual new member → actual
   source encoding → RecordedInvocation/run/RetainedSolve. The original built
   property bits are recorded as old; do not weaken their old identity check to
   pretend they equal the new member. A retained-preparation tag/version belongs
   in private provenance and deterministic replay evidence.
4. **Keep native admission; tighten both product readouts.** Invoke the unchanged
   retained solver schedule and R7/A1 admission on K_new. With that owner's
   accepted inverse-bound/verification view, evaluate exactly two **closed**
   residual laws sequentially:
   - AdmittedK: coefficient products (E_K A_K, G_K J_K, E_K I_zK, E_K I_yK)
     lifted exactly (at most 106 bits). Differences from K are exactly zero;
     source frame/load/constraint/spring maps are the same. The existing residual
     bound specializes to alpha=0, beta=2B and error <= beta*||rho|| in the
     accepted scaled coordinates. Exact frame interval formation still applies;
     zero coefficient difference is not an assertion of zero computed residual.
   - AnnularSource: current normalized geometry/material law and the accepted
     strict alpha<1 perturbation/residual theorem, with full recovery-functional
     change. Keep selected independent E/G and actual resolved coefficient cover.
   Each uses the existing arbitrary finite center, at most one cached-factor
   correction and a fresh rigorous residual. Approximate correction accuracy is
   not a premise: the residual afterwards proves the bound. There is no new
   factorization, matrix callback, general constitutive API or retry loop.
   Native published x/radius checks remain required. The new K readout enclosure
   replaces only the unnecessarily wide symmetric x±r representation inside
   the **product** proof; it still proves exactly q_Knew.
5. **Produce complete rows.** Feed each lane through the same identified B1/B2
   native/support/stress recipes, including represented Z alternatives. Hull the
   two justified readouts. For primary/component-stress rows, form a single
   1024-bit midpoint and one raw-unit projection, then RN64 output. The exact
   certificate measures that actual y; correct rounding of an ideal midpoint is
   not required for soundness. No neighboring-value search is proposed. Keep
   prescribed values exact. Support magnitudes use their own norm readouts and
   must also pass the existing published-component guard.
6. **Regenerate dependent observables from the candidate.** Run existing span
   statics/coefficient maximum construction on the candidate endpoint rows and
   new actual section, preserving its binary64 coefficient scope, bounds,
   midpoint and location. Do not replace this observable maximum by a free
   midpoint of the physical hull. Check its actual returned row against the
   physical dual-readout maximum interval separately. Rebuild headlines as
   aliases, support attribution and complete maximum coverage. Copy only actual
   captured ancillary observations with their unchanged exact metadata/text;
   they describe those observations, not new mechanical error estimates.
7. **Freeze, classify and commit atomically.** After the whole final candidate is
   immutable, rebuild scales/classes and run unchanged final_case gate, G5a,
   observables, metadata/coverage and invocation/case ownership. Produce receipt
   and method identity only after complete success in the later F2a transaction.
   A failure keeps ordinary fallback, named reason and all spent work. No failed
   covered row becomes NotCovered, and no refusal counts as publication.

This projection has a noncircular order: physical intervals depend on the owned
source/solve, proposed values depend on those intervals, final scales depend on
the complete proposed values, and acceptance is evaluated last. No scale is
chosen to make a failed projection pass.

## Likely exact file seams

| File / current seam | Proposed impact |
|---|---|
| PP/src/retained_product.rs:1063 case_source; 1158–1273 member build/facts/operational | Split original capture validation from authentic retained preparation; own actual new member facts and source. Preserve old ordinary built checks and support/observation custody. |
| PP/src/retained_product.rs:1435 finish; 1521 bind_rows; 1754 observables; 2442 g5a | Stage complete replacement values and dependent evidence before immutable final binding; keep exact G5a inputs tied to new source. Source-correction work now has two identified lanes. |
| PP/src/lib.rs:2231/3361 observer entries; 3476 source hook; 3914 observation hook | Connect the selected new producer through the existing captured-only transaction. Do not change ordinary formation/solver calls or the observer=None path. Future public activation remains its own gate. |
| FK/src/structural/retained/product_certificate.rs:512 build_member and geometry at 521–539 | Factor the existing finite annulus calculation into a narrow section-preparation helper; counted RN64 endpoint conversion/equality result. No arbitrary source callback or new arithmetic format. |
| FK/src/structural/retained/product_certificate/source_residual.rs:224 coefficients; 254 member; 631 source_residual; 651 run; 774 radius; 899 recover | Add private closed AdmittedK/AnnularSource selection; exact admitted coefficient specialization; two separately owned Spent returns. Preserve frame and support laws, limits, error precedence and post-correction residual proof. |
| FK/src/structural/retained/product_certificate/final_case.rs:540 recipe; 676 gate; 858 run_case | Replace product K x±r interval construction with authenticated K residual readouts; retain native A1 admission. Expose narrow owner-bound preparation/projection data needed by PP; reuse gates and final scales unchanged. |
| FK/src/structural.rs:10 retained_api; FK/src/structural/retained/origins.rs:786 RecordedInvocation product methods | Only necessary closed records/methods cross the crate. No public serializable radius, raw rational, arbitrary value/radius constructor or source-agnostic closure. Preserve the actual run/solve owner identity check. |
| PP/src/preview_physics.rs and existing extrema producer callsites in PP/src/lib.rs | Reuse actual maximum/coefficient routines for the candidate; any required seam refactor needs independent review. Their meanings/reader labels remain unchanged. |
| PP retained_product_tests; FK retained_k4 product/source-residual tests | New source/provenance/rounding controls, tight K lane, full both-mode actual run, and negative mutations. Preserve the fixture, formation assertions and all protected expectations. |

Every FK row above is a **retained/** impact requiring candidate review. The
new public retained publication, receipt/source-digest and reader/replay bytes
will change when activation is implemented; this is an explicit public-contract
integration impact, not a new semantic identity selected by this packet. Exact
profile geometry/material routes and historical identities remain unchanged.

## Local work and storage — part of the implementation, not deferred away

Preparation is one fixed per-member geometry schedule. Reusing current positive
interval operations gives 19 directed scalar arithmetic entries before conversion:
four subtractions; eleven multiplications; two additions; two divisions. Exact
power-of-two shifts, lifts, positivity/range/equality checks and eight binary64
endpoint conversions are additional charged operations. Use a documented named
scratch layout of at most 32 Endpoint slots plus the existing scalar context/sum
scratch; count actual helper temporaries and capacities when implementing. This
is a proposed bound, not a measured sizeof or stack assertion. The retained
preparation scalar payload is seven f64 fields per member (D,t,A,I,J,Z,c), plus
actual map/provenance and work records; no full copied source tree is needed.

The native solver count and p/2p schedule stay unchanged. The residual design
adds **one** bounded correction lane relative to today's one source lane:
at most two correction calls total for a loaded case, each reusing the same
owner's verification factor. Charge both residual/recovery passes, both correction
triangular solves, exact/directed arithmetic, every entered initialization/map/
comparison/conversion/row visit, and failed prefixes. Do not report only the
source lane or call the admitted lane free because delta=0. Existing auxiliary
library/serde/native lookup work remains explicitly unqualified until measured.

Run the two lanes sequentially. Retain K rows while source scratch is live, then
retain the two Q_native enclosure vectors during projection, as today's source
rows plus represented intervals already do. Drop each lane's center/rho/correction
scratch before the other begins; store both compact work results and actual
capacities. Stage at most Q_final replacement numeric values plus M maximum
evidence records and required row-owner maps. Do not retain per-row ExactWideSum,
verification reports or general rationals. Original ordinary envelope, new source,
selected solve, staged values/evidence and later transaction/caller serialization
can overlap and all must enter the resource account.

Checked counts/layouts precede reservations. Entered work precedes initialization;
record successful capacity immediately even if later work fails. Preserve spent
numeric cause and accounting fault precedence, including at first/second lane,
projection, maximum and final bind. No new LME tariff, 20B/60B relaxation, allocation
ceiling or admission policy is invented. Full M1/native/RSS/caller qualification
remains required, but these local counters/capacity obligations cannot wait for it.

## Decisive controls and live witness

1. Independently re-derive geometry endpoint rounding, new source identity,
   closed K residual specialization and all exact control rows before ROOT selection.
2. First live same-request run: both modes, complete 98/99 row census, all 97
   mechanical predicates per mode, same source/material/support/station identity,
   actual projection/evidence, actual G5a and owner-bound residual widths. Compare
   all values against the independent analytical intervals. Observe actual p;
   recompute p512 floors if applicable. Preserve the first failed prefix too.
3. Mutation controls: restore old J while claiming preparation; relabel old bytes
   as new; mix new K with old facts/Z/ka/kt; drop one readout; foreign case/invocation
   or selected material; E/nu substitution; unrounded wall subtraction; wrong
   endpoint sign; native contributor substituted for a support slot; missing
   component/observation; copied old extrema/headline; changed raw unit; stale
   scale/class; negative zero at zero resolution; first/second-lane work overflow
   and allocation failure after earlier successful capacity. Each must fail its
   intended identity, coverage, observable, arithmetic or numeric gate.
4. Preserve typed both-mode Sensitive result and ordinary fallback bytes; existing
   exact-block selections byte-identical with no W1 attempt; pressure refusal
   remains NUMERICAL_INTEGRITY_UNRESOLVED and its no-pressure control publishes.
5. After private success, carry public F2a/S-G1 through actual receipt/source
   replay and Rust/Python/TS G0–G8, no-Passed-breach both-entry checks, protected
   references, resource evidence and native successor Current witness. Native
   evidence and independent practitioner/owner holds remain separate. Review the
   actual full candidate, including the separately active I50 accounting repair.

The analytical witness does not predict the future residual enclosure width,
selected p, G5a evidence or resource success. A failure at one of those concrete
gates returns its actual inputs and mechanism; it does not authorize a new
readout choice, fixture search, silent source restriction or publishing refusal.
