# D1, the first admission domain: predicate, caps, refusal map and U8 fit

**D1 is a field-level predicate over the borrowed request.** It is evaluated allocation-free at G-A, before any W1 owner exists. Every clause is either a ruled cap (RR "U4 plan" ruling, D-1) or a family restriction that keeps the invocation inside the source paths I54/RV75 priced (the SOURCE-BLOCKS-1 legacy namespace). Anything outside D1 takes the unchanged ordinary path, with every census fact preserved privately.

**Basis:** NUM `a2c26cc885`; all source citations are identical at `dca3b65b3d` (no maintained-source change between them). **Abbreviations:** as in R/I65/u4_plan_01/PLAN.md, plus PL = P/core/loads/primitive_loads/src.

## 1. The predicate

`D1(entry, raw, request, build)` holds only if every clause below holds.

| # | Clause | Exact condition | Why it is needed |
|---|---|---|---|
| D1.0 | Caller | `entry == Entry::Direct` (PP/retained_memory.rs:308–311) | D-2: Direct only for the milestone |
| D1.1 | Build | the profile is not Missing and not Stale under D-6 (BUILD.md §2) | the strides and laws are bound to the registered build |
| D1.2 | Census | the raw census is `Complete`, `typed.dof_upper` is `Ok`, and the nested typed census (RESIDUALS T03) is complete | partial facts never qualify (retained_memory.rs:284–295) |
| D1.3 | Namespace | `model.schema_version ∈ {"0.1.0","0.2.0"}`; `model.pressure_contract` is `None`; `model.reference_configurations` is `Absent`; every `model.material_expansion_laws[i]` is `Absent`; `model.request_material_expansion_laws` is empty | the legacy source-blocks namespace: source_recovery.rs:604–608 requires "model0.1/0.2 without pressure contract or regions". With these values `pressure_runtime::is_exact` and `case_state::is_load_state` are both false (pressure_runtime.rs:77–83; case_state/mod.rs:39–41) |
| D1.4 | Invocation | `model.load_cases.len() == 1`; `model.combinations.len() == 0`; `model.components.len() == 0` | decision 6; the private producer's own scope gate (retained_product.rs:194–201, 1129–1131) |
| D1.5 | Case | for the one case: `pressure_regions` is `None`; `equivalent_static` is `None`; `modulus_basis_ref` and `modulus_basis_temperature` are `None`; `analysis_state` is `Absent` | no pressure, no generated loads, default basis only, no load-reference state (retained_product.rs:1137–1150; source_recovery.rs:604–612) |
| D1.6 | Supports | for every support: `hanger` is `None`; `nonlinear` is `None`; `family` is `None` or exactly one of `anchor`, `guide`, `line_stop`, `vertical_support`, `spring` | rigid restraints and one scalar spring only. The excluded families are hangers, constant-effort supports and nonlinear supports (PP/lib.rs:6716–6776, 6796–6823, 12005–12027). Exact-string match is deliberately stricter than the source's trimmed match |
| D1.7 | Loads | for every primitive load of the case: `target` is `LoadTargetInput::Node`; `dimension ∈ {"force","moment"}` | nodal force/moment primitives only: no element-uniform, thermal, pressure or imposed loads (PP/lib.rs:8466–8550; preview_physics.rs:1015; retained_product.rs:1137–1150). Category and direction strings are free within the text cap; an invalid one is an ordinary blocking diagnostic, priced in T08 |
| D1.8 | Members | pipe segments are straight members; no components (D1.4), so no bends, curved macro-elements or user-stiffness joints exist | retained_product.rs:1137–1140 |
| D1.9 | Caps | every fact in §2 is within its cap | ruled D-1 |

D1.3–D1.8 are read from the borrowed typed request by the extended census. That costs no parse, clone, map or graph build. Ordinary validity is not part of D1. An in-domain request that the ordinary route rejects (an unknown node reference, a non-positive spring, a bad DOF token) still runs the ordinary route, and its error prefix and diagnostics are priced (I54 BOUND `ErrorPrefix`; RESIDUALS T08).

## 2. The cap table, field by field

Raw facts come from `borrowed_value_census` over the captured raw Value (retained_memory.rs:98–166); typed facts come from the request.

| Cap | Fact and source | Value | Milestone |
|---|---|---|---|
| n | `model.nodes.len()` | ≤ 32, so N = 6n ≤ 192 | 2 (N = 12, F = 9) |
| m | `model.pipe_segments.len()` | ≤ 32 | 1 |
| g | `model.supports.len()` | ≤ 32 | 4 |
| r | Σ `supports[i].restraints.len()` | ≤ 192 | k = 3 unique |
| s | spring entries. Each D1 spring is one `LinearSupport::spring` on one DOF (PP/lib.rs:6756–6772), so s ≤ g | ≤ 192 ruled; effective ≤ 32 | 3 |
| l | `load_cases[0].primitive_loads.len()` | ≤ 192 | 3 |
| materials | `model.materials.len()`; `request.materials.len()` | ≤ 4 each | 1; 0 |
| temperature points | `materials[i].temperature_points.len()` for both lists | ≤ 16 each | 0 |
| sections | `model.sections.len()` | ≤ 32 | absent |
| text | every typed `String` and every raw string value or key: byte length ≤ 128 | ≤ 128 | max 46 |
| raw values | census `values`. A complete census already implies ≤ 16,384 (retained_memory.rs:105–108) | ≤ 16,384 | 150 |
| raw depth | census `maximum_depth` | ≤ 16 | 7 |
| raw text totals | census `string_bytes`; `key_bytes` | ≤ 65,536 each | 1,039; 855 |
| raw capacities | census `array_capacity_elements`; `string_capacity_bytes`; `key_capacity_bytes` | ≤ 32,768; ≤ 131,072; ≤ 131,072 | actual |
| digest | `captured_digest.capacity` | ≤ 128 | actual |

