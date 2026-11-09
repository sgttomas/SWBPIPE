# F1 modal analysis: governing design

**Status.** This is DEL-04-09's governing design for F1, modal analysis (natural frequencies and mode shapes). Scope entered by SCA-013 (`execution/_ScopeChange/SCA-013_2026-10-27_MODAL_ANALYSIS/DECISION.md`). Internal slices are built; public admission and every UI remain disabled (§5.3.9). The governing decisions are Agent 0's (HELP_HUMAN), made under the owner's delegation in `loop/LOOP_INIT.md` on 2026-10-27 and amended on 2026-10-31 for the dynamics lane's v1.0 plan (`docs/RELEASE_TARGET_v1.0.md` RT-A7, exit criterion 1). They include: the exact route on its own mass basis (D-r), nonlinear supports linearized at a named solved case (D-s), snubbers in the modal basis when requested (D-t), and the published-fixture review record (D-u); and participation on the uniform-support-motion excitation (D-v), from the DEL-04-10 review. This design makes no acceptance, issuance or lifecycle statement (F-PIP-3). Standard claim fence applies (claims taxonomy per DEC-081).

**Basis.** It was researched against main `0280fd4486`, together with the PF Timoshenko design `execution/PKG-05/DEL-05-03/Work/pf-timoshenko.md` (#1663, in review; not on main at that commit) and the feature survey `execution/_Coordination/PROPOSAL_2026-10-26_FEATURE_SURVEY.md` (#1657, open). Paths are relative to `projects/chirality-piping/` unless they start with `/`. "PP `lib.rs`" is `core/product_physics/src/lib.rs`.

**Labels.**
- **[code]**: a fact about existing code, with file:line.
- **[design]**: a fact about an existing design or record.
- **[decided]**: settled by this design or by Agent 0's decisions, with the reason.
- **[open]**: listed in §9, with its owner.

---

## Selected mass assurance: next modal unit after CE

**Selected rationale.** Retain REQ-005's model-derived mass assurance. Checking
totals and participation against a supplied M cannot detect a coherently wrong
M with matching totals; the independent model path must check the formation
as well. This serves RT-A7, RT-A8's uniform-support-motion/missing-mass basis
and the reference-system publication goal. Source implementation and preparation
may proceed in parallel with CE; the actual integrated S6, then CE/main
predecessor must be normally merged before final numerical/commitment review
and integration. CE keeps integration priority, with no resource-priority
change. WORKING_ITEMS owns this next modal unit after CE; this source design
supplies neither numerical-review clearance nor admission.

**Source basis.** Traced at `a5eabb649ca748c07522b634ca792bc72f0e1b80`:
`ScopeOfWork.md` REQ-001/REQ-005/REQ-006; `core/product_physics/src/modal/`
(`prepare`, `context`, `mass`, `weight_basis`, `stiffness`, `skew_pencil`,
`outcome`, `document`); product `self_weight.rs` (`stable_mass_per_length`,
`component_weights`), `rigid_component.rs` and `joint.rs`; kernel
`core/solver/modal_eigen/src/reformation/mass.rs`;
Rust `core/reporting/modal_result/src/lib.rs`
(`mr_binding`, `mr_mass`, `mr_participation`) and Python
`tests/verifiers/modal_evidence.py`.
The readers currently bind hashes/options to an optional supplied request and
rederive relations from the payload only. A model hash binds bytes; it does not
establish the mass implied by those bytes.

### Implementation boundary

- **Inputs and independence.** Each reader independently interprets the exact
  supplied request's model, materials and options after binding succeeds:
  model units, shared-section binding, route, node order/geometry, component
  mapping and named state context. Follow preparation's
  material precedence (nonempty request materials, otherwise model materials)
  and the authored-to-lowered case binding; do not trust a producer-authored
  mass recipe or normalized leaves as an independent source. Reuse checked
  canonical JSON, SHA-256 and the unit catalog/conversions; do not call/import
  product preparation, self-weight/eligibility, mass, kernel mass re-formation
  or the other reader. Rust remains a leaf, without a product/solver dependency;
  Python retains its producer-import independence check. A shared formula
  specification is the contract, not shared mass-executing code.
- **Family and section basis.** Independently enumerate pipes except objective
  connectors' replaced spans. Tee legs, geometry-only bends and annotation-only
  expansion joints retain pipe mass; realized arcs use their actual radius,
  orientation and two rigid half-arcs (§2.4), not chord mass. For an eligible
  rigid-family body (valve, flange, reducer, rigid or specialty), replace its
  mapped member's distributed mass by W/a at the component node and global
  CoG offset. Reproduce the eligibility/fallback
  distinction of `component_weights`: mapping conflicts/range failures block;
  `not_solver_consumed` or an unmet weight/CoG/consumption condition retains
  pipe mass with the existing disclosure where applicable. Do not silently add
  body and pipe mass. Objective connectors, tie rods and supports are massless.
  PF metal and bore use t − c_m; EX metal uses t and its contents bore uses
  (t − c_m) − c_a, with explicit c_a. Independently form metal, contents and
  insulation contributions from authored densities and source areas (§2.1).
  Missing required density or half an insulation pair is not a zero default.
  Form j̄ and ī = j̄/2 from metal and insulation only; contents contribute to
  translations, not section torsional/rotary inertia (D-e). Bodies use the
  required positive request a, including the existing applied-weight
  acceleration agreement/range premises; intrinsic CoG inertia stays zero
  because it is unentered. No concentrated-mass or W-17 field is introduced.
- **Quantities.** Independently form the full symmetric global M, including
  all straight consistent/Rayleigh terms, arc first/second moments, body
  offset translation–rotation coupling and rotary blocks (§2.2–§2.4). Compare
  the union of expected and published entries, including missing/extra entries
  and zeros, in physical DOF order; checking scalar totals alone is insufficient.
  Derive T_k = R_kᵀM R_k on all DOFs, the mass centre from its first moments,
  and the free-DOF mass R_f,kᵀM_ff R_f,k. Recompute m_i = φ_iᵀM_ffφ_i and
  p_k = (M R_k)_f, then Γ, m_eff, fractions and cumulative fractions from this
  independently derived mass and the published shapes, alongside the existing
  payload relations. Retain restrained-point coupling in p_k (D-v).
  Category contributions are internal cross-checks; modal-1 publishes three
  directional total/free rows and a centre, not a category table. CLM-001's
  category-publication wording remains unmet; no format change closes it here.
- **Restraints and state.** Independently reconstruct axis restraints, imposed
  motion's restrained DOFs (motion values do not enter M), springs' free DOFs
  and the requested dynamic-only snubber choice. For a nonlinear case, bind
  every reported support to the model and named case, check its state-to-
  treatment rule (§1.3), and derive the augmented partition from those states.
  The current payload does not independently establish that these are the
  converged, confirmed states of the static solve. Report that dependency
  explicitly; a conditional partition check is not independent state
  qualification or unconditional model-derived free-mass assurance. Do not
  run a second static solve. Global M remains independently checkable where
  its section basis resolves. Mass follows named-state contents when the
  shared contents consumer lands; at this source, condition contents are
  refused, never substituted with section contents. Reconcile against the
  integrated S6/CE head before final review; neither integration alone
  establishes gap qualification or modal admission. Modal-1 0.1.0 refuses skew
  records before solved projection (`MODAL_DOCUMENT_SKEW_RESTRAINT_UNSUPPORTED`).
  Keep that refusal: its internal reduced pencil uses Q̃ᵀM Q̃ and kept-subspace
  mass, which the global-coordinate document cannot describe.

### Verdict, verification and delivery seams

Use the existing mass/participation standings to make the model-derived check
mandatory, preserving the payload checks. No request, withheld payload or an
uncovered model/state basis is **scoped unsupported**, explicitly naming the
missing input or coverage; it cannot pass mandatory mass assurance or yield
`fully_validated`. A binding mismatch, a solved result in an explicitly refused
producer domain, missing/duplicate subject, or established mass contradiction
**fails**. A comparison undecidable on sound rounding/profile bounds is
**unresolved**, never passed; evaluate decidable contradictions even beside
unavailable operands. Refused documents retain `refusal_consistent` as a check
of the refusal record, not proof of the refusal's engineering cause. Preserve
all current numerical thresholds and per-value flags. Derive the new formation
comparison's roundoff coverage from its actual arithmetic; do not reuse the
self-weight identity tolerance for matrix entries, assume exact equality after
independent evaluation, or select a new acceptance value here. An uncovered
rounding case remains unresolved pending review.

The targeted threat is coherent producer mass-formation error on the bound
discrete inputs, including errors invisible to a·M·R_k. Maintained Rust/Python
mutations must leave the authored request fixed while resealing payload rows
and updating all dependent mass/participation values: wrong route/wall/bore or
density, omitted insulation/contents, omitted/double-counted/remapped body or
arc, wrong CoG/node/orientation, dropped restrained coupling, wrong restraint
partition, and rotary/Rayleigh/torsional changes that preserve M R_k. Both
readers must reject an established model contradiction even when old payload
relations pass. Include intact PF/EX, straight/tee/arc/body, alternate units
and section references, both normalization cases, named-state/refusal and
missing-input/rounding controls. Use maintained `f1_modal` mass references as
independent controls without changing frozen values. This verifies entered
model mass, not omitted real-world equipment/contents, the discretization
error R3, physical correctness, K, residual/re-formation evidence or eigenpairs.

WORKING_ITEMS owns the reader implementations in
`core/reporting/modal_result/src/` and `tests/verifiers/`, with regressions in
`core/reporting/modal_result/tests/`, `tests/test_modal_evidence.py` and
`fixtures/results/`; `modal/tests_document.rs` binds unchanged producer bytes.
Consult product `prepare/context/state`, `mass/weight_basis`, `stiffness/
skew_pencil` and `outcome/document` as the input/formation/projection seams,
not independent authorities. No producer algorithm, model/result schema,
reference or admission change is authorized by this design. If required
state/format evidence is absent, return the explicit coverage gap to Agent 0
before expanding those seams. Reader work/allocations and input normalization
join the existing projection/reader resource and format limits; use bounded
input counts and fail before unbudgeted work. Reassess affected retained pricing
and actual-source D3 if priced source, dependencies or a retained consumer moves;
no values or budget weights are selected here. Wiring into native custody,
recording/store policy and common-M Bessel evidence remain separate open work;
the subject census is built (§5.3.2). Targeted numerical regressions and exact old/new
independent scrutiny are required for the final scoped unit before integration.

**Implemented.** `core/reporting/modal_result/src/model_mass.rs` and
`tests/verifiers/modal_model_mass.py` (with `modal_model_mass_interval.py`)
implement this boundary independently; REQ-005's closing sentence states the
contract. Each derived quantity is an enclosure: the Design formula on the
authored binary64 inputs, widened by the producer's arithmetic. The one stated
premise is the kernel's double-word path (`dd.rs`; Joldes, Muller and Popescu
2017 for add/mul, the kernel's own bound for division, square root and series):
under 2⁻¹⁰⁰ per operation and under 2¹⁰ operations per contribution, so a value
accumulated from n contributions errs by at most 2⁻⁹⁰·(n + 1)·Σ(1 + κ)·|scale|,
each scale the absolute term sum of the producer's formation (for a member, of
its frame rotation, so an entry cancelling to zero keeps its allowance) and
κ = |y|/|y⊥| for a member frame. Where the readers sum turned blocks rather
than the producer's local ones, the scale bounds the local absolute sums through
the norms a rotation keeps (Σ|S_local| ≤ 2·Σ|S|, Σ|J_local| ≤ 3·Σ|J|). The
premise holds only inside two limits, beyond which an entry is outside the
rounding model and unresolved: (n + 1)·max(1 + κ) below 2³⁷, where the allowance
would reach binary64 resolution, and no nonzero double-word intermediate of the
element below the kernel's low-word floor 2⁻⁹⁶⁹ (`LOCAL_ENTRY_MIN`). Such an
entry leaves undecidable only what uses it; every other entry, total, the centre
and the participation quantities that do not use it are still compared. The
mass centre's path (`node_centroid`, `nodal_resultant`, the division by T_k) is
also plain double word, but its products are not compared one by one, so
instead of the floor it takes the arithmetic's underflow term: under gradual
underflow each binary64 operation errs by at most 2⁻¹⁰⁷⁵ more, absolutely, and
a subnormal sum is exact, so one double-word operation errs by its relative
bound plus under 2⁻¹⁰⁶⁴·max(1, 1/|divisor|). The participation path is the
kernel's extended-exponent `WideDd` and does not underflow. A present null
direction is refused as the producer refuses it. A
published value outside its enclosure fails; a flagged or outside-profile value
meeting it is unresolved. No threshold is selected or changed. A solved
document bound to a model the producer refuses (snubber, gap support, skew, a
support direction its admission refuses, condition contents on the state case,
Timoshenko, a body outside §2.3 item 6's range premise in the producer's
evaluation order) fails; an exactly axis-aligned direction is lowered to its
axis as the producer lowers it, rotational tokens included. The readers' boundaries are not identical: Rust's binary64 interval
arithmetic is wider than Python's exact rationals near a bound, and Rust scopes
an objective connector on the PF route where Python follows the producer's
mass selection (`replaced_span_ids`, exact route only). Neither re-derives the
component-weight acceleration agreement (§2.3 item 5) or the bend angle.
The shared corpus (`fixtures/results/modal_corpus.json`) carries
fixed-request wrong-M entries (`model:`) that only the formation comparison
rejects; entries pinning payload semantics on invented values, the sum-order
boundaries, the feasibility oracle and the producer edges are read without the
request, and with it the reader is never weaker than the oracle.

Public admission remains disabled under §5.3.9. The revised normalization
criterion distinguishes axis-aligned zero components from a skew physical
vector constraint; AC-006's nonzero skew-residual witness is still unmet.
The existing 14-group first-view frontier and six-term resource trace (3/6
closed) remain unchanged; this mass correction closes none of them. Resource,
recording and Bessel gaps, R3, gap qualification and all holds remain. H1, hanger/preload and the dormant Windows guest stay held. Neither
this design nor #164's three closed terms supplies the remaining proof.
[Original composition-00832 AC-004 observation](https://github.com/sgttomas/SWBPIPE/tree/dc037b5495ccd6fa35fb04758edbcdb019021bba/modal)
is retained for its historical source only. It authorizes no new witness run.

## 0. Scope

- **[design]** FR-024 (dynamic analysis modules) was explicitly deferred by DEC-078 (D-12, owner 2026-07-15). DEC-078 required that "any future dynamics scope re-enters only through a new governed scope change with its own human decision packet" (`execution/_Decomposition/SOFTWARE_DECOMP.md`, DEC-078).
- **[decided]** SCA-013 re-enters it for **modal analysis only**: PRD FR-SOL-016 and §12.9, scope item SOW-081, and this deliverable. Agent 0 decided it under the owner's delegation of 2026-10-27; the reason is recorded in the SCA-013 decision.
- **Out of scope:** response spectrum (F2), which SCA-014 re-entered and DEL-04-10 owns on this deliverable's modal basis (`execution/PKG-04/DEL-04-10/Work/response-spectrum.md`); time history (F3), harmonic analysis and every other dynamic module, which stay deferred under DEC-078.

---

## 1. What F1 solves

### 1.1 The problem

**[decided]** F1 solves the real symmetric-definite generalized eigenproblem

  K_ff φ = ω² M_ff φ

on the free degrees of freedom of the linear model, on the pressure-free (PF) route or the exact pressure (EX) route (D-r).
- K_ff is the assembled, restraint-reduced stiffness that the route's static solve forms. It is the case-independent part of the static K: thermal, pressure, wind and seismic loads do not enter it. [code] EX shares PF's assembly (`build_model_for_basis`, `assemble_basis_stiffness`, PP `lib.rs:4316`) with its own D-5 nominal-wall sections, and adds no load-dependent stiffness (`exact_admission.rs:117`: pressure stiffening "Not modelled").
- Nonlinear supports enter at the converged state of a named solved case (D-s), and snubbers when the request includes dynamic-only restraints (D-t).
- M_ff is a new mass matrix whose basis is the self-weight basis (§2).
- The case-dependent inputs are the modulus basis (§1.4) and, for a model with nonlinear supports, the named case whose converged support states linearize them (D-s).

### 1.2 Admitted, refused by name, unchanged

| Family | F1 (M1) | Why |
|---|---|---|
| Straight members, tee legs | **Admitted**, consistent mass (§2) | [code] Ordinary PF frames (PP `lib.rs:2457-2480`, `BuiltModel.frame_elements`). |
| Realized arcs (DEC-070 curved macro-element) | **Admitted**, mass lumped as two rigid-body half-arcs (§2.4) | [code] One 12×12 element per bend from Castigliano end flexibility (`core/solver/curved_bend/src/lib.rs:1-9`, `:300-318`). It has no interior interpolation, so no consistent mass is defined for it, and lumping is the honest choice. |
| Geometry-only bends | **Admitted** as their straight members | [design] Euler–Bernoulli under either shear model (`pf-timoshenko.md`, #1663). |
| W-15 rigid-family components (valve, flange, reducer, rigid, specialty) | **Admitted.** Stiffness s·E and s·G as today. Mass is the component body W/a at its centre of gravity (a from §2.3), replacing the member's pipe mass under the **same predicate** that self-weight uses | [code] `core/product_physics/src/rigid_component.rs:35-37`; `self_weight.rs:3-9`, `:548-650`. |
| Linear supports: anchors, guides, line stops, vertical supports | **Admitted**: restrained DOFs are eliminated from K and M | [code] Support families (PP `lib.rs:8645-8655`). |
| Springs, variable spring hangers | **Admitted**: diagonal spring stiffness | [code] PP `lib.rs:8596-8600`. |
| Constant-effort supports | **Admitted, disclosed**: zero stiffness and zero mass | [code] Already excluded from K (PP `lib.rs:8567-8569`) and applied only as a load (PP `lib.rs:13903`), so it contributes nothing to a linear eigenproblem. If removing it leaves a mechanism, the existing singularity refusal applies. |
| Objective connectors (expansion joints), tie rods | **Admitted, massless, disclosed** | [code] Linear elastic (`core/solver/frame_kernel/src/connector.rs:1-3`; `tie_rod.rs:1-3`, "bilateral linear elastic"). Self-weight applies no weight to them (`self_weight.rs:569` filters to the rigid family), so massless keeps the two bases identical. |
| Annotation-only expansion joints | **Admitted as pipe**, with the existing disclosure | [code] `joint.rs:5-6`. |
| Imposed displacement (W-16) | **Admitted**: the DOF is a rigid support DOF and stays restrained; the motion value does not enter | [design] PF imposed displacement acts only on rigid support DOFs, by partitioned elimination (DEL-04-08 `Work/retained-pricing-rules.md` §8.23). |
| Nonlinear supports (one-way, gaps, lift-off, friction) | **Admitted, linearized at the named state case, disclosed** (D-s, formerly M5): active or sticking, the support restrains its DOF with the stiffness the static solve uses in that state; inactive or sliding, its DOF is free. The state case is a required request input (`MODAL_STATE_CASE_REQUIRED`); an absent, unsolved or unconverged case is refused (`MODAL_STATE_CASE_UNAVAILABLE`) | [decided] The stiffness depends on contact state, so a modal result needs a stated state; the user names it, which is not a default. [code] The converged states survive a solve as `nonlinear_support_active_set_state_code` rows (PP `lib.rs:7499-7530`). The result names the case and each support's state. **As built (I2b-2):** admitted only at a confirmed, qualified, converged `dense_scrutiny` state of the same-run solve; **every model with a gap support is refused for now**, because the static path never confirms a gap state (§1.3, as built). |
| Exact route (`pressure_contract` present) | **Admitted** on its own section basis (D-5 nominal wall) and mass basis (`EXACT_MASS_METHOD`, §2.1), with every family EX itself admits (more as RT-A3 lands) | [decided] D-r: exit criterion 1 needs modal on EX. EX's stiffness is PF's assembly on its own sections, with no load-dependent term, and its self-weight basis gives the mass, so the identity of §2.5 holds on the route's own basis. Pressure is a load and is ignored, with EX's own disclosure that pressure stiffening is not modelled. |
| Load/reference-state documents | **Admitted through the named state case** (D-w): the request names a load case whose resolved state supplies the stiffness, each member's modulus selection included, the mass and the nonlinear-support states (§1.4) | [decided] Agent 0, 2026-10-31: criterion 1's reference system is necessarily such a document (RS-OPE's temperature and per-member modulus selection, RS-SAM's `boundary_motion`). [code] The static path resolves each case's per-member pairs with `case_state::resolve::resolve_case` and rebuilds stiffness from them (`build_model_for_members(…, Some(&resolved.pairs), …)`, PP `lib.rs:2968-3140`); modal uses the same resolver for the one named case. |
| Snubbers (model document 0.9.0, `SupportFamily` `snubber`) | **Admitted when the request includes dynamic-only restraints** (a required choice when the model has one, `MODAL_DYNAMIC_RESTRAINT_CHOICE_REQUIRED`): rigid in their translational DOFs or skew direction, or a spring of their stated stiffness; otherwise ignored. Disclosed either way | [decided] D-t: snubbers are dynamic-only restraints (SCA-014). A spectrum solve includes them (DEL-04-10). |
| Timoshenko straights (`model_settings.shear_model` on PF, `pressure_contract.shear_model` on EX, `timoshenko_annulus_energy_v1`) | **Refused by name on either route** (`MODAL_TIMOSHENKO_MASS_UNSUPPORTED`). **M4** admits them with a Timoshenko consistent mass | [design] PF Timoshenko has landed (#1663, model document 0.8.0). Agent 0, 2026-10-31: refused until M4; the v1.0 reference system uses Euler–Bernoulli members. [decided] Pairing a shear-flexible K with an Euler–Bernoulli mass would be inconsistent, so it is refused rather than mixed. |
| Retained route (route L/E, D1) | **Not entered** | [decided] F1 is a separate entry point with no retained-precision variant (§3.6). |
| Thermal, pressure-region, wind, seismic, nodal and member loads | **Ignored** by the eigenproblem, disclosed, not refused; the loads of the named case determine nonlinear-support states through its same-run solve (D-s) | [decided] |

**[decided]** An unrestrained model (rigid-body modes) is refused through the static path's singularity refusal. Kernel tests may use a negative shift for free-free references (§4.3); the product path does not.

### 1.3 Boundary conditions

- **[code]** Rigid restraints are eliminated by partitioned reduction (`core/solver/frame_kernel/src/lib.rs:1666-1800`; sparse `structural/sparse.rs:798`, `reduce_assembled_sparse_system`). Linear springs enter the global diagonal (`assemble_sparse_stiffness(…, springs, …)`, `sparse.rs:599-606`).
- **[decided]** M is reduced by the same partition: M_ff keeps the free rows and columns.
  - Mass at restrained DOFs, and the consistent-mass coupling M_fc, move with the supports: they enter the excitation p_k (§1.5, D-v) and the support reactions of a spectrum solve (DEL-04-10), but not the eigenproblem. The result states the total mass T_k and the effective-mass totals against it (§1.5).
- **[decided] Skewed restraints** (model document 0.9.0 admits `direction` on line stops, springs and snubbers). Modal uses the static path's adopted skew method, so the same subspace is restrained (DEL-05-02 Work `operating-case.md` §2.2, R-6 and R-7, Agent 0, 2026-10-27):
  - **Rigid:** the authored constraint dᵀu = 0 is kept exactly with a division-free nullspace basis, never a penalty and never a normalized d. With m the index of the largest |d_m| (ties to the lowest), T has the two columns d_m·e_j − d_j·e_m for j ≠ m, so dᵀT = 0 exactly. The node's translations are u = T·w, and K and M are reduced on that node as TᵀKT and TᵀMT, then lifted back by T. In exact arithmetic the eigenvalues equal those of an orthonormal formulation; T's rounding is the one the static path uses. The excitation p_k is reduced the same way.
  - **Finite stiffness k:** the rank-1 term k·d dᵀ/(dᵀd) with the authored d, added inside the node's 3×3 translational block, never a global diagonal term.
  - **Scaling and admission:** d is scaled by an exact power of two to max|d_i| ∈ [1, 2), and a direction whose scaling would be inexact is refused by name (`SUPPORT_DIRECTION_RANGE_UNSUPPORTED`), as statically. Modal admits a non-axis direction exactly when the static path does. Until the skew kernel lands (assigned to the solver lane), the static path refuses non-axis directions (`SUPPORT_DIRECTION_SKEW_UNSUPPORTED`), and modal refuses them with the same code.
- **[decided] Nonlinear supports at the named state case** (D-s, D-w). The modal entry solves the named state case (§1.4) on the current model and route in the same run; no stored result is reused, so the states are bound to the model being analysed. A solve that does not converge refuses (`MODAL_STATE_CASE_UNAVAILABLE`). Each nonlinear support then enters by its final state, as the static solver itself represents that state ([code] `prescribed_displacement_for_state`, `core/solver/nonlinear_integration/src/lib.rs:2128`: active one-way and lift-off supports, closed gaps and sticking friction are rigid displacement constraints, with no finite contact stiffness):

  | Behaviour | State | In the modal model |
  |---|---|---|
  | one-way, lift-off | active | rigid restraint of its DOF (its imposed value, if any, does not enter) |
  | one-way, lift-off | inactive | free |
  | gap | closed | rigid restraint of its DOF (the gap value does not enter) |
  | gap | open | free |
  | friction (tangential DOF) | sticking | rigid restraint of the tangential DOF |
  | friction (tangential DOF) | sliding | free, with the friction force frozen: it contributes no stiffness. This is a disclosed modal approximation; with a derived normal reaction the static solver's incremental friction force does not vanish and its tangent is nonsymmetric ([code] `nonlinear_integration/src/lib.rs:1560`), which a symmetric-definite eigenproblem cannot represent |
  | friction (tangential DOF) | inactive (normal not bearing) | free, with no friction force; its normal support follows its own state. [code] The classifier returns inactive when the normal is not positive (`classify_support_state`, `core/solver/nonlinear_supports/src/lib.rs:658`; a derived normal is the magnitude of its linear source's reaction, `nonlinear_integration/src/lib.rs:2652`), and the static path prescribes no displacement for an inactive state (`prescribed_displacement_for_state`, `lib.rs:2128`, `Inactive` → `None`): the DOF is free. Admitted only when the static path confirmed it (Agent 0, 2026-11-02, O-14 (b)). Its confirmation establishes an explicit-normal inactive state exactly when the explicit normal is not positive (`nonlinear_integration/src/confirmation.rs:843`) and never a derived-normal one (`:870`), so a derived normal that does not bear stays refused |

  The normal restraint of a friction support is its own support and keeps its own state. The result names the case, each support's state and the frozen-friction approximation where it applies.

  **As built (I2b-2): the same-run state solve, the typed seam and the linearization** (Agent 0's decisions B, D2 and D3, 2026-11-02).
  - **The solve.** `modal/state.rs` (`solve_modal_state_case`) solves the bound state case **once**, through the static path's own private case solve `solve_load_case_observed`, on the prepared model, build, boundary and case materials, with the state case's resolved case as `load_state` on a load/reference-state document and its modulus-basis record on a plain one (both kept by the preparation), no capture, no product observer and a fresh scratch `SourceRecoveryBudget`. It first applies the guards the static entry applies to a nonlinear invocation, in its order (`dense_loop_memory_guard`, then `dense_scrutiny_guard`), and forms the basis stiffness as the static entry does for one (`form_basis_stiffness(…, false)`). The mode is the modal dense path's, `dense_scrutiny` (`MODAL_DENSE_PATH_MODE`), recorded in the evidence and the outcome; no sparse or other mode is substituted (D3). The request still carries no `solver_mode` member: the dense entry is the `dense_scrutiny` path. Nothing is stored or reused, and the static entry is not changed.
  - **The seam (decision B).** PP `lib.rs` gains one typed field on `LoadCaseSolve`, `nonlinear_state: Option<Box<NonlinearStateEvidence>>`, and three private types: the evidence (case id, solver mode, `converged`, the qualification, the standing and the final `SupportStateRecord`s), `NonlinearQualification` (`Qualified`, or `InspectableUnqualifiedGap`: the branch the existing `product_selected_state_is_qualified` call took, with `product_unsupported_gap_state_is_inspectable` for the second, recorded where `append_nonlinear_support_loop_results` takes it, never re-run) and `NonlinearStateStanding` (`Confirmed` exactly when the case's own publication uses the confirmed state, after its integrity and load-row guards; otherwise `NotEstablished` with the case's own fail-closed sentence). The loop function returns the recorded qualification as a third tuple member; one `#[inline(never)]` helper, `capture_nonlinear_state_evidence`, forms the evidence from values the case solve already holds, on the one completed construction, only for a converged solve with a recorded qualification. Every blocked, early-return and linear construction sets `None`; the non-converged or unavailable state's blocked return precedes the capture. Nothing in the static path reads the field: on 15 requests in both modes (the K-D5 nonlinear references, a gap and a lift-off variant, the two committed nonlinear demo models, an arc with and without a gap, a component body and a load/reference-state document) every static envelope was byte-identical before and after (SHA-256 of the serialized envelope; a one-off observation, not a committed regression; the committed tests cover state/publication agreement, refusals and constructor coverage). The edits are line-preserving: the field, the third tuple member and the capture sit on existing lines, and the new types and helper are appended at the end of `lib.rs`, so no earlier line moves. `LoadCaseSolve` grows from 1,880 to 1,888 B (aarch64-apple-darwin, debug), and an ordinary nonlinear static solve gains a small heap capture (the Box, the case id, the cloned states). Output preservation is not resource preservation: the B5O1 carrier atom `Result<LoadCaseSolve, FrameKernelError>`, the helper's and its callers' frames and the static and retained stack chains are D3's to measure and price; nothing is assumed unchanged.
  - **Admission (D2).** The state admits only when the evidence is present and bound to the state case, its mode is the requested `dense_scrutiny`, it converged, its qualification is `Qualified` (the converged-but-unqualified inspectable gap state is refused), its standing is `Confirmed` (a failed confirmation attempt or an absent one is refused) and the solve published no blocking diagnostic. Any other outcome refuses `MODAL_STATE_CASE_UNAVAILABLE` with the specific reason (not converged, missing evidence with the blocking codes, case or mode mismatch, inspectable unqualified gap, not established with the case's own sentence, blocking codes), carrying every static diagnostic of the solve. Admission reads the typed evidence and diagnostic codes and severities, never message text. **Limitation, stated:** the static path confirms no gap state (`confirm && passed && !gap`), and on every gap model tested its qualification takes the inspectable branch, so **every model with a gap support is refused for now**. Admitting gaps needs a static qualification and confirmation of gap states, not a modal change. Gap admission is **required** by release-target criterion 1 (RT-A2, RT-A7), not an accepted limitation: it depends on **RT-A3 R-3** (exact gap qualification, replacing `StrictGapEvidence::Unsupported` so `product_selected_state_is_qualified` holds without the inspection fallback, and a gap-capable `confirm_converged_state` so the product drops `!gap`), owned by RT-A3 and ordered by Agent 0 after RT-A3 increments 3 and 4 and R-4's prerequisites (P, the sparse nonlinear loop; the §4 certificate) (DEL-04-07 `route-unification-r4.md`). R-3 fills this seam; it adds no parallel one. The owner's option to accept a gap limitation is open and untaken; until R-3 lands the gap path stays unqualified and refused here. A load/reference-state case with a nonlinear support also needs **RT-A3 S6** (nonlinear supports on load/reference-state documents, with connectors).
  - **The linearization.** Exactly one final state per built nonlinear support, by support id; a missing, duplicate or unexpected id refuses. Only the table's pairs are admitted: one-way, lift-off and gap active (rigid) or inactive (free); friction sticking (rigid), sliding (free, force frozen) or inactive (free, no force; O-14 (b), released by Agent 0, 2026-11-02). Any other pair refuses by name (`StatePairInvalid`): sticking or sliding of a one-way, lift-off or gap support, and friction active, which the classifier never returns. The DOF is the built support's (node index × 6 + DOF index). A rigid state adds its DOF to the restrained set with no imposed or gap value; a DOF a linear support already restrains stays restrained and is recorded (the static loop refuses an active support on a base-restrained DOF, so this is reached only with constructed states). The pencil (§3.2, as built) is then formed on that augmented restrained set; I2b-1's dense path and every §3.4 check run unchanged on it.
  - **The outcome** names the case, the mode, each support's state and its modal treatment (`Restrained`, `AlreadyRestrained`, `Free`, `FreeFrozenFriction`), the augmented restrained set, the state solve's non-blocking static diagnostics, and the disclosures: the linearization at the named case's confirmed states (R12), the rigid-or-free rule, the friction normal as its own support, the frozen friction of each sliding support, the free tangent of each inactive friction support and each already-restrained DOF.
  - **Load/reference-state documents.** The resolved case is threaded to the solve as the static entry threads it. Since RT-A3 S6 the exact route's admission no longer refuses nonlinear supports on these documents (`exact_admission`); whether such a case's state admits follows D2's evidence rules above, with no modal bypass.
  - **Tests** (`modal/tests_state.rs`): each mapping (one-way active and inactive, lift-off, friction sticking and sliding with a guide normal and with an explicit normal, and friction inactive) gives eigenvalues equal bit for bit to a hand-built linear twin with the same restraints. Friction inactive is a genuinely solved state: an explicit normal of 0 N (cases `load:SLIDE`, `load:STICK` of K-D5 F-1 explicit, and `load:SLIDE-POS`, `load:STICK` of F-1 with its guide S-N kept) and of −70 N (`load:SLIDE`) each solves converged, `Qualified` and `Confirmed` through `run_modal_dense`, with the tangent free and the guide restrained where present (`friction_whose_normal_does_not_bear_is_free_confirmed_and_matches_its_linear_twin`). A derived normal that does not bear (F-1 under a tangential load only, so S-N's reaction is zero) converges `Qualified` but `NotEstablished` ("friction normal not established") and is refused (`friction_whose_derived_normal_does_not_bear_stays_refused`); the refusals of a non-converged case, a failed confirmation (weakly active and at-capacity states), every gap model, and, on constructed values, missing evidence, a duplicate, an unexpected id, a mismatched case or mode, an absent confirmation, an inspectable gap, a blocking diagnostic and each invalid pair; the evidence equal to the case's own state rows in both modes; and source scans that every blocked construction sets `None` and that the state module reads no message.
- **[decided]** M's sparsity pattern is the union of the element patterns with K's. A rigid-body node block couples translation and rotation inside one node's 6×6 block, which K's pattern may not fill. The ordering and profile are therefore formed over the union pattern before reverse Cuthill–McKee (`core/solver/sparse_direct/src/lib.rs:505`), so that no M entry falls outside K's profile.

### 1.4 Inputs on the request (no model-document change in M1)

**[decided]** Every new parameter is on the modal analysis request, as `solver_mode` is today (PP `lib.rs:1053-1092`), not in `model_settings`. This needs no W-17 document version.
- **Extraction bound (required, no default).** Agent 0, 2026-10-27: "no built-in default. It's a required request input: a number of modes, a frequency cut-off, or both, whichever stops first. The desktop remembers the last value per project. No code-derived cut-off value ships."
  - `mode_count` n (positive integer), `frequency_cutoff` f_c (positive frequency quantity), or both. A request with neither is refused (`MODAL_EXTRACTION_BOUND_REQUIRED`). A named bound that is not positive (a count of 0, or a cut-off that is not a finite positive quantity of the catalog's `frequency` dimension, in Hz after normalization; a bad unit or dimension is named) is refused (`MODAL_EXTRACTION_BOUND_INVALID`, I2a).
  - The engine and the headless runner never supply a value. The desktop pre-fills the last value used in the project (M3, §5.1); the request records the value actually used.
- **[decided] The reported set** (D-c). Agent 0's "whichever stops first" governs, and cluster completion is its one stated extension.
  - **Bounds.** The reported set is the lowest modes, in ascending frequency, up to whichever bound stops first: the lowest n modes, or all modes with f ≤ f_c. The cut-off is **inclusive**. With both bounds, the set is the smaller of the two, and the result states which bound stopped it. Membership is decided on the computed frequencies; a reported or excluded mode whose error estimate (§3.4, check 2) reaches across f_c is disclosed by name.
  - **Cluster completion.** A cluster is the one defined in §3.4, check 5: a maximal run of adjacent computed eigenvalues, each within the relative cluster tolerance δ_c of the next. That is the only definition used, here, in the ScopeOfWork and in PRD §12.9. If the stopping boundary falls inside a cluster, the set is **extended to complete the cluster**, and the result discloses the extension by count ("n requested; m modes added to complete a cluster"). With a cut-off, this can include modes just above f_c, disclosed the same way. A partial cluster is never published, because the basis of a partial eigenspace would be arbitrary.
  - **Completeness.** The completeness count is taken at a **final shift** placed in the gap between the completed last reported cluster and the next computed cluster, under the separation premise of §3.4, check 4. It must equal the number of modes reported, cluster extensions included. With both bounds, the interval checked is the one actually reported. A final shift that cannot be placed with that separation refuses the set by name; it is never replaced by an unseparated count.
  - **The cut-off shift sizes; it does not decide completeness.** On the sparse path (M2), a cut-off-only request is sized by a count at ω_c²(1 + ε_s), with ω_c = 2πf_c and ε_s a stated relative offset (an algorithm parameter fixed in M2 and recorded here). If K − σM is singular there (the 1e-12 pivot guard) or the growth monitor rejects it (§3.4, check 4), the sizing count is retried at a second, documented offset, which the result records; the offset never changes silently. The sizing count only chooses q and allocation (§3.3); it never decides termination. When the cut-off controls stopping, extraction continues until a completed computed cluster lies above f_c, and the final shift is placed below that cluster. An underestimated sizing count therefore costs only a regrowth of q, never an omitted mode below f_c. On the dense path, which computes every finite eigenvalue, a cut-off above the whole finite spectrum takes the whole-spectrum endpoint (§3.4, check 4) instead of a cluster above f_c; on the sparse path that request is refused (§3.3). The dense path computes the whole spectrum and needs no sizing count.
- **Modulus basis.** Without a state case (a plain document with no nonlinear support), the request carries the same vocabulary as a load case: `modulus_basis_ref` or `modulus_basis_temperature`, or neither (base material E, as for a load case without one). With a state case, the modulus comes from that case's resolved state, and a request modulus basis is refused as a conflict (`MODAL_MODULUS_BASIS_CONFLICT`).
  - [code] Load cases carry these (PP `lib.rs:815-828`), and `materials_for_modulus_basis_observed` resolves them (PP `lib.rs:10842`, called at `:3161`). F1 reuses that resolver unchanged.
- **Weight-to-mass acceleration.** `weight_to_mass_acceleration` (a positive acceleration quantity). It is required when a rigid-family component's weight enters the mass, and it is never defaulted (§2.3). One that is not a finite positive acceleration after normalization is refused (`MODAL_WEIGHT_ACCELERATION_INVALID`, I2a).
- **`solver_mode`** chooses the path: `dense_scrutiny` (M1b) or `sparse_interactive` (M2).
- **`state_case`** (D-s, D-w): the id of a load case whose resolved, solved state defines the modal model. It is **required**, with no default, when the document is a load/reference-state document or the model has a nonlinear support, and optional otherwise; a request that needs it and lacks it is refused (`MODAL_STATE_CASE_REQUIRED`, naming why). From that state the modal model takes:
  - **stiffness**, from the case's resolved per-member material pairs (each member's modulus selection, for example `temperature_interpolation`), through the static path's own resolver (`case_state::resolve::resolve_case`) and build (`build_model_for_members` with the resolved pairs), and the case's modulus basis on a plain document;
  - **mass**, from the case's mass inputs as self-weight reads them. In model document 0.9.0 contents density is per pipe and case-independent (`section.contents_density`), so every state's mass is the section mass. The load-case lane's RT-A6 design adds one shared resolver, `case_contents_density(model, case_id, pipe)` (a condition set's density when the case's `condition_use.contents_density` names one, otherwise the section's), read by both mass-per-length functions; modal calls the same resolver and functions for the state case, so the bases cannot drift. **Contents mass follows the state** (Agent 0, 2026-10-31): metal and insulation mass are state-independent; the contents mass is formed from the state case's resolved contents density wherever that case resolves one (for example a hydrotest's test contents), and from the document's section contents otherwise, and the result discloses which was used, per pipe group. A modal run of an operating state therefore never carries test-water mass, nor the reverse. Until the shared resolver lands, a state case that uses condition contents is refused as the solve refuses it (`LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED`), never solved with section contents in their place. A condition's `added_weight` is a load the user enters in force units, not mass: it is excluded from the modal mass and from the identity (self-weight lineage does not claim it), and disclosed. The weight-basis identity (§2.5) runs the self-weight generator's inspection for the same case, so the modal mass is that state's self-weight mass. **Inspection interface** (review round 7): today `generate_operations(…, true)` refuses a request whose case id collides with an existing case (`self_weight.rs:217`), and its lineage caller clears every case on a clone (`:1372`), which would lose the state case's `condition_use`. M1a therefore adds one `pub(crate)` inspection entry to `self_weight.rs` (DEL-05-01's code, a priced change re-registered under D3) that evaluates the generator's loads for a named **source case** of the intact model, keeping that case and its condition context for the density resolver, and writes its generated loads under scratch identities disjoint from every existing case and primitive id, without clearing or adding cases. Modal never clears cases or substitutes a new case id, so the section density is never selected silently. Self-weight authoring for condition contents (the case request carrying its condition context) is the load-case lane's RT-A6 work; until it and the resolver land, a state case that resolves condition contents is refused (below);
  - **nonlinear-support states**, from the case's same-run solve (§1.3);
  - **restrained DOFs:** a `boundary_motion` or imposed-displacement DOF stays rigidly restrained and its motion value does not enter, as for W-16 imposed displacement; installation and operating temperatures do not enter the mass, and thermal and other loads do not enter the eigenproblem.
  - **Admission, as the static path's** (review round 7): the state case passes the static entry's document validation, `resolve_case`, and its separate guards, `refuse_resolved_span_strain` (a nonzero resolved strain on an objective connector's replaced span is refused) and `refuse_all_reference_states` (a document whose every case is a zero-action reference state is refused). Modal does not bypass either: a refusal there refuses the modal request (`MODAL_STATE_CASE_UNAVAILABLE`, carrying the static diagnostic), and a reference-only document stays refused. Prescribed values, eigenstrain (thermal) and declared loads all **enter the state solve** that fixes the support states; they are excluded only from the homogeneous eigenproblem that follows.
  - **As built (I2a): the inspection entry and the state binding.** The `self_weight.rs` entry built is `inspect_generated_self_weight(model, pipe_refs, gravity)` (Agent 0 decision D-SW): it calls `generate_operations(…, true)` unchanged on the intact authored model under a scratch case id used by no case or primitive load (`modal:weight-basis`, lengthened deterministically until unique), with an empty source hash, and clears, adds or renames no case. It is **case-free**, which is sufficient only for the S2-admitted subset: there condition-dependent contents are refused (`LOAD_CASE_CONDITION_CONTENTS_UNSUPPORTED`, upstream, with no modal bypass), so every state's mass is the section mass. The named-source-case inspection described above, keeping the case's condition context, and RT-A6's `case_contents_density` are retained for the contents slice, which extends the entry. It runs only after the preparation below has passed; its plan is consumed only for the in-memory identity comparison and is never applied, serialized, stored or given any publication or authority meaning, and neither its scratch id nor its empty hash appears in any modal output.
  - **The post-lowering binding (Agent 0 decision D-L, in place of an observation hook in the lowering).** The modal entry lowers the model once, through the solve's own `operating_conditions::lower`; S2's resolver bytes are unchanged, and no case is planned, judged or resolved a second time. Before lowering, the context captures the authored state case (its `condition_use` presence and value, its wind exposure). After lowering, the binding proves: (1) the same case index and id; (2) the complete authored primitive-load prefix equal by value (every field, magnitudes by their bits). The other members of the case are not compared (Agent 0 decision A, DEL-04-08 §8.43): a Debug-text comparison of the case remainder was removed as redundant, because its rendering is an unboundable retained-pricing TEXT row and the lowering's only case edits are pinned in source (S2-PIN: `ResolutionPlan::apply` appends primitive loads and sets the wind exposure list and flag; `lower` clears `condition_use`). The premise is therefore the binder's provenance: it is visible to the modal module only and runs once, on `prepare_modal`'s own clone of the request model, directly after the one lowering and before any stage edits the model (tested), so it is not a validator of an arbitrary lowered model; (3) the generated suffix exactly S2's documented output: every temperature record, then every added-weight record, each in `pipe_segments` order, with S2's identity (`temperature_load_id`, `added_weight_load_id`), provenance (`temperature_provenance`, `added_weight_provenance`), category, element target, direction and dimension. Membership follows from authored presence facts only (the uses the case states, each pipe's `condition_set_ref`, the set's condition key and `added_weight` presence); no temperature difference is recomputed (a generated temperature magnitude is S2's output, bound by its identity and provenance), and an added-weight magnitude, which S2 copies verbatim from the set, is compared with that authored value; (4) the wind exposure, distinguishing absent, unstated, authored (possibly empty) and resolved (possibly empty), the resolved list being exactly the pipes whose set marks `exposure.wind`, in pipe order. Any missing or contradictory evidence refuses (`MODAL_STATE_CASE_UNAVAILABLE`, with the reason); there is no fallback to a second resolution or to a prefix-only match. The evidence is taken on the model the lowering returned, before unit normalization rewrites primitive-load magnitudes, and its verdict is applied after the static admission of the state case; the case's place is then checked again. A condition's added weight bound this way is an action, never mass.
  - **The preparation (I2a).** The modal entry mirrors the static entry's pre-formation sequence (`run_linear_static_preview_observed_inner`), calling the same crate functions in the same order and stopping where it stops, as `validation/model_checks.rs` does, and does not change the static entry. It is a second copy of that sequence, so it can drift: its test compares every diagnostic with `pre_formation_checks` (itself a mirror, not the static entry) and, against the actual static entry on the same requests, only the decision (refused exactly when the entry is blocked, with the same blocking diagnostics). That holds on the tested fixtures only; it is not a proof that the sequences are identical. A static refusal that names the state case (or any refusal in the load-state resolution and its two guards) is carried by `MODAL_STATE_CASE_UNAVAILABLE`. On a plain document the state case's modulus basis, or the request's, is resolved by the static resolver (`materials_for_modulus_basis`). **I2b-2** retires I2a's interim refusal of a model with a nonlinear support (`MODAL_NONLINEAR_SUPPORT_INTERIM_UNSUPPORTED`): such a model passes the preparation, which keeps the state case's resolved case and modulus-basis record for its same-run solve (§1.3, as built).
  The result states the case, its resolved modulus selection per member group, its mass inputs and each support's state. A case whose state cannot be resolved, or whose solve does not converge, is refused (`MODAL_STATE_CASE_UNAVAILABLE`). If a document's case semantics would give one state two mass bases (for example a contents density that self-weight and the case resolver read differently), the request is refused by name and the conflict goes to Agent 0 (Agent 0, 2026-10-31).
- **`include_dynamic_only_restraints`** (D-t): a boolean, required when the model has a snubber, with no default. A response-spectrum solve sets it (DEL-04-10).

### 1.5 Outputs

**[decided]** (D-d) Every published quantity has integer unit exponents, so each one is expressible in the unit catalog (`core/units/src/lib.rs:26-56`, `DimensionVector` of `i8` exponents, `:224`). A mass-normalized shape (φᵀMφ = 1) would not be: its translations are in kg^-½, its rotations in rad/(m·kg^½) and its participation factors in kg^½. Per mode i, in ascending frequency:
- f_i (Hz), ω_i (rad/s) and T_i (s).
- The **mode shape**, as six components per node in global axes, in exactly one of two normalization cases. Each mode records its case (`unit_max_translation` or `unit_max_rotation`), and each row carries its own unit. Restrained DOFs are published as exactly 0. Ties for the largest component go to the lowest global DOF index.
  - **The "no translation" threshold.** Let t = the largest free translational component, θ = the largest free rotational component (rad) and ℓ = the diagonal of the bounding box of all the model's nodes, free and restrained (m), which is positive for any model with an element, including a one-element cantilever with one free node. A mode has **no translation** when t ≤ τ_t·ℓ·θ, and otherwise it is case (a):
    1. t ≤ τ_t·ℓ·θ, with τ_t = 1·10⁻⁶ (dimensionless). It is an algorithm parameter, not an engineering default. M1b confirms it on the pure-torsion and rigid-scaled references (§4) at no less than 100 times the largest translation found there in a mode that is rotational in exact arithmetic, and records the measurement here. It is only tightened (DEC-026).
    2. **No participation condition** (revised 2026-10-31). An earlier condition 2 required every translational effective-mass fraction to be at most τ_t and refused a mode that failed it. Under D-v, participation includes the mass coupling to restrained DOFs, so a shape with no free translation can carry large translational participation: the first bending pair of a one-element simply supported span has only free rotations and about 83 % of T_k (`f1_modal/mass_discrete`, E-SS-1). Such a mode is physical and its effective mass is normalization-invariant, so it is published in case (b) with its participation as computed, not refused. `MODAL_NORMALIZATION_UNRESOLVED` is withdrawn.
  - **(a) A mode with nonzero translation:** normalized to **unit maximum translation**. The largest-magnitude translational component is +1. Translations are dimensionless (m/m) and rotations are in rad/m. The generalized mass is in kg, Γ is dimensionless and the effective mass is in kg.
  - **(b) A mode with no translation**, such as pure torsion of a straight run: normalized to **unit maximum rotation**. The largest-magnitude rotational component is +1 (rad/rad). The translations are the computed vector's own, in m/rad, and are not altered: under the conditions above they are at most τ_t·ℓ. The generalized mass is in kg·m²·rad⁻² (the catalog family `rotational_generalized_mass`). Γ (in rad/m) and the effective translational masses (kg) are computed from that same published vector, so Γ²·m = m_eff holds in the catalog's dimensions, which track angle as an exponent: (rad/m)²·kg·m²·rad⁻² = kg. The rotational entries of M are read with the same convention (kinetic energy in J with rotations in rad). The participation is whatever the published vector gives, which can be large through restrained-DOF coupling; none is set to zero. The integrity checks (§3.4) are evaluated on the published vector itself, so the evidence describes the published shape. A display may show translations below τ_t·ℓ as 0, and says so; the published values are unchanged.
- The **generalized mass** m_i = φᵢᵀ M_ff φᵢ: kg in case (a), kg·m²·rad⁻² in case (b).
- **[decided] Zero denominators and no finite modes.** If T_k = 0 (no translational mass along k), every m_eff,ik is 0 by Bessel's inequality; the fractions along k are published as undefined with that reason. If M_ff = 0 (all mass on restrained DOFs, or none), there is no finite eigenvalue and the request is refused (`MODAL_NO_FINITE_MODES`).
- For k = X, Y, Z, with R_k the rigid unit translation over **all** DOFs (1 on each node's k translation, 0 elsewhere) and p_k = (M R_k)_f = M_ff R_k,f + M_fc R_k,c the uniform-support-motion excitation of the free DOFs (DEL-04-10 Design §2, D-v):
  - participation factor Γ_ik = φᵢᵀ p_k / m_i: dimensionless in case (a); in case (b), in rad/m, computed from the published vector, with no condition on its size (§1.5);
  - effective mass m_eff,ik = (φᵢᵀ p_k)² / m_i: kg, normalization-invariant in both cases; in case (b) computed from the published vector;
  - effective-mass fraction m_eff,ik / T_k (dimensionless), with T_k = R_kᵀ M R_k the model's total mass in direction k, and its cumulative sum. Because the modes are M-orthogonal, Σ_i m_eff,ik ≤ T_k over any set of modes (Bessel's inequality in the M semi-inner product, with φ̂ᵢ the mode extended by zeros on the restrained DOFs: φᵢᵀp_k = φ̂ᵢᵀM R_k). Over all modes the shortfall is the mass that moves with the supports, which the result states.
  - Γ, m_i and m_eff are what F2 needs: a modal response is Γ_ik·φᵢ times a modal coordinate, whatever the normalization.
- Per-mode integrity evidence (§3.4) and a cluster id when eigenvalues are repeated or nearly repeated (§3.4).
- **[decided] Units for these quantities.** The catalog has Time (s) but no frequency, angular-frequency, rotation-per-length, length-per-rotation or rotational-generalized-mass family. These families are an extension of the unit catalog, owned by DEL-02-02 (`core/units/**` is in its code paths), and carried by model document 0.9.0's S1 in its D3 turn (agreed with the load-case lane): `frequency`, `angular_frequency`, `rotation_per_length`, `length_per_rotation` and `rotational_generalized_mass` (kg·m²·rad⁻²). Hz is canonical for frequency, rad/s for angular frequency, and s (Time, already present) for period. The extension is a gating input of M1b (`deliverable.yaml`). `core/units/src` is a priced crate directory (`tools/retained_pricing/base/crate_dirs.txt`), so the change that makes the extension re-registers D3 (§3.6).

Per result:
- total model mass by category: pipe metal, contents, insulation and component bodies (this also gives the survey's A9 weight summary);
- free-DOF mass per direction, R_k,fᵀM_ff R_k,f, as a mass summary; it is not the denominator of the effective-mass fractions, which is the total mass T_k = R_kᵀM R_k (§1.5, D-v);
- the Sturm count, its final shift, the count's backward-error estimate e_b (formation and factorization parts), the inverse-norm estimate ν, the conditioning product e_b·ν against 1/κ_c, the separation ratio, the pivot-growth factor, and the decision under each premise (§3.4, check 4);
- the extraction bound used, which bound stopped extraction (the count, the cut-off, both when they coincide, or, on the dense path only, the end of the finite spectrum when neither bound is reached), any cluster extension by count, and, on the sparse path, the sizing shift used for a cut-off with its offset.

Rotational participation (about axes) is left to F2.

**As built (I2b-1): an in-crate typed outcome, not a publication.** `modal/outcome.rs` holds `ModalOutcome` (`pub(crate)`), returned by `run_modal_dense`; no row, schema, profile or entry publishes it (O-8). Per reported mode: index, cluster, λ, ω, f, T, its normalization case with the +1 DOF, the shape over every global DOF (restrained DOFs exactly 0), m_i, Γ_ik, m_eff,ik, the fractions and their cumulative sums, the per-value below-normal flags (shape indices as global DOFs), its check-2 evidence, its cluster e_λ and its standing. A fraction is `None` where T_k = 0, never 0 in its place; a product model cannot reach that case (a pipe's or a body's mass is the same in every translation, so T_k = 0 in one direction means M_ff = 0, refused `MODAL_NO_FINITE_MODES`), so it is tested on a constructed two-DOF pencil. Per result: the extraction bound, the stop reason, the count by the bound and the cluster extension, the completed endpoint (below a computed cluster, empty below the lowest mode, or the whole finite spectrum on an established census), the clusters, the integrity evidence, the standing, I2a's mass summary and identity report, the free-DOF mass R_k,fᵀM_ff R_k,f, the bound state case, the guard record, f_e per straight member, the disclosures (including any computed mode whose e_λ reaches across f_c) and the preparation's non-blocking diagnostics.

**As built (I3a): the published projection.** `ModalOutcome` also keeps the run's own authoritative payload (node ids and coordinates, the free DOFs, the formed global mass's stored upper entries, ℓ, τ_t and a), and `modal/document.rs` projects it as the experimental modal result family of §5.3. Every per-mode quantity above is published with its own case-aware unit, flag and reason; λ is not published (§5.3.5).

---

## 2. The mass model, on the self-weight basis

Agent 0, 2026-10-27: "consistent mass on straights, rigid-body lumps on arcs and components, on the self-weight basis, with g·M·1 = self-weight as a regression. Agreed. Settle the component W/a basis in Design, with Codex review."

### 2.1 What self-weight forms today

- **[code]** Self-weight is not computed in the solve. It is a **generated model edit**: `generate_self_weight_operations` drafts primitive loads with evidence and a method string (`self_weight.rs:135-142`). An inspection mode (`generate_operations(…, true)`, `self_weight.rs:142`, `:1363`) regenerates a plan for lineage checks without the source-hash and provenance checks (`:157`, `:241`, `:325`).
- **[code]** The gravity of a self-weight case is a request input: `SelfWeightGravity { value, unit, axis }` (`self_weight.rs:20-35`). The axis must be `global_x`, `global_y` or `global_z`. The value must be finite and nonzero, is signed, and is normalized to m/s² through the unit catalog (`:175-205`). The model document has no model-level gravity, and the product applies no physical-constant default for it (seismic generation states the same rule: "no physical-constant default is applied", PP `lib.rs:10491`).
- **[code]** Pipe self-weight is `mass_per_length × normalized acceleration`, applied as a distributed force along the global gravity axis on every selected member, whatever the member's orientation (`self_weight.rs:381-384`, `:416`).
- **[code]** Mass per length on the PF basis (`STABLE_MASS_METHOD = "source_od_effective_wall_areas/v1"`, `stable_mass_per_length`, `self_weight.rs:731-845`):
  - metal: `source_wall_area_m2(OD, t − c_m) · material_density`;
  - contents: `source_bore_area_m2(OD, t − c_m) · contents_density`;
  - insulation: `source_insulation_area_m2(OD, t_ins) · insulation_density`.
  - Density is required ("explicit material density required"). Insulation needs both thickness and density, and absence is never a zero default.
  - The inputs are on the pipe section (PP `lib.rs:509-536`); the area functions are in `annulus_geometry.rs:54-61`.
- **[code]** Rigid-family components (W-15 slice b): for a selected member that an eligible component maps, the member's distributed self-weight is **replaced** by the component's weight. It is applied at the node as F = W·ĝ and M = r × F, with r the centre-of-gravity vector in global axes from the node (`self_weight.rs:3-9`, `:503-516`, `:548-650`).
  - W is the authored force. The acceleration contributes only its sign: `wrench` takes `f = if acceleration > 0.0 { weight } else { -weight }` (`self_weight.rs:507-508`). [design] The W-15 reference states the convention: "The acceleration's magnitude does not scale W: W is the user's force." (`validation/references/w15_component_weight/README.md`, Convention.)
  - The component primitive's evidence records the case's acceleration as `normalized_acceleration_m_per_s2` (`self_weight.rs:707-708`).
  - A component that fails a condition (consumption not geometry-only, missing weight or centre of gravity, mapping conflict) leaves the member its pipe weight, with a warning naming it (`self_weight.rs:600-620`; `COMPONENT_WEIGHT_NOT_APPLIED` (code `SELF_WEIGHT_COMPONENT_WEIGHT_NOT_APPLIED`), `:473`).
- **[code] The EX mass basis** (D-r). The same function serves EX with `exact = true` (`self_weight.rs:311`, `:784-798`): `EXACT_MASS_METHOD = "source_od_nominal_wall_pressure_bore_areas/v1"` (`self_weight.rs:855`), metal on the nominal wall t and contents on the pressure bore from t_r = (t − c_m) − c_a, with an explicit `corrosion_allowance` required; insulation as on PF. EX seismic generation uses the same basis (`exact_generation_mass_per_length`, `self_weight.rs:844`). F1 on EX takes its m̄ from this method, and the component-weight path (`component_weights`) is route-independent.
- **[code] Finding: a second mass basis.** Seismic equivalent-static generation forms its mass with `compute_pipe_mass_per_length` (PP `lib.rs:10293-10420`, called at `:10502`), not with `stable_mass_per_length`.
  - It forms π/4·(OD² − ID²) directly, not through the annulus source-area functions.
  - It loops over every pipe segment and does not substitute component weights for rigid-family members.
  - So the product has two PF mass bases today. **[decided]** F1 joins the self-weight basis, which is the one the static weight check can confirm.
  - Agent 0, 2026-10-27: "record it as a finding. It goes on the remaining-items list beside the deferred PF seismic-mass alignment, since both should reconcile on one mass basis. No action in F1." It is recorded in `docs/COMPLETENESS_MAP.md` §4.3 (Remaining), in the row with the PF seismic-mass alignment to D-5 that Agent 0 deferred on 2026-10-27 (DEL-05-01 Design `exact-equivalent-static.md`, W-16c).

### 2.2 Element masses

**[decided]** (D-b)

| Element | Mass matrix |
|---|---|
| Straight, Euler–Bernoulli | **Consistent.** Hermitian-cubic transverse mass m̄L/420·[156, 22L, 54, −13L; 22L, 4L², 13L, −3L²; 54, 13L, 156, −22L; −13L, −3L², −22L, 4L²] in both bending planes. Linear axial m̄L·[1/3, 1/6; 1/6, 1/3]. Linear torsional j̄L·[1/3, 1/6; 1/6, 1/3]. Rayleigh rotary-inertia terms ī/(30L)·[36, 3L, −36, 3L; 3L, 4L², −3L, −L²; −36, −3L, 36, −3L; 3L, −L², −3L, 4L²] in both planes. In member axes (x along the member) the transverse matrices act on [u_y, θ_z] in the x–y plane and on [u_z, −θ_y] in the x–z plane: the sign change makes the second coordinate the slope of u_z, so the x–z blocks of the displayed matrices change sign in their translation–rotation entries; the rotary-inertia matrix likewise. Transformed with the member's own frame orientation. Standard results, e.g. Przemieniecki, *Theory of Matrix Structural Analysis* (1968), ch. 11; cited, not copied. |
| Realized arc | **Lumped**: two rigid half-arcs, each a 6×6 rigid-body mass about its end node, with the half-arc's true mass, first moment (centroid offset) and second moment (§2.4) |
| Rigid-family component body | **Lumped**: one 6×6 rigid-body mass about the component node. Mass m = W/a (§2.3), offset r = the centre of gravity, rotational block m·[r]×ᵀ[r]× (a point mass at the centre of gravity; the body's own inertia about its centre of gravity is not authored, so it is zero, and that is disclosed) |

Per-length quantities on the self-weight section basis:
- m̄ = metal + contents + insulation, exactly `stable_mass_per_length` (`self_weight.rs:731`) on the model's route (PF or EX, §2.1).
- j̄ (polar, per length) = ρ_p·J_metal + ρ_ins·J_ins, with J_metal on the metal wall of the route's mass basis (t − c_m on PF, t on EX), with J of an annulus = A(r_o² + r_i²)/2. Contents are excluded (D-e).
- ī (bending rotary, per axis, per length) = j̄/2 for the circular sections PF has.

Reasons:
- **Accuracy.** For conforming elements, consistent-mass frequencies converge from above (Rayleigh–Ritz), with O(h⁴) frequency error for the Hermitian beam. A lumped diagonal mass converges at O(h²) and gives no bound.
  - On one simply supported span, one consistent element gives ω₁ ≈ 10.95·√(EI/m̄L⁴) against π² ≈ 9.87, about 11 % high; two lumped elements give √96 ≈ 9.80, about 0.7 % low. These figures are stated from memory and are re-derived by the reference generator.
  - Either way, long spans need intermediate nodes (§2.6).
- **Memory.** Consistent M has the element pattern of K. In profile storage it costs one more P-entry array (8P B), where P is K's profile entry count (`sparse_direct/src/lib.rs:283`). A diagonal lumped M costs 8n B. K − σM needs one profile either way.
- **Robustness.** A purely translational lumped mass makes M singular. Both paths reduce with K, not M (§3.2, §3.3), so a singular M only adds infinite eigenvalues, which are never extracted.
- **Checkability.** Consistent M gives an element-wise weight identity (§2.5): g·M·r_k reproduces the consistent nodal vector of a uniform load, moments included. A lumped M gives only a resultant identity.
- A lumped-diagonal variant exists **only as an internal control** in the references, not as a user option, which would double the reference and reader matrix for little gain.

**[decided]** (D-e) Contents are in all three translations, as self-weight applies m̄ along any global axis whatever the member's direction (§2.1). This keeps g·M·r_k equal to self-weight in every direction, member-axial directions included. Contents are excluded from torsional and rotary inertia, which is disclosed in the result.

### 2.3 Component bodies: the W/a basis

**[decided]** (D-f) The component mass is m = W/a with a = `weight_to_mass_acceleration`, a required request input whenever a component's weight enters the mass. There is no default. The reasoning:

1. **What the self-weight basis fixes.** For a self-weight case at acceleration g (normalized, signed), the generator applies m̄·g per length to each pipe and W, with the sign of g, to each eligible component (§2.1). The pipe term scales with |g|; the component term does not.
2. **The identity that defines "the same basis".** The mass basis equals the self-weight basis when, for the same acceleration, g·M·r_k reproduces the self-weight loads. For the pipe terms this holds for any g, since m̄ is independent of g. For a component it requires |g|·m = W, so m = W/|g|.
3. **Where |g| comes from.** No model-level gravity exists, and the product ships no physical constant for it (§2.1). The g of a self-weight case is that case's request input, recorded only in the generated primitives' evidence. So there is no consistent a without an input: the modal request must carry it, in the same vocabulary and normalization as the self-weight request (an acceleration quantity through the unit catalog), with only its magnitude used, so it must be positive.
4. **When it is required.** It is required exactly when the shared eligibility function (`component_weights`, below) yields at least one component body. A request without it is then refused (`MODAL_WEIGHT_ACCELERATION_REQUIRED`, naming the components). For a pipe-only model it is optional; if given, it is recorded and has no effect on the mass.
5. **Agreement with the model's own weight cases.** When the model carries a current applied self-weight case with component-weight primitives (`inspect_applied_self_weight`, `self_weight.rs:1266`; method `COMPONENT_WEIGHT_METHOD`), their recorded `normalized_acceleration_m_per_s2` magnitude must equal a after normalization, bit for bit. Otherwise the request is refused (`MODAL_WEIGHT_ACCELERATION_MISMATCH`, naming the case and both values). The reason: with a different a, the modal component masses would not be the masses implied by the static weight the model already carries. A stale or manually overridden component primitive is disclosed and does not take part in the comparison.
6. **Binary64 range and rounding.** The premise has four parts, each checked in binary64 before the mass is formed:
   - W and a are positive normal binary64 numbers;
   - m = fl(W/a) is a normal binary64 number;
   - fl(a·m) is finite and normal;
   - every entry of the component's spatial mass block, m·r_i and m·r_i·r_j, is finite and either zero or normal (W = 1, a = 10⁻³⁰⁰ and an offset of 10¹⁰ pass the scalar parts but overflow here).

   Outside that premise the request is refused by range (`MODAL_COMPONENT_MASS_OUT_OF_RANGE`, naming the component and the operand). The third part is needed because W normal and W/a normal do not exclude overflow or underflow of a·m when W lies within a few ulps of the binary64 range's ends. Under the premise, m = (W/a)(1 + ε₁) and fl(a·m) = W(1 + ε₁)(1 + ε₂), with |ε₁|, |ε₂| ≤ u = 2⁻⁵³. So a·m differs from W by a relative amount of at most 2u + u², about 2.2·10⁻¹⁶, and is not always equal to it. The identity check (§2.5) is judged within its measured tolerance and against the modal formation check (§3.4); it is exact only in real arithmetic.

**[decided]** F1 calls the **same** eligibility function as self-weight (`component_weights`, `self_weight.rs:548`, exposed `pub(crate)`), over all members, so each predicate is read once and the two bases cannot drift.
- Where a component's weight applies, the member's distributed mass is replaced by the component body.
- Where it does not, the pipe mass stays and the same `COMPONENT_WEIGHT_NOT_APPLIED` warning is emitted.

**[code]** The model has no other concentrated mass: no node mass field and no mass on non-rigid-family components (PP `lib.rs:558-582`, `:586-650`). Valves and flanges are rigid-family, so their weights are covered.
- **[open]** A free-standing concentrated mass (an instrument, an actuator on a non-rigid member) or a component's own rotary inertia needs a model-document field (W-17). It must enter self-weight and modal together to keep the bases identical (§9).

This basis is settled here and goes to an independent Codex review, as Agent 0 directed (§9).

### 2.4 Arcs

- **[code]** Self-weight on a realized arc uses `consistent_uniform_nodal_loads`, documented as in equilibrium with the distributed-load resultant (`curved_bend/src/lib.rs:509-528`). The arc length is available (`:395`).
- **[decided]** Each half-arc (angle α = θ/2, mass m = m̄·R·α) is a rigid body about its end node n, with the 6×6 spatial mass block [[m·I, −m[c]×], [m[c]×, J_n]]:
  - c = (1/m)∫ m̄ ρ(s) ds, the centroid offset from the end node, with ρ(s) the arc point's position relative to n;
  - J_n = ∫ m̄ (|ρ|²I − ρρᵀ) ds + ∫ T(s)·diag(j̄, ī, ī)·T(s)ᵀ ds: the centreline second moment plus the section's own rotary inertia, with T(s) the local frame (tangent first) along the arc and j̄, ī the straight member's per-length polar and bending rotary inertia (§2.2), so arcs and straights share one inertia convention;
  - all three are closed-form integrals in α. Expressions with cancellation (for example the in-plane term m̄R³(3α/2 + sin 2α/4 − 2 sin α), whose leading term is m̄R³α⁵/20) are evaluated by their series, so that small angles keep their positive leading values. **As built (I1):** `modal_eigen` evaluates the half-arc quantities by their series across the whole admitted range 0 < α < π/2 (the closed forms lose up to about 10⁻²⁸ relative accuracy near α = 1/2), agreeing with the frozen `f1_modal/mass_discrete` references to 5·10⁻³³ normwise on M-ARC-SMALL and 4.9·10⁻³² on the other arcs.
  - Together the two lumps reproduce the arc's total mass and centroid, and so the self-weight resultant force and moment exactly in real arithmetic.
  - They do not reproduce the nodal split of `consistent_uniform_nodal_loads`, which depends on the flexibility factors. Arcs are therefore checked on the resultant (§2.5).
  - User flexibility factors change stiffness only, never mass.
- **[decided]** A typical bend (R ≈ 1.5 D) is short against its spans, so lumping is adequate. A long-radius bend falls under the discretization warning (§2.6). The product cannot subdivide a macro-element; the user can author two bends.

### 2.5 The weight-basis identity check

Agent 0's regression, "g·M·1 = self-weight", is a **blocking integrity check on every modal result**.
- For each global direction k, form F_k = a·M_full·r_k on the full (unreduced) mass, with a the request's acceleration, or 1 m/s² when no component weight enters. (With no component, the mass is independent of a; multiplying by 1 is exact in binary64, and the check scale publishes nothing.)
- Run the self-weight generator in inspection mode over the **mass-bearing members** (`generate_operations(…, true)`), with gravity a along k, on the model's route, so that on EX the exact mass method is compared. The mass-bearing members are every member except an objective connector's replaced span, which carries neither self-weight nor mass ([code] the connector contract, `joint.rs:164`: "the replaced span carries no load, member result or self-weight"). One shared selection function serves mass assembly and the inspection, so they cannot differ.
- Compare. The identity is not literal vector equality for every family: it holds **nodally for straight members**, and **on the resultant force and moment for arcs and components**, each within a measured tolerance.
  1. **Straight members, nodally.** With the Hermitian consistent mass, M_e·r_k is, in real arithmetic, the consistent nodal vector of a uniform load: forces a·m̄L/2 at each end and moments ±a·m̄L²/12. The rotary and torsional terms vanish for a rigid translation. Compare with `straight_pipe`'s uniform-load vector for the generated intensity. [design] The PF Timoshenko design keeps the Euler–Bernoulli vector for full-span uniform loads (`pf-timoshenko.md`, #1663), which M4 relies on.
  2. **Arcs and components, on the resultant.** Total force, and total moment about a fixed point: for each arc or component, its own node centroid; for the whole model, the model's node centroid (to limit cancellation). The half-arc lumps do not reproduce the arc's nodal split (§2.4), and a component body is compared as the wrench W·ĝ, r × W·ĝ (§2.1). Straight members also enter the resultant.
- **The tolerance: measured, then pinned (DEC-026).** Until M1a measures it, the identity tolerance has no value and is `TBD`, and no modal result is published. M1a evaluates the identity on the frozen `f1_modal` mass references and its test models, and records the largest relative and absolute discrepancies, nodal and resultant, with the models and the build. The tolerance is then pinned as a relative and absolute pair at a stated multiple of those discrepancies and recorded in this design and in the test. From then on it is only tightened; loosening it would be a governance event under DEC-026, not a code edit.
  - **D-TOL (Agent 0): experimental, unpinned.** The tolerance stays `TBD`. The I2a code takes it as an explicit argument with no default and no production value; only its tests supply a bound (a test-local, non-production `TEST_LOCAL_IDENTITY_BOUND`, relative 10⁻¹², absolute 0), and nothing admits or publishes on it. An entry fails when its discrepancy exceeds absolute + relative·scale, where a force's scale is the largest reference force and a moment's is the larger of the largest reference moment and that force scale times the compared body's size (the bounding-box diagonal of what is compared: an arc's or a component's own nodes, a component's centre of gravity included, and never the rest of the model, so a distant part cannot inflate the moment scale and hide a component's offset error; for the whole model, its nodes and its node centroid), so that a moment that is zero in exact arithmetic is judged against the moments its forces make.
  - **Measured (I2a, debug build, aarch64-apple-darwin, `modal::tests::d_tol_measurement_table`):** over the ten convertible `mass_discrete` cases on both bases (20 rows; `M-MASSLESS` holds a connector and a tie rod, which the test's converter does not author, and the replaced span is covered by the connector model below) and five test models (pipe-only PF, component PF, arc PF, EX n05, the T4-I12 connector demo on EX), the largest discrepancies were: nodal 1.38·10⁻¹³ N or N·m absolute, 5.45·10⁻¹⁶ relative; resultant 2.85·10⁻¹³ absolute, 4.38·10⁻¹⁶ relative. The product mass agreed with the references to at most 1.87·10⁻¹⁶ relative normwise, and T_k to at most 1.87·10⁻¹⁶. These are measurements, not a pin; pinning stays Agent 0's decision.
- A failure blocks with `MODAL_MASS_WEIGHT_BASIS_MISMATCH`, naming the member or component.
- If the model also carries a current applied self-weight case along k whose scope covers every member, its resultants are compared as a second, **non-blocking** row. A partial-scope case or a manual override (`MANUAL_OVERRIDE_METHOD`, `self_weight.rs:848`) gives a disclosure, not a failure. The acceleration rule in §2.3 item 5 is separate and blocking.

### 2.6 Mass discretization adequacy

- **[design]** The survey asks for SWBPIPE's own spacing rule, derived and benchmarked (survey R8).
- **[decided]** (D-l) A warning, not a refusal:
  - for each straight member, compute the first fixed-fixed frequency of the member alone, f_e = (4.7300²/2π)·√(EI/(m̄L_e⁴));
  - warn (`MODAL_MASS_DISCRETIZATION_COARSE`, naming the member) when f_e < c·f_max, where f_max is the highest reported frequency;
  - c is derived by benchmark in M1a, so that a span meshed at the threshold is within a stated error of its continuum frequency (with consistent mass, c ≈ 2–3 is expected). The factor and its derivation are published in the warning text.
- A helper that adds intermediate nodes is a model operation through the operation applier (user-visible, undoable): a separate slice (M6), not part of the solve.

---

## 3. Eigen-solver: dense and sparse paths

### 3.1 What exists

- **[code]** `PreviewSolverMode::{SparseInteractive, DenseScrutiny}`, sparse by default (PP `lib.rs:1053-1092`).
- **[code]** The dense scrutiny guard counts 96 B per n² entry (`DENSE_SCRUTINY_BYTES_PER_ENTRY`, PP `lib.rs:4279`) against a provisional 6 GiB ceiling (`DENSE_SCRUTINY_CEILING_BYTES`, `:4283`). That admits at most 8,192 DOF. It refuses before any n² allocation, and there is no automatic dense fallback (`dense_scrutiny_guard`, `:4303`; applied at `:3041-3048`).
- **[code]** The sparse solver (DEC-023, "zero new numerical dependencies") is reverse Cuthill–McKee ordering plus skyline LDLᵀ (`sparse_direct/src/lib.rs:1-11`, `:377-458`):
  - single-threaded, fixed operation order, std only;
  - no pivot reordering;
  - pivots at or below an absolute 1e-12 refused (`SPARSE_SOLVE_ZERO_PIVOT_GUARD`, `:23`);
  - **negative pivots counted and located**, not refused (`FactorizationReport.nonpositive_pivot_count`, `first_nonpositive_pivot`, `:126-130`);
  - `SymmetricProfileMatrix::from_entries_with_order`, `factorize_ldlt` and `ProfileFactorization::solve_ordered` are public (`:215`, `:382`, `:339`).
- **[code]** The frame kernel's positive factor has dense and profile backends (`structural.rs:2103-2133`). Its `solve` is private, and `finish_structural` is the only completion seam (`:2116`, `:2134-2141`).
- **[code]** The only eigenvalue routine in the tree is a cyclic Jacobi for condition-number evidence (`jacobi_symmetric_eigenvalues`, `core/solver/performance_harness/src/lib.rs:993`).
- **[decided]** No new numerical dependency (DEC-023). Both paths are written in-house, std only.

### 3.2 Dense path (M1b): reduce with K's Cholesky, tridiagonalize, implicit QL

**[decided]**
1. Assemble dense K_ff and M_ff.
2. Factor K_ff = R Rᵀ (Cholesky). K_ff is SPD on an admitted model; a non-positive pivot refuses as singular, as the static path does.
3. Form C = R⁻¹ M_ff R⁻ᵀ (symmetric positive semidefinite) with two triangular solves, symmetrizing from the lower triangle.
4. The eigenvalues of C are μ = 1/ω². The largest μ are the lowest frequencies, so the wanted modes are the **best-conditioned** end of C's spectrum, and a singular M only adds μ = 0 in exact arithmetic. Outside the recovery band in item 5, λ is the binary64 reciprocal of the QL μ; inside that band μ is recovered from the Rayleigh quotient before taking the reciprocal. Retained pairs are sorted again after recovery.
5. Householder tridiagonalization, then implicit QL with Wilkinson shifts and accumulated transformations (the tql2 algorithm). Public algorithm: Golub & Van Loan, *Matrix Computations*, ch. 8; written in-house.
   - **[decided] The finite-mode count** (review round 6). Computing every eigenvalue of C does not show that every finite generalized eigenvalue was recovered: a small positive μ can round to zero and be taken as infinite (review witness: K = I₂ and an SPD M with determinant 2⁻¹⁰⁴, whose second finite eigenvalue ≈ 1.3·10³¹ comes back as μ = 0, while the retained mode passes its residual and a Sturm count at σ = 2). So the number of finite modes is established independently: it is n minus the number of **structurally massless** free DOFs, those whose rows and columns of M_ff are exactly zero; and the remaining block of M_ff, **equilibrated** by the diagonal congruence D_M·M·D_M with D_M = diag(m_jj)^−½ so that translational and rotational entries become dimensionless, must factor by Cholesky with every pivot at least τ_m (a stated dimensionless threshold, measured under DEC-026). The infinite-μ classification is separate: μ_i is taken as infinite when μ_i ≤ τ_μ·μ_max, τ_μ stated likewise. **Condensation of the massless subspace** (I1, after numerical review round 2): before the reduction, the structurally massless free DOFs (set 0) are eliminated by static condensation, K̂ = K_mm − K_m0·K_00⁻¹·K_0m on the massive DOFs (m), with M̂ = M_mm, since M has no entries on set 0; K_00 is SPD because K is. When the remaining equilibrated mass block passes the Cholesky premise above, the condensed pencil has exactly n_m finite eigenvalues in exact arithmetic; without that premise no such count is established. The count is resolved only when every μ of the condensed problem is classified finite (μ > τ_μ·μ_max); any μ classified infinite, of either sign, leaves it unresolved. Shapes are recovered on set 0 as φ_0 = −K_00⁻¹·K_0m·φ_m.

     **Two floors and band recovery (I1 candidate, 2026-11-01).** The relative infinite-μ classification above is a hard verdict, never rescued by a residual. A second hard verdict is μ ≤ κ_μ·δ_q, where δ_q = c_μ·n·u·μ_max, n is the full free order and u = 2⁻⁵³. It screens the absolute resolution of the Householder/QL reduction; it is an algorithmic estimate, not a certified error bound. The coefficients c_μ and κ_μ are the same explicit inputs as the formation floor below, with no default. Neither floor is bypassed even when a pair has a zero residual. A rejected condensed μ leaves the finite-mode count unresolved.

     The broader formation screen remains δ_μ = c_μ·n·u·ν(K̃̂)·‖M̃_mm‖_∞, on the diagonally equilibrated pencil K̃ = D K D, M̃ = D M D, D = diag(k_jj)⁻½. Here ν(K̃̂) is the Hager–Higham estimate of ‖K̃̂⁻¹‖₁ from the condensed factor. This screen is invariant to a positive diagonal rescaling of the basis in exact arithmetic; the full free n is the conservative order. It can be pessimistic: E-RIGID-SCALE-1e6 has six genuine high modes below κ_μ·δ_μ despite accurate vectors. Only the band max(τ_μ·μ_max, κ_μ·δ_q) < μ ≤ κ_μ·δ_μ permits recovery:

     - Sum g = φᵀMφ and h = φᵀKφ exactly from the formed, uncondensed pencil and the recovered full vector. Require both to be positive and finite before division. Compute μ_R = fl(g/h), λ_R = fl(1/μ_R), and require positive normal results, with μ_R still above both hard floors.
     - With r = Kφ − λ_R Mφ summed exactly per row, evaluate the check-2 form on the **formed** pencil: η̃ = ‖Dr‖₂ / ((‖DKD‖_∞ + λ_R‖DMD‖_∞)·‖D⁻¹φ‖₂), and e_λ = ‖Dr‖₂·‖D⁻¹φ‖₂/(λ_R g). Norms and quotients use extended-exponent double-double. Retain only if η̃ ≤ τ_f = c_f·n·u and e_λ ≤ τ_e, inclusively. Every compared value must be finite and normal or an exact zero; sign/range failures reject the pair. Both estimates are dimensionless; e_λ estimates relative eigenvalue error, not frequency error or modelling error, and is not a certified bound.
     - Recovery is limited to this band by decision. Above the broad screen the QL pair is retained, and τ_e is not applied; §3.4 check 2 reports that pair’s accuracy observations. Re-sort all retained pairs in ascending λ with their vectors; no new n² buffer is needed.

     **Threshold basis and limits.** c_μ = 1, κ_μ = 10 and c_f = 64 remain the existing test-local coefficients. Agent 0 authorized explicit **TEST-LOCAL τ_e = 10⁻⁶** on 2026-11-01 for this candidate, subject to fresh independent numerical and exact-old/new-text scrutiny; this is no production pin, default or DEC-026 change. For recovered band modes only, it tests an estimated six-decimal relative-eigenvalue accuracy requirement independently of a desired mode count. It is not a whole-spectrum accuracy criterion. The closed-form K = [[1,1],[1,3]], M = diag(1,2⁻¹⁰⁰), φ = (0,1), λ = 2⁵² has a tiny normwise η_f but e_λ > 10¹⁴; the former screen therefore cannot establish eigenvalue accuracy. On the rigid reference recovery band e_λ is at most about 3·10⁻⁸, while the diagnosed false pairs have estimates at least 3·10³. These are observed separations in the tested population, not guarantees outside it. Tests span τ_e = 10⁻⁷, 10⁻⁶ and 10⁻⁵, both sides and equality at each cutoff, sign/normalization and diagonal scaling controls. The independent replay's earlier 10⁻² was a historical diagnostic input only and is not a product or test threshold.

     **Review limits (PR #33).** The independent review of numerical source `0ba65b9d4` found no blocking band-recovery defect. Its additional exact-spectrum pencils included an above-band resolved mode with relative eigenvalue error 1.58·10⁻⁵ and unresolved examples near 0.9%; those pairs are not judged by τ_e. M1b must retain §3.4's per-mode re-formed integrity checks and standing, and must not infer six-decimal whole-spectrum accuracy from band admission. The first-order e_λ can also be pessimistic and safely reject accurate band pairs; it remains an estimate, not a bound. No threshold is loosened or scope expanded on that basis. An all-negative M, outside the positive-semidefinite premise, currently receives the generic typed QL-floor range refusal; no valid mode is published. These observations are recorded for the consuming product slice and do not accept a release limitation.

     **Census limit.** GENERAL's offset point lump has a non-coordinate mass nullspace. Its formed binary64 M has negative rounding-level directions; structural condensation does not remove them, the equilibrated Cholesky does not establish a count, and the result stays unresolved. No mass-rank truncation or recovery below either hard floor is performed. The whole-finite-spectrum endpoint retains its named refusal; the existing lower-mode/gap route is unchanged.

     **Admission range** (I1, numerical review round 3): the modal kernel admits a pencil only when every nonzero entry of the formed K and M, in canonical SI units, has magnitude in [2⁻³⁰⁰, 2³⁰⁰], and refuses any other by name (`MODAL_PENCIL_RANGE_UNSUPPORTED`). Physical piping models lie far inside this range; it leaves headroom so that products and sums of entries, scaled quantities and double-double intermediates stay in binary64 range for every admitted dimension. Within it, bilinear forms (participation, generalized masses, residuals, orthogonality, formation differences, Sturm mass quotients) are evaluated with an extended-exponent double-double where an intermediate could still leave range, and accumulated so that no contribution dropped by exponent alignment can be exposed by later cancellation (an exact or exponent-binned accumulation, or a refusal when dropped information leaves the result ambiguous), and every compared value is checked finite (and normal, or an exact zero) before use. Normal-range classification is decided on the exact value, before rounding. A **published** quantity whose exact value is nonzero but below the binary64 normal range (for example a negligible participation fraction) is published as 0 with a per-value flag stating that it lies below the representable normal range, rather than refusing the mode; a published quantity that overflows, or is NaN, refuses; a NaN is never discarded by a comparison or a reduction, and a failure is a typed refusal, never a silent zero. **Revised (2026-10-31):** count-matching on the uncondensed pencil was withdrawn, because a structural zero and a genuine high mode can trade places while the counts still agree (review witness: K = [[1, 1, 0], [1, 3, 0], [0, 0, 10²⁰]], M = diag(1, 0, 1), where the 10²⁰ mode was returned as 2⁵² and the structural zero as a retained μ = 2⁻⁵²); the earlier "any negative μ" wording was withdrawn before that, because QL returns the zero μ of massless DOFs as tiny values of either sign. Admission witnesses guard against false refusals: a small-bore one-element cantilever (OD 6.35 mm, wall 0.89 mm, 1 m, steel with water contents), whose smallest raw pivot ratio is ≈ 6·10⁻⁶ but whose equilibrated pivots are all ≥ 0.22, and a short-member model must be admitted with their full finite counts. Then the whole-finite-spectrum endpoint is not used: the request is refused if it needs that endpoint (`MODAL_FINITE_COUNT_UNRESOLVED`), and otherwise proceeds only on modes below a completed computed cluster.
6. Back-transform φ = R⁻ᵀ z, then normalize (§1.5).

- **Control.** The Jacobi pattern, generalized, runs on C in tests for models up to a few hundred DOF. It is an independent kernel-level check of the QL path, not a product path.
7. **[decided] The completeness count on the dense path** (M1b). The Cholesky factor of K gives no inertia of K − σM. The dense path takes its Sturm counts (§3.4, check 4) from **`sparse_direct`'s `factorize_ldlt`**, on the scaled profile of K − σM, with the read-only observation accessor of §3.3 for the growth monitor. There is no dense LDLᵀ of its own. The reason: one inertia implementation and one growth monitor serve both paths, on an existing, public, tested factorization, and the inertia-risk tests aim at the code the product actually uses. The independent counts in tests come from the reference generator (§4.1) and the Jacobi control. So the accessor, the shift placement and the growth limit are M1b's, with their D3 obligation (§3.6), and M1b publishes dense results with every check of §3.4.

- **Control.** The Jacobi pattern, generalized, runs on C in tests for models up to a few hundred DOF. It is an independent kernel-level check of the QL path, not a product path.
- **Memory.** **Measured (I1):** the Sturm phase, not the eigen core, sets the dense peak: the shift pencil keeps 32 B per union-pattern position, and each shift adds a transient 24 B per entry list plus the profile and its clone, about 32·n² B on top of the formed K, M and the mode shapes; `DENSE_MODAL_BYTES_PER_ENTRY` is recounted from the built phases when the product entry lands (M1b, I2). The eigen core alone (Cholesky, reduction, tridiagonalization and QL) needs four live n² buffers, 32 B per entry, as built in `modal_eigen` (one work buffer carries M R⁻ᵀ, then C, Q, Z and φ). With the product's formed K and M, K, M, C and the accumulated Q are alive together, plus one n² work array, plus, for a Sturm count, the shifted profile and its clone inside the factor (each at most n(n + 1)/2 entries), plus the double-double accumulators of the assembled formation difference (§3.4, check 1; 16 B per stored entry of K and M): about 64·n² B. The re-formation of §3.4 is element by element and stores no global matrix.
- **Time.** Band recovery adds at most 4pn² exact-product insertions for p screened pairs (two quadratic forms and two residual products per full matrix entry), plus 2n² visits for scaling norms when a band exists; actual insertions skip zero terms. `dense_screen_product_estimate(n, p)` reports this additional work separately from binary64 flops. Exact insertions have up to three factors and four expansion parts; their runtime cost is not one binary64 flop. Cholesky n³/3, forming C about n³, tridiagonalization 4n³/3, QL with vectors about 6n³: roughly 9n³ flops, plus at most n³/3 for each Sturm factorization. n = 3,000 is about 2.4·10¹¹ flops (minutes, single-threaded, optimized build); n = 8,192 is about 5·10¹² (an hour or more). The registered retained identity is a debug, opt-level 0 build (DEL-04-08 `Work/retained-pricing-rules.md` §9), so every timing statement names its build.
- **[decided]** (D-k) Two guards, both applied before allocation and stated with their formula in the refusal text, as `dense_scrutiny_guard` is:
  - **Memory:** `DENSE_MODAL_BYTES_PER_ENTRY` is the count of live n²-sized binary64 buffers times 8 at the peak phase (64 for the layout above; the constant states the implementation's actual count), against the existing provisional 6 GiB ceiling. At 64 B per entry that admits n ≤ about 10,033 DOF.
  - **Work:** 9n³ + s_max·n³/3 plus the separately priced band-recovery exact products (worst case 4n³ insertions) and scaling visits, against a fixed operation budget, with s_max the stated largest number of candidate Sturm shifts (§3.4, check 4). M1b sets the budget by measurement under `t3_exclusive.sh` so that the dense path stays a scrutiny path for small and medium models (about 3,000 DOF is the target). The budget and the cost of exact products/scaling visits are measured and recorded here when M1b lands; I1 supplies counts, no unmeasured cost coefficient or product guard.
  - These are resource limits, not modelling defaults; they refuse and never truncate the extraction.
- **As built (I2b-1): one memory guard, the static one; no work guard; not modal product qualification.** The modal entry applies the existing `dense_scrutiny_guard` (96 B per global n² entry against the provisional ceiling) before any n² allocation. That formula's count covers the phases I2b-1 runs: the static structural solve used as the singular-model screen (the static path's own count), the kernel's eigen core (`modal_eigen::dense_path_peak_bytes`, 32 B per free n² entry), the Sturm phase (about 32 B per entry beside K_ff, M_ff and the shapes) and the formation accumulators. `DENSE_MODAL_BYTES_PER_ENTRY` is not defined. The eigen-core bytes and `dense_path_flop_estimate(n, s_max)` are recorded in the outcome as stated formulas: neither is a measurement or a safe product budget, and no work guard is applied, because the work budget (O-4) is unmeasured; its measurement needs a separate, bounded, exclusive authorization. I2b-1 recorded no timing.
- **As built (I2b-1): the pencil and the singular-model screen.** K is the static `form_basis_stiffness(&built, &springs, true)`; a `RangeDeferred` basis is refused by name (`MODAL_PENCIL_RANGE_UNSUPPORTED`), any other formation error as the static entry refuses it. K_ff and M_ff are the free rows and columns through `reduce_assembled_sparse_system`'s free DOFs and positions, on the prepared restrained DOFs (imposed-motion DOFs included, with no motion value) and a zero force. Before any eigen-solve K_ff passes the static dense path's structural solve on a zero force (the assembly and symmetry evidence `solve_preview_reduced_system` builds, then `SparseAssemblyEvidence::solve_assembled` in `DenseScrutiny` mode); a mechanism or singular model is refused there with the static integrity code (`NUMERICAL_INTEGRITY_*`). A bare `StructuralSystem` without that evidence was tried first and refused E-CANT-INCLINED-4's 2.2·10⁻¹⁶ formation skew, so the static evidence is required. Formed K positions whose triangles differ in their bits are counted and disclosed; K_ff takes the lower one, and the formation difference measures it.

### 3.3 Sparse path (M2): shift-invert block Lanczos on the existing LDLᵀ

**[decided]** (D-m)
- **Operator.** (K − σM)⁻¹M in the M-inner product, with σ = 0 unless the slicing below chooses otherwise; K's own factor, all pivots positive, is the common case.
- **Block Lanczos.** A scalar Lanczos run from one start vector spans at most one direction of each repeated eigenspace, so it cannot recover a repeated-mode subspace. A straight circular-pipe cantilever has exactly paired bending modes. F1 therefore uses **block Lanczos**:
  - **Block size.** b₀ = 2. Its basis is the paired bending modes of an axisymmetric section, the multiplicity the product's circular sections produce. b is an algorithm parameter, not an engineering default. A higher multiplicity, from a symmetric layout, is detected by the Sturm count (§3.4, check 4) and recovered by the restart below.
  - **Singular M.** M is only positive semidefinite (lumped bodies, massless rotational DOFs), so the M "inner product" is degenerate. Every basis vector is therefore kept in the range of T = (K − σM)⁻¹M, where M is definite on the finite eigenvectors (Ericsson & Ruhe 1980; Nour-Omid, Parlett, Ericsson & Jensen 1987): the start block and every replacement column are T applied to generator columns; and each Ritz vector x with Ritz value θ is recovered as the purified vector y = T·x/θ, whose residual is then checked (§3.4). The free space is exhausted when rank(M_ff) independent directions have been found.
  - **Deterministic start block.** b columns from a fixed-seed generator (the algorithm and the seed are stated in the code and recorded with the result), mapped by T, M-orthonormalized by Gram–Schmidt with one reorthogonalization, in fixed order. The start block is deliberately not the rigid translations r_k: they are M-orthogonal to every mode with zero participation, such as the antisymmetric modes of a symmetric layout, which exact arithmetic would then never find.
  - **Full M-reorthogonalization** of each new block against the whole basis, in fixed operation order. The projected block-tridiagonal matrix is solved with the M1b dense routine (Householder tridiagonalization and QL).
  - **Locking and deflation.** A Ritz pair whose residual backward error (§3.4, check 2) meets the convergence tolerance is locked. Later blocks are M-orthogonalized against the locked vectors, so a converged pair is never recomputed or perturbed.
  - **Breakdown.** If a column of a new block loses almost all of its M-norm to orthogonalization (below a stated relative threshold), the basis has found an invariant subspace or the block is rank-deficient. The column is replaced by T applied to the next column of the fixed-seed generator, orthogonalized against the basis and the locked vectors, and the event is recorded. **A failed replacement is not evidence of exhaustion** (review round 4: K = I₃, M = diag(1, 1, 10⁻¹²) gives replacement norms near 10⁻¹⁸ although rank(M) = 3). If no replacement survives, the run restarts deterministically as for a missed direction (below); if the restarts are exhausted, the set is refused (`MODAL_LANCZOS_BREAKDOWN`), never completed by assumption.
  - **Basis size.** q = max(2k, k + 8), rounded up to a multiple of b, grown if not converged, without thick restart in M2. The run also converges the next cluster above the reported set, which is not published, so that the final shift (§3.4, check 4) lies in a gap between computed clusters. The sparse path never uses the whole-finite-spectrum endpoint: a request that needs it (no computed cluster above the reported set) is refused on the sparse path (`MODAL_SPARSE_WHOLE_SPECTRUM_UNSUPPORTED`), and the dense path, which computes every finite eigenvalue, serves it.
  - **A missed direction.** If the Sturm count exceeds the number of converged modes, a direction was missed. The run restarts deterministically with block size 2b and the next seed of a fixed sequence. The number of restarts and the largest b are algorithm parameters stated in the code. If the counts still disagree when they are exhausted, the set is refused (`MODAL_STURM_COUNT_MISMATCH`). A count disagreement is never published.
- **Cut-off mode.** Spectrum slicing: repeat at shifts σ_j until a completed computed cluster lies above ω_c² (§1.4); the sizing count sizes q and the allocation only. Completeness is then decided by the final count of §3.4, check 4, at a shift below that cluster. Each slicing shift is placed in a gap between computed clusters, under the same separation premise as the final shift.
- **Built on `sparse_direct`'s existing public API, plus the read-only accessor that M1b adds:**
  - assemble K − σM over the union pattern;
  - apply the symmetric diagonal scaling D(K − σM)D, D = diag(|k_ii|^-½). This is a congruence, so it **preserves inertia**, and it makes the absolute 1e-12 pivot guard scale-free;
  - then `factorize_ldlt` and `solve_ordered`.
  - **[decided] The observation seam.** `ProfileFactorization` keeps its factor private (`sparse_direct/src/lib.rs:325-327`), and its report carries only the extreme pivot magnitudes and the negative-pivot count (`:119-131`), so max|L_ij| is not observable through the present API. F1 adds one narrow, **read-only** accessor to `ProfileFactorization`: borrowing iterators over the strictly lower factor's stored entries, row by row in ordered position, and over the pivots d_j, from which max|L_ij|, |L̂||D̂||L̂ᵀ| row sums and the growth factor are computed by the caller. It adds no mutation and no allocation. This is an **API addition to `sparse_direct`**, owned there by DEL-04-01, made in **M1b**, because the dense path's Sturm counts use it (§3.2, item 7). M2 uses it unchanged. `core/solver/sparse_direct/src` is a priced crate directory and its `lib.rs` is named in the pricing rules (`tools/retained_pricing/base/crate_dirs.txt`; `rules/o10_rules.py:3020`), so the change that adds it re-registers D3 (§3.6).
- **The unpivoted-LDLᵀ inertia risk is a first-class test target** (Agent 0, 2026-10-27; R2). `factorize_ldlt` does no pivoting, and on an indefinite K − σM element growth, or a shift too close to an eigenvalue, can corrupt the negative-pivot count. Nothing in binary64 makes the count exact; the count is relied on only under the separation premise of §3.4, check 4.
  - Mitigations, from M1b: place each shift in a gap between computed clusters, separated from both by a stated multiple of the estimated backward error of its factorization; read the growth (max|L_ij| and the |d_j| against the scaled diagonal) through the seam above; refuse a shift whose growth exceeds the stated limit, and try the next candidate position in the same gap. The growth limit and the separation multiple are algorithm parameters, fixed in M1b by the inertia-risk suite and recorded here (§9, O-6).
  - **Tests aimed at it** (M1b for the dense path's counts, extended in M2 to slicing shifts): models built to provoke growth (near-coincident eigenvalues, clusters, rigid-family scales 1e3–1e6, shifts placed close to eigenvalues), each checked against the reference generator's exact counts, and in M2 against the dense path within the dense guard, with the growth monitor's verdict and the separation decision asserted. They include Codex's round-2 witness: a 32×32 tridiagonal on which a shift within 10⁻¹⁷ relative of an eigenvalue rounds away and gives a wrong count with unchanged growth observations. The separation rule must refuse that shift, not count it.
  - If no shift can be placed under the premise, the affected set is refused by name, never published with an unchecked count.
- **[decided] Memory: the peak, counted.** At factorization, four P-entry value arrays are live together: K's profile, M's profile, the shifted and scaled profile, and its clone inside the factor (`factorize_ldlt` clones its input, `sparse_direct/src/lib.rs:386`). Each `SymmetricProfileMatrix` also carries three index arrays of about n entries (`:142-148`). The guard counts the peak live allocation of the phase that is largest, as the implementation states it:
  - B = max(B_fact, B_ref), with B_fact = 32P + 8n(q + b) + 24q² + c·n for the factorization and extraction phases and B_ref = 48P + 8n·q + c·n for the re-formation phase (K and M kept, the shifted profile and factor released, and the two double-double accumulators of §3.4 check 1 at 16 B per stored entry each); if the implementation keeps the shifted profile and factor alive during re-formation, B_ref = 64P + 8n·q + c·n. The implementation states which;
  - 32P (factorization): the four value arrays; 8n(q + b): the Lanczos workspace, basis and current block; 24q²: the projected problem and its vectors; c·n: the per-DOF metadata (the index arrays of four profiles, the factor's work vector and the scaling vector; about 112 B on a 64-bit target), with c stated by the implementation. Assembly transients (the entry list for K − σM) are counted if they outlive into the factorization phase, and otherwise form their own phase in the maximum.
  - Example: n = 60,000 DOF, mean half-bandwidth 60 (P ≈ 3.6·10⁶), k = 50, b = 2, q = 110: factorization about 115 MB of profiles, 54 MB of Lanczos workspace and 7 MB of metadata, about 180 MB; re-formation about 173 MB of profiles and accumulators plus 53 MB of basis, about 233 MB, which governs.
- **Time.** Factorization ≈ n·b̄² (≈ 2·10⁸ flops in the example), with b̄ the mean half-bandwidth; each block step ≈ b·(6P + 4nq) flops. Each Sturm check costs one more factorization.
- **[decided]** The sparse memory guard states B against a ceiling before allocation, in the same style as the dense guard.
  - **Admission is rechecked whenever q grows**, and whenever b grows on a restart, before the new allocation. A request is refused with the formula, never silently truncated.
  - **A cut-off-only request** is sized from the sizing count at the cut-off (§1.4), taken before Lanczos. That first factorization is admitted on its own term (32P + c·n). The count k_c then sets q.

### 3.4 Numerical integrity checks

**[decided] What the checks are** (D-n, Codex round 2). They are **numerical-integrity checks with stated premises**, not certified enclosures. Binary64 LDLᵀ inertia counts, a Rayleigh quotient on the formed pencil and one floating-point solve cannot enclose an eigenvalue without interval or exact arithmetic. Codex gave reproducible witnesses of each (§3.3; §9, O-13). Each check below therefore states what it computes, the premise under which it is relied on, and what it does not establish. This is the posture of the PF route's existing limitation: "numerical integrity does not establish physical correctness" (`preview_physics.rs:60`). No eigenvalue bound is certified, and no Kato–Temple refinement or inverse-norm bound is used. Rigorous enclosures are a later option (§9, O-13).

The checks re-form the model from authored operands, as K-D5 does (`frame_kernel/src/structural/formation_check.rs:1-12`), but the modal checks have criteria of their own. K-D5's criterion (D5C1) judges a static displacement correction against response scales (`formation_check.rs:972-995`). It is not an entrywise comparison of K, and a modal run has no static response, so it is not reused.

Notation: D = diag(|k_ii|^-½) of the formed K_ff (§3.3); a tilde marks a scaled quantity (K̃ = DKD, M̃ = DMD, φ̃ = D⁻¹φ); ‖·‖ of a matrix is the ∞-norm; u = 2⁻⁵³.

1. **[decided] Re-formation and formation difference** (D-o).
   - **Re-formation in double-double.** K and M are re-formed **independently from their leaves** in at least **double-double precision (≈106-bit significand)**. For K, the authored operands are the element geometry, sections and moduli, the bends' radius and plane reference, the connector and tie-rod stiffnesses, the spring stiffnesses and the rigid-family scaling. For M, they are OD, wall, mill tolerance, corrosion allowance (EX), metal, contents and insulation densities, insulation thickness, lengths, arc radius and angle, W, the centre of gravity and a. The binary64 matrices used by the solve are not inputs.
   - **Trusted leaves, stated.** K-D5's element re-formation (seam (i)) starts from the derived section properties (A, I, J) and moduli that the frame kernel uses, not from OD and wall. So that an error in the section derivation is also visible to the residual (review round 4: a doubled derived area would otherwise give a zero residual and a doubled λ), `modal_eigen` re-derives A, I and J from OD, wall, mill tolerance and, on EX, the D-5 nominal-wall basis, in double-double, and **K̂ is re-formed from those re-derived properties**, not from the kernel's: seam (i) takes the section properties, moduli and stiffness scales as supplied operands. The rigid-family products s·E and s·G are formed in double-double from s and the moduli. The section-derivation difference is also published beside the formation difference.
   - **Assembly.** The re-formation is element by element. The formation difference is measured on the **assembled** matrices: each re-formed element contribution is accumulated in double-double into one accumulator per stored entry of the formed K and M, and compared with the stored binary64 entry, so assembly rounding is included (exact element contributions 2⁵³ and 1 assemble to 2⁵³ in binary64). The accumulators' memory (16 B per stored entry) is counted in the guards (§3.2, §3.3).
   - **Why double-double, not true quad.** The re-formation's own rounding, and so the resolution of the measured formation difference, is at the 10⁻³¹ relative scale for well-scaled entries (a few units of 2⁻¹⁰⁶ over an element formula's few operations). The formation difference itself is at the 10⁻¹⁶ scale, so it is resolved to about 15 digits, and the residual thresholds of check 2 lie above 10⁻¹². A 128-bit or 113-bit significand would add nothing a check can use. Entries formed by heavy cancellation are less well scaled; the statement is for well-scaled entries, and the formation difference is measured, not assumed.
   - **Formation difference.** ΔK = K − K̂ and ΔM = M − M̂ (formed minus re-formed, assembled) are measured as ‖DΔKD‖ and ‖DΔMD‖, with row sums accumulated in double-double (as built in I1; `exact_sum` is not needed at that resolution). Connector and tie-rod stiffness leaves are their binary64 decode, as K-D5 re-forms them, and flexibility factors are taken as supplied binary64 values; those are stated trusted leaves. The re-formed element matrices are stored per element (about 1.2 KB per 12×12) and counted in the guards beside the 16 B per stored entry. They are published as evidence. A perturbation of the formed K or M away from its re-formation shows in check 2's residual, which is evaluated on the re-formed pencil.
   - **Seams it needs.**
     - **(i) K's element re-formation.** K-D5 already re-forms element stiffness from authored operands at p128, a 128-bit-significand correctly rounded software float (`intended_stiffness_pass`, `formation_check.rs:703`; `FORMATION_PRECISION = 128`, `:23`; `structural/retained/wide.rs`), inside a private module (`structural.rs:4`). That exceeds the double-double floor. M1b adds a read-only frame_kernel entry, a pure function of supplied operands, that returns re-formed element matrices for the elements K-D5 forms (straights, realized bends, connectors and tie rods) from the element geometry and the section properties, moduli and stiffness scales the caller supplies as wide operands (the re-derived ones above), without its D5C1 judgment. This is the only K-D5 code reused, and only where it applies. Springs and the rigid-family scaling are re-formed from their authored values in `modal_eigen`.
     - **(ii) M's re-formation.** It is new code in `modal_eigen`, in double-double built from binary64 error-free transformations (TwoSum, TwoProduct), deterministic on IEEE binary64 with round-to-nearest. The arc's trigonometric terms are evaluated to the same ≈106-bit target. **As built (I1):** M is re-formed with `modal_eigen`'s own double-double arithmetic, not frame_kernel's wide arithmetic.
     - A family that the re-formation does not cover refuses the set (`MODAL_REFORMATION_UNAVAILABLE`, naming the element). The check fails closed.
     - frame_kernel is priced, so (i) re-registers D3 in its change (§3.6).
2. **Residual and error estimate per mode.** For a computed pair (λ, φ), λ in binary64 and φ the binary64 vector:
   - **Residual on the re-formed pencil.** r̂ = K̂φ − λM̂φ is evaluated element by element from the re-formed element matrices, with double-double products accumulated exactly. Its scaled size is the normwise backward error of the pair for the re-formed pencil:
     η = ‖D r̂‖₂ / ((‖K̃̂‖ + λ‖M̃̂‖)·‖φ̃‖₂).
     The eigensolver's own error and the formation difference both show in η.
   - **Threshold.** η is compared with the stated residual threshold τ_r, which lies above 10⁻¹². A mode with η > τ_r is **demoted**, not refused: it is published Sensitive with its η, as the static path demotes. τ_r is measured under DEC-026 (§9, O-6).
   - **Error estimate.** The published relative eigenvalue error estimate is e_λ = η·κ_λ, with κ_λ = (‖K̃̂‖ + λ‖M̃̂‖)·‖φ̃‖₂² / (λ·φ̃ᵀM̃̂φ̃), the first-order condition number of a simple eigenvalue of a symmetric-definite pencil. **Premise:** first-order perturbation theory, with the eigenvalue isolated from the rest of the spectrum. For a cluster, e_λ is stated for the cluster as a whole (the largest over its members), not for each member. e_λ is an estimate under that premise, not a bound; it can understate the error where the premise fails, for example where a computed eigenvalue has a near neighbour that was not computed.
   - The check needs no M⁻¹ and no K⁻¹ norm, so it covers a singular lumped M.
3. **M-orthogonality.** max over i ≠ j of |φᵢᵀM̂φⱼ| / √(m_i m_j), over the published modes, evaluated in double-double on the re-formed M, is within a stated tolerance. It is invariant to the shape normalization (§1.5) and is evaluated on the published vectors. A breach demotes the result to Sensitive.
4. **Sturm-count agreement (completeness).** By Sylvester's law of inertia, in exact arithmetic the negative-pivot count (`nonpositive_pivot_count`) of K − σM is the number of eigenvalues below σ. The binary64 count is relied on only under the separation premise below.
   - **Where shifts go.** Counts are taken **only at shifts placed in gaps between computed clusters**: the final shift in the gap between the completed last reported cluster and the next computed cluster (§1.4), and, on the sparse path, the slicing shifts (§3.3). If the reported set is the whole finite spectrum, which only the dense path claims and only with an established finite-mode count (§3.2), the final shift is placed a stated relative offset above its largest eigenvalue, and the separation applies on the lower side only. If the reported set is empty (a cut-off below the lowest frequency), the count is taken at a shift in the gap between f_c and the lowest computed cluster and must be 0; the result states that no mode lies at or below f_c under the premise. The cut-off's sizing count (§1.4) is not a completeness count.
   - **Estimated backward error of a count.** For the computed factorization L̂D̂L̂ᵀ of Ã(σ) = K̃ − σM̃ (§3.3's scaling), the componentwise backward-error bound of unpivoted LDLᵀ gives Ã + E = L̂D̂L̂ᵀ with |E| ≤ γ_n·|L̂||D̂||L̂ᵀ| (Higham, *Accuracy and Stability of Numerical Algorithms*, ch. 10), γ_n = n·u/(1 − n·u). That bound covers only the matrix passed to the factorization, so the forming and scaling of the shifted matrix are added (review round 4: K = I₂, σ = 1.125 and a binary64 M whose exact K − σM is positive definite with determinant ≈ 1.1·10⁻²³ is miscounted as 1 with growth ≈ 1; the factor-only estimate admitted it): with Ã formed entrywise as fl(d_i·d_j·(k_ij − σ·m_ij)), |Ã − (K̃ − σM̃)| ≤ c_f·u·(|K̃| + |σ||M̃|) entrywise for a stated c_f covering the product σ·m_ij, the subtraction and the two scalings (c_f = 5 for that operation order, stated in the code). **Evaluation order and range premise** (review round 5): each entry is formed as a = fl(k_ij − fl(σ·m_ij)) and then scaled one factor at a time, fl(fl(d_i·a)·d_j), never by a precomputed d_i·d_j; the bound assumes every intermediate is a normal binary64 number or an exact zero; an operation whose **exact** result is nonzero but rounds to zero, or any subnormal or overflowing intermediate, is out of range, while exact cancellation (for example 1 − 1 = 0) is retained (review round 7 witness: K = [[2, 1], [1, 2]], M = [[0.25, 1], [1, 5]], σ = 1, a legitimate shift whose off-diagonal cancels exactly) (review round 6 witness: σ·m₁₂ = 2⁻¹²⁰⁰ rounds to zero while the exact scaled entry is −2⁻¹⁷⁸). A shift whose formation meets a subnormal or overflowing intermediate is rejected like one the pivot guard refuses, and the next candidate is tried (review witness: d² formed first underflows for entries near 10³⁰⁸). The count's backward-error estimate is e_b(σ) = γ_n·‖|L̂||D̂||L̂ᵀ|‖_∞ + c_f·u·‖|K̃| + |σ||M̃|‖_∞. The first term is computed from the factor through the observation accessor (§3.3) in two profile traversals with O(n) workspace (v = |L̂|ᵀ·1, then w = |L̂|·(|D̂|·v)), without forming the product. Both screens below use this e_b. The growth factor g = max(1, max_ij|L̂_ij|)²·max_j|d̂_j| / max_j|ã_jj| is still read and limited (below).
   - **Growth limit.** A factorization whose g exceeds the stated limit g_max is rejected, as is one that the 1e-12 pivot guard refuses. **Measured (I1):** g does not discriminate correct from wrong counts (it reaches 4·10¹³ on correct, well-separated rigid-scale gaps, above every observed miscount's g ≤ 7.7·10⁸), so g_max is set high and the factor term of e_b carries the growth effect; g is still published.
   - **The conditioning premise** (review round 3, finding 2). The inertia of a symmetric matrix is unchanged by a symmetric perturbation E with ‖E‖₂ < σ_min(Ã). The count at σ is relied on only when e_b(σ)·ν(σ) ≤ 1/κ_c, with ν(σ) an estimate of ‖Ã(σ)⁻¹‖₁ by the Hager–Higham 1-norm estimator (a few solves with the computed factorization) and κ_c a stated multiple (§9, O-6). This measures the distance of the whole shifted matrix from singularity, which the separation from neighbouring computed eigenvalues does not: the review's witness (K = diag(1, 1, B), M = diag(1/32, 1/256, 8B), B = HHᵀ with H the 40×40 unit lower bidiagonal with subdiagonal 2; forty eigenvalues at 1/8, then 32 and 256) miscounts 40 for 41 at σ = √(32·256) with a separation ratio of about 2·10¹¹ and modest growth, and is refused by this premise because B is ill-conditioned (as built: in the review's elimination order e_b·ν ≈ 159; in the witness's own numbering under RCM the pivot guard refuses it first). A shift failing it is refused like an unseparated one (`MODAL_STURM_SHIFT_ILL_CONDITIONED`, naming the gap and the estimate). The estimator can underestimate, so the premise is an estimate, not a certificate.
   - **The separation premise.** It is kept as a screen beside the conditioning premise. For a neighbouring computed eigenvalue λ_j with scaled vector φ̃_j, the separation of σ is
     s_j(σ) = (|λ_j − σ| − e_λ,j·λ_j)·(φ̃_jᵀM̃φ̃_j) / ‖φ̃_j‖₂²,
     the first-order size of a symmetric perturbation of K̃ − σM̃ that would move λ_j to σ, after allowing for λ_j's own error estimate (check 2). This is the gap's half-width, measured on the scaled pencil. The count at σ is relied on only when s_j(σ) ≥ κ_s·e_b(σ) for the nearest member of each neighbouring cluster, with κ_s a stated multiple (an algorithm parameter fixed in M1b, §9, O-6). The result discloses the premise, the shift, its separation ratio min_j s_j/e_b and g.
   - **Placement.** The first candidate is σ = √λ_lo·√λ_hi, the geometric mean of the gap's ends, formed so that it cannot overflow; norms and frequencies are formed with scaling likewise. If it is rejected (growth or pivot guard) or fails the premise, a stated finite sequence of other positions in the same gap is tried, at most s_max candidates in all. **As built (I1):** the candidates are dyadic positions in log scale between the gap's ends, formed by repeated √a·√b, never by a power; the whole-spectrum shift uses stated relative offsets above the top eigenvalue. Each candidate's evidence (σ, the formation and factor terms of e_b, ν, the product against 1/κ_c, the separation ratio, g, max|L|, max|d|, max|ã_jj|, the negative-pivot count and each premise's decision) is kept, and a refused set is named after the candidate that got furthest.
   - **Refusal, never publication.** If no candidate satisfies the separation premise, the affected set is refused (`MODAL_STURM_SHIFT_UNSEPARATED`, naming the modes on each side of the gap). If every separated candidate exceeds the growth limit, it is refused (`MODAL_STURM_GROWTH_LIMIT`, naming the gap and the largest g). A shift is never moved out of its gap to obtain a count.
   - **Rule.** The count at the final shift must equal the number of modes reported, cluster extensions included. A missed eigenvalue below σ makes the count larger, which is how a missed direction is detected (§3.3). A mismatch refuses the set (`MODAL_STURM_COUNT_MISMATCH`), after the sparse path's restarts (§3.3). A count disagreement is never published.
   - **What it does not establish.** Agreement shows that the count and the reported set agree under the premises. It does not prove that no eigenvalue is missing: the separation premise is first-order, ν is an estimate that can understate ‖Ã⁻¹‖, and the backward-error bound is evaluated in binary64. Product tests assert refusal of the shifts that binary64 miscounts (the round-2 and round-3 witnesses, and the scale witnesses that miscount at 10⁻¹³ relative offsets), not a particular count.
5. **Clusters.** Adjacent computed eigenvalues λ_i ≤ λ_{i+1} share a cluster id when λ_{i+1} − λ_i ≤ δ_c·λ_{i+1}. Membership is chained, so a cluster is a maximal run. That is the one cluster definition (§1.4; PRD §12.9; ScopeOfWork REQ-002). A straight circular cantilever has exact pairs.
   - δ_c is an algorithm parameter, fixed in M1b and recorded here. It is not an engineering default.
   - Two clusters whose error estimates overlap stay separate clusters by this definition. A boundary between them is a gap in which the separation premise of check 4, which subtracts each neighbour's e_λ, decides whether a shift can be placed; if it cannot, the set is refused.
   - The reported set is extended to complete a cluster at the boundary (§1.4).
   - Readers and references compare clusters as subspaces (principal angles), not as vectors. [design] VAL-REF-02 §10.4: "Cluster repeated or nearly repeated eigenvalues and compare the corresponding subspaces" (`docs/references/validation-programme/piping_solver_benchmark_research_and_agent_plan.md` at `d90e4ee0e5`, lines 669–682; catalogued at `validation/qualification/capability_inventory.json:70-82`).
6. **Weight-basis identity** (§2.5).

Standing:
- **Checks passed** when 1–6 hold within their tolerances and premises.
- **Sensitive** (published, demoted, as on the static path) for a mode whose η exceeds τ_r, or on an orthogonality breach, when nothing refuses the set.
- **Refused** on a Sturm mismatch, a final or slicing shift that cannot be placed under the conditioning and separation premises or within the growth limit, a Lanczos breakdown without recovery (§3.3), an uncovered re-formation, no finite modes (§1.5), a weight-basis mismatch, an acceleration mismatch or a component mass out of range (§2.3), or a non-SPD K.
- The tolerances and thresholds are measured and recorded under DEC-026, tighten-only.
- **Limitation text.** The profile's fixed limitations (§5.2) keep "numerical integrity does not establish physical correctness" and state what the checks do and do not establish: the residual is evaluated against a double-double re-formation of the stiffness and mass; the eigenvalue error is a first-order estimate; the Sturm count is relied on only at shifts where the estimated backward error of forming and factoring the shifted matrix is small against both its estimated distance from singularity and the shift's separation from the computed eigenvalues; and none of these is a mathematical bound on the eigenvalues, or a bound on modelling, discretization or input error.
- Product texts use the claims registry's numerical-health vocabulary and avoid "verified", "certified" and "guaranteed" (`docs/claims_registry.md`); "Sturm count" is the user-facing name of check 4.

**As built (I2b-1).**
- **Thresholds.** `ModalIntegrityParameters` carries every threshold and algorithm parameter as a required field: τ_μ, τ_m, c_μ, κ_μ, c_f, τ_e, the QL limit, τ_r, the orthogonality tolerance τ_o, δ_c, τ_t, κ_c, κ_s, g_max, s_max, the gap positions, the whole-spectrum offsets and the identity tolerance. It has no `Default`, no constructor with values and no production instance. Each value is checked finite and in its domain ((0, 1) for τ_μ, τ_m, τ_e, τ_r, τ_o, δ_c, τ_t and the identity's relative part; positive for c_μ, κ_μ, c_f; at least 1 for κ_c, κ_s, g_max; s_max and the QL limit at least 1; dyadic positions; non-empty positive offsets) and refused by name otherwise (`MODAL_INTEGRITY_PARAMETER_INVALID`, new). Only tests supply values, as one labelled `TEST_LOCAL_…` set taken from I1's test-local values where they exist (τ_r = 10⁻¹⁰ has no I1 value and is test-local only).
- **Scope of the checks.** Check 1 re-forms K and M from `BuiltModel`'s frames (section re-derived from OD and wall, moduli as used with the W-15 scale), realized arcs, component bodies, connectors and their tie rods and the springs. Check 2 runs on every computed pair, reported or not, band-recovered or above the band; a reported mode's on its published vector, any other's on its computed vector; all of it is kept. The cluster e_λ (the largest over the cluster) feeds the separation premise. Check 3 runs on the published shapes. A reported mode with η > τ_r is Sensitive; an orthogonality breach makes the set Sensitive; nothing else demotes, and every other failure refuses.
- **Codes.** Kernel refusals map to `MODAL_PENCIL_RANGE_UNSUPPORTED` (range, overflow or NaN of a checked or published value), `MODAL_REFORMATION_UNAVAILABLE`, `MODAL_NO_FINITE_MODES`, `MODAL_FINITE_COUNT_UNRESOLVED`, `MODAL_STURM_SHIFT_UNSEPARATED`, `MODAL_STURM_SHIFT_ILL_CONDITIONED`, `MODAL_STURM_GROWTH_LIMIT` and `MODAL_STURM_COUNT_MISMATCH`. A placement whose every candidate left the normal range in its formation (the kernel's `OutOfRange`) is refused as `MODAL_STURM_GROWTH_LIMIT`, as a shift the pivot guard rejects (check 4, "rejected like one the pivot guard refuses"). A dense-solve failure with no governed name (a non-SPD K_ff after the static screen, the QL iteration limit) is `MODAL_EIGENSOLVE_UNAVAILABLE` (new). Both new codes are adopted in this design (I2b-1).
- **Observations (TEST-LOCAL, debug build, aarch64-apple-darwin; nothing pinned).** Over the 26 class (a) cases of `f1_modal/mass_discrete` through the product path: η ≤ 5.5·10⁻¹³ on every computed mode; e_λ ≤ 1.6·10⁻⁸ (E-RIGID-SCALE-1e6); M-orthogonality ≤ 2.1·10⁻⁹; the final shift accepted at its first candidate in every case, with e_b·ν from 1.8·10⁻¹⁵ to 2.5·10⁻⁶, the separation ratio from 5.3·10⁵ to 5.6·10¹⁴ and g ≤ 1.6·10⁷; ‖DΔKD‖ ≤ 4.9·10⁻¹⁵ and ‖DΔMD‖ ≤ 2.4·10⁻²⁰. Every frequency agrees with the reference within the 10⁻⁹ seed except E-RIGID-SCALE-1e6's modes 1 and 2 (4.1·10⁻⁹ and 4.4·10⁻⁹, within the product's own e_λ/2 of 6.1·10⁻⁹ and 7.8·10⁻⁹: the binary64 formation of the rigid-scale pencil, with η ≈ 10⁻¹⁶). E-POINTMASS-AXIAL, -LATERAL and -GENERAL leave the census unresolved (an offset point lump), so their whole-spectrum endpoint is refused and a lower bound is served; E-POINTMASS-NODE is established at 21 with three condensed rotations. On EX n05 a bending pair at λ ≈ 5.42·10⁶ computes 1.4·10⁻⁸ apart, above the test-local δ_c = 10⁻⁸, with the upper member's e_λ ≈ 1.4·10⁻⁸ and η ≈ 1.9·10⁻¹⁰, so a bound stopping between them is refused (`MODAL_STURM_SHIFT_UNSEPARATED`) rather than counted. Its lowest mode (λ ≈ 1.2·10⁻⁴) has κ_λ ≈ 8.6·10¹⁰ and e_λ ≈ 9.8·10⁻⁶ with η ≈ 10⁻¹⁶. These observations may inform O-6; nothing is admitted on them.

### 3.5 Where the code goes

**[decided]**
- A new crate, `core/solver/modal_eigen`: dense reduction, tridiagonal QL, block Lanczos, the Sturm wrapper over `sparse_direct` with its shift placement and growth limit, the double-double re-formation and formation difference, residual estimates and participation. Std, plus path dependencies on `sparse_direct` and `frame_kernel` (`exact_sum` and the re-formation seam).
- **Seams in other crates**, each read-only and each owned by DEL-04-01: the `ProfileFactorization` observation accessor (§3.3, M1b) and the frame_kernel element re-formation entry (§3.4 check 1 (i), M1b). The unit-catalog families of §1.5 are DEL-02-02's (before M1b).
- A new module, `core/product_physics/src/modal.rs`: the request DTO, refusals, mass assembly, the weight-basis check and result rows. **As built (I2a):** `modal.rs` holds the codes and the `pub(crate)` submodules `modal/request.rs` (the closed request and its refusals), `modal/context.rs` (the post-lowering binding, D-L), `modal/prepare.rs` (the pre-formation mirror; its `Prepared` has no constructor outside `prepare_modal`, so nothing that takes it is reachable before preparation passes), `modal/mass.rs` (`mass_bearing_members`, the one selection; `assemble_mass`, held as 6×6 upper node blocks, dense only on demand; the mass summary rows) and `modal/weight_basis.rs` (the identity, the acceleration agreement and the non-blocking applied-case row). `lib.rs` gains only `mod modal;`, appended at its end so no snapshot-keyed line moves; `self_weight.rs` gains three line-preserving `pub(crate)` visibilities (`stable_mass_per_length`, `ComponentWeight` and its fields, `component_weights`) and `inspect_generated_self_weight` before its test module (D-SW); `core/product_physics/Cargo.toml` takes a path dependency on `open_pipe_stress_modal_eigen`. Nothing is public and no entry reaches it until I2b. Because the retained-pricing call graph resolves calls by name over the priced crate sources, the module's function names are chosen so that none is a call token elsewhere in them, and the request is parsed through a derived wire type with no hand-written `Deserialize` (serde entry points reach every custom one); the D3 analysis still decides reachability on its own graph.
- **As built (I2b-1):** `modal/stiffness.rs` (the pencil on the static partition, the static guard and the singular-model screen), `modal/eigen.rs` (the dense solve, the census, the reported set and its endpoint, the Sturm count), `modal/integrity.rs` (`ModalIntegrityParameters`, the re-formation operands, checks 1 to 3 and the code mapping, with the slice's codes as its constants), `modal/outcome.rs` (`ModalOutcome`, `run_modal_dense`) and `modal/tests_eigen.rs`; `modal.rs` gains only their declarations; `modal/tests.rs` makes its builders `pub(super)` and its reference converter falls back to the last node when every node carries a support (E-AXIAL-WITNESS). No priced file outside `modal/` changes, and every new function name is checked against the priced sources' call tokens. Nothing is public and nothing outside the tests calls `run_modal_dense`.
- **As built (I2b-2):** `modal/state.rs` (the state case's same-run solve, the admission of its evidence and the linearization, §1.3) and `modal/tests_state.rs`; `modal/prepare.rs` drops the interim refusal and keeps the state case's resolution and basis record; `modal/stiffness.rs` forms the pencil on the restrained set its caller passes; `modal/outcome.rs` records the state. In PP `lib.rs`, the decision-B seam (§1.3, as built): one field on `LoadCaseSolve`, a third return member of `append_nonlinear_support_loop_results`, the capture on the completed construction, and three types and one helper appended at the end of the file. The new names (`NonlinearStateEvidence`, `NonlinearQualification`, `NonlinearStateStanding`, `capture_nonlinear_state_evidence`, `nonlinear_state`, `solve_modal_state_case`, `admit_modal_state_evidence`, `linearize_modal_supports`, `ModalStateCase`, `ModalSupportState`, `ModalSupportTreatment`, `StateCaseRefusal`, `MODAL_DENSE_PATH_MODE`) occur nowhere else in the priced sources. `modal/state.rs` is the first modal caller of a static solve function (`solve_load_case_observed`); it is reached only from `run_modal_dense`, which nothing outside the tests calls.
- **As built (I3a):** `modal/document.rs` (the projection, §5.3; `pub(crate)`, called only by tests) and `modal/tests_document.rs`; `modal/outcome.rs` keeps the payload fields; `modal.rs` gains the two declarations. The Rust reader is the new leaf crate `core/reporting/modal_result` (unpriced: it is not in `crate_dirs.txt` and depends on no priced crate's code beyond `canonical_json` and `units`), the Python reader `tests/verifiers/modal_evidence.py`. Every new function name in the priced sources has the `modal_doc_` prefix or is `modal_semantic_table`/`modal_result_document`, none a call token elsewhere in them.
- A public entry, `run_modal_preview_with_mode`, beside `run_linear_static_preview_with_mode` (PP `lib.rs:2580`).
- Mass is formed in a **separate pass over `BuiltModel`**, never inside `build_model_for_members` or `build_member`. [design] `build_member` is a 50,544 B frame in the measured AC-004 peak stack chain (DEL-04-08 `Work/stack-headroom-repair.md:30-45`), so growing it would risk the R/64 headroom witnesses.

### 3.6 Retained-memory pricing (D3) and AC-004

- **[design]** AC-004 and REQ-004 govern **retained-precision invocations through the Direct entry** only (DEL-04-08 `ScopeOfWork.md`, REQ-004). "Any change to a priced source or to a reviewed input requires a new actual-source D3 audit".
- **[decided]** F1 is a **separate route that never enters D1 or the retained Direct entry**, so it needs no retained admission and no new D1 premise.
- **It still triggers D3**, because it touches priced sources and one reviewed input:
  - PP `lib.rs` (`mod modal;` and the entry). [design] W-16's one-line `mod imposed_motion;` moved every snapshot-keyed line and its full-file pins (`retained-pricing-rules.md` §8.23).
  - `self_weight.rs`, to expose `stable_mass_per_length` and `component_weights` as `pub(crate)`. Its full file is pinned in `b7a_formation_model.py` (§8.23).
  - **`core/product_physics/Cargo.lock`**, if product_physics depends on the new crate. [code] The lock is reviewed input #1 (`build_identity.rs:158-159`), so a new dependency makes the registered profile `Stale` until re-registered.
  - `core/reporting/result_export/src/semantic_contract.rs`, if the modal table's constants go there (literal-only, §8.23).
  - **`core/solver/sparse_direct/src/lib.rs`**, for the observation accessor (§3.3), in M1b. Its crate directory is priced and the file is named in the pricing rules (`tools/retained_pricing/base/crate_dirs.txt`; `rules/o10_rules.py:3020`). The accessor is priced because `sparse_direct` is.
  - **`core/solver/frame_kernel/src`**, for the re-formation entry (§3.4, check 1 (i)), and the wide arithmetic if it is exposed, in M1b.
  - **`core/units/src`**, for the unit-catalog families (§1.5): carried and priced by model document 0.9.0's S1 in its D3 turn, not by F1.
  - **PP `lib.rs`, the I2b-2 seam** (§1.3, as built). It changes the bodies of `solve_prepared_load_case` and `append_nonlinear_support_loop_results` and the `LoadCaseSolve` type, so the function-body pins on them move, and `LoadCaseSolve`'s size moves 1,880 → 1,888 B, which enters B5O1's CasePrefix carrier `2·s(Result<LoadCaseSolve,FrameKernelError>)` (DEL-04-08 `retained-pricing-rules.md` §5.2). On the retained route D1.6 refuses every nonlinear support, so the loop's early return and the capture's `None` are the only paths reached there; the new helper adds one call edge from the case solve. The edits preserve every line before the appended block. D3 decides the re-pricing; no pin is written here.
- **I3a** (§5.3). Priced change: `core/product_physics/src/modal/` only (`document.rs` new, `outcome.rs`'s payload fields and their copy, `modal.rs`'s two declarations, the tests); no `lib.rs`, `self_weight.rs`, lock or other crate changes, and no MODAL-FB body moves. The reader crate is outside the priced directories. One preparation on the final head (`084ae2036`, after an earlier one on the abandoned `result_export` placement stopped at exit 4 on that crate's W1 stage-2 `lib.rs` pin): exit 5, every point incomplete only for **five unclassified TEXT arguments**, all in `document.rs` (`DIRECTIONS[k]` at two sites, the row `ordinal`, the run id's `run.get(..16)`, a mass row's DOF id `a`, the candidate index `j + 1`); no unmapped header, no pin or premise stop; D and TAV equal §8.48's at every point (D: L(3,0) 43,783, L(2,1) 39,369, L(1,2) 35,231, E 76,356). The coefficient comparison needs the classification first; the expectation, to be shown by D3, is no coefficient movement (no retained root reaches modal). Nothing is registered here; the D3 is DEL-04-08 §8.52 (all 1,602 coefficients equal on the composition with PF1b and #109).
- **Not touched:** the preview-physics-2 table (reviewed input #5, `build_identity.rs:166`); F1 adds its own table, which is not a reviewed input. `results.v0.3.schema.yaml` is not a reviewed input (`build_identity.rs:158-181`).
- **Expectation, to be measured:** modal code is unreached from the retained Direct entry, so the priced tree should move only by line shifts and pins, coefficient-identical as W-16's was. The AC-004 stack witnesses are re-run if the D3 moves them.
- **[decided]** The first D3 is done in the first PR that touches priced files (M1a). The units extension, the frame_kernel seam and the sparse_direct accessor (both M1b) each re-register D3 in the change that makes them, or join a neighbouring slice's D3 when they merge together. M2 is expected to touch no priced file beyond what M1b has registered; if it does, it re-registers D3 in that change. Other slices stay out of priced files where possible. D3 runs join the queue on Agent 0's go (§6).

---

## 4. Validation references

### 4.1 The independent-reference rule

- **[design]** References are "derived and frozen by a TASK that neither reads nor writes product code, then refuted by a second TASK, before any product output is compared with it" (`pf-timoshenko.md`, #1663). Existing example: `validation/references/w15_component_weight/` ("The generator imports, reads and runs no product code. All model data are invented.").
- **[decided]** For F1, `validation/references/f1_modal/`:
  - a Python generator, standard library only;
  - exact rationals of the binary64 inputs (`fractions`), and transcendental roots by bisection or Newton in `decimal` at about 50 digits;
  - its own element K and M from the published formulas, its own generalized eigen-solve and its own Sturm counts;
  - frozen JSON and `SHA256SUMS`, then refutation by a second TASK.
- **Reference classes:**
  - **(a) Formulation-matched discrete references**: the same discretization as the product, compared at the analytic class tolerance, seeded at 1e-9 relative for frequencies (DEC-026).
  - **(b) Continuum references**: compared at a stated convergence tolerance across mesh refinement. They show the element converges at the expected rate; they do not check an individual mode.
  - **(c) Published piping-system benchmarks** (§4.4): comparison evidence at a stated tolerance, not exact answers.
  - Mode shapes are compared by MAC or subspace angle at common nodes (VAL-REF-02 §10.4).

### 4.2 Closed-form beam frequencies (class b, and class a with the matched discretization)

The generator re-derives every formula and root below; they are stated from memory here.
- **[decided] Which continuum the product converges to** (review round 3, finding 17). The straight mass includes Rayleigh rotary inertia (§2.2), so on refinement the product converges to the **Rayleigh** continuum, from above in the Rayleigh–Ritz sense; the Euler–Bernoulli frequencies below are reached only in the slender limit. Product convergence references are therefore Rayleigh (simply supported in closed form; cantilever and fixed-fixed by their boundary determinants, as frozen in `f1_modal/continuum`). The Euler–Bernoulli formulas serve as kernel controls with ī = 0. Rings likewise: the half-arc lumps include section rotary inertia (§2.4), so the matching ring references include rotary and torsional inertia; Hoppe's and Love's formulas neglect both and are slender-limit controls. The ring's n = 0 uniform-twist mode is counted.
- **Euler–Bernoulli, ω_n = (β_nL)²·√(EI/(m̄L⁴)):**
  - cantilever, cos βL·cosh βL = −1: β_nL = 1.875104069, 4.694091133, 7.854757438, 10.995540735 ([design] the first also at VAL-REF-02 line 290, `AN-MODAL-02`);
  - simply supported: nπ;
  - fixed-fixed, cos βL·cosh βL = 1: 4.730040745, 7.853204624, 10.995607838.
- **Axial and torsional rods:** cantilever axial ω_n = ((2n − 1)π/2)·√(EA/(m̄L²)); torsional ω_n = ((2n − 1)π/2)·√(GJ/(j̄L²)). These isolate D-e: contents are in m̄, not in j̄.
- **Rotary inertia (Rayleigh) and Timoshenko, simply supported**, α = nπ/L: (κGAα² − m̄ω²)(EIα² + κGA − īω²) − (κGAα)² = 0. The Rayleigh limit (κG → ∞) is ω² = EIα⁴/(m̄ + īα²). The Timoshenko cantilever uses its transcendental frequency equation (cite Han, Benaroya & Wei, *J. Sound Vib.* 225(5), 1999; derive, do not copy tables). Rayleigh cases are in M1; Timoshenko cases belong to M4.
- **Lumped-mass controls (class a):** a single DOF, f = √(k/m)/2π (VAL-REF-02 line 289, `AN-MODAL-01`); a point mass on a massless cantilever, ω = √(3EI/(M_tL³)), on the kernel directly; a fixed-free axial chain of N equal lumped masses, ω_j = 2√(k/m)·sin((2j − 1)π/(2(2N + 1))).
- **Point mass on a cantilever with distributed mass:** 1 + cos βL·cosh βL + (M_t/(m̄L))·βL·(cos βL·sinh βL − sin βL·cosh βL) = 0. In the product the tip mass is a rigid-family component weight with a = the request acceleration, which exercises §2.3. Variants: centre of gravity at the node; offset along and across the axis (rotary terms m·[r]×ᵀ[r]×); and the fall-back case, where an ineligible component keeps the pipe mass and emits the warning.

### 4.3 Rings and arcs

- **Class a.** An L-line, a Z-bend and a U-loop with realized arcs, with flexibility factors of 1 and other than 1, against the generator's own Castigliano arc stiffness and half-arc rigid lumps. The arc stiffness is the curved-bend energy N²/EA + T²/GJ + k(|M|² − T²)/EI (one k for both bending planes, no shear), as the frozen `t4_u4a` and `t4_i7` references state it.
- **Class b.** A free complete ring of n_b bend macro-elements: in-plane inextensional ω_n² = (EI/(m̄R⁴))·n²(n² − 1)²/(n² + 1) (Hoppe, 1871); out-of-plane ω_n² = (EI/(m̄R⁴))·n²(n² − 1)²/(n² + EI/GJ) (Love, *Treatise on the Mathematical Theory of Elasticity*). The ring is free, so this is a **kernel-level test** with a negative shift (K − σM SPD for σ < 0), not a product run. It shows lumped arcs converging as n_b grows.

### 4.4 Piping-system benchmarks and their provenance

- **[design]** The repository catalogues them as `NRC-DYNAMIC`: "NUREG/CR-1677 volumes 1/2, CR-6414, CR-6645; ANSYS descendants", readiness `catalogued_unadmitted` (`validation/qualification/capability_inventory.json:2523-2535`). NUREG/CR-1677 Vol. 1 (uniform support motion, with reference frequencies and participation factors) and Vol. 2 (independent support motion) are BNL problem sets for the NRC; NUREG/CR-6414 has three AP600 problems; NUREG/CR-6645 covers modal-response combination (an F2 concern).
- **[design]** NUREG/CR reports are prepared by contractors (for these, BNL) for the NRC. NRC sponsorship does not by itself make a contractor-authored report a US-government work. `docs/IP_AND_DATA_BOUNDARY.md` §1 requires human or legal review "when redistribution rights or protected-content status are uncertain", and §4 holds a record with `source_license = TBD` or `redistribution_status = unknown` until a maintainer records a disposition.
- **[decided]** (D-i) Agent 0, 2026-10-27, revising the earlier premise:
  > "Confirmed: the per-report condition. My premise was too broad. NRC sponsorship doesn't by itself make a contractor-authored NUREG/CR a US-government work. So:
  > - Transcribe from a report only with that report's own redistribution evidence (its rights statement and NRC distribution terms), recorded per fixture.
  > - Hold any report with uncertain rights for owner review, per IP_AND_DATA_BOUNDARY.
  > - Prefer sources whose rights are clear: original closed-form benchmarks we derive ourselves, and textbook formulas restated in our own words with citation.
  > - FR-SOL-016 stays partial until at least one published piping benchmark is compared with that evidence. Agreed."
- **Transcription is conditional on report-specific evidence.** Nothing is transcribed from a report until its own redistribution evidence is in hand: the report's rights statement (its front-matter notice, including any copyright or contractor-rights notice) and the NRC's distribution terms for it. A report whose rights are uncertain, or whose notice reserves rights, is **held for the owner's review** under `docs/IP_AND_DATA_BOUNDARY.md`, and nothing is transcribed from it in the meantime. Content inside a report that is marked as copyrighted or adapted from another source, such as figures adapted from journals, is never transcribed.
- **Per-fixture provenance record.** Each transcribed problem is committed with a `PROVENANCE.json` beside its fixture under `validation/references/f1_modal/published/<source>/<problem>/`, carrying the `docs/IP_AND_DATA_BOUNDARY.md` §4 fields:

  | Field | Content |
  |---|---|
  | `source_name` | Report number, volume, title and the problem's own identifier |
  | `source_location` | The report's NRC accession number (ADAMS) or NTIS/OSTI identifier, and the pages and tables transcribed |
  | `source_license` | The redistribution basis this report's own evidence supports, with its rights statement quoted from the front matter and the NRC distribution terms cited. `TBD` while the evidence is incomplete, in which case nothing is committed |
  | `contributor` | The transcribing role (TASK) and its PR; no personal identity |
  | `contributor_certification` | That only problem inputs and reference values were transcribed, and that no figure, table or text marked as copyrighted or adapted from another source was copied |
  | `redistribution_status` | `public_permissive` only when the report's own evidence supports it; otherwise the report is held and no fixture exists |
  | `review_status` | `pending`, the governing vocabulary's value (`docs/IP_AND_DATA_BOUNDARY.md` §4: pending, accepted, rejected, quarantined), until the maintainer records a disposition; agents never set another value |
  | `transcription_verification` | A separate field (D-u): an independent Type 2 reviewer verifies the transcription against the source and records agent verification, naming the reviewer role, the source pages and the date. It is evidence, not acceptance. If the rights are unclear, the fixture is not committed and the report holds for the owner |

  The record also lists what was **excluded** as copyrighted or adapted, with its location in the report, so a reviewer can see the line drawn.
- **Use.** NUREG/CR frequencies are numerical results of other programs, not exact answers ("NRC and ANSYS outputs retain shared lineage", `capability_inventory.json:2530-2533`). They are class-c comparison evidence at a stated tolerance. A report's participation factors may use another shape normalization, so they are compared through normalization-invariant quantities: effective masses, or Γ·φ.
- **Other sources:**
  - **Preferred, rights clear:** closed-form benchmarks derived in this repository (§4.2, §4.3), and textbook formulas restated in our own words with citation.
  - **Code_Aster** SDLX02 (Hovgaard modal) and SDLL14 (elbow frequencies) (VAL-REF-02 lines 94–95): GPL-3.0-or-later code; usable for independently implemented fixtures, not by vendoring, with the redistribution evidence for the transcribed data recorded per fixture as above.
  - **ASME PVP benchmark papers and NAFEMS natural-frequency benchmarks:** licensed or paywalled. Cite only; copy no data.
  - **CAEPIPE:** no outputs are copied. Any comparison is the owner's private exchange.
- **[decided] The published-benchmark obligation** (D-q). PRD §12.9 requires comparison with published piping benchmarks.
  - **Criterion.** Before FR-SOL-016 is reported complete, at least one published piping benchmark with recorded redistribution evidence is compared with the product, and the comparison is recorded as **comparison evidence, not exact verification**.
  - **Prerequisite and timing.** The per-report evidence above is the prerequisite. The comparison comes after M1b (the dense path).
  - **Until then**, FR-SOL-016 stays partial (`docs/COMPLETENESS_MAP.md`). M1–M3 rely on classes a and b. The first class-c target is one NUREG/CR-1677 Vol. 1 problem, whose modal part needs only frequencies and participation, if its report's evidence supports redistribution. Otherwise it is another published piping benchmark whose evidence does.
  - **Status of the first candidate (2026-10-31).** NUREG/CR-1677 Vol. 1 (BNL-NUREG-51267, August 1980; OSTI 6403318), Problem 1, is recorded with `source_license = TBD` and `redistribution_status = unknown` and held for the owner's review (O-3). Its notices mark distribution as unlimited and carry no copyright or contractor-rights notice, but they grant no licence, and the report is mostly contractor-authored. Its Problem 2 is adapted from the literature and is excluded regardless. Vol. 2, NUREG/CR-6414 and Code_Aster SDLX02 are held or unsuitable on their own evidence. Nothing is transcribed until the owner rules. A search for a problem by US government employees found one candidate, also held for the owner's ruling: D. H. Peters, "Shock Loads on Piping Systems" (Naval Postgraduate School thesis by a serving US Navy officer, 1972; DTIC ADA751643). It is marked "approved for public release; distribution unlimited" and carries no copyright notice; its §105 basis is inferred from authorship. Its 3-D, 12-element model has square corners and no bends, consistent mass, frequencies and participation factors. It is recorded with `source_license` TBD and `redistribution_status = unknown`.

---

## 5. Desktop UI, result schema and readers

### 5.1 Where results appear (M3)

- **[code]** The viewport's deformed shape is a display transform of result rows. `buildDeformationOverlay` reads `displacement_magnitude` and `global_nodal_displacement_{x,y,z}` rows, normalizes by the maximum to a display offset of 0.65 and labels the boundary `scale=normalized_display_offset_not_physical_length` (`apps/desktop/src/features/viewport/deformationOverlay.ts:20-116`). It is consumed in `PipeViewport.tsx:618` (three.js, `:7`). No animation loop exists in `PipeViewport.tsx` today.
- **[decided]**
  - **Request entry.** The modal request form takes the extraction bound with no pre-set value in a new project. The desktop remembers the last value used per project and pre-fills it; the user sees and may change it before solving.
  - **Modes table** in `features/results/` (a new `ModalResultsPanel.tsx` beside `ResultsPanel.tsx`). Columns: mode; f (Hz), with T (s) and ω (rad/s) in project display units; effective-mass fractions X/Y/Z and cumulative; cluster; standing (checks passed or sensitive, with the residual backward error and error estimate).
  - **Header lines:** the Sturm count ("N modes up to the checked shift; N reported"), with its separation premise, the extraction bound, which bound stopped extraction and any cluster extension, the modulus basis, the weight-to-mass acceleration, total and free-DOF mass, and disclosures (massless connectors and tie rods, constant-effort supports, mass at restrained DOFs, contents excluded from rotary inertia).
  - **Mode-shape view.** A new `features/viewport/modeShapeOverlay.ts`, reusing the 0.65 display offset and the `scale=…not_physical_length` boundary wording. Positions are x₀ + sin(phase)·offset·φ̂, with φ̂ the published shape (case (a), unit maximum translation, §1.5); a case (b) mode (unit maximum rotation, no translation) is shown by its nodal rotations, labelled as such. It is animated by a viewport-owned loop that runs only while a mode is shown. Under `prefers-reduced-motion`, or by user choice, it shows the static extreme shape with a phase step control (DEL-07-06). Display-only Hermitian interpolation along straight members from nodal rotations is optional (Euler–Bernoulli only).
  - **Report section** "Modal analysis" (`core/reporting/report_sections`; the desktop's `features/report/`): request and basis, mass summary, modes table, integrity evidence, refusals and disclosures, limitations. No claim language beyond the existing fences.
  - **Survey A6** (key-results panel) gains frequencies once F1 exists, in A6's own slice.

### 5.2 Result schema and semantic contract (M1b)

**Superseded in part by §5.3 (I3a).** The profile and its two limitations below stand and are published by §5.3's family. The `results.v0.3` and `analysis_run.v0.3` additions, the table identity `…/0.3.0/modal-1`, the readers' placement in `result_export` and the desktop and headless commands are a **superseded proposal**: the modal result is a family of its own (§5.3.1), its run recording is open policy (§9 O-8), and the native and desktop path is §5.3.10's later slice. The text below is kept as the record of the proposal.

- **[code]** Result profiles today: `formulation_basis.profile_id` ∈ {`product_preview_mechanics_v1`, `exact_pressure_v3`, `product_preview_retained_w1a_v2`, `exact_straight_retained_w1a_v2`} (`schemas/results.v0.3.schema.yaml:1035-1041`). Semantic tables are identified as at `core/reporting/result_export/src/semantic_contract.rs:126-128` (`…/preview-physics-2`).
- **[decided] Additions:**
  - **Profile:** `product_preview_modal_v1`, with two fixed limitations.
    - Limitation 0: "Small-displacement, linear-elastic Euler-Bernoulli frame modal preview about the unloaded configuration, with any nonlinear supports at the stated state of a named solved case, with consistent mass on straight members and lumped rigid-body mass on realized arcs and component bodies; numerical integrity does not establish physical correctness."
    - Limitation 1: "The integrity checks are numerical estimates under stated premises, not mathematical bounds: each mode's residual is evaluated against a double-double re-formation of the stiffness and mass, its eigenvalue error is a first-order estimate, and the Sturm count is relied on only at shifts where the estimated backward error of forming and factoring the shifted matrix is small against both its estimated distance from singularity and the shift's separation from the computed eigenvalues. They do not bound modelling, discretization or input error."
  - **Table:** `openpipestress.result_semantics/0.3.0/modal-1` (`fixtures/results/semantic_contract_v0_3_modal_1.json`, new, not a reviewed input). Row kinds:
    - `modal_frequency`, `modal_circular_frequency`, `modal_period`;
    - `mode_shape_{ux,uy,uz,rx,ry,rz}` (normalized per §1.5, each mode carrying its normalization case);
    - `modal_generalized_mass`;
    - `modal_participation_factor_{x,y,z}`, `modal_effective_mass_{x,y,z}`, `modal_effective_mass_fraction_{x,y,z}`, `modal_cumulative_effective_mass_fraction_{x,y,z}`;
    - `modal_mass_total_by_category`, `modal_mass_free_dof_{x,y,z}`;
    - `modal_residual_backward_error`, `modal_eigenvalue_error_estimate`, `modal_formation_difference`, `modal_orthogonality_deviation`, `modal_sturm_count`, `modal_sturm_shift` (with e_b, ν, the conditioning product and κ_c, the separation ratio, the growth factor and each premise's decision).
    - Every row's unit is a catalog unit with integer exponents (§1.5).
    - Each row carries `basis_ref {ref_type: "mode", ref_id: "mode:<i>"}` and, where one exists, `cluster_ref`.
  - **Run kind** (superseded proposal, open policy: §9 O-8): a `modal` analysis run in `schemas/analysis_run.v0.3.schema.json`, so stored results keep the request (extraction bound, modulus basis, a). Not implemented or adopted; §5.3's document binds the request itself.
  - **Readers:** a Rust `core/reporting/result_export/src/modal_evidence.rs` and a Python mirror under `tests/verifiers/`, sharing no code. They check the profile and limitation against the request; re-derive the free-DOF and total masses from the model (§2) and check Σ_i m_eff,ik ≤ T_k = R_kᵀM R_k; check each mode's normalization case and sign (its normalizing component is +1; in case (b) every translation is at most τ_t·ℓ), and that Γ_ik² m_i = m_eff,ik; and check the counts, ordering, cluster structure and any cluster extension. They do not re-run the eigen-solve (the DEF-O stance, DEL-04-08 REQ-005).
  - **Desktop and headless:** a Tauri command beside the static one (`apps/desktop/src-tauri/src/lib.rs:44`, `:1904`), TS types, and a headless runner command (`core/runner/headless`).

### 5.3 The modal result family (I3a, experimental)

**Status.** Agent 0 released I3, starting with I3a: the dense modal result contract, its full Rust reader and an independent Python reader, with fixtures (2026-11-03). Built and tested with labelled test-local parameters only. **Public engineering-result admission and every UI stay disabled** (§5.3.9). Nothing here adopts the joint task record, a store, a version policy, an `analysis_run` kind or recording support; no native, worker, Tauri, TS or UI code changes. Agent 0's binding scope for I3a governs where this section is silent.

#### 5.3.1 Identity and placement

- **[decided] A family of its own.** The document is not a `MechanicsEnvelope`, and nothing is added to `results.v0.3` or `analysis_run.v0.3` (§5.2's results.v0.3/analysis_run additions are **superseded**, §9 O-8). Wire identifiers:

  | Member | Value |
  |---|---|
  | `document_kind` | `openpipestress.modal_result` |
  | `schema_version` | `0.1.0` |
  | `semantic_contract` | `{id: "openpipestress.result_semantics/modal-1", sha256}` |
  | `profile` | `{profile_id: "product_preview_modal_v1", limitations: [§5.2's two texts, byte-equal]}` |
  | `canonical_profile` | `openpipestress_jcs_ijson_v1` (every hash is the SHA-256 of the checked canonical text) |
  | `admission` | `{status: "experimental", public_engineering_result: false, parameter_standing: "test_local", parameter_label}`; 0.1.0 cannot express a public result |
  | `run` | `{run_id: "modal-run:" + the first 16 hex of SHA-256({request_sha256, integrity_parameters_sha256}), mode_id_scope: "run_local", solver_path: "dense_scrutiny"}` |
  | `status` | `solved` or `refused` |

- **[decided] Table bytes.** modal-1 is `fixtures/results/semantic_contract_modal_1.json` (16,798 B as committed after the units slice, pretty JSON with a final newline; its checked canonical text's SHA-256 is `21d0012729d8466436d0c064d7edecbdc0fc481bf101b6514a65eecbee6fb23b`). It lists every value kind with its table, its unit selector (`fixed`, `normalization_case` or `dof_pair_class`) and, per selector key, its unit rule, `{unit_status: "catalog", unit, symbol, dimension}` for every kind. The former `catalog_family_absent` form, with `dimension_exponents`, is retired (§5.3.5). It also carries the closed list `equilibrated_basis_kinds` (empty, §5.3.5); a rule `{unit_status: "equilibrated_basis", unit: null}` may be stated only for a listed kind. The producer and each reader state the table independently, and each is held to these bytes. The earlier name `…/0.3.0/modal-1` and `semantic_contract_v0_3_modal_1.json` (§5.2) are not used.
- **[decided] Placement.** The projection is `core/product_physics/src/modal/document.rs` (`modal_result_document`, `pub(crate)`), because `ModalOutcome` is crate-private; nothing outside the tests calls it. The Rust reader is a leaf crate of its own, `core/reporting/modal_result` (`open_pipe_stress_modal_result`, dependencies: `canonical_json`, `units`, `serde_json`, `sha2`), as the hanger-sizing family's reader is. Reasons, on merit: the family is deliberately not the `results.v0.3` envelope that `result_export` reads; the reader is reached from no retained root, so placing it in `result_export` would add about 190 format sites and six name-resolved `max`/`min` call edges (the MODAL-FB pattern, §8.43) to a priced crate whose `lib.rs` is pinned by the W1 stage-2 owner model, enlarging the retained proof surface with code the retained route never runs (the first preparation, on that placement, stopped at that pin); and the reader needs nothing from any product crate. The Python reader is `tests/verifiers/modal_evidence.py`. The producer and both readers meet only at committed bytes.

#### 5.3.2 The document

- **Only in-run typed evidence.** Every value is copied from the `ModalOutcome` this run returned, or derived from it by a stated function: f, ω, T of a computed mode by the kernel's own `frequency_from_eigenvalue`, and ω of a shift or gap end as √σ. `ModalOutcome` gains the payload the projection needs, copied from the run's own formed values: the node ids and canonical coordinates, the free DOFs, the formed global mass as its stored nonzero upper entries (the binary64 values the pencil, T_k and p_k were formed from), ℓ, τ_t and the request's normalized a. No header, stored result or current authority is read.
- **Solved documents** carry `basis` (route, τ_t, ℓ, a, the identity's acceleration, the mass centre, the state case with its linearization), `extraction` (the bound, the stop, bounded and extension counts, the count, the computed count and the endpoint), `census`, `standing`, `sturm` (count, reported, placement with its gap ends as ω, κ_c, 1/κ_c, κ_s, g_max), `orthogonality`, `formation`, `guard` (the stated formulas as decimal strings) and fifteen tables: `nodes`, `dofs`, `mass_matrix`, `mass_totals`, `modes`, `shapes`, `participation`, `clusters`, `computed_modes`, `sturm_candidates`, `identity`, `supports`, `discretization`, `disclosures` and `diagnostics`. **Refused documents** carry the identity, the binding and `refusals` (every blocking diagnostic of the run, its code, message and references). An unparsable request is refused as `MODAL_REQUEST_INVALID` (new in I3a, for the document only).
- **Rows.** Every row has a `row_id` unique in the document, its source `ordinal` (its position in the run's own sequence: node order, global DOF order, node-block order of the mass, mode, computed-mode and candidate order; equal to its position in the table) and a `checksum`, the SHA-256 of the row's checked canonical text without its checksum. A table is `{status: "published", row_count, digest, rows}`, the digest being the SHA-256 of the canonical array of its rows' checksums. A row is a record of named scalars, never an array packed as one value. Ids: `node:<id>`, `dof:<node>:<ux…rz>`, `mass:<dof>|<dof>` (row ≤ col), `mass-total:<x|y|z>`, `mode:<i>` (1-based, **run-local**), `shape:mode:<i>:<dof>`, `participation:mode:<i>:<x|y|z>`, `cluster:<c>`, `computed:<j>`, `sturm-candidate:<k>`, `identity:<n>`, `support:<id>`, `member:<pipe>`, `disclosure:<n>`, `diagnostic:<n>`, `refusal:<n>`.
- **Values.** Each published number is a value object `{kind, value, unit, unit_status, below_normal, undefined_reason}` (every unit is a catalog unit, §5.3.5). Its unit follows modal-1 for its kind and selector: a mode's shape, generalized mass and Γ take its normalization case's unit; a mass entry takes its DOF pair's. A nonzero value below the binary64 normal range is 0 with `below_normal` true; an undefined value is null with its reason (`total_mass_zero`, `not_factored`, `no_neighbour`, `no_reported_mode`, `not_finite`, or `outside_canonical_profile` for a magnitude of 2⁵³ or more, which the checked profile cannot carry). The global mass rows use the model basis (`{ref_type: "model", ref_id: "model"}`).
- **Identity rows** (second review, 2026-11-04). Each row carries `subject_kind` (`member`, `component_body` or `model`) beside `subject`, so the model's aggregate resultant (kind `model`, subject `model`) stays distinct from a member or body that is itself named `model`. A row of kind `model` names subject `model`. Along each axis each (kind, subject) appears once, and the aggregate row comes last.
- **Subject census (built).** A solved document's `binding.subject_census` records the subjects the run read from the model: the counts `straight_members`, `arcs`, `component_bodies`, `supports` and `nonlinear_supports`, and `subjects_sha256`, the SHA-256 of the checked canonical array of `{kind, id}` pairs sorted by (kind, id). The members are the pipe segments less the spans objective connectors replace. A member whose mass a rigid-family component body replaces is listed once, as `component_body` with the component id; any other member is `arc` when a curved-bend component realizes it and otherwise `straight_member`. Every support record, massless or not, is a `support`. `nonlinear_supports` counts the supports the linearized state case lists in the `supports` table (the supports with a nonlinear behaviour), 0 without a state; `supports` counts every support record, which no table lists. The record is formed from the `ModalOutcome` the run returned and bound by `binding.model_sha256`; a refused document's census is null.
  - Without the bound request, a reader holds the tables to the counts: along each axis one nodal `member` row per straight member, one resultant `member` row per arc and one `component_body` row per body; one `discretization` row per straight member; one `supports` row per nonlinear support; and no more nonlinear supports than supports. A contradiction fails evidence. The subjects are not re-derived, so a subject missing from both the census and the tables cannot be seen: evidence stays scoped, and the producer's own census never passes it.
  - With the bound request, each reader derives the subject list from the model by its own statement of this rule. The pass is structural: no subject is dropped because its mass is unresolved or outside the rounding model. The reader holds each table's subjects, kind included, to its own sets (the identity rows on every axis, the discretization members and the supports-table supports), then every count and the digest. Any difference fails evidence as `model-derived`, so a correct digest cannot hide a forged table, and a missing, extra, duplicated or substituted member, arc, body or nonlinear support fails. A model whose subjects the reader's rule does not cover stays scoped, with the reason. Each reader also checks that its own mass pass formed exactly the census's members and bodies; a disagreement is the reader's defect and leaves evidence unresolved, never passed or failed.
- **Authoritative payload.** `mass_matrix` and `shapes` are the payload the relations re-derive from. Either may be `{status: "withheld", reason}`; no other table may. A withheld payload makes the relations that need it scoped unsupported; it is never reconstructed from a digest.
- **Not published.** λ = ω² (Agent 0, 2026-11-04: no `angular_frequency_squared`, §5.3.5) and the μ-domain evidence (μ_max, δ_μ, δ_q, the screened μ). ω and the cluster rule's ω-form serve instead.

#### 5.3.3 The binding

`binding` captures the request exactly: `request_sha256`, `model_sha256`, `materials_sha256` (with whether `materials` was authored), every request option verbatim (`mode_count`, `frequency_cutoff`, `state_case`, `modulus_basis_ref`, `modulus_basis_temperature`, `weight_to_mass_acceleration`, `include_dynamic_only_restraints`), any undeclared member's name, the state case (`case_id`, `case_index`, the SHA-256 of its authored case object) and the full integrity-parameter set with its SHA-256. The run id derives from the request and parameter hashes.

#### 5.3.4 The readers

The inventory below describes the built payload checks. The selected
model-derived mass correction above is built beside them; it does not close
the full-reader condition.

**Corrected after independent review (2026-11-04).** The governed entry reads the **original text**, admitted first by the checked profile `openpipestress_jcs_ijson_v1` (duplicate members, NaN and Infinity, unsafe integers and integral values beyond 2⁵³ − 1 refuse the closure), so a duplicate `public_engineering_result` cannot collapse in a map. A **below-normal flag is a claim held to the payload**: a flagged m_i, Γ_k or m_eff,ik whose payload-derived value is normal, beyond its rounding allowance, fails; a flagged fraction or m_eff is held to the other published side of m_eff = fraction·T_k; only a withheld payload scopes the flags. 

**The relation inventory, anchored to the producer** (re-check of b87c5b58e, 2026-11-04, under Agent 0's steer; it replaces the third review's dependency table). Every governing relation is listed with:
- the exact producer operands and formation (`file:line` in `core/solver/modal_eigen/src` and `core/product_physics/src/modal`);
- operand identity;
- the accumulation order;
- each operand's domain from the producer's flagging rule;
- the full-reader check;
- what is explicitly unresolved.

Both readers implement THIS, independently: Python with `Values`, Rust with `Reach`. Where the producer forms a quantity from published values, the readers use published values.

**Domains** (from `WideDd::to_published`, dd.rs:476, and `to_normal_or_zero`, dd.rs:507):
- an unflagged number is that binary64 number, and an unflagged 0 is an exact zero;
- a flagged 0 is the producer's `to_published` result for a NONZERO value of magnitude below 2⁻¹⁰²², decided on the extended-exponent value. Its domain is (0, 2⁻¹⁰²²), signed for a signed kind, with no lower bound: 2⁻²⁰⁰³ is producer output (modal_quantities.rs:852);
- **each field's own domain** (class confirmation of cab0ea356) follows the rounding rule the producer publishes it under. A value outside it is no producer output and fails its owning relation. The readers and the oracle enforce the same rules:

  | Producer rule | Fields (owning relation) | Domain | Violation (fails) |
  |---|---|---|---|
  | `to_published` (dd.rs:476) | shape (normalization); m, Γ, m_eff, fraction (participation); orthogonality deviation (evidence) | 0, normal, or flagged (0, 2⁻¹⁰²²); Γ and the shape signed | an unflagged subnormal |
  | `to_normal_or_zero` (dd.rs:507; a subnormal refuses the run) | η, κ_λ, e_λ (evidence); cluster e_λ and the mode's copy (clusters) | 0 or normal, never flagged | a flag, or a subnormal |
  | plain binary64 (`Dd::to_f64`, f64 arithmetic) | mass entries, T_k, free-DOF mass (mass); cumulative fractions (participation); Sturm values and the shift, including the formation-separation ratio, which is not re-derived (sturm); identity relative discrepancy, whatever the identity tolerance (evidence) | any finite binary64, never flagged; an off-diagonal entry signed | a flag |

- `outside_canonical_profile` (document.rs:239) is finite, magnitude in [2⁵³, largest binary64]; `not_finite` (:238) is ±∞ or NaN;
- every other reason is a claim, checked where it is consumed: `total_mass_zero` (document.rs:687, :691) needs a published T_k = 0 and m_eff = 0; `not_factored` needs an unfactored candidate; `no_neighbour` (:754) cannot hold, since every placement has a neighbour (sturm.rs:1075, :1109); `not_formed` states no value.

**Arithmetic and outcomes:**
- Products are real and outward-rounded; 0 times a finite value is exactly 0.
- A nonzero product that underflows is never 0: it is kept within one subnormal step, nonzero, and the point paths defer to the set path.
- m's coefficient w·φ_i·φ_j is formed on sets, never as a binary64 product that can drop the term, because the producer sums each full triple product exactly (`mass_bilinear`, modal_quantities.rs:294). The φᵀp_k and e_λ point paths decide only products they kept, because the producer multiplies in extended range (reformation/mod.rs:964). T_k and the free-DOF mass multiply by 1 or 2 only, so they are exact.
- A relation instance **fails** when no value in its operands' domains satisfies it.
- It is **unresolved** (never passed) when some value does but the domains do not decide it.
- It **passes** only when all its operands are published points and it holds.
- Each instance is decided on its own operands. A null leaves unresolved only the instances that consume it; counts, basis, units and independent contradictions still run.

| Relation | Producer formation and operands | Identity and order | Domains | Reader check | Unresolved |
|---|---|---|---|---|---|
| T_k | `summarize_mass` (mass.rs:401, :409): Σ over nodes of (M r_k) in double-double from the DD mass blocks (`apply_mass` :147, `rigid_product` :171), rounded once. The published entries are those blocks rounded (outcome.rs:220) | each entry once, weight 1 on the diagonal and 2 off it; all DOFs, restrained included | entries: published or outside the profile (never flagged); off-diagonal signed | Σ w·M_ij within A(N+4)u·Σ\|terms\| | two or more entries outside the profile in one block |
| Free-DOF mass | stiffness.rs:256–268: a DD sum over the free pairs of the axis, rounded | each entry once | as T_k | as T_k, over free pairs | as T_k |
| Bessel | no producer formation: Σ_i m_eff,ik ≤ T_k for modes orthogonal in the assembled M | each m_eff once | m_eff flagged: (0, 2⁻¹⁰²²) | **interim contract** (below): passes where highest Σ ≤ lowest T_k·(1 + A(n+2)u); never refused | otherwise, a named Bessel unresolved (a temporary capability limit) |
| Shape | `normalize_mode` (modal_quantities.rs:346, :373–396): v/pivot, flagged on the exact quotient \|v\| < \|pivot\|·2⁻¹⁰²² and published as 0 | — | flagged: (0, 2⁻¹⁰²²); everything downstream uses the PUBLISHED 0 | normalization bounds (a flagged component is inside them; a flagged restrained one fails) | — |
| m | modal_quantities.rs:395 (`mass_bilinear` :294): Σ_ij φ_i·M_ij·φ_j over the full symmetric M_ff, exact sum, rounded once, flagged by `to_published` | each entry ONE variable, coefficient w·φ_i·φ_j (2 off the diagonal); φ published (flagged → 0) | m flagged: (0, 2⁻¹⁰²²) | within A(N+4)u·Σ\|terms\|, N counting every free pair | an entry outside the profile with a nonzero coefficient |
| φᵀp_k (not published) | p_k = (M r_k)_f, one DD row sum per free DOF, rounded (stiffness.rs:255–259); q = exact Σ φ_i·p_k,i (`wide_dot` :306, :429) | each entry ONE variable: its coefficients φ_i (row i) and φ_j (row j) are added; allowance e·Σ\|coefficients\|·\|M_ij\|, as x·[c − e·w, c + e·w] | φ published | enters Γ and m_eff below | — |
| Γ_k | q/m in WideDd, `to_published` (:433) | — | flagged: (0, 2⁻¹⁰²²), signed | Γ·m ≈ φᵀp_k within e(Σ w\|x\| + \|Γm\|), e = A(n+6)u over the n incidences | a flagged Γ or m; an entry outside the profile |
| m_eff,k | q²/m (:434–435) | — | flagged: (0, 2⁻¹⁰²²) | m_eff·m ≈ (φᵀp_k)² (as above); Γ²·m = m_eff within Au | a flagged operand whose set covers the other side |
| Fraction | q²/(m·T_k) (:437), from q and m, NOT from the published m_eff; `None` when T_k = 0 → `total_mass_zero` | — | flagged: (0, 2⁻¹⁰²²) | fraction·T_k ≈ m_eff within Au; T_k = 0 exactly when `total_mass_zero` | as m_eff |
| Cumulative fraction | `cumulative_effective_mass_fractions` (:460): a DD accumulation of the PUBLISHED fractions, in mode order, rounded once each | each published fraction once | the published value (a flagged fraction is 0) | the running sum of published fractions within A(i+2)u·Σ | a fraction outside the profile (never a flagged one) |
| e_λ | reformation/mod.rs:952–964: η·κ_λ from the extended values, each `to_normal_or_zero` | — | 0 exact, or normal | within 8u·max | a non-finite operand |
| Cluster e_λ | `cluster_error_estimates` (reformation/mod.rs:976): it first refuses any estimate that is not finite and nonnegative (:985–989), then takes the maximum per cluster; the mode copies it (outcome.rs:302–306) | each member once | as e_λ (0 or normal) | bit for bit; the copy is the same value object (value, reason, flag) | a member outside the profile |
| e_b, e_b·ν | sturm.rs:879–883: f64 `factor_term + formation_term` and `backward_error * inverse_norm_estimate` | — | published, outside the profile, not finite | bit for bit when published, else within 8u | an operand outside the profile that the sum or product admits |
| Decision | sturm.rs:841, :888–905: separated is s ≥ κ_s·e_b (ratio s/e_b); within is g ≤ g_max; conditioned is the product ≤ 1/κ_c; the first failing test in that order names the decision | — | a non-finite ratio is separated; ≤ is false for ±∞ and NaN | each decision's tests possible on the published sets | a ratio within 8u of κ_s |
| Accepted shift | ω = √σ (document.rs:745) in its gap | — | published or outside the profile | some value in [lower, upper], or ≥ the top mode | a shift outside the profile against an unbounded end |
| Orthogonality, standing | orthogonality.rs:193 (`to_published`, flagged); outcome.rs:342 (η > τ_r), :383 (deviation > τ_o) | — | deviation flagged: (0, 2⁻¹⁰²²) | the token possible for the published η and deviation | a non-finite η or deviation |
| Identity tolerance | weight_basis.rs:93: the largest relative discrepancy | — | published | ≤ the bound tolerance | a non-finite value |

ω, f, T, the coordinates, ℓ and the shape components are published-only: a null in them is refused. This is conservative and is not an adopted narrowing. Withheld payloads, and the subject census without the bound request, remain `scoped_unsupported`.

**Publication status.** Every verdict carries `validation`. It is `fully_validated` only when every mandatory governing relation has passed (decision `consistent`), and `not_fully_validated` otherwise, listing the relations not passed. Unresolved or scoped evidence confers no validated or current-authority status. Public admission stays disabled (§5.3.9).

**The exact limit, stated in every verdict (`limit`).** The verdict establishes per-relation consistency, not joint realizability. Each relation is decided on its own operands, so a document every relation admits may still have no single coherent set of underlying values: for example, x needing m ≈ 10¹⁶ while y needs m ≈ 4·10¹⁶. Agreement with the feasibility oracle does not establish payload coherence either. No joint solver is built (Agent 0, 2026-11-04).

**Bessel: Agent 0's interim reader contract (option C, corrected; 2026-11-04). This is a temporary capability limit, not an accepted release limitation.**
- **Old governing text:** "Bessel's Σ_i m_eff,ik ≤ T_k(1 + A(n+2)u)" is a check whose violation rejects the document.
- **New governing text:** "Bessel's Σ_i m_eff,ik ≤ T_k is established, and passes, where the readers' sound rounded or interval arithmetic shows highest Σ m_eff ≤ lowest T_k·(1 + A(n+2)u) from the published operands, whatever the orthogonality deviation. Otherwise it is a named Bessel-relation unresolved: not a contradiction, and not passed. It is refused only under an independent, existing common-M premise that proves the violation impossible for valid producer output; none exists today, so it is never refused."
- **How the comparison is decided** (review of `f5c66b167`): the decision itself must not round in the passing direction. The Python reader compares in exact rationals (every operand is a finite binary64 value); the Rust reader rounds outward, stepping each partial sum of the m_eff upper ends up one ulp and the allowed total T_k,low·(1 + 16(n+2)u) down one ulp, with the slack exact in binary64. Outward rounding can only turn an established case into `unresolved`, never the reverse. A boundary witness (T = 1.3, m_eff = [1.3000000000000094, 0], exactly 8.9·10⁻¹⁷ above the allowed total) is not established by either reader.

Why: Σ m_eff ≤ T_k holds for modes orthogonal in the assembled M, which participation uses. The published deviation d̂ is measured on the re-formed M̂. Nothing is inferred from d̂ alone in either direction, and 1 + (n − 1)·d̂ is not a bound. The excess bound [1 + (n − 1)·d]·T holds only for a common metric, so it would need a metric transfer from M̂ to M that is not justified. The unit-test witness: M = I, M̂ = [[1, −1], [−1, 2]], φ₁ = (1, 0), φ₂ = (1, 1), r = (1, 1) are exactly M̂-orthogonal, yet Σ m_eff = 3 > T = 2.

What still runs, and the gating:
- Every independent check keeps running: units, nonnegativity, the mass basis, Γ²m = m_eff, the fraction, the cumulative sums, the counts and the per-mode checks.
- An unresolved Bessel confers no `fully_validated` status and no fresh publication. The document stays inspectable with that scope.
- The producer's numerical Sensitive standing is separate from a reader-unresolved relation.

Agent 0 approved selecting this scoped handling subject to review. It does not adopt the earlier rationale (the 1 + (n − 1)·d proposal of cab0ea356) and gives no universal never-reject guarantee.

**Open item: option B, not built, cost to be analysed.** The producer would publish direct Gram evidence on the common assembled M (φᵢᵀMφⱼ for the reported modes, or a bound on it), so that Bessel could be decided without a metric transfer.

The controls are in both readers:
- the two-mode witness as a unit test;
- a corpus document where the sum is established (passed);
- a document beyond the strict allowance (unresolved);
- each beside a wrong Γ²m, which still refuses.

**Controls.**
- **Producer-generated edges (governing false-refusal controls).** `modal::tests_document::modal_edge_entries` runs the real solve on the component-body request. It replaces the reported modes' participation quantities with modal_eigen's own `normalize_mode` and `cumulative_effective_mass_fractions` outputs on edge bases, and projects through the producer's own `modal_doc_solved`. The result is `fixtures/results/modal_producer_edges.json`, held by a byte test. Its three edges: the m_eff = 2⁻²⁰⁰³ regression; a flagged fraction inside the cumulative sums; and the reviewer's probe (Γ_x 10⁻²⁰⁰, T_x 10⁻³⁰⁰, a flagged m_eff with a normal fraction). Both readers and the oracle accept all three; each is `unresolved`, never `passed`.
- **Independently derived mutations.** Four mutations of those edges have expectations derived from the formation above, not from either reader:
  - an unflagged 0 m_eff beside Γ²m = 2⁻²⁰⁰³ (reject);
  - a flagged m_eff beside Γ = 0.5 (reject);
  - a cumulative counting the flagged fraction's true value (reject);
  - a doubled normal fraction of a flagged m_eff (unresolved).
- **Unit tests in both readers.** The duplicated free/free entry with +1 and −1 is ONE variable whose coefficient is exactly 0. With m = 2⁻¹⁰²² and Γ = 0.5 it is rejected; two independent unknowns would have admitted it. A flagged value is nonzero, and an underflowing product is never 0.
- **The feasibility oracle is supporting evidence only.** It has the same domains, published operands and per-entry identity. The 525 cases include flagged free shape components. A reader never fails a family the oracle finds feasible. It never passes one the oracle proves infeasible: it fails it, or leaves it unresolved where a flagged value's extended exponent is beyond binary64. That allowance is used in 0 of 525 cases.
- **Disagreements found and fixed this round:**
  - three Python false scopings, where an entry's allowance was taken against its set's largest magnitude instead of the same x;
  - a subnormal point product widened down to 0, caught by the Rust unit test and fixed in both readers.

ω, f and T must be positive and finite before any division. **Coverage is required by construction, over what the document lists:** at least one computed mode; the identity rows along each axis in order, each straight member of the discretization table compared nodally as kind `member` (and nothing else nodally), the aggregate (kind `model`) last and once, and one (kind, subject) list on every axis; a linearized state case's supports, its restrained set equal to the document's restrained DOFs and every rigid treatment's DOF restrained, and no supports without one; and the fixed numerical-integrity disclosure. **Coverage is complete only against the model.** Without the bound request, coverage is established for the subjects the census counts and for the aggregate; a subject missing from the census and from every table cannot be detected, and evidence is scoped by saying so. With the bound request, the subject census (§5.3.2) re-derives the subjects, so a missing, extra, duplicated or substituted subject fails evidence. On a linearized state case the identity rows and the supports table are formed on the partition from the reported support states, so evidence stays scoped on those states' qualification, as mass and participation do.

Both readers return the same verdict: `closure`, `binding` (with the inputs not supplied), nine relations, `health` (`checks_passed`, `sensitive` or `not_assessed`), the refused document's blocking codes, a `decision` (`consistent`, `consistent_with_unresolved`, `consistent_with_scoped_unsupported`, `refusal_consistent` or `rejected`), `validation` and the `limit` and `public_admission: disabled` with §5.3.9's conditions. Each standing is passed, failed (with the first difference), unresolved (with the instances its operands leave open; never passed), scoped unsupported (with its scope) or not checked. When closure fails nothing else is checked. They re-derive **without an eigen solve**, from the payload, in binary64, summing in the document's row order, with u = 2⁻⁵³ and the reader allowance A = 16 (a rounding allowance, not an engineering threshold):

- **closure:** members, constants, fixed texts, the table digest, row ids, ordinals, checksums, digests, every reference, every value object's shape and kind, the DOF order and free numbering, the guard's DOF counts.
- **binding:** the parameter hash and run id; the extraction bound, f_c and a against the options (through the unit catalog, within 4u); the state case; τ_t, κ_c, κ_s and g_max against the parameters; with a supplied request, its hashes, options and state-case object.
- **mass:** T_k = Σ w·M_ij over the entries with both DOFs translational along k (w = 2 off the diagonal) within A(N+4)u·Σ|terms|; the free-DOF mass likewise over free pairs; a non-negative diagonal; Bessel's Σ_i m_eff,ik ≤ T_k(1 + A(n+2)u).
- **normalization:** ℓ re-derived from the nodes (within 8u); each mode's reference DOF free, of its case's kind and exactly +1; restrained components exactly 0; case (a) translations at most 1 and not τ_t·ℓ·θ ≥ 1 within 8u; case (b) rotations at most 1 and translations at most τ_t·ℓ(1 + 8u).
- **participation:** m_i = φᵀM_ffφ within A(N+4)u·Σ|terms|; s_k = φᵀp_k, p_k = (M R_k)_f, with Γ_k·m against s_k and m_eff·m against s_k² within A(N+6)u-scaled bounds; Γ²m = m_eff within Au; fractions against T_k within Au, undefined exactly when T_k = 0 (then m_eff = 0); the cumulative sums within A(i+2)u.
- **counts:** f = ω/2π and T = 1/f bit for bit; ascending computed modes; the reported set re-derived from the bound by the kernel's rule over the published f and cluster ids (bounded count, stop and cluster completion, exactly); the endpoint against the census; the Sturm count and its reported count equal to the modes reported.
- **clusters:** ids chained from 1; each row's members; the set ending at a cluster boundary; δ_c from ω, ω²_{i+1} − ω²_i against δ_c·ω²_{i+1} with margin 16u·(1 + δ_c)·ω²_{i+1} (§5.3.5), a pair inside the margin scoped unsupported (λ is not published); each cluster e_λ the largest member's, bit for bit.
- **sturm:** at most s_max candidates, only the last accepted; e_b = factor term + formation term and e_b·ν bit for bit; the accepted shift's count, e_b·ν ≤ 1/κ_c, g ≤ g_max and separation ≥ κ_s(1 − 8u); each refused candidate's decision consistent with its evidence; the placement's ends equal the neighbouring modes' ω (for an empty set, fl(√fl(fl(2π·f_c)²)), the producer's own operations, which must be a finite positive square: the producer refuses a zero or overflowing λ_c as `InvalidGap`) and the shift inside its gap.
- **evidence:** e_λ = η·κ_λ within 8u; the identity discrepancies within the bound tolerance (scoped when its absolute part is not 0, whose scale is not published); non-negative formation values; the orthogonality pair stated exactly when two modes are reported.
- **standing:** a mode Sensitive exactly when η > τ_r, the set exactly when a mode is or the orthogonality deviation exceeds τ_o, each with its reasons.
- **units:** every value against modal-1, by name. Each value's status must be `catalog` (or `equilibrated_basis` for a kind in the closed list, which is empty) and its unit the kind's catalog unit: the mass coupling `unit:kilogram_meter_per_radian`; the Sturm and stiffness-side formation magnitudes `unit:one`; the mass-side ones `unit:square_second` (§5.3.5). The standing is passed or failed, never scoped. It closes no other relation: a document is still not `fully_validated` while Bessel is unresolved or the evidence relation is scoped (a subject census not re-derived from the bound request, or a linearized state case).

They do not re-derive K, the double-double re-formation (the orthogonality deviation and η are evaluated on M̂ and K̂, not published), the census's μ evidence or the eigen-solve (the DEF-O stance). **Tests:** the Rust reader's `tests/corpus.rs` and the Python `tests/test_modal_evidence.py` hold each reader to every verdict of `fixtures/results/modal_corpus.json` (124 entries: every fixture coherent, with and without a supplied request; tampered T_k, a mass entry, the free-DOF mass, Bessel's inequality, the shape sign, the normalization case, a case (b) translation, a restrained component, ℓ, Γ, m_eff, m_i, a cumulative fraction, the count, the extension, a frequency, the endpoint, a cluster id, a cluster e_λ, a pivot count, a shift outside its gap, a premise, e_λ, an identity discrepancy, the set and a mode standing, a unit, the coupling's unit, a row id, a stale checksum and digest, a claimed public admission, a limitation, a withheld non-payload table, the table digest, the parameters, a different request, an option, the state case, a withheld mass and withheld shapes (scoped unsupported), and three refused-document cases; and 30 adversarial entries on the absence and contradiction paths of each relation: below-normal flags contradicting the payload or the other published side, a withheld payload with a flag (scoped), missing Sturm evidence beside a contradictory count or conditioning product, a profile-null product, an accepted growth null without a bound (a failure since the second review), no candidates, duplicate members in the header and in a row, a NaN and an unsafe integer in the text, zero ω, f and T, an emptied identity table, a dropped axis row, a dropped straight member's row, a straight member compared as a resultant, emptied supports, a changed restrained set, supports without a state case, emptied disclosures, an emptied mass matrix, a restrained reference DOF, emptied participation and cluster tables, dropped options and reasons without a Sensitive token); and 16 entries on the profile bounds and the subject kind. The bound entries are:

- against published values: a factor term, a formation term, an e_b, a conditioning product and an accepted growth each outside the profile;
- an undecided e_b sum beside a failed conditioning premise;
- a separation ratio null without a bound on a factored candidate;
- an e_λ outside the profile, against η·κ_λ and against its cluster's e_λ;
- a T_k outside the profile against its entries;
- an m_eff outside the profile against Γ²·m;
- an η outside the profile, which must make the mode Sensitive;
- an off-diagonal mass entry outside the profile (scoped).

The subject-kind entries are a straight member named `model` (consistent), the aggregate row relabelled as a member, and a member's row labelled as the aggregate (closure).

The third review adds 9 entries:
- the reviewers' reproductions: a restrained entry outside the profile beside a reversed Γ; `total_mass_zero` claims beside a T_x outside the profile; a wrong cumulative fraction beside a T_x outside the profile; and a zero e_b times an outside-the-profile ν;
- three genuinely undecidable cases (scoped): a below-normal m with Γs outside the profile and the payload withheld; two couplings outside the profile along x (arc); and a not-finite η → e_λ → cluster chain;
- two cases the oracle found: a claim-null restrained entry, which fails mass only; and a Γ outside the profile against a published m_eff.

Two earlier entries change intent:
- `scoped:large_mass_coupling_is_undecidable` was contradictory (no single coupling of magnitude ≥ 2⁵³ closes T_x) and is now `mass:one_large_coupling_cannot_close_its_sum`;
- `scoped:below_normal_mass_with_payload_withheld` contradicts its published Γ and m_eff (Γ²·m ≤ 2⁻¹⁰²²·Γ²) and is now `participation:below_normal_mass_against_published_gamma_squared`.

The re-check of b87c5b58e adds 7 entries: the three producer edges (`unresolved`) and the four independently derived mutations of them (see the controls above), for 118 in all. Earlier entries whose relations a flagged or outside-the-profile operand leaves open now read `unresolved` where they read `scoped_unsupported`; no rejection or pass changed.

The class confirmation of cab0ea356 adds 6 controls, for 124 in all:
- a nonzero η·κ_λ published as e_λ = 0 (reject);
- a subnormal η and e_λ, outside to_normal_or_zero (reject);
- the four Bessel documents: established (passed), unresolved, and each beside a wrong Γ²m (refused).

The units slice (§5.3.5) adds 3 adversarial units entries, for 127 in all. Each fails units only:
- a Sturm e_b published `equilibrated_basis` (outside the closed list);
- a translation–rotation mass entry published in its former form, `catalog_family_absent` (not modal-1's `unit:kilogram_meter_per_radian`);
- the time-squared ‖DM̂D‖_∞ claimed as `unit:one`.

Regenerating the expectations changed no verdict of the 124 earlier entries, and `units:coupling_given_a_unit` still fails units.

The `time_squared` change and the empty-set correction (§5.3.5) add 5 entries, for 132 in all:
- `coherent:cantilever_subnormal_cutoff`, consistent with its census scope;
- three time-squared rejections (the obsolete `catalog_family_absent` form, `equilibrated_basis`, and the spelling `unit:second_squared`);
- the subnormal cut-off's lower end published as 2π·f_c, which fails sturm.

The review of `c3f824e85` adds 2 more, for 134: a cut-off of 10⁻²⁰⁰ Hz, whose square underflows to zero, with the lower end published as 0.0 and as −0.0. The producer refuses a zero λ_c (`InvalidGap`), so no solved document has one, and both fail sturm (`sturm:cutoff_square_underflows_to_zero_lower_end_zero`, `…_negative_zero`).

The earlier `units:time_squared_claimed_dimensionless` now fails against `unit:square_second`. `units:coupling_published_without_its_catalog_family` now fails at the status vocabulary, and its phrase follows.

Regeneration delta: units turn from `scoped_unsupported` to `passed` in 102 earlier entries. No decision, closure, binding, health or other relation changes, because the evidence relation's census scope keeps every coherent solved document `consistent_with_scoped_unsupported`.

`mass:bessel_exceeded` is now `participation:a_bessel_excess_beside_a_wrong_gamma_squared`: its Bessel is unresolved under the interim contract, and its Γ²m still refuses. m's producer probe (m = 2⁻¹²⁰⁰ from `normalize_mode`) is a relation-level control in `modal_producer_edges.json`, run through each reader's own m reconstruction and the oracle, because a real model's payload cannot carry it.

**Point-sum order.** Each reader decides a point relation on binary64 sums. The Python reader evaluates them as `modal_result` does: left to right, from +0.0 for the `fold` sites (m, T_k and the free-DOF mass, Γ / m_eff's s and a) and from −0.0 for the cumulative fraction's `Iterator::sum` (`_sequential_sum`). It never uses Python's compensated float `sum` (3.12+). `fixtures/results/modal_sum_order_boundaries.json` holds both readers to 16 boundary entries in the corpus format. For each of the four relations, a published value sits at the last value the sequential evaluation admits and at the first it refuses, on each side; the Rust reader writes the verdicts. With a correctly rounded sum (`math.fsum`) in place of `_sequential_sum`, 8 entries, at least one per relation, take another verdict, which shows that the fixtures catch the compensated order. This makes the four point-sum relations agree; it is not a claim that the readers agree on every payload. One known difference is outside it: the Bessel bound, which Rust decides with outward-rounded binary64 sums (`modal_result` `lib.rs:~1982`) and Python with exact `Fraction`s (`bessel_established`). Exactly at that bound Python can establish equality where Rust leaves `mass` unresolved (passed against unresolved, never a false refusal; witness n = 1, T = 1, effective highs [1 + 48u]).

The readers agree on every decision; each entry also names the standing it targets, which must fail, and each adversarial entry a phrase that the failing standing's difference must contain in each reader. Corpus agreement alone is weak evidence of independence, because the readers were written to one rule set and can share a blind spot (the reviews found four: the flags, the Sturm `continue`, profile-null operands that hid contradictions, and inexact intervals that skipped families or mis-handled zero, sign and below-normal values; the last is why the feasibility oracle tests feasibility directly, not only agreement); the adversarial entries test absence and contradiction paths, not only tampered values, and each reader states its own reason. Both readers are held to the committed modal-1 bytes and check the catalog's canonical units; the Python reader imports only the standard library and its two siblings.

#### 5.3.5 Units: the dimensional trace, the catalog families and the closed equilibrated-basis list

**Decided by Agent 0 (2026-11-04), implemented in the I3a units slice.** Agent 0 made four decisions:
- add `mass_length_per_rotation`;
- add no `angular_frequency_squared`;
- keep `equilibrated_basis` only as an explicit coordinate-basis magnitude category with an exact closed list of kinds;
- after the trace, add `time_squared` for the two D·M·D magnitudes (option (a)).

Both additions are recorded as dated engineering extensions beside D-01 (`execution/_Coordination/_DECISIONS/D-01_ENGINEERING_EXTENSION_2026-11-04.md`). D-01 and DEC-018 are unchanged.

**What passing units means.** Every published value now has its catalog unit, so the units standing passes for current documents. It closes no other condition. The mandatory Bessel relation (interim contract C) and the production conditions (§5.3.9) stay open. Read with its bound request, each linear solved fixture is now `fully_validated`: its Bessel relation is established and its subject census re-derived. Each linearized state case leaves mass, participation and evidence unmet. A reader's `fully_validated` states per-relation consistency only and admits nothing (§5.3.9).

- **The families.**
  - **`mass_length_per_rotation`.**
    - Vector (L, M, T, Θ, A) = (1, 1, 0, 0, −1).
    - Units: `unit:kilogram_meter_per_radian` `kg*m/rad` ×1 (canonical, a project-governed binding); `unit:pound_mass_inch_per_radian` `lb*in/rad` ×(0.45359237·0.0254); `unit:pound_mass_foot_per_radian` `lb*ft/rad` ×(0.45359237·0.3048). The display factors are definitional products of the reviewed constants.
    - Witnesses: `mass_length_per_rotation_units_have_definitional_conversion_witnesses` (0.011521246198 and 0.138254954376 kg·m/rad, each in both directions).
    - It covers the translation–rotation mass entries (unit chosen by `dof_pair_class`) and the rotational rows of M·R_k (p_k is not published).
  - **`time_squared`.**
    - Vector (0, 0, 2, 0, 0).
    - One unit: `unit:square_second` `s^2` ×1, SI canonical, with no display unit.
    - Witness: `time_squared_has_one_canonical_unit_and_is_matched_by_name`.
  - **Both** are named dimensions, checked by name and never matched by vector, and both are transcribed in `tests/verifiers/units.py`. Every earlier unit identity is unchanged.
- **Coordinate conventions.**
  - Each free DOF i has a coordinate q_i: a translation in m, a rotation in rad.
  - Through virtual work, δW = Σ_i F_i·δq_i, the generalized force conjugate to a rotation is a moment in J/rad. So K_ij = ∂F_i/∂q_j is in J/(q_i·q_j): N/m, N/rad and J/rad².
  - A rotational stiffness authored in N·m/rad (the catalog's `rotational_stiffness`) is the same number read as J/rad², since the moment's N·m is J/rad.
  - Likewise M_ij is in J·s²/(q_i·q_j), which is what modal-1 publishes: kg, kg·m/rad and kg·m²/rad².
  - The product's matrices are SI binary64 numbers with rotations in radians. They are the same numbers under either reading of the radian; the trace uses the virtual-work reading, the only one under which K − λM is homogeneous with one λ.
- **The eigenparameter against the published frequencies.**
  - λ (an eigenvalue of K φ = λ M φ) and σ (a Sturm shift) are in s⁻²: J/(q_i·q_j) over J·s²/(q_i·q_j), entry by entry.
  - The published circular frequency is ω = √λ in s⁻¹, written rad/s by the catalog's `angular_frequency` convention: the radian there is the phase angle (one cycle is 2π rad, and 1 rad = 1), not a coordinate rotation, so it does not enter any q_i.
  - f = ω/(2π) in Hz (one cycle is 2π rad), and T = 1/f in s.
  - The rounding: ω = fl(√λ), within u of √λ; fl(ω²) is within 3u·λ of λ (+O(u²)); f = fl(ω/fl(2π)).
- **Which D each route uses (the code, not only the convention).**
  - **Sturm.** `ShiftPencil::from_entries(n, &pencil.stiffness_entries, &pencil.mass_entries)` (eigen.rs:226) forms d_i = fl(1/fl(√k_ii)) in binary64 (sturm.rs:627), refusing k_ii ≤ 0. The k_ii are the diagonal of the **formed** free–free stiffness K_ff of `ModalPencil` (stiffness.rs:204–218, the assembled stiffness partitioned to the free DOFs).
  - **Formation.** `reform_modal_pencil` is called with the same `&pencil.stiffness_entries` (outcome.rs:287–294) and passes them to `formation_difference` (integrity.rs:422). It forms d_i = 1/√|k_ii| in double-double from that **formed** diagonal (reformation/mod.rs:809), not from the re-formed K̂. That D is applied to the formed and to the re-formed matrices alike.
  - **The same normalization in units.** Both are |k_ii|^−½ of the same formed entries (k_ii > 0, so |·| changes nothing), so d_i carries J^−½·q_i on both routes. Numerically, Sturm's binary64 d_i and the formation's double-double d_i differ by the binary64 rounding of the root and the reciprocal (about 1.5u relative), which has no unit consequence.
  - **The result.** (D X D)_ij = d_i·X_ij·d_j = X_ij·q_i·q_j/J, entry by entry and whatever the DOF classes. For X = K or K̂ it is dimensionless. For X = M or M̂ it is s². For Sturm's shifted entry a_ij = fl(k_ij − fl(σ·m_ij)), σ·m_ij is in J/(q_i·q_j) like k_ij, so ã_ij is dimensionless.
  - **The code is consistent with this.** It adds the two scaled norms weighted by the eigenparameter: ‖DK̂D‖_∞ + λ·‖DM̂D‖_∞ in η's scale (reformation/mod.rs:950), and Σ\|k̃_ij\| + σ·Σ\|m̃_ij\| in the formation term (sturm.rs:758). Those sums are homogeneous exactly when D M D is s² against a dimensionless D K D. The arc fixture is **numerical support only, not the dimensional proof**: ‖DK̂D‖_∞ = 4.20 (at least 1 for a unit-diagonal K̃), while ‖DM̂D‖_∞ = 9.3·10⁻⁸ s², of the order of 1/ω² for ω up to 4.9·10³ rad/s.
- **The trace.** Every kind formerly published as `equilibrated_basis`, with its assigned unit; the trace holds for every row:

  | Kind (modal-1) | Formula (producer) | D from | Dimension | Assigned |
  |---|---|---|---|---|
  | `sturm_formation_term` | c_f·u·‖\|K̃\| + \|σ\|·\|M̃\|‖_∞ (sturm.rs:758) | formed K_ff, binary64 | K̃: 1; σ·M̃: s⁻²·s² = 1 | `unit:one` |
  | `sturm_factor_term` | γ_n·‖\|L̂\|\|D̂\|\|L̂ᵀ\|‖_∞, LDLᵀ of Ã = D(K − σM)D | formed K_ff, binary64 | 1 | `unit:one` |
  | `sturm_backward_error` | e_b = factor term + formation term (sturm.rs:880) | formed K_ff, binary64 | 1 | `unit:one` |
  | `sturm_inverse_norm_estimate` | ν, the Hager–Higham estimate of ‖Ã⁻¹‖₁ | formed K_ff, binary64 | 1 | `unit:one` |
  | `sturm_max_abs_lower_factor` | max\|L̂_ij\| (unit lower factor of Ã) | formed K_ff, binary64 | 1 | `unit:one` |
  | `sturm_max_abs_pivot` | max\|d̂_j\| (pivots of Ã) | formed K_ff, binary64 | 1 | `unit:one` |
  | `sturm_max_abs_scaled_diagonal` | max\|ã_jj\| | formed K_ff, binary64 | 1 | `unit:one` |
  | `stiffness_formation_difference` | ‖D(K_ff − K̂)D‖_∞ (reformation/mod.rs:706, `scaled_norm`) | formed K_ff, double-double | 1 | `unit:one` |
  | `reformed_stiffness_norm` | ‖D K̂ D‖_∞ | formed K_ff, double-double | 1 | `unit:one` |
  | `mass_formation_difference` | ‖D(M_ff − M̂)D‖_∞ | formed K_ff, double-double | s² (0, 0, 2, 0, 0) | `unit:square_second` |
  | `reformed_mass_norm` | ‖D M̂ D‖_∞ | formed K_ff, double-double | s² (0, 0, 2, 0, 0) | `unit:square_second` |

  - **Already `unit:one`, unchanged.** The derived quantities: the conditioning product e_b·ν, the growth factor max(1, max\|L̂\|)²·max\|d̂\|/max\|ã_jj\|, the separation ratio s/e_b and the formation-separation ratio. s_j(σ) = (\|λ_j − σ\| − e_λ·λ_j)·(φ̃ᵀM̃φ̃)/‖φ̃‖₂² is s⁻²·s², so it is dimensionless too.
  - **Split kinds.** `formation_difference` and `reformed_norm` each carried a K value and an M value of different dimension under one kind. They were split by matrix into the four kinds above (named for the matrix, with unchanged formulas), and the documents' header members bind them by name: `stiffness_difference`, `mass_difference`, `reformed_stiffness_norm` and `reformed_mass_norm`.
  - **No escape status.** No kind keeps a non-catalog status. The `catalog_family_absent` rule form is retired from modal-1 because every kind now has its family.
- **The closed list and the status vocabulary.**
  - modal-1 carries `equilibrated_basis_kinds`, the coordinate-basis magnitudes whose exponents genuinely mix. Such a magnitude claims no dimension and no physical unit, and is never called dimensionless.
  - The trace leaves the list **empty**, `[]`.
  - Each reader, from its own statement of the table, holds a value's status to `catalog`, or to `equilibrated_basis` for a listed kind only.
  - So each of these fails units: an obsolete `catalog_family_absent` ("… is not a modal-1 unit status"); an `equilibrated_basis` ("… is not in modal-1's closed equilibrated_basis list"); a misassigned `unit:one`; and any other spelling, such as `unit:second_squared` ("… is not modal-1's unit:square_second").
  - Adding a kind to the list needs a trace showing mixed exponents and a modal-1 change.
- **Superseded provenance.** The first units finding (I3a, before 2026-11-04) is **superseded 2026-11-04**, after the code trace above. It claimed that a K̃ translation–rotation entry carries rad^−½ and that the equilibrated magnitudes therefore have no catalog dimension. That figure read the K diagonal in the catalog's `rotational_stiffness` units (N·m/rad) without the virtual-work reading of the moment (J/rad). Under that reading K − λM is not homogeneous, and the code's own sums ‖K̃‖ + λ‖M̃‖ would mix dimensions; it is not the dimension of what the code forms. It is kept verbatim:

  > - **Finding.** Every per-mode quantity stands on existing families: f (`unit:hertz`), ω (`unit:radian_per_second`), T (`unit:second`), the shapes (`unit:one`, `unit:radian_per_meter`, `unit:meter_per_radian`), m_i (`unit:kilogram`, `unit:kilogram_square_meter_per_square_radian`), Γ, m_eff and the fractions. **λ need not be published:** ω carries the spectrum, and the δ_c and Sturm relations are re-derived from ω with a stated rounding margin (a pair inside it is scoped unsupported, not guessed). **The mass payload does not stand on the catalog:** with angle tracked as a dimension, a translation–rotation entry of M (and p_k's rotational rows) is kg·m·rad⁻¹, exponents (L, M, T, Θ, A) = (1, 1, 0, 0, −1), for which the catalog has no family; it is published with its exponents and `catalog_family_absent`. The equilibrated-basis magnitudes (e_b and its terms, ν, the formation differences and re-formed norms, max|L|, max|d|, max|ã|) are norms of the diagonally scaled pencil, which mixes angle exponents (a K̃ translation–rotation entry carries rad^−½), so no catalog dimension is claimed (`equilibrated_basis`). No dimensionless token is used for either.

  Its proposal to DEL-02-02's owner (`mass_length_per_rotation` as built above; `angular_frequency_squared` only if λ were published) is in the Git history of this file at `c0a46799a`.
- **λ, σ and ω where they interconvert.** No λ or σ value is published. ω, f and T come from `frequency_from_eigenvalue` (ω = fl(√λ), f = ω/2π, T = 1/f). The Sturm shift is `sturm_shift_circular_frequency` = fl(√σ) in rad/s (document.rs:745), and the placement ends are fl(√λ) through the same `sqrt` (document.rs:558). Both readers account for the rounding:
  - **(i) Clusters.**
    - The kernel breaks where fl(λ_b − λ_a) > fl(δ_c·λ_b) (clusters.rs:71). The readers decide this from ω_b² − ω_a² against δ_c·ω_b².
    - Each fl(ω²) lies within 3u·λ of its λ, so reader and kernel differ by at most (8u + 5u·δ_c)·λ_b + O(u²).
    - Both readers use a margin of 16u·(1 + δ_c)·ω_b² (the former 8u·ω_b² fell short of the bound for any δ_c > 0), and a pair inside it is scoped. A targeted search found no binary64 pair that the old margin misjudged.
  - **(ii) The gap.** λ_lo < σ < λ_hi strictly (sturm.rs, `count_in_gap`), and a correctly rounded √ is monotone, so ω_lo ≤ ω_σ ≤ ω_hi, possibly with equality. The readers test membership of the closed interval, and test the whole-spectrum shift only as ≥ the top ω.
  - **(iii) The ends.** The gap ends equal the neighbouring modes' ω bit for bit, because they are the same function of the same λ.
  - **(iv) The empty set (corrected 2026-11-04 after the review of `9b22b5a2b`).**
    - The lower end is fl(√λ_c), with λ_c = fl(ω_c²) and ω_c = fl(2π·f_c) (outcome.rs:310).
    - The earlier claim, "within 1.5u of fl(2π·f_c), and the readers allow 8u", holds only while ω_c² is normal. Below f_c ≈ 2.4·10⁻¹⁵⁵ Hz the square is subnormal and keeps fewer significant bits. At f_c = 10⁻¹⁶⁰ Hz the published lower end is 6.283177176472489·10⁻¹⁶⁰, against 2π·f_c = 6.283185307179586·10⁻¹⁶⁰: 8.1·10⁻¹⁶⁶ apart, beyond 8u·ω_c = 5.6·10⁻¹⁷⁵. That refused a valid document.
    - Each reader now repeats the producer's binary64 operations, fl(√fl(ω_c²)), and compares bit for bit.
    - `modal_cantilever_subnormal_cutoff` (producer-generated, f_c = 10⁻¹⁶⁰ Hz) is read consistent by both readers, and the corpus mutation that publishes 2π·f_c instead fails sturm.
  - **(v) The cut-off** is compared in f by the kernel (clusters.rs, `reported_set`: f ≤ f_c) and by the readers on the same published f. The cut-off disclosure ("reaches across the cut-off") compares λ in the producer and publishes only text.
- **No `angular_frequency_squared`.** λ is not published, and ω carries the spectrum.

#### 5.3.6 Fixtures and their generation

Twelve maintained fixtures, `fixtures/results/modal_<name>.json` with their requests `modal_<name>.request.json`, projected from real `ModalOutcome`s under the **TEST-LOCAL, EXPERIMENTAL** parameter set `TEST_LOCAL_I3A_EXPERIMENTAL` (I2b-1's `test_local_parameters`): EX n05 (3 modes), the arc model (6), the component body (3, a = 9.8125 m/s²), the I2b-2 state cases `load:PRESSED` of K-D5 U-1 (one-way active, restrained) and `load:SLIDE-POS` of F-1 (friction sliding, frozen), the SEC-A two-element cantilever's whole finite spectrum (12 modes, two case (b) torsion modes), its cluster extension (bound 1, two reported), an empty set below a 0.5 Hz cut-off, an empty set below a 10⁻¹⁶⁰ Hz cut-off (a subnormal (2πf_c)², §5.3.5), a Sensitive set (`TEST_LOCAL_I3A_EXPERIMENTAL_SENSITIVE`: τ_r = 10⁻³⁰), and two refused documents (EX n05 with 4 modes, `MODAL_STURM_SHIFT_UNSEPARATED`; no bound, `MODAL_EXTRACTION_BOUND_REQUIRED`). `modal::tests_document::fixtures_are_the_projection_of_their_requests` re-projects each and requires its committed bytes (and a second projection's equality); the ignored `write_modal_fixtures` is the deterministic generation script (`fixtures/results/modal_README.md`). They are test fixtures, not run evidence.

#### 5.3.7 Production parameters: Agent 0's A1 and A2 records, the held set and the μ-domain evidence (no production instance)

`ModalIntegrityParameters` keeps no `Default` and no production instance; every document carries the labelled test-local set. **Agent 0's decisions (2026-11-04):**
- **A1, approved as numerical acceptance and algorithm criteria, not as a domain-wide error theorem:** τ_t = 10⁻⁶; the identity tolerance (relative 2⁻⁴⁴, absolute 0); g_max = 2⁵²; τ_e = 10⁻⁶; τ_m = 2⁻³⁶; and the existing s_max = 7, gap positions, whole-spectrum offsets and QL limit of 30. The empirical witness margins in the table stay empirical: they describe the witnesses, not every admitted model. The identity tolerance is not a guarantee that every physical error is detectable. The approval enables no public result and creates no production default.
- **A2, decided in part (2026-11-04, on the I3a evidence below):**
  - **κ_c = 2¹⁰, κ_s = 2⁷ and τ_o = 2⁻²³ are adopted as explicit engineering acceptance and demotion criteria.** Each carries its stated assumptions and observed margins (table). None is a guaranteed error or count theorem. τ_o is applied to the measured deviation on the re-formed M̂ (check 3); a large estimated bound alone demotes nothing. Every other premise is unchanged and still mandatory: the Sturm count and its separation and conditioning premises (check 4), the full per-mode re-formed evidence (check 2) and the numerical Sensitive standing. **These three criteria establish no count certificate by themselves.**
  - **δ_c = 2⁻²⁰ is adopted as the computed-cluster grouping convention** (an algorithm threshold, not an exception to a release criterion). The cluster definition stays §3.4 check 5: the adjacent relative gap, chained. Being a convention, it need not classify exact physical multiplicities and distinct modes perfectly. Close distinct modes may merge into one cluster, and the cluster's completion and extension are disclosed. That the n·S sufficient bound (item 5) fails on a model does not by itself invalidate the fixed grouping. Count completeness and the Sturm separation premise stay mandatory, and a grouping that is not numerically established refuses with its named refusal. Nothing claims that every repeat is captured, that a cluster identifies a persistent physical mode, or that every model resolves. No spread-dependent clustering redesign is started. If a specific witness shows that the convention violates an existing mandatory check, rather than merging clusters, the exact failure is reported before the value is pinned. None is known: EX n05's second bending pair, which refuses `MODAL_STURM_SHIFT_UNSEPARATED` at the test-local 10⁻⁸, is one cluster at 2⁻²⁰.
  - **τ_r = 2⁻³⁰ ≈ 9.31·10⁻¹⁰ is adopted (Agent 0, 2026-11-04, second decision)** as an explicit engineering measured-residual demotion criterion on the re-formed η (check 2). It corresponds to a normalized-residual quality target below 10⁻⁹. It is not a stable-algorithm guarantee, not an O(n·u) bound, and not a value fitted so that every mode comes out Good.
    - A mode whose η exceeds τ_r stays Sensitive, as governed. A residual alone never causes a refusal.
    - τ_r alone promises no eigenvalue or frequency accuracy. The full κ_λ, e_λ, orthogonality and count checks, and the governing standing above the band, all remain.
    - The earlier "constant ≤ 10³" claim stays withdrawn, and no global growth proof is needed for this criterion.
    - Focused text, source and regression confirmation comes before any production instance.
- **A3, decided (Agent 0, 2026-11-05), on the proposal below (D1–D5):** τ_μ = 2⁻⁵³, c_μ = 1 with κ_μ = 10, and c_f = 64, as **explicit retention and screening criteria**. They are not binary64 resolution theorems, true-rank decisions or all-count certificates. "A3, as decided" below the proposal states the exact scope. The unresolved finite census of the component body and of point lumps offset from their nodes stays a limitation of a structural premise. It is not removed by tuning a parameter or truncating rank.
- **A complete production parameter instance needs A1, A2 and A3 all closed.** All three are now selected (Agent 0, 2026-11-05). The instance may be implemented as a bounded explicit constructor with no `Default`, under the existing product implementation, review and D3 scope, with the source and regression evidence that "A3, as decided" lists. Selecting the parameters enables nothing. Public and native admission stay disabled until every other §5.3.9 prerequisite closes (the resource guards and their weights, the subject census, the full reader, Bessel). Until that instance is reviewed, only the test-local set exists.

n_max = 8,192 is the largest order the dense guard admits (below, τ_m); n_max·u = 9.09·10⁻¹³. Values are powers of two where a margin rule sets them.

**How to read the table** (corrected after independent review, 2026-11-04). Each row separates three things. *Established:* what follows from a stated argument (a rounding bound, a theorem, a definition) for every admitted model. *Assumed:* an allowance that is a judgment, not derived (such as a growth or underestimate factor), and what it covers. *Empirical margin:* the ratio to the extreme observed value on the witnesses (I1's kernel cases, I2b-1's product observations, I2a's identity measurement, the eleven fixtures and I3a's observations below); it describes those witnesses only and is not a domain-wide guarantee. *Needed before pinning:* the evidence that would turn an assumption into an established allowance, or the review that accepts it as a stated assumption.

| Parameter | Test-local | Agent 0 (2026-11-04) | Derivation | Evidence and margin |
|---|---|---|---|---|
| τ_r (residual) | 10⁻¹⁰ | **A2: 2⁻³⁰ ≈ 9.31·10⁻¹⁰** (Agent 0, 2026-11-04, second decision), an engineering measured-residual demotion criterion on the re-formed η. Exceeding it means Sensitive, never a refusal. It is not an O(n·u) bound or an accuracy promise. The ≤ 10³ claim stays withdrawn | *Established:* a backward-stable reduction gives η = O(n·u) for a constant that depends on the reduction's conditioning (‖R⁻¹‖ in the K-based reduction). *Assumed (withdrawn as a basis):* the proposal took that constant to be at most 10³ at every admitted order, so 2⁻³⁰ ≥ 10³·n_max·u. **I3a contradicts it on an existing witness:** EX n05's unreported computed mode 5 has η/(n·u) = 2.44·10⁵ (item 1 below). A fixed τ_r therefore has no n·u derivation; at n_max, 2⁻³⁰ corresponds to η/(n·u) ≤ 1,024, and a model conditioned like EX n05 at that order would be demoted to Sensitive (the safe direction, a loss of service, not a refusal). A breach demotes, it does not refuse | *Empirical margin:* the largest η on any computed pair, 1.90·10⁻¹⁰ (EX n05 mode 5, unreported), is 4.9× below 2⁻³⁰; on a reported mode 2.45·10⁻¹¹ (EX n05 mode 3), 38× below; the other 34 models ≤ 5.5·10⁻¹³ (E-CANT-8, unreported). *Decided (Agent 0, 2026-11-04):* 2⁻³⁰ as the measured-residual engineering target, set jointly with the κ_λ and e_λ checks. Not a universal growth bound, and not a value fitted to remove sensitive modes. Focused text, source and regression confirmation comes before the production instance |
| τ_o (orthogonality) | 10⁻⁷ | **A2: 2⁻²³ ≈ 1.19·10⁻⁷**, an engineering acceptance and demotion criterion on the measured deviation on the re-formed M̂; a large estimated bound alone demotes nothing | **Corrected (second review, 2026-11-04).** QL's orthogonality alone does not bound the M-orthogonality by n·u·κ(R). Counterexample: R = I and M̃ = diag(1, 10⁻⁸); a rotation of the exact eigenvectors by an angle u gives a deviation of u/√(10⁻⁸) ≈ 1.11·10⁻¹², against n·u·κ(R) = 2.2·10⁻¹⁶. The bound depends on the reduced spectrum. With C = R⁻ᵀM̃R⁻¹ (eigenvalues μ = 1/λ), φᵢᵀM̃φⱼ = zᵢᵀCzⱼ and m_i = μ_i for unit z, so the deviation is \|zᵢᵀCzⱼ\|/√(μᵢμⱼ). *Established (first order, the reduced problem):* QL with accumulated orthogonal transformations returns z that are orthonormal to O(n·u) and exact eigenvectors of C + E, with ‖E‖₂ ≤ c·n·u·‖C‖₂. Then \|zᵢᵀCzⱼ\| = \|zᵢᵀEzⱼ\| + O(n·u)·μ_max ≤ c′·n·u·μ_max, so the deviation is at most c′·n·u·μ_max/√(μᵢμⱼ) = c′·n·u·√(λᵢλⱼ)/λ_min. The counterexample attains this form. No eigenvalue gap enters this bound for accumulated orthogonal transformations: gaps govern each vector's accuracy (sin θ ≲ ‖E‖/gap), not the C-orthogonality of the computed set. A gap term ‖E‖/gap does enter for vectors not produced by those transformations (the recovered band modes, §3.2), and none is derived for them. *Assumed:* forming C from the computed R̂ and M̃, and back-transforming φ = R̂⁻¹z, add first-order terms that scale with n·u·‖R̂⁻¹‖₂²‖M̃‖₂ and n·u·κ(R)·‖R̂⁻¹‖₂²‖M̃‖₂, each over √(μᵢμⱼ). No derivation bounds those factors for the product population; the constants c, c′ are taken as O(1). **Corrected (third review):** only the reduced-problem contribution has a threshold statement. c′·n·u·√(λᵢλⱼ)/λ_min stays below 2⁻²³ while c′·√(λᵢλⱼ)/λ_min ≤ 2⁻²³/(n_max·u) = 1.3·10⁵. The formation and back-transformation terms carry the factors ‖R̂⁻¹‖₂²‖M̃‖₂/μ_max and κ(R)·‖R̂⁻¹‖₂²‖M̃‖₂/μ_max, which nothing bounds for admitted models. So no condition is stated under which the combined bound stays below τ_o, and τ_o is not derived from it. **A large bound does not demote anything.** Sensitive is triggered only by the measured deviation, evaluated in double-double on the re-formed M̂, exceeding τ_o. A high κ(K̃) or a wide spectral spread makes demotion possible, not certain. *Observed (the committed fixtures, from their published ω and n_free; no new run):* the whole-spectrum cantilever deviates 7.7·10⁻¹⁴ against n·u·√(λᵢλⱼ)/λ_min = 2.2·10⁻¹¹ (spread 1.6·10⁴); the arc 5.7·10⁻¹⁵ against 1.3·10⁻¹³. EX n05's bound is 3.8·10⁻⁷, above the proposed τ_o (spread 4.9·10⁸ between its reported modes), while its measured deviation is 0. *Empirical margin:* class (a) ≤ 2.1·10⁻⁹ (E-RIGID-SCALE-1e6), 57× below; fixtures ≤ 7.7·10⁻¹⁴. *Needed:* the measured deviation against n·u·√(λᵢλⱼ)/λ_min and ‖R̂⁻¹‖₂²‖M̃‖₂/μ_max per witness, to confirm the form and the O(1) constants, a stated treatment of band-recovered vectors, and Agent 0's acceptance that the measured deviation, not the bound, decides the standing (**accepted, 2026-11-04**). **I3a:** item 4 below measures every computed pair, band-recovered ones included: c′ ≤ 0.394, and the largest deviation, 2.10·10⁻⁹, is 57× below 2⁻²³. The form and the O(1) constants remain assumptions, and the formation factors are still not computed |
| δ_c (cluster) | 10⁻⁸ | **A2: 2⁻²⁰ ≈ 9.54·10⁻⁷**, the computed-cluster grouping convention (above); not a physical-multiplicity classifier | *Established:* δ_c must exceed the computed split of an exact multiplicity, or the exact pair is two clusters and the gap between them refuses. *Empirical rule (proposal):* the smallest power of two ≥ 64× the largest split observed. It covers the observed exact-pair splits only; it is not a separation guarantee. **I3a:** an exact repeat's computed split scales with n·u·λ/λ_min (item 5: 0.06–0.14·n·u·S on rotated controls at spread S), so a fixed relative δ_c splits an exact pair once n·u·S exceeds about 7·δ_c (then the gap refuses: safe, a loss of service), and two distinct modes closer than δ_c are merged, extended and disclosed | *Empirical margin:* EX n05's second bending pair splits 1.38·10⁻⁸, inside its upper member's own e_λ = 1.42·10⁻⁸ (at 10⁻⁸ it refuses `MODAL_STURM_SHIFT_UNSEPARATED`; at 2⁻²⁰ it is one cluster), 69× below 2⁻²⁰; it is not an independently established multiplicity. The 82 reference multiplicities split at most 1.40·10⁻¹⁰; the smallest distinct reference gap is 7.96·10⁻⁴ (E-SPRINGS). *Decided (Agent 0, 2026-11-04):* merging modes closer than δ_c is accepted, with completion and extension disclosed. No spread-dependent rule is started; where an exact pair splits beyond δ_c, the gap's separation premise decides, and it refuses by name if unestablished |
| τ_t (no translation) | 10⁻⁶ | **A1: 10⁻⁶** | §1.5's M1b rule: at least 100× the largest translation of a mode rotational in exact arithmetic | Case (b) max t/(ℓθ) = 7.8·10⁻¹⁶ (fixtures), 1.3·10⁹× below; a rounding-level bound n_max·u, 1.1·10⁶× below; the smallest case (a) ratio 0.013 (arc mode 4), 1.3·10⁴× above |
| Identity tolerance | (10⁻¹², 0) | **A1: (2⁻⁴⁴ ≈ 5.68·10⁻¹⁴, 0)** | The smallest power of two ≥ 100× the largest measured intact discrepancy; absolute 0 keeps the scale-relative check scale-free (§2.5). It is an acceptance criterion for the formed mass against the self-weight basis, not a detection guarantee: an error whose effect on a compared row is below 2⁻⁴⁴ of that row's scale passes, and an error that leaves a·M·r_k unchanged (rotary and torsional inertia, which vanish for a rigid translation, §2.5) is invisible to the identity by construction | Intact: I2a 5.45·10⁻¹⁶ nodal, 4.38·10⁻¹⁶ resultant; fixtures ≤ 2.2·10⁻¹⁶. **Corrected (I3a):** the earlier statement that a 1 % density error, a dropped body or a body on the wrong node is always ≥ 10⁻² relative is withdrawn. The injected witnesses (`modal::observation_tests::identity_discrepancies_of_injected_mass_errors`; each error injected into the lowered model so the formed mass is assembled from it while the inspection reads the authored model; largest relative discrepancy per compared row, own row / model row): PF pipe-only, pipe:p1 density ×(1 + 10⁻⁹): 6.48·10⁻¹⁰ / 3.24·10⁻¹⁰ (the I2a test refuses it at the test-local 10⁻¹²); density ×1.01: **6.48·10⁻³** / 3.24·10⁻³ (the density scales the metal share of m̄ only); component PF valve, W = 500 N: weight ×1.01: 1.00·10⁻² / 4.13·10⁻³; centre of gravity (−0.5, 0.25, 0) → (−0.5, 0, 0) m: 0.500 / 5.17·10⁻²; on node b instead of c: 0.658 / 0.207. The same three on a light body, W = 0.5 N: the body's own row is unchanged (it is judged on its own scale), the model row falls to **7.04·10⁻⁶**, 8.80·10⁻⁵ and 3.52·10⁻⁴. A body record dropped from the mass is refused by the body census ("a body on one basis only") before any tolerance applies. So a light body's or a diluted member's effect can be far below 10⁻² in a row; the smallest witness value, 3.24·10⁻¹⁰, is 5.7·10³× above 2⁻⁴⁴ |
| κ_c (conditioning) | 10³ | **A2: 2¹⁰ = 1,024**, an engineering acceptance criterion within check 4's premises, not a count theorem | **Corrected (second review, 2026-11-04).** The earlier row stated two √n conversions but concluded a single √n factor, which was inconsistent. Symmetry removes both. *Established:* Ã and the backward perturbation E are symmetric (L̂D̂L̂ᵀ is symmetric by construction, so E = L̂D̂L̂ᵀ − Ã is). The count is the exact inertia of Ã + E (Sylvester), and it equals Ã's when ‖E‖₂ < σ_min(Ã) = 1/‖Ã⁻¹‖₂ (Weyl). For a symmetric matrix the 2-norm is the spectral radius, which every induced norm bounds, so ‖E‖₂ ≤ ‖E‖_∞ and ‖Ã⁻¹‖₂ ≤ ‖Ã⁻¹‖₁. With e_b bounding ‖E‖_∞ to first order, ‖E‖₂·‖Ã⁻¹‖₂ ≤ e_b·‖Ã⁻¹‖₁, and **no √n factor is needed**. The premise e_b·ν ≤ 1/κ_c therefore establishes the count whenever ‖Ã⁻¹‖₁/ν < κ_c. *Assumed:* κ_c covers only the estimator's underestimate (and e_b's first-order omissions). The Hager–Higham estimate is a lower bound with no worst-case ratio, so the allowance of 1,024 is a judgment, not derived. *Empirical margin:* correct counts have e_b·ν ≤ 2.1·10⁻⁴ (I1), 4.7× below 1/κ_c = 9.77·10⁻⁴; product ≤ 2.5·10⁻⁶; every observed miscount ≥ 35.8, 3.7·10⁴× above. *Needed:* the estimator's ratio ν/‖Ã⁻¹‖₁ on the witnesses (computable exactly for small orders) before the 1,024-fold underestimate allowance is accepted. **I3a:** item 2 below compares ν with an independently computed ‖Ã⁻¹‖₁: the underestimate factor is 1.0000–3.01 on the counted shifts, 340× below 1,024. The estimator still has no worst-case ratio, so the adopted allowance stays a judgment |
| κ_s (separation) | 10² | **A2: 2⁷ = 128**, an engineering acceptance criterion within check 4's premises, not a separation theorem | **Corrected (second review, 2026-11-04).** The earlier √n_max = 90.5 conversion is withdrawn. s_j is the first-order 2-norm size of a symmetric perturbation that would move λ_j to σ, and for the symmetric E, ‖E‖₂ ≤ ‖E‖_∞ ≤ e_b, so no norm conversion is needed. *Assumed:* κ_s = 2⁷ covers the first-order omission in s_j and the reliance on λ_j's own estimate e_λ,j (check 2). Both are judgments, not derived. *Needed:* s_j against the exact eigenvalue displacement on the witnesses. **I3a:** item 3 below compares s_j with the true separation |α|: s_j overstates it by up to 15.3×, which 2⁷ covers 8.4×. Nothing bounds that factor's growth for wider gaps or spreads | Correct: ratio ≥ 5.7·10³ (I1), 44× above; product ≥ 5.3·10⁵; fixtures ≥ 1.8·10¹¹; miscounts ≤ 6.2·10⁻³ |
| g_max (growth) | 10¹⁵ | **A1: 2⁵² ≈ 4.50·10¹⁵** | 1/(2u): beyond it the factor's pivots exceed the scaled matrix by more than the inverse unit roundoff; e_b's factor term carries the growth below it (§3.4: g does not discriminate counts) | Correct g ≤ 4·10¹³ (I1 rigid scales), 113× below; product ≤ 1.6·10⁷; fixtures ≤ 1.4·10³. It is also below 2⁵³, so the checked profile carries it |
| s_max, gap positions, offsets | 7; ¼, ¾, ⅛, ⅜, ⅝, ⅞; 1, ½, ¼ | **A1: unchanged** | Dyadic positions of levels 2 and 3 (≤ `MAX_GAP_POSITION_LEVEL`) after the geometric mean: the gap at log-resolution ⅛; s_max = 1 + 6 bounds the Sturm work at 7·n³/3 | Every product final shift was accepted at its first candidate (§3.4, as built; fixtures) |
| τ_e (band screen) | 10⁻⁶ | **A1: 10⁻⁶** (a stated accuracy requirement, not a fitted value) | Recovered band pairs must estimate about six significant digits of λ. **Band only:** τ_e judges only the band-recovered pairs of §3.2; a pair above the band is retained from QL without it, and every computed pair, above the band or in it, keeps its full re-formed evidence (§3.4 check 2: η, κ_λ and e_λ on the re-formed pencil) and its standing (a reported mode with η > τ_r is Sensitive). τ_e is not a whole-spectrum accuracy criterion | Genuine band e_λ ≤ 3·10⁻⁸ (33× below); I3a product band pairs (E-RIGID-SCALE-1e5, -1e6) ≤ 1.50·10⁻⁸ (67× below); diagnosed false pairs ≥ 3·10³ (§3.2) |
| τ_m (mass pivot) | 10⁻¹² | **A1: 2⁻³⁶ ≈ 1.46·10⁻¹¹** | An equilibrated mass pivot of a structurally singular block rounds at about n·u, so τ_m ≥ 16·n·u, and 2⁻³⁶ = 16·n_max·u exactly at n_max = 8,192, with n the order of the equilibrated block (n_f less the structurally massless DOFs ≤ n_f ≤ the global order). **The criterion holds only where a guard establishes n ≤ 8,192 at every use of τ_m.** *Source check (I3a):* τ_m has one use, `established_finite_mode_count` (`modal_eigen/src/dense.rs:751`), called only from `solve_dense_pencil` (`:491`). The product reaches that only through `solve_modal_pencil`, after `form_modal_pencil` has applied `dense_scrutiny_guard` to the global order 6N before any n² allocation (`modal/stiffness.rs:137–141`): 96·n² ≤ 6 GiB admits n ≤ 8,192, so the block order ≤ 8,192 on the product path. Not guarded: the kernel's public `solve_dense_pencil` and `established_finite_mode_count` carry no order limit of their own, so any other caller (M2's sparse path, a native or headless entry) must establish n ≤ 8,192 itself; the 6 GiB ceiling is provisional (ROOT Q8(a)); and the test-only `DENSE_SCRUTINY_CEILING_OVERRIDE` (compiled in tests only) can set any ceiling. **If a resource or admission guard later widens n beyond 8,192 where τ_m is used, the 16·n·u basis is revised and reviewed first** | Genuine equilibrated pivots ≥ 0.22 (the small-bore witness), 1.5·10¹⁰× above |
| τ_μ, c_μ, κ_μ, c_f | 10⁻¹², 1, 10, 64 | **A3 (2026-11-05): τ_μ = 2⁻⁵³, c_μ = 1, κ_μ = 10, c_f = 64**, explicit retention and screening criteria; scope under "A3, as decided" below | At the test-local values, τ_μ sets the hard cut only where κ_μ·δ_q (= 10·n·u·μ_max) does not already dominate (n ≤ 900), and is inert at n ≥ 901. Under the decided tuple it never sets h above κ_μ·δ_q for n ≥ 1, in the kernel's actual binary64 evaluation (scope under "A3, as decided"). The classification depends on the paired product κ_μ·c_μ = 10 only, an engineering criterion, not a derived worst-case rounding bound; c_μ, κ_μ and c_f as §3.2 states them. Proposal and decision: "A3: minimum resolution and screening proposal" below | The μ/μ_max observation table (item 6 below). **The high-mode issue:** EX n05's top retained mode has μ/μ_max = 3.86·10⁻¹², 3.9× above the test-local τ_μ = 10⁻¹² |
| QL limit | 30 | **A1: 30** | 30 is tql2's established iteration limit; a failure refuses (`MODAL_EIGENSOLVE_UNAVAILABLE`) | Every product solve within it |

**A2 and A3 evidence (I3a, 2026-11-04; TEST-LOCAL observations, debug build, aarch64-apple-darwin; nothing is pinned or proposed by it).** Every observation comes from existing correctness fixtures plus small additions, in maintained deterministic tests: `modal::observation_tests` (product path) and `modal_eigen/tests/sturm_counts.rs` (`conditioning_and_separation_premises_against_independent_references`, `exact_repeat_and_close_distinct_controls_in_a_rotated_basis`). The product population is 35 models: the 26 class (a) cases, SEC-A at N = 1, 2, 4, 8 (whole spectrum), EX n05, the arc, the component body and the two I2b-2 state cases, n ≤ 48. They run through `replay_modal_phases`, which `the_phase_replay_is_the_product_path` holds bit for bit to `run_modal_dense`, so every computed pair, reported or not, and the re-formed pencil are visible. Each test asserts regression tripwires at the next power of ten (or more) above the observed extreme; a tripwire is not a threshold or a pin. **What applies to every item:** an empirical ratio describes this population only. It cannot turn an assumption into a domain-wide theorem: an estimator with no worst-case bound stays an estimator, and witness evidence informs the choice without proving it.

1. **τ_r: η/(n·u) over every computed pair.** *Shows:* the largest is 2.44·10⁵, on EX n05's unreported computed mode 5 (η = 1.90·10⁻¹⁰, n = 7, the condensed stiffness's inverse-norm estimate ν(K̃̂) ≈ 4.3·10¹⁰, κ_λ up to 8.6·10¹⁰). The next are E-CANT-8 (104, ν ≈ 1.3·10⁴), SEC-A N = 8 (89.9), E-POINTMASS-AXIAL (42.7) and E-CANT-4 (27.7). The rigid-scale family 10³–10⁶ stays ≤ 0.24, band pairs included, although its ν(K̃̂) reaches 1.5·10⁸; so η/(n·u) does not track ν alone. The largest reported η is 2.45·10⁻¹¹ (EX n05 mode 3). *Assumed:* nothing new. The proposal's "constant ≤ 10³" is contradicted on an existing witness and is withdrawn as a basis (table). *Limits:* n ≤ 48, one exact-route witness, and ν is an estimate. Nothing here predicts η at larger orders.
2. **κ_c: ν against the true ‖Ã⁻¹‖₁.** *Population:* the 376 factored shifts of the inertia suite's fixture (8 witnesses and 9 class (a) pencils; one shift that lies exactly on an eigenvalue, where Ã is singular, is skipped), of which 222 are counted. *Independent basis:* Ã(σ) is formed densely in the kernel's own operation order (a = k − σ·m, then (d_i·a)·d_j), so the reference describes the matrix the factorization was given. ‖Ã⁻¹‖₁ comes from Gauss–Jordan elimination with partial pivoting in double-double, used only where n·κ₁·2⁻¹⁰⁴ ≤ 10⁻⁶ and the reference's own residual ‖I − Ã·X‖₁, evaluated in double-double, is ≤ 10⁻⁶ (both hold on all 376 shifts; the test fails otherwise). It shares nothing with the estimator: no profile LDLᵀ, no unpivoted order, no solve with the binary64 factor, none of the estimator's vectors. *Shows:* on the counted shifts the underestimate factor ‖Ã⁻¹‖₁/ν is 1.0000 to 3.01 (largest: E-FF-4, a gap geometric mean), 340× below κ_c = 1,024. On a counted shift ν exceeds the true norm by at most 1.5·10⁻⁶ relative, the solves' rounding. On 65 refused shifts ν exceeds ‖Ã⁻¹‖₁, by up to 8.1× (E-RIGID-SCALE-1e6, `cluster_18_binary64_nearest`), because the binary64 solves are inaccurate there. So ν is no lower bound where the premise refuses. That direction is safe, since a larger ν refuses sooner. *Established, unchanged:* ‖Ã⁻¹‖₂ ≤ ‖Ã⁻¹‖₁ for a symmetric Ã, so the 1-norm factor bounds the 2-norm factor the premise needs. *Assumed:* the Hager–Higham estimator has no worst-case ratio, so the 1,024 allowance remains a judgment, and e_b bounds ‖E‖ only to first order. *Limits:* n ≤ 48 and the fixture's pencils.
3. **κ_s: s_j against the true separation.** *Independent basis:* by Sylvester's law Ã(σ) has c negative eigenvalues, with c the fixture's exact count (from the references' generator, not the binary64 inertia). The eigenvalue of Ã that changes sign at σ = λ_j is α_c for the neighbour below and α_{c+1} for the neighbour above. α comes from cyclic Jacobi on Ã, a standard symmetric eigenproblem, and the test fails unless Jacobi converges (off-diagonal Frobenius norm ≤ 10⁻¹⁴·‖Ã‖_F): no pencil eigenpair, no first-order formula and no inertia count enters. By Weyl's theorem |α| is the 2-norm of the smallest symmetric perturbation of Ã that moves that eigenvalue to zero, which is the quantity s_j estimates to first order. A pair is used only when α has the sign the count requires and |α| ≥ 10³·n·u·‖Ã‖_F: that holds for 559 neighbour separations, 420 of them on counted shifts. The 150 unresolved all lie on refused shifts. The test re-evaluates s_j and requires its minimum to reproduce the kernel's. *Shows:* on counted shifts s_j/|α| runs from 1.7·10⁻⁵ to **15.3** (E-RIGID-SCALE-1e6, a gap geometric mean; then WIT-SCALE-1E3 10.5, E-SPRINGS 5.2, E-Z-BEND-K1 below its lowest mode 4.1). 157 of the 420 exceed 1. Leaving out the e_λ allowance (s⁰_j = |λ_j − σ|·q_j) changes nothing at the printed precision. So the first-order s_j overstates the true separation by up to 15× at shifts in wide gaps, and κ_s must cover that omission: the test-local 10² covers the observed factor 6.5×, the proposed 2⁷ 8.4×. The miscount population's separation ratios (≤ 6.2·10⁻³) remain far below either. *Assumed:* that the observed overstatement does not grow much beyond 15× for wider gaps and larger spreads. The largest factors occur in the rigid-scale and wide-gap cases, and nothing bounds the growth. *Limits:* n ≤ 48, Jacobi's absolute accuracy, and e_b as a first-order bound of ‖E‖.
4. **τ_o: measured orthogonality, formed and re-formed, every computed pair.** Per model: check 3's measure on the published shapes (re-formed M̂), the same over every computed vector on M̂ and on the formed M_ff, and c′ = deviation / (n·u·√(λᵢλⱼ)/λ_min) per pair on M_ff (double-double, exact products). *Shows:* the largest c′ is 0.394 (SEC-A N = 2, pair (2, 4)), so c′ < 0.40 over every pair of all 35 models, consistent with the reduced-problem form with c′ = O(1). On EX n05 c′ is 5·10⁻⁴: the form is pessimistic there. The formed (M_ff) and re-formed (M̂) measurements over all computed vectors agree to the printed three digits on 31 models and differ on four, all at rounding level: E-CANT-1 9.35·10⁻¹⁶ on M̂ against 8.98·10⁻¹⁶ on M_ff, SEC-A N = 1 2.32·10⁻¹⁵ against 2.02·10⁻¹⁵, E-CANT-2 5.62 against 5.61·10⁻¹⁴, and the component body 4.72 against 4.73·10⁻¹⁴. So the two measurements are close on this population but not interchangeable, and a criterion names the one it uses. The re-formed values come from the product's own check-3 measure, so they observe the production checker; they do not verify it independently. Unreported pairs exceed published ones: E-CANT-8 6.8·10⁻¹² against 3.4·10⁻¹³ published; EX n05 1.83·10⁻⁹ (modes 3 and 5, mode 5 unreported) against 0 published. The overall largest, 2.10·10⁻⁹ (E-RIGID-SCALE-1e6), is attained on a pair with a band-recovered vector: band-recovered pairs carry the model maximum on 1e5 (1.37·10⁻¹⁰) and 1e6, and their c′ stays ≤ 0.12, although the reduced-problem derivation does not cover recovered vectors. 2.10·10⁻⁹ is 57× below 2⁻²³. *Assumed:* the formation and back-transformation factors (‖R̂⁻¹‖₂²‖M̃‖₂/μ_max) are not computed, and the measured deviation, not a bound, decides the standing. *Limits:* n ≤ 48, and only two band witnesses.
5. **δ_c: exact repeats and close-distinct controls.** *Independent basis:* the class (a) references state each exact multiplicity (`clusters[].multiplicity`, from their 80-digit generator), and the kernel controls construct K = Q·Λ·Qᵀ from a fixed Householder Q with M = I, so the exact eigenvalues are Λ to rounding (≲ n·u·‖Λ‖). *Shows (product):* the 82 reference multiplicities split at most 1.40·10⁻¹⁰ (E-POINTMASS-AXIAL at λ ≈ 1.05·10⁹), and a split can exceed its members' own e_λ (by up to 1.39×, E-CANT-2). The smallest distinct reference gap in the population is 7.96·10⁻⁴ (E-SPRINGS), computed to 2.3·10⁻¹⁰ relative: no population pair lies near 2⁻²⁰. *Shows (kernel controls, n = 8, top pair at spread S = 10²…10¹⁰):* an exact repeat splits by 0.057–0.137·n·u·S; a distinct pair with gap g ≥ 10³·n·u·S is computed to ≤ 3.7·10⁻⁶ relative error of its gap (≤ 3.3·10⁻⁵ down to g ≈ 270·n·u·S); below that resolution the computed gap is not reliable (at S = 10¹⁰, g = 2⁻²⁴, it is 9.9× the true gap). So a fixed relative δ_c separates exact repeats from distinct pairs only while n·u·λ/λ_min is well below δ_c. At 2⁻²⁰ that means n·S below about 6·10¹⁰ on these controls; beyond it, exact pairs split (and refuse) and close-distinct pairs are unresolved. *Assumed:* that product models' spreads behave as these controls do. *Limits:* one construction, n = 8, and the population's spreads (EX n05 is about 2.6·10¹¹, whose computed split of 1.38·10⁻⁸ is not an established multiplicity).
6. **A3: the μ/μ_max observation table** (`mu_domain_observation_table`; τ_μ = 10⁻¹², c_μ = 1, κ_μ = 10, c_f = 64, test-local). The hard threshold over μ_max is 10⁻¹² on every model (τ_μ dominates κ_μ·c_μ·n·u at n ≤ 48), and the band edge κ_μ·δ_μ/μ_max differs from it on the rigid-scale family (9.0·10⁻¹¹ to 8.9·10⁻⁸) and, marginally, on E-L-LINE-K1 (1.04·10⁻¹²).

   | Model (n) | min retained μ/μ_max | band edge/μ_max | screened / band kept / rejected (≤ 0, floor or band) | largest rejected μ/μ_max | census | band η̃/(n·u), e_λ |
   |---|---|---|---|---|---|---|
   | EX n05 (7) | **3.86·10⁻¹²** | 10⁻¹² | 0 / 0 / 0, 0 | – | established 7 | – |
   | E-RIGID-SCALE-1e6 (21) | 2.69·10⁻¹¹ | 8.90·10⁻⁸ | 6 / 6 / 0, 0 | – | established 21 | 0.051, 1.50·10⁻⁸ |
   | E-RIGID-SCALE-1e5 (21) | 2.69·10⁻¹⁰ | 8.91·10⁻⁹ | 5 / 5 / 0, 0 | – | established 21 | 0.152, 9.57·10⁻⁹ |
   | E-RIGID-SCALE-1e4, -1e3 (21) | 2.69·10⁻⁹, 2.69·10⁻⁸ | 8.93·10⁻¹⁰, 9.00·10⁻¹¹ | 0 | – | established 21 | – |
   | E-POINTMASS-NODE (24) | 6.54·10⁻⁷ | 10⁻¹² | 0 | – | established 21 (3 massless rotations) | – |
   | E-POINTMASS-AXIAL (24) | 6.98·10⁻⁷ | 10⁻¹² | 2 / 0 / 0, 2 | 3.51·10⁻¹⁷ | unresolved | – |
   | E-POINTMASS-LATERAL (24) | 4.61·10⁻⁷ | 10⁻¹² | 1 / 0 / 1, 1 | 4.24·10⁻¹⁷ | unresolved | – |
   | E-POINTMASS-GENERAL (24) | 4.13·10⁻⁷ | 10⁻¹² | 1 / 0 / 2, 1 | 8.44·10⁻¹⁹ | unresolved | – |
   | component PF (12) | 3.00·10⁻⁴ | 10⁻¹² | 2 / 0 / 1, 2 | 6.63·10⁻¹⁷ | unresolved | – |
   | E-CANT-8, SEC-A N = 8 (48) | 1.74·10⁻⁶, 1.16·10⁻⁶ | 10⁻¹² | 0 | – | established 48 | – |
   | the other 23 models | ≥ 9.2·10⁻⁶ | ≤ 1.04·10⁻¹² | 0 | – | established | – |

   *Shows:* genuine retained values reach down to 3.86·10⁻¹² (EX n05's top mode, spread λ_max/λ_min ≈ 2.6·10¹¹), only 3.9× above the test-local τ_μ. The rejected values (rounding-level μ of the offset point lumps' non-coordinate nullspaces) lie at ≤ 6.6·10⁻¹⁷. The population's window between rejected and genuine is about 6·10⁴ wide, and EX n05's top mode sits near its upper end. The census limit stands as found: AXIAL, LATERAL, GENERAL and the component body's offset point lump leave the count unresolved (their whole-spectrum endpoint stays refused; lower bounds are served). Nothing is truncated, and nothing below either hard floor is recovered. Band pairs' η̃/(n·u) ≤ 0.15 against c_f = 64 and e_λ ≤ 1.5·10⁻⁸ against τ_e. *Assumed:* nothing; no value is proposed. *Limits:* the census limit and the high-mode issue are not resolved by any choice of τ_μ within this window. A model with a wider spread than EX n05 would put a genuine top mode below 10⁻¹², and a τ_μ chosen inside the window is a population fit, not a derived bound.

##### A3: minimum resolution and screening proposal (2026-11-05; decided as D1–D5, see "A3, as decided" at its end)

The proposal is kept as written for review; Agent 0's decision and its exact scope follow it. It implements nothing and starts no measurement. Source is read at `874378dde`, which contains I3a (`08bfdfdaf`). The figures are hand arithmetic on the recorded observations, with u = 2⁻⁵³ and n the full free order.

**1. What each parameter decides (source).** After the QL solve the kernel forms two cuts (`core/solver/modal_eigen/src/dense.rs:486–496`):
- the hard cut h = max(τ_μ·μ_max, κ_μ·δ_q), with δ_q = c_μ·n·u·μ_max;
- the broad cut b = max(h, κ_μ·δ_μ), with δ_μ = c_μ·n·u·ν(K̃̂)·‖M̃_mm‖_∞ (:478–482).

Each condensed μ then falls in one of these regions (`classify_mu` :656; the loop :556–584):

| μ region | Verdict | Census effect |
|---|---|---|
| μ ≤ 0 | infinite, not returned (:508) | unresolved |
| 0 < μ ≤ h | hard-rejected; never recovered, even with a zero residual | unresolved |
| h < μ ≤ b | band: μ_R is recovered on the formed pencil (`recover_band_mode` :796). The pair is retained only if μ_R > h, η̃ ≤ τ_f = c_f·n·u and e_λ ≤ τ_e (:537, :568–569, `screen_passes` :882); otherwise it is rejected | retained: none; rejected: unresolved |
| μ > b | retained from QL, without τ_f or τ_e; it keeps check 2's re-formed evidence and standing | none |

The census is unresolved when any pair is rejected or when the τ_m mass-block Cholesky fails (:588, :615–617, `established_finite_mode_count` :959). The product maps that to `FiniteCensus` (`product_physics/src/modal/eigen.rs:106–118`).

| Parameter (test-local) | Where it enters | What it decides | Source |
|---|---|---|---|
| τ_μ (10⁻¹²) | h = max(τ_μ·μ_max, κ_μ·δ_q) | Hard rejection, but **only where τ_μ > κ_μ·c_μ·n·u**, that is n ≤ 900 at the test-local values. At n ≥ 901 it is inert: κ_μ·c_μ·n·u is 1.07·10⁻¹² at n = 960 and 2.13·10⁻¹² at 1,920 | dense.rs:376, :489–492; the product validator requires (0, 1), `integrity.rs:146`; the kernel accepts [0, 1), dense.rs:377 |
| c_μ (1) | δ_q and δ_μ, always multiplied by κ_μ | **Nothing on its own.** The classification depends only on κ_μ·c_μ; c_μ alone rescales the reported δ_q and δ_μ | :478–482, :487–496 |
| κ_μ (10) | κ_μ·δ_q in h, κ_μ·δ_μ in b | the reduction's hard resolution floor, and the band edge, which decides which pairs need recovery | :489–496 |
| c_f (64) | τ_f = c_f·n·u | band acceptance on the formed η̃, inclusive | :537, :882–889 |

**Where a genuine finite mode can be lost.** These are the only two routes:
- (i) a genuine μ ≤ h, which is always dropped;
- (ii) a genuine band μ whose recovery fails: η̃ > τ_f, e_λ > τ_e, a non-positive g or h, μ_R ≤ h, or a non-normal value.

A genuine μ > b is never dropped by an A3 parameter. The cuts act on μ, so a dropped mode always comes from the top of the spectrum. Every drop raises `below_threshold_mu_count`, so the census is unresolved (:588, :616–617). The system then does three things:
- It refuses the whole-spectrum endpoint by name (`MODAL_FINITE_COUNT_UNRESOLVED`, eigen.rs:178–195).
- It still serves a set that ends below a completed computed cluster, with a disclosure (outcome.rs:534–536), and only if the Sturm count at the gap shift equals the reported count (eigen.rs:229–266). A mode dropped below the final shift is therefore caught as `MODAL_STURM_COUNT_MISMATCH`, not hidden.
- It never adds a dropped mode to a count.

The opposite risk is a spurious μ. A μ ≤ h is rejected without recovery. One in (h, b] goes to band recovery, whose screens are estimated-error criteria (c_f, τ_e and μ_R > h), not a theorem that rejects every spurious pair. Above b it is retained from QL, so h and b are the current kernel's admission floors in that direction, not a certification.

**2. Known failure behaviour.** In each case below the criterion claims no resolved count.
- **(a) SEC-A at n_f = 1,920 (§5.3.8).** The census reads `Unresolved { rejected_mu: 640, mass_block_established: true }`. The mass block establishes 1,920 finite modes; 1,280 are returned, so 640 genuine top modes are missing.
  - **Attribution corrected:** at n = 1,920, κ_μ·c_μ·n·u = 2.13·10⁻¹² exceeds τ_μ = 10⁻¹². The hard cut was therefore κ_μ·δ_q, and **τ_μ decided no pair**.
  - The record does not split the 640 into hard-rejected and band-failed pairs. `band_screened` is `screened.len()` (`phase_measurand_tests.rs:387`), and the kernel's `hard_rejected_pair_count` and `attempted_band_pair_count` (dense.rs:309–314) were not printed. For the same reason, 4pn² with p = 640 is an upper estimate.
  - dense_solve grew 19.7× against about 8× for n³. That is consistent with recovery being attempted on most of the 640, which would make band acceptance (route ii) the rejecting verdict. It is not established.
  - Either way, these modes fall below the current kernel's admission floors at this spread: rejected at h, or not retained by band recovery. No value of τ_μ, including 0, returns them. This describes the kernel as built, not a proven binary64 resolution limit.
  - The criterion drops them, leaves the census unresolved, refuses the endpoint, and serves the ten low modes under the lower-gap count.
- **(b) The offset point lumps and the component body (item 6).** M has a non-coordinate nullspace, so these μ are true infinite eigenvalues computed at rounding level. Every rejected value is at most 6.6·10⁻¹⁷·μ_max: at most 0.050·n·u·μ_max on component PF (n = 12) and at most 0.016 on the lumps (n = 24). They are hard-rejected, which is correct. The count is not established because the structural premise (coordinate nullspaces only) fails. Unresolved is the true answer, not a defect of τ_μ, and it is not removed by truncation.
- **(c) EX n05 (n = 7, spread ≈ 2.6·10¹¹).** At n = 7 the active hard cut is τ_μ (κ_μ·c_μ·n·u = 7.8·10⁻¹⁵). The top mode sits at 3.86·10⁻¹², 3.9× above it, and the census is established at 7. With a spread about 4× wider, τ_μ would hard-reject a mode lying about 124× above the derived floor κ_μ·c_μ·n·u·μ_max. The current top mode is about 497× above it, and being above that floor does not by itself establish that a mode is accurately resolved. The census would then be unresolved, the endpoint refused and the lower modes served. That is τ_μ's own failure mode: **it drops modes the reduction resolves.**

**3. The minimum proposal.** A fixed τ_μ cannot be defended across spreads.
- The only derived floor is the reduction's resolution. QL is backward stable on the symmetric C, with ‖C‖₂ = μ_max, so by Weyl each computed μ lies within c·n·u·μ_max of an eigenvalue of C. A μ at or below a modest multiple of n·u·μ_max cannot be told apart from 0. This is a rounding argument with an assumed O(1) constant, not a certified bound. QL's backward stability on the reduced C also does not certify C's formation, or its correspondence to the original pencil; the integrity checks of §3.4 cover those.
- A fixed τ_μ either lies below κ_μ·c_μ·n·u, where it is inert (every n ≥ 901 today), or above it, where it drops resolvable modes (case c).
- No fixed value changes case (a).

The proposal is therefore a role change plus three values:
- **P1. τ_μ becomes inert, and the hard floor is κ_μ·δ_q alone. Proposed value τ_μ = 2⁻⁵³ (= u).**
  - **Why it is inert:** κ_μ·c_μ·n ≥ 1 for every n ≥ 1 once κ_μ·c_μ ≥ 1, so u·μ_max ≤ κ_μ·δ_q and τ_μ never sets h. The value passes the product's (0, 1) validation, so no code changes.
  - **Case (a):** no change. h is the same at n ≥ 901, the 640 stay dropped and the census stays unresolved. **This is not a recapture.**
  - **Case (b):** h falls to 1.33·10⁻¹⁴ (n = 12) and 2.66·10⁻¹⁴ (n = 24). Every rejected value stays at least 200× below it, and the census stays unresolved.
  - **Case (c):** the top mode sits about 500× above h, not 3.9×. The high-mode issue at n ≤ 900 becomes the same spread limit as at large n.
  - **Population:** no verdict changes on the 35 models. Genuine values are ≥ 3.86·10⁻¹² and rejected values ≤ 6.6·10⁻¹⁷, with nothing in between.
  - **Cost:** at n ≤ 900 the extra fixed margin against a spurious pair above the derived floor is removed. Protection then rests on κ_μ and on the band with recovery. That trade is the decision.
- **P2. c_μ = 1 and κ_μ = 10, with the product κ_μ·c_μ stated as the only acting quantity.**
  - c_μ = 1 normalizes δ_q and δ_μ to plain n·u-scale quantities. κ_μ is the single allowance on the reduction's constant, and it carries both h and b.
  - **Basis:** an assumed allowance. Observed true zeros are at most 0.050·n·u·μ_max, 200× below 10. The band edge errs in the safe direction (the rigid-scale band pairs are recovered).
  - **The resulting admission limit on spread at order n** (the current kernel's floor, not a proven resolution limit) is λ_max/λ_min < 1/(κ_μ·c_μ·n·u): about 4.7·10¹¹ at n = 1,920 and 1.1·10¹¹ at n_max = 8,192. Beyond it, top modes are not retained and the census is unresolved.
  - κ_μ is not lowered toward the observed 0.05 to widen that limit. Doing so would be the excluded tuning.
- **P3. c_f = 64 = 2⁶.**
  - **Basis:** a recovered pair with an accurate vector has a formed η̃ at rounding level, because the sums are exact and only φ and λ_R carry rounding. Observed band η̃/(n·u) ≤ 0.152, 420× below 64.
  - c_f alone guarantees no vector accuracy. It is one conjunct of band acceptance, with e_λ ≤ τ_e (A1, which catches the normwise-blind pair, dense.rs `normwise_blind_pair_is_not_an_accurate_eigenvalue`) and μ_R > h. On the witnesses no false acceptance was seen; that is an observation, not a guarantee.
  - In case (a), the record shows 640 screened and 0 retained. It does not show which pairs were attempted, or which recovery condition (c_f, τ_e or μ_R > h) failed (see (a)). Loosening c_f to accept them is excluded either way.
- **P4. The census rule is unchanged.** The census is unresolved on any μ ≤ 0, any rejected screened pair or a failed mass block. Nothing is truncated, the whole-spectrum endpoint is refused, and lower sets are served only under the Sturm count and completed-cluster premises.

Taken together, the current kernel's **retention** criterion (not a proof of resolution) is: μ > h, and either μ > b, or band recovery passes η̃ ≤ c_f·n·u and e_λ ≤ τ_e and its recovered μ_R > h (`dense.rs:568–569`). Here h = max(τ_μ·μ_max, κ_μ·δ_q) and b = max(h, κ_μ·δ_μ). Every failure leaves the count unresolved and never produces a count claim.

**4. Changes kept apart from adoption (none adopted, none implemented).**
- **K1.** Only if Agent 0 prefers τ_μ = 0 or removing τ_μ: widen the product validator to [0, 1) (`integrity.rs:146`), or remove τ_μ from h (dense.rs:489–492) and from the recorded parameter names (`document.rs:357`, `reporting/modal_result/src/lib.rs:96`), which is a schema change. P1 at u needs neither.
- **K2.** A test-harness change only: record `hard_rejected_pair_count`, `attempted_band_pair_count` and each band pair's recovery outcome in the O-4 line (`phase_measurand_tests.rs:387–418`). No window is requested.
- **K3.** Resolving genuine modes below κ_μ·δ_q (case a) needs a different kernel route: refinement on the formed pencil (inverse iteration), a mass-side reduction where the block is established, or M2's shift-invert path. It is named only so that no parameter is mistaken for it. It is not proposed for A3.
- **K4.** Establishing the count with a non-coordinate nullspace (case b) needs a structural mass-model change, such as expressing an offset lump through a rigid-offset transform so that its nullspace becomes coordinate-aligned. That is M1a scope, not A3.

**5. Decisions needed from Agent 0.**
- **D1, τ_μ:**
  - (i) P1, τ_μ = 2⁻⁵³ and inert (recommended);
  - (ii) keep 10⁻¹² as an explicit population floor at n ≤ 900, accepting failure (c);
  - (iii) τ_μ = 0 with K1.
  - Not offered: a τ_μ chosen to recapture case (a), which does not exist; or a fixed τ_μ ≥ κ_μ·c_μ·n_max·u = 9.09·10⁻¹², which would reject EX n05's top mode.
- **D2, κ_μ·c_μ:** c_μ = 1 with κ_μ = 10 (recommended), or a power of two (2³ or 2⁴) in margin form.
- **D3, c_f:** 64.
- **D4:** whether to accept the case (a) attribution correction and K2.
- **D5:** whether the resolvable-spread limit and the unresolved census of cases (a) and (b) are recorded as experimental capability limits, not release limitations, until K3 or K4.

##### A3, as decided (Agent 0, 2026-11-05)

**D1–D3, the criteria.** τ_μ = 2⁻⁵³ (D1), c_μ = 1 with κ_μ = 10, keeping the current product 10 (D2), and c_f = 64 (D3). These are explicit retention and screening criteria. They are not binary64 resolution theorems, true-rank decisions or all-count certificates.
- **τ_μ is never the active floor, within this scope only:** this tuple, n ≥ 1, and the kernel's actual arithmetic. The kernel evaluates both terms of h in binary64, left to right (`dense.rs:486–492`): τ_μ·μ_max = u·μ_max, and κ_μ·δ_q = 10·(((1·n)·u)·μ_max). For μ_max > 0 the exact value of the second is at least 10× the first, and rounding is monotone, so the computed second term is never below the computed first. Under underflow both can be zero, and h is the same value. For μ_max ≤ 0 no μ is positive, so h decides nothing. Nothing is claimed for other parameters.
- **What the decision removes:** the separate 10⁻¹² low-n population floor, deliberately. It does not recapture the 1,920 top modes and does not truncate the mass nullspace. This supersedes the earlier "no lowering of μ" instruction for that removal only.
- **What it preserves:** h = max(τ_μ·μ_max, κ_μ·c_μ·n·u·μ_max); the band edge b and the full band-recovery criterion (η̃ ≤ c_f·n·u, e_λ ≤ τ_e and μ_R > h, `dense.rs:568–569`); every computed pair's full re-formed per-mode evidence, above the band included (§3.4 check 2); and the accepted lower-gap Sturm count and completed-cluster premises.
- **c_μ and κ_μ are a paired criterion.** Only the product κ_μ·c_μ = 10 acts. It is an engineering criterion, not a derived worst-case rounding bound.
- **c_f** is a conjunctive acceptance criterion, with e_λ ≤ τ_e and μ_R > h. Alone it guarantees no vector accuracy. No false acceptance was seen on the witnesses.

**D4, the 1,920 attribution and K2.**
- The attribution correction is accepted: at n_f = 1,920 the active hard cut was κ_μ·δ_q, and the earlier τ_μ-only attribution is withdrawn.
- The existing records (640 screened, 0 retained) do not say which pairs were hard-cut and which failed attempted recovery. Every cost claim that assumed p = 640 attempted pairs is restated as an upper or scenario value, not measured attempt work (§5.3.8: the 4pn² ≈ 9.4·10⁹ insertions, w_band, and the 27 s band / 12 s binary64 split).
- No raw record is rewritten, and no mode failure is assigned retroactively. No extra 1,920 run or window fills the gap.
- **K2 is approved:** test-only, opt-in diagnostic aggregate counters for hard-cut, attempted-band, rejected by actual first failure (η̃, e_λ or μ_R) and retained pairs.
  - The counters match the actual branch order and identity. Counts are bounded, arithmetic is checked, and they reset per call. They reuse the existing S1 counts where possible.
  - No public wire or algorithm change.
  - Evidence: small unit count examples and exact known-reference controls only. A future measurand includes these counts only if separately cleared.

**D5, capability limits.** Recorded here as experimental capability limits, not an accepted release limitation and not a standing new Must.
- **The retained-spread limit.** At order n the kernel retains top modes only while λ_max/λ_min < 1/(κ_μ·c_μ·n·u): about 4.7·10¹¹ at n = 1,920 and 1.1·10¹¹ at n_max = 8,192. These numbers are relative to the floor under the stated tuple and evaluation order. They are not a physical solver impossibility. Beyond the limit, top modes are not retained, the census is unresolved, the whole-spectrum endpoint is refused, and lower sets are served only under the Sturm count and completed-cluster premises.
- **Coordinate and general-nullspace counts** (case b) stay unresolved wherever their premise is absent.
- No optional rank truncation or rescue, and no mass-model change.
- K3 and K4 are not adopted or assigned. No new kernel route starts without its own scope.

**The production instance.** With A1, A2 and A3 selected, the instance may proceed as a bounded explicit constructor with no `Default`, under the existing product implementation, review and D3 scope. Its source and regression evidence:
- a true positive below the old τ_μ = 10⁻¹² and above the derived floor, now retained;
- true zeros still rejected;
- the n = 1,920 derived floor unchanged;
- the unresolved whole-endpoint refusal and the lower-path count premises.

Selecting the parameters does not enable public or native admission. The resource guards and their weights, the full reader and the Bessel item remain conditions (§5.3.9). No new measurement is authorized. The confirmation is focused on the implementation and its new claims, with reference evidence, reusing the earlier reviews of unchanged source.

**As built (PR #131; D3 at DEL-04-08 §8.57, AC-004 7/7 at `089a5698f`).** The constructor is `ModalIntegrityParameters::selected_a1_a2_a3_criteria()` (`core/product_physics/src/modal/integrity.rs`). Only tests call it, and nothing native, public or in a document reads it. Its evidence is in `modal::observation_tests`:
- **The true positive:** an exact 4-DOF pencil whose top mode has μ/μ_max = 2⁻⁴³, retained under the instance and hard-rejected by τ_μ alone under the test-local set.
- **True zeros:** the offset point lumps and component PF stay hard-rejected, with the census unresolved.
- **The n = 1,920 floor:** h is bit-identical under both sets over the **normal** μ_max range. At deep-subnormal μ_max (about 2.6·10⁻³¹² to 1.1·10⁻³¹¹) it differs by at most two subnormal ulps, because there the old τ_μ term was the active one. An exact-spectrum scale argument puts admitted pencils far above that range. It is not a certified exclusion for computed results.
- **The endpoint:** the whole-spectrum endpoint refusal and the lower-path Sturm premises hold under the instance.

On a verdict projection of the 35-model population plus EX n05 at a bound of 4, the only differences come from δ_c = 2⁻²⁰. EX n05's modes 4 and 5 form one cluster, and at bound 4 five modes are served where the test-local set refused `MODAL_STURM_SHIFT_UNSEPARATED`.

#### 5.3.8 Resources: the recount, and the O-4 stage-1 readiness plan

**Recount from source (no measurement).** With N nodes (6N global DOFs), n_f free DOFs, n_c computed modes (on the dense path every finite mode, ≤ n_f), n_r reported modes and P_M stored mass entries:
- **Rows:** N + 6N + P_M + 3 + n_r + **6N·n_r** (shapes) + 3n_r + (clusters ≤ n_c) + n_c + (≤ s_max) + identity, support, member, disclosure and diagnostic rows. The shapes dominate, then the computed modes.
- **Canonical bytes per row** (the committed fixtures): shape 338–357; mass entry 406–462; DOF 165–193; node ≈ 515; participation ≈ 840–900; computed mode ≈ 1,070; mode ≈ 1,150; Sturm candidate ≈ 2,285; the header ≈ 7 KB. The pretty committed text is about 1.6× the canonical. So a document is about 7 KB + 0.35 KB·6N·n_r + 1.07 KB·n_c + 0.43 KB·P_M + 3.8 KB·n_r.
- **Peak (producer):** the outcome, plus the document as a `serde_json` tree (each value object a six-member map: an estimated 1–2 KB per shape row in memory, several times its canonical size), plus one row's transient re-serialization for its checksum. **Work:** one serialization, one checked parse and one SHA-256 per row and per table: O(document bytes), about three passes.
- **Readers:** the parsed tree (as above) plus the Rust reader's parsed copy (16 B per shape entry, 32 B per mass entry) and one clone per value object for the units relation; closure costs one canonical re-serialization and SHA-256 per row; the relations cost about 7·P_M·n_r multiply-adds (m_i and three φᵀp_k per mode), 3·P_M for the totals, 6N·n_r for the normalization and n_cl·n_c for the cluster rows. The Python reader holds the same tree as Python objects, several times larger.
- **Growth.** Output is O(6N·n_r) rows. At the dense guard's largest order (n_f ≈ 8,190, N ≈ 1,365) a whole-spectrum request would publish about 6.7·10⁷ shape rows, about 23 GB canonical and far more in memory: **the projection needs its own pre-allocation guard on its row and byte counts** (stated formula, refusing, never truncating) before any native path forms one. Even N = 500 and n_r = 50 give 1.5·10⁵ shape rows, about 51 MB canonical, near the native custody path's 64 MiB displayable-result limit (#59, `MAX_DISPLAYABLE_RESULT_BYTES`).
- **The recount is from source.** The coefficients above are fixture observations, not bounds; the format bound that replaces them for a guard is under "Stage 1 results" below.

**O-4 stage 1: readiness plan (run once, 2026-11-04; results below; replaces the withdrawn 60-minute proposal, which was not approved).** Stage 1 measures small points only. Its measured phase costs are what any larger stage is then proposed from. It reserves no 3-repeat series and no 3,000-DOF hour. Every O-4 value it yields stays **test-local and experimental** until separately reviewed measurements adopt one. The window pins nothing, sets no budget or guard coefficient, and enables no result.

1. **The measurand** (maintained, `#[ignore]`d, outside the normal test path): `modal::phase_measurand_tests::o4_phase_measurand_sec_a_cantilever` in `core/product_physics/src/modal/phase_measurand_tests.rs`.
   - *Model:* the SEC-A straight cantilever of `tests_eigen::sec_a_cantilever` (L = 6 m, OD 0.2 m, wall 0.008 m, steel with water contents, anchored at x = 0), with n_f/6 elements: n_f free DOFs and 6(n_f/6 + 1) global DOFs. The dense cost depends on n, not on sparsity. The parameters are the labelled TEST-LOCAL set (`TEST_LOCAL_I3A_EXPERIMENTAL`).
   - *Environment:* `MODAL_MEASURAND_NF` (a positive multiple of 6), `MODAL_MEASURAND_MODES` (the request's `mode_count`) and `MODAL_MEASURAND_PASS` ∈ {`phases`, `entry`, `both`}. None has a default.
   - *Pass `phases`:* `replay_modal_phases` times, with a monotonic clock, each of the product's own phase functions in `run_modal_dense`'s order: parse; prepare (including the state-case solve); mass and identity; pencil and screen (the dense guard, K and M and the static singular-model screen); dense solve (Cholesky, reduction, QL, band recovery, census); clusters and the reported set; normalization; re-formation (check 1); residuals (check 2, every computed pair); Sturm evidence (check 4, every candidate); orthogonality (check 3). The maintained `the_phase_replay_is_the_product_path` holds the replay to `run_modal_dense` bit for bit, so the timed sequence is the product's. Band recovery runs inside the kernel's dense solve and is timed with it. Its work is reported as a count (screened pairs p, exact insertions 4pn²). Timing it separately would need kernel instrumentation, which is not added.
   - *Pass `entry`:* `run_modal_dense` timed whole (the product path, including the row assembly the replay omits), then the document projection (`modal_doc_solved`) and its canonical text, each timed separately. The binding comes from the product's own projection of the same request refused at the dense guard (test-only ceiling override). The same test shows that this binding reproduces the full entry's document byte for byte.
   - *Reports* (`O4 key=value` lines): the build (`debug_assertions`, target, pointer width), n_f, n_global, `mode_count`, reported and computed counts, clusters, census and QL iterations; Sturm placement, candidates, factored shifts, inverse-norm solves and the shift pencil's pattern entries; re-formation operands, stored K and M entries and residual evaluations; output rows per table, total rows and canonical and compact bytes. Work is reported in three kinds that are never added together: the **binary64 flop estimate** 9n³ + s·n³/3 at the factored count (a stated formula, not a measurement); the **band recovery's exact-product insertions** 4pn²; and the **exact re-formation**, by its drivers (operands, stored entries, residual evaluations). It also prints the static guard's and the eigen core's stated byte formulas, labelled as formulas. **No ~64n² final guard is assumed.** The recovery, Sturm-evidence and output peaks are observed only as the process peak. *Peak RSS* is taken per process by the window's harness from `os.wait4` (`ru_maxrss`). No in-process sampler exists, so no per-phase RSS is reported.
   - *Priced-source impact:* none. The two new product files end in `_tests.rs`, which every retained-pricing scanner skips (`price.py`'s template inventory, `callgraph_g5.py`, `loopscan.py`'s test-file rule, the line map). `modal.rs` gains only two `#[cfg(test)]` module items, which the scanners blank. The pricing preparation at `372a16aa5` confirms that neither file enters the template inventory. Its 8 unclassified arguments are all at sites of the earlier I3a source (`modal/document.rs`, `modal/tests_document.rs`), which that change's own D3 still owes (§5.3.9 item 6). The measurand changes no reachable priced input, so it can be planned before that D3. It runs through the product's own dense guard and the labelled test-local set, and no measurement is a registration.
2. **The binary, built outside the window under normal slots** (not exclusive; the exclusive clearance is granted later and separately), from `core/product_physics` at the reviewed head:
   - release: `/Users/ryan/dev/chirality-t3/tools/t3_cargo.sh test --release --lib --no-run`;
   - debug, only if the debug point is taken: `/Users/ryan/dev/chirality-t3/tools/t3_cargo.sh test --lib --no-run`.

   Each executable named by Cargo's `Executable` line is identified by `shasum -a 256` before the window, together with the head and `rustc -Vv`. The window builds nothing.
3. **Points, ascending, one release observation each:** n_f = 60, 120, 240, 480 and 960 (10, 20, 40, 80 and 160 elements), `MODAL_MEASURAND_MODES=10`. Each point is two processes, `phases` then `entry`, so that each pass has its own peak RSS. Each process is `<binary> modal::phase_measurand_tests::o4_phase_measurand_sec_a_cantilever --exact --ignored --nocapture --test-threads=1`, with the three variables set. The operation estimates at s = 1 factored shift are 2.0·10⁶, 1.6·10⁷, 1.3·10⁸, 1.0·10⁹ and 8.2·10⁹; their rate is unmeasured.
4. **The debug point, only if needed:** one `phases` process at n_f = 60 on the debug binary, taken last, and only if the release points leave time and the build ratio is wanted (the registered retained identity is a debug build).
5. **Deadlines** (wall clock per process, d): 20 s at 60 and 120, 30 s at 240, 45 s at 480, 90 s at 960, and 60 s for the debug point. The termination grace is g = 5 s for every process.
6. **Supervision (accepted in principle by Agent 0, 2026-11-04; the launcher, harness and inert-child tests are authorized for writing and review only, with no measurement, native run or exclusive window).**
   - **Deadlines.** A practical userspace abort at T₀ + 570 s and an intended total deadline of T₀ + 600 s, with 30 s of margin. An overrun is reported at once. **This is not a mathematically absolute promise about kernel exit.** A userspace design bounds when it acts (each wait, each signal sent), not when the kernel schedules the launcher, delivers a signal or completes an exit. T₀ is when the launcher invokes `t3_exclusive.sh`, so the queue wait for the locks is counted inside the window explicitly. `t3_exclusive.sh` holds slot 1 while it waits for slots 2–4 (`t3_exclusive_stage2.sh`). The window ends when the last lock the invocation holds is released. All times are on `time.monotonic()`, the system-wide clock on macOS, with T₀ passed to the harness as an argument.
   - **The owned group.** The launcher starts the invocation as the leader of a new process group G. The launcher created G, and it is the leader's parent, so G is verified as owned without matching any name or pattern. No process of the window leaves G. The lock scripts create no session or group (they `exec` and fork `lockf`, which keeps its group). The harness starts every measurement as its own direct child with no new session or group. Membership is inherited atomically at fork, so `killpg(G, SIGKILL)` reaches every process of the window: the four `lockf` holders, the lock scripts, the harness and any measurement. A `lockf` lock is released when its holder exits. The exit of G's leader alone never means that the window is over, because nested holders can outlive the outer `lockf`. **No pattern signal and no cleanup of anything outside G.**
   - **The harness** is the command under the locks. As its first action it writes a start marker to a file the launcher named. The marker records its pid, its parent chain up to G's leader (the lock-holder and lock-script pids) and G. It then runs points. Each measurement is started with `subprocess.Popen` (no `start_new_session`, no `process_group`) on the reviewed executable directly; `/usr/bin/time` is not used. The harness waits at most d. At the deadline it sends SIGTERM to the child's pid, waits g, sends SIGKILL and reaps the child with `os.wait4`, its only reaping call. **The lock-holding command stays alive until its children are reaped:** on every catchable path (normal end, an exception, SIGTERM, SIGINT) it kills and reaps any running child before it returns. It never exits on purpose with a child running, so it never releases the exclusive while one runs. **Remaining-time rule:** a process starts only if (now − T₀) + d + g + R ≤ 570 s, with R = 60 s the teardown reserve. Otherwise no further point is taken. Teardown copies the `O4` lines and the rusage records to the window's scratch, then writes a done marker.
   - **Lock coverage is proven after each measurement, not by atomic arming.** A process confirmed alive by identity (pid and start time) at time t was alive at every earlier time since its start. So after each measurement is reaped, the harness confirms every recorded lock holder and lock script alive by identity. That one check proves the locks covered the whole measurement. If it fails, the measurement is discarded and the window is an ABORTED FAILURE. The checks before a spawn, and the launcher's arming check, only avoid starting a doomed point; they prove nothing. A holder exit seen before the final post-reap check is an abort, and one after it is teardown. The launcher never stops handling exits. Losing the controller at any time before the harness's acknowledgement is an ABORTED FAILURE, and measurements already proven valid are kept and labelled valid-before-abort. A listed group member that is neither excluded by identity nor confirmed a zombie counts as live. Containment is confirmed only when every such member is gone, and a kill error leaves it unconfirmed and recorded.
   - **The launcher** is the independent watchdog. It runs outside the locks and acts only through system calls on G and on the pids its marker names (`killpg`, `waitpid`, kqueue `EVFILT_PROC`/`NOTE_EXIT` on those pids, file checks). (a) It records T₀ and starts G. (b) **The lock wait is bounded:** if no start marker exists by T₀ + 30 s, it kills G and the window is abandoned with nothing measured. The harness abandons in the same way, with nothing measured, if the remaining budget when it starts admits no first point. (c) **Abort on loss of control or lock coverage:** if the harness or any recorded lock holder exits before the done marker exists, the launcher kills G at once. That is an **ABORTED FAILURE**. The affected measurement is discarded, no later point is taken and nothing is retried. If the harness dies uncatchably, its running measurement can outlive the lock release by the kernel events between the exits, the launcher's wake-up and the kill. The record gives the observed times: when each exit notification was received and when the kill was sent, with their uncertainty. Receipt time is at or after exit time, and a kill is sent before the process dies, so these times bound the overlap in neither direction exactly. They are recorded raw and labelled as an estimate with that uncertainty. 0.1 s is a polling interval, not a bound. (d) At T₀ + 570 s, if G still has any member, it kills G. (e) **After the window ends, the parent checks:**
     - no member of G remains, zombies included, from a group-membership listing that includes zombies (not `killpg(G, 0)` alone: XNU's group signalling skips zombies, and on this host, Darwin 25.6, it returns `EPERM` rather than `ESRCH` when only zombies remain; the scripts use libproc's `proc_listpids(PROC_PGRP_ONLY)`, verified to list a zombie); the launcher reaps the leader, and launchd reaps any reparented member;
     - on the normal path, every measurement the harness started is reaped, from the harness's `wait4` records. On an aborted path, where the harness may have died before it could reap, those records prove nothing for the measurement it was running. The evidence then is the launcher's own: the zombie-inclusive check that G is empty;
     - every child of the harness belonged to G, from the pgid recorded at each start;
     - `exclusive.pending` does not name a live process of this window;
     - every recorded lock-holder pid has exited (lock closure).

     It writes its record: T₀, the markers, each exit and kill with its time, the overlap if any, and each check's result. If G still has a running member at T₀ + 600 s, it reports an overrun at once and keeps checking until T₀ + 660 s, when it marks the window unresolved. That limit is proposed, not decided. Every overrun or aborted failure goes to Agent 0, and the window is not reused.
   - **Limits of this supervision** (from the scripts' tests):
     - If the launcher itself dies, nothing enforces T₀ + 570 s or runs the post-window checks. The harness then kills and reaps its child and exits without a done marker. Its own remaining-time rule bounds the window, and the harness record is the only evidence.
     - A memguard event is not visible to the harness. The abort rule for it (item 7) applies only through a measurement that exits nonzero, or through the operator.
     - A refusal is caught only if the measurand exits nonzero.
   - **The inert-child tests** (authorized; no measurement):
     - **Scope.** Bounded tests of the launcher and harness with inert children: processes that sleep, ignore SIGTERM, exit early, allocate a known size, or are left running when the harness is SIGKILLed. The lock command is a stand-in nested `lockf` chain on test-only lock files, built like `t3_exclusive.sh`'s. The existing exclusive tools and their semantics are untouched; this is not a replacement lock manager. The tests run under the normal slot wrapper, with short deadlines.
     - **Coverage.** A timeout escalates TERM to KILL. Harness death and controller death each produce an ABORTED FAILURE with G empty and the stand-in locks closed. A lock-holder exit triggers the abort. The lock-wait abandonment, the remaining-time refusal, the zombie-inclusive membership check and the overrun report all work.
     - **The rusage reading.** `ru_maxrss` is in bytes on macOS, checked against an inert child allocating a known size. It is the measurement process's own peak, its whole life included, not a per-phase peak. The harness's own memory is not in it.
   - **Schedule not approved.** After the containment tests are reviewed, the exact binaries, points, deadlines and resource proposal (items 2–5) are brought for a separate decision. The ten release points and the debug point are a proposal, and not all of them are promised to fit. Their worst-case sum, with each process at its deadline and 30 s for the locks, is 30 + 460 = 490 s for the release processes, so the debug point (60 s + 5 s) is not admitted in the worst case. The models and numerical parameters stay source-bound and labelled experimental until every pin closes.
7. **Abort rules:**
   - a process that reaches its deadline is terminated and reaped as above, and no larger point is taken;
   - the next point is taken only if 8× the current point's slower process wall time (the cubic step for a doubled n) is within the next point's deadline;
   - any memguard event, test failure, refusal or panic stops the window;
   - an `O4 build` line that does not match the binary's profile (`debug_assertions=false` for release) stops the window.

   Nothing is retried within the window.
8. **O-4 evidence proposals (from the stage-1 table, for review; nothing adopted):**
   - (a) per-phase wall time against n_f, with the log–log slope between adjacent points, to separate the cubic phases (the dense solve, and the Sturm evidence if the profile is dense) from the quadratic and linear ones (re-formation, residuals, projection);
   - (b) each pass's peak RSS against the stated formulas (the static 96·n_global², the eigen core's 32·n_f², the shift pattern), as the basis for counting the dense modal path's real peak, instead of assuming one;
   - (c) the document's rows and canonical bytes against the per-row coefficients above, for the projection's size guard;
   - (d) a proposal for the next stage (larger points, repeats, any debug ratio), sized from the measured slopes, for a separate authorization.

   D-k's work budget, the dense modal memory constant and the projection guard's coefficients are proposed only from reviewed measurements, never from this stage alone.

##### Stage 1 results (2026-11-04)

**Standing.** Every value in this subsection is **TEST-LOCAL and experimental**: one release observation per process, one host, one model, one `mode_count`. Nothing here pins, enables or adopts a budget, a guard or a coefficient. The guard and measurement texts are proposals for Agent 0, ahead of an independent review. MiB is 2²⁰ B.

**The window** (`window-20261104T120707Z`, kept as raw evidence in the I3a O-4 scratch and not copied here):
- *Identity.* The binary has SHA-256 `f7c87223…cc6c`, built at `48b3cc692` (release, `debug_assertions=false`, aarch64-macos, rustc 1.97.1). Its identity and the supervision scripts' were recorded before the queue.
- *Outcome.* The launcher reported `COMPLETED`, cleanup `CONTAINED`, exit 0, and the harness `schedule complete`. All 10 of 10 processes are valid: each exited 0, with no signal and an empty stderr, and lock coverage was proven after each reap. There was no kill, overrun, refusal or lost record, and no memguard event (the log is unchanged since 2026-10-08). From T₀ to the window's end took 20.3 s. The normal path is confirmed.
- *Background, disclosed only.* During the window, fseventsd was reported at about 50 GB RSS, with 78% of system memory free. This is an operator observation and is not in the retained records. No causal claim is made about any value, and it did not stop the window.

**What each timed phase executes** (from source; `replay_modal_phases` follows `run_modal_dense`'s order):
- *parse:* the request's `ModalPreviewRequest` parse, on the measurand's side, before the replay. It is not in the phases total.
- *prepare:* the parameter refusals, `prepare_modal` (the static entry's pre-formation sequence) and `solve_modal_state_case` (this model has no state case).
- *mass_and_identity:* `assemble_mass` (consistent straight-member mass, as node blocks) and `check_weight_basis`. The identity check compares, on each of 3 axes, every member's nodal load and the model resultant.
- *pencil_and_screen:* `form_modal_pencil`. In order: the 96·n_g² guard, the sparse `form_basis_stiffness` and the static partition. Then the static dense structural solve on a zero force: the dense K view, the two symmetry views, the scaled prepared matrix, the contribution audit in exact expansions, a dense Cholesky of order n_f with its pivot screen, the gate and the residual. Then K_ff and M_ff, dense and range-checked, p_k and the free-DOF mass.
- *dense_solve:* `solve_dense_pencil`, in order:
  - the ordered Cholesky R of K, and C = R⁻¹MR⁻ᵀ;
  - Householder reduction and implicit QL with vectors (1,279 iterations at 960), then the sort;
  - the Hager inverse-norm estimate;
  - φ = R⁻ᵀz for every kept row, and the band screen (no pair was screened here);
  - the equilibrated mass-block Cholesky (the census).

  The stated 9n³ lists the Cholesky, the formation of C, the reduction and QL. The back-transform (≈ n³ flops) and the census Cholesky (≈ n³/3) also run in this phase and are not in that formula.
- *clusters_and_reported_set:* the census, the δ_c clusters, the frequencies and `complete_reported_set`.
- *normalization:* `modal_basis`, then `normalize_modal_shape` for each reported mode. These are dense M_ff bilinear forms, O(n_r·n_f²).
- *reformation:* the element operands, the element-by-element re-formation (the p128 seam) and the formation difference, with one double-double accumulator per stored entry.
- *residuals:* check 2 for **every computed mode**, n_c = n_f evaluations here. Each sums K̂φ, M̂φ and M̂φ again (for φᵀM̂φ) exactly, element by element: at most 3·Σ_e d_e² exact products, where d_e is element e's free-DOF count. The cluster e_λ follows.
- *sturm_evidence:* the union-pattern `ShiftPencil`, the candidate shifts in the gap, the scaled shifted profile and `sparse_direct::factorize_ldlt`, then the inverse-norm solves.
- *orthogonality:* check 3 on M̂ for the n_r published shapes: n_r exact products M̂φ_j, then n_r(n_r + 1)/2 exact bilinear forms.
- *The entry pass:*
  - `run_modal_dense`, timed whole: the same functions, plus the outcome's row assembly;
  - *binding:* the projection the product refuses at the dense guard, timed but not a product phase;
  - *projection:* `modal_doc_solved` builds the value tree, with a SHA-256 of each row's checked canonical text and of each table;
  - *canonical:* `serde_json::to_string`, then `canonical_json_checked_v1_text`, which makes a checked parse into a second tree and then runs the canonical writer.

**Times (seconds), rows, bytes and peaks:**

| n_f (n_g, elements) | pencil_and_screen | dense_solve | residuals | normalization | orthogonality | mass_and_identity | reformation | phases total | run_modal_dense | projection | canonical | rows | canonical bytes | peak RSS phases / entry (MiB) | process wall phases / entry |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 60 (66, 10) | 0.0011 | 0.0004 | 0.0214 | 0.0003 | 0.0048 | 0.0011 | 0.0005 | 0.031 | 0.026 | 0.0140 | 0.0044 | 1,110 | 468,577 | 10.5 / 17.4 | 0.04 / 0.05 |
| 120 (126, 20) | 0.0029 | 0.0028 | 0.0909 | 0.0007 | 0.0054 | 0.0016 | 0.0011 | 0.106 | 0.108 | 0.0206 | 0.0064 | 2,092 | 860,779 | 12.8 / 24.2 | 0.11 / 0.15 |
| 240 (246, 40) | 0.0073 | 0.0182 | 0.3639 | 0.0019 | 0.0092 | 0.0027 | 0.0018 | 0.405 | 0.446 | 0.0302 | 0.0129 | 4,058 | 1,647,689 | 20.4 / 41.6 | 0.41 / 0.50 |
| 480 (486, 80) | 0.0387 | 0.1704 | 1.5082 | 0.0068 | 0.0187 | 0.0056 | 0.0037 | 1.753 | 1.699 | 0.0589 | 0.0263 | 7,982 | 3,219,890 | 48.0 / 88.9 | 1.76 / 1.80 |
| 960 (966, 160) | 0.2376 | 2.0004 | 5.4650 | 0.0527 | 0.0360 | 0.0107 | 0.0078 | 7.811 | 7.247 | 0.1086 | 0.0458 | 15,848 | 6,383,582 | 150.4 / 220.1 | 7.82 / 7.43 |

Parse, prepare, the clusters and reported set, and the Sturm evidence were each at most 0.001 s at every point. Log–log slopes between adjacent points: dense_solve 2.87, 2.72, 3.23, 3.55; residuals 2.09, 2.00, 2.05, 1.86; pencil_and_screen 1.33, 1.34, 2.41, 2.62; phases total 1.79, 1.94, 2.11, 2.16; run_modal_dense 2.03, 2.05, 1.93, 2.09; projection 0.56, 0.55, 0.97, 0.88; rows 0.91, 0.96, 0.98, 0.99; canonical bytes 0.88, 0.94, 0.97, 0.99.

**Counts** (phases pass). At every point, computed = n_f, reported = 10, clusters = 2n_f/3, the census is `Established` with finite count n_f and no structurally massless DOF, nothing was band-screened, and there was one Sturm candidate, factored once. By n_f = 60, 120, 240, 480 and 960:
- QL iterations: 79, 152, 316, 628 and 1,279;
- inverse-norm solves: 5, 5, 9, 11 and 11;
- shift-pencil pattern entries: 164, 336, 682, 1,366 and 2,752;
- stored K_ff entries: 534, 1,104, 2,244, 4,524 and 9,084, and stored M_ff entries as for the pattern;
- residual evaluations: n_f.

**Rows per table** (entry pass, for n_f = 60 / 120 / 240 / 480 / 960):
- nodes: N = 11 / 21 / 41 / 81 / 161;
- dofs: 6N;
- mass_matrix: P_M = 182 / 354 / 700 / 1,384 / 2,770;
- mass_totals: 3;
- modes: 10;
- shapes: 6N·10;
- participation: 30;
- clusters: 2n_f/3;
- computed_modes: n_f;
- sturm_candidates: 1;
- identity: 3·(E + 1), E the element count;
- discretization: E;
- disclosures: 4;
- supports and diagnostics: 0.

**Reading limits:**
- The phases pass and the entry pass are **separate executions**: separate processes with separate peaks, and the entry's `run_modal_dense` re-runs every phase. Their times and peaks are not additive.
- The slopes are over n_f = 60–960, one process per measurand and one observation per point. They describe this population on this host only, and **they are not complexity bounds.**
- The source's drivers, for contrast:
  - dense solve: O(n_f³) flops;
  - residuals: O(n_c·Σ_e d_e²), which is O(n_f²) here;
  - screen: O(n_g²) memory and the static Cholesky's n_f³/3 flops;
  - normalization: O(n_r·n_f²);
  - orthogonality: O(n_r·Σ_e d_e² + n_r²·n_f);
  - projection: O(rows).
- At n_f ≤ 960 the residuals are the largest phase (70% of the phases total at 960). Continuing the last slopes (3.55 against 1.86), or the cubic and quadratic exponents, the dense solve would overtake them somewhere between n_f ≈ 1,700 and 2,600. That is an extrapolation.

**RSS reconciliation from source** (n_g = n_f + 6; the formulas evaluated at 960 / 480, in MiB):

| Buffer (all binary64 unless noted) | Formula | 960 | 480 | Allocated | Dropped |
|---|---|---|---|---|---|
| Static screen: dense K view | 8·n_g² | 7.12 | 1.80 | `SparseStiffness::to_dense` in `SparseAssemblyEvidence::solve_assembled` (`DenseScrutiny`) | when `solve_assembled` returns, the end of `form_modal_pencil`'s screen block |
| Static screen: symmetry views (roundoff, and counts as `usize`) | 16·n_g² | 14.24 | 3.60 | `dense_symmetry_view` | as above |
| Static screen: prepared matrix | 8·n_f² | 7.03 | 1.76 | `prepare_bound` | when `solve_prepared` returns |
| Static screen: contribution sums (`Expansion` = `Vec<f64>` + `usize`, 32 B) | 32·n_g² | 28.48 | 7.21 | `contribution_sums` in `audit_contributions` | when `audit_contributions` returns |
| Static screen: contribution differences (`sums.to_vec()`) | 32·n_g² | 28.48 | 7.21 | `contribution_differences` | as above |
| Static screen: Cholesky factor | 8·n_f² | 7.03 | 1.76 | `cholesky`, after the audit | when `solve_prepared` returns |
| Static screen: intended-action sums (a second `contribution_sums`) | 32·n_g² | 28.48 | 7.21 | `audit_intended_action` (frame_kernel `structural.rs:1145`), called from `finish_checked_factor` (`:1941`) once the residual passes | when `audit_intended_action` returns |
| Static screen: load-fidelity sums (a third `contribution_sums`) | 32·n_g² | 28.48 | 7.21 | `audit_load_fidelity` (`:1257`), reached because the assembled force binding's `audit_terms()` is `Some` (`:298`) | when `audit_load_fidelity` returns |
| K_ff and M_ff (flat) | 16·n_f² | 14.06 | 3.52 | `form_modal_pencil`, after the screen | end of the run: the test's end in phases, the return of `run_modal_dense` in entry |
| R (K's Cholesky, massless first) | 8·n_f² | 7.03 | 1.76 | `cholesky_ordered` | `drop(r)`, after the back-transform |
| Work: M R⁻ᵀ, then C, Q/Z and φ | 8·n_f·n_m = 8·n_f² | 7.03 | 1.76 | `solve_dense_pencil` | kept as `solution.vectors` until the end of the run |
| Census Cholesky of the equilibrated M | 8·n_f² | 7.03 | 1.76 | `established_finite_mode_count`, after `drop(r)` | when it returns |

- **Small terms.** Everything else is O(n) or O(stored entries): the sparse K and its assembly evidence, the stored entries, the `ShiftPencil` (32 B per pattern position, 2,752 positions), the profile and its clone, the per-evaluation `ExactSum` rows of the residual (n × (48 B + limbs)), the re-formed element operands, the formation accumulators (16 B per stored entry) and the n_r·n_f normalized shapes. Together these are of the order of 1 MiB at 960 (an estimate). The Sturm pencil's 32·n_f² of the `sturm` module docs is its full-profile case. This banded model's profile is O(n).
- **Route re-checked for further n²-scale allocations** (after the review of `cfcd55f24` found the two later audits):
  - The formation check does not run: `prepare_assembled_structural` builds no formation source, so `gate_formation()` is `None`.
  - The condition estimate, the original-equation residual bound and the bounded refinement are O(n).
  - `ModalMass::to_symmetric` (dense M) has no caller on this route.
  - The re-formation, normalization, Sturm, orthogonality and cluster code forms no n_f² buffer at n_r = 10.
  - No further term was found.
- **Live sets, from the lifetimes:**
  - at the contribution audit: 88·n_g² + 8·n_f² = **85.3 / 21.6**. That is the static 96·n_g² to 0.1 MiB: the formula counts the prepared matrix at n_g², not n_f²;
  - at the static factor: 24·n_g² + 16·n_f² = 35.4 / 9.0;
  - during each of the two later audits: 56·n_g² + 16·n_f² = 63.9 / 16.1. Each array is dropped before the next is formed, so they are sequential, not simultaneous;
  - in the eigen core: 32·n_f² = 28.1 / 7.0, both with K, M, R and the work buffer, and later with K, M, the work buffer and the census factor;
  - after the solve, to the end: 24·n_f² = 21.1 / 5.3.
- **Cumulative.** Counting every listed buffer once, with no reuse, the static screen is 152·n_g² + 16·n_f² and the modal path 40·n_f². That gives **152·n_g² + 56·n_f² = 184.5 / 46.5 MiB**, before the baseline and the small terms.
- **Baseline.** The process baseline is B₀ ≈ 9.7 MiB: the 60-point phases peak (10.5) less that point's cumulative (0.8). The entry pass's smallest peak, at 60, is 17.4 MiB, and it already includes that point's projection.
- **Phases pass.** The source brackets the peak: the largest live set plus B₀ is 95.0 / 31.3 MiB, and the no-reuse cumulative plus B₀ is 194.2 / 56.2 MiB. The observed peaks, **150.4 / 48.0**, lie inside that bracket: above the live set by 55.4 / 16.7, and below the cumulative by 43.8 / 8.2. At 120 and 240 the brackets are 11.1–12.8 and 15.2–21.5, against observed 12.8 and 20.4. **No excess over the corrected ledger remains to be explained.** The peak is not the live set of any one instant.
- **Page faults.** In all ten processes, ru_minflt × 16 KiB exceeds ru_maxrss by 4.3–4.4 MiB, assuming the 16 KiB VM page of arm64 macOS.
  - The listed buffers are written or read through in full (source). So if minor faults count every first touch of an anonymous page, about 40 MiB of the 960 point's listed allocations were served from pages already resident: allocated bytes exceed faulted bytes.
  - **Plausible hypothesis, not a finding:** freed buffers stayed resident (allocator retention) and later buffers reused part of them in place, so that ru_maxrss is a high-water above the live set and below the no-reuse sum.
  - Cumulative faults and the maximum RSS do not identify allocation lifetimes. Other patterns of resident-page reuse give similar aggregates, with the high-water at another instant or with different buffers sharing pages.
  - What would distinguish them is the per-phase trace below.
- **What the source cannot settle.** Whether the allocator returns or reuses a freed block depends on its size classes, not on this repository's source. At 960 the screen's rows are 7.7 KB and 30.9 KB, and each flat n_f² buffer is 7.03 MiB; at n_f = 1,920 each flat buffer is 28.1 MiB. Nor can the source say where in the bracket a peak falls, or at which instant it occurs.
- **What would settle it.** A test-only allocation trace per phase (live high-water, cumulative bytes and the largest block), with the resident size read at each phase boundary. This is a deterministic count, not a timing. **It is not proposed now:** the guard decision below rests on measured process peaks. It is needed only if a memory formula is to be extrapolated beyond measured points.
- **Entry pass.** `run_modal_dense` allocates the same list, and its outcome is small at n_r = 10 (shapes n_r·n_g·8 B and the computed evidence). The binding is the product's projection of the same request refused at the guard, with no n² buffer. Then the projection runs. Every term in it is an estimate, from serde_json's `BTreeMap` map with this toolchain's node layout (a leaf of 11 entries is about 632 B) and assumed 16 B malloc quanta, not from this repository's source:
  - the value tree T: about 1.7 KB per shape or mass row, 6.5 KB per computed-mode row and 0.8 KB per value object, about 30.7 / 15.5 MiB in all;
  - the compact text: 6.09 / 3.07 MiB;
  - the checked-parse tree: ≈ T;
  - the canonical text: 6.09 / 3.07 MiB.

  All four are live at the canonical write: about **73.5 / 37.2 MiB**, or up to 85.7 / 43.4 MiB with doubled text capacities. With B₀ that is 83.2–95.4 / 46.9–53.1 MiB, before the outcome and the request, which are small here.
  - **The governing live phase differs by size.** At 960 the screen's audit (95.0 with B₀) and the projection's canonical write (83.2–95.4) are comparable. At 480 the projection governs (46.9–53.1, against 31.3 for the screen).
  - The source bracket for the entry peak runs from that largest live set, about 95 / 47–53 MiB, to the no-reuse run plus B₀ plus the projection's upper estimate, about 280 / 100 MiB. The observed peaks, **220.1 / 88.9**, lie inside it.
  - The two passes remain separate executions. Nothing is added across them.
- **What the stated formulas cover.**
  - 96·n_g² (`dense_scrutiny_guard`) covers the static screen's six buffers at its audit, and so its live peak.
    - At n_r = 10 it exceeds the eigen-core, after-solve and later-audit live sets.
    - It does **not** cover the entry's projection instant: at 960 the estimate (73.5–85.7 MiB) is comparable to the formula's 85.4 MiB, and at 480 it exceeds the formula's 21.6 MiB (37.2–43.4).
  - It omits:
    - the process baseline;
    - retained or reused memory (the no-reuse cumulative 152·n_g² + 56·n_f² is the source's upper term);
    - the projection's trees and texts (at 480 the estimated 37.2 MiB exceeds the formula's 21.6);
    - the orthogonality products, n_r·n_f·(24 + 16k) B for k exact terms per entry, which reach n² scale on a whole-spectrum request;
    - the outcome's and normalization's n_r·n shapes;
    - a full-profile Sturm pencil, up to about 32·n_f²;
    - the residual phase's per-evaluation O(n) buffers;
    - outside this route, the state case's static load-case solve in `dense_scrutiny` mode (`state.rs`), which runs before the modal screen and is not reconciled here.
  - 32·n_f² (`dense_path_peak_bytes`) covers K, M, R and the work buffer only, and omits everything else.
  - Neither is a process-peak bound, and neither is fitted here.

**Projection size: a proposed bound from format and input lengths.** B_doc below is a **proposed bound**, not yet a bound. Its open items are named at the end of this part, and closing them is a prerequisite to adopting it.
- **Value encoding** (`openpipestress_jcs_ijson_v1`, `canonical_json`):
  - a binary64 below 2⁵³ in magnitude is ECMAScript `Number::toString` of ryu's shortest digits, at most **25 B** (−0.0000012345678901234567). A value at or above 2⁵³, or not finite, is `null`, with its reason;
  - an integer is its decimal digits;
  - a boolean is at most 5 B;
  - a checksum is 66 B with its quotes;
  - a string is serde_json's escape: at most 6× its UTF-8 bytes (control characters as `\u00xx`), plus 2;
  - keys are fixed by source, and the sort order does not change length;
  - a value object (`below_normal`, `kind`, `undefined_reason`, `unit`, `unit_status`, `value`) carries either a number with a null reason (25 + 4) or a null with its longest reason (4 + up to 28 for `no_neighbour_or_not_formed`).
- **Identifier dependence.** Model identities are any string with at least one non-whitespace character, Unicode admitted and with no length cap (`schemas/model.schema.yaml`). Let E(x) be x's escaped length, with no quotes. A row adds E for each occurrence of an input identity.
- **Bound per row**, S_t(D) + Σ E(identity). D is the digit width of the largest integer or reference index in the document (D = 8 covers documents below 10⁸ rows). The values below are at D = 8; each S_t(D) is computed exactly from D:

  | Table | S_t(8), bytes | Identity occurrences |
  |---|---|---|
  | shapes | 379 | 2·E(node) |
  | mass_matrix | 407 | 2·E(node_i) + 2·E(node_j) |
  | dofs | 175 | 2·E(node) |
  | nodes | 596 | 2·E(node) |
  | mass_totals | 525 | none |
  | modes (one Sensitive reason) | 1,360 | E(node) |
  | participation | 958 | none |
  | clusters | 423 | none |
  | computed_modes | 1,145 | none |
  | sturm_candidates | 2,394 | none |
  | identity | 383 | E(subject) |
  | discretization | 666 | 2·E(member) |

  - *Rows with string payloads* are each a fixed skeleton plus E of each string:
    - disclosures: E(text), on a 163 B skeleton at D = 8;
    - diagnostics: id, code, severity, message and affected refs;
    - supports: E(behavior), E(state), 2·E(support), E(node).
  - *Each table* adds 121 + d B of wrapper (d the digits of its row count) and one comma between rows. The `tables` object's 15 keys add 204 B.
  - **The header, from source** (`document.rs`: `modal_doc_identity`, `modal_doc_binding`, `modal_doc_solved`). Transcribed with every variant at its widest at D = 8, the fixed part is **H₀ = 8,450 B**. The variants taken are:
    - a state case present in both `binding.state_case` and `basis.state_case`;
    - a mass centre, a formation section, a frequency cut-off and an acceleration;
    - the above-spectrum placement, the below-cluster endpoint, the unresolved census and an orthogonality pair;
    - each u128 guard string at 41 B and each usize parameter at 20 digits.

    The variable-length fields, each added to H₀, are:
    - E(`admission.parameter_label`);
    - the canonical length of `binding.options` less 2: the request's option members, verbatim;
    - E(state-case id) **twice**: `binding.state_case.case_id` (`:393`) and `basis.state_case.case_id` (`:482`);
    - Σ (E(node) + 10) over `basis.state_case.restrained_dof_refs`: one quoted DOF id, with its comma, for each restrained DOF of a linearized state;
    - E(node) for each of `formation.stiffness_row_ref` (`:574`) and `formation.mass_row_ref` (`:577`). Each is null or a DOF id, and the id's skeleton is in H₀;
    - 26 B for each element of `sturm_gap_log_positions` and `sturm_whole_spectrum_offsets`;
    - `standing.reasons`: at most n_r mode reasons of ≤ 110 B each, plus one orthogonality reason of ≤ 128 B, each with a comma (D = 8, `{:e}` at ≤ 24 B).

    `binding.undeclared_members` is empty on a solved document: the request is `deny_unknown_fields`, and `unknown_field_refusal` refuses an undeclared member before any solve. On all 10 solved fixtures, the actual header lies within H₀ plus its variable terms, with margins of 1,456–1,832 B. The fixtures include the state-case, Sensitive, cut-off, body and arc variants.
- **Checked against the committed fixtures** (`fixtures/results/modal_*.json`, 1,691 rows over 12 tables): every row lies within its bound. The smallest margin is 0 B, on a DOF row.
- **Row counts for a general request:**
  - shapes: 6N·n_r;
  - modes: n_r;
  - participation: 3·n_r;
  - computed_modes: n_c ≤ n_f, every retained finite mode on the dense path;
  - clusters: n_cl ≤ n_c (every computed cluster, not only completed ones);
  - mass_matrix: P_M ≤ 21·N + 36·(distinct node pairs joined by a mass-bearing member);
  - nodes: N;
  - dofs: 6N;
  - mass_totals: 3;
  - sturm_candidates: ≤ s_max;
  - identity: 3·(n_straight + n_arc + n_body + 1);
  - discretization: n_straight;
  - supports: the nonlinear supports;
  - disclosures: ≤ 12 fixed and conditional, plus the state case's, plus the identity's (one per applied self-weight primitive or case), plus one per computed mode whose e_λ interval reaches across a cut-off (≤ n_c);
  - diagnostics: those of the preparation, the mass and the state solve.

  n_r by request:
  - `mode_count` m: n_r = min(m, n_c) plus the cluster extension, with n_r ≤ n_c;
  - a cut-off: the modes below it plus the extension, which can reach n_c;
  - the whole finite spectrum: n_r = n_c ≤ n_f. The endpoint needs an established census, whose finite count is n_f less the structurally massless DOFs. At the static guard's limit (N = 1,365, n_f up to 8,184) that is up to 6.7·10⁷ shape rows and about 2.6·10¹⁰ B under the proposed bound.
- **Observed against the proposed bound.** With the observed counts and this model's identities ("n0"–"n160" and "p0"–"p159", 2–4 B), the formulas give exactly the observed rows at all five points. The proposed bound, with the corrected header and table keys, is 499,985 / 917,126 / 1,746,284 / 3,401,269 / 6,752,196 B. The observed bytes are 468,577 / 860,779 / 1,647,689 / 3,219,890 / 6,383,582: 93.7–94.7% of it. This is a consistency check and no coefficient is fitted. Ten reported modes of one cantilever with short identities exercise no whole-spectrum, cut-off, arc, component, state, support, diagnostic or long-identity row, and so price none.
- **Proposed projection guard** (refusing, never truncating; coefficients reserved):
  - *(i)* optionally, refuse before the solve by the request-level count 6N·min(m, n_f) shape rows for `mode_count` m. This is a stated formula. It can refuse a request whose solved set would have been smaller (rejected μ, massless DOFs), and a cut-off request has no such count;
  - *(ii)* immediately after `complete_reported_set` and **before the normalization**, refuse unless rows R ≤ R_max and B_doc ≤ B_max. R is the exact count above. B_doc is H + Σ_t Σ_rows (S_t(D) + 1 + ΣE) + Σ_t (121 + d_t) + 204.
  - **Position kept, with template bounds** for what is formed later. Some strings exist only after the residuals, the re-formation or the Sturm check: the cut-off disclosures (`run_modal_dense`, after the residuals), the Sensitive reasons, the Sturm candidates and the formation row refs. At the check they are bounded from their format templates and input lengths:
    - *cut-off disclosures:* "computed mode {} ({}) has a first-order eigenvalue error estimate e_λ = {:e} that reaches across the cut-off", with ({}) at most "not reported" and `{:e}` at most 24 B. That is ≤ 308 B a row at D = 8, for at most n_c rows when the request names a cut-off, and none otherwise;
    - *Sensitive reasons:* every reported mode's row is bounded with one reason, and the header with n_r + 1;
    - *Sturm candidates:* s_max rows at S_sturm;
    - *formation row refs:* E of the longest node identity;
    - *orthogonality pair:* present.

    Everything else exists at the check, and the guard takes its exact length:
    - the other members of `modal_disclosures`, whose inputs (`prepared`, `mass`, `pencil`, `completed`, `census`) are formed by then;
    - the state-case and identity disclosures;
    - the diagnostics, supports, identities and request options.
  - Placing (ii) there also refuses before the O(n_r·n_f²) normalization and the O(n_r²·n_f) orthogonality of a large set.
  - B_max follows #59's custody limit (64 MiB displayable). R_max, and any in-memory multiple, are I3b's and Agent 0's to set. The in-memory projection was estimated above at about 12× canonical at its peak (two trees and two texts), which is an estimate, not a bound.
  - **Open items, before B_doc may be called a bound:**
    1. An independent check of the hand transcription of `document.rs`: the row skeletons, H₀ and its variable fields. The fixture checks cover 1,691 rows and 10 headers, not every producer path. A maintained test computing the bound from an outcome and asserting it on the fixtures and the producer-edge corpus would close this (a code change, outside this write boundary).
    2. The std formatting widths relied on: `{:e}` of a binary64 at ≤ 24 B, and the `{:?}` forms of `Option<(usize, usize)>` and `FiniteCensus`.
    3. That `binding.options` carries each option member exactly as the request's canonical text has it.
    4. Refused documents, which are not bounded here (their size is their diagnostics').

**Work-guard proposal.**
- **Observed scope only:** **n_f ≤ 960 AND a completed reported set of n_r ≤ 10**, refusing otherwise. The n_r check runs right after `complete_reported_set`, before normalization.
  - *The family it applies within* is explicitly restricted to:
    - a single straight-member, SEC-A-like model: one anchored straight run with consistent mass;
    - no arcs, no component bodies, and no state-case solve (so no nonlinear support);
    - the observed screening: the zero-force static dense screen on a banded K, with no band recovery and an established census with no massless DOF;
    - the observed Sturm profile: a banded RCM profile, one candidate, one factored shift;
    - `mode_count` requests only, with no cut-off;
    - a release build on aarch64 macOS.

    **Everything else stays outside the evidence**, and the guard says nothing about it.
  - *Evidence:* at n_f = 960, the phases total was 7.81 s, `run_modal_dense` 7.25 s, the process walls 7.82 / 7.43 s, and the peaks 150.4 / 220.1 MiB.
  - *Equivalent stated form within the family:* 9n_f³ + s·n_f³/3 ≤ 8.26·10⁹ at s = 1, and 3·n_c·Σ_e d_e² ≤ 6.6·10⁷ exact products.
  - *Not proposed as the guard:* the formula form. It would admit cost mixes the window did not measure: other element types, dense Sturm profiles, s > 1 factored shifts and larger n_r.
  - *Limits:* one observation per point, one host under a disclosed background load, one model family and one n_r, and no debug ratio.
- **The intended dense product scope.**
  - D-k's target is about 3,000 DOF (§3.2). The static guard admits n_g ≤ 8,192 at 6 GiB.
  - Continuing the last slopes to n_f = 8,184 is an **extrapolation, not a bound**:

    | Phase | Basis | Time |
    |---|---|---|
    | dense solve | 2.00 s × r³ to r^3.55, r = 8.53 | 1,240–4,030 s |
    | residuals | slopes 1.86–2.05 | 290–440 s |
    | screen | slopes 2.62–3 | 65–150 s |
    | normalization | | about 33 s |
    | **total per pass** | | **about 27–78 min** |

  - The source's live sets at 10 reported modes would be:
    - screen: 88·n_g² + 8·n_f² ≈ 6.0 GiB, at the 6 GiB ceiling by the guard's construction;
    - eigen core: 2.0 GiB.

    The no-reuse cumulative 152·n_g² + 56·n_f² would be ≈ 13.0 GiB, plus the projection. Both exceed what stage 1 can price. Where a peak of 511 MiB buffers would fall within the 6.0–13.0 GiB bracket is not known.
  - Whole-spectrum and cut-off requests add, as an n_r axis that no stage measured:
    - O(n_r·n_f²) normalization;
    - O(n_r²·n_f) exact orthogonality;
    - n_r·n_f-scale orthogonality products.
- **What a larger cantilever can and cannot close.** A larger point of the same family extends only **this family's** measured extent. It cannot close arcs, component bodies, state cases, dense Sturm profiles, other screening, cut-off or whole-spectrum requests, or larger n_r. Those stay outside the evidence, whatever n_f is measured, and closing them would need measurements of their own. None is proposed here.
- **The only additional measurement proposed** (one window, extending the observed family's extent; nothing else is proposed). **n_f = 1,920** (320 elements, n_g = 1,926), `MODAL_MEASURAND_MODES=10`, passes `phases` then `entry`, one process each.
  - *Binary:* the same binary. It is reusable if the head's measurand and product source are unchanged; otherwise rebuild and re-identify outside the window.
  - *Supervision:* the same scripts and rules.
  - *Deadlines:* d = 120 s per process, with g = 5 s.
    - Stage 1's growth rule holds: 8 × 7.82 s = 62.6 s ≤ 120 s.
    - The worst case is 30 s for the locks, plus 2 × 125 s, plus 60 s of reserve: 340 s, within the 570 s abort and the 600 s window.
  - *Expected (extrapolation):*
    - time: phases total ≈ 38–49 s (the dense solve 16–23 s, the residuals 20–23 s), and `run_modal_dense` about the same;
    - output: about 31,600 rows and about 12.8 MB canonical, under a proposed bound of 13.5 MB;
    - peak RSS, as a source bracket (estimates, not bounds): phases from 349 MiB (the largest live set plus B₀; 96·n_g² is 339.6 MiB) to 744 MiB (the no-reuse cumulative 152·n_g² + 56·n_f² = 734.6, plus B₀). Entry runs from about 349 MiB to about 915 MiB, the upper end adding the projection's 146–171 MiB.
  - *What it decides, for this family only:*
    - whether the family's measured extent, and with it the work guard, moves to n_f ≤ 1,920;
    - where the process peak falls within the source bracket once each flat buffer is 28 MiB, which bears on whether the dense modal memory constant must count retained or reused memory;
    - whether D-k's ~3,000 target is affordable for this family.
- **Conditional, only if Agent 0 keeps the ~3,000-DOF target after reviewing 1,920:** n_f = 3,072 (512 elements, n_g = 3,078), the `entry` pass only (the whole product path and its peak), in its own window.
  - **Admitted only under stage 1's 8× growth rule:** the slower 1,920 process must take **≤ 45 s**, so that 8 × 45 s = 360 s fits the deadline.
    - The 1,920 extrapolation runs to 49 s, so the rule may refuse the point.
    - If it does, the point is not taken. It needs Agent 0 to amend the rule explicitly, for example to (n_next/n)^3.6 × the slower wall, which is about 5.4× for 1,920 → 3,072 and follows the steepest observed dense-solve slope. That amendment is proposed for Agent 0's decision and is not assumed.
  - *Deadline:* d = 360 s with g = 5 s (worst case 455 s), not raised above 360 s.
  - *Expected:* 120–195 s, about 50,500 rows and 21.6 MB under the proposed bound. Peak RSS is bracketed at about 0.86–1.84 GiB for the run (the live set to the no-reuse cumulative, plus B₀), plus up to about 0.27 GiB of projection (estimates).
  - *Not proposed:* 3,840. It exceeds the target, and at its upper estimate one process would take a whole window.
  - *Also not proposed:* repeats, a debug ratio, a larger n_r, or the old 12-point-hour benchmark.

**Proposed D-k and guard decision for Agent 0** (nothing adopted):
1. **Work, now:** either
   - (A1) the observed-scope guard above, as TEST-LOCAL (n_f ≤ 960 and n_r ≤ 10, refusing, within the restricted family only); or
   - (A2) no work guard until the 1,920 point. The dense modal path has no product entry yet (§5.3.9), so nothing is exposed meanwhile.

   A stated-formula budget with cost coefficients (A3) is not supportable from stage 1.
2. **Scope:** either
   - (B1, recommended) narrow D-k's measured dense modal scope to the restricted family at n_f ≤ 1,920, extended by the single 1,920 point; or
   - (B2) keep the ~3,000 target for that family and add the conditional 3,072 point, under the 8× rule or an amendment Agent 0 makes; or
   - (B3) keep the static guard's whole range, n_g ≤ 8,192.

   B3 is not supported: by extrapolation it is 27–78 min per pass, with a 6.0–13.0 GiB source bracket, which only the hour-scale benchmark Agent 0 excluded could price. Under every option, the other model and request families stay outside the evidence.
3. **Memory:** keep `dense_scrutiny_guard` (96·n_g²) as the refusal before any allocation, recorded as a live-set formula rather than a process-peak bound. At 960 the observed peak was 1.6× the source's largest live set plus baseline, inside the source bracket that the no-reuse cumulative closes. `DENSE_MODAL_BYTES_PER_ENTRY` waits for the 1,920 peak. The source finding stands: the static screen, not the eigen core, sets the live peak.
4. **Projection:** adopt the guard's structure: rows and the proposed format bound, refusing, applied before the solve and after the reported set. The bound itself is adopted only after its open items close, and its limits are set with I3b against #59's custody limit.
5. **The n_r axis:** larger reported sets stay outside the measured scope. The projection limit bounds n_r·6N, and no n_r measurement is proposed.

##### Agent 0's decisions on the stage-1 analysis (2026-11-04, on `95808fec1`)

- **No production work guard from one benchmark family.** The observed n_f ≤ 960, n_r ≤ 10 family may serve as an explicit *experimental* measured profile and refusal boundary. It does not replace general modal admission, and it is not an accepted release limitation.
  - The dense-path goal stays about 3,000 DOF across the adopted families. The exact user-facing cap comes from a reviewed resource budget, without any obligation to support the whole static range (n_g ≤ 8,192).
  - Arcs, component bodies and state cases are not narrowed away for lack of measurement. They are priced from source cost components plus representative calibration.
  - Every required check is sourced and accounted: computed pairs, band recovery, cluster and Sturm variation, and the state case's static solve.
  - The state-case solve runs **before** the modal screen, so a combined preflight budget must refuse before that allocation and solve; a guard placed after it is too late.
  - One cantilever is not a worst-case claim.
- **Memory.** 96·n_g² is recorded as a live-buffer formula only, and no universal peak constant is fitted from any point. Bounded source allocations, output and reader peaks and their lifetimes are reconciled with calibration, every model and string cost included.
- **B_doc stays PROPOSED** until four things are independently checked: the transcription, the format widths, the canonicity of the options, and refused documents.
- **The projection-guard STRUCTURE is approved:** checked arithmetic, refusal before allocation, no truncation, and a model that stays editable. No numerical limits or coefficients yet.
  - "Before normalization" holds only if the bound covers normalization's expansion and the actual dimensions. Otherwise there is a conservative early bound plus an exact second check before projection.
  - A completed cluster is never assumed to stay within the requested mode count.
- **One calibration window at n_f = 1,920 was cleared.** The point at 3,072 is not approved. A fixed target of about 3,000 DOF does not trigger it automatically, and any further proposal uses the 1,920 data plus source.

##### The n_f = 1,920 calibration (2026-11-04; measured outcome only, TEST-LOCAL)

The window is `window-20261104T131510Z-nf1920`. Its identity record is `calib1920-prep.txt`: the same reviewed scripts, config `0344ce9f…9005`, and a binary rebuilt outside the window that is byte-identical (`f7c87223…cc6c`) at exactly `48b3cc692`.
- **Acquisition:** immediate at 13:15:10Z. The window ended at 13:17:02Z, rc 0, 112 s from T₀ including teardown. No overrun, kill or retry.
- **Closure:** verified by the launcher (group empty with zombies included, normal-path reaps, every child in G, no pending marker, lock holders exited) and independently at 13:17:21Z. memguard logged no event.
- **Both processes valid**, each with a post-reap identity proof.
  - **phases:** wall 56.24 s, peak RSS 542.1 MiB. dense_solve 39.47 s, residuals 14.01 s, pencil and screen 1.62 s, total 55.31 s.
  - **entry:** wall 55.70 s, peak RSS 589.1 MiB. run_modal_dense 55.37 s, projection 0.20 s, 30,581 rows, 11,905,720 canonical bytes.
- **The point leaves the stage-1 profile.**
  - 1,280 of 1,920 modes are computed. The band screen took 640 pairs and retained none. (The record does not say how many were hard-cut and how many were attempted and failed recovery, §5.3.7 "A3, as decided", D4.)
  - The census is **Unresolved** (`rejected_mu: 640`, mass block established) under the test-local τ_μ = 10⁻¹². (Corrected 2026-11-05: at this order the active hard cut is κ_μ·δ_q = 2.13·10⁻¹²·μ_max, not τ_μ, and the record does not split hard rejections from failed band recovery; see §5.3.7's A3 proposal.) At this mesh, SEC-A's spread puts genuine high modes below the test-local μ floor, which is §5.3.7's high-mode issue on a product-path model. The ten reported low modes are served, and the whole-spectrum endpoint stays refused.
  - This was A3 evidence and decided nothing by itself; A3's decision (§5.3.7, 2026-11-05) does not refit it.
- **Cost shape.**
  - The band recovery's exact insertions run inside dense_solve, which grew 19.7× from n_f = 960, a log–log slope of 4.3 against stage 1's roughly cubic trend. Their count is 4pn² for p attempted pairs. p is not recorded, so 4·640·1,920² ≈ 9.4·10⁹ is an upper (scenario) value, not measured attempt work (corrected 2026-11-05, D4).
  - Residuals cover 1,280 pairs, a slope of 1.36.
  - The growth ratio is 7.07 against the 8× rule.
  - Peak RSS is inside the source analysis's estimates (349–744 MiB for phases, up to about 915 MiB for entry). No peak constant is fitted.
- **Consequence for pricing:** band recovery and census variation are now measured cost components within one model family. The source cost model must carry them, and must not treat them as outside the profile.
- **Agent 0 on the outcome (2026-11-04).** The calibration is accepted as complete and contained. No further measurement is approved; the 3,072 condition is not met, and no larger point is proposed merely to fit a curve.
  - **What the source guard must cost, term by term:** the binary64 dense work; the band recovery's exact insertions (4·p·n²) and its 2n² visits; every check on computed and recovered pairs; the Sturm profiles; the state-solve preparation; and the projection. One fitted cubic coefficient is not a guard.
  - **When it must cost them:** p and the census and request variation are bounded **before** the costly stage, by a conservative upper bound on p or by a stage-specific budget check. A missing finite census never drops work from the account.
  - **What must not change:** τ_μ is not tightened artificially, and no high-mode check is skipped, to force admission or hide cost.
  - **Keep the populations separate:** the 35-model mathematical references stay separate from the cantilever cost calibration. The memory bracket from source plus the observed RSS is not a universal peak bound, and B_doc's open items still have to close.
  - **The ten low modes** are inspectable and reportable only under the accepted lower-gap count and completed-cluster premises. An unresolved whole finite census does not by itself invalidate the lower path, and it never becomes a whole-spectrum proof.
  - The current test-local status, with production disabled, is unchanged, and nothing is adopted from this observation.

##### The source resource guard: cost model, check placement and budget form (proposal, 2026-11-04; nothing adopted)

This proposal comes from source plus the existing cantilever calibration (stage 1 and n_f = 1,920). It keeps the two populations apart: the calibration prices cost only, and the 35-model mathematical references stay separate. Wherever it uses the 1,920 outcome, **the ten low modes are reportable only under the accepted lower-gap count and completed-cluster premises.** The unresolved finite census at 1,920 neither invalidates that lower path nor becomes a whole-spectrum proof.

**Cost model, term by term** (revised after the focused review of `d1d36002c`). Notation:
- n_f is the free order; n_m ≤ n_f the massive order; n_c ≤ n_m the computed (retained) pairs; n_r ≤ n_c the completed reported set.
- p is the **attempted** band pairs (μ in the band, not hard-rejected). The raw O4 line's `band_screened` counts hard rejects too, so it does not supply p.
- I is the total QL iterations, and R = Σ (m − l) the QL rotations, with R ≤ I·(n_m − 1).
- d_e is element e's free-DOF count; nnz(·) is a stored count over the full symmetric matrix.
- P_u is the shift pencil's union pattern; w_i are the profile's row widths after RCM, and P = Σ w_i; s ≤ s_max are the Sturm candidates. N_ns is the nonlinear supports, and d ≤ the friction supports is the derived sliding-friction candidates per iteration.
- MA is a multiply-add. A *visit* is one `ExactSum::add_product` call, which returns at once on a zero factor (`exact.rs:113`). An *insertion* is a visit with every factor nonzero, or one `add_dd_product`, which also builds a small `Vec` (`exact.rs:161`).
- **L is an accumulator's limb span.** `ExactSum` keeps 32-bit limbs from its lowest to its highest part (`exact.rs:51`–`:100`). It grows by `splice` when a lower part arrives, and `terms()` returns one term per nonzero limb (`exact.rs:217`).
  - So k (terms per entry) ≤ L, and limb moves per sum ≤ L × its adds.
  - **No data-independent bound on L is established** (narrow re-check, 2026-11-04). The earlier derivation of a 6,243-bit span, and so of L ≤ 202, is withdrawn, for three reasons:
    - three minimum subnormals enter `add_scaled` at scale −3,222 with a normalized part of 1.0, whose encoded lsb is −3,274 (`exact.rs:63`, `:153`), not −3,222;
    - `normalize()` can append limbs;
    - the generalized-mass and orthogonality terms multiply accumulated sums, and a factor count alone does not bound carry growth.

    No underlying monomial was found with more than three binary64 factors, so some bound near 202 may hold under explicit count bounds, but none is proved.
  - *(2026-11-06: a data-independent span L̄ = 492 limbs is now derived from source, pending independent scrutiny; see "P0 and S1 in the product, as built" below.)*
  - **So L is held by an enforced allotment, not a bound.** P0 charges every L-dependent term at a proposed allotment L_A; 202 is the working proposal, and the value is not adopted. S2b's exact scan, after re-formation and before any residual or orthogonality array is built, refuses (`MODAL_RESOURCE_CHECKS_EXCEEDED`) if the exact L exceeds L_A. The formation maps (T7) are built before S2b, so their per-entry limb growth is checked against L_A as the sums grow, before the next allocation.

**Counts are upper bounds** wherever data can only lower them: zero factors skip insertions, and an early failure skips the rest of a pair's work.

| # | Term | Count from source (upper bound unless stated) | Source | Bounded / checked at |
|---|---|---|---|---|
| T1 | State-case solve (with a state case only; runs **before** the screen) | Structural solves S_T1 ≤ (I_as + 1)·(2 + d) + 1. Here:<br>• I_as = 4 iterations;<br>• the +1 is the single all-active recovery trial that iteration 1 may add after a structurally failed first attempt, both counted as iteration 1 (`lib.rs:813`);<br>• 2 + d is, with derived friction, one base solve, d unit solves and the final solve (`:1574`–`:1680`), and without it one solve;<br>• the last +1 is the confirmation's gate factorization.<br>Each solve is priced as T2 on its free set, including `reduce_system_with_prescribed_displacements_binary64` (`:2219`). Per iteration with friction, add the d × d influence and coupling matrices and `solve_dense`. That is general Gaussian elimination with pivoting, not Cholesky: 2d³/3 flops leading, plus validation and lower-order work (frame_kernel `lib.rs:2003`, `:2029`) (`:1590`, `:1655`). The confirmation also runs E_conf K-D5 formation evaluations (`structural_adapter.rs:375`; `confirmation.rs:332`, `:696`, `:711`), each re-forming every element from primitives (`formation_check::check_at`) and solving with the gate factor, and reaction certifications (`confirmation.rs:961`, `:1030`). | product `lib.rs:195`; nonlinear_integration `lib.rs:759`, `:813`, `:1514`, `:2212`; `confirmation.rs:78` | P0: S_T1 and d³ from inputs (d ≤ friction supports). **E_conf is not bounded here from source:** an enforced allotment A_conf, a counter on `ConfirmationContext::evaluate_at` and `reaction`, refusing before the next evaluation |
| T2 | Static screen of the modal pencil | n_f³/3 flops (dense Cholesky), plus O(n_g²) passes: three 32·n_g² sum arrays, the differences, prepare, skew check, audits | frame_kernel `structural.rs:2060`, `:934`, `:1047`, `:1141`, `:1240`, `:1489` | P0 (n_g; n_f ≤ n_f,max) |
| T3 | Kernel, binary64 | (19/3)·n³ + 6n·R + O(n²) flops: Cholesky n³/3; reduction 2n_m³; Householder 4n_m³/3 plus accumulation 4n_m³/3; QL 6n_m·R; back-transform ≤ n³; census Cholesky m³/3; Hager ≤ 11 triangular pairs; sort ≤ n_m² compares and n_m swaps | modal_eigen `dense.rs:803`, `:880`, `:955`, `:1061`, `:865`, `:751`, `:1148`; `norm_estimate.rs:8` | R: enforced allotment A_QL (S1-QL counter) |
| T4 | Band recovery | Two separately priced parts:<br>• *scaling:* once if p > 0, `formed_scaling`, 2n² entries, each two WideDd divisions and one WideDd addition (`:555`), which is not an `add_product` visit;<br>• *per attempted pair:* ≤ 2n² visits in `recover_band_mode` (`:590`) and ≤ 2n² in `formed_pair_errors` (`:629`), of which ≤ 2·(nnz(K_ff) + nnz(M_ff)) are insertions. A zero shape component removes insertions, and a failed energy, mass or μ premise skips `formed_pair_errors`.<br>Hard-rejected pairs cost O(1). | `dense.rs:544`–`:629` | p ≤ n_f at P0, or allotment A_band. **S1-band** counts p exactly before `formed_scaling` and before any insertion |
| T5 | Census, clusters, reported set | O(n_c). n_r ≤ n_c ≤ n_f; cluster completion can take n_r beyond `mode_count` | `eigen.rs`; `clusters.rs` | S2 (exact) |
| T6 | Normalization | ≤ n_r·n_f² visits (`mass_bilinear` on dense M_ff), plus 3 exact dots of n_f per mode | `modal_quantities.rs:294` | S2 (exact n_r) |
| T7 | Re-formation and formation difference | Per element, one p128-seam re-formation (stiffness and mass). Assembly into a `BTreeMap<(r, c), ExactSum>` per matrix: Σ_e d_e² insertions, each with an O(log P) map step (`reformation/mod.rs:559`). Per formed stored entry, one `ExactSum` accumulator (`:745`). Per re-formed position, one `binary_search` and one `terms()` (≤ L terms) folded into the difference (`:754`). Two scaled norms: n WideDd row sums each (`scaled_norm`, `:706`). Every `ExactSum` has dynamically allocated limbs (≤ L). | `reformation/mod.rs:559`, `:700`–`:800` | P0 (element counts; L_A); S2 |
| T8 | Residuals, every computed pair, **including retained recovered pairs** | Per pair:<br>• 3·Σ_e d_e² insertions (`product_into` for K̂ and λM̂, and `mass_product_exact`; `:506`, `:553`);<br>• the generalized mass's `add_terms_product` over each of n_f rows' `terms()`, ≤ L terms each, so ≤ n_f·L insertions (`:937`);<br>• n_f `to_wide` reads of O(L) each;<br>• O(n_f) WideDd.<br>Limb moves ≤ L·(3Σ_e d_e² + 3n_f). | `reformation/mod.rs:506`, `:895`–`:940` | P0 at n_c ≤ n_f and L_A; **S2b** (exact L) |
| T9 | Sturm evidence | *Construction, once:* the merged-entry sort O(P_u log P_u), RCM O(n + P_u), adjacency (`sturm.rs:535`–`:690`).<br>*Per candidate (≤ s_max):* the formed list O(P_u); the profile O(P); `factorize_ldlt`, whose inner loops execute ≤ ½·Σ_i w_i² + Σ_i w_i MA (`sparse_direct/src/lib.rs:419`; n³/6 + O(n²) for a full profile); the factor observations O(P) a pass (`sturm.rs:935`); ≤ 11 inverse-norm solves of 2P each.<br>*Per neighbour (≤ 2):* the mass quotient's exact sums over P_u (`sturm.rs:700`). | as given | P0: s_max with the dense profile (P ≤ n(n + 1)/2). **S2:** an explicitly priced symbolic step (pattern sort, RCM and the profile's first columns, O(P_u log P_u + n + P)), moved ahead of `modal_sturm_count` and its result reused there, gives the exact P and w_i. Today the RCM runs inside `modal_sturm_count`, after the residuals. |
| T10 | Orthogonality, **published set** | n_r·Σ_e d_e² insertions (`mass_product_exact` per shape), **retaining every product** (`orthogonality.rs:121`). Then n_r(n_r + 1)/2 bilinear forms, each n_f `add_terms_product` calls of ≤ L terms (`:150`). | `orthogonality.rs:116`–`:190` | P0 at n_r,max and L_A; S2b (exact L) |
| T11 | Projection | O(rows): per row one serialization, one checked parse, one canonical text and one SHA-256; per document one serialization, one checked parse and one canonical text | `document.rs`; `canonical_json` | S2 (early B_doc), S3 (exact strings) |

**Where each check sits.** Each refuses by name and truncates nothing; the refused document states the formula and its terms.
- **P0, the combined preflight.** In `run_modal_dense`, after `prepare_modal` and **before** `solve_modal_state_case` and any n² allocation, from request, model and parameter inputs only. Each unknown enters at its conservative bound, or at an allotment that a later check enforces:
  - n_f ≤ n_f,max;
  - n_c ≤ n_f, and n_r ≤ n_r,max = min(n_f, ⌊R_max / 6N⌋), enforced at S2, never assumed ≤ `mode_count`;
  - p ≤ n_f, or A_band;
  - R ≤ A_QL;
  - s ≤ s_max, with the dense profile;
  - L = L_A (an enforced allotment; no data-independent bound is established);
  - S_T1 and d from the state case's supports, with A_conf.

  **A missing finite census never removes work.** The rejected pairs are in p and the computed pairs in n_c, whatever the census verdict. τ_μ is not tightened, and no high-mode check is skipped, to reach admission.
- **S1, inside the kernel** (**built: #109, merged `23f68bd7d`**, 2026-11-04). `modal_eigen` is outside the priced crates (`tools/retained_pricing/base/crate_dirs.txt`), and `price.py` at base and head compared equal on all 1,602 coefficients, so S1 needed no D3. The budget input is an additive `DenseAllotments` passed to `solve_dense_pencil_with_allotments`, not a field of `DenseOptions`, and it defaults to no allotment. Refusals are typed and map to the approved `MODAL_RESOURCE_*` names, and the exact rotation and band-pair counts are exposed. No allotment value is adopted. **Wired into the product path** with P0 (2026-11-06, below: "P0 and S1 in the product, as built"). The design as specified:
  - *S1-QL:* a cumulative rotation counter in `implicit_ql` against A_QL;
  - *S1-band:* p counted exactly from the classification thresholds, before `formed_scaling` and any insertion, against A_band.
- **S2, after `complete_reported_set` and before normalization.** Exact here: n_c, n_r, n_cl, the computed vectors and the symbolic Sturm profile (P, w_i). It checks T6, T7 (still at L_A), T9 and the early B_doc, and reserves T8 and T10 at L_A.
- **S2b, after `reform_modal_pencil` and before the residuals** (new). One scan reads the binary exponents of the stored K_ff and M_ff entries, the re-formed element entries (both dd words), every computed vector, and λ. The published shapes' range is each computed vector's, shifted by its normalization scale, whose exponent is the normalizing class's largest component's, ±1. From that scan S2b takes the exact span bound L_S2b ≤ L_A, and checks T8 and T10's insertions, limb moves and storage before either is constructed.
- **S3, in the product entry before `modal_doc_solved`.** The same B_doc with the outcome's exact strings and Sturm count. Refuse before the tree is allocated.

**Calibration status per weight** (cantilever family only; one definition per weight; from the raw phase times, not the rounded table; no rate is adopted). Each weight is time per **counted unit** under the stated count, not per executed operation.

| Weight | Counted unit | Existing records give (240 / 480 / 960 / 1,920) | Status |
|---|---|---|---|
| w_b64 | one T3 flop counted with R = I·(n − 1), i.e. (19/3)n³ + 6n·I·(n − 1) | 0.0925 / 0.1087 / 0.1579 / 0.3871 ns. The 1,920 value includes the band, so it bounds only the binary64 share from above | determined ≤ 960; at 1,920 only as that bound |
| w_ins | one T8 insertion, counted as 3·n_c·Σ_e d_e² (the generalized-mass terms and limb work at the observed, narrow spreads are folded in) | 89.4 / 91.8 / 82.7 / 79.4 ns | determined for straight elements at the observed spreads; **broad spreads not determined** |
| w_visit | one T6 visit (n_r·n_f²) | 3.375 / 2.950 / 5.721 / 2.431 ns | determined as a range |
| w_band | one T4 visit | **Not determined, and not bounded.** 4.18 ns is the whole 1,920 dense phase divided by 4·640·1,920², which assumes that all 640 screened pairs were attempted (the records do not say) and charges none of the phase to binary64. An upper count cannot give an upper rate. Scenario only: if all 640 were attempted, and visits cost w_visit at 1,920, the band would be about 27 s and binary64 about 12 s. That split rests on both assumptions and is not measured attempt work (D4, 2026-11-05); the K2 counters would supply p only in a separately cleared measurand. | not determined |
| w_scale | one `formed_scaling` entry (two WideDd divisions) | none separately | not determined |
| w_screen | one n_f³/3 flop, the n_g² passes folded in | 1.583 / 1.050 / 0.806 / 0.686 ns | determined for the no-state screen (conservative: 1.05 ns at ≥ 480) |
| w_reform | one straight element operand (T7, the formation difference included) | 45.6 / 46.0 / 49.0 / 45.8 µs | determined for straight members at the observed spreads; **arcs not** |
| w_sturm | per banded run | ≤ 1.5 ms | banded RCM only; **dense profiles not** |
| w_proj | one canonical byte (projection plus canonical) | 24.2 ns at 960, 24.0 ns at 1,920 | determined for these row mixes |
| w_limb | one limb move | none: the observed spreads are narrow | not determined |
| T1, K-D5 | one state-case solve set; one formation evaluation | none (no state case was run) | not determined |
| bodies | (no re-formation; mass entries only) | priced by source counts at w_ins | determined by source |

The weights rise with n (w_b64 from 0.0925 to 0.1579 over 240–960), so **no weight above n_f = 1,920 is determined.** Rates that would need a future measurement (none is proposed now), with the decision each one holds:
- the band/binary64 split, w_scale, and the kernel weights above 1,920: the work cap near 3,000 DOF in any family;
- T1, K-D5 and A_conf: admitting state-case families;
- dense Sturm profiles: s_max·T9 for branched networks;
- w_limb and broad-spread w_ins: models with wide magnitude ranges (P0 bounds their counts at L_A, but cannot price them);
- arc re-formation: a minor term.

**Proposed budget form.**
- *Units and comparison.* Counts c_t are u128, and weights w_t are integer picoseconds per counted unit.
  - W = Σ_t w_t·c_t, with `checked_mul` and `checked_add`. An overflow refuses as `MODAL_RESOURCE_ARITHMETIC_OVERFLOW`.
  - Stage allotments are A_s = Σ_{t∈s} w_t·c̄_t, with c̄ the bound P0 uses.
  - P0 requires Σ_s A_s ≤ W_budget. Each later check requires its exact Σ_{t∈s} w_t·c_t ≤ A_s.
  - **A term whose weight is not determined cannot be admitted by arithmetic.** Until its weight is reviewed, the budget either assigns it a reviewed conservative weight or refuses requests in which its count is nonzero.
- *Refusal names* (proposed, blocking):
  - `MODAL_RESOURCE_PREFLIGHT_EXCEEDED`: P0, for work or memory, naming the dominant term;
  - `MODAL_RESOURCE_QL_ALLOTMENT_EXCEEDED` and `MODAL_RESOURCE_BAND_ALLOTMENT_EXCEEDED`: S1;
  - `MODAL_RESOURCE_CONFIRMATION_ALLOTMENT_EXCEEDED`: T1's counter;
  - `MODAL_RESOURCE_CHECKS_EXCEEDED`: S2 and S2b;
  - `MODAL_PROJECTION_LIMIT_EXCEEDED`: S2 and S3;
  - `MODAL_RESOURCE_ARITHMETIC_OVERFLOW`.
- *Checked arithmetic, illustrative only.* The case is a SEC-A-like model at n_f = 2,994, n_g = 3,000, with no state case, at P0 bounds:
  - A_QL at R = 1.5·n·(n − 1);
  - p ≤ n_f;
  - R_max = 2·10⁵ rows, so n_r ≤ 66;
  - L = L_A = 202 (a proposed allotment, enforced at S2b);
  - s_max = 7;
  - w_band = 4.18 ns as a stated proxy, not a bound.

  W ≈ 1.0·10¹⁵ ps, within u64.

  | Term | Seconds |
  |---|---|
  | T2 screen | 9.4 |
  | T3 kernel | 159.3 |
  | T4 visits | 448.7 |
  | T4 insertions | 37.4 |
  | T4 scaling (proxy) | 0.1 |
  | T8, L-dependent terms at L_A included | 225.9 |
  | T6 normalization | 3.4 |
  | T10, at L_A | 123.5 |
  | T11 projection | 1.6 |
  | **total** | **≈ 1,009 (16.8 min)** |

  - The T8 limb moves (≤ 1.4·10¹¹), T7, and T9's dense-profile work are counted but **not priced**, because their weights are not determined.
  - S1-band, S2 and S2b replace the p and L_A terms with exact counts before that work starts.
  - **It is not a cap.**
- *What is reserved.* The ~3,000-DOF goal across adopted families, the user-facing cap derived from a reviewed resource budget (W_budget and the allotments), and every weight are Agent 0's to set from that review. No coefficient is adopted.
- *Memory: a cumulative ledger in requested bytes.* With ρ = 1 (no reuse), P0 compares B₀ + C with the memory budget, where C is the sum of every allocation of every stage. Each stage checks its own entries when their sizes become exact. Those checks are S1-band and S2; S2b is the first exact check of the L-dependent entries.
  - **Correction (narrow re-check, 2026-11-04).** The ledger below lists per-pair and per-product transients (T8, T10, and the `terms()`/`to_wide()` clones and per-product `Vec`s at `exact.rs:163`, `:177`, `:204`) once each, which is a reuse assumption. A true no-reuse C counts them for every pair.
    - T8 alone is n_c × 2n_f × (48 + 8L) + 16L·n_c. At the illustration's P0 bounds (n_c = n_f = 2,994, L = 202) that is **27.78 GiB**, before anything else.
    - The ρ = 1 cumulative form is therefore not usable as stated for the exact stages. The 3.8 GiB illustration below is the live-plus-one-transient sum, not a no-reuse cumulative bound.
  - **The options for Agent 0 (none adopted):**
    - (a) a stage-wise form, the largest live set plus the largest single transient per stage. This depends on the allocator returning freed transients, which is an allowance to review, not a source fact;
    - (b) the true no-reuse cumulative, which would refuse every large request on the exact stages;
    - (c) an implementation change that reuses one buffer per stage for those transients, making (a) a source fact.

    Allocator allowances cannot stand in for omitted source allocations.
  - **Agent 0 chose (c), 2026-11-04, and released its implementation:** a bounded reuse of workspace per stage, per pair and per product.
    - **Numerics unchanged:** the order of every numeric operation, insertion and reduction stays identical, with no change to matrix identity, normalization or rounding.
    - **Reset:** every accumulator and state is reset explicitly (ExactSum limbs, status and sign), not only the logical arrays.
    - **The ledger counts capacity, not length:** retained maximum capacity; simultaneous inputs, outputs and workspaces; old plus new allocations during growth; clones and alias copies; error and return paths; and stage overlap. It reserves once at a checked extent, or uses fallible allocation, where the route allows, and later stages' peaks never omit retained capacity.
    - **What it guarantees:** the reuse of requested allocations becomes a source fact. It is no total-RSS guarantee; the runtime, allocator rounding, layout, stack and custody allowances stay separate. No general allocator trace or custom allocator.
    - **Formation-map growth:** its guard comes before the allocation that would breach it, because S2b, which runs after formation, cannot repair an earlier overspend.
    - **Review:** independent numerical and resource review, with bit identity on the existing references and producer edges. That includes many modes reusing a workspace, empty, zero and cancelling inputs, and an early refusal followed by reuse and growth.
    - **Not adopted:** a ceiling, ρ or a value of L_A; 202 remains a proposed allotment, not a proven limb bound. The 3.8 GiB illustration is updated only after the real workspace ledger is verified in source, and a passing test is no proof of every model's heap peak.
    - **Not yet confirmed:** the corrections in `f89a36359` are not claimed clear until the focused fix confirmation is done in the implementation review.

  | Stage | Allocations, each counted once (bytes) | Exact at |
  |---|---|---|
  | T1 (state case) | S_T1 × the T2 ledger on its free set; per iteration with friction, 16·d² + 8·d·n_g (influence, coupling, unit forces), plus `solve_dense`'s matrix clone 8·d² and its cumulative pivot-tail allocations, ≤ 8·d² (frame_kernel `lib.rs:2000`, `:2025`); E_conf × the K-D5 evaluation's buffers (not derived here, so held under A_conf) | P0 (A_conf enforced) |
  | T2 | 152·n_g² + 16·n_f² + O(nnz) (rounding records) | P0 |
  | Pencil | K, M 16·n_f²; entries 24·(nnz_l(K) + nnz_l(M)); labels, maps and p_k O(n_g) | P0 |
  | T3 | R 8·n_f²; work 8·n_m·n_f; census factor 8·m²; O(n) vectors | P0 |
  | T4 | roots 8n; p screened records; one O(n_f·L) row sum at a time | S1-band |
  | T5 | ids, frequencies, clusters O(n_c) | S2 |
  | T6 | ≤ 16·n_r·n_f (shapes and below-normal lists), plus one `ExactSum` per call | S2 |
  | T7 | about 4.8 KB per element (`Mapped` rows and the 12 × 12 dd matrices for K and M); the two `BTreeMap`s, ≤ (80 + 8·L) per re-formed position; formed-entry accumulators, (48 + 8·L) per stored entry; the off-pattern map; 2 × n WideDd | S2 at L_A |
  | T8 | transient 2·n_f·(48 + 8·L) + 16·L per pair; retained n_c records | S2b |
  | T9 | `ShiftPencil` 32·P_u + adjacency 8·(n + 2P_u) + merged entries 24·(nnz_l + n) (transient); **per candidate, cumulative** 24·P_u + 16·P + 40·n (formed list, profile, its clone in `factorize_ldlt`, first columns and work, observation vectors) (`sturm.rs:50`) | S2 (symbolic P) |
  | T10 | **every product retained**: n_r·n_f·(24 + 16·L) + 24·n_r; transient rows n_f·(48 + 8·L) per shape | S2b |
  | Outcome | shapes 8·n_r·n_g; computed evidence O(n_c); mass entries 24·P_M | S2 |
  | T11 | two value trees and two texts | S3 |

  - *Illustration at n_f = 2,994* (P0, L_A = 202, dense Sturm profile): **the live set plus one transient per stage** ≈ **3.8 GiB**, under the provisional 6 GiB. It is not the no-reuse cumulative; see the correction above. The parts are:
    - T2: 1,441 MiB;
    - pencil: 137 MiB;
    - T3: 205 MiB;
    - T9: 487 MiB at the dense profile;
    - T7: 222 MiB;
    - T10: 614 MiB;
    - the projection estimate: 768 MiB;
    - the rest: under 30 MiB.
  - The L_A and dense-profile terms shrink at S2 and S2b.
  - **Still outside repository source, and held by a stated allowance:**
    - allocator size-class rounding and metadata (macOS libmalloc);
    - the value trees' per-node size (serde_json's `BTreeMap` layout at the pinned toolchain). S3 checks a per-row bound derived from that layout, which is an open item.
  - The observed phase fractions (0.67 / 0.56 / 0.49 at 480 / 960 / 1,920) are calibration evidence only, not a fitted constant. At 1,920 the entry's faults exceed its peak by 95 MiB, so in aggregate some memory was returned and faulted again. 96·n_g² stays a live formula only.
- *The projection guard under the approved structure.*
  - At S2, the early check uses B_doc with template bounds. Its "before normalization" placement holds: N, n_c, n_cl, n_r and P_M are the actual dimensions once cluster completion has run, and normalization changes values only. **The widths stay bounded: the 25-byte allowance covers normalization's change.**
  - S3 repeats the check with exact strings and the exact Sturm count, before the tree is allocated.
  - Rows and bytes are checked u128, and nothing is truncated.
  - B_doc stays **proposed** until its four items are checked independently:
    1. the transcription of `document.rs`;
    2. the format widths (`{:e}` ≤ 24 B and the `{:?}` forms);
    3. the canonicity of the options;
    4. refused documents.

##### Remaining production pin choices (2026-11-04)

| Choice | Status | Evidence that exists | Decision needed, from whom | Blocked by |
|---|---|---|---|---|
| τ_r (residual threshold, A2) | **adopted, 2⁻³⁰**, as an engineering measured-residual demotion criterion (Agent 0, 2026-11-04) | §5.3.7's targeted A2 evidence | focused text, source and regression confirmation | the production instance (with A3) |
| A3: τ_μ, c_μ, κ_μ, c_f | **decided (Agent 0, 2026-11-05): τ_μ = 2⁻⁵³, c_μ = 1, κ_μ = 10, c_f = 64**, as explicit retention and screening criteria; K2 approved (test-only counters) | §5.3.7's μ/μ_max table, its A3 proposal and "A3, as decided". At 1,920, 640 genuine top modes were not retained (hard cut κ_μ·δ_q = 2.13·10⁻¹²·μ_max, or failed band recovery; the record does not split them), so the census is unresolved, a recorded experimental capability limit (D5) | none for the values; the production instance's source, regression and D3 | K2's implementation; no parameter recaptures the 1,920 top modes (K3, K4 not adopted) |
| A1 (τ_t, the identity tolerance, g_max, τ_e, τ_m, s_max, gap positions, offsets, the QL limit) and A2's κ_c, κ_s, τ_o, δ_c | approved or adopted as criteria (Agent 0, 2026-11-04) | §5.3.7 | none, beyond the joint production instance | nothing among the pins (A3 decided 2026-11-05); τ_r (2⁻³⁰) and the A3 criteria take their focused source and regression confirmation with the instance |
| O-4 work budget: W_budget, the allotments, the user-facing cap | proposed form above; no coefficient | the stage-1 and 1,920 weights (table above) | the reviewed resource budget and the cap across adopted families; Agent 0 | the undetermined weights above (w_band, w_scale, w_limb, kernel above 1,920, T1/K-D5, dense Sturm, arcs), the S2/S2b reordering (S1 is built and wired into the product with P0, 2026-11-06), T1's confirmation counter (a held NI seam), priced changes with D3 |
| O-4 memory budget: ρ, B₀ and the ceiling | proposed form above | the ledger; the observed brackets at 480 / 960 / 1,920 | ρ (1 proposed), the allowance for allocator rounding and library layout, and the ceiling beyond the provisional 6 GiB; Agent 0 | the stated allowance (outside repository source); A_conf and the K-D5 buffers for state cases |
| Projection limits: R_max, B_max, any in-memory multiple | structure approved; no limits | #59's custody limit (64 MiB displayable); the projection estimates | the limits; Agent 0 with I3b | #59 merging; I3b |
| B_doc | proposed | 1,691 fixture rows and 10 headers within the bound | an independent check of its four items; then Agent 0's adoption | that check |
| The full reader's scope (§5.3.9 item 4) | model-derived mass assurance selected; implementation and review open | §5.3.4 and “Selected mass assurance: next modal unit after CE” | correction, scope review and verdict-path wiring; Agent 0 | S6, then CE integration for the mass unit; I3b for wiring |
| Recording policy, O-8 (§5.3.9 item 5) | open | none | task record, store and version; Agent 0 with DEL-00-06 | the decision |
| D3 (§5.3.9 item 6) | **registered:** DEL-04-08 §8.52 merged with I3a (PR #113, `08bfdfdaf`; coefficient-equal, AC-004 recorded in that PR) | §8.52 | none for I3a. The production instance's own D3 is §8.57 (coefficient-equal; AC-004 7/7 at `089a5698f`), registered when PR #131 merges; later priced changes (the O-4 guard stages) take their own D3: P0 and S1's is §8.58 (coefficient-equal; AC-004 7/7 at `ef009fa0b`), registered when PR #155 merges | — |
| Independent numerical review of I3a (§5.3.9 item 6) | open; a separate obligation from D3 | the reviews of the slices so far (PR #33 and the I3a rounds) | an independent numerical review of the I3a result family and readers; Agent 0 to commission | its scheduling |
| Subject census (§5.3.2) | **built:** `binding.subject_census`, the subject rule stated independently in both readers, and regenerated fixtures. The shared corpus forges a missing, extra and duplicated arc, a substituted straight member, arc, body and nonlinear support, a duplicated nonlinear support, a body relabelled as an arc, a forged digest and forged and omitted support counts; except the duplicates, each is read with and without the bound request. Each reader's own census tests cover the other missing, extra and duplicated subjects | §5.3.2; the shared corpus; each reader's census tests | the independent numerical review of I3a covers it (item 6); the reader on the native verdict path is §5.3.9 item 4 | none |
| Bessel (§5.3.9 item 7) | temporary capability limit | §5.3.4 | option B, or other common-M evidence; Agent 0 | that evidence |
| Discretization factor c (O-5) | not adopted | the f_e observations (O-5) | whether to adopt `MODAL_MASS_DISCRETIZATION_COARSE`; M1a owner | its benchmark |

##### Agent 0's decisions on the source guard (2026-11-04, on `8d02280c7`)

- **Approved for bounded implementation: the guard FORM and the staged placement P0, S1, S2, S2b and S3.** No numerical weight, allotment or ceiling is adopted.
- **What W is.** W, a checked u128 sum of picosecond weights × counts, is a calibrated WEIGHTED-WORK budget in nominal time units. It is not a runtime guarantee.
- **What the counts and allotments include:** the state case and the static confirmation, before their work; every computed and recovered pair; the massless census; and refused outcomes. Nothing is dropped when a quantity is unresolved.
- **When each allotment is checked.** QL-rotation and band-pair allotments are checked before the expensive iteration or scaling. The confirmation allotment is checked before each unbounded formation evaluation. A refusal refuses the whole operation cleanly, with no partial authority.
- **The S2 symbolic-RCM move** keeps exactly the old ordering, selection and diagnostics, and accounts for its own cost.
- **Bounds and allotments.** P0 may use conservative bounds, and later exact checks refine them. A bounded projection allotment is reserved, rather than allocating or charging a full-spectrum document unconditionally when the request is small. A completed cluster is never truncated.
- **Refusal names approved** as proposed, in the existing MODAL namespace. Readers and outcomes distinguish a numerical refusal, a resource-budget refusal and a counter overflow.
- **The conservative-reviewed-weight-or-refuse-a-nonzero-term rule is approved** for the experimental and production admission guard, as an honest capability boundary.
  - It is NOT an accepted release limitation, and it is no permission to drop arcs, bodies or state cases permanently.
  - A concrete conservative weight needs an independent basis, and a guessed weight is never called a hard bound.
  - The release target's adopted families and the ~3,000-DOF dense goal stay. Budget fit against usability is weighed without a new benchmark programme.
- **Not adopted by this decision:** the user cap, the weights, the stage allotments, R_max, B_max, the memory ρ and the ceiling.
  - ρ = 1 over cumulative REQUESTED bytes is no guarantee of process RSS or the allocator's peak. The allowance for rounding, layout, stack, runtime and result custody has to be covered explicitly and reviewed.
- **B_doc stays proposed** until its four items close, refused documents included. The 25-byte allowance is verified against the actual std format paths, not a sample width.
- **Coordination.** The S1 `modal_eigen` change and the static-confirmation allotment seams are coordinated with RT-A3 P and the certificate changes before any shared write. The modal resource budget may not silently change any ordinary static output.
  - **RT-A3's post-P shape (2026-11-04; P at `claude/rt-a3-p` `7d320980c`, under review, may still change).** Writes to NI `confirmation.rs`, `structural_adapter.rs` and NI `lib.rs` are held until P integrates.
    - **Solves are counted separately from factorizations.**
      - Sparse mode: one factorization per iterate and k+1 completions (k unit loads plus the final), plus one legacy DEC-050/053 observation factorization on the final solve.
      - Dense mode: k+2 full solves, each with its own factorization.
      - The all-active recovery trial: a separate step with one factorization (k = 0).
      - Confirmation: one gate factorization per converged state.
      - T1's term therefore becomes separate solve and factorization counts per mode, replacing "≤ (I_as+1)·(2+d)+1 solves".
    - **Hook points**, neither inside evidence arithmetic:
      - the solve count at `LoopSystem::solve_iterate`, the single funnel for every loop solve in both modes;
      - the confirmation allotment inside `ConfirmationContext::{evaluate_at, reaction, displacement}`, or at their call sites in `confirmation.rs`, refusing before each evaluation. Entry is `confirm_converged_state`, then `confirm_with_applied`, then the dense or sparse `confirm_with_formation_check`.
    - **Registration:** hooks there require restating the R2/option-(c) source pins and the B7a adapter pin.
    - **R-3's G2** keeps its own exact-work allotment in `scrutinize_gaps`. The modal counter stays out of it, and the order of refusal between the two is defined. If R-3 routes per-iterate checks through `ConfirmationContext`, the modal allotment is scoped to post-convergence confirmation.
- **Sequence.** The priced closure and D3 order stays #105/B → PF1b → I3a; an isolated, pre-registered implementation is fine. No measurement window, native enablement or recording-policy adoption follows.
- **A3 was then held** (decided 2026-11-05, §5.3.7 "A3, as decided"). The next step was a minimum concrete proposal for a resolution and screening criterion: the exact roles of τ_μ, c_μ, κ_μ and c_f, and their known failure behaviour.
  - τ_μ is not tightened to recapture the 1,920 high modes.
  - The general nullspace is not truncated.
  - No resolved finite count is claimed.
  - Any eigen-kernel change it needs stays clearly separate from adopting parameters.

##### P0 and S1 in the product, as built (2026-11-06; no value adopted)

Built on `claude/rt-dynamics-p0` from main `17aa9a1f9`, under Agent 0's approval of the guard form and placement (above), and revised after the independent review of `4952e5c61` (three blocking findings, two should-fixes). **No weight, allotment, R_max, L_A or ceiling is adopted;** L_A = 202 and the illustration's values stay proposals.

**The rule P0 follows.** Every count c̄_t that P0 charges is an upper bound on the work it charges, in a stated unit, derived from source for every admitted input. A term with no such bound is *unbounded*, and P0 refuses any request in which it occurs. An allotment that only a later, unbuilt stage would enforce is never charged as a bound. The module docs of `modal/resource.rs` carry each derivation.

**The capability boundary today: P0 refuses every budgeted request.** T2's integrity work is unbounded and occurs in every request (and T1's integrity and confirmation work in every request with a state solve). This is the approved weight-or-refuse boundary, not a permanent narrowing: P0 admits nothing until the unbounded terms are bounded. A request without a budget is unaffected.

**The support terms, bounded (2026-11-07; independently scrutinized source candidate `b1e2d1f97`, D3/integration pending; AC-004 7/7 on actual composed `8c26b7556`).** `T3.filters`, `T7.assembly` and `T9.construction` were unbounded because they rested on std's `binary_search`, sorts, `BTreeMap` and `VecDeque`, whose step constants are not repository source. `modal_eigen`'s `support` module replaces them on the modal path with repository-owned operations that give exactly the former results: the massive-DOF lists by a cursor walk over the ascending massless list; every ordering by a stable counting sort (column then row, and by matrix first for the Sturm records); the formation maps by slots assigned in key order with the visits replayed in their former order, so each `ExactSum` receives the same adds in the same order and the same add refuses an allotment; the searches by merge walks; and the reverse Cuthill–McKee order by compressed neighbour lists ordered without a sort, with the same seeds, pseudo-peripheral rule and breadth-first order. Each function states its count in *support steps* (`T3.kernel`'s step: at most 64 operations, counted literally, under the same premises) and its capacity. **Reservations** (independent review of `1091d8851` found the capacity claims unenforced: unchecked sizes, infallible allocation, and an empty counting order whose starts overflowed where the former map returned `Ok`): every structural vector of these terms (the complements, the counting orders and their starts, the RCM lists and traversal state, the assembly's keys, slots and entries, the formed and sorted lists, the accumulators, the off-pattern list, the norms' rows, and the Sturm construction's merged, sorted and union lists, scaling and row sums) is requested once by `support::reserve`/`filled`, at an element count computed with checked arithmetic, its byte size checked against `isize::MAX` before the request, and the request made by `try_reserve_exact`. A capacity that does not fit refuses `MODAL_RESOURCE_ARITHMETIC_OVERFLOW`, and an allocator refusal the new name `MODAL_RESOURCE_ALLOCATION_REFUSED`, typed (`SupportResourceError`, re-exported by `modal_eigen`, inside `DenseError`, `ReformationFailure`, `FormationError` and `SturmError`) and mapped by the product's `modal_support_resource_refusal` (`diagnostic:modal:resource:support`) on the dense solve, the re-formation, the formation difference and the Sturm construction: a resource refusal, never a numerical, mass-census or budget refusal. An input with nothing to order requests nothing (the empty pencil of any dimension is `Ok` again). A reservation is one support step, so each count gains its reservations' steps; a refusal ends its call there. Each structural vector then holds at most its requested count and makes no further request; the stated capacities are requested sizes, not allocator rounding or resident memory. The `ExactSum` limbs, the reads' scratch and term lists, the element lists and the retained stage workspace keep their existing growth by std's infallible allocation, whose failure still aborts (the limb allotment bounds the limbs); their lifetimes are the open ledger's. The kernel's public `established_finite_mode_count` and the evidence accessors (`assembled_stiffness`, `assembled_mass`, `assembly_visits`) keep their signatures and panic with the refusal's message, as the former allocation did; the solve takes `established_finite_mode_count_checked`. Tests inject an allocator refusal at every reservation of each consumer through a test-only failpoint (no product build has it) and check the inclusive size boundary and overflow with no oversized request. The old code is kept only as test-only oracles: `sparse_direct`'s `reverse_cuthill_mckee`, and verbatim copies of the former map, sort and search code in `modal_eigen`'s unit tests. Old against new: those oracles on random and adversarial cases (every allotment from 0 to one above the longest sum, duplicates, empty and singleton inputs, reverse orders, degree ties), compared by every field and every float's bits; and on the 35 population models the RCM order against `sparse_direct`'s on each model's own pattern, and each actual size against its bound. The committed fixtures and producer edges are to pass unchanged. `dense.rs` keeps its line numbers, so `T3.kernel`'s ledger references stand.

- **Entry.** `run_modal_dense(req, params)` delegates to the internal `run_modal_dense_with_budget(req, params, None)`. The budget is a separate, borrowed `Option<&ModalResourceBudget>`. It is not a field of `ModalIntegrityParameters` and is never defaulted. With `None`, no P0 runs and the kernel takes `DenseAllotments::default()`.
- **Placement** (premise checked). `prepare_modal` forms no stiffness and no mass, and allocates only the model, the build and O(N + E) records (`prepare.rs:138`–`:388`). The first n²-scale work is the state solve's `form_basis_stiffness` (`state.rs:348`), or with no state case `form_modal_pencil`'s guard (`stiffness.rs:142`). P0 runs before both.
- **Arithmetic.** Checked u128 counts for 34 terms. W = Σ w_t·c̄_t, with `checked_mul`/`checked_add`. The stage allotments A_s are formed for P0, S1, the confirmation counter, S2 and S2b, and Σ_s A_s ≤ W_budget is inclusive. An overflow refuses as `MODAL_RESOURCE_ARITHMETIC_OVERFLOW`. Each weight is an `Option<u64>` in picoseconds per counted unit. An unbounded term, or a nonzero count with a `None` weight, refuses as `MODAL_RESOURCE_PREFLIGHT_EXCEEDED`, naming every such term with its reason. An exceeded W_budget names the dominant term.
- **Dimensions** (each an upper bound from source):
  - **n̄_f** = n_g − |distinct linear restrained DOFs|; with a restraint basis, n_g.
  - **n̄_c** = n̄_f, and **n̄_r** = n̄_f. R_max is recorded and not used, because S2, which would enforce it, is not built.
  - **p̄** = min(n̄_f, A_band) and **R̄** = A_QL; S1 enforces both.
  - **s** = s_max.
  - **D_K, D_M, E** are taken from the re-formation's element sizes. **μ** (exact-product calls per element entry) is 1, or 36 with a restraint basis, and **Q** = 8μ is the parts per entry.
  - **ν** and **θ**, the assembly's position visits and traversal steps per element entry, are 1 and 3, or 9 and 15 with a restraint basis (at most three coordinates per mixed DOF).
  - **F̄ = P** entries in each formed list, so **J̄ = 2P** bounds their combined entry count J: K's list is one entry per stored column c ≤ r of each free row of a compressed matrix with sorted, deduplicated rows; M's one per stored DOF pair of the upper node blocks (i ≤ j), or one per coordinate pair of each node pair with a restraint basis; each a distinct lower position under the injective free numbering.
  - **L̄ = 492 limbs** for every `ExactSum`, derived as follows; L_A is recorded and not used:
    - An accumulator whose parts have representation bits in [B_lo, B_hi] and fewer than 2^c adds holds at most ⌊(B_hi + c)/32⌋ + 4 − ⌊B_lo/32⌋ + 1 limbs. Its low limb only moves down, its length never shrinks, and a carry pushes a limb only while the top limb is at least 2³¹.
    - A TwoProduct part of a product of binary64 factors (e ∈ [−1074, 1023]) lies within the exact product: bits at or above Σ(e − 52), a representation lsb at most 52 below that, and a top bit at most Σe + k.
    - **Level-1 sums** take at most five factors per product (`product_into` on a mixed element with a factor), so B ∈ [−5682, 5120] and L̄₁ = 347.
    - **Level-2 sums** (`add_terms_product`, and the formation difference's terms) multiply a level-1 sum's terms by at most two factors, so B ∈ [−8000, 7456] and L̄ = 492. No level-2 sum's terms are multiplied again (`reformation/mod.rs:1510`, `orthogonality.rs:176`).
    - **Adds:** fewer than 2¹³¹ per accumulator, since every add is a part of a counted call and P0 refuses a count ≥ 2¹²⁸; so c = 132.
    - **Limb work:** one part's add costs at most 5L̄ + 16 limb operations, and one read at most 12L̄ + 32.
    - This withdraws §5.3.8's earlier "no data-independent bound on L". The three reasons given there are each covered: the subnormal representation lsb (the −52), limb growth by `normalize()` (the carry allowance), and sums of accumulated terms (level 2).
    - This bound is a numerical-meaning claim. Independent scrutiny of `1091d8851` confirmed the 347/492 spans and their T7 dependency; the repaired-source recheck at `b1e2d1f97` cleared the connected support accounting. This supplies no adopted L_A or whole-invocation memory bound.
- **Bounded terms** (unit; derivation; stage):
  - **T1** (with nonlinear supports; P0). S̄ = (I_as + 1)(2 + d̄) + 1 solves, each through frame_kernel's `cholesky`. `T1.factor` = S̄ × T2.factor, `T1.entries` = S̄ × T2.entries, and `T1.coupling` = (I_as + 1)(⌈2d̄³/3⌉ + 2d̄² + d̄·n_g).
  - **T2** (P0). `T2.factor` = (n − 1)n(n + 1)/6 Cholesky inner iterations, exactly the k-loop of `structural.rs:2089`–`:2095`. Each iteration is one checked product, one checked subtraction and one checked absolute addition; at n_f = 6 that is 35 iterations, as the review counted. `T2.entries` = 3n² entry steps: the n(n + 1)/2 factor entries (each a division, or a square root with its pivot screen), the finiteness scan, the symmetry scan and the factor allocation.
  - **`T3.filters`** (S1) = 2(n + 2) support steps: two `support::ascending_complement` walks (`dense.rs:424`, `:969`), n + 2 each (the indices, the call and the reservation).
  - **`T7.assembly`** (S2) = (6θ + 24ν + 7)(D_K + D_M) + (55 + L̄)(min(νD_K, P) + min(νD_M, P)) + 73P + 70n + 81 support steps. Per matrix: both assemblies (pencil and formation difference) at 3t + 12v + 4n + 11 each; the pencil's range check at 5 a position, and the refusal's element search at 7D; `formed_lower` at 9F + 4n + 9; and `difference` at 27F + 50u + uL̄ + 14n + 7, which includes `scaled_norm`'s WideDd arithmetic at 24 steps an entry and 7 a row. Once: the diagonal walk and scaling at 18n + F_K, and 5. The constants include each reservation's step and the checked visit count's call (before the reservations: 3, 3, 3 and 4, so 28 in all). The WideDd figures are literal operation counts rounded up with a margin (about 1,253, 265, 230 and 632 operations for the entry, the row, the range check and a diagonal's scaling); they are part of the review.
  - **`T9.construction`** (S2) = 13J + 9n + 5P + 24 for `ShiftPencil::from_entries` (the checks, three stable counting sorts, the union pattern, the scaling and the row sums, with twelve reservation steps), plus the RCM order's 15n + 8q + 14 + (n + 3)(5n + 2q) + 3n(n + 1) with q = P − n (eleven reservation steps); at J̄ = 2P, 39P + 16n + 38 + (n + 3)(3n + 2P) + 3n(n + 1). Per component of m nodes and h half-edges: at most m + 2 pseudo-peripheral traversals and the final one, each at most 3m + h + 2 steps, and least-node scans at most (m + 1)(m + 2).
  - **T3** (S1). `T3.kernel` counts kernel steps of `solve_dense_pencil_with_allotments` (`dense.rs:358`) and its helpers, item by item. The count is 5·n(n+1)/2 + 2·Σ_i Σ_{j≤i} j + 37·n(n−1)/2 + 3n·n(n−1)/2 + 3·Σ_{t<n} t² + 21n² + 148n + 23 + R̄(2n + 5).
    - **The unit** (revised in round 3: round 2's audit omitted reads and assignments under its own convention). A kernel step is at most 64 operations, counted literally:
      - every read of a named value;
      - every arithmetic operation, absolute value, negation and `!`;
      - every comparison or predicate (including `&&`, `||`, match and `?` tests);
      - every binding or assignment (including parameter bindings, returns, constructed fields and pushes).

      Only loop control, index arithmetic, bounds checks and `debug_assert`s are excluded. An unclear count is rounded up, and a body of k operations is charged ⌈k/64⌉ steps.
    - **Premises, not repository source:** a core slice or `Vec` element primitive is one step per element, and allocator calls are charged in no term. `f64::sqrt` is one operation.
    - **The ledger** is in `resource.rs`'s module docs: one row per body, with its lines, per-line counts, total and steps. The bodies near or over 64 operations:

      | Body | Lines | Operations | Steps |
      |---|---|---|---|
      | `hypot`, inlined | `:1143`–`:1150` | 29 | — |
      | QL iteration | `:1305`–`:1333`, `:1357`–`:1360`; `charge_ql_rotations` 28 inlined | 149 | 3 |
      | QL rotation scalar | `:1335`–`:1350`; 81 before `row_pair`, `row_pair` 19 | 100 | 2 |
      | QL row update | `:1352`–`:1354` | 18 | 1 |
      | screen body | `:557`–`:581` | 106 | 2 |
      | thresholds | | 129 | 3 |
      | ordered Cholesky entry | | 52 | 1 |
      | census entry | | 39 | 1 |
      | Householder per-row block | | 37 | 1 |
      | other straight-line code | | ≈ 412 | 12 (768) |
      | Hager's scalar control | | ≈ 380 | 8 (512) |

      Every other body is at most 36 operations. The audit also added per-row and per-call overheads the earlier count missed: the Cholesky row closures, the forward-solve calls and the back-transform calls.
    - R̄ enters as R̄(2n + 5): the iterations are at most the rotations, each iteration is 3 steps plus its shift loop (≤ n), and each rotation 2 steps plus its row update (n).
    §5.3.8's 19n³/3 + 6n·R and n³/3 are leading-order calibration units (w_b64, w_screen), not upper bounds. They are not used as counts.
  - **T4** (S1). `T4.scaling` = 2n² + n; `T4.visits` = `T4.insertions` = 4p̄n²; `T4.rows` = p̄(n + 2); and `T4.limbs` for the limb work.
  - **T5** (S2). `T5.census` = 16n̄_c linear-pass steps, and `T5.outcome` = n̄_c·p̄ + 2n̄_c² + n̄_c for the outcome's band flags and cluster ranges.
  - **T6** (S2). `T6.visits` = n̄_r(n² + 3n), plus `T6.limbs`.
  - **T7** (S2). `T7.elements` = E, plus `T7.limbs`.
    The support-operation repair makes the complete limb multiplicity explicit:
    `ReformedPencil::new_on_coordinates` assembles each matrix for its range check,
    and `formation_difference` assembles it again. The first reads each assembled
    sum once; the second reads it for matching/off-pattern data and for the
    re-formed norm. Thus the insertion bound is
    2Q(D_K + D_M) + n(n + 1) + L̄·μ(D_K + D_M), and the read bound is
    3μ(D_K + D_M) + n(n + 1). The weights per insertion/read remain
    5L̄ + 16 and 12L̄ + 32 limb operations. These are upper counts, not runtime
    measurements or adopted resource weights. The three assembly traversals
    replay already re-formed entries; they do not repeat element re-formation,
    so `T7.elements` remains E. Test-only observations of the actual constructor
    and difference path verify the two assemblies per matrix, the range/matching/
    norm reads, and an early limb refusal; a separate hand count checks the
    connected P0 charge. Capacity lifetimes remain stated in each support helper;
    the whole-invocation memory ledger is still open.
  - **T8** (S2b). `T8.insertions` = n̄_c·μ(D_K + 2D_M); `T8.generalized_mass` = n̄_c·n·L̄; `T8.rows` = n̄_c(2n + 1); plus `T8.limbs`.
  - **T9** (S2). `T9.factorization` = s(n(n − 1)(n − 2)/6 + n²) LDLᵀ steps; `T9.passes` = s(29P + 98n + 70), covering the formed list, the profile, the observations and ≤ 11 ordered solves; `T9.neighbours` = 2(P + 3n); plus `T9.limbs`.
  - **T10** (S2b). `T10.products` = n̄_r·μ·D_M; `T10.bilinear` = n̄_r(n̄_r + 1)/2·n·L̄; plus `T10.limbs`.
- **Unbounded terms (refuse):**
  - `T2.integrity`, and per state solve `T1.integrity`. This is the M03-INTEGRITY work around the static factor: the `SparseAssemblyEvidence` construction, the dense view, `prepare_bound` (radix scaling, `ExactAccumulator` right-hand sides, the skew audit), `audit_contributions`, the triangular solves, the residual records, the condition estimate and the finish. Its `Expansion` lengths are bounded (≤ 2,099 nonoverlapping binary64 components), but its pass counts are not derived in this slice.
  - `T1.confirmation`. E_conf has no source bound, and its A_conf counter is a held NI seam.
  - Until 2026-11-07 also `T3.filters`, `T7.assembly` and `T9.construction`, which rested on std's search, sorts and maps; they are bounded above.
- **S1 in the product** (`eigen::solve_modal_pencil_within`). With a budget, the kernel runs under A_QL and A_band. Its typed resource errors map to `MODAL_RESOURCE_QL_ALLOTMENT_EXCEEDED`, `MODAL_RESOURCE_BAND_ALLOTMENT_EXCEEDED` and `MODAL_RESOURCE_ARITHMETIC_OVERFLOW` (`diagnostic:modal:resource:kernel`). While P0 refuses every budgeted request, `run_modal_dense_with_budget` cannot reach S1, so S1 is exercised through the product's own solve on the product's own pencil.
- **Catalogue.** `MODAL_RESOURCE_PREFLIGHT_EXCEEDED` is defined in `integrity.rs`, and the three kernel names are re-exported there. The kernel's fourth, `MODAL_RESOURCE_ALLOCATION_REFUSED` (`dense.rs`, 2026-11-07: an allocator's refusal of a support structure's requested capacity), reaches the product only through `SupportResourceError::code`. No generated document or table carries them. `document.rs` is untouched, so `modal_doc_solved`'s §8.52 pin `f4907717…` stands.
- **Evidence** (`modal/tests_resource.rs`; test-local budgets only):
  - every count equals its formula, evaluated independently, on SEC-A N = 1 (T2 by hand: 35 and 108), the arc and the U-1 state case, and T3 at small n;
  - every actual count (n_f, n_c, n_r, p, R, s, D_K, D_M and E, and with them every bounded c̄_t) is within its bound on all 35 population models;
  - R_max = 0 and L_A = 0 leave the bounds unchanged, and T6, T7, T8 and T10 are charged;
  - L̄ matches its derivation and holds on adversarial level-1 and level-2 sums (a witness, not the proof);
  - every population model refuses under a budget, naming exactly its unbounded terms (since 2026-11-07, `T2.integrity`, and with a state case `T1.integrity` and `T1.confirmation`);
  - weight-or-refuse, the inclusive boundary with the dominant term, and overflow (each on the judgment);
  - placement before the state solve (a gap model's solve runs without a budget, and P0 refuses first with one) and before the pencil guard;
  - S1's codes, with the actual R and p passing bit for bit;
  - the support reservations' refusals: an overflowing dimension on the actual re-formation and formation-difference route and the Sturm construction's actual error refuse `MODAL_RESOURCE_ARITHMETIC_OVERFLOW`, and an allocator refusal `MODAL_RESOURCE_ALLOCATION_REFUSED`, each by `diagnostic:modal:resource:support`; in `modal_eigen`, a test-only failpoint refuses each reservation of the complements (2 through the solve), the counting orders, the RCM order (11), the constructor's assemblies (14), the formation difference (33) and the Sturm construction (23) in turn, each typed and stopping its call, with the unarmed result after;
  - T3's itemized count against its collected polynomial, and at n = 1 by hand.
- **Old against new.** The one changed numerical call, `solve_dense_pencil` before against `solve_modal_pencil` now, is compared on all 35 models' own pencils by an exhaustive key. Every field of the solution and of each screened mode is included (the destructuring has no `..`, so a new field fails to compile), `Option` presence is kept, and every float is compared by `to_bits()`. The committed fixtures' bytes and the producer edges (`tests_document`), which main's code generated, are unchanged. `run_modal_dense` against `run_modal_dense_with_budget(.., None)` checks only the wrapper's delegation, by `Debug` text, and is described as such.
- **Not charged:**
  - T11, the projection. It runs after `run_modal_dense`, and its count needs B_doc.
  - Work outside the cost table: K's formation, the mass assembly, the pencil's n² fill, and the per-mode O(n_g) publication.
  - Two specifically identified pre-existing std-set operations also remain
    outside the table: `ModalBasis::new` inserts each free DOF into a `BTreeSet`
    to reject duplicates (at most n_f insert attempts and n_f retained keys);
    P0's dimension calculation collects valid restrained DOFs into a `BTreeSet`
    (r insert attempts and at most r keys, r the supplied restrained-list length,
    on the non-skew path; none on the skew path). For prepared product boundaries,
    `add_restrained_dof` rejects repeats and only valid global DOFs reach P0,
    so r ≤ n_g. Their comparison/node/capacity constants are not charged by
    `T6.visits` (which charges per reported mode's mass/dot products), nor by
    `T5.census` or any self-charge for P0. This is a source classification of
    existing whole-invocation gaps, not an additional admitted cap, release
    limitation or implemented T6/P0 extension. The algorithms remain unchanged.
- **Open items:**
  1. Bound the unbounded terms: T2's and T1's integrity passes and T1's confirmation counter (held NI seam). The support terms of T3, T7 and T9 passed independent numerical and resource scrutiny at `b1e2d1f97` (2026-11-07); their private D3 proposal and integration remain pending. `ModalBasis::new`'s duplicate-DOF check is a std `BTreeSet`, work outside the cost table and uncharged.
  2. The memory ledger is not checked. The option-(c) ledger's rows still carry O(·) terms and approximate constants. #110's reuse (`modal_eigen::workspace`) retains each stage's capacity until its owner (`ReformedPencil`, `ModalBasis`) drops, so a ledger must count it as live over that span.
  3. S2, S2b and the confirmation counter are not built.
  4. The weights stay undetermined.
- **Pricing.** `resource.rs` is a new priced source in product_physics. Its D3 is DEL-04-08 §8.58 (proposed): coefficient-equal to main's §8.57 tree (1,602 equal), with a MODAL-FB subtraction for `modal_resource_bound_dimensions`' u128 `min`, MODAL-ID classes for the refusal texts (`resource.rs` pinned whole) and the three `run_modal_dense` sites re-keyed to `run_modal_dense_with_budget`. AC-004 is required, because both witnessed binaries change; it ran once and passed 7/7 at `ef009fa0b` (§8.58). This is pricing registration only: no value is adopted and nothing is admitted. The bounded support increment has its own unadopted proposal at DEL-04-08 §8.59, selected for actual-main `00832d746` composition; the earlier 7/7 does not transfer, and the one fresh direct-supervisor attempt passed 7/7 on clean `8c26b7556`/core `a3633b180` (§8.59). Exact-head CI, consultation and integration remain pending; this closes no additional source-bound frontier term and adopts no value or admission.

#### 5.3.9 Admission: what stays disabled, and why

Public engineering-result admission and every UI stay **disabled**. They remain closed until each of these closes:
1. **Production parameters:** one complete instance, which needs **A1, A2 and A3 all closed** (§5.3.7): A1's approved criteria, A2's adopted criteria (κ_c, κ_s, τ_o, δ_c, and τ_r = 2⁻³⁰, 2026-11-04), and A3's criteria (τ_μ = 2⁻⁵³, c_μ = 1, κ_μ = 10, c_f = 64, 2026-11-05). All three are selected. The instance may now be implemented as a bounded explicit constructor with no `Default`, reviewed with its source, regression and D3 under DEC-026 (O-6, D-TOL). Selecting the parameters enables nothing: admission still waits for every other item here. Until the instance is reviewed, every document carries a labelled test-local set.
2. **Units:** the mass-coupling family is adopted (`mass_length_per_rotation`, §5.3.5; D-01 engineering extension 2026-11-04), the equilibrated-basis list is closed and empty, and λ is not published (Agent 0, 2026-11-04). The two D·M·D magnitudes are `time_squared` (option (a)), so units pass for current documents. Units passing closes none of the other conditions here.
3. **Resources:** O-4 measured in stages (§5.3.8: stage 1's small points first, any larger stage proposed from its measured phase costs and separately authorized), then D-k's work guard, the dense modal path's counted memory peak and the projection's own size guard built from reviewed measurements. Stage 1 alone closes none of them.
4. **The full reader's scope:** the selected model-derived mass correction above, its exact-text and numerical review, Agent 0's scope review and the reader wired into the verdict path that a native consumer uses. The built payload checks alone do not satisfy REQ-005's model-derived mass promise. K, the re-formation and the eigen-solve remain outside reader rederivation; state/format coverage limits remain explicit.
5. **Recording policy:** O-8 (task record, store, version) decided; until then documents are transient.
6. **D3:** I3a's priced change registered (DEL-04-08), and an independent numerical review of I3a. The pricing preparation at `372a16aa5` stopped (exit 5, every point incomplete) at 8 arguments with no class, all at I3a sites in `modal/document.rs` and `modal/tests_document.rs`. The O-4 measurand and the A2 and A3 observation tests add none (§5.3.8). **The D3 is proposed at DEL-04-08 `retained-pricing-rules.md` §8.52**, on PF1b's §8.51 head with #109 (priced source `687fafdca`). It adds MODAL-ID classifications for those arguments and corrects three by-name classes below the DOF-id and mode-ref bounds. It re-pins B7a's `UNITS_SHA` for the two unit-catalog dimensions. All 1,602 coefficients are equal, so §8.51's block is kept. It merged with I3a in PR #113 (`08bfdfdaf`), with AC-004 run on its binary and recorded in that PR. The independent numerical review of I3a remains open and is separate from the D3. The production instance (PR #131) takes its own D3, **proposed at DEL-04-08 §8.57** on main's §8.56 tree (priced source `11e9a1429`): the constructor is unreached and adds no loop, text site or class, all 1,602 coefficients are equal, and no pin moves. AC-004 is required, because the witnessed binaries change; it ran once and passed 7/7 at `089a5698f` (§8.57). This is pricing registration only: it admits nothing.
7. **Bessel (temporary capability limit, not an accepted release limitation):** under Agent 0's interim contract (§5.3.4), Bessel passes only where the readers establish it, and is otherwise unresolved, which confers no `fully_validated` status. Closing it needs option B, common assembled-M Gram evidence, or another justified common-M bound, reviewed by Agent 0.

FR-SOL-016 stays partial (REQ-010; O-3's benchmark rights stay held), and the gap-support limitation (O-14 (a)) stands; neither is changed here, and no O-3 preview gate is added.

#### 5.3.10 The next slice (proposed, not started)

**I3b, the native worker and custody path,** after #59 (RT-U4 C1, native result custody) merges, or stacked on it at the hunk boundary agreed with the performance lane:
- **No second job stack.** One modal job kind in the existing `SolveJobRegistry` (`apps/desktop/src-tauri/src/solve_jobs.rs`) and its same-executable worker (`solve_worker.rs`), through one public product entry that parses the request, runs `run_modal_dense` and projects the document (a priced `lib.rs` change with its D3).
- **Custody.** The document's bytes are held natively as #59's `HeldResult` with `ResultCustody::Native` (its `solve_jobs.rs` hunks adding the custody constants and `HeldResult`/`ResultOrigin`); the webview receives the reader's verdict and paged projections (the modes table, the header lines, one mode's shape), never the full bytes.
- **No header shortcut.** Health, standing and admission shown come only from `open_pipe_stress_modal_result::read_modal_result` run natively on the held bytes with the bound request, never from a transport header or the document's own `standing` member.
- **Guards first.** The projection's size guard (§5.3.8) runs before the document is formed, against #59's custody limits.
- **The subject census (§5.3.2) is built; the native path is not.** When I3b is built, it must run `read_modal_result` with the bound request. That closes the census scope, so a native document's subjects are decided against its own model; a linearized state case stays scoped on its support states' qualification (§5.3.4).
- **Still needed as production evidence:** the pins (§5.3.7), the units extension (§5.3.5), the O-4 measurement, the size-guard coefficients from that window, a native memory-fit observation of the held document under #59's custody, and one native smoke of a modal request. UI (M3) follows I3b.

---

## 6. Sequence and size

Agent 0's sequence, 2026-10-27:
1. **M1a, mass:** `modal.rs` request DTO and refusals (§1.2, §1.4); mass assembly (§2.2–§2.4); the W/a basis, its range premise and its refusals (§2.3); the weight-basis check and the measurement that pins its tolerance (§2.5); mass summary rows; the discretization warning and the derivation of c (§2.6); the `pub(crate)` seams in `self_weight.rs`. References for the mass and resultants in the `w15_component_weight` generator style. **The one D3** (PP `lib.rs`, `self_weight.rs`).
2. **M1b, the dense path, with every check of §3.4:** after DEL-02-02's unit-catalog families (§1.5); `core/solver/modal_eigen` (Cholesky reduction, tridiagonal QL, the Jacobi control in tests, the double-double re-formation with frame_kernel's re-formation seam, residual estimates, M-orthogonality, the reported-set and cluster rules, both normalization cases, participation); **the count infrastructure the dense results need**: the scaled inertia factorization through `sparse_direct`, the `sparse_direct` observation accessor, the shift placement under the conditioning and separation premises and the growth limit, with the inertia-risk tests for the dense path's counts (§3.2 item 7, §3.3, §3.4 check 4); the linearization of nonlinear supports at a named case solved in the same run (D-s, formerly M5); skewed restraints and snubbers (§1.3); the product entry; the dense memory and work guards; the modal-1 table, profile and run kind; the Rust and Python readers; the headless command. References for §4.2 and §4.3. D3 for the PP `Cargo.lock` (and `semantic_contract.rs` if used), the frame_kernel seam and the `sparse_direct` accessor, joined with M1a's if they merge together.
3. **M3, UI:** the Tauri command and types, the request form with the per-project remembered bound, the modes table, the mode-shape overlay and animation with reduced motion, the report section (Rust and TS), Vitest and a native smoke step.
4. **M2, the sparse path:** shift-invert block Lanczos with full reorthogonalization in the range of T (singular M, §3.3), locking, breakdown handling and deterministic restarts; spectrum slicing, with its shifts under M1b's conditioning and separation premises; the sparse guard with its peak count; the inertia-risk tests extended to slicing shifts and to dense-against-sparse agreement, since "the unpivoted-LDLᵀ inertia risk" is "a first-class test target" (§3.3); scale benchmarks timed under `t3_exclusive.sh`. It uses M1b's count infrastructure unchanged, and no D3 is expected (§3.6).

**Order (Agent 0, 2026-10-31).** M1a and M1b cover both routes, and M1b includes the linearization of nonlinear supports (D-s). M3 comes before M2: the sparse path follows the views and precedes response-spectrum results, reordered if the dense guard bites earlier.

**Start and queue.** PF Timoshenko (#1663) has landed. D3 runs join the queue on Agent 0's go.

**Size (estimates, not measurements):**

| Slice | Product and test lines | Reference lines | PRs | D3 |
|---|---|---|---|---|
| M1a | 1,000–1,400 | 400–600 | 1 | yes, once |
| M1b | 2,600–3,600 | 800–1,200 | 2 | yes (lock, frame_kernel seam, `sparse_direct` accessor), joinable with M1a's |
| M2 | 1,000–1,500 | reuse M1, plus generated large models checked against dense | 1–2 | none expected |
| M3 | 1,700–2,300 | – | 2 | none |
| **F1** | **≈ 6,300–8,800** | **≈ 1,200–1,800** | **6–7** | one or two, plus the units change's own |

**Later slices, outside F1's core:**
- **M4.** Timoshenko consistent mass (interdependent interpolation, e.g. Friedman & Kosmatka, *Computers & Structures* 47(3), 1993; cited, derived). It requires the PF Timoshenko unit and model document 0.8.0 (#1649).
- **M6.** The mass-point helper: a model operation that subdivides spans.
- A concentrated-mass field (W-17, a new document version).

**Dependencies on other lanes:**
- **W-17 (model document):** none for M1–M3, since every parameter is on the request. M4 needs 0.8.0's `shear_model` (#1649).
- **PF Timoshenko** (#1663): M4 only, and the start condition above.
- **Survey A9** (weight and centre-of-gravity summary) falls out of M1a's mass assembly. A6 consumes M3.
- **DEL-02-02 (units):** the unit-catalog families of §1.5, before M1b.
- **DEL-04-01 (kernel):** the frame_kernel re-formation entry and the `sparse_direct` observation accessor (both M1b), both read-only seams that F1's slices make in DEL-04-01's code.
- **The deliverable graph.** `python3 -m tools.deliverables … check` reports one cycle, a strongly connected group. DEL-08-01 needs DEL-04-09 (the report section), DEL-04-09 needs DEL-04-07 and DEL-04-08, which are in the group, and it needs DEL-04-05, which needs DEL-04-07. These declarations bring DEL-04-09 and DEL-04-05 into the group, which had 11 members when F1 was declared and has 19 at the 2026-10-31 revision (as the structural check reports it). The cycle is a fact of the declarations, not an ordering error; the slices' own sequence (above) orders the work.

---

## 7. Settled decisions

| ID | Decision | Settled as | Reason |
|---|---|---|---|
| **D-a** | Re-enter FR-024 for modal analysis and create the deliverable | **Taken:** SCA-013, FR-SOL-016, SOW-081, DEL-04-09. F2 and F3 stay deferred | Agent 0 under the owner's delegation (SCA-013) |
| **D-b** | Lumped or consistent mass | Consistent on straights; rigid-body lumps on arcs and component bodies; lumped diagonal only as a reference control | Agent 0, 2026-10-27 ("Agreed"); O(h⁴) convergence from above and an element-wise weight identity |
| **D-c** | Mode count, cut-off and any default | Required request input: a mode count, a cut-off or both; the lowest modes up to whichever bound stops first, the cut-off inclusive; a cluster (one definition: adjacent computed eigenvalues within the relative tolerance δ_c, chained, §3.4 check 5) at the boundary completed and disclosed by count, never published partially; completeness counted at a separate final shift in the gap above the completed cluster, under the separation premise, on the interval actually reported; the cut-off's offset shift used only to size the sparse extraction (§1.4); no default; the desktop remembers the last value per project. **Changed from the proposal:** no fixed source cap on the mode count. **Changed after Codex round 2:** the final shift replaces the cut-off shift as the completeness count | Agent 0, 2026-10-27: "No code-derived cut-off value ships." A fixed count cap would be a code-derived bound on the extraction; the resource guards (§3.2, §3.3) refuse an oversized request with their formula instead |
| **D-d** | Output units and normalization | f (Hz), ω (rad/s), T (s); exactly two normalization cases (§1.5): (a) a mode with nonzero translation, unit maximum translation (translations dimensionless, rotations in rad/m), m_i in kg, Γ_ik = φᵢᵀp_k / m_i (p_k = (M R_k)_f, D-v) dimensionless, m_eff,ik = (φᵢᵀp_k)²/m_i in kg; (b) a mode with no translation (t ≤ τ_t·ℓ·θ, τ_t = 10⁻⁶), unit maximum rotation (rad/rad), the computed translations unaltered in m/rad (at most τ_t·ℓ), m_i in kg·m²·rad⁻², and Γ in rad/m and effective translational masses in kg computed from the published vector, with no condition on their size; **Revised 2026-10-31** (review round 3 and D-v): case (b)'s generalized mass gained rad⁻², and the participation condition and `MODAL_NORMALIZATION_UNRESOLVED` were withdrawn; cumulative fractions dimensionless; display in project units. **Changed after Codex round 1:** mass-normalized shapes were withdrawn. **Changed after Codex round 2:** the two cases and the threshold made exact | Every quantity keeps integer unit exponents, which the unit catalog requires (`core/units/src/lib.rs:224`); Γ, m_i and m_eff keep what F2 needs |
| **D-e** | Contents in axial and torsional inertia | In all three translations; excluded from torsional and rotary inertia, disclosed | [code] Self-weight applies m̄, contents included, along any global axis (§2.1), so the identity holds in every direction only if contents are in every translation |
| **D-f** | Component mass m = W/a | `weight_to_mass_acceleration`, required when a component weight enters, never defaulted; equal bit for bit to any current applied component-weight case's acceleration magnitude, or refused (§2.3) | [code] W is an authored force that the acceleration's magnitude does not scale, and the model holds no gravity; only a request input gives a consistent a without a default. **Changed from the proposal:** the agreement with applied cases is a refusal, not a disclosure, because a different a would give masses that disagree with the model's own static weight |
| **D-g** | Refusals by name | (Until M4) Timoshenko straights on either route refused by name; load/reference-state documents admitted through the state case (D-w); constant-effort supports, connectors, tie rods and W-16 imposed displacement admitted with disclosures; the exact route admitted (D-r); nonlinear supports admitted only at a named case's solved state (D-s). **Revised 2026-10-31**: until then D-g also refused the exact route and nonlinear supports | Linearizing silently, or solving about an unstated state, would be an unstated engineering choice; a state the user names is not |
| **D-h** | Parameters on the request or in `model_settings` | On the request, captured in the analysis-run record | No W-17 version, and F1 stays uncoupled from the document lane |
| **D-i** | Transcription from published benchmarks | Only with the report's own redistribution evidence (its rights statement and NRC distribution terms), recorded per fixture; a report with uncertain rights is held for the owner's review and nothing is transcribed from it; copyrighted or adapted content excluded; sources with clear rights preferred (§4.4) | Agent 0, 2026-10-27, revising the earlier premise that NUREG/CR reports are US-government works (quoted in §4.4) |
| **D-j** | Participation and effective mass in M1 | Yes, translational X/Y/Z | Without them there is no mass-coverage evidence, and a close frequency does not qualify a wrong mass model (VAL-REF-02 line 682) |
| **D-k** | Dense modal guards | Memory (live n² buffers × 8 B against the provisional 6 GiB ceiling) and work (9n³ + s_max·n³/3 plus separately priced recovery work, including candidate Sturm factorizations, against a budget measured in M1b) | A memory-only guard would admit models that run for an hour on the dense path |
| **D-l** | Discretization factor c | Derived by benchmark in M1a; a warning, not a refusal, with c and its derivation in the warning text | Coarse models otherwise give high frequencies with clean residuals (R3) |
| **D-m** | Sparse eigensolver | Shift-invert block Lanczos, b₀ = 2, deterministic start block, locking and deflation, breakdown replacement, deterministic restart with a larger block and a new seed on a Sturm miss (§3.3) | Scalar Lanczos spans one direction per repeated eigenspace; circular sections give paired modes. Codex round 1 |
| **D-n** | What the integrity checks establish | Numerical-integrity checks with stated premises, not certified enclosures: per mode, the residual's backward error on the double-double re-formed pencil against a stated threshold, with a first-order error estimate; M-orthogonality; Sturm-count agreement at shifts placed in gaps between computed clusters, relied on only under the conditioning premise (the formation-and-factorization backward-error estimate e_b times an inverse-norm estimate ν at most 1/κ_c) and the separation premise (separation at least κ_s·e_b); a shift that cannot be so placed, or whose growth exceeds g_max, refuses the affected set by name (§3.4). **Revised after review rounds 3 and 4** from the earlier (‖K̃‖ + \|σ\|‖M̃‖)·u·g, separation-only rule. No Kato–Temple refinement and no inverse-norm bound beyond the estimator; rigorous enclosures are a later option (§9, O-13) | Binary64 inertia counts, a Rayleigh quotient on the formed pencil and one floating-point solve give no enclosure without interval or exact arithmetic; Codex round 2's witnesses (a shift that rounds away on a 32×32 tridiagonal, a re-formed quotient displaced by 0.01 despite isolation, a solve that underestimates xᵀK⁻¹x by 3×). **Changed after Codex round 2** from inertia-certified Kato–Temple gaps |
| **D-o** | Modal formation check | K and M re-formed independently from their leaves in at least double-double precision (≈106-bit significand), element by element; the formation difference measured and published; each mode's residual evaluated on the re-formed pencil, a mode over the threshold demoted; an uncovered family refuses the set; K-D5's p128 element re-formation reused through a read-only seam, its D5C1 criterion not reused (§3.4) | K-D5 judges a static correction, and a modal run has no static response (Codex round 1). Double-double resolves the formation difference at the 10⁻³¹ relative scale for well-scaled entries, ample against residual thresholds above 10⁻¹², so true quad is not required (Codex round 2) |
| **D-p** | Units for modal quantities | Frequency (Hz), angular frequency (rad/s), rotation per length, length per rotation and rotational generalized mass (kg·m²·rad⁻², revised after review round 3) added to the unit catalog by DEL-02-02 before M1b; period uses Time (s) (§1.5) | The catalog has no such families today. Codex round 1 |
| **D-r** | The exact route | Admitted on its own section basis and mass basis (`EXACT_MASS_METHOD`); the weight-basis identity on the route's own generator; pressure ignored with EX's disclosure | Agent 0, 2026-10-31: exit criterion 1 needs modal on EX. EX shares PF's assembly and adds no load-dependent stiffness (§1.1) |
| **D-s** | Nonlinear supports | Linearized at the final state of a named load case, solved on the current model in the same run (a required input with no default): the constraint table of §1.3 (active one-way and lift-off, closed gaps and sticking friction rigid; inactive, open and sliding free, sliding with its friction force frozen as a disclosed approximation); the case and each state disclosed; an unconverged solve refused | Agent 0, 2026-10-31 (formerly M5): criterion 1's reference system has a gap, lift-off and friction. A named state is the user's statement, not a default. Review round 3 (findings 5, 15): the static solver represents these states as rigid constraints; sliding with a derived normal has a nonsymmetric tangent, so it is frozen and disclosed; re-solving binds the state to the model |
| **D-t** | Snubbers | In the modal basis when the request includes dynamic-only restraints, a required choice when the model has one; otherwise ignored; disclosed either way | Agent 0, 2026-10-31; SCA-014 (snubbers as dynamic-only restraints). A spectrum solve includes them |
| **D-u** | Verification of a transcribed published fixture (O-2) | An independent Type 2 verifies the transcription against the source and records it in a separate `transcription_verification` field, naming reviewer and source; `review_status` stays in the governing vocabulary (`pending` until the maintainer decides); never recorded as accepted; a fixture with unclear rights holds for the owner | Agent 0, 2026-10-31; review round 3 (finding 16) on the field vocabulary |
| **D-w** | Load/reference-state documents and the state case | Admitted: one required named load case (`state_case`) whose resolved state supplies stiffness (per-member modulus selection), mass (the state's self-weight inputs, with contents from the case's resolved contents density where it resolves one and from the section otherwise, disclosed) and nonlinear-support states; required on such documents and whenever nonlinear supports exist; a request modulus basis conflicts with it; boundary-motion DOFs restrained; stated in the result; an unresolvable or unconverged state refused; a two-basis mass conflict refused and taken to Agent 0 | Agent 0, 2026-10-31: criterion 1's reference system is a load/reference-state EX document. One named state is the user's statement, and the static path's resolver is reused |
| **D-v** | Participation excitation | Γ and m_eff on p_k = (M R_k)_f, consistent-mass coupling to restrained DOFs included; fractions relative to the total mass T_k | DEL-04-10's first independent review (gpt-6.1-sol, finding 1): with M_ff R_f alone, one fixed axial element gives u/A = m/(3k) instead of m/(2k). The response spectrum uses these factors unchanged |
| **D-q** | Published-benchmark obligation | At least one published piping benchmark with recorded redistribution evidence compared, after M1b, as comparison evidence, before FR-SOL-016 is reported complete; FR-SOL-016 stays partial until then (§4.4) | PRD §12.9 requires the comparison. Codex round 1; Agent 0, 2026-10-27 |

Agent 0 accepted D-c, D-d, D-m, D-n, D-o and D-p as Design decisions on 2026-10-27, subject to the next independent review round. Round 2 (at `d3c54e4f72`) revised D-c, D-d, D-n and D-o, and moved the count infrastructure from M2 into M1b (§6), under the manager's decisions; those revisions go to Agent 0 after the next review.

Items that reach the product or data boundary are marked **for Agent 0** in §9.

---

## 8. Risks

- **R2, inertia from unpivoted LDLᵀ.** `factorize_ldlt` does no pivoting (`sparse_direct/src/lib.rs:377-381`). On an indefinite K − σM, element growth, or a shift within rounding of an eigenvalue, can corrupt the negative-pivot count. The count is relied on only under the conditioning and separation premises (§3.4, check 4), from M1b on; mitigations and tests: §3.3. A shift that cannot be placed under the premises refuses the set; an ill-conditioned shifted matrix, or a formed shift's rounding, can corrupt the count even far from any eigenvalue (review rounds 3 and 4).
- **R3, mass discretization.** Too few nodes on long spans give wrong frequencies with clean residuals: the residuals check the discrete problem, not the physics. The warning (§2.6) and the M6 helper address it, and the limitation text says so.
- **R4, repeated eigenvalues.** Circular sections and symmetric layouts give repeated eigenvalues with non-unique bases. Block Lanczos recovers every direction, and the Sturm count checks it under its separation premise (§3.3, §3.4). Determinism holds (fixed start block and seed sequence, fixed order, one thread), but bases can change under small input changes. Clusters are completed at the boundary, published and compared as subspaces.
- **R5, two mass bases in the product today** (`self_weight.rs:731` against PP `lib.rs:10293`). A finding, recorded on the remaining-items list (§2.1). No action in F1.
- **R6, rigid-family scaling.** s·E up to and beyond the 1e5 precision-warning scale (`rigid_component.rs:17-24`) adds very stiff, high-frequency directions and raises K's condition number. The low modes stay the well-conditioned end under K-based reduction, but residuals and error estimates widen, and so does the backward-error estimate that the separation premise uses. The W-15 precision warning stays, and the references include a rigid component at s = 1e3…1e6.
- **R7, D3 and the reviewed lock.** A new crate dependency changes `Cargo.lock`, a reviewed input, so the profile is `Stale` until re-registered (§3.6).
- **R8, timing claims.** The dense path's cost is cubic and debug builds are much slower. Every timing statement names its build and runs under `t3_exclusive.sh`.
- **R9, stack.** Mass formation stays out of the frames on the W1 peak chain (§3.5). The AC-004 witnesses are re-run if D3 moves anything.
- **R10, public benchmarks' lineage.** NUREG/CR modal results are numerical, from other programs, so a match is comparison evidence, not an exact check.
- **R12, linearized state.** A modal result with nonlinear supports holds only at the named case's support states; a different operating state gives different frequencies. The result names the case and each state, and the limitation text says so. **As built (I2b-2):** the outcome names the case, the mode, each support's state and treatment, and its disclosure states that the result holds only at those states.
- **R11, benchmark rights.** A contractor-authored report may carry rights that prevent transcription. Then no published benchmark may be available, and FR-SOL-016 stays partial (§4.4).

---

## 9. Remaining open items

| # | Item | Owner | When |
|---|---|---|---|
| O-1 | Independent Codex review of the W/a basis (§2.3), as Agent 0 directed, and of this design. Round 1 (at `a640e10e50`) and round 2 (at `d3c54e4f72`) are answered in this revision; the next round is pending | Manager, then Agent 0 | Before M1a starts |
| O-2 | **Settled (D-u, Agent 0, 2026-10-31):** an independent Type 2 verifies a transcription against its source and records agent verification, naming reviewer and source; never acceptance. Unclear rights hold for the owner | — | — |
| O-3 | **For the owner (data boundary):** the review of each report held for uncertain rights (§4.4; `docs/IP_AND_DATA_BOUNDARY.md`). Nothing is transcribed from a held report | Owner | At acquisition of each report |
| O-4 | The work budget of the dense guard (§3.2), set by measurement. **Status (I2b-1): unmeasured and unpinned.** No work guard is applied; the operation estimate is recorded as a formula only, and the memory side uses the static guard (§3.2, as built). Measurement needs a separate, bounded, exclusive authorization. **I3a:** the 60-minute window proposal was not approved and is withdrawn; §5.3.8 holds the stage-1 plan and the maintained, ignored phase measurand. **Stage 1 ran once, 2026-11-04** (`window-20261104T120707Z`, binary `f7c87223…` at `48b3cc692`): 10 of 10 processes valid on the normal path, release, n_f = 60–960 at `mode_count` 10, all values TEST-LOCAL; analysis in §5.3.8, "Stage 1 results". **Still unpinned; no guard adopted.** Proposed for Agent 0 there: an observed-scope work guard (n_f ≤ 960 and n_r ≤ 10, within the restricted single-straight-run family only) or none yet; one more window at n_f = 1,920 (both passes, d = 120 s), which extends only that family's extent; 3,072 (entry only) only if the ~3,000-DOF target is kept and the slower 1,920 process takes ≤ 45 s (8× rule), or if Agent 0 amends the rule; and the projection guard's structure, from a *proposed* format bound with named open items (limits with I3b). The memory constant waits for the 1,920 peak. The observed 960 peak (150.4 MiB) lies between the source's largest live set (95.0) and its no-reuse cumulative (194.2, both with baseline). **Agent 0, 2026-11-04:** no production work guard comes from one family. The ~3,000-DOF goal holds across adopted families, priced from source plus calibration, with a combined preflight that refuses before the state-case solve. 96·n_g² is a live formula only. B_doc stays proposed. The projection-guard structure is approved without limits. **The n_f = 1,920 calibration ran once** (`window-20261104T131510Z-nf1920`): both processes valid, 55.3 s, 542/589 MiB. Band recovery and the census became active (640 pairs screened, census Unresolved under the test-local τ_μ), so dense_solve grew with slope 4.3. 3,072 is not approved. The ten low modes there are reportable only under the accepted lower-gap count and completed-cluster premises; the unresolved finite census neither invalidates that path nor becomes whole-spectrum proof. **Proposed (not adopted):** §5.3.8's source resource guard, a term-by-term cost model with a preflight before the state-case solve and exact stage checks, and the remaining production pin choices | M1b | M1b |
| O-5 | The discretization factor c (§2.6), derived by benchmark. **Status (I2b-1): not adopted.** f_e is computed per straight member and reported with f_e/f_max; no `MODAL_MASS_DISCRETIZATION_COARSE` decision is made. Test-local observations on the SEC-A Rayleigh cantilever (L = 6 m, N = 1, 2, 4, 8) through the product path, f_e/f_n against the frequency error from the continuum: 0.64 → 0.58, 1.2 → 0.22, 2.9 → 1.4·10⁻², 4.0 → 8.5·10⁻³, 5.8 → 7.7·10⁻³, 11.9 → 2.2·10⁻³, 16 → 1.2·10⁻³, 23 → 6.1·10⁻⁴; these match the frozen `fe_convergence.json` errors to 10⁻⁵ relative | M1a | M1a |
| O-6 | **Measured basis so far (I1, test-local constants, not yet pinned):** over 352 well-separated published shifts of `f1_modal` whose raw counts were all right, e_b·ν ranged 1.8·10⁻¹⁵ to 2.1·10⁻⁴ and the separation ratio 5.7·10³ to 5.6·10¹⁴; over 34 factored binary64 miscounts, e_b·ν ≥ 35.8 and the separation ratio ≤ 6.2·10⁻³. η ≤ 1.44·10⁻¹³ on every dense mode of 11 class (a) cases, actual eigenvalue error ≤ 0.85·e_λ there. The integrity tolerances, the residual threshold τ_r and the orthogonality tolerance (§3.4) and the identity tolerance (§2.5): measured, then pinned under DEC-026; the identity tolerance was measured in I2a (§2.5, largest relative discrepancy 5.45·10⁻¹⁶ nodal, 4.38·10⁻¹⁶ resultant) and stays experimental and unpinned (D-TOL). I2b-1 adds the product-path observations of §3.4 (as built), all test-local, nothing pinned. The algorithm parameters, recorded with their basis: the confirmation of τ_t (§1.5); δ_c, the growth-factor formula g, the growth limit g_max, the conditioning multiple κ_c and the estimator's iteration count, the separation multiple κ_s, the half-arc series switch point (§2.4), the candidate-shift sequence and s_max (§3.4, M1b); the cut-off sizing offsets ε_s (§1.4), the block size limits, restart count and breakdown threshold (§3.3, M2). **I3a:** production-pin proposals with their derivations and margins are §5.3.7. **Agent 0, 2026-11-04:** A1 approved as numerical acceptance and algorithm criteria, not a domain-wide error theorem (τ_t, the identity tolerance, g_max, τ_e, τ_m, s_max, the gap positions, the offsets and the QL limit); A2 decided in part on 2026-11-04 (κ_c, κ_s and τ_o adopted as engineering acceptance and demotion criteria, δ_c as the computed-cluster grouping convention; τ_r = 2⁻³⁰ adopted later that day as a measured-residual demotion criterion); A3 decided 2026-11-05 (τ_μ = 2⁻⁵³, c_μ = 1, κ_μ = 10, c_f = 64, as explicit retention and screening criteria). With all three closed, the production instance may be built as a bounded explicit constructor; it pins nothing until reviewed. The targeted A2 evidence and the A3 μ/μ_max table are in §5.3.7; nothing is pinned | M1a, M1b, M2 | As each lands |
| O-7 | Where the desktop keeps the remembered extraction bound per project (project store or desktop settings), and whether that needs a store version under the one-version rule. It is persistence, so it takes targeted checks | M3 | M3 design |
| O-8 | **Superseded proposal; open policy (I3a, Agent 0's I3 release).** The earlier plan added `product_preview_modal_v1` to `results.v0.3` and a `modal` run kind to `analysis_run.v0.3`. Neither is implemented or adopted: the modal result is its own family (§5.3), and `results.v0.3`, `analysis_run.v0.3` and the mechanics envelope are unchanged. Open, and unadopted: the joint task record, a store, the version policy for a stored modal result and recording support. A result-family-only design proceeds without them; no exact collision with an adopted policy was found | Agent 0, with DEL-00-06 | Before any recording or store work |
| O-9 | A concentrated-mass field and component rotary inertia (W-17), entering self-weight and modal together | W-17 lane | After F1's core |
| O-10 | M4 (Timoshenko mass, on either route), M6 (mass-point helper); M5 moved into M1b (D-s) | Later slices | After F1's core |
| O-11 | One mass basis: PF seismic generation and self-weight (finding R5), with the deferred PF seismic-mass alignment to D-5 | Remaining items (`docs/COMPLETENESS_MAP.md` §4.3) | Not in F1 |
| O-12 | The unit-catalog families of §1.5 (frequency, angular frequency, rotation per length, length per rotation, rotational generalized mass in kg·m²·rad⁻²; carried by model document 0.9.0's S1), with their D3 re-registration. **Landed** with 0.9.0's S1. **I3a:** `mass_length_per_rotation` (kg·m·rad⁻¹) is added for the translation–rotation mass entries, and λ is not published (Agent 0, 2026-11-04; D-01 engineering extension 2026-11-04). The equilibrated-basis magnitudes are traced and assigned, and the closed list is empty (§5.3.5). `time_squared` (s²) is added for the two D·M·D magnitudes (option (a)) | DEL-02-02 | Before public admission |
| O-13 | **A later option: rigorous eigenvalue enclosures.** The checks of §3.4 are estimates under premises. Enclosures would need interval or exact arithmetic: interval-verified inertia counts (or exact rational LDLᵀ for small models), and then Kato–Temple or Lehmann bounds across a gap that those counts establish, on a pencil whose formation is itself enclosed. Kato–Temple across gaps between *computed* eigenvalues proves nothing if one is missed (K = I, M = diag(1, 2, 3): with μ = 2 missed, the computed vector (cos t, sin t, 0) has a Kato–Temple bound of sin²t·cos²t / (2 − sin²t) against a true error of sin²t). Codex's round-2 witnesses show that the binary64 versions of these steps do not give enclosures. Any such option is a new commitment, outside F1's | Later change | Not in F1 |
| O-14 | **I2b-2's stated limits.** (a) Gap supports: every model with one is refused, since the static path qualifies and confirms no gap state; required for criterion 1 and owned by **RT-A3 R-3** (§1.3). (b) **Closed** (Agent 0, 2026-11-02): friction inactive (its normal does not bear) is free, as the static path treats it (`prescribed_displacement_for_state`), with its normal support following its own state; §1.3's table has its row, admission is unchanged, and a solved, confirmed case matches its linear twin. Only an explicit normal that is not positive confirms; a derived normal that does not bear stays `NotEstablished` in the static confirmation (`confirmation.rs:870`) and is refused. (c) A load/reference-state document with a nonlinear support waits on **RT-A3 S6** (nonlinear supports on load/reference-state documents, with connectors) | RT-A3 for (a) (R-3) and (c) (S6) | (a), (c) when R-3 and S6 land |
