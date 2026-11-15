---
name: swbpipe-theory-modal-analysis
description: Explain the modal method, its integrity checks and its current experimental standing.
metadata:
  kind: explanation
  surface: reference
---

# Modal Analysis

## Purpose and boundary

This note explains how SWBPIPE computes the natural frequencies and mode shapes
of a piping model, with the mass each mode carries. It covers the inputs as the
method uses them, the method, its assumptions and its limitations. The method
is defined by DEL-04-09 (`execution/PKG-04/DEL-04-09/ScopeOfWork.md`); where
this note and that scope differ, the scope stands and this note is the one to
correct.

**Standing.** A modal result is **experimental**. Every modal document states
`admission.status: experimental` and `public_engineering_result: false`. The
method's thresholds and parameters have selected values, which the document
states as its parameter standing, separately from that result standing. What
is still missing before a modal result can be relied on as an engineering
result is in [Limitations](#limitations).

**Numerical integrity does not establish physical correctness.** The checks
described below concern the discrete eigenproblem the method forms. They do not
bound modelling, discretization, input or physical error, and none of them is a
mathematical bound on an eigenvalue.

SWBPIPE does not determine code compliance. A frequency, a mode shape or a
participation factor is a mechanics result; whether a line is acceptable is the
user's engineer's judgement, under their own responsibility.

## Inputs

Every engineering input is the user's. No input below is defaulted by the
engine, the headless runner or any other entry.

| Input | What it means to the method |
|---|---|
| The model and its materials | The linear model whose free degrees of freedom form the eigenproblem, on its own route: pressure-free (PF) or the exact pressure route (EX). |
| Extraction bound | A mode count n, a frequency cut-off f_c, or both. At least one is required. The reported set is the lowest modes up to whichever bound stops first; the cut-off is inclusive. |
| State case | Required when the document is a load/reference-state document or the model has a nonlinear support. It names the load case whose resolved, solved state defines the modal model: its moduli, its mass inputs (including a test contents density), its nonlinear-support states and its restrained DOFs. |
| Modulus basis | Without a state case, the modulus comes from a load case's vocabulary through the existing resolver. With a state case, a request modulus basis is refused as a conflict. |
| Weight-to-mass acceleration a | Required when a component's weight enters the mass: a component body's mass is W/a. When the model carries an applied component-weight primitive, its recorded acceleration must equal a exactly. |
| Dynamic-only restraint choice | `include_dynamic_only_restraints` is recorded with the result. No support family the product admits today uses it. |

## Method

### The eigenproblem

The method solves the generalized eigenproblem

K_ff φ = ω² M_ff φ

on the free degrees of freedom of the linear model.

- **K_ff** is the restraint-reduced stiffness of the route's static solve,
  without load-dependent terms. The exact route adds no pressure stiffening:
  pressure enters it only as loads, which the modal solve ignores.
- **M_ff** is the restraint-reduced mass of the mass model below.
- Nonlinear supports enter at their final state in the named state case, solved
  in the same run: an active one-way or lift-off support, a closed gap or
  sticking friction rigidly restrains its DOF; an inactive or open support
  leaves it free; sliding friction leaves its tangential DOF free with no
  stiffness, which is disclosed as a modal approximation. A state is used only
  when the static solve confirmed it.

For each reported mode the result gives the frequency f in Hz, the circular
frequency ω = 2πf in rad/s and the period T = 1/f in s; the shape; the
generalized mass m = φᵀM_ffφ; and, in each global direction k, the
participation factor Γ_k = φᵀp_k / m, the effective mass (φᵀp_k)²/m, the
effective-mass fraction and its cumulative sum. Here p_k = (M R_k)_f, the free
rows of the full mass times the rigid unit translation R_k of every DOF, so
that consistent-mass coupling to restrained DOFs is included. Fractions are
relative to T_k = R_kᵀ M R_k, the model's total mass in direction k.

### The mass model

The mass is formed on the same basis as the model's self-weight, so that the
two cannot disagree:

- **Straight members** carry the consistent mass of Euler–Bernoulli beam theory
  (the Hermitian consistent mass with Rayleigh rotary inertia), with linear
  axial and torsional mass. Their mass per length is exactly the self-weight
  generator's on the model's route: on PF, metal and contents on the wall
  t − c_m; on EX, metal on the nominal wall and contents on the pressure bore;
  insulation alike on both.
- **Contents** are in all three translations and excluded from torsional and
  rotary inertia; this is disclosed.
- **Realized arcs** carry two rigid-body half-arc lumps with the half-arcs'
  exact mass, centroid and second moments.
- **Rigid-family components** that self-weight applies carry a rigid-body lump
  of mass W/a at their centre of gravity, in place of the member's pipe mass.
- **Massless families:** objective connectors (their replaced span carries
  neither self-weight nor mass), tie rods and constant-effort supports carry no
  mass; this is disclosed.
- **Restrained DOFs:** mass at restrained DOFs is excluded from M_ff and
  disclosed with the total model mass.

**Weight-basis identity.** Every result runs a blocking check that a·M·r_k,
for each global direction k, equals the self-weight generator's loads at
gravity a along k over the mass-bearing members. It holds node by node for
straight members against the consistent uniform-load vector, and on the
resultant force and moment for arcs and components, within 2⁻⁴⁴ relative. A
failure refuses the request.

### The dense path

The product's modal entry uses the dense path. It reduces the pencil with K's
Cholesky factor, tridiagonalizes and applies implicit QL. It uses no new
numerical dependency.

### Clusters, completeness and the Sturm count

- A **cluster** is a maximal run of adjacent computed eigenvalues, each within
  δ_c = 2⁻²⁰ relative of the next. When the extraction bound falls inside a
  cluster, the reported set is extended to the whole cluster, and the result
  discloses the extension by count. A partial cluster is never published.
- **Completeness** is checked by a Sturm count: the number of negative pivots
  of the LDLᵀ factorization of the scaled K − σM, at a shift σ placed in the
  gap between the last reported cluster and the next computed one, equals the
  number of eigenvalues below σ (Sylvester's law of inertia). That count must
  equal the number of modes reported. A mismatch refuses the set.
- **A count is relied on only under two premises**, both disclosed with the
  shift: the estimated backward error of forming and factoring the shifted
  matrix, times an estimate of the norm of its inverse, is at most 2⁻¹⁰ (the
  conditioning premise); and the shift's separation from the neighbouring
  computed eigenvalues exceeds 2⁷ times that backward error (the separation
  premise). A factorization whose growth exceeds 2⁵² is rejected. Where no
  shift meets these premises, the set is refused by name.
- **Whole spectrum.** When the reported set is the whole finite spectrum, the
  final shift lies above its largest eigenvalue. The dense path claims this
  only when the number of finite modes is established independently;
  otherwise the request is refused (`MODAL_FINITE_COUNT_UNRESOLVED`).

### Integrity checks

Every published mode set carries:

1. a **formation check**: K and M re-formed independently from their authored
   operands, element by element, in double-double precision (about a 106-bit
   significand), with the formation difference measured and published;
2. per mode, the **residual** ‖Kφ − λMφ‖ against the re-formed pencil, scaled
   as a backward error and compared with τ_r = 2⁻³⁰, with a first-order,
   residual-based estimate of the eigenvalue error (stated for a cluster as a
   whole; an estimate, not a bound);
3. the **mass-orthogonality** deviation of the published modes, against 2⁻²³;
4. the **Sturm-count** agreement above;
5. the **cluster** identifiers and any extension;
6. the **weight-basis identity**.

A mode whose residual exceeds τ_r, or an orthogonality breach, publishes the
result as **Sensitive** when nothing refuses the set. Each threshold is an
acceptance, demotion or grouping criterion, not an error or count theorem.

### Normalization: a display convention

Shapes are in global axes, and normalization fixes only their scale and sign:

- **(a) A mode with translation** is scaled so that its largest translational
  component is +1. Translations are then dimensionless and rotations in rad/m;
  the generalized mass is in kg.
- **(b) A mode with no translation**, such as pure torsion, is scaled so that
  its largest rotational component is +1 (rad/rad). Its translations are the
  computed vector's own, in m/rad, and its generalized mass is in kg·m²·rad⁻².
  A mode has no translation when its largest translation is at most
  τ_t·ℓ = 10⁻⁶·ℓ times its largest rotation, with ℓ the diagonal of the
  bounding box of the model's nodes.

Normalization is a convention for display and comparison, not a physical
amplitude: a mode shape has no physical size until a response analysis scales
it. Frequencies, effective masses and fractions do not depend on it; Γ and the
generalized mass do, consistently, so that Γ²·m equals the effective mass in
either case. Each mode records its case.

### Readers

A published document can be checked by the product's Rust modal reader
(`core/reporting/modal_result`), which shares no code with the producer, and,
in the repository's tests, by an independent Python verifier. Neither re-runs
the eigen-solve. Given the bound request, the reader re-derives the model's
mass and restraint basis within its coverage and recomputes the mass-dependent
quantities from that mass and the published shapes; it checks the counts,
ordering, clusters, Sturm placement and premises, the Sensitive standing and
every value's unit. Its verdict states each relation as passed, failed,
unresolved, scoped unsupported or not checked, and states that public
engineering-result admission is disabled. A consistent verdict shows that the
published relations agree with each other and, with the request, with the
model's mass; it does not show that the eigenvalues are correct.

## Assumptions

- Small displacements, linear elasticity, and free vibration about the
  unloaded configuration of the linear model, with nonlinear supports
  linearized at the named state.
- No damping: the frequencies are undamped natural frequencies.
- Euler–Bernoulli straight members (Timoshenko members are refused; see
  [Limitations](#limitations)).
- No pressure stiffening on either route; thermal, pressure, wind, seismic,
  nodal and member loads are ignored by the eigenproblem and disclosed.
- The mass is the self-weight mass only. There is no separate seismic mass
  basis and no concentrated-mass field.

## Limitations

- **Experimental, not an engineering result.** DEL-04-09's Admission
  conditions are not met: the dense path's work guard has no measured budget,
  and the resource preflight refuses every budgeted request while the
  integrity and confirmation work it charges is unbounded. The headless entry
  runs without a budget. It applies the dense memory guard, the request and
  model refusals and the integrity checks, and it publishes an experimental
  document.
- **Dense path only.** The product's entry solves on the dense path, under the
  dense memory guard; a model too large for that guard is refused. The sparse
  path is not a product entry.
- **Timoshenko straight members are refused** on either route
  (`MODAL_TIMOSHENKO_MASS_UNSUPPORTED`) while the Timoshenko mass is not yet
  formed by the product's mass assembly, the integrity re-formation and the
  readers. Exact-route authoring writes Timoshenko members by default, so a
  model authored with that default is refused (`docs/RELEASE_LIMITATIONS_v1.0.md`
  LIM-020).
- **Realized bends** enter the stiffness with a thin curved-beam element whose
  application range SWBPIPE's verification has not established; a modal result
  of a model with realized bends inherits that limitation
  (`docs/RELEASE_LIMITATIONS_v1.0.md` LIM-021).
- **Skewed restraints** are refused by the modal document
  (`MODAL_DOCUMENT_SKEW_RESTRAINT_UNSUPPORTED`): the document states a global
  mass and global shapes only.
- **An unrestrained model or a mechanism** is refused through the existing
  singularity refusal; a model with no mass on its free DOFs has no finite
  mode and is refused (`MODAL_NO_FINITE_MODES`).
- **Not recorded.** A modal result is not an analysis run: it is not saved
  with a project and is run again when needed. It is not part of the static
  result envelope, report or export.
- **Normalized shapes** are not physical amplitudes (see
  [Normalization](#normalization-a-display-convention)).

## Evidence

The frozen references under `validation/references/f1_modal/` (mass, discrete
and continuum references) and `validation/references/modal_timoshenko_mass/`
were each derived by a generator that imports no product code. The product's
modal tests compare against them, and the readers are held to the committed
reader corpus (`fixtures/results/modal_corpus.json`). Which acceptance
criteria of DEL-04-09 these cover, and which remain open, is the scope's
account.

## Source notes

- Commitments: `execution/PKG-04/DEL-04-09/ScopeOfWork.md`.
- Implementation: `core/product_physics/src/modal.rs` and `modal/` (request,
  mass, weight basis, dense path, integrity, document);
  `core/solver/modal_eigen` (the eigen kernel and Sturm count);
  `core/reporting/modal_result` (the Rust reader).
- The generalized eigenproblem, consistent beam mass, participation and
  effective mass, and the inertia count of an LDLᵀ factorization are classical
  structural dynamics and linear algebra, stated here without external source.
  The independent references derive them in closed form or by exact
  arithmetic.
