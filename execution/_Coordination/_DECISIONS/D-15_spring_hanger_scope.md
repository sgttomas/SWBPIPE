# D-15 - Spring-Hanger Scope

**Status:** RULED - human project authority approved Option B; ruling recorded as `DEC-049`.
**Prepared:** 2026-06-21 by WORKING_ITEMS, per the Application Integration And Issuance Loop decision-escalation step.
**Register row:** `execution/_Coordination/_DECISIONS/_REGISTER.md` row D-15.
**Plan basis:** `plans/PLAN_2026-06-17_prd_completion.md` §2.3 row D-15 and Phase D row D5.
**Epistemic posture:** evidence below is `FACT` with citations; inferences are labeled `ASSUMPTION`; the recommendation is labeled `PROPOSAL`; unknowns stay `TBD`. This packet decides nothing.

Citations pinned at repo HEAD `79338fab3`. Symbol names are durable anchors if line numbers drift.

---

## 1. Decision Statement And Scope

**Decide:** whether the R4 "spring hangers" deliverable is satisfied by the
existing generic `spring` support behavior, or whether Phase D item D5 must add
a dedicated spring-hanger model for:

- **variable spring hangers** - user-entered spring rate, load/travel metadata,
  and hot/cold/load-range evidence;
- **constant-effort supports** - user-entered constant support force and travel
  evidence;
- provenance/manufacturer/source notes and diagnostics for missing or invalid
  user-entered hanger data.

This is a **scope and data-contract decision**, not a value-selection decision.
The software must not size hangers from protected/vendor tables, supply catalog
defaults, or claim support acceptability. Under every option, hanger values are
user-entered or lawfully imported private data, and missing values remain
explicit findings.

**In scope:** whether D5 is a no-op over existing generic `spring`; whether new
support family tokens / schema slots are required; whether constant-effort
support belongs in the linear support tranche, load-side path, nonlinear
support tranche, or a new bounded integration slice; what evidence D5 must land
for the invented preview path.

**Out of scope:** actual hanger sizing, vendor catalog selection, code-derived
load variation allowability, protected standards criteria, final nonlinear
assembled solve convergence, sparse live-path adoption (`D-17`), and R4 exit
acceptance. This packet does not issue a deliverable or assert that R4 is met.

**What this blocks.** Phase D item **D5 spring hangers** is explicitly gated by
`D-15`; D5 cannot open as implementation work until the human rules this packet.
If D-15 waits for ruling, the next unblocked Phase D implementation item is D6
under already-ruled `DEC-044` / `DEC-046`.

---

## 2. Current State Evidence

### 2.1 PRD Treats Spring Hangers As Post-MVP R4 Scope, Separate From Generic Springs

- **FACT:** Post-MVP scope lists "Reducers, flanges, valves, expansion joints,
  and spring hangers" separately from nonlinear supports (`docs/PRD.md:315-320`).
- **FACT:** PRD §11.4 lists support/restraint types including "Translational
  spring", "Rotational spring", **"Constant-effort support"**, and
  **"Variable spring hanger"** as separate bullets (`docs/PRD.md:476-489`).
- **FACT:** PRD R4 deliverables include "Spring hangers" alongside expansion
  joints and gaps/lift-off/friction (`docs/PRD.md:1223-1232`).
- **FACT:** The PRD validation examples include a "Linear spring support
  benchmark", but no explicit constant-effort or variable spring hanger
  benchmark is currently named in that list (`docs/PRD.md:894-909`).

**Implication (`ASSUMPTION`):** a generic translational/rotational spring is a
necessary primitive, but PRD wording is too explicit to treat it as obviously
sufficient for the R4 "spring hangers" deliverable without a human ruling.

### 2.2 Current Model Schema Has Generic `spring`, Not Hanger Families

- **FACT:** `schemas/model.schema.yaml` `Support.support_type` enum includes
  `anchor`, `guide`, `line_stop`, `restraint`, `spring`, `gap`, `friction`,
  `other`, and `TBD`; it does **not** include `variable_spring_hanger` or
  `constant_effort_support` (`schemas/model.schema.yaml:1611-1639`).
- **FACT:** the same `Support` object has a generic `properties` map whose
  values are `Quantity` records, plus one `provenance` object
  (`schemas/model.schema.yaml:1650-1660`). It has no named hanger fields for
  hot load, cold load, travel range, manufacturer reference, or constant-effort
  load.

