# Retained pricing: the reasons for the name-keyed rules

The registered memory profile (`core/product_physics/src/retained_memory.rs`, its GENERATED PROFILE block) is produced by the retained pricing tool, [`tools/retained_pricing`](../../../../tools/retained_pricing/README.md). That tool is SQ2's TEXT + cap-priced chain. This text gives the argument for each rule the tool adds by name: `rules/o10_rules.py` and the later adaptations in `rules/sq2_decisions_*.py`. Each rule is cited here by its label in the code. A rule whose function is absent at a revision is not added, because there is nothing for it to bound.

Line numbers are those at the head named in each section. The tool resolves every key by name, or by the site's own text inside a named function, at the snapshot it prices.

**Current qualification budget.** The owner selected M = 12 GiB (12,884,901,888 B). Keep the 0.9 M margin and require at least 5 % of the phase's applicable TAV_W or TAV_X as remaining TEXT reserve, rounded upward to whole bytes. Existing model counts, all positive owner/backing charges and source refusal guards remain in force. Qualification binds the K-D5/C8 profile to its actual source and native values; the old 10.5 GiB declaration is superseded for this registration. The 64 GiB testing-host ceiling is not a product threshold. Machine-dependent scaling needs its separate resource/concurrency design before implementation.

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
- **The pressure-source join (33218f87c4).**
  - The join deletes `append_retained_source_not_joined` and `RETAINED_SOURCE_NOT_JOINED_TEXT`. N1 and N2 are guarded by that function's name, so they add nothing at join heads; KD5A3's `route` class no longer assumes N2 created `O10_ARGS_FIRST`.
  - The join removes stress_recovery's `#[path]` `correct_norm` module (four lines). The basis key `lib.rs:179:new` named both load_case_algebra's `AlgebraFinding::new` and a stress_recovery line, and the shift makes it ambiguous. The five `AlgebraFinding::new` per-call bounds now resolve that function by name in load_case_algebra (unchanged there); their counts and arguments are unchanged.
  - The join's re-reviewed source pins, its eighteenth KD5C5 caller and its rules on batch 4 are in §J.
- **`g5_native_combinations`' operand loop (99cb958e85).**
  - 99cb958e85 zipped the loop with the combination's terms. Both header forms are bound at 3, one per term.
  - The pre-99cb958e85 form is kept so that the registered tree's revision (`f78f52b7e2`) still prices.


## E. T4 batch 4: U2 and O-13 at `98dba87714`

These additions close the new header and argument inventory. They do not
zero a newly reached branch. D1 still admits no components and no pressure-v3
namespace (§B P1–P3); true array and collection bounds are used even when the
chain conservatively reaches an arc helper. One exception: the chain charges a
named closure's body sites once per enclosing call, so the text rows of the
`append` closure in `append_arc_pressure_results` (called 2 + 3·|sections| ≤ 17
times per call) are charged at the call's multiplicity, not up to 17 times it.
This is safe on D1, which forms no realized bend and no exact-pressure state, so
the closure is unreachable; a domain that admits them must charge the closure
per call first.

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

Batch 4 on K-D5 (`9b128d5ed0`) adds:

| Label | Rule and code argument |
|---|---|
| **U2A2** | Review M-2 gives `append_arc_pressure_results` a second blocking diagnostic (an unknown section location is refused, never read as end_j) whose affected-ref line equals the unrepresentable-section diagnostic's. Each `vec![case.id.clone(), pipe_id.to_string()]` is keyed by its own unique preceding line and checked; both keep U2A's IN128 classes and both sites stay priced. |
| **U2A3** | That refusal's `{location:?}` is one of `sections`' locations: the literals end_i/end_j or a `StationResultants.location`, a `&'static str` (quarter_1, midspan, quarter_3). STATIC (128). |
| **U2L4** | The per-section recovery loop `for (location, elastic) in sections` runs over the two ends plus the caller's station rows; every `solve_load_case_observed` branch forms at most three (two fixed arrays of three, `curved_bend_station_resultants -> [_; 3]`, or none; KD5L7). Bounded at 5. |
| **O13T2** | O-13's arc predicate is gone (the O13 rows above stay for historical snapshots), but `s11g_tests/arc_recovery.rs` remains, rewritten, inside the cfg(test) `s11g_tests`. The chain reaches some of its helpers through conservative name candidates, so its arguments keep honest classes: the test `group`'s kind (callers pass only axial/shear/torsion/bending) STATIC, the P2/P5 tags' project ids (fixed `invented:o13:*` ids, longest 42 B) IN128, mode Debug ≤17 B, f64 Display ≤327 B and a copied source line ≤ the included lib.rs + formation_guard.rs bytes at the snapshot. No zero. |
| **KD5A9** | `certified_loads::member_source` formats `member:{len}:{member}:{source}`. Batch 4's pressure closure binding (the arc pressure free strain keyed by region id) gives the site a positive multiplicity. `member` is a pipe or connector component id (IN128). `source` is an authored load or region id, the static `reference`, or on the 0.4.0 route `source_recovery::eigen_source_id` (≤ 23 + 20 + 1 + 128 B), so it takes the existing 600-B composite class. The full multiplicity stays. |

U2 deletes `connector_evidence::row_basis`; its E4 edge and A3 audit are added
only when that function is present. This avoids a null stale key without
weakening a surviving function's rule. The registered revision still carries
the original E4/A3 entries.


## J. The pressure-source join on batch 4

