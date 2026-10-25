# Load combinations on the exact route

This design admits model-authored load combinations on the exact pressure
route (EX, `3.0.0/exact_pressure_v3`), for FR-SOL-009. Today EX refuses every
combination by name (`core/product_physics/src/pressure_runtime.rs` ~337-343;
`exact_admission.rs` `ExactFamily::Combination`), while the pressure-free route
(PF) evaluates them through `core/loads/load_case_algebra`. It is W-16 slice b
in `docs/COMPLETENESS_MAP.md` §4.3.

Paths are relative to `projects/chirality-piping/`. PP is
`core/product_physics/src`.

## 1. Scope and inputs

The input is the model's existing `combinations`, in the closed records #1573
introduced (DEL-04-07 `Design/pressure-contract.md` ~122-167). It has three
bases: `mechanics` (a linear combination), `result_state_subtraction` and
`range_envelope`. Each is evaluated by the same `load_case_algebra` functions
PF uses: `linear_combination` (one exact sum of exact products, rounded once),
`subtract` and `range_envelope`.

Every #1573 validation and refusal code applies unchanged on EX:
- `LOAD_COMBINATION_BASIS_UNSUPPORTED`, `_SHAPE_INVALID`,
  `_LOAD_CASE_UNKNOWN`, `_TERMS_EMPTY`, `_FACTOR_INVALID`,
  `_RANGE_MODE_UNKNOWN`, `_OPERANDS_EMPTY`;
- `LOAD_COMBINATION_DUPLICATE_TERM`;
- `LOAD_COMBINATION_SOURCE_RESULT_MISSING`;
- `LOAD_COMBINATION_ROW_WITHHELD`.

`UNKNOWN_MODEL_FIELD` keeps refusing undeclared keys. No new input key is
added.

## 2. Which rows combine

The pressure-2 semantic table already classifies every row kind by
`combination_algebra` (DEL-04-07 `u4b-u5b-implementation.md` §2.7):
- **`signed_linear` rows** (81 kinds) combine directly as matching signed
  quantities. They include displacements, rotations, section resultants,
  signed stress components, torsional shear, the pressure-family forces (wall
  force, effective force S) and stresses (membrane, Lamé hoop and radial),
  support components and connector generalized actions. Lamé hoop and radial
  stress are linear in p for a fixed annulus, and pressure stiffening is not
  modelled (`exact_admission.rs` ~95), so their sum is the stress of the
  summed pressure state.
- **`recomputed_from_combined` rows** (3 kinds: the displacement and support
  magnitudes) are formed from the combination's own components, never summed.
  A range envelope keeps its per-row selection, as on PF.
- **`never_combined` rows** (45 kinds: every maximum, extreme and headline
  row, the shear maximum, intensified rows, nonlinear rows) are not published
  for a combination. As on PF, `COMBINATION_STRESS_MAXIMUM_UNAVAILABLE` says
  so.

**Section bases.** A row combines only with the same row on the same section
basis. Nominal twins combine with nominal twins, and reduced twins with
reduced. A combination publishes both bases' twins where its operands do.

**Pairing.** EX pressure rows are `_v2` and carry their case id inside the row
id (PP `u4b_rows.rs` ~441), so PF's base-id index would never pair them.
Combination pairing on EX uses a structural key: the row id with its case-id
component removed. That leaves kind, member, station, section basis and
component.

**An operand with no row for a key.** A pipe pressurized in one operand may be
outside every region in another. That operand then has no pressure rows for
the pipe. The design treats this as an exact zero only when the pipe is
published in that operand's solved case and lies in none of that case's
pressure regions. A pressure-family row there is structurally zero, because no
pressure term acts on the member. Any other absence is a missing source, and
`LOAD_COMBINATION_SOURCE_RESULT_MISSING` withholds the combination, as #1573
specifies.

## 3. Gates

The PF mechanics gates apply to EX combinations in the same precedence order,
where the conditions can arise. `NONLINEAR_COMBINATION_REQUIRES_SOLVE` and
`CONSTANT_EFFORT_COMBINATION_REQUIRES_SOLVE` cannot arise there, because EX
refuses nonlinear and constant-effort supports.

`COMBINATION_MODULUS_BASIS_MIXED` applies to EX, and one gate is added,
`COMBINATION_RESOLVED_STIFFNESS_MIXED`. It withholds a combination whose
operands were solved with different resolved member stiffness:
- a member's selected E/ν pair differs between operands;
- or its section bases, shear model or Timoshenko shear area differ.

This can happen on a load/reference-state document with per-case material
selection. A combination of states solved against different stiffness is not
the response of one linear system. The gate follows the modulus-basis
precedent.

## 4. Standing and certification

