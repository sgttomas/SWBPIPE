# Imposed displacement on the pressure-free route

This design builds FR-SOL-006 (support and anchor movement, including nozzle
movement) on the pressure-free route (PF). It also lets the W-9 settlement
benchmark close FR-SOL-002 on PF. It is W-16 slice a in
`docs/COMPLETENESS_MAP.md` §4.3.

**Ruling.** Owner, 2026-10-23, verbatim: "Row 2 I approve option A". The
ruling is quoted in DEL-04-07 `Design/pressure-contract.md`, "Imposed
displacement on the pressure-free route". It builds the feature in two stages.
Stage 1 maps a support-targeted primitive into the partitioned reduction of
prescribed values that the exact route already uses. Stage 2 makes the
nonlinear active-set loop measure gap closure and lift-off relative to the
imposed motion. This document specifies stage 1. Stage 2 has its own design
(§6).

Paths are relative to `projects/chirality-piping/`. PP is
`core/product_physics/src`.

## 1. What the user authors

A primitive load in a load case uses the existing wire shape, which the schema
defines (`schemas/model.schema.yaml` ~1456-1474) and the desktop already emits
(`LoadCaseManagerPanel.tsx` ~1713):
- `category` is `imposed_displacement`;
- `target` is `{type: "support", support: <support id>, dof: <DOF>}`;
- `direction` names the same DOF;
- `dimension` is `displacement` for `UX`, `UY` and `UZ`, and `rotation` for
  `RX`, `RY` and `RZ` (PP `lib.rs` ~14420-14444; desktop
  `primitiveLoadVocabulary.ts` ~52);
- the magnitude is a quantity with an explicit unit. It is normalized to
  metres or radians by the existing conversion path (PP `lib.rs` ~9663-9678),
  as the exact resolver does (`case_state/resolve.rs` ~1067-1084).

`target.dof` and `direction` must agree.

A nozzle movement is a six-DOF anchor carrying up to six such primitives in one
case. Thermal growth of the equipment is the user's input; the product supplies
no nozzle or equipment movement.

## 2. Meaning

In each case, every primitive becomes one prescribed value g on the global DOF
of its support's node. The system is the partitioned reduction:

  K_ff·u_f = f_f − K_fc·g_c,  u_c = g_c,  R_c = (K·u − f)_c.

The exact route already solves this for `boundary_motion` through the shared
implementation:
- prescribed values are formed at PP `lib.rs` ~4998-5008;
- the reduction runs at ~5042-5050;
- the structural solver receives them in both modes at ~7328-7348;
- the full u is rebuilt at ~5355-5362;
- the unreduced reactions are formed at ~4781-4788.

That is the mechanical meaning. Published reactions keep the existing
corrected-recovery semantics (`certified_publication.rs` ~358-366).

**Plumbing.** PF reaches this shared solve, but today it supplies no motion:
its case calls pass `load_state: None` (~2984, ~3051), and prescribed values
come only from `load_state.prescribed`. Stage 1 therefore:
- deserializes the support target (a `LoadTargetInput::Support` variant);
- resolves and validates its rigid global DOF;
- carries a separate optional PF prescribed map into both the reduction and
  the structural solve.

It does not fabricate a load/reference-state case. It does not also add the
motion as a force, because the solve already applies −K_fc·g.

**Unaffected cases.** A case with no motion primitive takes the existing `None`
path and gains no diagnostic. Its bytes stay unchanged except for the declared
text and pin changes of §4.

**Superposition.** For fixed stiffness and a fixed boundary partition, motion
superposes with thermal, weight and every other supported contribution in its
case (~4297-4319). Where a fixed installed preload or reference load is
present, motion superposes about that baseline, and summing complete case
responses also sums their baselines. In the shared mechanics, a connector's
`q_ref` (~4613-4617) is an example, though connectors are admitted on EX v3
only.

**Combinations.** PF combinations take motion cases under their existing gates
(`preview_physics.rs` ~791-818). Linear combinations combine signed rows.
Subtraction stays a labelled state difference. A range envelope keeps its
row-envelope meaning rather than representing one equilibrium state. Mixed
modulus bases and nonlinear models stay gated, and constant-effort
combinations keep their factor-sum restriction. Signed support components and
magnitudes already have dedicated combination handling (~984-1046), so no
motion-specific gate is needed.

