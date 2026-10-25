# Load combinations on the exact route

This design admits model-authored load combinations on the exact pressure
route (EX, `3.0.0/exact_pressure_v3`) for FR-SOL-009 (PRD ~518). The PRD
defines combinations and ranges over primitive solved cases.

Today EX refuses every combination by name (PP `pressure_runtime.rs` ~337-343;
`exact_admission.rs` `ExactFamily::Combination`). The pressure-free route (PF)
already evaluates combinations through `core/loads/load_case_algebra`. This is
W-16 slice b in `docs/COMPLETENESS_MAP.md` §4.3.

Paths are relative to `projects/chirality-piping/`. PP is
`core/product_physics/src`; RE is `core/reporting/result_export/src`.

## 1. Inputs

The input is the model's existing `combinations`, in the closed records of
#1573 (DEL-04-07 `Design/pressure-contract.md` ~122-167). There are three bases:
- `mechanics`: a linear combination over primitive load cases;
- `result_state_subtraction`;
- `range_envelope`: over primitive load cases, with its mode.

Operands are primitive load-case ids, as they are today (PP `validation.rs`
~1973-1989, ~2142-2176). A combination cannot be an operand.

Each basis is evaluated by the same `load_case_algebra` functions PF uses. The
dimension helper (PP `lib.rs` ~14241-14248) gains Pa, the unit of pressure-family
stresses, and m for connector displacements.

Every #1573 code applies unchanged on EX: the `LOAD_COMBINATION_*` validation
codes, `LOAD_COMBINATION_DUPLICATE_TERM` (once per repeated case),
`LOAD_COMBINATION_SOURCE_RESULT_MISSING` (including the first operand), and
`LOAD_COMBINATION_ROW_WITHHELD`. `UNKNOWN_MODEL_FIELD` still applies. No input
key is added.

## 2. Mechanics of a combined row

EX is linear in its resolved loads and eigenstrains for a fixed stiffness and
reference state:
- **Straight pressure recovery** is linear in mechanical force and pressure:
  N_w = N_m + 2νpAᵢ and S = N_m + (2ν − 1)pAᵢ (PP
  `pressure_exact/source_geometry.rs` ~346-375).
- **Membrane stress** is linear on each basis area (~396-403).
- **Thermal and pressure-Poisson loads** are additive, with no product term
  (PP `pressure_exact.rs` ~310-330).
- **Closure transfer** selects a linear cap load (PP `pressure_runtime.rs`
  ~864-890).
- **Realized bends** are pressure-linear (PP `bend_pressure.rs` ~4-20).
- **Toroidal hoop and radial stress** are pressure-linear at fixed admissible
  geometry.

The linearity is in resolved loads, not necessarily in authored temperatures or
reference-state parameters. A load-state strain composition can contain a
fit/thermal product (DEL-04-01 `kd5-member-relative-bound.md` ~98), which is
resolved per case before the solve.

The pressure-2 table classifies each row signature by `combination_algebra`
(DEL-04-07 `u4b-u5b-implementation.md` §2.7). The reviewed table has these
counts:
- 87 `signed_linear` signatures (39 kinds), combined directly as matching
  signed quantities;
- 3 `recomputed_from_combined` signatures (3 kinds), formed from the
  combination's own components. A range envelope keeps its per-row selection,
  as on PF.
- 45 `never_combined` signatures (26 kinds), not published for a combination.
  The one exception is the combination's own modulus-basis record, as on PF
  (§2.7). `COMBINATION_STRESS_MAXIMUM_UNAVAILABLE` says so.

Headlines and maxima stay primitive-case only.

**Section bases.** A row combines only with the same row on the same section
basis. Combination stress metadata keeps the section-basis suffixes; PF's
generic metadata replacement is not used for these rows.

## 3. Pairing operands

Rows are paired by a typed structural key built from the rows' structured
fields. Row-id strings are never parsed. The key is:
- kind and unit;
- the entity references: member, connector, tie-rod (rod identity included),
  support or node;
- station and frame;
- component and section basis.

A row family whose rows lack a field needed for its key is not combined. Each
withheld row is named by `LOAD_COMBINATION_ROW_WITHHELD`, never silently
dropped. Tests cover non-default cases with pressurized arcs, whose `_v1`
toroidal rows already carry the case in their base id (PP `u4b_rows.rs`
~37-38, ~279-285, ~419-441), and ids rich in delimiters.

When one operand lacks a pressure-family row, a pipe pressurized in one operand
may lie outside every region in another. For such a member, the operand's
values are recovered from the formulas, with p = 0, as follows:
- **Pressure-only quantities** (Lamé hoop and radial, toroidal hoop and
  radial) are exactly zero.