Each EX operand is certified against its own intended system. A combination
has no combination-level enclosure. A combination result therefore:
- carries no certified status of its own;
- has a standing equal to the weakest of its operands' standings;
- is labelled as derived from its operands.

Product text says its rows are "combined from certified case results", never
"certified". This follows the existing precedent that a source-blocks envelope
containing any combination is `SOURCE_BLOCKS_ENVELOPE_UNQUALIFIED`
(`result_export/src/reader_verdict.rs` ~366-370) and K-D5 R5's
"established", not "certified".

A subtraction keeps PF's `SUBTRACTION_SIGN` meaning, "not an equilibrium
state". A range envelope keeps its row-envelope meaning.

## 5. Publication and contract

`contract_evidence` gains `exact_combinations` (closed): one record per
combination. Each record holds:
- the combination id and basis;
- the terms (case id and factor) or the operands;
- each operand's standing and the combination's derived standing;
- the gate outcome;
- the pairing outcome (paired rows, structural zeros used, withheld rows).

`results_pressure_2.v0.3.schema.json`'s closed `PressureContractEvidence`
gains the member. The pressure-2 table gains a `combination_policy` modelled
on preview-physics-2's (policy id `PRESSURE-2-COMBINATION-v1`), with:
- the gates of §3;
- the pairing rule of §2;
- the standing rule of §4.

The derived tables and pins are regenerated with `semantic_contract_tables.py`,
and the checksum-bearing carriers with their own writers.

**Readers.** The model-bound readers re-derive every combination row from its
operand rows, using the policy:
- `physics_evidence.rs` admits `ref_type == "load_combination"` alongside
  `load_case`;
- `load_reference.rs` handles combinations on load-state documents;
- `tools/validation/qualification_exact_pressure.py` already accepts
  `combination`;
- the Python verifiers mirror the Rust readers.

## 6. Text that changes

- **The identity sentence** in `exact_admission.rs` ~83-85, "Every other
  component, support, combination or equivalent-static family is refused by
  name", loses "combination". About 98 files carry it, including fixtures,
  desktop twins, the `schemas/results.v0.3.schema.yaml` `const` and
  `source_receipt/pressure_source_context_tests.rs`. They are regenerated by
  their writers. W-15 (#1604) also edits this sentence. This slice lands after
  it and takes in its text.
- **The retained ruling-4 premise** (`retained_memory.rs` ~913-918) says EX
  combinations are refused "which the ordinary route also blocks". It is
  restated: the retained exact route still refuses combinations, and the
  ordinary exact route now evaluates them. The retained refusal itself is
  unchanged.
- **DEL-05-02 ScopeOfWork** ~77, "the exact pressure route refuses a
  combination by name", goes through the owner's wording grant.

## 7. What stays refused

- **Retained exact route.** Combinations stay refused there
  (`retained_memory.rs` ~916-918).
- **Source recovery.** `source_eligible` requires no combinations (PP
  `lib.rs` ~1348-1350). An EX model with a combination therefore loses
  retained-source recovery for the whole document, as a PF model already does.
  This is disclosed in the combination's evidence record. The design does not
  extend source recovery to combinations.
- **`equivalent_static` on EX** is slice c.

## 8. Verification

- **Independent references** under `validation/references/w16_exact_combinations/`,
  from an exact-rational generator that runs no product code. Cases:
  - **Pressure and force.** An invented two-case EX model: case A has pressure
    on member 1 only and case B a tip force. Checks a linear combination
    1.0·A + 1.3·B on both bases (Lamé hoop and radial, membrane, wall force,
    displacements, reactions), and the structural zero for member 2's pressure
    rows in A.
  - **Subtraction.** A − B.
  - **Range envelope.** Over A, B and A + B.
  - **Load-state document.** A two-case document with equal resolved
    stiffness that combines, and one with differing E/ν that is withheld by
    `COMBINATION_RESOLVED_STIFFNESS_MIXED`.
- **Route parity** with PF on a pressure-free EX model, for the force rows.
- **Negatives:** every #1573 code on EX, the missing-source case that is not a
  structural zero, mismatched bases and a forged combination row through the
  readers.
- **Byte identity.** Every EX document without a combination is unchanged
  except for the identity-sentence text and its pins.

## 9. Retained-memory pricing (D3)

The ordinary EX combination path is in priced files: PP `lib.rs`,
`preview_physics.rs` (or a new module), `pressure_runtime.rs`,
`exact_admission.rs`, the pressure-2 table (a static owner), and the result
readers (`physics_source.rs`, `semantic_contract.rs`). The identity text is
priced TEXT. The retained route keeps refusing combinations on E but prices
them on L. Re-pricing and re-registration, with independent review, queue
behind the solver lane's W-15 and W-16a turns.