The join (b7-plan) merged with T4 batch 4 (#1314, `745f55f918`). Each rule
applies only where the join's code exists, so earlier revisions price as
before, except JP1's pins, which are now the join's bytes. Every zero below is
a dead edge or a guarded multiplicity-0 site; no reached branch is zeroed and
no bound is unbounded.

| Label | Rule | Code argument and guard |
|---|---|---|
| **JN1** | `price.py` scans `core/numerics/correct_norm` where the snapshot has it and adds each dependent crate's edge. The seven crate-qualified `open_pipe_stress_correct_norm::norm2/norm3(` calls in frame_kernel are bound to that crate's functions: `rigid_body.rs assess_rigid_body` (norm3, norm2), `certified_recovery.rs norm_up` (norm2) and `certified_group` (norm2, norm3, norm2), `final_case.rs support_hypot` (norm3). | frame_kernel's `correct_norm` module moved byte-identical into the crate. A crate-qualified call keeps only its last qualifier, so by name it would also reach the same-crate private `final_case.rs:norm2` and `formation_check.rs:norm3`, which those paths do not name. Before the move, `crate::correct_norm::normN(` resolved the same way. **Guard:** the run stops unless frame_kernel's non-test files hold exactly these seven calls, each found once in its caller. |
| **J1** | `EDGE_ZERO` `semantic_contract.rs for_source` → `physics_source.rs validate_in` (its `PRESSURE_SOURCE_ID` arm) | for_source's only D1 caller is `retained_precision::validate`'s G7 on `project(source)`. project() sets `producer.semantic_contract_id` to `route.base_id`, either preview-physics-1 (route L) or physics-1 (route E). So the arm is never taken; this is R1's argument. **Guard:** the run stops unless `retained_precision.rs` holds exactly those two `base_id` literals and the projection line, and the arm text is found once. |
| **J2** | `EDGE_ZERO` `lib.rs CaseSet::attempted` → `source_recovery/j7_t4_consequences.rs request` | `attempted.push(request, id)` passes the loop's pattern binding `(&request, &id)`. The function-as-value scan picks the J7 test module's private `fn request()` by name. That module is declared only under `#[cfg(test)]`, so lib.rs cannot name it. Only this edge is zeroed. A callgraph exclusion cannot remove it, because the value scan's edges bypass exclusions. **Guard:** the call is found once, and `source_recovery.rs` declares the module under `#[cfg(test)]`. |
| **J3** | `site_size` 0 at twelve `source_recovery/j7_rf_census.rs` format sites: five in `actual` (Debug of `blocking(&diagnostics)` and `boundary.findings`), `method_row_holds` (`ordinal`), `leaf_diff` (`k`), two in `method_value` (Debug of `f.helper_stage`, `f.error`), `public_value` (`d["code"]`, `d["message"]`), `census_entry` (Debug of `f`), and the expected-file writer (`sha(..)`, `lines.join(..)`) | The J7 RF census is a child module that `source_recovery.rs` declares only under `#[cfg(test)]`. The template inventory's file-name filter does not see it as test code. None of its rows is reached on either route. Across both J7 test modules, the one reached row (`j7_t4_consequences.rs sha`, a by-name candidate) has multiplicity 0. This is O-10 J's form: **no `site_zero`**, so the chain still computes each multiplicity. **Guard:** `o10_check` fails the run if any J3 site has a positive multiplicity or is no longer found, and the run stops if the module is no longer `#[cfg(test)]`. Unlike O13T, the Debug vectors and the joined census have no finite type class, so these sites take J's guarded size rather than per-argument classes. |
| **J3'** | Classes for J3's composed arguments at the two sites that also hold an identifier placeholder (`node`, `path`), which the chain still evaluates: `ordinal` ≤ 13 B; `k` ≤ 1,024 B (RES) | **`ordinal`:** a '.'-separated part of a reference key `r[0]` from `references.json`. The census pins that file by sha `7b176dbb…` and asserts the pin. Its longest key is 13 B. **Guard:** the run stops if the census's pin changes.<br>**`k`:** a JSON object key of the twin's produced public envelope, or of the test's own literals. These are schema field names or entity-keyed maps of D1-capped ids, so the class is RES. The longest key in the committed raw fixtures is 47 B.<br>Both classes are unused at multiplicity 0 and keep the class honest if the J3 guard ever trips. |
| **J4** | `ID_AUDIT` `exact_admission.rs pressure_v3_formulation_basis_in`: `identity.to_string()` ≤ the longer of 2 literals; `source.to_string()` ≤ the longest of 3 literals | The two limitations are whole `&'static str` selections. `identity` is chosen by `selected == Some(true)` between the pressure-source-1 and pressure-1 texts. `source` is chosen by `match selected` among None, Some(false) and Some(true). Each bound is the raw literal length read at the snapshot (≥ the decoded length). **Guard:** the run stops unless each `let` selects exactly that number of literals. |
| **J5** | `EDGE_ZERO` `physics_evidence.rs validate_physics_evidence_in` → `load_reference.rs prepass` | `prepass(source, true, Method::PressureJoined)` runs only under `if resolved`, where `resolved = composite && contract == PressureV3 && evidence has load_reference_states`. The one call passing `composite = true` is `physics_source::validate_in`. It is unreached on both routes: its only reached caller's edge is J1's zero, and its PhysicsV2 sibling is covered by G4 (T17). Every other call passes `false`: `validate_physics_evidence` (route E's reached caller, StraightV2), `validate_pressure_evidence` and `load_reference::validate`. Zeroing the edge removes route E's reach into `prepass`/`validate_record`, along with their `shape_in_resources` recursion, nine load-reference loop headers and the `support_state`/`member_state` id arguments. None of these is reachable on D1. **Guard:** the run stops unless the call sits under that `resolved` test and is the file's only prepass call, and unless the `validate_physics_evidence_in(` call inventory is exactly three `false` calls and physics_source.rs's one `true` call. |
| **J6** | `recursion_reviewed` (via the new `O10_RECURSION_REVIEWED` family in `sq2_rules.py`): `source_blocks.rs shape_in_resources` | The join moved `shape_in`'s body into `shape_in_resources`. The only change is an explicit `resources` list, consulted for a non-local `$ref`; an unknown name or missing fragment fails closed. `shape_in` now only delegates with no resources. The registered review of `shape_in`'s bounded self-recursion applies to the function that now recurses: multiplicity 0 on D1, because the source-block shape check is not on the D1 path, and text only on its error path. Its graph callers are `shape_in`, whose D1 reach was only through `prepass` (J5), and physics_source's receipt and transport-metadata checks, which are unreached. **Guard:** `shape_in`'s delegation and the self-call are each found once. |
| **JP1** | K-D5's three source pins are re-pinned to the join's bytes on batch 4: `certified_recovery.rs`'s production prefix, `lib.rs solve_load_case_observed` (in `kd5_text_model.PINS` and `kd5_first_error_model.LIB_PINS`), and stress_recovery's non-test source (`STRESS_SUBJECT_SOURCE_PIN`). `JOIN_REVIEWED` and its alternative-acceptance helper are removed. | Each digest was reviewed against batch 4's bytes (`745f55f918`). `certified_recovery.rs` differs only in its four `norm2`/`norm3` call paths: `crate::correct_norm` became `open_pipe_stress_correct_norm`, the same functions (JN1). `solve_load_case_observed` differs only by the removed not-joined diagnostic call; its route strings, record labels and KD5C6's attempt/clone consumers are unchanged. stress_recovery differs only by the removed `#[path]` `correct_norm` module; KD5T3's closed subject producer and its one copy site are unchanged. The pins are exact, so revisions before the join no longer price with this tool. The registration moves to the join head (D3). |
| **JC5** | KD5C5's outside-caller inventory is a fixed 18 pairs: the 17 K-D5 pairs and the J7 census's `method_row_holds` `.mul(` (plus U2's pairs where `bend_pressure.rs` exists). `JOIN_ONLY_CALLER_FILES` is removed. | The census calls `.mul(` only on the validation crate's exact rational `Exact`, its own `Exact::mul`, never `FallbackError::mul`. The caller body is digest-pinned. |


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
  up to 7 locations ×4 groups and g supports 4 evaluands (force/moment
  vectors and their separately published magnitudes). Price
  V(RecoveryRecord,m+g), each record's separately grown group backing
  (m·PushCap(28)+g·PushCap(4))·s(RecoveryGroup), every member/support id, two
  owned labels per group at 128 B each, and the error child at
  TextCap(8192) for every group. Group allocation headers are in the actual
  in-build `RecoveryRecord`; all group fields are in the in-build
  `RecoveryGroup`. The fired-items Vec also gets V(String,28m+4g) for its separately grown backing; its formatted String children remain in TAV. No unavailable reason's storage is silently omitted.
  `member_scales` collects its paired maxima map into a second paired map.
  Both `(usize,(f64,f64))` bucket maps are priced concurrently, with no
  allocator-reuse assumption. Older snapshots retain the original paired
  map plus scalar map formula.

Every weighted layout atom binds to actual in-build sizes or an explicit
source upper over those types. Workspace unions are source uppers, not
compiler-frame measurements. Python ASSUMED strides are not budget evidence.
Final completeness and budget claims require the implementing commit's full
pricing run and in-build atom evaluation.


- **KD5T1/T2 (unified fixed text).** Final source basis 9b128d5ed0, T4 batch 4
  merged onto K-D5 (re-reviewed from 3903ab6f28 and before it from
  c62d759f1556cb7c0380aea9f4146a0c8412ddce; the moved pins are reviewed below)
  has source-pinned record constructors, group/magnitude_group helpers,
  member_group_decision, the D1 case consumer and both maximum evidence
  producers. Production-prefix pins protect the certified-publication,
  certified-recovery and exact-sum source/slot inventories as well.
  Every produced group label comes from the exhaustive constructor list:
  all, axial, bending, connector_local, cut_i, cut_j, end_i, end_j, force,
  midspan, moment, node, pipe_wall, quarter_1, quarter_3, recovery, shear,
  support_force, support_force_magnitude, support_moment,
  support_moment_magnitude, torsion, wall_cut_i, wall_cut_j, wall_end_i,
  wall_end_j, wall_midspan, wall_quarter_1 and wall_quarter_3. The maximum
  is24 UTF8 B; non-D1 wall/connector labels remain conservatively included.
  Numeric/bound-unavailable/comparison-unavailable item literals are
  10/38/36 B; the unified sentence literal is234 B. The pinned comparison
  producer's longest reason is35 B (nonfinite certified group magnitude).
  Bound-unavailable reason content remains8192 B, member ids IN128 and
  f64 exponent spelling24 B. Thus F_len=max(234,8406,247)=8406 B and
  the six-item sentence bound is50859 B. Fixed route tokens are at most30 B.
  One unified recovery sentence has no three-sentence merge factor.
  Current maximum coefficient_basis/enclosure_scope literals are46/79 B,
  present in both pinned producers and within the existing120-B ordinary
  evidence allowance; the evidence object field count remains within15.
  Changed producer/consumer/inventory source fails closed for renewed
  review. Historical registered snapshots retain their original formula.
  Pins re-reviewed at 3903ab6f28:
  - certified_recovery.rs production prefix ab2a378f…: c85d346213 maps an
    underflowing midpoint to 0.0 (scalar match); 9ef0cbc78a's RowLoad is a
    stack value (one ExactAccumulator and two f64), and straight section rows
    are formed one at a time with one live load sum moved into the row's
    evaluation, within the eight-accumulator census; arc rows (a = 0 at D1)
    and the envelope's addend counts are stack arrays; 8ac8c57ade forms an
    element block only when it is adjacent to the reaction DOF. These changes
    add no allocation, String, format site or error literal, and no owned
    layout or caller/workspace union changes.
  - lib.rs solve_load_case_observed 4b96ef49… (both models): six early
    returns add an empty withheld_summaries Vec (no allocation); one new
    per-member warning in the certified summary-error arm (KD5A8, KD5O14).
    It adds no route string or maximum literal, and no CPP first-return
    allocation site; its {error} copy is priced as a TEXT site.
  - preview_physics.rs render eb43e10b…: a cfg(test) witness hook only,
    with no `divided` call.
  Pins re-reviewed at 9b128d5ed0 (batch 4: U2 arc pressure, O-13's arc
  predicate and short-bend rule removed, review repairs):
  - certified_publication.rs production prefix ea24eab4… (text model,
    KD5C5 and KD5C6) and whole file b54ebf46… (KD5C6): one new function,
    `arc_pressure_wall`, adds pAi to each section's axial row of a cloned
    CertifiedMemberRecovery (fixed `[CertifiedRow; 12]` and `[[_; 6]; 5]`
    arrays) and negates/copies the two wall end rows. It is stack-only, its
    errors are FK's static RecoveryFailure, and it adds no String, format
    site, label literal, FallbackError use or first-return allocation; the
    test module adds `arc: None` to a fixture. The KD5T2 inventory and the
    KD5C6 site rules are unchanged apart from line numbers (checked by
    evaluating both models at 3903ab6f28 and 9b128d5ed0).
  - formation_guard.rs recovery_finding c499ed59…: the scales map's value
    type is O-13's `ActionScales { force, moment }` instead of a tuple
    (`unwrap_or_default`, `.force`/`.moment`); its three format templates
    and their arguments are unchanged.
  - lib.rs solve_load_case_observed 435591f5… (both models): the realized
    arc branches of U2 (arc free expansion with the pressure strain, the
    certified arc wall rows, `append_arc_pressure_results`), the
    case-qualified arc extrema id (already audited, §E U2A) and the removal
    of the short-bend finding. It adds no route string or maximum literal
    and no CPP first-return allocation site; `wall.clone()` and the new
    `Option<CertifiedMemberRecovery>` are stack values.
- **KD5A8 (withheld summary ids).** The certified summary-error arm's
  warning copies only the pipe's own authored id: the stable_suffix id
  suffix, the message's member id and the affected ref (IN128). Its
  `{error}` operand keeps the chain's own TEXT cap, and the site keeps its
  full m-per-case multiplicity in D. The rule needs the
  `withhold_certified_summaries` consumer and the exact affected-ref line;
  otherwise it stops.
- **KD5O14 (withheld summary backing).** Each case's LoadCaseSolve owns a
  separately push-grown Vec<Diagnostic> of at most one warning per member.
  An append into the envelope leaves its capacity, and a rendered preview
  never appends, so each case's backing lives beside the envelope's vector
  until the solves drop. Price C·PushCap(m)·s(Diagnostic) in the
  Diagnostics family. The strings and the count are already in TAV and D.
  Snapshots without the field keep their formula.
- **KD5L1/L2 (group census).** The surviving `record in records` header is
  re-bound to m+g on the unified record shape; the separate per-record
  `group` header is bounded at 28 (supports have four evaluands). This protects
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
  closure identities use capped region/pipe ids and a fixed kind token;
  batch 4 moves that `format!("{}:{}:{}", term.1, term.2, term.3)` site,
  unchanged, from finish_source_groups' map closure into its new callee
  `closure_source`, and the audit follows it there by name.
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
  Historical helper bodies receive no new charge only after all three
  wrapper bodies match their pinned originals (identical at basis2243be380b,
  registered B3 and U2 snapshots), and no new nonnegative_parts helper
  remains. Comparator-name absence alone never selects historical pricing.
  Renaming or removing the new comparator changes the wrapper bodies and
  fails closed; a changed original wrapper is also rejected. All five new
  helper bodies remain individually pinned for the positive608-B branch.
  A changed source stops for a renewed source/slot review. Native profile
  and whole-stack claims remain pending final immutable source evaluation.

- **KD5C4 (primitive min/max call targets).** New private
  certified_publication::FallbackError min/max methods made the heuristic
  graph incorrectly fan primitive-number calls into their error-returning
  bodies. The type is private, has no escaping producer/return, and every
  named outside caller's receiver is a primitive number or its numeric
  iterator. These calls use the standard intrinsic, with no repository
  method body or String allocation. Explicit caller/name bindings remove
  that false target; all real FallbackError min/max calls in
  fallback_section_errors remain. The one included CP caller,
  fallback_denominator_relative_error, has f64 lower/ratio operands only.
  The current production-prefix pin protects that typed boundary.
  Existing true visits and R2 call paths remain; this is a type binding,
  not a fixture-derived multiplicity or an unpriced-site waiver.
  Batch 4 renames formation_guard `moment_scales` to `action_scales`
  (O-13's coupled ActionScales); the binding follows the name. Its min/max
  receivers are still the f64 scales (`entry.*.max(value)`,
  `force.max(moment / extent)`, `moment.max(extent * force)`).
- **KD5C5 (private add/mul/divided target subtraction).** On b3a2c739's
  source, the heuristic graph adds private FallbackError methods to outside
  callers whose actual receivers are Scaled, Bytes, ExactAccumulator,
  CertifiedRow or CertifiedValue. FallbackError, its impl and these three
  methods are private within certified_publication. The entire production
  prefix has no trait, public return, function-pointer or macro escape of
  that type. The production-prefix SHA256 822cf28ac85c9a41856f7b1c8d8bace246d913044a7c3cbe8ce344186120e3a8,
  three private method bodies and all 16 caller bodies are guarded by
  `kd5_text_model.py::inspect_fallback_boundary`. A changed receiver,
  visibility, escape path, body, or missing/duplicate function stops pricing.
  The exact 17 caller/name pairs are in FALLBACK_OUTSIDE_CALLERS: annulus
  source_wall_area_scaled, source_bore_area_scaled and source_insulation_area_m2
  (mul); lib exact_straight_summary_extrema (add) and
  append_exact_pressure_results (divided); pressure_exact from_radii,
  lame_at_radius and eigenload_pair (mul); preview render (divided);
  retained_memory strings, retained_error_text and complete_observations
  (add); pressure_exact/source_geometry from_od_wall (add and mul),
  pressure_group_value (mul) and surface_stresses (add); and
  case_state/temperature exact_kelvin (mul).
  KD5C5b (batch 4): on a snapshot whose bend_pressure.rs defines
  `append_arc_pressure_results`, two more outside callers join the
  inventory (17 + 2 pairs, FALLBACK_OUTSIDE_CALLERS_U2, body-pinned):
  bend_pressure `append_arc_pressure_results` (divided; receiver
  `wall.sections[index][0]`, FK's CertifiedRow) and source_geometry
  `arc_pressure_strain` (mul; receivers Scaled). At 9b128d5ed0 the
  production prefix is ea24eab4… (the new `arc_pressure_wall` uses no
  FallbackError). Without the two exclusions KD5C6's cutset check stops,
  because the name resolver adds these false private candidates.
  The new `exclude_targets` mechanism subtracts only the qualified CPP
  add/mul/divided target from each existing candidate list after any existing
  replacement rule. Every other candidate and every inside-CPP caller is
  preserved, including conservative inside candidates. Both the inventory
  and an actual removed-candidate encounter must hold; an absent target or
  quiet no-op refuses. Historical revisions without the fallback function
  add no C5 exclusions and retain their original graph. This changes no
  rows, cases, counts, source/workspace owners, D1 premise or memory/text
  limit. The failed b3a2c739 tree remains evidence; corrected contributions
  and native budget outcomes require the subsequent immutable-head run.
- **KD5C6 (conditional first-return String allocation).** Exactly14
  returning allocation alternatives in certified_publication receive per1
  site_from bounds: checked/divided use fallback_circular_maximum,
  summary_row, stress_mpa and maximum_summary_mpa; the six transport errors
  use both fallback_circular_maximum and straight_stress_spans; the two
  maximum_cover errors and three public-MPa errors use maximum_cover;
  direct fallback transport failure uses fallback_circular_maximum.
  Each genuine inner Err propagates immediately from that complete attempt,
  including Result<Vec<_>,String> collection's first-error stop. Preserved
  conservative CertifiedRow/Value receiver candidates retain positive root
  allowances. Every outer attempt/member/case/replay count remains, along
  with all graph/function/loop counts, 4194304 arithmetic visits and2l+1
  span capacity/traversal. Inner returned Strings, outer formatted Strings
  and stored certified_spans error clones are distinct positive owners;
  solve_load_case_observed may catch and continue, so no bound spans those
  attempts. Sites731/765/816/872/931/943, engine formatters, outer clones
  and all other String sites remain unchanged. The whole CPP file/prefix,
  all12 private helpers, roots and solve/exactStraight consumers are pinned.
  ALL fully qualified incoming graph edges must match the reviewed five
  entry functions and exact shared-helper parents; exact kind+site inventory
  and positive root witnesses must hold. Every consumer lookup spelling
  (short, qualified, kind-qualified and lexical fallback) refuses count,
  size, zero, aggregate or total overrides outside the14 canonical per1
  records. The actual post-rebind multiplicity must equal its positive
  source-root sum. Catch/continue/retry, swallowed
  Err, collection, parent or allocation changes refuse for re-review.
  Historical no-cap behavior requires CPP absence plus pinned original
  directed wrappers; current marker removal alone never selects history.
  Limits, criteria and positive-path arithmetic are unchanged. This source
  proof confers no budget pass; final immutable pricing/native evaluation
  remains required before registration.
- **KD5L14–L16 (new fixed/span headers).** The mapped certificate/fallback
  span walk retains positive2l+1 under KD5O10's current-D1 one-interval
  producer. Section maximum coverage walks exactly5 sections. Coefficient
  controls, stress components, sample values and local intensities are
  arrays of3; section-force/moment walks have6 entries. Rules are limited
  to their named actual functions and retain all case/replay multipliers.
  False primitive min/max ancestors no longer create fictitious TEXT
  visits on Bodies coordinates/edges, g5a rows or notice group scans.
- **KD5L17/L18 (power-transport error).** Each i32 remaining/exponent moves
  toward zero in signed steps of512, except its final smaller step. The
  positive universal loop bound is ceil(2^31/512)=4194304; no pressure
  namespace inference is used. fallback_power_up's sole error copy returns
  Err immediately, so it allocates at most once per actual invocation.
  Its existing positive site_from family preserves the full function's
  case/replay count while bounding only the returning allocation.
- **KD5A7 (MPa copies and identities).** stable_suffix replaces each colon
  by one hyphen, preserving the capped pipe-id byte length. pipe_id and
  record.member remain IN128; pipe.element_id.len() remains numeric20.
  Endpoint/station location rewriting uses fixed tokens; kind/component/
  label/basis and the three stress-name alternatives use existing128-B
  fixed-token caps. The pinned caller supplies only current code constants.
  The lexicon additionally inventories the actual sign.map(str::to_string)
  copy at append_stress_mpa, which a method-only pattern missed. Its two
  caller constants measure167/179 UTF8 B; use the measured179-B bound and
  retain the fallback format allocation separately as an alternative sum.
  No global identifier/message cap is narrowed. Historical snapshots
  without this producer do not gain this lexical row or these rules.


## G. Nonlinear K-D5 at `ba5dddd098`

The nonlinear K-D5 confirmation (DEL-04-01
[kd5-nonlinear-active-set.md](../../../PKG-04/DEL-04-01/Design/kd5-nonlinear-active-set.md)
§1.5, §3, §4) adds a confirmation attempt behind the nonlinear helper's early
return, two friction load formulas, a supplied-u K-D5 evaluation and the
nonlinear support slot in certified publication. No count, owner, margin,
reserve or policy changes. The rules and re-reviewed pins:

| Label | Rule and code argument |
|---|---|
| **KD5A3 (re-key)** | The unavailable-family label `family.clone()` moved, unchanged, from `formation_check::check` into its callee `check_at`; its STATIC class is kept and keyed by whichever function holds it. |
| **KD5M1 (re-review)** | `check` is now a pure delegation to `check_at(…, judged = true, epsilon_cache = None)`; `check_at` holds the former `check` body, calls `evaluate` once and `Failure::detail` at most once on its Err arm. The KD5M1 cap (two detail allocations per invocation, each of the seven exclusive sites charged) moves from `check` to `check_at` invocations, which also counts the confirmation's `evaluate_at` calls. `evaluate` gained only the judged filter (D5C-1 and body scales skipped when unjudged) and the per-attempt ε_E cache: no allocation, error or detail call. `detail` is byte-identical. Pins: `detail` ae006b02…, `check` d22bba23…, `check_at` 29c96900…, `evaluate` 89c4ab51…, `load_closure` 179a7dcf…. |
| **KD5M2 (re-review)** | `load_closure` gained only the two friction formulas in the existing nodal DOF-binding arm; its 15 error allocation sites are unchanged. |
| **KD5T2 pins (re-review)** | `certified_publication.rs` production prefix 5e440b80… and whole file 5824b14d…: `support` gains an optional map of confirmed nonlinear rows and copies one row into its slot; both refusals are static `WideError::CountRange` literals; no String, format site, label literal or first-return allocation. `certified_recovery.rs` production prefix 45e90a6e…: `certify_displacement`, `certify_load_primitive`, the exact sign helper (one stack `ExactAccumulator`, as the eight-accumulator census) and `CertifiedRow` decision methods; the friction formulas return `None` from the member pair. No String, format site or owned layout change. `lib.rs` `solve_load_case_observed` 273d31e0… (both models): the confirmed state's `member_bound`, Stage-B force and nonlinear rows, the route literal now the single "on this route" (KD5A3's two literals become one; the route bound is the text model's measured maximum), and the evidence-line site below. |
| **NLKD5L1** | The Passed evidence line finds the case's one integrity diagnostic by id: `iter:diagnostics.iter_mut().find` is one scan of the case's diagnostic Vec, bounded by D (as KD5L8). The id is formed once before the scan (`ba5dddd098`); at `fb3609f8e1` it was formed inside the closure, one String per scanned diagnostic (62.1 MB of TAV on route E), which was a real per-diagnostic allocation and was removed in code rather than bounded. |
| **NLKD5A1** | The gate-refusal reason prints the gate's reciprocal condition estimate with Debug. It is an `f64` (FK `StructuralReport.reciprocal_condition_estimate`), whose Debug is at most 24 B (for example "-2.2250738585072014e-308"); the 30 B cap is kept and is sound. |

**What the chain charges.** The confirmation's functions are reached
through `append_nonlinear_support_loop_results`; their text sites carry the
computed multiplicity of that helper's body after its early return (0 under
D1.6, which admits no nonlinear support), with their honest classes (support
ids IN128, kd5 details and structural errors the existing 8192-B class). No
zero rule is added for them. The evidence-line site in
`solve_load_case_observed` (`format!("{} {NONLINEAR_CONFIRMED_EVIDENCE_LINE}",
record.message)`) is charged positively at the chain's computed multiplicity
with the integrity message's class, although under D1.6 it is never entered
(it needs a confirmed state); this is a conservative charge, not a claimed
zero.

**A spurious edge removed at the source.** A binding named `standing` in
`solve_load_case_observed` matched `reader_verdict::standing` by name, so the
chain's function-value resolution reached the whole RE reader graph (43
unmapped loop headers). `fb3609f8e1` renames the binding; no rule hides the
edge.

**Compare and registration.** `run ba5dddd098` is complete on all four points (no unmapped header, no unclassified argument; both o10 checks 0; the six pruned rules are main's). Against the registered tree (main `745f55f918`, the committed block at `9b128d5ed0`), 685 coefficients are equal and 4 are higher: TAV_text_requested 6,715,665,264 → 6,730,974,564 and TAV_text_moving, TAV_W and TAV_X each by the same 15,309,300 B, which is the evidence-line site's conservative charge. With these rules and the KD5 source pins restored to the registered revision's values (a scratch copy of the tool), `745f55f918` still reproduces the registered tree exactly (689 of 689 coefficients). The block is regenerated from the new tree; the pinned in-build record and the challenge binary's W1 and maximum phase literals move by the same 15,309,300 B in every phase and both modes (W3 sparse 10,206,071,081 B, dense 10,265,202,425 B, without R). The 17 reviewed inputs and the reader layouts are unchanged. Qualification at the owner's M = 12 GiB with the existing 0.9 margin and 5 % reserve passes: the nine library filters of the batch-4 qualification (`profile_` included) and the challenge binary's `retained_direct_peak_is_within_the_profiles_bounds` are green at `ba5dddd098` plus the regenerated block.

**Re-registration on the merge with main (`e0ec460bf3`).** The merge brings main's memory pool and memory ingress: the caller's limit L (`RetainedByteLimit`, `limited_bound`, `qualified_required`, `retained_profile`) in `retained_memory.rs` and `lib.rs`, and the desktop host's pool and capped worker. Main's memory lane added no pricing rule and did not re-register; its registered tree is still `745f55f918`'s. No rule or pin is added or changed here. `run e0ec460bf3` is complete on all four points (no stale pin, no unmapped header, no unclassified argument; both o10 checks 0), and its six pruned rules are the same six as at `ba5dddd098`. Against this section's registered tree (`ba5dddd098`) all 689 coefficients are equal (only the basis differs): L's checks add nothing that the chain charges on D1's graph. Against main's registered tree the comparison is the one above (685 equal, the same four higher by 15,309,300 B). The block is regenerated for the basis line only; the 17 reviewed inputs hash to their bound values, the reader layouts and the in-build record are unchanged, and the pinned record and challenge literals stay as above. Qualification at M = 12 GiB with the 0.9 margin and 5 % reserve passes with main's limit tests included; the least headroom after the reserve is dense W3's 1,045,167,427 B below ⌊0.9 M⌋.

**Re-run on the implementation review's repairs (`be03ccfb0e`).** The repairs change priced NI source (`confirmation.rs`: the named prescribed-row precondition, an unformed ball's per-kind name, and Stage A's influence evaluation ahead of β) and NLKD5A1's wording above (the estimate is an `f64`; the 30 B cap is unchanged). No other rule or pin is added or changed. After the merge with main at `3c95a9de85`, which touches no piping file, `run be03ccfb0e` is complete on all four points (no unmapped header, no unclassified argument; both o10 checks 0) with the same six pruned rules. Against the registered tree (`e0ec460bf3`) all 689 coefficients are equal and only the basis differs, so the block is not regenerated and keeps its `e0ec460bf3` basis line. The 17 reviewed inputs hash to their bound values, and the qualification at M = 12 GiB (the twelve library filters with `profile_` and main's limit tests, the challenge peak test and both admission suites) passes at `be03ccfb0e` with the in-build record unchanged (W3 sparse 10,206,071,081 B, dense 10,265,202,425 B, without R).

## C7. Borrowed prescribed keys, positive overlapping owners

The released retained_precision NumericCase prescribed field and its four
constructor/query regions use borrowed str pairs. Ordering, duplicate tests,
row visits, classifications and failure paths remain. References point into
integral_receipt's live normalized Value (Cow Borrowed or Owned); no backing
Value/String/JSON BTree allocation is removed. g5b only changes final_scales;
no key reference escapes g5c. The exact struct/numeric_cases/numeric_owner/g5c source pins
in prescribed_key_model guard the new owner count and lifetime. numeric_cases
constructs one numeric owner per selected case or retained-selected combination;
its unchanged body9342fa881a9a15724197456be1dab39570904d2e4120a5185b366a19e9163cd9
is required in both known owned and borrowed branches. Duplicating either push
refuses rather than silently exceeding C; old owned source keeps all original pricing.

Add to SETS, without removing its existing owned-key node/backing allowance:
(C*tree_nodes(k)+tree_nodes(Q))*Node((&str,&str),())
+pushcap(C)*NumericCaseBorrowed +(C+1)*BTreeSetBorrowed
+3*VecHeader +(2*pushcap(Q)+4)*PairBorrowed
+2*NumericCaseResultBorrowed.
C<=3 covers all retained NumericCase sets; k<=192 is each constraint domain.
The simultaneous sequential actual selection set is bounded at Q=1472 before
equality/duplicate rejection. Two positive Q tuple buffers cover FromIterator's
input Vec and stable-sort scratch; count them with the entire Q tree and all
retained sets, without allocation reuse. Three Vec headers cover the NumericCase
Vec and both temporary buffers; set headers are explicit in addition to the
conservative complete NumericCase stride. Four tuple slots cover the four
released source constructions/queries; two complete Result carriers conservatively
cover the numeric_owner return and caller staging (variant-sum over the full
record, public ValidationError and a word tag). Existing reader stack allowances
stay. Resident additions remain in V4/V5/V6 through G7/G8; both temporary
selection buffers and the Q tree remain charged there conservatively as well.
All global owned StringPair20480 and existing SETS/Cow/backing charges stay.
This is a T17 working-owner addition (V4/V5/V6), used by W4's precommit maximum;
old full/W source allocation volume remains431376384B attribution only.
No E+R, margin or5% pass follows from that volume.

The focused native allocator/type probe on rustc1.97.1/8bab26f4, aarch64,
observes borrowed pair32B align8 and set header24B. Inserted leaf/internal
blocks are368/464B align8; FromIterator at Q1472 additionally allocates two
47104B buffers. The existing btree_node_upper helper at the measured borrowed
key layout gives472B; its11key/12edge bound agrees with the exact toolchain
[standard-library node source](https://github.com/rust-lang/rust/blob/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library/alloc/src/collections/btree/node.rs).
The input/sort path is confirmed in the exact-toolchain
[FromIterator source](https://github.com/rust-lang/rust/blob/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library/alloc/src/collections/btree/map.rs).
The472B charge is a source/layout upper bound driven by in-build sizes,
not a node-size subtraction from the existing648B owned-key atom. The private
NumericCase field-sum is SourceUpper, with the isolated168B witness. Current
source still requires independent proposal scrutiny and immutable full pricing
and fresh native confirmation before any registration.

C7A1 migrates exactly the five replaced basis audit keys3635/3644/4329/4330/4346
from text_args.g4.json to function-qualified O10_ID_AUDIT sites. The reviewed
owned/borrowed source distinction selects exact constructor/query text under
the existing owner/count pins. Both representations retain original IN128
constraint/selection node IDs, STATIC128 selection components and RES1024
result entity IDs. Current borrowed lines resolve these same source-bound
input metadata definitions, without placing nonallocating sites in the G6
allocation id_audit inventory. Only exact pinned owned syntax emits the five
allocation audits. Actual template/LEX inventory and the unchanged G6 consumer
check this distinction; no fake allocation or zero rule is added. This migration
changes no multiplicity, visit,
byte class, source backing, arithmetic or positive owner. Known-owned audit
tables remain equivalent; line replacement is neither deletion nor a zero rule.


## C8. Borrowed member comparison, positive caller staging

The composite_exact MemberSection check borrows the descriptor quantity and
member/station/component fields, preserving ordered byte/bit/variant comparisons,
convention/case checks and every error. Its exact old993146b2/new8f9f963b body
pins include the enclosing loops, later JSON/projection work and non-escape
boundary. The old body is identical at the basis, f78/2c5/e9cc, batch3ff5347,
U2 98dba, e929 and936; changed/renamed/duplicated bodies refuse.

Charge a positive MemberComparisonWorkspace SourceUpper to every T25 case and
invocation stage, at CC copies (<=3), in addition to all prior owners. The
source-union inventory is16 thin reference/pointer slots,8 borrowed byte-slice
views,16 usize/index/length/size slots,4 u64 station-bit slots,4 f64 station/bitcast
slots,4 u8 component/cast slots,16 bool pattern/guard/short-circuit/return slots,
and4 i32 byte-comparison result slots. Charge every slot rounded to the maximum
alignment across these actual types, retaining argument/return/move staging and
no reuse between alternatives or source frames. The references include the
quantity view, three pattern bindings, expected member/array cell, lifted
reference comparison and String/Vec equality interfaces/raw pointer staging;
byte views include returns and arguments; lengths include equality/byte-size
and array-index/cast staging. Numeric operands, comparisons, stored
quantity_matches, negation and outer condition remain positive. The standard
String representation and comparison path are documented in the exact-toolchain
[String source](https://github.com/rust-lang/rust/blob/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library/alloc/src/string.rs)
and [slice comparison source](https://github.com/rust-lang/rust/blob/8bab26f4f68e0e26f0bb7960be334d5b520ea452/library/core/src/slice/cmp.rs).
Current native widths/alignments give640B per union and1920B at CC3. This is a
source-slot bound, not a compiler-stack measurement; existing library/helper
allowances and R stay positive. The one guard is nonrecursive; its references
and boolean do not escape the sequential row loop. Charging CC complete copies
at every S/I stage conservatively covers overlap with all carried/resident
member/descriptor enums, Strings, JSON, section checks, projections and collectors,
even after these temporary slots drop; no old owner is credited or replaced.

Only route E's basis audit869 migrates. Its original member.member_id IN128
class remains in source-resolved C8_INPUT_METADATA for both representations.
Exact known-owned syntax emits its allocation audit; the nonallocating borrowed
guard emits none. Route L and every sibling audit (including maximum97) remain.
Actual G6 allocation inventory must accept this distinction without a fake row,
zero rule or waiver. The old155934720B site volume is attribution, not an actual
phase/peak saving. All counts, visits, criteria and failed936 evidence remain;
immutable-source/full pricing/native confirmation is still required before any
registration or budget claim.

## H. T4 batch 5: pressure-only U4a stage 1 and dormant U5a

The source basis is `15d75d7e143b00b8759714c5bd08f3bfb8e9ea72`.
The retained domain remains D1: at most three case owners, no components,
and absent pressure regions on L or an explicitly empty region list with
capacity zero on E. This does not admit the 0.5.0 document seam. Pressure-1
uses tangent end frames; preview retains chord end frames until C5. The
pricing below retains existing positive counts, owners, library allowances,
kernel and product workspaces, R, the 0.9 margin and the upward-rounded
5% applicable-TAV reserve. Pricing completeness does not qualify a build.

**B5K1 (extracted bodies).** `solve_load_case_observed` now prepares pressure
and calls `solve_prepared_load_case` once. The former mechanics body, its
site audits, KD5L7–L9 and NLKD5L1 bounds, and the E-route exact/ordinary
publication edges follow that helper. The wrapper's cases cap remains;
the helper has the actual incoming call count. `build_pressure_case_with_members`
retains classification and its early refusal and delegates once to
`assemble_pressure_case`. U2E2 and the E-route materials-error per-call
bound follow the extracted body. After E's original exact-count removal,
L's surviving historical pressure-body zero edges move to the new body.
No extracted-function bound is pruned. Both wrapper and mechanics helper
are hash-pinned; the pressure production prefix is separately pinned.

**B5O1 (positive carriers and overlapping staging).** Add a separate
CasePrefix family, multiplied by the full CC case-owner count:

`5*s(Option<ExactPressureCase>) + 2*s(PressureRegionInput)`
`+ 2*s(LoadFormula) + 2*s(Result<LoadCaseSolve,FrameKernelError>)`
`+ s(PreparedCaseWorkspace) + s(BTreeMapHeader)`
`+ HashReq((&str,&T),mats) + 24*N + (3*N+8)*s(Vec)`.

All carrier strides bind to actual in-build types. Five complete pressure
Option carriers conservatively cover the assembly output/Some return,
public preparation wrapper return, solve-wrapper assignment and prepared
argument, without move elision or compiler slot reuse. Two complete solve
Result carriers cover the extra wrapper return/staging. Two complete input
and LoadFormula carriers retain the added serde-skipped connector-id Vec
owner and new JointEffectiveArea variant positively, including alignment
and padding; this is an additional conservative charge, not a replacement
of typed request or existing primitive owners. The pressure Option includes
the new assembled_operand_bounds Vec and joint_pressures map headers.

The three N-element f64 backings are eigenloads, cap_loads and
assembled_loads. cap_terms and eigen_terms each allocate N empty Vec
headers and remain in scope while finish_source_groups allocates by_dof,
the third N-header backing. Eight more Vec headers cover those three root
vectors and evidence/local return staging conservatively. The separately
built material_map is charged at mats; the empty factor_groups map retains
its actual standard map header. No early drop or allocation reuse is assumed.

PreparedCaseWorkspace is a source upper of 64 usize-width scalar/reference/
index/return slots, 16 borrowed-slice views, 16 Option-reference slots and
four actual ArcEndFrame slots. The two wrapper/helper signatures together
contain 16 thin references (model, built, stiffness, case, budget and
diagnostics), eight material/restraint/spring slices, eight optional
capture/load-state/product references and two optional string views. Their
argument/move copies fit the stated slice/optional union; the remaining
thin/scalar allowance covers the frame branch, pressure call's references,
indices, flag/comparison and return staging. Complete owned returns are
charged separately. This is a conservative source union, not a measured
compiler frame, and leaves the prior R and helper allowances in place.

Only backing children forced empty by the actual D1 guards are zero:
components and region lists (also capacity), therefore connector records,
joint pressures, factor groups, assembled operand bounds and supplied
joint primitives. No whole pressure-producer zero follows on E; its three
node vectors, nested headers, materials map and evidence construction
remain priced. `batch5_model.py` pins the whole pressure production prefix
(including both input/output declarations, assembly and finish-source
producers), the wrapper/helper bodies and namespace/family refusal bodies.
The no-feature branch requires the exact reviewed old pressure prefix and
case body, not merely helper-name absence. Changed producers, declarations,
allocations or refusal predicates stop for renewed review.

**B5C1 (actual multiplication receiver).** The joint-area branch in
finish_source_groups calls Scaled::mul. The heuristic graph also adds the
unrelated private FallbackError::mul; the exact caller body is hash-pinned
and joins KD5C5's outside-receiver inventory. Subtract only that qualified
private candidate, preserving the actual Scaled target and all other
conservative candidates. KD5C6's complete incoming cutset still must hold.

**B5M2 (first closure error).** load_closure adds two static, immediately
returning joint end/DOF binding errors. Its source pin is re-reviewed to
`6bfe5799…`; all 17 exclusive allocations keep a positive per-invocation
bound of one. The source-loop and full case/replay counts remain. The
new joint closure label has the same authored region/connector ID bounds
and static kind class as the existing pressure label; both sites are audited.

**B5L1/L2 and B5A1 (changed inventories).** The bore/joint FactorArea key
changes the factor-group loop header; its zero follows the same D1.5
region producer, and both new group families require such a region. The
reader's `for edge in edges` walks connector edges emitted only for a
pressure region with built connectors. D1.4/5 exclude both; the actual
projected E source retains empty pressure evidence. This child loop is zero
under those predicates, while the outer reader scans and owners remain.
Each rule is checked against its one named function. pressure_finding's
connector operand is the authored component ID (IN128). The descriptor's
two recovery_treatment literals are measured from actual source each run:
73 bytes for tangent pressure, 50 for chord preview; the maximum 73 is
priced. No descriptor alternative is treated as empty or chosen by fixture.

The CPP whole/prefix and FK certified-recovery prefix pins are re-reviewed:
the optional effective-axial group retains its real label/record ownership;
the closed label maximum stays 24 bytes. The new tangent kernels operate
on fixed arrays, with the pressure-only frame parameter passed into both
binary64 and certified publication. Existing kernel/PP workspace charges
remain. The mid-file U4 test module is moved unchanged after all production
so the production-prefix pin includes every later constructor and helper.
The wrapper and prepared mechanics helper are both guarded by KD5T2/KD5C6;
the route and maximum literal inventory is read from the prepared body.

Registration adds the pressure result schema as input 18, after the old 17
positions. This is a governed contract definition, distinct from the reader's
include_str inputs. Its actual bytes and all original inputs must bind, and
actual reader layouts and native phase values must be checked before the
new record qualifies. The previous record is stale while these differ.

**Comparison and native law evaluation.** All four source-basis points are
complete (including all TEXT variants and the diagnostic fixed point), with
no unmapped header or unclassified argument; both O-10 checks pass. The six
prunes are the same inherited deleted rules as nonlinear main. Restoring
only reviewed KD5/text/C6/closure source pins in a scratch tool reproduces
the registered e0ec460bf3 tree byte for byte, all 689 coefficients unchanged.
Against it, the batch 5 tree has 679 equal and 22 higher union coefficients,
none lower: 12 entries introduce six positive carrier atoms in the two
O_base forms, and ten existing coefficients increase. TAV requested/moving
each rise 22,081,536 bytes, and TAV_W/X each rise 22,020,096 bytes.

The one generated profile has 277 atoms, 47 forms and no Estimate. Its actual
native additional carriers are 248/176/744/1744/900/24 bytes for pressure
Option/input/LoadFormula/solve Result/prepared workspace/map header.
B5O1 is 26,539 bytes per case, 79,617 bytes at CC=3. Every native phase's
requested-plus-moving value rises 22,099,713 bytes; the actual maxima without
R remain W3, sparse 10,228,170,794 and dense 10,287,302,138 bytes. Dense W3's
E+R is 10,354,411,002 and its upward-rounded 5% reserve is 220,033,988 bytes,
leaving 1,021,966,709 bytes below floor(0.9 M), the least of all fourteen
phase/mode headrooms. The original 17 input hashes are unchanged; input 18
is `774dad290e17ece0292b7a6021c59c8067e13aa4267c6bdbb25bb3fb17da75c5`.
The actual reader size/alignment pairs remain 56/8, 64/8, 96/8 and 16/8.