- **Total quantities** are derived from N, the operand's tension-positive
  section force on that member, as follows:
  - **N at each station.** At `end_i`, N is the negated `element_local_axial_force`
    end-action row. At `end_j`, N is that row unchanged. At interior stations,
    N is the matching interior section row. On a realized arc, the same rule
    applies in the end-tangent frames (`bend_pressure.rs` ~207).
  - **Wall and effective force.** N_w = S = N at p = 0, from
    N_w = N_m + 2νpAᵢ and S = N_m + (2ν − 1)pAᵢ on straights, and
    N_w = N_el + pAᵢ and S = N_el on arcs (`pressure_exact/source_geometry.rs`
    ~346-378).
  - **Membrane.** σ_m,b = N / A_b, one binary64 division of the published N by
    the basis's validated area leaf A_b (from `pipe_sections` evidence),
    rounded once. This is derived from published operands. It can differ by
    one ulp from a direct recovery, which divides the internal force
    (PP `lib.rs` ~12820). The evidence states this.
  - **`pipe_wall_endpoint_action_v2`** is recovered in its node-on-element
    convention, −N_w at `end_i` and +N_w at `end_j`: the published end-action
    row value at each end.

  Each witness records the source row, the station, the sign transformation,
  the area leaf and the rounding. Tests cover both ends, interior stations,
  realized arcs and both bases.

A member qualifies only when it is published in that operand's solved case
and is validated, by the case's region membership, as lying in none of its
pressure regions. Each recovered operand value is recorded in the
combination's evidence with its provenance, `pressure_free_operand_recovery`.

Every other absence is a missing source. This covers:
- a toroidal domain withheld (PP `u4b_rows.rs` ~313-339);
- a recovery failure (PP `lib.rs` ~12719-12731, ~12835-12843);
- a span replaced by a connector;
- a withheld support group.

As on PF (`preview_physics.rs` ~936-979), the affected row is omitted under
`LOAD_COMBINATION_SOURCE_RESULT_MISSING` alone, and the other rows publish.
`LOAD_COMBINATION_ROW_WITHHELD` names the rows withheld for other reasons:
missing key fields and tie rods. Support groups still publish whole or not
at all (~987-1049).

**Tie rods.** Tie-rod rows share their connector's entity, component, frame
and location. Their rod identity lives only in their ids and in the connector
evidence (PP `lib.rs` ~4526). This slice withholds tie-rod rows from EX
combinations by name, under `LOAD_COMBINATION_ROW_WITHHELD`. Joining the rod
identity from validated evidence is a later extension.

## 4. Gates (mechanics basis only)

The gates apply to `mechanics` combinations only, as on PF (~799-819).
Subtraction and envelopes compare states and may legitimately span different
stiffness.

In precedence order:
1. **`COMBINATION_MODULUS_BASIS_MIXED`**, as today.
2. **`COMBINATION_RESOLVED_STIFFNESS_MIXED`** (new). Some member's resolved
   stiffness differs between operands:
   - its selected E/ν, or its operating or property temperature selection;
   - its section bases, shear model or Timoshenko shear area;
   - its W-15 effective stiffness scale.

   Load-state documents resolve materials per member and case (PP
   `case_state/material.rs` ~13-56). The old modulus key describes only the
   legacy selector (PP `lib.rs` ~10534-10576).
3. **`COMBINATION_INSTALLED_REFERENCE_REQUIRES_SOLVE`** (new). This gate
   withholds the combination when two things hold:
   - the model has a connector or tie rod whose raw installed reference is
     nonzero (q_ref, or the rod's reference extension), including a reference
     in the null space with K·q_ref = 0;
   - the factors fail the shared unit-sum rule. Under that rule, the exact
     dyadic sum Σf of the represented binary64 factors must satisfy
     |Σf − 1| ≤ 64ε. The comparison is exact, with no Σ|f| scaling, and any
     non-finite factor is refused.

   **Why a unit sum.** A connector's action is K(q − q_ref) (PP `lib.rs`
   ~4516-4520; rod at `joint.rs` ~988). Summing cases sums their reference
   loads, so only a unit factor sum leaves the combination as the state of the
   original system. The rule bounds the reference-law defect at
   |Σf − 1|·|K·q_ref| ≤ 64ε·|K·q_ref|.

   **Why this form of tolerance.** Exact equality to 1 would refuse 0.3 + 0.7,
   whose represented sum is 1 − 5.55e−17. A tolerance scaled by Σ|f| would pass
   2^53 + (−2^53) = 0. The exact accumulator cannot overflow to inf.

   **The shared check.** The exact dyadic unit-sum check is one shared helper
   in `load_case_algebra`. The numerical lane adds it when it fixes PF's
   constant-effort gate, which had the same tolerance weakness (Agent 0,
   2026-10-25). This gate calls that helper and builds no second
   implementation. Implementing this slice waits for the helper to land.