## 3. Refusals

PF diagnoses each case by name. As today, any blocking input diagnostic
withholds the whole document's results: the blocked envelope has `results: []`
(PP `lib.rs` ~2991, ~3058, ~14183). Stage 1 adds no partial publication.

| Code (blocking) | When |
|---|---|
| `IMPOSED_DISPLACEMENT_SUPPORT_UNKNOWN` | the target support does not exist |
| `IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID` | the target is a constant-effort device (a nonlinear field included). Until R-c, spring-family records without a nonlinear field remain refused on every DOF; those with a nonlinear field remain refused on their spring axis and nonlinear DOF. These targets receive `IMPOSED_DISPLACEMENT_SUPPORT_NOT_RIGID`. Under stage 2 (`pf-nonlinear-imposed-motion.md`) a nonlinear record's own DOF moves its ground (R-b) and its rigid restraints take base values (R-d) |
| `IMPOSED_DISPLACEMENT_DOF_UNRESTRAINED` | the target is a rigid record (no spring family, constant-effort device or nonlinear field) and the DOF is not among its rigid restraints. A non-acting DOF on a nonlinear record receives `IMPOSED_DISPLACEMENT_NONLINEAR_DOF_MISMATCH`. |
| `IMPOSED_DISPLACEMENT_DOF_MISMATCH` | `target.dof` and `direction` disagree, or a DOF token is invalid |
| `IMPOSED_DISPLACEMENT_DOF_DUPLICATE` | two primitives in one case address one base, even with equal values. Duplicates are checked per base: (node, DOF) for rigid targets and (support, DOF) for spring and nonlinear targets. No map insertion may overwrite another. |
| `IMPOSED_DISPLACEMENT_COINCIDENT_RESTRAINT` | another rigid restraint holds the same global node DOF |
| `IMPOSED_DISPLACEMENT_WITH_NONLINEAR_SUPPORTS` | retired by stage 2's R-a (`pf-nonlinear-imposed-motion.md`); it refused any motion primitive, including a zero one, in a model with a nonlinear support |
| `IMPOSED_DISPLACEMENT_DIMENSION_INVALID` | the dimension does not match the DOF, the unit is incompatible, or the magnitude is not finite. This named check runs ahead of the generic unit checks (`validation.rs` ~909), so a test of an incompatible unit asserts this code. |
| `IMPOSED_DISPLACEMENT_TARGET_INVALID` | a support target with any other category, or `imposed_displacement` on a node or element target |

**Rigid target and restrained DOF.** These checks reuse the exact resolver's
rules (`case_state/resolve.rs` ~61-63, ~1050-1064). A flexible connected member
does not make a rigid support non-rigid.

**Allowed alongside motion.** Each of these is a regression case, and none is
a refusal:
- an elastic spring or a W-15 rigid component at the same node, provided no
  other rigid restraint holds that DOF. A connected elastic member is not a
  coincident rigid restraint. Existing route gates still apply: PF refuses
  objective connectors (`PREVIEW_CONTRACT_VERSION_MISMATCH`,
  `pressure_runtime.rs` ~267-285; `tests/exact_route_refusals.rs` ~238), and
  that refusal is unchanged;
- a different DOF of the same node;
- an unrelated constant-effort support. It contributes a fixed nodal force
  (PP `lib.rs` ~13487-13495), does not select nonlinear solving (~2869), and
  keeps its consumption rules and combination gates.

**Order of checks.** The named coincident and duplicate checks run before the
generic repeated-rigid-DOF refusal in boundary preparation
(`core/solver/linear_supports/src/lib.rs` ~634-649).

**The exact route.** EX refuses a support-targeted primitive with
`EXACT_IMPOSED_DISPLACEMENT_PRIMITIVE_UNSUPPORTED`, with or without the
load/reference-state group, and names `analysis_state.support_states[].boundary_motion`
as the exact route's form. The check scans the authored primitives before
effective-case selection, so an unlisted primitive cannot escape it. This
refusal prevents unintended admission once the shared target type gains the
support variant.