**Implication (`ASSUMPTION`):** implementing hangers only through
`support_type = "spring"` plus arbitrary `properties` would be backwards
compatible, but weakly typed; implementing explicit hanger support types and
named fields would be clearer for validation, reporting, and authoring.

### 2.3 Current Linear-Support Crate Implements Generic Linear Springs

- **FACT:** `core/solver/linear_supports` declares `SupportFamily::Spring` but
  no constant-effort or variable-hanger support family
  (`core/solver/linear_supports/src/lib.rs:17-25`).
- **FACT:** `LinearSupport::spring(...)` takes one node, one DOF, and an
  optional `SupportQuantity` stiffness (`core/solver/linear_supports/src/lib.rs:183-197`).
- **FACT:** `prepare_spring(...)` requires one affected DOF and explicit
  stiffness; missing stiffness becomes `MissingSupportStiffness`, and a
  stiffness dimension mismatch becomes `InvalidSupportDimension`
  (`core/solver/linear_supports/src/lib.rs:536-573`).
- **FACT:** unit tests cover the generic spring behavior and dimension check:
  missing stiffness blocks; translational stiffness is accepted for a
  translational DOF; translational stiffness is rejected for a rotational DOF
  (`core/solver/linear_supports/src/lib.rs:732-763`).
- **FACT:** `docs/SPEC.md` describes the linear support slice as anchors,
  guides, line stops, vertical supports, linear springs, and imposed
  displacement boundary data; it explicitly does not provide support stiffness
  defaults, catalog values, protected standards data, or professional/code
  compliance claims (`docs/SPEC.md:450-470`).

**Implication (`ASSUMPTION`):** current generic spring behavior can serve as
the solver primitive for part of a variable spring hanger, but it does not
record the hanger-specific evidence PRD names: constant-effort support,
variable spring hanger identity, travel range, hot/cold load evidence, or
manufacturer/source metadata beyond one generic provenance field.

### 2.4 Current Product Preview Adapts Only Generic Support Families

- **FACT:** `core/product_physics` maps preview supports with
  `family == "spring"` into `LinearSupport::spring`; otherwise six restrained
  DOFs become an anchor and other restraint sets become a guide
  (`core/product_physics/src/lib.rs:1067-1124`).
- **FACT:** support unit validation checks only a generic `support.stiffness`
  quantity, choosing linear versus rotational stiffness by the stiffness DOF
  (`core/product_physics/src/validation.rs:260-291`).
- **FACT:** the invented preview model currently has anchor/guide support
  examples, not spring-hanger examples (`fixtures/product_preview/invented_preview_model.json:87-91`).

### 2.5 Current Nonlinear-Support Crate Does Not Own Hanger Behavior

- **FACT:** `core/solver/nonlinear_supports` supports `OneWay`, `Gap`,
  `LiftOff`, and `Friction`, and explicitly classifies those active-set
  behaviors from supplied trial displacement/reaction facts
  (`core/solver/nonlinear_supports/src/lib.rs:30-45`, `:519-579`).
- **FACT:** the nonlinear crate states it "does not assemble or solve the global
  nonlinear system" and leaves production tolerance policy / final constraint
  strategy outside the bounded crate (`core/solver/nonlinear_supports/src/lib.rs:335-337`).
- **FACT:** `D-16` / `DEC-044` assigns the assembled nonlinear loop to a new
  PKG-04 integration tranche and keeps DEL-04-04 as the classifier/state oracle
  (`execution/_Decomposition/SOFTWARE_DECOMP.md:614`).

**Implication (`ASSUMPTION`):** treating constant-effort hanger behavior as a
nonlinear support would over-couple D5 to the D6 assembled loop unless the human
explicitly chooses that path. A minimal R4 D5 can remain user-entered support
data + load-side/review evidence, with D6 consuming it only where necessary.

### 2.6 Binding Boundaries

- **FACT:** project invariants require no protected standards text/tables,
  no bundled public defaults for code-specific values, no silent defaults for
  missing solve-required values, solver mechanics separated from rule-pack and
  professional judgment, and deterministic verification tests for solver
  changes (`docs/CONTRACT.md:23-35`, `:47-50`).
- **FACT:** SOW-011 owns anchors, guides, line stops, vertical supports,
  springs, and imposed displacements in DEL-04-03; SOW-012 owns one-way,
  lift-off, gaps, and friction in DEL-04-04 (`docs/_Registers/ScopeLedger.csv:12-13`).
- **FACT:** DEL-04-03's local context currently names "anchors, guides, line
  stops, vertical supports, springs, and imposed displacement boundary data" as
  its implementation scope (`execution/PKG-04_Solver Core and Numerical Methods/1_Working/DEL-04-03_Linear support and restraint models/_CONTEXT.md:9-14`).

