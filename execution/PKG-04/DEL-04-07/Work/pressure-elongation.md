# Pressure elongation switch (RT-L2)

Design for the v1.0 pressure-elongation option on the exact pressure route
(`3.0.0/exact_pressure_v3`). Governing commitments: `docs/RELEASE_TARGET_v1.0.md`
RT-L2 and [pressure-contract.md](pressure-contract.md) D-3.

## 1. Rulings this design implements

The owner ruled on 2026-11-09 (relayed by Agent 0), consistent with RT-L2 as written:

1. Pressure elongation is **off by default**; the user switches it on.
2. When on, it applies in the **operating** load cases only, never in sustained ones, so it
   appears in the operating-minus-sustained difference (the placement industry practice
   calls Bourdon elongation "to expansion").
3. Existing documents get **no compatibility or migration machinery**; a contextual note is
   enough. Our own fixtures and reference models are rebuilt or re-run, with the setting
   explicit, by their maintained producers.

## 2. What the route does today

Every pressurised case on the exact route applies the closed-end free pressure strain

> ε_p = (1 − 2ν) · p · A_i / (E · A_s)

in Lamé form on the pressure bore (A_i = π r_i², r_i = OD/2 − t_r; A_s the member's
section area; signed p), on straight members and realized arcs, in every pressurised
state including sustained ones. The engine has no load-case kinds: `kind` is a desktop
string the solver ignores, and sustained and operating cases differ only in what the user
authored.

- **Straight members** carry no separate free-strain term. Inside a region the end caps
  are pre-cancelled at internal nodes and each member carries the Poisson eigenload pair
  `±κ·p·A_i` with κ = 2ν (`pressure_exact.rs` `eigenload_pair`; source group
  `poisson_eigen`, coefficient `sign·2ν`). Since 2ν = 1 − (1 − 2ν), this pair is the cap
  pair −c plus the free-strain push K·u_free(ε_p).
- **Realized arcs** carry the free strain as its own push, `K_b·u_free(ε_p)`
  (`LoadFormula::PressureFreeStrainCurved`, `lib.rs` `add_curved_bend_pressure_equivalent_load`),
  with the cap part `−c_b` in source group `bend_cap_removed`; recovery subtracts
  `u_free(ε_th + ε_p)` (`bend_pressure::recovery_strain`).
- **Thrust** at unbalanced locations is separate and is not part of ε_p: `terminal_cap`
  at non-transferring terminals, `kink_remainder` at bend junctions, `joint_effective_area`
  and `joint_face_cap` (p(A_e − A_i)) at expansion joints, and elastic tie rods as plain
  connector members (RT-K4).
- **Wall and effective force.** Arcs publish N_w = N_el + p·A_i and S = N_el. Straights
  publish the same law through the member force N_m of the eigenload solve:
  N_w = N_m + 2ν·p·A_i, S = N_m + (2ν − 1)·p·A_i, that is N_el = N_m + (2ν − 1)·p·A_i
  (`source_geometry.rs` `recover_wall_effective`; certified twin `certified_publication.rs`).

## 3. The setting

### 3.1 Document member

`model_settings.pressure_elongation`, a closed record:

```json
"model_settings": { "pressure_elongation": { "operating_load_cases": ["load:operating-1"] } }
```

- `operating_load_cases` is required, non-empty, and holds unique ids of the document's
  own `load_cases[]`. These are the operating cases in which pressure elongation applies;
  every other case solves without it. The engine has no case kinds, so the user names the
  operating cases: nothing is inferred from `kind`, labels or combination roles.
- **Absent means off**: no case includes pressure elongation. The product never writes an
  empty record; switching off removes the member (`clear_model_setting`), as the
  `shear_model` precedent never writes its default.
- **Refusals by name** (document validator, solve validation and applier):
  - `MODEL-SETTING-PRESSURE-ELONGATION-WITHOUT-PRESSURE-CONTRACT` when the document has no
    `pressure_contract` (there is no pressure to elongate);
  - `MODEL-SETTING-PRESSURE-ELONGATION-CASE-UNKNOWN` for an id that names no load case;
  - `MODEL-SETTING-PRESSURE-ELONGATION-CASES-INVALID` for an empty or duplicated list.
  Deleting a load case the setting names is refused by name until the user removes it from
  the setting, as other model references are.
- **Version.** The member joins model document 0.9.0 without a version step (DEL-02-01
  `Work/model-document-0.9.0.md` §3.1 amendment). The member is optional and absent means
  off, so every saved 0.9.0 document still opens and solves; one with a pressure contract
  now solves without pressure elongation, and the desktop says so (§3.3). Writing the
  member changes the model hash, as any member does.

