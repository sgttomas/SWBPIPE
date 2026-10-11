# Retained pricing: the reasons for the name-keyed rules

The registered memory profile (`core/product_physics/src/retained_memory.rs`, its GENERATED PROFILE block) is produced by the retained pricing tool, [`tools/retained_pricing`](../../../../tools/retained_pricing/README.md). That tool is SQ2's TEXT + cap-priced chain. This text gives the argument for each rule the tool adds by name: `rules/o10_rules.py` and the later adaptations in `rules/sq2_decisions_*.py`. Each rule is cited here by its label in the code. A rule whose function is absent at a revision is not added, because there is nothing for it to bound.

Line numbers are those at the head named in each section. The tool resolves every key by name, or by the site's own text inside a named function, at the snapshot it prices.

## A. Rulings applied to T4's batch 1 heads

These are Agent 0's rulings. Heads: `c2f1db4814`, `8388da7a8c`, `666cb30520`.

- **R1.** `O10_EDGE_ZERO`: the edge from `semantic_contract.rs for_source` to `physics_evidence.rs validate_pressure_evidence` is zero.
  - Why: the precommit reader validates `project(successor)`. The Rust reader's `project()` sets `producer.semantic_contract_id` to the route's base id: preview-physics-1 on route L, physics-1 on route E. It is never pressure-1, so `for_source` never takes the `PRESSURE_ID` arm.
  - This is the argument of the registered G4 (T17) sibling edges.
  - Without R1, `8388da7a8c` is incomplete: 26 unmapped headers and 5 unclassified arguments, all behind this edge.
- **The v3 identity text is priced.** No rule.
  - TAV rises by exactly 23,920 B on both routes and in every variant.
  - The increase is `exact_admission.rs pressure_v3_formulation_basis` (+17,648) plus `lib.rs mechanics_producer_for_model` (+6,272).
- **The rule whose callee `UserStiffnessElement::orientation` was deleted is dropped.** The line map's prune drops `loop_bounds` `edge_zero` {caller `build_pressure_case_with_members`, callee `lib.rs:675:orientation` at the basis}.
  - The frame kernel's `lib.rs` defined `fn orientation` twice: in `impl FrameElement` (604) and in `impl UserStiffnessElement` (675).
  - a601e3706d deletes `UserStiffnessElement`, so at `666cb30520` only `FrameElement`'s definition remains, and the dropped rule's callee is the deleted one. `FrameElement`'s own zero edge is kept.
  - The definition count falls from 2 to 1, and that alone does not say which definition went. The prune therefore happens only through the reviewed entry for this key in the tool's `rules/linemap_resolutions.json`; without it the run stops.
  - Route E's `EDGE_ZERO_REMOVE_CALLER` count (51 at the basis) is reduced by exactly the pruned edges, and the reduction is logged.

## B. Connector rules (L, E, A, J), at `666cb30520`

These rules rest on three premises.

**P1 (D1.3).** `retained_memory.rs namespace_branch` admits only two forms:
- `("0.1.0"|"0.2.0", None)`;
- `("0.3.0", Some(2.0.0 / exact_straight_pressure_v2))`.

Everything else is refused. So no admitted model has `exact_contract(model) == PressureV3`.

**P2 (D1.4).** `retained_memory.rs` refuses any model whose `components` is not empty (`C::Invocation`, `F::Components`). The registered rule `model.components → 0` encodes this.

**P3 (connector origin).**
- `ObjectiveConnector`'s fields are private. `ObjectiveConnector::new` is called only by `joint::build_connector`.
- `build_connector` is called only inside `build_model_for_members`, under `exact_contract(model) == Some(PressureV3)` and in a loop over `model.components`.
- So under D1 (by P1, and independently by P2), `built.connectors` and `built.connector_records` are empty.
- Every other connector collection on the D1 graph is a borrow or clone of these, or `Vec::new()`.