`NONLINEAR_COMBINATION_REQUIRES_SOLVE` and
`CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE` cannot arise on EX, which refuses
those supports.

Pressure regions, closure transfer, reference eigenstrains and prescribed
boundary motion may differ between operands. They are right-hand-side
contributions to the same K. Boundary motion combines through both f and the
prescribed values g, on already restrained rigid DOFs (PP
`case_state/resolve.rs` ~1050-1085). A combined right-hand side is not claimed
to correspond to one authorable pressure-region topology.

## 5. Standing and wording

A combination has no numerical check of its own:
- **Product text.** It says its rows are "derived from case results". It
  never says "certified" (DEL-04-01 `kd5-nonlinear-active-set.md` R5
  ~259-264; `docs/claims_registry.md` ~143-162).
- **Operand findings.** The combination's evidence reports each operand's
  actual numerical findings and standing, including fallback or non-passing
  cases.
- **Reader eligibility.** Combination rows are derived inspection results.
  They never count towards a document's numerical-eligibility verdict, which
  stays based on primitive cases (RE `reader_verdict.rs` ~391-415;
  `semantic_contract.rs` ~910-945). An envelope of source blocks containing a
  combination stays `SOURCE_BLOCKS_ENVELOPE_UNQUALIFIED` (~366-370).

A derived enclosure, rounded up from Σ|fᵢ|·rᵢ plus δ_publication, would be
needed for numerical eligibility. It is not part of this slice.

A subtraction keeps `SUBTRACTION_SIGN`, meaning it is not an equilibrium state.
A range envelope keeps its row-envelope meaning.

## 6. Contract and readers

**`contract_evidence.exact_combinations`** (closed; omitted when the document
has no combination, so existing bytes hold) has one record per combination:
- id and basis;
- the complete expression: terms (case id and factor), the subtraction's
  minuend and subtrahend, or the envelope's operands and mode;
- per operand: its standing and numerical findings;
- the gate outcome;
- the expected published rows, with their operand source rows;
- the `pressure_free_operand_recovery` witnesses;
- the withheld rows with reasons;
- the source-recovery exclusion (§7).

The raw reference token for a combination row stays `combination` (PP
`preview_physics.rs` ~843).

**Schemas:**
- `results_pressure_2.v0.3.schema.json`: `PressureContractEvidence`
  (~1786-1819) gains the closed member. The combination stress metadata keeps
  the basis suffixes (~1937-1970).
- `results.v0.3.schema.yaml`: pressure references, metadata and identity
  constants, and pins.
- `stress_neutral_export.v0.3.schema.json`: the pressure evidence reference
  (~3982) and pins.
- `analysis_run.v0.3.schema.json`: pins only.

No input record changes. No combination `LoadReferenceStateRecord` is created.
The retained and source receipt schemas are not broadened.

**Semantic tables:**
- **Pressure-2 table.** Its limitation "Direct load cases only; derived
  combinations and state differences are unsupported." (~2284) is replaced by
  this design's statement. It gains `combination_policy` (policy id
  `PRESSURE-2-COMBINATION-v1`) with the gates, pairing, recovery and standing
  rules. `semantic_contract_tables.py` does not inherit `combination_policy`
  (~70-82), so the member is authored in the pressure-2 table alone.
  physics-retained-1 and pressure-source-1 do not carry it, because their
  families refuse combinations. Their table test asserts its absence.
- **physics-retained-1 and pressure-source-1.** Text and pins are reconciled,
  and their families keep refusing.

**Readers** (Rust and their Python mirrors):
- A direct-case pass first validates the operands as today.
- A separate combination pass, bound to the model, then re-derives every
  combination row from the complete expression, the validated operand rows
  and the model's region and stiffness facts. It compares complete coverage:
  extras, omissions, provenance, recovery witnesses, gates, standing and
  withheld reasons. A forgery of any of these is refused.

Surfaces:
- Rust: RE `physics_evidence.rs` (direct derivation refs stay refused;
  combinations go through the new pass), `load_reference.rs` (closed keys
  ~56-61, projection ~600-615), `connector_evidence.rs`, `section_basis.rs`,
  `u4b_evidence.rs`, `global_end_evidence.rs`, `reader_verdict.rs`,
  `semantic_contract.rs`.
- Python: `tests/verifiers/pressure_2.py`, `physics_evidence.py`,
  `load_reference_evidence.py`, and the section and U4b helpers.
- `tools/validation/qualification_exact_pressure.py`: its input binding
  (~725-731) learns combination rows.
- W-15's `component_body` record (#1604) is carried on body stress rows
  across combinations, with consistent records, and its rule-input restriction
  is preserved.