**The retained sub-route.** It refuses the support target at D1.7
(`retained_memory.rs` ~971-998), and its census gains the variant (~433-436).
Its existing guards on prepared imposed contributions stay
(`retained_product.rs` ~1915, `source_recovery.rs` ~903).

**The old code.** `IMPOSED_DISPLACEMENT_REQUIRES_LOAD_STATE_MODEL` is retired,
and its regression at `tests/preview_physics_runtime.rs` ~1043-1053 is
replaced. Its pre-parse path (`preview_physics.rs` ~1087-1138) must not
intercept the EX refusal.

## 4. Text and contract changes

Each change below is stated, and its carriers are regenerated by their writers.

- **PF `LIMITATIONS[1]`** (PP `preview_physics.rs` ~58) drops "and so is a
  support displacement imposed outside a load-state model". There are 34
  exact-string carriers, including both preview semantic tables
  (`semantic_contract_v0_3_preview_physics_2.json` ~1321,
  `semantic_contract_v0_3_preview_physics_retained_1.json` ~1337), the
  results schema, fixtures, and DEL-04-08 `b7-plan.md` ~283.
  `semantic_contract_tables.py derive` replaces preview slots 2-4 only (~162,
  ~276, ~295), so slot 1 is edited in both maintained tables by hand. Then
  schemas are derived and carriers repinned.
- **DEL-04-07 `pressure-contract.md`** ~186-187 ("Until stage 1 lands ...")
  becomes the stage-1 statement.
- **W-9 README** (`validation/references/w9_benchmarks/README.md` ~182 and
  ~184-185): both EX-only statements become "both routes", and its
  `SHA256SUMS` is regenerated. Observations tied to the old revision are kept
  as such.
- **K-D5 §0 M1** (DEL-04-01 `kd5-nonlinear-active-set.md` ~59) keeps its
  conclusion: the nonlinear route still cannot receive imposed displacement
  until stage 2. Its explanation that preview refuses imposed displacement
  everywhere is corrected.
- **Completeness map**: rows FR-SOL-002, FR-SOL-006, §12.5 and §22.2 (~178,
  ~182, ~193, ~196).
- **ScopeOfWork, through the owner's wording grant.** DEL-04-03 ~294 reads:
  "Imposed displacement boundary data may interact with load-case handling in
  PKG-05, but this deliverable currently implements the support-side boundary
  data and frame-kernel prescribed-displacement reduction only." Proposed
  replacement: "Imposed displacement boundary data may interact with load-case
  handling in PKG-05. This deliverable implements the support-side boundary
  data and frame-kernel prescribed-displacement reduction; the
  pressure-free product route maps support-targeted primitives into that
  reduction (W-16 slice a, Design `pf-imposed-displacement.md`)." It needs an
  independent review of the exact text and confirmation before it is applied. DEL-09-01
  RQ-002 does not name a route, and DEL-04-07 and DEL-05-01 already own the
  composition and interface, so none of them changes.

## 5. Verification

- **W9-C settlement on PF** in both modes. Its frozen rows are reused
  unchanged: its force-method compatibility F·R = d does not depend on the
  route (`w9_reference.py` ~173-195), and its unloaded out-of-plane problem
  makes the PF/EX shear-modulus difference irrelevant. The test gains a PF
  builder; its `request()` asserts today that no motion reaches Preview
  (`tests/w9_solver_benchmarks.rs` ~248).
- **Nozzle movement.** A new frozen reference
  (`validation/references/w16_imposed_motion/`) comes from an exact-rational
  generator that runs no product code.
  - **Model.** An invented L-frame anchored at both ends. One anchor moves in
    three translations and one rotation.
  - **Method.** A six-redundant force-method (complementary-energy) closed
    form. Release the moved anchor and form
    F_ij = Σ_e ∫ (n_i·n_j/EA + m_yi·m_yj/EI_y + m_zi·m_zj/EI_z + t_i·t_j/GJ) ds
    from statics on the released frame. Then R = F⁻¹(g − d_T). The fixed
    anchor's actions follow by equilibrium, and the interior motions by member
    integration.
  - **Cases.** Motion alone, thermal alone, and both together.
  - **Cross-check.** An independently written direct-stiffness calculation.
  - **Matching.** E, G and the section theory match the product's on both
    routes, with Euler–Bernoulli explicitly on EX.
  - **Criterion.** 1e-9 relative, with the scaled-zero rule
    (`w9_solver_benchmarks.rs` ~17-19).