| Label | Rule | Argument |
|---|---|---|
| **L1** | `for (connector, record) in built.connectors.iter().zip(&built.connector_records)` → 0, in `append_connector_results` and `add_connector_reference_loads` only | Both slices are empty (P3). |
| **L2** | `for c in &source.connectors` → 0, in `structural/formation_check.rs body_scales` only | `source.connectors` holds the built model's connectors (P3). |
| **L3** | `iter:connectors .iter().map` → 0, in the nonlinear integration `structural_adapter.rs new` only | Those connectors are the built model's clone or empty (P3). |
| **E1** (route E) | `for r in records` → 0, in `connector_evidence.rs validate` | **Guard.** `require_connector(admitted \|\| records.is_empty(), "UNSUPPORTED")?` runs before the loop, with `admitted = (contract == PressureV3)`.<br>**Callers on route E** pass `StraightV2` (`validate_physics_evidence` → `validate_physics_evidence_in(…, StraightV2)`).<br>**PressureV3 callers** are `validate_pressure_evidence` and its load-state arm `load_reference::validate_pressure_load_reference_evidence`. Both are behind R1's edge.<br>**Result:** a non-empty `records` returns Err before the loop, and an empty one iterates 0 times. |
| **E2** (route E) | `for component in spans.keys()` → 0, in `validate` | `spans` is filled only inside E1's loop. |
| **E3** (route E) | `for row in rows.values()` → `Pall`, in `validate` | This is a true count, not a zero: one pass over the validated envelope's result rows, at most C · P_final. |
| **E4** (route E) | `validate` → `row_basis` edge is zero | `row_basis` is reached only after `require_connector(admitted, …)?`. That call returns Err when not admitted (E1). |
| **E5** (route E) | `site_from` for `require_connector`'s `Err(format!("SOURCE_PHYSICS_CONNECTOR_{code}"))`, from `validate_physics_evidence`, per 1 (+306 B) | `require_connector` allocates only on Err. All 16 calls are `require_connector(…)?` with a literal code, and the error propagates with `?`. This is the registered rule for `physics_evidence.rs require()`. |
| **A1** | `id_audit`: `connector.rs` `ConnectorError` Display's `name` at its three `write!` sites → STATIC (128) | The `name` fields are `&'static str`. The longest literal is "connector displacements" (23 B). The Display rows stay priced. |
| **A2** | `id_audit`: `require_connector`'s `code` → STATIC (128) | All 16 calls pass a literal. The longest is "REPLACED_SPAN_PUBLISHED" (23 B). |
| **A3** | `id_audit`: `row_basis`'s `span` → IN128 | `span` is a replaced pipe id, at most 128 B under D1's text cap. The row has multiplicity 0 by E4, but its argument is still classified. |
| **J** | `site_size` 0 at six `joint.rs` format sites: `refuse_legacy_joints` (`what`), `decode_fields`, `model_findings` (`problem`), `classify_connectors` (`problems`) | **Why 0:** every path to these sites iterates `model.components`, so they have multiplicity 0 on both routes before any O-10 rule (P2).<br>**No `site_zero`:** the chain still computes each site's true multiplicity.<br>**Guard:** `o10_check` fails a run if any J site has a positive multiplicity or is not found. A negative control, which priced `refuse_legacy_joints`'s loop, made the guard fire. |
| **J'** | Bounds for J's composed arguments: `problem` ≤ 512 B; `problems` ≤ 8,222 B | **`problem`:** a literal of at most 60 B with up to two 128 B ids.<br>**`problems`:** up to 16 findings × 512 B, joined by "; ".<br>Unused at multiplicity 0; they keep the class honest if J's guard ever trips. |

`o10_check` also confirms that each appended header rule (L1–L3, E1–E2) matched only its listed `file:fn` functions.

## C. T4 batch 2's final head `e9cc7e3f5f`

This head adds two diagnostic functions and a test module, and no new loop. Each production site stays priced; only its arguments are classed.

| Label | Site | Class and argument |
|---|---|---|
| **N1** | `lib.rs append_retained_source_not_joined`: the id `format!("diagnostic:source-recovery:{}:not-joined", stable_suffix(case_id))` and `vec![case_id.to_string()]` | `stable_suffix(case_id)` and `case_id` are IN128.<br>A load-case id is at most 128 B. `stable_suffix` replaces `:` with `-`, so the length does not change.<br>This is the registered class for these arguments. |
| **N2** (`O10_ARGS_FIRST`) | That diagnostic's message `exact_admission::RETAINED_SOURCE_NOT_JOINED_TEXT` | The literal's length (233 B at this head), read from `exact_admission.rs` on every run. An edit of the text is priced as written. |
| **N3** | `lib.rs append_short_bend_refusal`: id, message and refs | `stable_suffix(case_id)`, `case_id`, `bend.component_id` and `bend.pipe_id` are IN128.<br>D1.4 makes the function return before allocating: its bends come only from `model.components`. It is priced anyway (+28,086 B, conservative). |
| **T** | `s11g_tests/short_bend.rs` test labels | `mode\|?` is 17 B (`SparseInteractive`); `phi` is 327 B (f64 Display).<br>Test-only (`#[cfg(test)] mod s11g_tests`): these sites are not reached and have multiplicity 0. The classes are honest, with no size override. |

