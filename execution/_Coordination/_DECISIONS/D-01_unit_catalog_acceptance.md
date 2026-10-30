# D-01 — Unit Catalog Acceptance — Decision Packet

| Field | Value |
|---|---|
| Decision ID | D-01 |
| Status | **RULED** — accepted as proposed by the human project authority on 2026-06-10; ruling record `DEC-018` in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 |
| Prepared | 2026-06-10 by TASK (Type 2) for WORKING_ITEMS, tranche TP-APP-R2-EDITLOOP-001 decision-preparation subscope |
| Blocks | Phase B entirely; FR-002; full value of Phase A authoring (`plans/PLAN_2026-06-10_prd_completion.md` §2 row D-01) |
| Authority | Human project authority only. All recommendations below are `PROPOSAL` (`docs/CONTRACT.md` OPS-K-AGENT-4: "Agent outputs are drafts/proposals until accepted by a human gate") |

## 1. Decision statement and scope

Accept (or modify and accept) the unit contract content that `docs/SPEC.md` §4 currently gates as `TBD`, so the Phase B units engine (`core/units`) can be implemented "exactly as accepted in D-01" (`plans/PLAN_2026-06-10_prd_completion.md` §3 Phase B row B1). Five sub-decisions:

- **D-01a — Canonical unit set:** the accepted unit catalog (which units may be entered/displayed) and the canonical calculation unit basis used internally.
- **D-01b — Conversion constants:** the accepted conversion-factor source set and numeric representation.
- **D-01c — Offset-temperature semantics:** absolute temperature scale vs temperature interval (degC/K interval vs absolute), and how each converts.
- **D-01d — Gauge/absolute-pressure semantics:** how gauge and absolute pressure values are distinguished, combined, and converted.
- **D-01e — Numerical tolerance policy for conversions:** what deterministic conversion tests assert.

**Not ruled here** (adjacent `TBD`s from DEL-02-02 `Specification.md` "Open Contract Decisions" and `core/units/README.md` "Open Decisions" that remain open unless the ruling chooses to bundle them): unit identifier namespace/alias policy; dimensionless/ratio/percentage/coefficient classification; angle/rotation treatment; persisted quantity shape + hash canonicalization (interacts with D-08, model-document schema migration); schema file layout/tooling; unit diagnostic code namespace. The ruling on D-01 should name the decision owner for these five items, which partially resolves the "Human decision owner / review gate" `TBD` (DEL-02-02 `Specification.md` Open Contract Decisions, last row).

## 2. Current state — evidence

All paths relative to `projects/chirality-piping/`.