### 3.2 How it reaches the engine

The whole document already travels to the solve. `ModelSettings` (`model_settings.rs`,
`deny_unknown_fields`), the operation applier's document validator (allowed members) and
`model_settings_authoring.rs` gain the member; the schema `$defs/ModelSettings` gains it;
TS `types.ts` gains it. Per case, the exact route reads one boolean,
`elongation_included(case) = setting lists case.id`.

### 3.3 Desktop

- The model settings section (`ModelSettingsSection.tsx`) gains **Pressure elongation
  (Bourdon)**, shown only on a document with a pressure contract: an off/on switch and,
  when on, a checklist of the document's load cases titled "Operating load cases". Queueing
  "on" requires at least one ticked case; nothing is pre-ticked. Intents use the existing
  `set_model_setting` / `clear_model_setting` route on the project, so undo and drafts are
  the existing ones.
- **Contextual note** while the member is absent on a pressure document: "Off (default).
  Pressure elongation is not included in any load case. Documents saved before this
  setting existed included it in every pressurised case; turn it on and name the operating
  cases to include it there." No other compatibility path exists.
- **Results label**: each case's result header states "Pressure elongation included" or
  "Pressure elongation not included"; a combination states nothing of its own (its rows are
  formed from its cases' rows).

## 4. Mechanics in each setting

Let κ be the straight member's Poisson-group coefficient: κ = 2ν when the case includes
pressure elongation, κ = 1 when it does not.

### 4.1 Not included ("off")

Removes the free pressure strain **only**:

- **Straights:** the `poisson_eigen` group coefficient is `sign·1` (the cap pair −c
  alone). Inside a region these pairs cancel with the internal caps, and with
  `terminal_cap` and `kink_remainder` at transferring terminals and bend junctions, so what
  loads the system is exactly the unbalanced thrust: p·A_i at non-transferring terminals
  and the joint's p(A_e − A_i). The source-grouped ledger keeps its groups (with the new
  coefficient) so evidence stays comparable.
- **Arcs:** no `PressureFreeStrainCurved` push; recovery subtracts `u_free(ε_th)` only;
  `bend_cap_removed` stays.
- **Unchanged:** every thrust source of §2 (terminal caps, kink remainders, joint terms,
  tie rods), the pressure stress terms published today, and the pressure contract's region
  and closure rules.

### 4.2 Included ("on")

Exactly today's mechanics (κ = 2ν on straights, the `PressureFreeStrainCurved` push and
`u_free(ε_th + ε_p)` recovery on arcs), in the named operating cases only. ε_p keeps the
Lamé form on the pressure bore. A thin-wall form on the mean diameter (σ_L = p·D_m/(4t))
gives a larger strain by (r_m / r_i)²; the difference is disclosed in the included
statement (§5).

### 4.3 Wall and effective force, both settings

One law for both settings and both member kinds:

> N_w = N_el + p·A_i,  S = N_el

On a straight, with the member force N_m of the eigenload solve:
N_el = N_m + (κ − 1)·p·A_i and N_w = N_m + κ·p·A_i. With κ = 2ν this is today's straight
recovery; with κ = 1 it is N_w = N_m + p·A_i, S = N_m, today's arc form. On an arc,
N_el is recovered with or without ε_p in `u_free` as §4.1–4.2 state. The certified
publication twin follows the same κ.

**Closed-form controls** (anchored–anchored closed straight, uniform p, no thermal): not
included: anchor reactions 0, N_el = 0, N_w = p·A_i, S = 0. Included:
N_el = −(1 − 2ν)·p·A_i, N_w = 2ν·p·A_i, anchor reactions ∓(1 − 2ν)·p·A_i. The
operating-minus-sustained difference of an included operating case and a not-included
sustained case with the same p is the elongation response alone.

### 4.4 Out of scope, unchanged

- **Bourdon opening** of bends, pressure stiffening of k/SIF and ovalization stay excluded
  and disclosed (D-3); users enter their own k and SIF.
- **Hydrotest (H1).** No hydrotest solve path exists (`test_condition` and the legacy
  `hydrotest` primitive are refused). The setting does not touch H1; a case written as an
  ordinary pressurised case is included only if the user names it.

## 5. Publication

- **Identity.** The meaning of a stored pressure result changes (default excluded, inclusion
  per case), so the ordinary semantic identity moves from `pressure-2` to `pressure-3` and
  `pressure-2` is refused by name (`SOURCE_PRESSURE_2_RETIRED`), following the
  `pressure-1` → `pressure-2` precedent ([u4b-u5b-implementation.md](u4b-u5b-implementation.md)).
  Raw/component (0.2) and canonical (0.3) result formats are unchanged.