---

## 3. Open Questions Awaiting Ruling

1. **Sufficiency:** Does existing `support_type = spring` / `SupportFamily::Spring`
   satisfy the PRD R4 "Spring hangers" deliverable, or is a distinct hanger
   model required? `TBD`.
2. **Families:** If distinct, are the required families exactly
   `variable_spring_hanger` and `constant_effort_support`, matching PRD §11.4,
   or should they be modeled as subtypes of generic `spring`? `TBD`.
3. **Mechanics seat:** Does a variable spring hanger enter the assembled system
   as a linear spring plus user-entered preload/load-side evidence, or as a
   richer support element? `TBD`.
4. **Constant-effort seat:** Is a constant-effort support represented as
   user-entered constant vertical load plus travel/provenance review evidence,
   or as a nonlinear support behavior consumed by the future D6 loop? `TBD`.
5. **Schema shape:** Are generic `Support.properties` map fields sufficient, or
   should the schema add named hanger fields so validation/reporting can reject
   missing hot/cold/travel/manufacturer data deterministically? `TBD`.
6. **Validation bar:** Is an invented preview-path visibility/provenance slice
   enough for D5, or must D5 include a mechanics benchmark for variable spring
   and constant-effort support before being considered landed? `TBD`.

---

## 4. Options

### Option A - Existing Generic Spring Is Sufficient For R4 D5

Treat the existing generic `spring` support as the R4 spring-hanger deliverable.
D5 would land a preview example using `family = "spring"` and explicit stiffness,
then surface it in the app/report evidence. No new support type, schema slot, or
constant-effort behavior is added.

- **For:** smallest build; reuses proven `linear_supports::Spring`; no schema
  churn; no risk of inventing vendor-specific hanger fields.
- **Against:** PRD §11.4 names "Translational spring", "Rotational spring",
  "Constant-effort support", and "Variable spring hanger" as distinct support
  types, and PRD §22.5 names "Spring hangers" as an R4 deliverable
  (`docs/PRD.md:476-489`, `:1227-1232`). Option A leaves constant-effort
  support and hanger-specific travel/hot-cold-load evidence absent.
- **Risk:** low implementation risk, high scope-acceptance risk.
- **Boundary:** code-neutral and safe, but likely under-specified for the PRD.

### Option B - Minimal Dedicated Spring-Hanger Model (Recommended)

Add a bounded D5 model for **variable spring hangers** and
**constant-effort supports**. This model remains user-entered and code-neutral:
no vendor catalog selection, no protected tables, no sizing formulas, and no
acceptability claims. It adds explicit data contract fields and diagnostics for
the values the user supplies.

Recommended shape:

- `variable_spring_hanger`: node/DOF, user-entered spring rate or stiffness,
  installed/cold/hot load metadata where supplied, travel range / movement
  limit, hanger source/manufacturer/provenance, and load-side/preload review
  evidence.
- `constant_effort_support`: node/DOF, user-entered constant support load,
  travel range, source/manufacturer/provenance, and load-side review evidence.
- validation emits missing/invalid hanger diagnostics rather than defaults;
  reports disclose hanger provenance and user-entered values.
- mechanics consumption starts conservatively: variable spring hanger consumes
  the existing linear spring primitive where a stiffness is supplied;
  constant-effort support is represented as user-entered load-side evidence and
  not as a converged nonlinear behavior unless D6 later consumes it.

- **For:** matches PRD's distinct support-type wording without opening catalog
  sizing; keeps the implementation bounded to DEL-04-03 + app/report evidence;
  preserves `D-16`/D6 ownership for any future nonlinear assembled behavior;
  gives D8/D9 explicit provenance and diagnostics.
- **Against:** requires schema/app/core/report/test updates; introduces new
  support vocabulary that must be versioned and validated; constant-effort
  mechanics are only a conservative user-entered load/review path until a later
  assembled-solve tranche consumes them more deeply.
- **Risk:** medium build risk, low boundary risk.
- **Boundary:** strong. Every engineering value is user-entered or private;
  missing values block/review; no proprietary support catalog or code-derived
  load-variation rule enters the public repo.

### Option C - Full Hanger Design/Selection Module

Build a richer spring-hanger module that computes or selects variable/constant
hanger properties, load variation, travel checks, and manufacturer-specific
metadata.