| # | Evidence | Source |
|---|---|---|
| E1 | `core/units/` contains only `README.md` (a prose module contract); no Rust crate or code exists. FR-002 status: "Unit metadata everywhere; no conversion engine (`core/units` empty)" | `core/units/` directory listing; `plans/PLAN_2026-06-10_prd_completion.md` §4 FR-002 row |
| E2 | Binding invariant: "All calculations, formulas, imported values, and exports must be unit-aware and dimensionally checked" | `docs/CONTRACT.md` OPS-K-UNIT-1 (§1 table) |
| E3 | An accepted 30-identifier dimension vocabulary already exists and is shared across surfaces: `dimensionless, length, mass, time, temperature, temperature_interval, angle, rotation, force, force_per_length, moment, pressure, stress, area, volume, density, linear_stiffness, rotational_stiffness, displacement, velocity, acceleration, thermal_conductivity, specific_heat, thermal_expansion_coefficient, second_moment_area, section_modulus, mass_per_length, volume_per_length, slope, TBD` (with retired aliases mapped) | `docs/SPEC.md` §4; `schemas/units.schema.yaml` `DimensionId` enum; `schemas/model.schema.yaml` `Quantity.dimension` enum; `docs/TYPES.md` ("shared by `schemas/units.schema.yaml` `DimensionId` and `schemas/model.schema.yaml`") |
| E4 | `CONFLICT:` `core/model_operations/validation_preview/engine.py` `CANONICAL_DIMENSIONS` (lines 41–70) carries 29 of those identifiers — it **omits `force_per_length`** — while claiming to enforce "the accepted PKG-02 vocabulary" via blocking diagnostic `OP-UNIT-DIMENSION-UNKNOWN`. The preview fixture uses `"dimension": "force_per_length"` on distributed loads (2 occurrences) | `core/model_operations/validation_preview/engine.py`; `fixtures/product_preview/invented_preview_model.json`; vs `docs/SPEC.md` §4 and both schemas |
| E5 | Schema structure for the catalog already exists: `units.schema.yaml` defines `UnitSystem` records, `ConversionDeclaration` (required: source/target unit refs, `dimension_id`, `transform_kind` ∈ {`identity`, `multiplicative`, `affine`, `nonlinear`, `unsupported_TBD`}, `factor_representation` string, `provenance`, `review_status`; optional `offset_representation` "required only for reviewed affine conversions"), `QuantityKind` ∈ {`absolute`, `interval`, `relative`, `ratio`, `percentage`, `coefficient`, `dimensionless`, `unit_bearing`, `TBD`}, and a storage-convention enum {`entered_units_preserved`, `canonical_units_only`, `TBD`} | `schemas/units.schema.yaml` (ConversionDeclaration ~lines 108–164; QuantityKind ~576–589; storage convention ~830–840) |
| E6 | The schema's own `open_decision` topic enum names this decision's parts verbatim: `unit_catalog`, `conversion_source_set`, `numeric_representation`, `conversion_tolerance_policy`, `offset_temperature_semantics`, `gauge_absolute_pressure_semantics`, `canonical_calculation_basis`, … | `schemas/units.schema.yaml` open_decision topic enum (~lines 380–400) |
| E7 | `model.schema.yaml` requires every `Project` to carry a `unit_system` whose `base_units` object requires `length`, `mass`, `force`, `temperature`, `angle` (free strings today); every `Quantity` requires `{value: number, unit: string, dimension: <enum E3>, provenance}` | `schemas/model.schema.yaml` `Project` (~1031–1056), `UnitSystem` (~1638–1680), `Quantity` (~1155–1210) |
| E8 | Units actually in use in the invented preview fixture: project units block `{length: m, force: N, temperature: degC, stress: MPa}`; quantity unit strings `m`(8), `Pa`(4), `N/m`(2), `N`(2), `degC`(1), `1/degC`(1); dimensions used `force_per_length`(2), `force`(2), `pressure`(2), `temperature_interval`(1) | `fixtures/product_preview/invented_preview_model.json` |
| E9 | The solver consumes raw consistent numbers: frame-kernel inputs "must arrive through unit-aware upstream contracts"; "canonical calculation unit basis" remains `TBD` for both the frame kernel and linear supports | `docs/SPEC.md` §5 (frame-kernel and linear-supports passages); `docs/TYPES.md` `FrameKernel` row |
| E10 | Diagnostic hook for unresolved offset semantics already exists: `OFFSET_OR_REFERENCE_UNRESOLVED`; the README requires "offset/reference-sensitive quantities, including temperature scale versus interval and gauge versus absolute pressure, require explicit semantics before conversion" | `schemas/units.schema.yaml` (~line 794); `core/units/README.md` §Dimension Checking |
| E11 | PRD demand: FR-002 (Must) "All numerical fields have units; incompatible units are rejected; reports show units used"; FR-001 round-trip "without loss of model, units, …"; §6.6 unit safety; §8.1 step 1 "User creates a new project and selects a unit system"; §11.2 "Unit-aware result storage"; §11.8 "Detect inconsistent units"; §22.1 lists "Unit system" as an R0 deliverable (deferred by governance gating and pulled back as Phase B per plan §1); Appendix A example uses `units: "inch_lbf_F"`; §24 rates "Unit conversion errors" High | `docs/PRD.md` §10 FR-001/FR-002, §6.6, §8.1, §11.2, §11.8, §22.1, §27, §24 |
| E12 | DEL-02-02 (lifecycle `CHECKING`) defines the contract requirements U-001..U-016 and the open-decision table this packet operationalizes; its Datasheet records "Unit catalog: TBD. No authoritative unit catalog, conversion table, or dimensional registry was supplied in the accessible references" | `execution/PKG-02_…/1_Working/DEL-02-02_…/_STATUS.md`, `Specification.md`, `Datasheet.md` |