D rises by 6 (two diagnostic sites at multiplicity 3): route L goes from 41,773 to 41,779 and route E from 51,938 to 51,944.

## D. Adaptations for later heads

- **`preview_namespace` (#1251).**
  - #1251 renamed the Rust reader's `legacy_namespace` to `preview_namespace`.
  - Route E's zero for it resolves whichever name the snapshot defines (`rs_any`).
  - The argument is unchanged: `if route.exact {…} else { preview_namespace }`, and route E is exact.
- **E13 (#1251): `append_combination_results` deleted.**
  - The combination engine's `append_combination_results` is deleted. `EDGE_ZERO[0]`, which zeroed its only call edge, now names nothing.
  - Its key may stay uncarried (`KEYMAP_ALLOW`). Any other uncarried decision key stops the run.
- **`g5_native_combinations`' operand loop (99cb958e85).**
  - 99cb958e85 zipped the loop with the combination's terms. Both header forms are bound at 3, one per term.
  - The pre-99cb958e85 form is kept so that the registered tree's revision (`f78f52b7e2`) still prices.


## E. T4 batch 4: U2 and O-13 at `98dba87714`

These additions close the new header and argument inventory. They do not
zero a newly reached branch. D1 still admits no components and no pressure-v3
namespace (§B P1–P3); true array and collection bounds are used even when the
chain conservatively reaches an arc helper.

| Label | Rule and code argument |
|---|---|
| **U2L1** | `exact.pipe_states` in `add_curved_bend_pressure_equivalent_load` has one entry per built pipe, at most m. |
| **U2L2/U2L3** | The thermal and pressure free-expansion helpers walk `element_dof_map` and `free_expansion`, each a 12-element array (`ELEMENT_DOF`). Both headers are bounded at 12. |
| **U2E1** | RE `arc_evidence::ROWS` is a four-element array, so its coverage header is bounded at 4. |
| **U2E2** | `build_pressure_case_with_members` constructs one junction for each adjacent checked traversal pair. The traversal has at most m pipes, so the junction list is bounded at m. |
| **U2A** | The `bend_pressure::append_arc_pressure_results` case and pipe ids are IN128. Their `.len()` spellings are integers (20 B upper). Its row kind, location and component are static tokens (each <128 B). The new exact-stress diagnostic suffixes preserve the case and pipe id lengths; the arc recovery record's member is a pipe id. The test observation's project is the D1-capped project id. `pressure_treatment` selects one of two static literals (<128 B). The newly conservatively reached `LoadLedger::push_product` source is the capped pressure region/source id. The arc guard endpoint `name` (i or j), group label and `unit` are static tokens. |
| **U2C** | `component_solver_consumption` copies a component's authored text or the static default. D1.4 excludes components, but the chain conservatively reaches this helper at a positive multiplicity. It stays priced at that computed multiplicity. Its lexical receiver is audited as IN128: the authored field is within D1's text cap, and the static defaults also fit 128 B. No edge or site multiplicity is overridden. |
| **O13L1** | `ArcRowGroup::ALL` is `[Self;4]`. The guard walks all four groups, bounded at 4. |
| **O13A1** | A fired item is bounded by the existing F_len = max(id+34+3·24, id+41+8192): numeric items fit the first/unavailable upper, and an unavailable certificate carries the 8192 B error class. Each finding has at most six named items. A sentence is bounded by 600 literal B + id (128) + count (20) + 6·F_len + 10 separator B + 31 suffix B = 50,955 B at D1. The merge receives at most the straight, arc and short-bend findings, in that order, so any prefix and the whole are at most 3·50,955 + 2 B. `composite_text.py` uses this whole-sentence bound for `evidence_line`; the per-site `merged.sentence` is classed by the same bound. The old formula remains on revisions without the arc finding, preserving the registered tree. |
| **O13T** | `s11g_tests/arc_recovery.rs` is included only with `#[cfg(test)]`; no test function is on the product graph. These arguments nevertheless receive honest bounds: mode Debug ≤17 B, f64 Display ≤327 B, project/member ids IN128, each sentence ≤50,955 B, and a copied source line no longer than the total included source bytes measured at the snapshot. No test site is assigned a fabricated zero. |

U2 deletes `connector_evidence::row_basis`; its E4 edge and A3 audit are added
only when that function is present. This avoids a null stale key without
weakening a surviving function's rule. The registered revision still carries
the original E4/A3 entries.


## F. K-D5 actual owned layouts (KD5O1–KD5O6)

These model additions apply only when the priced source defines
`MemberBoundInputs`. Pricing an older registered revision keeps its original
formula. The tool's snapshot path is passed to `ordinary_caps.py`; this is
history pricing, with no product format compatibility path.

- **KD5O1/O2 (ordinary formation peak).** `evaluate` reuses its rho backing for
  s and s₂, then drops it before `epsilon_e` allocates one reusable F-row of
  exact accumulators. The conservative three-row allowance in §6 is retained:
  +2·Xc(ExactAccumulator,F) over the old row and +8·5F bytes for psi, the
  correction right-hand side/solve scratch and epsilon's row/column sums. The
  old per-row list allowance also remains conservative. Every element/load
  re-formation and certified recovery remains stack-only; the matrix-cache
  row stays zero, subject to the source/T06 checks.
- **KD5O3 (closure owners and their children).** Every nodal term now retains
  a `LoadClosureRecord` (formed=false, manifest=None) beside its ordinary
  formation record: add V(LoadClosureRecord,l), and its exact-count scaled
  clone in W2. The empty primitive Vec inline owners in AssembledForce, FormationSource and the scaled force remain positive (`s(Vec)`); manifest headers are already inside each closure-record stride. D1.3 admits only the preview/straight-pressure namespace and excludes 0.4.0, v3, expansion laws and load-state references; `lib.rs w1_route` also returns None when `case_state::is_load_state(model)`. D1.7 admits only nodal force/moment input loads; D1.5 excludes
  analysis states, equivalent-static generation and every pressure region
  (also its spare capacity); D1.4 excludes components and D1.6 hanger and
  nonlinear supports. At `e9296aae32` the new nodal producer first queries
  `certified_loads::source_factor`, which returns None unless
  `case_state::is_load_state(model)` (the exact 0.4.0 namespace). Thus every
  admitted D1 nodal load takes `ledger.push` of its normalized exact leaf,
  never `push_formed` or `attach_last`. Unit normalization precedes this
  intended-input boundary and installs no closure metadata. `NodalProduct`
  is positively stored on admitted 0.4 source-factor routes in the general
  product, but those routes are outside D1/W1. This re-audited seam, together
  with the exclusions above, establishes that no supplied LoadPrimitive,
  expected LoadClosureEntry, operand-bits Vec, intensity-product factors or
  thermal-table clone exists on the current D1 route. Their backing children, including the per-primitive temporary binding operand Vec, are priced
  at zero by these domain predicates, independently of typical fixtures.
  The positive closure-record vector remains priced. Adding one of these
  families to D1 (including B7 namespace/domain extension) requires re-auditing these guards and a new count law for both independent expected
  manifests and supplied primitives, including all identity strings,
  operand vectors and cloned thermal tables, before the profile can bind it.
- **KD5O4 (solution payload).** `MemberBoundInputs` retains f64[N] w and w₂,
  i32[N] global exponents and the inline owner: B(20N)+s(MemberBoundInputs),
  through completion and recovery. W2's evaluate includes the larger core
  twice as before.
- **KD5O5 (W2 tail).** `force_scaling_recovery` clones the complete payload into
  PreviewLinearSolve while the outcome's solution still lives. Both payloads
  are included at that peak: 2·(B(20N)+s(MemberBoundInputs)). The unscaled
  exponent vector is counted as well as w/w₂.
- **KD5O6 (recovery records and groups).** At D1, m straight members each own
  up to 7 locations ×4 groups and g supports up to 2 groups. Price
  V(RecoveryRecord,m+g), each record's separately grown group backing
  (m·PushCap(28)+g·PushCap(2))·s(RecoveryGroup), every member/support id, two
  owned labels per group at 128 B each, and the error child at
  TextCap(8192) for every group. Group allocation headers are in the actual
  in-build `RecoveryRecord`; all group fields are in the in-build
  `RecoveryGroup`. The fired-items Vec also gets V(String,28m+2g) for its separately grown backing; its formatted String children remain in TAV. No unavailable reason's storage is silently omitted.
  `member_scales` collects its paired maxima map into a second paired map.
  Both `(usize,(f64,f64))` bucket maps are priced concurrently, with no
  allocator-reuse assumption. Older snapshots retain the original paired
  map plus scalar map formula.

Every new weighted layout atom binds to `size_of` of the actual PP/FK type.
The Python ASSUMED strides remain illustrative and are not budget evidence.
Final completeness and budget claims require the implementing commit's full
pricing run and in-build atom evaluation.


- **KD5T1/T2 (unified fixed text).** `kd5_text_model.py` pins all four
  certified record constructors, their private group helper, the D1 case
  consumer and the recovery finding. This establishes that every produced
  group location/kind comes from the enumerated literal inventory, and that
  the consumer uses these records without replacing their labels. A changed
  pinned body stops pricing for a new producer/consumer review. At exact
  `e9296aae32`, the complete constructor inventory is: all, axial, bending,
  connector_local, cut_i, cut_j, end_i, end_j, force, midspan, moment, node,
  pipe_wall, quarter_1, quarter_3, recovery, shear, support_force,
  support_moment, torsion, wall_cut_i, wall_cut_j, wall_end_i, wall_end_j,
  wall_midspan, wall_quarter_1 and wall_quarter_3. The maximum is 15 B;
  even non-D1 wall/connector labels are included conservatively. Numeric
  item literal is 10 B, unavailable-item literal 38 B, and unified sentence
  literal 200 B. Member ids remain IN128, error content stays 8192 B and
  f64 exponent spelling stays 24 B. Thus F_len = 8388 B and the six-item
  sentence bound is 50717 B. Fixed route tokens are at most 30 B. K-D5 has
  one recovery sentence, with no three-sentence merge factor. Historical
  O-13/registered snapshots retain their former bounds. The four-evaluand
  support repair and comparison-unavailable detail require their actual
  next source inventory/pins before this proof can be carried forward.
- **KD5L1/L2 (group census).** The surviving `record in records` header is
  re-bound to m+g on the unified record shape; the separate per-record
  `group` header is bounded at 28 (supports have only two). This protects
  every new support record instead of retaining the former member-only m
  census. The new-header rule is limited to the two actual guard functions.

- **KD5A1 (publication identity and labels).** `certified_publication::member`
  forms a prefix from the pipe id (IN128) and its integer length. Its record
  constructors own a member/support/entity id, each taken directly from the
  D1-capped model id. `group` copies the static location and kind tokens from
  those constructors; all are shorter than 128 B. Each copy remains priced
  at its computed multiplicity. The removed `arc_recovery_record` audit is
  retained only on snapshots that still define that function.


- **KD5C1 (surviving condition-estimator call).** K-D5 renames the trait body
  `ConditionMatrix::estimate_rcond` to `estimate_condition`, returning the
  gate's rcond, inverse estimate and norm. The public `estimate_rcond` still
  delegates to that body and returns its first value. Its original basis
  callgraph rule moves to a named rule that requires exactly one public
  wrapper and one new trait method, or the original two old definitions.
  No edge or bound is pruned; the historical wrapper/trait edge is preserved.
- **KD5C2 (source-replaced identifier audit sites).** The two old recovery
  end-message sites become the unified group-message sites with the same
  IN128 member id and fixed ≤128 B location/kind labels. The old inline
  `RecoveryRecord` member copy moves to certified-publication constructors
  (KD5A1). `check` still copies its unavailable family at the existing static
  class (128 B). These four audits move from the basis line table to named
  sites; historical snapshots receive the original arguments/classes exactly.


- **KD5O7 (certified support norms).** `preview_physics::CaseRecord` now
  retains one `(String,[f64;2])` per support with certified rows. Add its
  separately grown backing V((String,[f64;2]),g), g capped support-id byte
  strings and the vector header. The inline `MemberRecord` also grew two
  certified bending rows; its existing in-build stride covers those fields.
- **KD5A2/A3 (new owned text).** The lexicon explicitly recognizes the
  certified-publication record constructors' &str-to-String conversions
  (`member`, `support`, `entity`). Their ids are IN128; kind/location tokens
  are static ≤128. The finite-a notice copies a capped case id. Summand
  family is a static token, source is at most the existing 600-B composite
  class (two capped ids and a length or pressure source tuple). Pressure
  closure identities use capped region/pipe ids and a fixed kind token.
  Unavailable-route text selects one of two ≤128-B literals. The earlier
  `formation_entity_bodies` map is no longer called on the unified graph;
  its three copy audits remain on historical snapshots where the old guard
  consumes it. A future reached copy again requires classification.
- **KD5C3 (scaling call binding).** `AssembledForce::force_scaled` maps its
  `Vec<LoadPrimitive>` with `p.force_scaled`, calling `LoadPrimitive`'s method.
  The callgraph is bound to that exact method. It never recursively calls
  the ledger method; adding a fictitious recursion allowance would conceal
  the callgraph error.
- **KD5L3–L12 (new header bounds).** Supplied primitives and per-term manifests
  are 0 on the re-audited D1 producer seam in KD5O3; trusted term/record walks
  are l. Member locations are exactly 2 ends +5 sections, station recovery
  has 3 interior stations, and section summary walks 5 sections. The notice's
  diagnostic-position scan is bounded by D. Exact-pressure closure groups
  are absent/empty under D1.5. Every appended rule names only its actual
  function, checked by O-10 after the full run.

- **KD5O8 (recovery FormationSource).** On exact `e9296aae32`,
  `certified_publication::source` clones m FrameElements and collects sp
  spring pairs for support recovery. Price this source positively and
  separately: Xc(FrameElement,m)+Xc((usize,f64),sp)+6·s(Vec), preserving
  the existing assembly/check-source allowances. Its primitive, connector,
  curved and unavailable backings are empty under the current D1 guards.
  This formula does not rely on old sources dropping or allocation reuse.
  The proposed NI recovery-view API is absent on this head; its actual
  constructor must be rechecked if it replaces this source.
- **KD5A4 (closure details).** The three pre-manifest error messages copy
  `term.source`, the D1-capped nodal load id (IN128). Those sites remain
  conservatively priced even though trusted input records continue before
  those formed-term branches on admitted D1 cases.

- **KD5L13 (pressure source-term map).** `finish_source_groups` receives only
  factor groups produced by pressure-region traversal. D1.5 admits no such
  region (absent on L, present and empty on E), so its mapped term list is
  empty. This zero rests on the producer/admission seam, not fixture counts.

- **KD5A5 (text-typed failure operands).** The inherited bare `s` rule is
  f64 Display (327 B). The new Shape, Closure and Second enum bindings are
  text and receive explicit site classes. Shape is a fixed ≤128-B literal.
  Closure's source cases are bounded by max(128+40,128+14+8192): a capped
  D1 load id with the longest ordinary suffix40, or the reformation
  prefix14 and existing 8192-B error class. The conservative site cap8352
  retains both cases. Second holds an inner subtraction/correction
  failure detail (error8192 + longest inner prefix21 =8213); those helpers
  do not produce nested Second. With each outer literal, the maximum
  returned formation detail is 8366 B; T08 prices its capacity at16732 B.
  These are positive, honest text bounds, with no changed multiplicity.

- **KD5M1 (failure detail multiplicity).** On the reviewed and source-pinned
  `formation_check::{check,evaluate,Failure::detail}` bodies, each check
  invokes evaluate once. An error either reaches the outer detail directly
  or creates one inner detail for Second and then the outer detail; the
  inner `?` exits prevent a second inner allocation. There are at most two
  detail allocations across all alternatives per actual check invocation.
  Existing positive `site_from` rules conservatively charge each of the
  seven exclusive allocation sites at two per check, retaining the full
  reached ordinary/W2/case/replay multiplicity. This is a sum-of-alternatives
  allowance, not a global cap on the function's run count.
- **KD5M2 (first closure error).** Every one of the 15 reviewed, source-pinned
  load_closure error allocation sites returns Err directly or propagates
  through `?`; success allocates none. At most one error allocation occurs
  across those sites per actual closure invocation. The positive site_from
  rules charge every exclusive site at one per closure call, preserving
  all ordinary/W2/case/replay invocation multipliers. Loop counts are still
  audited and retained; this bound applies only to the returning allocation.
  A changed producer or error-control-flow body stops for renewed review.

- **KD5A6 (unavailable detail copy).** Failure::Unavailable carries a
  private fixed &str and copies it through `(*s).into()`. The lexicon now
  includes this actual String allocation on implementing snapshots. The
  pinned evaluate body constructs only contraction, Neumann and non-finite
  Lambda sigma literals, all within a positive 128-B allowance. This site
  also retains KD5M1's per-check invocation multiplier.

- **KD5O9 (support magnitude evaluands).** A snapshot whose actual support
  constructor emits support_force_magnitude also retains four support
  groups: the force vector and its separately published magnitude, plus
  the moment vector and its magnitude. Price four group children, their
  labels/error allowances and four possible fired strings per support,
  retaining per-record PushCap(4) backing. The two physical scale families
  remain force and moment. Exact e929 predates that constructor and uses
  its actual two-group model; proposed constructors are never charged as
  already implemented. Source-pinned final constructor inventory governs
  this detection and must be reviewed when those bodies change.

- **KD5O10 (same-source coefficient vectors).** On snapshots whose actual
  certified-publication constructor calls certify_straight_stress_spans,
  retain the conservative S=2l+1 allowance. On current D1, KD5O3's
  admission/producer guards make source_count=0, so the actual constructor
  has only 0/1 and one interval, even when the nodal count l is positive.
  Thus S remains an honest positive upper without assuming a general
  primitive/source count law. FK owns an exact-capacity
  Vec<CertifiedStraightStressSpan>; PP maps it into the engine's
  Vec<EnclosedQuadraticStressSpan>. Price their simultaneous backing
  as s(FK span)·S+s(engine span)·PushCap(S), plus both vector headers.
  Result collect receives a conservative push-growth allowance; no
  allocation reuse is assumed. Add the earlier FK boundary backing
  8·(2l+2) and its header conservatively over the constructor/mapping
  peaks. Both maximum laws borrow this one PP vector sequentially. The
  existing per-case maximum family also preserves the exact-span fallback
  and Node subdivision allowance. The new span atoms are in-build sizes
  (observed 168/160 B); the engine Node stride stays the actual existing
  atom (observed 184 B). Add the kernel's source-owned typed workspace
  upper from certified_stress_control_sizes().workspace_upper as a
  SourceUpper atom. It retains constructor/evaluation contexts, nested
  matrix/section/row/accumulator arrays and return/move slots. This is
  a conservative model of the declared kernel workspace, not a compiler
  frame or allocator/system-library stack measurement. Primitive/source
  exclusion and span-bound premises must be re-audited with any B7
  admission-domain extension.

- **KD5O11 (product caller certificates).** The kernel workspace is
  kernel-only. Product recovery retains certified_member and
  certified_effective, each Option<CertifiedMemberRecovery>, while the
  maxima borrow the coefficient vector. Each member payload has 12 end
  rows and 5×6 section rows. Add both actual in-build Option strides.
  Conservatively sum the source-owned transient carriers too: two
  Result<Member,RecoveryFailure> slots (FK→PP and PP→and_then returns),
  four Member slots (mechanical parameter, its clone, and the effective/
  published match bindings), and two Result<(Member,Member),RecoveryFailure>
  slots (closure return and certificate binding). This sum retains return
  and move staging without claiming compiler slot reuse.
  The support loop adds two [CertifiedRow;6] arrays (builder rows and
  Ok binding), its Result return and its retained Option. Private
  stress/summary/group helpers retain up to three named Row operands,
  their chained scalar/return staging and two group-value staging slots;
  a positive 16·s(CertifiedRow)+2·s(CertifiedGroup) allowance preserves
  that fixed nonrecursive caller union. Kernel arithmetic frames remain
  in KD5O10; none is discounted against this separate caller allowance.
  All strides, including enum/Option padding, bind to actual size_of.
  The allowance joins the existing per-case ordinary-recovery family,
  preserving every case multiplier and the other positive owners.

- **KD5T3 (one fixed subject copy).** Only the lexical allocation
  StressFinding::new's subject_id.into() receives a positive 128-B cap,
  replacing the unrelated snapshot-wide STATIC maximum at that site.
  The stress-recovery production prefix before cfg(test) is hash-pinned:
  4526e43a3dca37a265516ad5556a07efb9f25693fca6ee8e065a8211637c47c1.
  Thirteen private constructor calls and their private forwarding helpers
  resolve to exactly 17 source-literal subjects: analysis_status, area,
  axial_force, axial_normal_range, bending_moment_y, bending_moment_z,
  bending_normal_y_range, bending_normal_z_range, first_state, second_state,
  section_modulus_y, section_modulus_z, torsion_constant, torsion_radius,
  torsional_moment, torsional_shear and torsional_shear_range. All are ASCII;
  the maximum UTF8 length is 22 B. The literal seed callers are
  recover_stresses, validate_stress_input_unit_metadata, torsional_shear
  and recover_stress_range; subject/numerator_name/denominator_name
  forwarding does not admit an external value. Independent T3 scrutiny
  confirmed this closed boundary. The existing identifier audit stays128,
  message stays2173, all authored/derived ids and full case/replay counts
  stay governed by their original caps. The public field remains externally
  authorable; this creates no global subject_id or type-wide rule. Historical
  registered snapshots retain their original formula, and changed production
  source fails closed for renewed caller review. Final native pricing decides
  the resulting budget margin; this rule alone establishes no passing claim.

- **KD5O12 (fallback arithmetic replay workspace).** The current fallback
  preserves the supplied QuadraticStressSpan vector and builds a separate
  exact-capacity engine enclosure vector. Their backing and headers are
  positively covered by the summed exact-span fallback and KD5O10 vector
  allowances; no simultaneous allocation is removed. FallbackError has
  only magnitude/error f64 fields, checked before the ordinary model runs.
  Its fixed replay inventory retains the former256 pair-value slots:
  six section values plus l/distance/start/end/active_end/active/lever/q;
  six returned rows, three samples/intensities, nine powers, component/
  control values and helper argument/return staging. Scaled constituents
  use fixed mantissa/exponent pairs and iterative power transport, with
  no new heap array or recursive fallback-product call. The pressure
  branch adds fixed fluid/coefficient/wall/generic/reference-law operands.

  fallback_product_up has nine positive call sites, executed sequentially
  by mul/rounded, power transport, reference-law transport, uniform-slope
  and unit transport. Its one owned ExactAccumulator (actual1088 B)
  remains live through add_product, round, add(-rounded), then signum.
  The source call tree is add_product→decompose/add_shifted→add_word;
  round→round_scaled→compare/subtract→project→window/bit/any_below;
  then add→decompose/add_shifted→add_word and signum→compare. None
  recurses or creates another nested fallback_product_up. Each array
  size is the fixed68-u64 magnitude grid, checked by the model.

  Conservatively sum two actual accumulator strides (product owner and
  new Self return/move staging) and eight68-u64 buffers: two initializer
  arrays, plus both exclusive round_scaled branches' subtract output,
  returned-array staging and project-call temporary. The scalar/index
  census adds256 slots for decompose/product/shifted-word, compare/net
  subtraction and projection helper parameters, locals and return staging
  to the previous128 replay slots. This retains all alternatives and
  integer/u128 argument/move staging without compiler reuse assumptions.
  The resulting SourceUpper is
  256·size_of((f64,f64))+384·max(size_of(usize),size_of(f64))
  +2·actual ExactAccumulator stride+8·size_of([u64;68])
  (13696 B on the current64-bit build). Scalar slot width remains at
  least f64 width. These charges are additional to the kernel workspace,
  product certificates, simultaneous vectors and String allocations.

  Current D1 has no member uniform source, so load iterators are empty;
  all positive fixed arrays, values and exact-product call-depth buffers
  remain priced independent of that emptiness. Any extended source/grid
  domain or changed replay layout needs source review before binding.
  This establishes a source upper bound, not a compiler-frame measurement.

- **KD5O13 (FK directed-helper scalar staging).** Exact source checkpoint
  d0fe28ce8209921489174b1a75114b0660d4d132 changes load_ledger's
  upward/downward product and downward quotient to a bounded integer
  comparator. The complete source file SHA256 is
  84772047aee55e330ab4f7723a81cb8a0ebf255f8ee2b7a8f5fef547ef97f115;
  the model separately pins all five actual helper bodies. The call chain
  is one of product_upward/product_downward/quotient_downward→
  compare_nonnegative_product→nonnegative_parts. It is nonrecursive;
  the three decomposition calls are sequential, with no heap or persistent
  layout. Stack-only is not used as a zero premise.

  Add a separate positive SourceUpper:
  8·size_of(u128)+6·size_of((u64,i32))
  +48·max(size_of(u64),size_of(f64),size_of(usize))
  (608 B on the current64-bit layout). Eight u128 slots conservatively
  retain the product/reference values, three integer conversion results
  and two shifted comparands plus staging. Six dyadic tuple slots retain
  the three decomposition bindings and each return's staging, even though
  the calls run sequentially. Forty-eight scalar-width slots cover the
  wrapper's operands/rounded result, comparator operands and ep/hp/hr/
  common/sp/sr/order, leading-zero and conversion/shift temporaries, plus
  the decomposition's value/bits/biased/fraction and scalar return/argument
  staging. The sum retains exclusive alternatives and return/move slots
  without inferring compiler reuse or a measured native frame.

  Charge the atom explicitly in the typed-core family (therefore in both
  W2 evaluations through the existing core multiplier) and again in late
  formation-guard recovery. This conservative separate charge covers
  S11-G/formation and late guard consumers without relying on PP fallback
  KD5O12 or certificate-caller KD5O11 allowances. Existing case/replay
  multipliers, M/text limits and all earlier positives remain unchanged.
  Original historical helper bodies receive no new charge; a changed new
  helper body stops for a renewed source/slot review. Native profile and
  whole-stack claims remain pending the final immutable source evaluation.