- **Formulation limitations.** The D-3 sentence keeps "bend opening under pressure (Bourdon
  effect)" among the "Not modelled" terms, and one of two fixed statements is added:
  - "Pressure elongation is not included in any load case: the closed-end free pressure
    strain is omitted; pressure thrust at unbalanced locations is applied."
  - "Pressure elongation is included only in the operating load cases the model settings
    name, as the closed-end free strain (1 − 2ν)·p·A_i/(E·A) in Lamé form on the pressure
    bore; a thin-wall mean-diameter form gives a strain larger by (r_m/r_i)²."
  Both are fixed strings, so the report's hash-pinned statement table
  (`exactReportLimitations.ts`, `fixtures/reports/exact_route_limitations.json`,
  `exact_report_statements.json`) gains exactly these two.
- **Per case.** Each case's pressure evidence carries `pressure_elongation: included |
  not_included`, which the desktop shows as the §3.3 label.

## 6. CAEPIPE MBF mapping

MBF `OPPRES` field 2 is the Bourdon flag (0 or 1 today; other values refused with
`MBF-IMPORT-FIELD-UNDOCUMENTED`).

- **Import, field 2 = 0:** the setting stays absent. This now matches the source exactly:
  the importer puts pressure in its sustained case (`case:mbf-sustained`), which until now
  solved with elongation the source had switched off.
- **Import, field 2 = 1:** stays listed as `MBF-IMPORT-PRESSURE-EFFECTS-OMITTED`, with the
  reason stated: the importer forms a sustained case carrying pressure and a thermal-only
  expansion case, and no operating case, so "elongation in the expansion range" has no
  case to be named in. Representing it (for example an imported operating case) is MBF-lane
  work after this switch. Field 3 (bend pressure correction) is unchanged.
- **Write:** field 2 = 1 when the setting is present, 0 otherwise; the writer stops
  classifying `model_settings` wholesale as omitted input-validation settings, and its
  OPPRES note is corrected (today it says the exact route models neither effect, which is
  wrong for elongation).
- Both twins (Python and Rust) change together with their vectors. This lands after PR 197
  (which rewrites the same OPPRES note and vectors), coordinated with the MBF lane; if PR 197
  has not merged when the rest is ready, the MBF change follows as its own PR so it cannot
  hold the switch.

## 7. Re-qualification plan

The default change moves every pressurised exact-route result, so:

1. **Regression anchor (on = today).** Each existing pressure reference and fixture is
   re-produced by its maintained producer with `pressure_elongation` naming **all** its
   pressurised cases; its physical numbers must be bit-identical to the current committed
   values (only identity, setting and label bytes change). This proves "included" is
   today's mechanics exactly.
2. **New off controls.** Closed-form controls of §4.3 on straights; an arc control
   (no ε_p in the arc); a joint control showing p(A_e − A_i) unchanged with elongation off;
   a mixed O/S pair showing the O − S difference equals the elongation response; certified
   and ordinary publication agree in both settings.
3. **Fixtures and references** (re-produced, setting explicit): `validation/references`
   pressure sets (t4_u4a/u4b/u5a/u5b/u6/u8, t4_i7/i8/i12, rt_a3_*, w15_rigid_scaling,
   o4_retained_source), `validation/qualification/fixtures/exact_pressure` (MANIFEST),
   `fixtures/results/pressure_*`, `fixtures/product_preview/pressure*`,
   `fixtures/reports/invented/**`, desktop replay/reopen fixtures, the semantic-contract
   table `fixtures/results/semantic_contract_v0_3_pressure_3.json`, reader verifiers
   (`tests/verifiers/pressure_2.py` successor, `test_pressure_arc_readers.py`,
   `qualification_exact_pressure.py`). Frozen persistence records and the owner-reserved
   native-smoke `expected.json` are not touched (native smoke carries no pressure).
4. **Retained pricing.** `product_physics` and `frame_kernel` are priced crates and the
   pressure table digest is a reviewed input, so the registered profile is re-priced with an
   actual-source D3 audit.
5. **AC-004.** `product_physics` changes, so AC-004 runs once on the exact candidate in a
   coordinator-scheduled window.
6. **Review.** Numerical meaning changes: targeted regression checks (items 1–2) and
   independent numerical scrutiny; persistence review of the member, its refusals, undo and
   hash; hosted CI green on the exact head.