## 3. The specific TBDs awaiting ruling

`docs/SPEC.md` §4 (gating paragraph): "Deterministic conversion tests that require numeric constants remain gated until the project has accepted the unit catalog, conversion source set, numeric representation, and tolerance policy. Until accepted, unsupported conversion semantics such as offset temperature, gauge versus absolute pressure, and angle/rotation treatment remain explicit `TBD` decisions or blocking diagnostics."

From DEL-02-02 `Specification.md` "Open Contract Decisions" (rows mapped to this packet):

| DEL-02-02 open decision | Required disposition (quoted) | Sub-decision |
|---|---|---|
| Numeric representation for conversion factors and stored magnitudes | "Human-approved design decision; must support deterministic testing." | D-01b |
| Offset quantities, especially temperature scale versus temperature interval | "Human-approved design decision before accepting temperature conversions." | D-01c |
| Gauge versus absolute pressure semantics | "Human-approved design decision before pressure-bearing schemas rely on it." | D-01d |
| Canonical calculation unit basis | "Human-approved design decision tied to solver and persistence behavior." | D-01a |
| Conversion constants and tolerance policy | "Executable deterministic conversion tests must wait for approved constants, representation, and tolerances…" | D-01b, D-01e |
| Public unit/conversion source set | "Must satisfy provenance and redistribution requirements." | D-01a, D-01b |
| Human decision owner / review gate | "Identify the human owner or review gate … before treating the unit contract as issued." | All (the D-01 ruling itself) |

Same items appear in `core/units/README.md` §Open Decisions and the `units.schema.yaml` open_decision enum (E6). Note: the dimension *identifier vocabulary* is already accepted (E3); what remains `TBD` for D-01a is the unit catalog and canonical basis, not the dimension names. The base-dimension *exponent-vector* basis row in DEL-02-02 remains an adjacent TBD (§1 "Not ruled here") unless bundled.

## 4. Options and recommendations

Conversion constants quoted below are public definitional physics (open mechanics per `docs/TYPES.md` glossary "Open Mechanics"); offering them as options includes no code tables, allowables, or standards-derived data. Accepting any of them into the governed catalog is itself part of the ruling (provenance + review status per `docs/SPEC.md` §4 and `schemas/units.schema.yaml` `ConversionDeclaration`).

### D-01a — Canonical unit set

| Option | Content | Impacts / trade-offs |
|---|---|---|
| A1 | **Minimal SI catalog.** Canonical calculation basis = SI coherent (m, kg, s, K, rad; derived N, Pa, N·m, …). Entered-unit catalog limited to units already exercised (E8): m, mm, N, N/m, N·m, Pa, kPa, MPa, degC, K, 1/degC, kg, kg/m³, deg/rad. | Smallest review surface; unblocks B1 immediately. US-customary entry deferred — PRD §8.1 "selects a unit system" satisfied only for SI; Appendix-A-style `inch_lbf_F` projects unsupported until a catalog amendment. |
| A2 | **SI canonical + dual entered catalog.** Same SI canonical basis; entered catalog = A1 set **plus** a curated US-customary set (in, ft, lbf, lbf·in, lbf·ft, psi, ksi, degF, degR, lb, lb/ft, lb/in³). | Satisfies FR-002 "unit systems" (plural) and the PRD Appendix A `inch_lbf_F` example; every added factor is exact-by-definition (see D-01b appendix); roughly doubles catalog review rows. |
| A3 | **Open extensible registry.** A2 plus user-defined units with user-supplied factors as governed records at runtime. | Maximum flexibility; pushes provenance/review burden into runtime; weakens the deterministic test surface; contradicts DEL-02-02 `Guidance.md` trade-off "Start with the smallest source-backed or decision-backed catalog needed for tests and early solver work; expand through review." |