- **Route parity.** PF against EX `boundary_motion` for the same model and
  motion, in both modes. This supplements the oracle.
- **Negatives.** Every refusal in §3, on PF. The EX refusal for listed and
  unlisted primitives, with and without the group. The retained D1.7 refusal,
  with the ordinary PF result returned after retained admission declines. A
  mixed document with one valid and one invalid case, which is refused as a
  whole document.
- **Allowed cases** (§3): node-sharing with a spring or a rigid component; a different DOF; an unrelated constant-effort support; motion
  together with ordinary forces and generated weight; non-SI translation and
  rotation inputs.
- **Combinations.** Linear combination, subtraction and range envelope with a
  motion case.
- **Byte identity.** Exceptions are limited to the declared text and hash
  changes of §4.

## 6. Stage 2 (separate design)

Stage 2 lets a model with nonlinear supports carry imposed motion, which today
the nonlinear loop cannot do. `NonlinearFrameSolveInput` carries
`base_restrained_dofs` only (`core/solver/nonlinear_integration/src/lib.rs`
~240). `active_boundary` fixes them at 0 (~1890). The support classifier
compares the absolute u_d with 0 or ±gap (`core/solver/nonlinear_supports/src/lib.rs`
~581-640). Stage 2 must carry the base prescribed values into the loop. Where a
nonlinear support's base moves, it measures the support state against
u_d − g_s and g_s ± gap. It must also shift K-D5's pass conditions (DEL-04-01
`kd5-nonlinear-active-set.md` §2.2). Stage 2 is designed first, with two
independent numerical reviews and its own benchmarks, before any code.

## 7. Retained-memory pricing (D3)

Stage 1 touches these priced files:

| File | Why | Pricing coverage |
|---|---|---|
| PP `lib.rs` | target variant, preparation, prescribed-map plumbing | `b7a_domain_model.py` ~89; `w1_reductions_model.py` ~63 |
| PP `preview_physics.rs` | retired refusal, limitation text | `b7a_domain_model.py` ~89; `w1_reductions_model.py` ~67 |
| PP `model_fields.rs` | support-target keys and schema binding | `sd_owners_model.py` ~15-47; `rules/o10_rules.py` ~2376-2405 |
| PP `retained_memory.rs` | census and D1.7 refusal | `b7a_domain_model.py` ~130-147 |
| PP `retained_product.rs` | exhaustive captured-target match (~4575) | `b7a_domain_model.py` ~89 |
| PP `self_weight.rs` | payload and identity matches (~601, ~866) | `b7a_formation_model.py` ~21 |
| PP `pressure_runtime.rs` | the EX refusal, if placed there | `b7a_domain_model.py` ~89 |

This table is not exhaustive. At the D3 turn, also check the applicable
`kd5_text_model.py` function pins. Both preview tables are static owners
(`b7a_domain_model.py` ~119).
`primitive_loads` already prepares support motion (~953-965, ~2473-2513), so it
needs no change. Re-pricing must consider any enlarged target/primitive layout,
even though D1 refuses support primitives. Excluding the variant does not show
that admitted variants' memory is unchanged.

D3 prices this in DEL-04-08 `Design/retained-pricing-rules.md` §8.23:
`case_prescribed`'s node index and `validate`'s findings vector are priced
positively on D1 (W16-NI, W16-VF), as is the node/element
`imposed_displacement` refusal D1.7's dimension-only node arm admits (W16-ID);
everything behind a support target is zero on D1 by its premise (W16-NS). The
enlarged target raises the primitive load's layout by 16 B. P_FINAL is
unchanged, and all 80 evaluations fit M, the tightest at +2,361,180,063 B
(merged dense W3). Independent review of the D3 and the AC-004 stack-headroom
witnesses are separate steps.