**Two census extensions the predicate needs** (G5, in retained_memory.rs):
- **Maximum lengths.** The census keeps totals but no maximum string or key length. Add `max_string_bytes` and `max_key_bytes`. They are allocation-free, like the existing counters.
- **The typed walk** of RESIDUALS T03. It supplies D1.3–D1.8 and the nested capacities.

**Derived quantities at the caps** (`_run_records/caps_arithmetic.out.json`):

| Quantity | Value at the caps | Limit it must respect |
|---|---|---|
| N | 192 | `DENSE_SOURCE_DOF_LIMIT` = 256 (source_recovery.rs:30); exact-boundary `dofs` = 256 (FKS/exact_boundary.rs:57–71) |
| Q = 7n+30m+s+k+2g | ≤ 1,472 | — |
| P_final ≤ 7n+51m+8g+3 | 2,115 | — |
| C ≤ 144m+s | 4,640 | — |
| Z ≤ min(N², 144m+s) | 4,640 | — |
| skyline H ≤ F(F+1)/2 | 18,528 | — |
| source_count = 144m+s+l | 4,832 | 16,384 (source_recovery.rs:514–524) |
| functional descriptors | ≤ 1,760 | — |

**Every pricing formula is evaluated at these caps.** It is first checked to be monotone nondecreasing in its counts (G3 records the per-row lemma). Fixed source limits remain in force and are never relaxed by a cap: the 6 GiB dense guard, the 24H observation guard, and the 131,073-node maximum heap.

## 3. The refusal map

Each failing clause refuses the permit. The ordinary path runs once, unchanged, and the private `RetainedAdmissionReport` keeps every fact and the first failing clause. The precondition names are the accepted schema's `unavailable_precondition` enum (`schemas/retained_precision_mp_v2.schema.json:6475–6486`; C2 CONTRACT_DELTA:72).

| Failing clause | Private reason (proposed `DomainRefusal` variant) | Precondition kind |
|---|---|---|
| D1.0 | `Caller` | `caller` |
| D1.1 | `Profile(Missing \| Stale)` | `resource_admission` |
| D1.2 | `Census(DepthLimit \| ValueLimit \| ArithmeticOverflow \| TypedIncomplete)` | `resource_admission` |
| D1.3–D1.8 | `Family(clause id)` | `source_family` |
| D1.9 | `Cap(fact id, observed)` | `resource_admission` |
| G-B or G-C fact outside its cap | `PhaseFact(gate, fact id, observed)` | `resource_admission` |
| reserved-stack spawn fails (STACK_PLAN.md) | `StackReservation` | `resource_admission` |

D1 has no `upstream_no_wrap` refusal; D4_RECONCILIATION.md §4 explains why. Whether a refused retained-direct invocation also carries a public `RETAINED_PRECISION_UNAVAILABLE` notice is U3's facade decision under C1 §2 (C1:64, 68). The map only fixes the private reason and the precondition kind U3 would use. A refusal never changes an ordinary byte.

## 4. How the milestone and the U8 witnesses fit

- **The milestone, RF-SKEW-T-CANT-OFF-122-r1e-04, satisfies every clause.**
  - Schema "0.1.0"; one load case; no combinations, components, pressure contract or regions; four rigid-or-spring supports; three nodal loads.
  - Every count, text and raw fact is inside its cap (§2; the fixture facts are from I54 BOUND:87–93 and RV75's recount, and the material counts were read from the fixture in this grant).
  - Both modes are in D1.
- **The L = 0 base** is "case 0 plus one memberless, fully restrained node" (R/I62/coverage_shared_python_01/SNAPSHOT_05_PLAN.md:86). It adds one node and one rigid support to case 0, so it is inside the caps whenever case 0 is.
  - Its memberless node passes D1, which places no connectivity condition.
  - Whether the producer admits such a node is the open producer question RR:7567 records, not a D1 question.
- **The native Ceiling witness is not yet built.** RR:7636 says it needs a case with "genuinely different numerics". The caps give 16× headroom in nodes, members and supports over the milestone. If its construction exceeds a cap, it stays ordinary until ROOT widens D1 (the plan's D-9). Widening is cheap before G5.
- **The two-load-case synthetic witness is outside D1** (D1.4). Multi-case remains wider F2a.

## 5. Limits

- D1 qualifies nothing by itself. A permit also needs G3–G6, U1 and U3.
- A predicate clause that cannot be read allocation-free in G5 is a stop item for G5, not a reason to drop the clause.