**Recommendation (`PROPOSAL`):** A2. Canonical calculation basis = SI coherent; closed dual catalog as governed `ConversionDeclaration` records; registry stays closed until a later recorded amendment. Persisted documents preserve the entered representation (`entered_units_preserved`, E5) per the existing draft direction in `core/units/README.md` §Storage Convention — final persisted shape interacts with D-08 and stays flagged there.

### D-01b — Conversion constants (source set + numeric representation)

| Option | Content | Impacts / trade-offs |
|---|---|---|
| B1 | **Exact definitional constants embedded as reviewed records**, stored as f64; `factor_representation` carries the derivation text (e.g., "25.4 mm/in, exact by definition"). | No third-party dependency; fits `ConversionDeclaration` as-is; determinism straightforward; f64 rounding of derived ratios is documented and covered by D-01e. |
| B2 | **Exact rational/scaled-decimal representation** (numerator/denominator or decimal strings) evaluated at load. | Strongest exactness story; more Rust and schema work (`model.schema.yaml` `Quantity.value` is `number` today; `units.schema.yaml` magnitude already allows string) — schema change pulls in D-08 migration concerns. |
| B3 | **Adopt a third-party unit library** (e.g., a Rust units crate) as the factor source. | Fastest to code; redistribution/provenance review shifts to a dependency; factor set exceeds the accepted catalog; drift risk between library and governed records. `ASSUMPTION:` candidate crates are permissively licensed — must be verified before this option could be accepted. |

**Recommendation (`PROPOSAL`):** B1. Candidate public constants (all exact by public definition unless noted): 1 in = 25.4 mm; 1 ft = 0.3048 m; 1 lb = 0.453 592 37 kg; g0 = 9.806 65 m/s² (conventional); 1 lbf = 4.448 221 615 260 5 N (= lb × g0); 1 psi = 4.448 221 615 260 5 N / 0.000 645 16 m² ≈ 6 894.757 293 168 4 Pa (exact ratio, f64-rounded); 1 ksi = 1000 psi; 1 bar = 100 000 Pa; 1 atm = 101 325 Pa; SI prefixes exact (1 MPa = 10⁶ Pa); 180° = π rad; temperature per D-01c. Derived factors (e.g., lb/in³ → kg/m³) follow by exact arithmetic from this base set.

### D-01c — Offset-temperature semantics (degC/K interval vs absolute)

| Option | Content | Impacts / trade-offs |
|---|---|---|
| C1 | **Dual-dimension semantics** (already in the accepted vocabulary, E3): `temperature` = absolute scale point, `affine` conversions (K = degC + 273.15; degF = 1.8·degC + 32, exact); `temperature_interval` = difference, `multiplicative` only (1 K = 1 degC interval; 1 degF = 5/9 K interval, exact). Operation rules: abs − abs = interval; abs ± interval = abs; interval ± interval = interval; abs + abs rejected. `thermal_expansion_coefficient` (1/degC, E8) pairs with interval. Canonical basis: K for both kinds. | Matches the accepted vocabulary, the fixture (`temperature_interval` in use, E8), `QuantityKind` {absolute, interval}, and `transform_kind: affine` (E5). Retires `OFFSET_OR_REFERENCE_UNRESOLVED` for temperature. Most rules to implement/test in B1. |
| C2 | **Kelvin-only internal, intervals untyped** (plain numbers; retire `temperature_interval`). | Simpler engine; breaks the accepted vocabulary, the fixture, and SPEC §4 retired-alias direction (`temperature_difference` → `temperature_interval`); silent misuse risk for degF↔degC differences. |
| C3 | **Defer temperature conversion** (degC-only catalog; `affine` remains `unsupported_TBD`; degF/K entry → blocking diagnostic). | Partially unblocks B1; leaves FR-002 unmet for thermal inputs — thermal initial strain is a core solver capability (`docs/PRD.md` §11.2) — and leaves the blocking diagnostic live. |

