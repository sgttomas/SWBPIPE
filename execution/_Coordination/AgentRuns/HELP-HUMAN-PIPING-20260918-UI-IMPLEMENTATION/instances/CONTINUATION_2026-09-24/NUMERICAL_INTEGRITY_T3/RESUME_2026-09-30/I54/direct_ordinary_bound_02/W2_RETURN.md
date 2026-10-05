# W2 return and unscaling — closed source composition

Pinned `24af17c470`. This closes the W2-specific return/overlap residual from checkpoint B. It reuses the ordinary typed-core, report, formatting and layout parameters already required by BOUND; it does not select a scale, run a solve, or assign unknown layouts. All expressions below exclude the actual outer raw/model/basis/force roots, which remain in the ordinary phase baseline exactly once.

Let N,F,m,s,k,l,C,Z,d and identifier lengths be BOUND's facts. Let `A(T,h)` be the reviewed append upper, `V(T,h)` the reviewed push upper, and `X(T,h)=s(T)*h`. All arithmetic/layout operations are checked. `I_l` and `I_m` are total nodal-source and member-ID byte lengths. Let `L_sym`, `L_audit_error` and `L_formation_detail` bound the existing report/optional error Strings; these are the common text-grammar inputs, not new runtime oracles.

## Source sequence and actual overlap

1. PP/lib.rs:1412–1460 owns a fresh spring tuple vector and calls `solve_with_force_scaling`. SA:1966–1996 evaluates b=0 and then at most one chosen b. Failed evaluation locals drop before the second; there is no third attempt. `ForceScaleCensus` is scalar state, not a new heap census.
2. Every evaluation creates a fresh SparseStiffness. At nonzero b, FK sparse.rs:595–612 owns scaled frame/spring primitive Vecs while recursively assembling; SA:534–554 creates another scaled primitive generation while constructing evidence. These are sequential source generations, not one shared cache allocation. Pattern evidence and a scaled force remain live through typed solve and unscaling. No legacy DEC050/053 profile observation runs inside this W2 evaluator.
3. `AssembledForce::force_scaled` (load_ledger.rs:330–371) clones source Strings, by_dof outer/inner buffers and allocates fresh values/None formation slots. This scaled force drops when the force-scaled solver entry returns. A **second generation** is later created in SparseStiffness::force_scaled_reactions (sparse.rs:416–444) while outcome stiffness and StructuralSolution remain live.
4. `unscale_structural_solution` (structural.rs:2613–2677) retains the old report while it creates a replacement exponent vector, then replaces one accumulated/difference expansion at a time. It pushes at most 6F RecordOutcome records: three fields per residual and intended-residual row. RecordOutcome contains a static record name and scalar fields, no owned String. A nonzero-scale NegativeEnergy error can allocate a new N-direction while the original survives (:2696–2714).
5. Publication (PP:1590–1673) retains the complete outcome. Reaction computation additionally holds the second scaled force and an exact N-vector from SparseStiffness::multiply. The returned PublishedValue vector is collected into a new `(usize,PublishedValue)` vector by zipping it with the cloned/sorted DOFs; both original input Vec backings coexist with the new tuple Vec. Springs and member actions are then collected, followed by member-ID copies, spring DOFs and a **clone of the outcome's record vector**.
6. PP:1487–1516 constructs PreviewLinearSolve by cloning report/load-fidelity/formation while the original outcome and complete publication object still exist. Its free-DOF vector and new F-solution are additional. Returning moves publication and the new PreviewLinearSolve; it does not retain a second copy of every W2 attempt.

## Evaluable functions

A scaled nodal-force generation has upper

    ScaledForce = 8N + A(ForceTerm,l) + I_l
                  + X(Vec<usize>,N) + 8l
                  + X(Option<FormationRecord>,l)
                  + V(usize,min(l,N)).

`values` uses exact capacityN; by_dof.clone uses exact outer N and total inner length l; formations are exact l fresh None slots. The terms Result-collect and underflow list use the stated upper. For moving storage add at most their old terms/underflow backings; neither the cloned inner arrays nor exact values vector need geometric-growth additions.

Let `KBuild`, `EvidenceBuild`, `Core_mode` be the source uppers from BOUND, with Core_mode covering **only the typed structural core**, including H_formation128 and output children, not the separate legacy observation. Define

    ScaledPrimitives = A(FrameElement,m) + A((usize,f64),s)
    EvaluateW2 <= 2*(KBuild + EvidenceBuild + Core_mode + ScaledForce
                     + 2*ScaledPrimitives + N + A(usize,F)).

This is the permitted conservative sum of the two possible evaluation maxima. Some addends are absent at b=0; keeping them is an explicit upper. The original outer basis stiffness and force are not part of EvaluateW2 and still belong in the caller baseline. Source-scaled arrays do not change m,s,N,F or the primitive/formulation scope.

The extra unscaling storage beyond the already owned StructuralSolution is

    UnscaleExtra_req <= V(RecordOutcome,6F) + 4F + 8*(d+1).