- **For:** closest to commercial piping-support workflows.
- **Against:** out of proportion for D5; likely requires vendor/private data,
  code or company-standard criteria, and additional validation sources not
  currently in the repo; risks violating the no-default/no-protected-data
  boundary unless heavily gated.
- **Risk:** high scope, IP, and validation risk.
- **Boundary:** unacceptable unless all catalogs/criteria are private user
  inputs or rights-cleared; not recommended for the public R4 path.

### Option D - Defer Dedicated Hanger Behavior To D6/D9

Do not open D5 now. Record generic spring support as a placeholder and wait for
the D6 assembled nonlinear solve / D9 validation work to decide hanger behavior.

- **For:** avoids premature data-contract work before the D6 loop exists.
- **Against:** PRD R4 lists spring hangers as their own deliverable; waiting for
  D6 leaves D5 unresolved and can turn a scoped support-model question into a
  product-hinge dependency.
- **Risk:** medium schedule risk; medium scope-acceptance risk.
- **Boundary:** safe but defers the decision rather than resolving it.

---

## 5. Recommendation - `PROPOSAL`

Adopt **Option B: minimal dedicated spring-hanger model**.

Rationale:

1. PRD §11.4 lists variable spring hanger and constant-effort support
   separately from generic translational/rotational springs, so Option A is too
   weak unless the human explicitly accepts that narrowing.
2. The current `linear_supports::Spring` primitive is useful, but it lacks the
   data vocabulary needed for hanger-specific evidence: hot/cold load metadata,
   travel range, constant-effort load, and manufacturer/source provenance.
3. A bounded Option B implementation can remain code-neutral and local-first:
   it records user-entered values, validates units and completeness, surfaces
   provenance in reports, and avoids catalog sizing or protected/default
   support values.
4. Constant-effort mechanics should not be silently approximated as a generic
   spring. The D5 slice should disclose it as a user-entered constant support
   load / review path unless and until a later D6/D9 tranche proves a deeper
   assembled-solve consumption.

Recommended acceptance wording:

> D-15 approved: adopt Option B. R4 D5 requires a minimal dedicated
> spring-hanger model with explicit user-entered variable spring hanger and
> constant-effort support data/provenance. Generic `spring` alone does not
> satisfy the D5 scope. No catalog sizing, protected standards values, or
> professional/code-compliance claim is authorized.

---

## 6. Downstream Impact Map

| Surface | Impact if Option B is ruled |
|---|---|
| **D5 / DEL-04-03** | Extend the linear support boundary with dedicated hanger data and diagnostics while reusing generic spring mechanics where valid. |
| **schemas/model.schema.yaml** | Add explicit support type(s) and named hanger fields or a governed named-property contract; bump schema version if required by the persistence policy. |
| **core/product_physics** | Parse/validate invented preview hanger records, normalize stiffness/load/travel units, and emit `SPRING_HANGER_*` / `CONSTANT_EFFORT_*` diagnostics. |
| **fixtures/product_preview** | Add invented spring-hanger example(s), preserving invented/no-catalog provenance. |
| **apps/desktop** | Surface hanger records in model tree, inspector/editor, viewport marker, validation evidence, native package, and report packet. |
| **reports / DEL-08-03** | Include hanger provenance, source/manufacturer notes, travel/load/stiffness fields, and boundary flags. |
| **D6 / DEC-044** | Future assembled nonlinear loop remains the owner for any deeper constant-effort or nonlinear support consumption. |
| **D9 validation** | Add invented validation/review evidence for any mechanics behavior claimed by D5; do not claim convergence from data visibility alone. |

If Option A is ruled, D5 becomes a much smaller generic-spring preview/provenance
slice. If Option C is ruled, the project likely needs a new protected-data /
private-library decision before implementation. If Option D is ruled, D5 stays
blocked and D6 becomes the next implementation item by necessity.

---

## 7. Authority And Ruling Record

Only the **human project authority** rules on D-15. Agents prepared this packet
and may not certify, approve, or adopt it.

**Ruling:** approved Option B on 2026-06-21.

**Ruling record:** `DEC-049` in
`execution/_Decomposition/SOFTWARE_DECOMP.md` §12.

Accepted wording: R4 D5 requires a minimal dedicated spring-hanger model with
explicit user-entered variable spring hanger and constant-effort support
data/provenance. Generic `spring` alone does not satisfy the D5 scope. No
catalog sizing, protected standards values, hidden defaults, or
professional/code-compliance claim is authorized.

This packet creates no lifecycle transition, release-readiness claim,
professional approval, certification, sealing, authentication, or
code-compliance acceptance.