**Recommendation (`PROPOSAL`):** C1.

### D-01d — Gauge/absolute-pressure semantics

| Option | Content | Impacts / trade-offs |
|---|---|---|
| P1 | **One `pressure` dimension + explicit quantity kind.** Use `QuantityKind` `absolute` vs `relative` (gauge) (E5). Gauge↔absolute conversion requires an explicit, provenance-carrying reference-pressure record; **no silent atmospheric default** (`docs/CONTRACT.md` OPS-K-DATA-2). 1 atm = 101 325 Pa is available as a catalog constant but applied only by explicit user/project selection. Mixed-kind arithmetic without explicit conversion = blocking diagnostic. | No vocabulary churn; uses existing schema machinery; pushes one explicit choice (reference pressure) to users — consistent with no-silent-defaults. |
| P2 | **Separate dimensions** `pressure_gauge` / `pressure_absolute`. | Strong type separation; enum churn in three surfaces (E3) plus retired-alias machinery and D-08 migration cost; conversion still needs the same reference record. |
| P3 | **Project-level declaration** ("all pressures in this project are gauge" / "absolute"), no per-quantity kind. | Simplest; silently ambiguous at import/adapter boundaries (adapters must not bypass unit checks, `docs/SPEC.md` §1/§4); weakest audit story. |

**Recommendation (`PROPOSAL`):** P1.

### D-01e — Numerical tolerance policy for conversions

Scope boundary: this rules *conversion* testing only. Solver/benchmark variance and coverage thresholds are D-04 (`_REGISTER.md` row D-04; plan §2 row D-04).

| Option | Content | Impacts / trade-offs |
|---|---|---|
| T1 | **Two-tier witness policy.** Conversion arithmetic is deterministic f64. Tests assert: identity conversions bit-exact; single exact-by-definition multiplicative factors round-trip within ≤ 1 ULP; chained/derived conversions and affine round-trips within relative error ≤ 1e-12. | Tight enough to catch representational drift; achievable with f64 (B1); explicit about where rounding may appear. |
| T2 | **Single global relative tolerance** (e.g., 1e-9) for all conversion witnesses. | Simpler to state; hides drift in exact factors; weaker regression sensitivity. |
| T3 | **Defer entirely to D-04.** | Serializes Phase B3 behind a mid-plan decision; leaves SPEC §4 test gating unresolved with no compensating benefit. |

**Recommendation (`PROPOSAL`):** T1, recorded as a software-testing policy (not an engineering allowable), with the D-04 boundary stated in the ruling.

## 5. Downstream impact map