## 7. Text, refusals that remain, and the source-recovery exclusion

**Text that changes:**
- **The identity sentence** (PP `exact_admission.rs` ~83-85) loses
  "combination". About 98 files carry it, and they are regenerated by their
  writers. W-15 (#1604) also edits this sentence; this slice follows it and
  takes in its text.
- **DEL-04-07 `pressure-contract.md`** ~7 changes.
- **The retained ruling-4 premise** (`retained_memory.rs` ~913-918) becomes:
  "the retained exact route refuses combinations; the ordinary exact route
  evaluates them". The refusal itself is unchanged.
- **Completeness map:** ~120, ~182 and the FR-SOL-009 row.
- **DEL-05-02 ScopeOfWork:77**, through the owner's wording grant. The old
  row is: "| Product use | The product preview route evaluates model-authored
  `combinations` (bases `mechanics`, `result_state_subtraction` and
  `range_envelope`) through this crate; the exact pressure route refuses a
  combination by name. | Implemented in DEL-04-07's `core/product_physics` |".
- No acceptance criterion changes. DEL-05-02 AC-001 and DEL-04-08's retained
  and source exclusions stay.

**Still refused:** the retained exact route keeps refusing combinations (~916);
`equivalent_static` on EX is slice c.

**Source recovery.** `source_eligible` requires no combinations (PP `lib.rs`
~1343-1349). An EX document with any combination therefore loses
retained-source recovery for all its cases, including its primitive cases. PF
never attempts source recovery, because it fails the exact condition. The
combination evidence and the user-facing diagnostic both state this exclusion.

## 8. Verification

**Independent references** go in `validation/references/w16_exact_combinations/`.
They come from an exact-rational generator that runs no product code and has a
documented independent π treatment. Exact algebra checks use the represented
binary64 factors.

**Closed form.** A straight, closed-ended cantilever has P = p·A_i,r (the
reduced bore), nominal area Aₙ, length L and tip force F:
- u_A = (1 − 2ν)PL/(E·Aₙ) and u_B = FL/(E·Aₙ);
- for A + cB: N_w = P + cF, S = cF and σ_m,b = (P + cF)/A_b;
- R_x = −cF.

The Lamé stresses on each basis b use C_b = p·r_i,b²/(r_o² − r_i,b²),
σ_θ = C_b(1 + r_o²/r²) and σ_r = C_b(1 − r_o²/r²). Serial members add their
axial compliances.

**Cases:**
- **Pressure and axial force.** A pressurized member 1 and an unpressurized
  member 2. Case A has pressure. Case B has axial force on both members and
  pressure on member 2 only. The case checks:
  - pairing against existing member-2 pressure keys;
  - the `pressure_free_operand_recovery` values;
  - A + 1.3B with negative, zero and cancelling factors.
- **Subtraction**, including across different stiffness, where it is allowed.
- **Range envelopes** over primitive cases A, B and a third primitive case C
  carrying A's and B's loads together, in every range mode.
- **Connectors and rods.**
  - Connectors with nonzero installed references. These pass: 1; 0.3 + 0.7;
    a sum within 64ε of 1. These are withheld: 2, 0, 2^53 + (−2^53), and a sum
    just outside 64ε. A non-finite factor is refused.
  - A null-space q_ref.
  - Two distinct rods, whose rows are withheld by name.
- **Arcs.** A non-default case with a pressurized realized arc.
- **Load state.** A load-state document with equal resolved stiffness and
  prescribed boundary motion, which combines; and one with different E/ν,
  which is withheld.
- **W-15 body rows** across a combination (after #1604).

**Other checks:**
- **Parity.** Sparse and dense; native and wasm readers; PF parity for the
  force rows of a pressure-free EX model.
- **Negatives.** Every #1573 code on EX; missing sources that are not
  recoverable; mismatched bases; forged expressions, gates, recoveries,
  standing and rows through the readers.
- **Byte identity.** Every EX document without a combination is unchanged,
  except for the identity sentence and its pins.

## 9. Retained-memory pricing (D3)

**Priced files.** The path touches priced files: PP `lib.rs`,
`preview_physics.rs` or a new module, `pressure_runtime.rs`,
`exact_admission.rs`, the RE readers, `physics_source.rs` and
`semantic_contract.rs`. It also touches the pressure-2 table and its inheriting
tables, which are static owners, and the identity TEXT.

**Retained route.** The retained E branch keeps refusing combinations
(`retained_memory.rs` ~913-918). It still prices combinations on L. The
premises of §8.19 are kept: the first-missing-source rule, the withheld names
and the whole support group.

**Queue.** Re-pricing and re-registration, with independent review, queue
behind the solver lane's W-15 and W-16a turns. Fit is not established until
then.
