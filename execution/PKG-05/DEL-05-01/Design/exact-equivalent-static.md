# Equivalent-static generation on the exact route

This design admits a load case's `equivalent_static` generation on the exact
pressure route (EX, `3.0.0/exact_pressure_v3`), for PRD §12.5 ("seismic
static-equivalent load", "wind load") under DEC-068. Today EX refuses it by
name (PP `pressure_runtime.rs` ~366-372; `ExactFamily::EquivalentStatic`),
while the pressure-free route (PF) generates it
(`append_equivalent_static_generated_loads`, PP `lib.rs` ~10192). It is W-16
slice c in `docs/COMPLETENESS_MAP.md` §4.3. It ships in the same
implementation as slice b (DEL-05-02 `Design/exact-combinations.md`), so the
exact-route identity sentence changes once.

Paths are relative to `projects/chirality-piping/`. PP is
`core/product_physics/src`.

## 1. Inputs and generation

The input is the load case's existing closed `equivalent_static` record
(#1573), unchanged. Generation follows DEC-068, as on PF, from user inputs only:
- **Seismic.** The user's g-factor per axis, times gravity, times each member's
  mass per length. The result is distributed force on the member.
- **Wind.** The user's pressure, shape factor and global direction, on the
  user's marked spans and fractions. The load acts over the projected
  diameter OD + 2·t_ins.

Every PF validation and refusal code for `equivalent_static` applies unchanged
on EX, including `EQUIVALENT_STATIC_INPUT_MISSING` and the insulation-pair
rule. No code value, coefficient or default is supplied.

## 2. Mass basis on EX

EX uses its own D-5 mass basis: pipe metal on the nominal wall, and contents on
the pressure bore (DEL-05-03 `section-bases-and-timoshenko.md` ~22-24, ~40).
This is the basis EX self-weight already uses (`self_weight.rs` ~420-590,
`EXACT_MASS_METHOD` = `source_od_nominal_wall_pressure_bore_areas/v1`), and
seismic generation on EX calls the same function. Insulation mass is computed
as on PF, over OD to OD + 2·t_ins.

PF keeps its current generator mass (`compute_pipe_mass_per_length`, PP
`lib.rs` ~10045: the wall less mill tolerance for both metal and bore). A model
solved on both routes can therefore carry different seismic loads. This is a
property of the two routes' section bases, not a defect. Each generated
primitive's evidence names its `mass_method`. Aligning PF to D-5 is
deferred (Agent 0, 2026-10-27, under the owner's delegation in #1662): it
would move PF results and goldens, and the difference between the routes is
documented by design. It is revisited if the CAEPIPE comparisons show that
the PF basis is the less accurate one.

Wind does not depend on mass, and is identical on both routes.

**The certified producer.** The certified load producer must form the same
mass as the generator. Today it builds mass with `corrosion_allowance: None`
(PP `certified_loads.rs` ~198-213) and uses it in its seismic branch
(~246-254). The frame kernel then re-forms PF mass
(`structural/formation_check.rs` ~1783-1849).

On EX, the producer instead selects the section's `Some(c_a)`, including an
authored zero. It keeps `None` on PF. Tests check:
- the raw-source re-formation;
- its binding to the applied loads;
- that an authored load id resembling the generated prefix is not mistaken
  for a generated one (the producer dispatches by prefix, ~233-259).

As built, the producer dispatches through `is_generated_equivalent_static_load`
(PP `lib.rs`): a load is generated when its case has `equivalent_static`, its
id carries `<case>:generated:<kind>:`, and no authored primitive of the case
has that id. An exact collision between an authored id and a generated id is
refused on both routes, `EQUIVALENT_STATIC_GENERATED_ID_COLLISION`, so the test
is exact. The S11-G operand-bound flag in `add_uniform_element_loads` keeps its
prefix test; for a lookalike it only widens the bound.

**Evidence.** For a case with generated loads, `exact_cases[]` gains a closed,
optional `equivalent_static` record. It holds the mass method
(`EXACT_MASS_METHOD`) and the ids of the generated primitives. Cases without
generation omit it, so existing bytes hold. The pressure-2 schema and the
readers bind it.

As built, the record is `{"mass_method": "source_od_nominal_wall_pressure_bore_areas/v1",
"generated_load_ids": [...]}`, the ids in generation order
(`exact_equivalent_static_record`, PP `lib.rs`). It is present exactly when the
case authors `equivalent_static`; wind-only cases carry the same mass method.
The pressure-2 schema defines it as `PressureExactEquivalentStatic`; the source
schema carries the same closed shape as `pressure_source_equivalent_static`, so
its one-resource shape check needs no external reference. The Rust reader
(`result_export` `physics_evidence::equivalent_static_record`) and its Python
mirrors (`tests/verifiers/pressure_2.py`, `physics_evidence.py`,
`physics_source.py`) require the closed keys, the exact mass method, and at
least one id, each once and each in `<case>:generated:`. A selected
retained-source case may not carry the record (`SOURCE_EQUIVALENT_STATIC`).

A source-recovery envelope can also carry the record. Another case in the
document may select source recovery, and the whole envelope then becomes
pressure-source-1 (PP `lib.rs` ~3063, ~3099, ~3198). That envelope still
carries the generated case as an ordinary companion. The record is therefore
also covered by:
- the companion's captured closed-key binding (`source_receipt/composite.rs`
  ~508);
- the closed source schema (`schemas/physics_source_recovery.schema.json`
  ~1835);
- the transport shape check (`result_export/src/physics_source.rs` ~185);
- the Python mirrors, carriers, pins and D3 registration.

Generated cases stay excluded from selected retained-source recovery
(`source_recovery.rs` ~903). A mixed regression checks a selected nodal
source case together with a passing generated ordinary case. As built, the
companion binding re-forms the record from the captured model's own
generation (`source_receipt/composite.rs`). A Sensitive generated case would
itself attempt recovery, be declined and withhold the join, as any declined
case does, so the regression's generated case passes its checks.

The shared mass calculation keeps the equivalent-static diagnostic codes. As
built, EX first runs PF's input checks with their codes, then takes the D-5
mass (`self_weight::exact_generation_mass_per_length`); a failure there is
reported as `EQUIVALENT_STATIC_INPUT_INVALID` with its message.

## 3. Which documents

**Plain v3 documents** (without the load/reference-state group) admit
generation. The generated primitives are ordinary distributed loads of the
case, prepared like any other (PP `lib.rs` ~10035, ~4867, ~4292-4322). They
are mechanical loads: they create no pressure regions and do not change
closure ownership. Realized arcs take full-span loads per unit arc length, as
on PF. On arcs, any explicit extent stays blocked, including [0, 1]
(~11532-11557). Whole-arc wind is marked through `exposed_pipe_refs`.

**Connector-replaced spans** carry no load, member result or self-weight
(`joint.rs` ~164). The design keeps them out of generation in two ways:
- Seismic generation skips a replaced span before its mass is evaluated. The
  surviving pipes keep their original `element_index`; they are not renumbered.
- A wind marking on a replaced span, in `exposed_pipe_refs` or
  `exposed_spans`, is refused by name with
  `EQUIVALENT_STATIC_WIND_SPAN_REPLACED`.
- The filter applies to generated loads only. Stored primitives keep
  `JOINT_REPLACED_SPAN_LOAD_UNOWNED` (`joint.rs` ~845), with its existing
  presence and exclusion rules.

Loads are never moved implicitly to connector nodes. Regressions cover both
paths. As built, both rules are in the shared generator, through
`joint::replaced_span_ids` (empty off the exact route).

**Timoshenko and partial-span wind.** On an EX document selecting
`timoshenko_annulus_energy_v1`, a wind marking of a proper fraction of a
straight span is admitted, as on Euler–Bernoulli. "Proper" means
`!(start == 0.0 && end == 1.0)` after ordinary fraction validation, the
predicate `straight_pipe` and the frame kernel use. Exactly [0, 1] is full
span, whether given through `exposed_pipe_refs` or `exposed_spans`, and keeps
the Euler–Bernoulli vector, which a Timoshenko member shares on a full span.
Any near-full extent counts as partial. A proper fraction takes the exact
Timoshenko fixed-end vector in assembly and recovery: for w on [aL, bL],
feq_T = feq_EB + ΔV₀·[1, L/2, −1, L/2] with
ΔV₀ = φ/(1+φ)·(wL/2)·[a²(1−a)² − b²(1−b)²] (DEL-05-03
`section-bases-and-timoshenko.md` §3.4). Its two formation checks are those of
DEL-05-03 §4 item 4: S11-G records each slot as `Bounded` with an exact
a-posteriori bound, and K-D5 re-forms the vector as a ball from the frame's own
leaves. A failure of either is fail closed, so every case keeps its formation
checks. The former refusal
`EXACT_EQUIVALENT_STATIC_PARTIAL_SPAN_TIMOSHENKO_UNSUPPORTED` is retired and
not reused. A realized arc is not a straight span: any extent on it keeps the
arc's own refusal (`LOAD_INPUT_INVALID`) under either shear model.

**Load/reference-state documents** refuse a case's `equivalent_static` by name,
with `EXACT_EQUIVALENT_STATIC_LOAD_STATE_UNSUPPORTED`. The document-level
detector (PP `case_state/mod.rs` ~30-40) decides which documents these are,
including an authored null group. The scan covers every authored case
before resolution. That includes cases no combination uses and cases with no
state record. Without the refusal, effective-case construction would keep
`equivalent_static` and apply loads outside the ledger (`resolve.rs`
~1090-1105). The load-state contract
states that a case's loads are "exactly the case's declared source ledger" and
that "per-case mass selection [is] not provided" (PP `exact_admission.rs`
~107). Generated loads have no ledger class, so admitting them there needs its
own design. The refusal is checked before the effective cases are selected.
As built, it is emitted with the profile checks (PP `pressure_runtime.rs`
`validate_profile`), through `case_state::carries_fields`.

**The retained sub-route** keeps refusing `equivalent_static` on every branch
(`retained_memory.rs` ~942-944). **Source recovery** keeps declining
equivalent-static and non-nodal producers (`source_recovery.rs` ~903-912).

**Combinations.** A generated case combines like any other primitive case
under slice b.

## 4. Text that changes

- **The identity sentence** (PP `exact_admission.rs` ~83-85) loses
  "equivalent-static" together with slice b's "combination". It is one
  revision of the sentence, after W-15's (#1604). Its ~98 carriers are
  regenerated by their writers. As built, slice c merged first without it;
  the sentence changed with slice b, for both slices (DEL-05-02
  `Design/exact-combinations.md` §7 quotes the old and new text).
- **The load-state limitation** (PP `exact_admission.rs` ~106-107) stays, and
  gains the statement that generated equivalent-static loads are refused on
  load-state documents.
- **DEL-04-07 `pressure-contract.md`** ~7.
- **Completeness map:** the §12.5 row.
- **DEL-05-03 `section-bases-and-timoshenko.md`** ~161, the statement that
  partial-span Timoshenko is unreachable. It now states the partial-span
  formation and its checks (§3). The identity quotations at ~89 and ~93 are
  reconciled with W-15.
- **DEL-05-01 ScopeOfWork.** Its requirements and acceptance criteria name no
  route, so REQ-05-01-008 and AC-001 do not change. Two evidence descriptions
  are repaired through the owner's wording grant:
  - **:204.** OLD: "Unit tests cover wind/seismic/occasional explicit
    equivalent mechanics loads, required equivalent-static basis/provenance
    refs, and rejection of acceleration/dynamic-placeholder dimensions."
    NEW: "Unit tests cover wind/seismic/occasional explicit equivalent
    mechanics loads, required equivalent-static basis/provenance refs, DEC-068
    seismic/wind generation from explicit user inputs on PF and eligible EX
    documents, route-specific seismic mass, EX generated-primitive evidence
    binding, the named unsupported boundaries in
    Design/exact-equivalent-static.md, and rejection of
    acceleration/dynamic-placeholder dimensions."
  - **:284.** OLD: "Documents limit wind, seismic, and occasional
    equivalent-static handling to explicit mechanics loads with
    caller-supplied basis/provenance refs and no dynamic procedure
    generation." NEW: "Documents describe wind, seismic, and occasional
    equivalent-static handling as explicit mechanics loads with
    caller-supplied basis/provenance refs, including DEC-068 seismic/wind
    generation from explicit user inputs and the model's mass/geometry under
    the documented route-specific mass bases and named unsupported
    boundaries; no dynamic procedure generation is provided."
  - Both NEW texts are the reviewer's own. They go through the wording grant
    when the slice is implemented.
- **Byte identity.** b and c ship together, so documents with combinations
  change as well.

## 5. Verification

Independent references go in
`validation/references/w16_exact_equivalent_static/`. They come from an
exact-rational generator that runs no product code.
- **Seismic.** An invented EX cantilever with a corrosion allowance and mill
  tolerance, contents and insulation, under a lateral g-factor. The
  definitions are t_r = (t − c_m) − c_a, r_n = R − t and r_p = R − t_r. Then:
  - mass per length:
    μ = π[ρ_m(R² − r_n²) + ρ_c·r_p² + ρ_i((R + t_i)² − R²)];
  - load: q = μ·k_g·g;
  - nominal second moment: I_n = π(R⁴ − r_n⁴)/4;
  - tip: u_L = qL⁴/(8EI_n) and θ_L = qL³/(6EI_n);
  - reactions: qL and qL²/2;
  - section actions: V = q(L − x) and M = q(L − x)²/2;
  - bending stress: one set of actions, stressed on both bases, M/Z_n and
    M/Z_r.

  Timoshenko adds qL²/(2GA_s) to the tip. A PF twin, authored without a
  corrosion allowance, shows the documented difference:
  μ_EX − μ_PF = π[ρ_m·c_m(D − 2t + c_m) + ρ_c{2c_a(R − t + c_m) + c_a²}].
- **Wind.** The load is q_w = p·C·(D + 2t_i) on a·L to b·L, where a and b are
  fractions. Then:
  - |R| = q_w·L·(b − a);
  - |M₀| = q_w·L²·(b² − a²)/2;
  - u_L = q_w·L⁴·[(b³ − a³) − (b⁴ − a⁴)/4]/(6EI_n).

  Timoshenko adds q_w·L²·(b² − a²)/(2GA_s) to u_L, which is q_w·L²/(2GA_s)
  on a full span; θ_L is unchanged. The fractions bound the
  interval; they do not multiply the intensity a second time. The witness
  uses L ≠ 1 so that a missing length factor shows. The generated
  loads must be equal on PF and EX.
- **Coverage.**
  - Both solver modes, Euler–Bernoulli and Timoshenko.
  - Full-span and partial-span wind on Timoshenko, at either end and in the
    interior, on cantilevers, a subdivided cantilever, a fixed-fixed run and a
    skew member, against `validation/references/w16_timoshenko_partial_wind/`.
  - Arcs: full spans, with partial spans blocked.
  - Replaced spans, for seismic and for wind (the latter refused).
  - Shared sections, non-SI inputs, and authored and generated loads mixed.
  - Certified-source controls, including the prefix lookalike and an exact
    collision between an authored id and a generated id.
  - A mixed source-recovery envelope (§2).
  - W-15 body rows: the scale multiplies stiffness, not mass.
- **Combination.** A combination of a seismic case with a pressure case,
  through slice b.
- **Negatives.** Every PF code on EX, the load-state refusal (including a case
  not listed in the group), and the retained refusal.
- **Byte identity.** Every EX document without `equivalent_static` or
  combinations is unchanged, except for the identity sentence. Documents with
  combinations follow slice b.

## 6. Retained-memory pricing (D3)

The path touches priced files: PP `lib.rs`, `pressure_runtime.rs`,
`exact_admission.rs`, `self_weight.rs` and `certified_loads.rs` (pinned in
`b7a_domain_model.py` ~89). It also touches the pressure-2 table, the evidence schema
and readers, the pressure-source-1 companion binding, the source schema and
transport reader with their pins, and the identity TEXT. Slice c changes no
frame-kernel or `straight_pipe` code; the partial-span Timoshenko formation
(§3) prices its own files. The retained
route keeps refusing generation. Re-pricing comes with slice b's, behind the
solver lane's earlier turns.

As built (slice c alone), the priced files touched are PP `lib.rs`,
`pressure_runtime.rs`, `exact_admission.rs`, `self_weight.rs`,
`certified_loads.rs` and `source_receipt/composite.rs`, RE `physics_source.rs`
and `physics_evidence.rs`, and the two schemas (reviewed inputs). The
pressure-2 table is not changed: none of its statements names the
equivalent-static family, and its admitted-families statement moves with the
identity sentence under slice b. The new code runs only for a case that
authors `equivalent_static`, which every retained branch refuses, so a
retained-reachable case meets no new allocation. Byte identity: an EX document
without `equivalent_static` is unchanged except that, on a load/reference-state
document, the applied-loads limitation gains its refusal sentence (its
carriers are regenerated by their writers).

**Partial-span Timoshenko (§3).** The formation touches priced files: SP
`lib.rs` (`equivalent_global_nodal_loads_with_spans_formed` and its new bound),
FK `structural/formation_check.rs` (`load_reformation`,
`uniform_straight_balls`), PP `lib.rs` (the generator loses the refusal and two
locals) and `exact_admission.rs` (one code and its text removed). No reader,
schema, table, identity text or ledger layout changes. The new branches run
only for a Timoshenko frame with a proper span. Extents arise only from
generation, which every retained branch refuses, so a retained-reachable case
meets no new allocation. The priced owners keep their bounds, and the pins and
keys are re-registered.