`4F` is the new scale_exponents Vec while the old one remains. Only one expansion is being replaced at a time, with length<=d+1; the original report's expansion children are already present. For moving growth add one old RecordOutcome backing, bounded by V(RecordOutcome,6F). A separate negative-direction error path has at most 16N bytes for old+new direction. Those error fields move otherwise; scalar rescaling does not allocate.

For a reusable report upper, define

    Report = 4F + X(PivotEvidence,F) + 2*V(ResidualRow,F)
             + X(ContributionRounding,Z) + 8*(2C+Z) + L_sym
    Fidelity = V(LoadFidelityRow,N) + l*s(String) + I_l + L_audit_error
    Detail = L_formation_detail.

These upper-bound both the output and a deep clone. A clone often uses fewer slots than this upper. No original report is discarded early merely because it will be cloned into the returned PP record.

The final publication backings have upper

    Publication = X((usize,PublishedValue),k) + A(PublishedValue,s)
                  + A([PublishedValue;12],m)
                  + X(String,m) + I_m + 8s + X(RecordOutcome,6F).

During reaction preparation, additionally retain `ScaledForce + 8N + 8k + A(PublishedValue,k)`. The internal product vector uses Vec::with_capacity(N) (sparse.rs:325–356). The old DOF and PublishedValue vectors are real input owners during the zip collect, not realloc shadows of the new tuple Vec.

`force_scaling_admission` owns a fresh HashSet<&str> over at most l authored nodal IDs (PP:1540–1545). Its no-element/no-constant-effort checks do not allocate a new nonlinear solve. Let `AuthoredSet=Hash((&str,()),l)` with its reviewed layout.

The publication-failure quantity formatting is also bounded: for maximum node/member identifier length I, `label_len<=max(I+3,31)` (the fallback is global_dof plus at most 20 digits), and `quantity_len<=max(14+label_len,12+I,8)`. The label and outer quantity Strings can coexist. Each uses the reviewed format/append law `cap<=max(8,2*len)`; taking their two moving pairs is a conservative source upper. This covers eager quantity construction even if the error-mapping closure later discards it. It does not price later general diagnostic formatting, which remains the common ErrorPrefix term.

A source-proved conservative **return-tail upper** is therefore

    Tail_req = K(N,Z) + 8N + 2*Report + 2*Fidelity + 2*Detail
               + V(RecordOutcome,6F) + Publication
               + ScaledForce + 8N + 8k + A(PublishedValue,k)
               + A(usize,F) + 8F + AuthoredSet + QuantityFormat_req.

This deliberately sums the reaction-preparation and final-clone maxima; it does not claim they are all simultaneous. For Tail_mov, add old growable backings of reaction/spring/end-action Result-collects, free-DOF collection and ScaledForce's growable fields, and the quantity-format moving additions. Fresh exact clones already coexist with their original owners in Tail_req; do not count a third old copy. A simple valid addition is

    Tail_mov - Tail_req <= A(PublishedValue,k)+A(PublishedValue,s)
                          + A([PublishedValue;12],m)+A(usize,F)
                          + A(ForceTerm,l)+V(usize,min(l,N))
                          + QuantityFormat_req.

The W2 branch can thus be composed as the PP spring-argument vector plus EvaluateW2, UnscaleExtra, Tail and the separate direction-error upper. It introduces **no remaining unbounded W2-specific owner/iteration count** under this family's source premises. Final type layouts, H_formation128 and common text/error grammar are still explicit pre-existing dependencies; this is not a complete admitted profile.

## Named first-case substitution

N=12,F=9,m=1,s=k=l=3,C=147,Z=144,d=2,I_l=18,I_m=2, maximum node/member ID length 2. Use the safe format fallback bound even though successful source validation supplies real IDs: QuantityFormat_req<=62+90=152 bytes and moving<=304.

- `ScaledForce = 170 + 6*s(ForceTerm) + 3*s(Option<FormationRecord>) + 12*s(Vec<usize>)`.
- Outcome records use at most 64 slots; publication record clone at most 54: **118*s(RecordOutcome)** in the tail.
- Reaction input, spring and end-action Vec upper capacities contribute **60*s(PublishedValue)**; the final reaction tuples add **3*s((usize,PublishedValue))**.
- New exponent/expansion replacement storage is36+24=60 bytes, plus the outcome RecordOutcome Vec; old/new error direction is192 bytes.
- After expanding Report/Fidelity, Tail_req is

      12608
      +18*s(PivotEvidence)+64*s(ResidualRow)+288*s(ContributionRounding)
      +32*s(LoadFidelityRow)+118*s(RecordOutcome)+60*s(PublishedValue)
      +3*s((usize,PublishedValue))+7*s(String)+6*s(ForceTerm)
      +3*s(Option<FormationRecord>)+12*s(Vec<usize>)
      +2*L_sym+2*L_audit_error+2*L_formation_detail+AuthoredSet.

No tuple, enum or private type size has been inserted from a guessed layout. The arithmetic script records these coefficients and a bounded check of the constant term. This is a substantially resolved W2 term, not a claim that W2 is selected for the named case, nor that any complete requested/moving total fits a chosen M.
