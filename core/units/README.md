# OpenPipeStress Unit Core Contract

DEL-02-02 defines the contract for unit-aware quantities and dimensional analysis in the domain core. This contract is structural: it does not ship protected standards data, proprietary dimensional tables, code-specific values, material allowables, SIF/flexibility factors, or vendor data.

## Storage Convention

Every physical quantity that crosses a schema, solver, import/export, report, or rule-evaluation boundary must carry:

- a magnitude;
- an explicit unit reference;
- an explicit dimension identifier;
- a quantity kind, such as unit-bearing, dimensionless, ratio, percentage, coefficient, absolute, interval, or relative;
- provenance for values that can affect engineering reliance;
- a missing-unit behavior.

Persisted project data should preserve the entered unit representation when available for audit and round-trip behavior. Downstream implementations may also maintain a canonical calculation representation, but the canonical basis, conversion constants, numeric representation, and tolerance policy remain `TBD` until accepted by a human review gate. JSON payloads that are hashed must use the project canonical JSON/JCS-compatible basis where applicable.

Dimensionless values are not a fallback for missing units. A value may be unitless only when its field is explicitly classified as dimensionless, ratio, percentage, or coefficient. Otherwise missing unit metadata is a diagnostic.

## Dimension Checking

The unit core owns dimensional compatibility checks for calculations, schemas, imports, exports, and rule evaluations. Adapters and plugins must call through this contract instead of bypassing it.

Required operation behavior:

- addition, subtraction, comparison, and conversion require compatible dimensions;
- multiplication, division, and power operations must produce explicit derived dimensions where implemented;
- unknown, unsupported, or unresolved dimensions remain `TBD` or produce blocking diagnostics;
- offset/reference-sensitive quantities, including temperature scale versus interval and gauge versus absolute pressure, require explicit semantics before conversion.

The schema models dimensions with stable identifiers and exponent vectors. The PKG-02 canonical vocabulary is `dimensionless`, `length`, `mass`, `time`, `temperature`, `temperature_interval`, `angle`, `rotation`, `force`, `moment`, `pressure`, `stress`, `area`, `volume`, `density`, `linear_stiffness`, `rotational_stiffness`, `displacement`, `velocity`, `acceleration`, `thermal_conductivity`, `specific_heat`, `thermal_expansion_coefficient`, `second_moment_area`, `section_modulus`, `mass_per_length`, `volume_per_length`, `slope`, and `TBD`. This vocabulary is a contract surface for checking structure; it is not a source of engineering design values.

Retired aliases are not accepted as canonical enum values. Use `temperature_interval` for former `temperature_difference`, `second_moment_area` for former `area_moment`, and classify generic stiffness as either `linear_stiffness` or `rotational_stiffness`.

## Operation Rules

The unit contract records dimensional operation rules explicitly. Implementations must not hide compatibility behavior in adapters, GUI code, report rendering, or rule-pack evaluation.

Required operation categories are:

- same-dimension operations: addition, subtraction, comparison, and conversion;
- derived-dimension operations: multiplication, division, and power where implemented;
- boundary validation operations: schema validation, import validation, export validation, and rule evaluation;
- explicit classification operations for dimensionless, ratio, percentage, and coefficient values.

Unsupported operation semantics are represented as blocking diagnostics or `TBD` decisions. Deterministic conversion tests that require numeric conversion constants remain gated until the unit catalog, factor representation, and tolerance policy are accepted.

## Conversion Provenance

Conversion declarations are records, not hidden constants. Each conversion declaration must identify:

- source unit and target unit;
- dimension;
- transform kind;
- factor or offset representation;
- provenance;
- redistribution and review status.

Public conversion data must satisfy the project IP and data-boundary policy. If a conversion source appears to come from protected standards content, proprietary vendor data, or undocumented commercial data, ingestion stops and the record is quarantined for human/legal review.

## Missing-Unit Handling

Missing solve-required or rule-check-required units are findings, never silent defaults. The expected handling is:

- solve-required physical quantity missing unit: blocking unit diagnostic;
- rule-pack input missing unit or incompatible with rule expectation: rule-check-blocking diagnostic;
- explicitly dimensionless value without provenance concerns: accepted only if the schema field permits that classification;
- weak or missing provenance for public conversion/unit data: provenance warning or quarantine, depending on risk;
- unsupported special semantics: blocking diagnostic until the behavior is accepted.

The schema diagnostic codes include missing unit, unknown unit, ambiguous unit, dimension mismatch, unsupported conversion, missing conversion provenance, unresolved offset/reference semantics, required dimensionless classification, and suspected protected unit data.

## Open Decisions

The following decisions remain contract-visible and must not be filled by implementation convenience:

- unit catalog and conversion source set;
- base dimension vector and derived-dimension rules;
- dimensionless, ratio, percentage, coefficient, angle, and rotation semantics;
- numeric representation for magnitudes and conversion factors;
- conversion tolerance policy;
- offset temperature and gauge/absolute pressure behavior;
- canonical calculation basis and storage shape;
- schema file layout and diagnostic-code namespace;
- human decision owner or review gate for these topics.

Each accepted decision that affects public schemas, persistent files, or external contracts should be recorded through the project ADR or equivalent decision-record discipline.

## Downstream Obligations

Solver, load, stress, rule-pack, report, GUI, API, and adapter implementations must preserve this boundary:

- unit checks support mechanics and rule evaluation;
- unit checks do not certify, seal, approve, authenticate, or declare code compliance;
- imports and exports must reject or flag missing, ambiguous, or incompatible units;
- rule evaluators must be unit-aware and deterministic;
- reports must disclose unit-system identity, diagnostics, assumptions, and provenance where relevant;
- tests must cover schema parsing, required dimensions, required quantity fields, operation rules, incompatible dimensions, dimensionless classification, missing-unit diagnostics, alias ambiguity rejection once a namespace is approved, deterministic conversion behavior once constants are approved, and absence of silent defaults.

Open decisions remain `TBD`: final unit catalog, conversion constants, numeric representation, tolerances, offset temperature behavior, gauge pressure behavior, angle/rotation treatment, code-specific values, release claims, professional claims, canonical calculation units, alias policy, and human decision owner.