| Consumer | What the ruling changes | Source |
|---|---|---|
| Phase A authoring (A3 inspector, A4 load cases) | Until B lands, Phase A "may proceed SI-consistent with an explicit visible 'single unit system (preview)' label — no silent unit assumptions". The D-01a catalog defines what unit pickers/display eventually offer and when the interim label can be retired | `plans/PLAN_2026-06-10_prd_completion.md` §3 Phase B interim posture |
| Phase B1 (catalog + conversion crate) | Consumes D-01a–d directly: "decision-bound constants, dimension algebra, offset-temperature and gauge/absolute-pressure handling exactly as accepted in D-01" | plan §3 row B1 |
| Phase B2 (unit-aware I/O) | Consumes D-01a (entered/display units; solver-boundary normalization to the canonical basis) and D-01d (pressure entry). Coordinate write scopes with A3 — "B is independent until B2 touches the same input fields A3 builds" | plan §3 row B2; plan §5 |
| Phase B3 (round-trip + tolerance tests) | Consumes D-01e (conversion witnesses, rejection tests); solver thresholds stay with D-04 | plan §3 row B3 |
| FR-002 acceptance criteria | D-01a → "All numerical fields have units"; D-01c/d dimension+kind rules → "incompatible units are rejected"; entered-representation + report disclosure → "reports show units used" | `docs/PRD.md` §10 FR-002 |
| FR-001 / A2 persistence | Round-trip "without loss of … units"; persisted quantity shape + JCS hashing interact with D-08 | `docs/PRD.md` §10 FR-001; DEL-02-02 `Specification.md` open decision "Persisted quantity shape and hash canonicalization" |
| Rule packs (Phase C) | Rule evaluation consumes the same dimension/conversion contract (rule-pack unit-mismatch diagnostics) | `docs/SPEC.md` §4; `core/units/README.md` §Downstream Obligations |
| Reports (A7) | Reports must disclose unit-system identity and diagnostics | `core/units/README.md` §Downstream Obligations |
| Validation preview engine | E4 `CONFLICT` (missing `force_per_length` in `CANONICAL_DIMENSIONS`) should be fixed regardless of the ruling; the ruling makes the catalog the single authority the engine must mirror | `core/model_operations/validation_preview/engine.py` |

## 6. Ruling authority and record mechanism

Only the human project authority rules on D-01. This packet and its recommendations are agent-prepared `PROPOSAL` content and confer no acceptance (`docs/CONTRACT.md` OPS-K-AGENT-4, OPS-K-AGENT-1; software/agents make no compliance or certification claims, OPS-K-AUTH-1).

Mechanism (per `_REGISTER.md` header): record the accepted ruling as a `DEC`/`SCA` entry in `execution/_Decomposition/SOFTWARE_DECOMP.md` §12 (existing practice: DEC-001..DEC-017, e.g., DEC-017 accepting the SCA-003 storage profile) or a successor register; then the dispatching persona updates the D-01 register row to `RULED` with a pointer. DEL-02-02 `Specification.md` §Documentation also expects an ADR/decision record for "any accepted unit registry, conversion-factor representation, numeric representation, canonical calculation basis, or offset/gauge semantics that affect public schemas or persistent files."

A complete ruling states: (1) the accepted option (or modified content) for each of D-01a–e; (2) the decision owner for the remaining adjacent unit `TBD`s (§1); (3) which catalog rows are accepted as governed public conversion records with provenance/review status; (4) the explicit D-04 and D-08 boundaries.

## 7. Open items surfaced by preparation

- `CONFLICT:` `force_per_length` missing from `core/model_operations/validation_preview/engine.py` `CANONICAL_DIMENSIONS` while present in `docs/SPEC.md` §4, `schemas/model.schema.yaml`, `schemas/units.schema.yaml`, and in use in `fixtures/product_preview/invented_preview_model.json` (E4). Code fix is outside this packet's scope.
- `TBD:` angle/rotation dimensional treatment (DEL-02-02 `Specification.md` row "Angle and rotation dimensional treatment", status `ASSUMPTION`) — not bundled into D-01a–e; needs its own ruling or explicit bundling by the human authority.
- `TBD:` whether `model.schema.yaml` `UnitSystem.base_units` free-string fields should be constrained to the accepted catalog identifiers after the ruling (schema change → D-08 interaction).
- `ASSUMPTION:` the curated US-customary set in option A2 (in, ft, lbf, psi, ksi, degF, lb, lb/in³, …) is inferred from `docs/PRD.md` §27 Appendix A (`inch_lbf_F`, `lb/in^3`, `psi`, `degF`) and Phase D's user-entered-factor posture; the exact membership list is for the human authority to confirm.
