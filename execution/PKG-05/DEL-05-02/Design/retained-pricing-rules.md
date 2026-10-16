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


## J. The pressure-source join on current main

The join (b7-plan) incorporates T4 batch 4, nonlinear K-D5 (#1327) and
batch 5 (#1333, `f01d1a32d2`) by merge commit. The current source basis is
`20ad730689`. Each rule applies only where the join's code exists, so earlier
revisions price as before except for JP1's exact current source pins. Every
zero below is a dead edge or a guarded multiplicity-0 site; no reached branch
is zeroed and no bound is unbounded.

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
| **JP1** | K-D5 and batch 5 pins bind the actual merged production prefixes and case wrapper/prepared helper. `JOIN_REVIEWED` and its alternative-acceptance helper remain removed. | Against batch 5, `certified_recovery.rs` differs only in its four correct-norm crate paths (JN1), and stress recovery only by the removed local correct-norm module. Batch 5's wrapper and complete certified-publication production prefix are unchanged. The prepared helper removes the obsolete not-joined diagnostic call; the composite-context refusal contributes one diagnostic allocation, giving a net decrease of two in D and D_env under the unchanged TEXT recipes. Route strings, record labels and C6's attempt/clone consumers remain bound. The batch 5 pressure carrier, private preparation, directed helpers and positive workspace/TEXT charges all remain. Exact pins stop pricing on unreviewed source. Registration binds the actual join source and inputs under D3. |
| **JC5** | KD5C5's outside-caller inventory is the fixed current union of 21 pairs in 20 callers: batch 5's 20 pairs and the J7 census's `method_row_holds` `.mul(`. `JOIN_ONLY_CALLER_FILES` remains removed. | The census calls `.mul(` only on the validation crate's exact rational `Exact`, its own `Exact::mul`, never `FallbackError::mul`. All outside caller bodies remain digest-pinned, including batch 5's conditional pressure descriptor callers. |


**Current D3 binding.** The immutable assembled source `20ad730689` prices
completely at all four points, with no unmapped header or unclassified
argument, both O-10 checks passing and the same six inherited prunes. Against
batch 5's registered source tree, the 701 union coefficients are 643 equal,
20 higher and 38 lower; JP1 explains the diagnostic reductions, and all
native phase totals increase. The generated block is exactly the tool's
1,382-line output, with 277 atoms, 47 forms and no Estimate. The original
17 reviewed-input positions (the PP lock and 16 reader statics) remain,
pressure-source-1's table stays
at index 17, and the pressure result schema is appended at index 18, giving
19 actual digest bindings. The latter two are nonresident definitions: the
closed guard requires their exact full paths and positions, zero production
includes for each, and all original resident include obligations. Removing
or changing either named token, or an unavailable input, fails the full
binding. Similar paths/names and production hidden in a test region are
refused by the maintained include-auditor controls.

At M = 12 GiB (12,884,901,888 B), R = 67,108,864 B and the unchanged
floor(0.9 M) = 11,596,411,699 B, native W3 is 10,295,522,396 B sparse and
10,354,653,740 B dense including R. The upward-rounded applicable W/X
TEXT reserves are 220,037,296/241,963,980 B; dense W3 leaves the least
headroom, 1,021,720,663 B after its reserve. Reader size/alignment pairs stay
56/8, 64/8, 96/8 and 16/8. The bound public/law suite passes 65 tests;
all 67 maintained native witnesses and the allocator challenge pass in
separate measured processes. Maximum process RSS is 182,517,760 B, and the
allocator's four positive successor controls peak at 8,399,636 B, below the
current sparse/dense profile bounds. Both non-maximal SQ2 shapes stay below
L(3,0) in all fourteen native phases. Independent source, generated-block,
native-atom, exact-input, margin/reserve and measured-process scrutiny finds
no blocking D3 defect. This registers the join build; the join, B7a and B7b
still integrate to main as one batch.


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
  B7a E14 removes observation_lane_profile and its sole caller
  observation_lane_guard with the DEC-050/053 lane. The former helper's
  only max receiver was its Iterator<usize> half-bandwidth, and it has no
  current successor. A raw full-file lib.rs fingerprint plus explicit
  absence of the retired definition binds this one inventory removal.
  All 27 current caller bindings, their strict uniqueness refusals and
  the real CP fallback_section_errors min/max remain. Any lib.rs byte
  edit, including a test-only addition, requires re-audit of this source
  binding before pricing is reused; this is not a generic missing-name
  exemption.
  E13's deleted support-vector reaction arm also uses this exact full-file
  source binding. Its one complete decision preimage, basis lib.rs:13085
  row.id RES1024, is removed before ordinary strict decision carry. The
  component/operand/reference/qualified displacement-source audits remain.
  A changed or duplicated preimage refuses; no replaced-key waiver is added.
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
boundary. B7a's additional exact b2507ed6 body differs from reviewed8f9f963b
only by removing the obsolete non-composite selector true arguments from
rows::expected_primary_for and rows::bind_for (B4/D1 retirement). The borrowed comparison's bindings, branches, staging and lifetime stay
unchanged. Retain its entire 640 B per-case and 1,920 B at CC=3 source-frame
allowance together with every existing positive owner charge. The model hashes the UTF-8 body from raw bytes, so CRLF drift refuses.
The old body is identical at the basis, f78/2c5/e9cc, batch3ff5347,
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
four u8 bytes retaining the historical ArcEndFrame-slot allowance after
B7a C5 retires the selector and its API. The qualified batch-5 native witness
reports SourceUpper PreparedCaseWorkspace 900 and estimate 1028: 64*usize8 +
16*slice16 + 16*Option-reference8 = 896, plus four one-byte enum slots. Four
u8 bytes preserve the same native 900 and the unchanged estimate 1028,
conservatively retaining the complete positive allowance; no other slot,
coefficient, R, M or margin changes. The two wrapper/helper signatures together
contain 16 thin references (model, built, stiffness, case, budget and
diagnostics), eight material/restraint/spring slices, eight optional
capture/load-state/product references and two optional string views. Their
argument/move copies fit the stated slice/optional union; the remaining
thin/scalar allowance covers the current pressure call's references,
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
connector operand is the authored component ID (IN128). B7a C5 removes
the recovery selector and embeds the unconditional 73-byte tangent label
directly in the descriptor's basis format template. The template's literal
charge is 631 bytes, replacing the former 558 literal bytes plus the
73-byte maximum borrowed recovery operand; all other seven operands remain
priced. The whole descriptor body and its one exact inline label are
source-bound, with missing, changed and duplicate-label refusals. The
existing literal TEXT accounting charges the same 73 output bytes once;
no obsolete operand or empty alternative is credited.

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


## B7a: carry of retired preview source rules

B4/D1 (b) removes the non-composite preview source receipt and its support
resultant, straight-stress input vector and stress-summary recipes. Before
the ordinary line carry, lib/b7a_retired_carry.py transforms copies of two
canonical base files only on exact full-file fingerprints of the current
source_receipt/rows.rs and source_receipt.rs. It verifies the complete
preimages and removes ID-audit keys 390/396/518/529, whose four owned
allocation sites no longer exist, and only the deleted exact/finalize
tokens from the ZW sensitivity CSV. finalize_composite and finalize_for
remain. A partial retirement, changed source or changed preimage refuses
the transformation. Missing source and original exact/finalize constructors
send the untouched original rules to the same strict line carry; neither
branch exempts a replaced current key. The full-file fingerprints hash
raw bytes, including newline representation.

Current positive nodal, functional-binding, projected/derived row-ID,
missing-derived-row format and source functional-ID audits remain intact.
No current owner, loop, call count, cap, numerical criterion, M or margin
is narrowed. The D3 report records each retirement, both full-file source
fingerprints, the adapter hash and input/output rule hashes. The canonical
base files remain unchanged and every replay derives the same two copies.
These full-file guards are part of the pricing tool's dependency: an edit
to either guarded file, including a test-only suffix, requires this source
binding to be re-audited before pricing is reused. The currently planned
public corpus and D8 fixture paths are outside these two guarded files;
any actual helper or test-suffix edit in either file requires re-audit.

Route E's three historical EDGE_ZERO entries at indices 13–15 name the
deleted preview constructors exact, ordinary and finalize. Before ordinary
decision-key carry, the same raw source fingerprints and the complete
caller/callee/reason tuples permit only these keys' deleted status through
KEYMAP_ALLOW. A changed tuple, order, duplicate, source byte or restored
constructor refuses. Replaced or fewer-definition statuses remain errors
under the unchanged generic mapper. Current composite_exact,
ordinary_physics, finalize_composite and finalize_for ownership stays priced;
no surviving edge, cap or classification is removed.

E11's from_radii constructor remains source-bound at its existing body
fingerprint, but its exact cfg(test) attribute makes it absent from the
callgraph's existing R1k production projection. C5 verifies the whole raw
pressure_exact.rs file, the attached attribute and that projection before
omitting only the constructor's private-CPP mul exclusion. Current source
has 19 exclusion pairs from 18 callers; source_geometry::from_od_wall and
the production methods following from_radii remain, and all three private
CPP methods retain their positive owner charges. A restored production
constructor, changed body, attribute, raw newline or resumed production
method refuses; the callgraph's generic missing/witness checks stay strict.
This full-file source binding also requires re-audit for later test edits.

## B7a stage-1 positive ordinary domain costs

The existing B4/D1 domain admits named-point material selectors and branch-L
full-span uniform straight member inputs. The old nodal-only zero premise
does not price that domain. The canonical b7a_domain_model binds twenty-one full
raw owner-source files plus the unique raw nongenerated admission prefix before D6 and late law_order,
admit and sole-permit admission bodies. Actual census, capacities and
cap enforcement are bound; generated registry bytes are excluded. Before
ordinary strict carry, it transforms copies of exact base-rule preimages:
uniform-load execution and formed-row decisions become positive; force
terms are bounded by 12*l and uniform spans by l; material bases by cases.
Every other base rule remains. Calls stay 1+z; basis groups are at most
cases+z, and the existing C_eq=3 bounds build/storage families.

Each straight uniform input emits all twelve DOFs, including zero values.
q=12*l safely covers mixed nodal/member inputs without a target-ID discount.
Every formed term retains one LoadPrimitive and a distinct closure entry
whose Uniform/Authored binding has26 u64 words. Child backings use the
existing 4q+2x and push-cap laws; identity strings have family16 and
member-source268 byte bounds. Separate source/supplied/temporary constructor,
typed FormationSource/comparison, scaled force per existing two-evaluation
allowance, and recovery-source copies remain positive. Original formed-force
records additionally retain their moved scaled_intended f64 children:
each can push80components (128capacity). Single-call construction charges
all63 expansion outputs plus2initial variable children conservatively;
these children are not copied by FormationSource or W2's None formations. No intensity-table
heap is imported: D1's absent analysis state selects Authored intensity.
One row0 per prepared input proves physical source_count<=l, even with
duplicate targets/IDs; full-span boundaries are0/1. The larger old2l+1 span
and coefficient-workspace allowances remain.

Ordinary retains the base and up to3 named material/BuiltModel/stiffness
states through rendering. Original working/base material copies remain;
new states add their independent children plus the exact private tuple
backing and owned key/record strings. Sequential resolver scratch includes
the all-pipe HashSet iterator bound, provenance strings/vector and join;
no three-way scratch overlap is presumed or credited. The invocation's
named BasisCustody objects retain selections/record children independently
of CaseSlot swaps. Exact-growth old backing uses n-1, not a geometric half.
Fresh in-build atoms bind LoadPrimitive, LoadClosureEntry, CertifiedSummand,
BasisCustody and the actual BasisState tuple; illustrative values never
establish ABI or qualification.

With ID128 and four used materials, one provenance record is161+4ID=673,
its joined text is4*673+9=2701, and aggregate record is85+ID+2701=2914.
Formatted cached records use the existing2*length capacity bound; adapter
copy and row clones preserve their existing length-capacity law. The
previous600/1024 basis/copy classes and2648 join are raised at their actual
source sites. Existing11474-byte row and both aggregation-copy charges
cover modulus rows (3755-byte case/3745-byte combination bounds); they are
retained. The separate combination record map is priced even when
combinations are empty. Existing T11/T07, legacy/parity reserves, every
positive B5/J/nonlinear charge, M12GiB, .9 margin and5% TEXT requirement
remain. Additional failed Preview manifest envelope headers, limitation
Vec/string children and transient LocatedQuantity strings are priced beside
the retained original. A separate FAILED_MANIFEST_COPY form includes the complete
additional typed envelope clone and its quality-case/reference children in every
phase conservatively; the old STAGED family is kept separately. Source-bound
site classes and exact O10 owner lists close the14new headers/24arguments,
without a zero or generic uncapped waiver. These source/model changes require actual four-point/native15-input
D3; no diagnostic tree or historical profile qualifies them. Any actual
guarded-file edit requires re-audit before reuse.

The first full positive replay additionally reaches four uniform-load loop
headers and twenty-two previously unclassified identifier operands. The two
PP array/formation walks use the exact twelve-element DOF map; the SP
constructor receives the single [global] input from the sole production
formed-load caller, and its powers map sees the two literal span endpoints.
The FK crate root is also bound by an exact full raw-file guard, including
ELEMENT_DOF, DOF_PER_NODE and element_dof_map. The exact O10 headers retain
only those source-qualified owners. The source
guards also bind all authored case/load/member identifiers and stable_suffix
(which only replaces ':' with '-', preserving byte length) to the enforced
128-byte input bound at these exact sites. The call graph also reaches curved
arms statically: the guarded retained domain refuses components before
dispatch, so no new curved admission is implied. Their authored identifier
operands receive the same conservative positive input bound; no branch or
allocation is assigned a new zero. Generated-intensity prefix diagnostics
likewise keep positive site-local case-ID classes, without admitting generated
loads or weakening their retained-domain refusal. The historical base and
every other positive rule remain unchanged.

The static-tree inventory follows actual reader ownership. Three retired inputs
(v0.2, source-blocks-1 and its schema) leave the historical twelve-file list;
all nine surviving inputs keep their order and positive parsing costs. The
combination and exact prepared definitions and physics-retained table enter
positively. These are twelve distinct resident JSON inputs, independently
confirmed by their production includes/loaders, not merely by registry names.
Two retained tables each have two independent OnceLock<Value> owners: preview
table()/preview_physics_retained_contract() and exact_table()/
physics_retained_contract(). All fourteen allocations are priced; their equal
input bytes do not imply shared parsed storage. Exact full raw guards cover
the three loader files (retained_precision, semantic_contract, physics_source).
The closed inventory requires its exact historical preimage and g4's unchanged
static_tree() still refuses an absent input. The actual pressure-source table
and results-pressure schema are reviewed nonresident inputs; neither becomes
a cached tree here. Their production include absence and pressure-source
verification/vocabulary obligations retain the existing closed guards.
All surviving shape, owned-string, array-growth, hashing/validation, TEXT,
moving and phase duties remain, and actual new static facts are recounted on
each immutable canonical replay. No static cut selects a new budget policy.


### B7a formed-load traversal and finding ownership

The earlier 717871 four-point run is algorithmically complete but does not
establish source-complete pricing: two load-row String sites and the Element
self-weight identity formatter still carried historical nodal-only zeros.
The five KD5 source/ledger/manifest loop rows, seven force-term BASE loop rows plus the expansion-component premise,
and three aggregate ledger-copy totals also retained 0 or the input count l.
The tracked b7a_formation_model replays only their exact preimages before the
ordinary strict carry; historical BASE files and generic refusal checks remain.

Current D1 census/enforcement, the sole private prepared case body, Ledger.finish
and the formation guard bind the replacement premises. One case has <=l authored
inputs and <=q=12l force terms, including the twelve zero-component terms from
each admitted full-span straight uniform input. Each term sits on one DOF;
Ledger.finish constructs exactly built.nodes.len()*6 by_dof slots and rejects
out-of-range terms. formation_rows returns <=N=6n rows. The existing
function-qualified LOOP_TOTAL seam supplies cases*N only to load_row_finding's
row loop, so unrelated published-row loops retain P. All five KD5 traversals
remain positive: outer supplied and complete term walks are q. The two inner
manifest and matching-primitive BODY traversals are1: attach_last creates one
Summand for a new term and attach installs one entry/primitive for that index;
plain nodal records have no formed manifest. D1 rejects all other formed
constructors, and force_scaled/NI copies preserve indices/cardinalities.
The outer predicates/count/matching scans still traverse q; foreign/preclosure
refusal costs do not inherit the admitted-domain condition. Descriptor/row-offset loops retain
their old stiffness and spring allowances plus q. Rule256 is split: old.loads()
remains the authored l bound, while the force traversal is q. The scaled-intended
child walk retains max(l,80), preserving l128 at the current cap while its source
premise is the expansion constructor's 80-push ceiling. Genuine authored-input/nodal
PrimitiveSource loops retain l. Full prepared L/E first-match and function-total
controls establish that a later historical rule cannot shadow these premises.

The two source lists independently deduplicate bare load IDs within each row;
their aggregate entry counts are each <=q. Every admitted load ID is <=128
bytes; member_source's separately charged 268-byte identity is unchanged.
For either independent Debug generation, G=2N+(10*128+4)q includes every row's
brackets and each ID's quotes, separator and existing 10-byte escape allowance.
The three decide_row format recipes retain their separate positive capacities:
2*cases*N*8208; 2*cases*(87N+G); 2*cases*(321N+G). No branch receives a zero.
The returned reason selects one recipe, giving aggregate fired content
H=N*(600+2+8208)+G; fired construction pays 2*cases*H independently.
Individual maxima remain R165057, F165659, six-item join J993964, sentence
S994321. Every full list keeps N String headers; first-six clones own their
backing and copied payload independently. Native FormationRow and the actual
(usize,[f64;4]) map key/value tuple are size_of bindings, not guessed strides.
The six full raw owner files are added to the existing source/admission guard
chain; source, CRLF, preimage and constructor/enforcement drift refuse pricing.

ordinary_finding copies the full fired Vec and sentence into the persistent
FindingSeed. ordinary_members creates independent finding and formation Values,
and ordinary_entry copies formation again. The producer family, two constructor
Value originals and final ORDINARY/BODY all pay these owners independently;
all old observation, row, diagnostic, STAGED, failed-manifest and reserve
allowances remain. The sentence's actual source class is propagated to the
integrity message amendment, evidence/retained diagnostic envelope and L_PUB,
preserving the old recovery/range terms. Existing escaping/hash/copy/moving laws
and every phase remain. Naming only six items never reduces the retained list.

The load-fidelity row_sources copy override is positively rebound from the
historical2*l to q=12*l. Each full audit partitions every force term into one
DOF list. Its successful LoadFidelityRow and terminal Range fallback are
exclusive: the successful source clone occurs after the last fallible
operation. Both complete audit caller multiplicities, the128-byte String
class and capacity2 remain; there is no row-local deduplication credit. Its two aggregate row format rules and the first matched ^rows$ join now price q source occurrences with10-byte Debug escaping; every fixed row/scalar/bracket term remains. The individual sources_debug class keeps its valid per-row l bound. Source-qualified structural/sparse/source-recovery guards and controls exercise actual text_budget site_from/site_total/aggregate precedence as well as full prepared L/E loop priority. No later cap can hide the positive owner.

Self-weight's Element identity formatter executes before the continue and is
positive. Its downstream 17 function and later site zeros retain their actual
guarded warrant: nonobject provenance and the deterministic generated-ID refusal
make recognized/looks_generated false before continue825. The old Node-only
explanation is historical, not the current warrant. Current ordinary fallback
for inputs refused by D1 is outside these conditional bounds. The registration
remains UNREGISTERED until source-complete pricing, actual native/M12/.9/5% TEXT
evaluation and genuine G0/witness/allocator/RSS checks pass; no diagnostic output
confers qualification or permits a public W1 writer.


The first replay of this correction at dd870 stops at the unchanged identifier
audit gate on sixteen newly reached expressions. Their follow-up audits are
site-specific: load-row dof_label is the guarded integrity_dof_label callback
(node ID128 plus three bytes, or global_dof plus usize20), retaining the old600
upper; guard case_id, self-weight case.id/pipe and every LoadLedger ForceTerm
source are admitted bare IDs128. Self-weight byte-length displays retain
usize20. Uniform assembly and plain nodal construction pass load.load_id to
the ledger; force scaling clones it unchanged. The separately owned
member_source268 identity is unchanged. Existing27 full raw source guards and
admission/cap laws bind these constructors; foreign/preclosure and refusal
costs do not inherit an admitted-domain alias. No generic fallback classifier,
audit-gate relaxation or qualification claim is added.


The FK895 first-return count preserves full evaluate multiplicity: the four
dd870 graphs bind check_at's sole evaluate edge and672/448/224/672 evaluator
calls. Only the reformation error String at895 is rebound to one allocation per
evaluate call: map_err runs on Err and immediate? returns it, with no catch,
retry or continuation in the guarded function. Its8334 content and capacity2
remain; no visit, q traversal, graph/function call count, source/manifest owner,
foreign/preclosure failure or downstream detail/unavailable/Preview/W2 copy is
removed. Existing C6 positive/post-rebound and override-alias refusal semantics
are reused for this one site, with an exact separate marker, source/rule,
qualified caller/graph, inventory and pre/post witness. Every unknown source,
changed rule, conflicting override or erased current owner stops pricing.
This source-bound allocation correction is not an acceptance/budget change or
a fit/D3/public-writer claim.


The shared illustrative layout inventory carries FormationRow at the existing
helper estimate 3*s(ExactAccumulator)+128 (3440 in its illustrative64-bit
table), and (usize,[f64;4]) at5*s(usize) (40 in that table). The ordinary
constructor and final G4 evaluator run in separate processes; a helper-local
ASSUMED update cannot populate the final evaluator. Declaring these same two
positive values in g3lib makes every process use the same illustrative
inventory. Symbolic coefficients and actual G5 InBuild bindings are unchanged:
FormationRow and the tuple still require their own actual native observation
before qualification. Missing or unknown atoms continue to stop evaluation;
no fallback size, qualified stride, reserve, cap or fit claim is introduced.


Route E retains the complete common SITE_FROM list before adding its four
existing physics-evidence first-return entries, in their original order with
unchanged caller/per/why values. Its former replacement assignment dropped
the new common FK895 entry; the exact rule guard correctly stopped pricing.
The consuming assignment now accumulates both lists and refuses duplicate or
conflicting keys. Actual full E preparation must preserve every inherited
rule and add the sole common FK895 clause at its resolved source key; L is
unchanged. No downstream injection, marker relaxation, caller-count cut or
qualified fit claim is used.


Five further owned error allocations are bounded at one allocation per full
enclosing function invocation: lib10470/10476 in exact_straight_summary_extrema,
lib11141 in recover_curved_bend_local_forces, and retained_product1745/2831 in
check_support_maps/support_observables. The first three immediately propagate
through the outer Result with ?; the latter two directly return their owned
Association String. The actual five allocation kinds, full caller inventories,
source/type bodies, rule aliases and pre/post counts are checked before pricing
and around the actual format/lexical SITE_FROM override. The common rules reach
both L/E without replacing the E entries. Existing content/capacity bounds,
numeric and scan work, successful equivalents and every downstream detail,
display, clone and capture owner remain positive and unchanged. This candidate
uses count-only precision; it does not use the optional typed55-byte span bound.
Additional full raw exact_sum and curved_bend source bindings preserve the
independently reviewed receiver/error closure. No M, margin, TEXT reserve,
admission, oracle, sample count or qualification change is introduced.

The envelope-only diagnostic projection retains its exact existing ENVZERO
source declaration, run helper and tb emitter. Only its two excluded
retained_product allocation sites have zero counts in this mode; their zero
multiplicity and absent owner counts are required. The three lib sites keep
positive witnesses there, and all five keep positive witnesses in base/X/W.
Unknown/partial/extra exclusions, changed emitter code and arbitrary default
zeros refuse. Every source/rule/caller/site inventory check still runs.


Three ordinary diagnostic scans form their desired integrity ID once per
scan, including an empty scan. The local String remains within that lookup;
the notice lookup returns its first index after dropping the desired ID and
before inserting the notice. The assessed quality and publication standing
still use the first matching diagnostic, including duplicate IDs. No ID is
shared across cases or whole solves. All diagnostic scans and resulting
messages, row order and status are unchanged; canonical pricing must derive
the new call multiplicity and retain each hoisted allocation. The actual
full lib binding and sole prepared body bindings follow these source bytes.
The five first-error owner/site/caller boundaries retain their line identities
and content; their raw source warrant is re-anchored through the same lib pin.
The generated unregistered carrier, native bindings and approved memory policy
are unchanged. A measured combined canonical result remains required.


Current producer spellings are bounded at their exact sites. The private
pressure problem retains all calls and owned copies: message453, five refs
with760 collection bytes,644 joined bytes and718 complete ID bytes. Its
converted ref loop is qualified to this owner; the existing global four-ref
rule and every other helper call remain. stable_suffix keeps its original
128-byte allowance for all callers and adds the private problem's516-byte
delta with both capacity units. Source-local unit message600 updates the
authoritative audit and lexical rule together; its existing702-byte format
is unchanged. Reader require keeps its89 call sites and both formatted detail
temporaries, with40 detail bytes and15 prefix bytes. Mode/sign/arc clones
retain all calls and positive46/167/47-byte literal storage.

Formation range reasons come from the exact ten static ledger origins;
the longest43-byte origin plus16-byte wrapper gives59. Individual reason,
fired, joined and sentence maxima are unchanged. The same updated aggregate
H reaches every independent finding, capture, JSON, ordinary body and staged
copy. No formation site or owner is removed. Admitted copied envelope and
receipt ref slots remain governed by the unchanged components-empty domain:
the only five-ref calls require an arc, and the sole arc builder consumes
those excluded components. Raw/shared/refused/headless five-ref conversions
and spellings remain positive. No broader phase refinement or policy change
is used. Actual combined canonical pricing and native evaluation are pending.


The current stage-1 route and phase model separates actual authored cases
(`lc`) from the unchanged typed storage ceiling (`C_eq`, caps key `c`). On
admitted E, the components/element/named-basis refusals and empty pressure
regions bind plain nodal force terms (`q<=l`), one base state and no formed
primitive, manifest, expansion or load-row finding producer. Generic force,
closure-record/Option, vector, wrapper/null, reader, raw census and unrelated
ordinary owners stay positive. Refused or foreign ordinary calls do not use
this admitted pricing warrant. The first measured variant deliberately keeps
the effective TEXT force/source q bounds conservative at12*l where unchanged.

A separate successful-publication context retains all actual ordinary calls
and authored lc*l inputs. Same-request custody and stage-1 member preparation
refusal prove at least one Frozen case is nodal, so formed/finding cases are
at most max(lc-1,0). Every member case retains its full12*l force bound; no
per-case or nested-loop q is averaged. Only six closed formed-only requested
factories and the isolated finding children use this occurrence count.
Each factory rounds every nonnegative integer atom coefficient upward before
the existing C_eq multiplication. Moving/peak owners, common nodal work,
generic ORDINARY and formation parent wrappers, named L states and all other
storage/copy charges remain at their full bounds. The failed-manifest branch
and simultaneous original/manifest/staged and PreparedCases holdings retain
their existing conservative positive charges in this first variant.

The four formation formats239/241/251/322 use only their guarded row/finding
generations. Exact dual markers, source/admission bindings, qualified caller
and loop stacks, format/lexical inventory and canonical total preimages guard
them. Competing zero/from/size/aggregate/total aliases, erased or changed
M(load_row_finding)=lc, and incorrect effective pre/post row counts refuse.
The capacity factor2 is independent of loop iteration count. Individual
reason/fired/join/sentence maxima and all full capture/JSON copies remain.

The driver runs the canonical base/env/X/W consumers and downstream cost
chain independently in the success context. Only W3/W4/W5 select its forms;
W1/W2 and X1/X2 keep the full ordinary/fallback forms. Both native modes and
all fourteen phase identities and report fields are preserved, as are the
coefficientwise route union and shared native bindings/atom values. Full
ordinary TAV_W/TAV_X forms and text atoms remain unchanged: the existing
runtime law and actual evaluator therefore keep the full upward5% TEXT
reserve. No illustrative ASSUMED value is replaced with a success scalar.

Making member cases preparable or Frozen in W1 stage2 invalidates the
successful case-count premise. The raw source guard then refuses reuse;
actual repricing and re-registration are required. This model changes no
budget, margin, reserve, admission, criterion or expectation. Actual complete
canonical pricing and native all-phase evaluation remain required before
qualification or a public corpus writer can proceed.


T25's selected invocation publication hash has a route-qualified producer.
source_eligible1244 requires exact captured recovery; the per-case gate6136
requires exact capture for all finalized case constructors; and the invocation
gate3077/3081 requires selected exact context before finalize_composite and
finalize_for. The latter obtains policy from the same capture's context,
not an envelope label. Admitted pressure-free L has none of these selected
producers. The model binds the existing source/admission/capture bytes and
the complete qualified constructor/finalizer caller and stack inventory.

Only the extra hash(publication) term inside T25 I1 and its corresponding
row10 representation is absent on L. Every other S/I form, invocation/body,
carried case, C8, header, initial raw/typed parse/hash, moving and W1/T16/T17
owner stays positive at its existing bound. E's actual hash and foreign or
ordinary product paths are unchanged. G4 consumes the stage_forms directly;
the ordinary T25 row alone cannot change that profile. Full TAV_X, both X
phase names, all fourteen phase identities, native bindings, coefficientwise
union and upward reserve remain. Actual complete pricing/evaluation is still
required; this producer distinction supplies no fit or qualification claim.


Metadata validation selects one complete expected-base-ID String from ten
mutually exclusive recipe constructors, including the exclusive support
combination/case alternatives. Their source-qualified content bounds are
46,37,140,143,158,162,188,338,169,322 bytes. The largest complete base is338;
the allocation volume uses one capacity2 generation per full validator call,
without changing its physical visits or any numerical, row or scan work.
The report preserves every potential site's standalone bound and names the
nonnegative allocation group that replaces their mutually exclusive sum.

The prematch owned stable_suffix, displacement/stress signs, postmatch
combination wrapper, qualified-case helper and their owned suffixes remain
independent positive generations. Errors, RefusedCase and receipt failures,
public_error/capture_error detail copies, STAGED/failed and JSON owners also
remain. Exact current source/cap/custody/helper/enum, full caller/stack and
allocation inventories, authoritative audit preimages and competing override
checks bind this local group; no global STATIC/RID/error/default changes.
Envelope-only exclusion is restricted to the existing canonical emitter.
All current native, phase, union, budget and reserve conditions remain.

The metadata errors at2544/3005/3227 retain their full149/1049/1046-byte content
and capacity2. Their Err propagates immediately through2567? out of the whole
bind_rows_subject invocation; each site allocates at most once per full
owner call. All freeze, serializer and combination callers remain, including
later failures beside other Frozen cases. Association/detail/JSON/RefusedCase
and receipt failures keep their independent owned generations and copies.
This is a positive per-invocation bound, never a prepared-custody zero.

## B7b: actual-source D3 at `837a04b770`

B7b deletes the `2.0.0/exact_straight_pressure_v2` contract with its four
tables and readers (A4/A5), rebases route E's successor on pressure-1 (D4 (a)),
makes the current pressure-source table and the ordinary pressure table
independently resident (O-10), adds model 0.5.0's exact half (`shear_model`,
an explicit per-section `corrosion_allowance`) and keys the load/reference-state
selector on presence. D3 was first done at `7e638477a8`, re-done at
`1bcc55a80b` and again at the final source `837a04b770` (below, "Re-done at
..."). Line numbers are at `7e638477a8` unless stated. Each stop of the tool is
resolved here; nothing below changes M, the 0.9 M margin, R, the 5 % TEXT
reserve, the numerical domain or an acceptance criterion.

**Reproduction.** The committed tool's one-current-value pins (B7a's source
models, re-bound here) hold the registered revision's bytes, so the committed
tool reproduces the registered tree at that revision (`837a04b770`), not at an
earlier one: an earlier B7b revision stops at a result_export pin (exit 4). Reproduce one with a scratch copy of the tool
whose pins are restored to that revision's values, as README step 5 describes.

**Prunes.** Five rules are pruned beyond B7a's 22, each because its subject no
longer exists anywhere in the crate (not moved): the `for_source` zero edges to
`load_reference.rs validate_load_reference_evidence`,
`load_reference_source.rs validate_load_reference_source_evidence`,
`physics_evidence.rs validate_physics_evidence` and `semantic_contract.rs
forbid_load_reference_evidence` (`edge_zero` 52, 53, 54, 57), and the ID audit
`load_reference_source.rs:47` inside the deleted private `fn code`. Zero edges
to deleted callees bound nothing; the audited site no longer allocates.

**Site-key resolution (`lib/linemap_lib.py`).** A line-keyed site whose hunk
replaced its line was always an unmapped stop. It is now `deleted` only through
a reviewed `linemap_resolutions.json` entry `{"as": "deleted", "fn": name}`,
and only when `name` is the OLD enclosing function and the NEW file defines no
`fn name`. A moved function still stops. The one entry is
`load_reference_source.rs:47` (`fn code`; the file is the 16-line pre-pass).

**Full-file and body pins.** Every B7a source pin that changed was re-audited
against B7b's diff and re-bound to the current bytes: `b7a_retired_carry`
(`source_receipt.rs`: exact/ordinary/finalize constructors still absent,
`finalize_composite` and `finalize_for` unique); `b7a_domain_model` (twelve
owner files, the admission prefix and `family_clauses`; `pressure_runtime.rs`
again for a later `mod tests`-only edit, production prefix byte-identical);
`b7a_formation_model` (`source_recovery.rs`, `mod tests` only); `kd5_text_model`
(`lib.rs`: the same five production `.min(`/`.max(` callers); `batch5_model`
(`namespace_branch`, `family_clauses` and the pressure prefix: v2's declaration,
refusal and negative-pressure gate deleted, the 0.5.0 contract/shear/corrosion
refusals are diagnostics only, `member_connector_ids` decodes, and admitted
regions stay empty on E or absent on L, so no carrier child is populated);
`b7a_returning_errors` (the canonical emitter's transport hash: its only change
is the carried `retained_w1` key). Line-keyed preimages that moved with
unchanged text were re-keyed to their B7b lines (`b7a_domain_model` audits,
`b7a_owner_choices` helper rows, `b7a_phase_model` T25 callers and SOLVE,
`b7a_returning_errors` sites and parents, `b7a_scoped_text_model` LN table).
The route-E keymap allowance resolves its solve caller by name.

**Static inventory (`b7a_domain_model.STATIC_OWNERS`).** The four deleted
`OnceLock<Value>` owners (physics-1, load-reference-1, load-reference-source-1,
physics-source-1) leave; `pressure_source_contract()` and
`pressure_rows_contract()` are distinct resident owners over their reviewed
bytes and are priced positively in their own right. `load_reference::
transport_schema()`'s `OnceLock<Result<Value, String>>` over the pinned
`schemas/load_reference_state.schema.json` is a surviving independent
process-lifetime owner too (b7-plan D2): although its one caller,
`validate_transport_metadata_in`'s resolved branch, is unreached on the D1
graph, any earlier use in the same process leaves it resident, so it is priced
as its Ok Value exactly like the other tables (the Err String is smaller).
Thirteen allocations over eleven inputs; a test fails if result_export gains a
cached owner that is not listed (the one `#[cfg(test)] shared_test_corpus`
excepted). The pressure result schema stays nonresident.

**B7b-B (`o10_rules.py`).** The route-E rules select B7a's or B7b's argument
from retained_precision's two route base ids: {physics-1, preview-physics-1}
(B7a) or {pressure-1, preview-physics-1} (B7b). Any other pair stops the run.

**B7b-R1.** Since D4 (a) route E's successor carries base id pressure-1, so
`for_source` takes the `PRESSURE_ID` arm on E. R1's zero edge to
`validate_pressure_evidence` remains on route L (base preview-physics-1) and is
removed on route E, where that validator is priced at its multiplicity.

**B7b-J1.** J1's guard accepts B7b's pair; either pair excludes
pressure-source-1, so `for_source` never takes the `PRESSURE_SOURCE_ID` arm.

**B7b-J5.** v2's deletion removes `validate_physics_evidence`'s call of
`validate_physics_evidence_in`; the remaining inventory (load_reference false,
physics_evidence false from `validate_pressure_evidence`, physics_source true)
keeps the prepass edge untaken on both routes.

**B7b-F, B7b-E5.** Route E's reached physics-evidence entry is
`validate_pressure_evidence`, whose physics arm returns
`validate_physics_evidence_in`'s Result unchanged. The `require()`/`text()`/
`number()`/`array()` and `require_connector` sites keep their `<= 1 per call`
bounds, now per `validate_pressure_evidence` call.

**B7b-P1.** Section B's premise P1 ("no admitted model has
`exact_contract(model) == PressureV3`") and `o10_rules.py`'s `NOV3` text no
longer hold: route E is `3.0.0/exact_pressure_v3`. `build_model_for_members`
calls `build_connector` only in its loop over `model.components`, and
`connector_specs` reads only `model.components`, so P3's empty built connector
collections, and with them L1–L3 and J, rest on P2 (D1.4: no components) alone.
On B7b sources `NOV3` states this.

**B7b-E1, B7b-E4.** On route E every reached caller of
`connector_evidence::validate` now passes PressureV3, so `admitted` is true and
the old StraightV2 refusal premise is gone. E1's zero now rests on D1.4: the
records are `project(successor)`'s `contract_evidence.connector`, which the
producer builds one per objective-connector component, and an admitted model has
no components. E4 and A3 are inert at B7b: `connector_evidence.rs` no longer
defines `row_basis`, so neither rule is added (both are guarded by its name).

**B7b-LS.** `case_state::is_load_state` is an exact contract plus an authored
load/reference-state field. D1.3 refuses every such field (reference
configurations, model and request expansion laws, analysis state) on every
branch, so it is false on admitted E. This premise replaces I95's version-based
one in the existing route-E zeros (`captured_load_state_case`,
`check_load_state_input`).

**B7b-LR.** `validate_pressure_evidence` takes its load/reference-state arm
only when the successor carries `contract_evidence.load_reference_states`; the
producer inserts that member only under `is_load_state` (false, B7b-LS) and
`project()` copies `contract_evidence` unchanged. The edge to
`validate_pressure_load_reference_evidence` is zero on route E.

**B7b-K1.** `KEYMAP_ALLOW` admits `load_reference_source.rs:46:code` as deleted:
the function-as-value zero edge's callee left with the v2-era readers.

**B7b-J3.** B7b's test-only census rewrite deletes `leaf_diff`; J3's size-0
rule and ID audit for it resolve to nothing and are dropped. The filter applies
only to J3's own entries, and the run stops unless the dropped functions are
exactly ones the census no longer defines, of which only `leaf_diff` may be
missing. Every other J3 site keeps its rule; no production site is affected.

**B7b-VP.** On a 0.5.0 document `validate_profile` adds one per-pipe match on
(contract, corrosion allowance): at most one `problem()` per pipe. The base
rule's per-load bound L is rebound to L+m. The contract/shear/version arms
cannot fire on an admitted document (L has no contract; E requires the exact v3
contract; every admitted document is 0.5.0).

**B7b-CA.** The corrosion-allowance identifiers are audited as admitted pipe IDs
(IN128): the unit-conversion diagnostic ID and refs in `normalize_model_units`
and the two section diagnostics' IDs and refs in `derive_pipe_section`, as their
mill-tolerance siblings. Each site keeps the chain's own multiplicity.

**Scoped TEXT (`b7a_scoped_text_model`).** `problem()` keeps its nine callers.
B7b's new `validate_profile` messages are at most 327 B with at most two refs,
inside the unchanged 453/760 bounds; physics_source's longest `require` detail
literal is 37 B, inside 40 (`1bcc55a80b`'s two new details are 25 and 29 B).

**B7b-DPS and B7b-DX (exclusive diagnostics).** B7b changed `derive_pipe_section`
(two corrosion-allowance refusals beside the dimension and mill-tolerance ones).
The premise, exactly as `o10_rules.py` checks it on the source: (a) every
`diag(` in the body is a `diagnostics.push(diag(..));` statement; (b) each such
statement is immediately followed by `return None;`; (c) after the signature,
the `diagnostics` parameter appears only in those statements, so no other push
and no callee receives it. One call therefore pushes at most one diagnostic.
Pushes by other functions, including this function's callers, are their own
sites and are priced separately. The four sites are one exclusive group; a fifth
site, a missing `return None;`, any other push or use of `diagnostics`, or a
bare `diag(` stops the run.
`text_budget.py` then adds the owner's call count to D once and the largest
member's diagnostic bytes once per call. It refuses (unclassified) unless every
member is a reached diag site of that owner, outside any loop, with exactly the
owner's multiplicity. Each site's own TEXT request, ID format and refs stay
priced at every site, as in B7a's mutually exclusive allocation groups. The
rule also bounds B7a's two pre-existing sites at one per call instead of two.
It is load-bearing: without it, the merged dense W3 phase exceeds ⌊0.9 M⌋
after the 5 % reserve. With it, all point and merged phase evaluations pass.

**Re-done at `1bcc55a80b`.** After the first registration two priced files
changed; each diff was audited to be exactly the following, and the
`physics_source.rs` full-file pin was re-bound.
- **`physics_source.rs` (`033626030c`).** `validate_transport_metadata_in`'s
  resolved branch gains two bindings, `require(..., "LOAD_REFERENCE_CASE_ORDER")`
  and, per receipt case, `require(..., "LOAD_REFERENCE_REQUESTED_MODE")`; its
  closed test module's include paths move (test only). On the D1 graph the
  function is unreached: its one reached-caller path, `for_source` →
  `validate_in`, is J1's zero (the precommit reader's projected base id is never
  pressure-source-1), and the branch also needs `load_reference_states`, absent
  by B7b-LS. The new sites stay positively priced if ever reached: each call
  allocates only in `require`'s own `Err(format!(...))` (detail ≤ 40 B STATIC)
  or `array()`'s Err, both priced per call, and the new `zip` loop has no rule,
  so a reached path stops the run as an unmapped header.
- **`joint.rs` (`b4447b7212`), B7b-JR.** `pressure_finding` may add a fourth ref,
  the listed replaced span (an admitted pipe ID, ≤ 128 B), through
  `refs.extend(extra.map(str::to_string))`. Its refs Vec then holds ≤ 4 Strings
  in capacity ≤ 6 (6·24 + 4·128 = 656 B), beyond the generic `refs` class
  (4·(24 + 128) = 608 B). Its multiplicity is 0: `classify_connectors` calls it
  in the loop over each case's `pressure_regions` and their
  `member_connector_ids` (regions absent on L, explicitly empty on E, D1.5), and
  `classify_pressure_participation` is reached only inside the loop over
  `model.components` (P2). The zero is guarded rather than assumed: the new
  `O10_MULT0_SITES` family lists the site, and `o10_check` fails the run if its
  own multiplicity is ever positive (a positive price would then need the
  656 B class).
Both stay at multiplicity 0, so the only coefficient change at `1bcc55a80b` is
STATICS, from the transport schema owner above.

**Re-done at `837a04b770`, B7b-ZA.** `ccb19ba0bf` makes the readers require the
exact contract's zero corrosion allowance; `837a04b770` changes only test code
(the `#[cfg(test)]` census, the J2 join test and their fixtures; no new
`format!` site; J3's texts still resolve), and the later B7b commits through
`2b47627e66` change no Rust source. The census's sites stay at multiplicity 0
under `o10_check`, and with those test-only changes reverted the tool prices
the identical tree. Each
priced diff was audited to be exactly the following, and the `retained_precision.rs`,
`physics_source.rs` and `semantic_contract.rs` full-file pins were re-bound.
Moved line keys with unchanged text were re-keyed: `b7a_domain_model`'s three
`retained_precision.rs` audits (+17 lines) and the scoped model's `require`
line (`physics_source.rs:78`).
- **`retained_precision.rs` `section_allowances`**, called by `exact_namespace`
  and `preview_namespace`. Those are G8's two namespace predicates, each reached
  once per invocation on its route. Its one loop,
  `list(&model["pipe_segments"]).iter().all(..)`, visits each captured pipe
  segment at most once: bound `m` (D1.9), keyed to this function only and
  `o10_check`-confined. On the exact route each visit runs one `unit_value(..,
  Length)`; the `unit_by_symbol` lookup is priced at its own site per call.
  `unit_value`'s `error(..)` sites stay under their SITE_FROM cap (769 per
  validate), whose premise is that no reader caller swallows an `Err` except
  `g5_native`'s `tw()`; `section_allowances` also swallows one
  (`.is_ok_and`), and `all()` stops at the first failure, so it adds at most one
  uncharged error value per validate, under 1 KB. On the D1 graph it adds none:
  the producer refuses a missing, invalid or nonzero allowance before any
  successor exists (`pressure_runtime.rs` and `derive_pipe_section`), so every
  captured allowance the reader converts is a valid zero length. (Independent
  D3 review, 2026-10-15; adding `section_allowances` to the premise's list or
  charging +1 in the formula is left for the next re-registration.) The call graph carries the
  conversion on both routes (32 more `unit_value`/`unit_by_symbol` calls each),
  an overcount on the preview route, which only reads a key. This is the only
  pricing change: TAV_W and the TEXT atoms rise by 139,144 B, which is
  `unit_by_symbol`'s per-call TEXT ×32 plus 72 B below. A guard stops the run if
  the function gains a loop or a second conversion.
- **`physics_source.rs` `zero_corrosion_allowances`**, from
  `SourceContract::check_invocation`. Its callers, `validate_in` and
  `source_blocks::validate_source_contract_in`, are unreached on the D1 graph:
  `for_source` → `validate_in` is J1's zero, and no reached function calls the
  other. Each pipe's `require` Err would be priced at `require`'s site (the
  detail `ACTUAL_CORROSION_ALLOWANCE` is 26 B ≤ 40), and its `for` loop has no
  rule, so a reached path stops the run.
- **`semantic_contract.rs` `current_header` guard** (load/reference states with
  `carrier_evidence`). It is on the reached `for_source_metadata` path, once per
  call, and its `Err("SOURCE_PRODUCER_CONTRACT_UNSUPPORTED".into())` is priced
  positively there (+72 B TEXT).
- **`reader_verdict.rs` `route()`** pressure-source arm's evidence-object guard.
  Its callers (`source_recovery`, `standing`, `dispatch`, `row_semantics`) are not
  reached from the retained route's roots. The guard's one allocation, the
  constant code's `.into()`, sits in that unreached function and would be priced
  per call there if it were reached.

**B7a reproduction.** With B7a's nine source-bound model files restored from
`3e89c23ea1` and every other current tool file kept, the tool reproduces B7a's
registered tree at `3e89c23ea1` coefficient for coefficient (1,450), and
`generate --check` matches the block committed at `1b24b01bd3`. The B7b rules
above apply only where their B7b source exists.


## T4-U6/U8: actual-source D3

T4-U6 publishes every exact-route stress row on the nominal and the reduced
section (D-5), and T4-U8 adds the Timoshenko straight member (D-6); DEL-05-03's
[section-bases-and-timoshenko.md](../../DEL-05-03/Design/section-bases-and-timoshenko.md)
§9 sets the budget work. D3 priced PP's source at `20c6126a16` and then at
`433fc1f1e4`, which adds U6-SH below (option A, which HELP_HUMAN accepted on
2026-10-16: "Apply the borrowed {domain, payload} wrapper in source_receipt.rs
hash, with its own adjudication rule, a source pin, a test pinning identical
hash text, and an independent review") and the epoch's reader binding of each
section's beam theory. Registration prices the epoch head `addbdc030e` (below,
"Epoch source"). The tool's one-current-value pins hold `addbdc030e`.
Line numbers are at `433fc1f1e4` unless stated. Each rule below is added only
where its source exists, so the registered B7b source adds nothing. Nothing
here changes M, the 0.9 M margin, R, the 5 % TEXT reserve, the budget model,
the numerical domain or an acceptance criterion.

**Reproduction.** A scratch copy of the tool whose pin-bearing files (the
B7a/B7b source models, `kd5_*`, `batch5_model`, `b7a_retired_carry`, KD5's
`evaluate` pin in `o10_rules.py` and C7's NumericCase pins) are restored to the
registered values, with every new rule kept, prices `837a04b770` with exit 0.
`generate --check` of that tree matches the block committed since `e70c115764`
(and unchanged at `433fc1f1e4` and `addbdc030e`) exactly; re-checked with the
tool that prices `addbdc030e` (1,450 coefficients identical).

**Pins and moved keys.** Each moved pin was re-bound after its diff was read:
- **Full-file pins** (`b7a_domain_model`, `b7a_formation_model`,
  `b7a_retired_carry`): the D-5 geometry, rows and evidence; the Timoshenko
  admission, frame and K-D5 member-identity binding; the D-5 exact mass branch
  (`self_weight.rs`); `prepare_sources`' three O4 declines (static
  `unsupported(..)` texts after the existing declines); `rows.rs`'s
  `derived_for` basis twins; `source_receipt.rs`'s U6-SH (nothing else; its
  exact/ordinary/finalize constructors stay absent); `physics_source.rs`'s one
  Err-only `require(.., "ACTUAL_SHEAR_MODEL")` per pipe in `actual_materials`'
  existing loop (18 B, inside the 40 B detail class). No owner these models
  price is added or removed.
- **Body pins** (`batch5_model`, `kd5_text_model`, `kd5_first_error_model`,
  C7, C8): `family_clauses` adds one refusal (a nonzero mill tolerance or
  allowance on branch E, `FamilyFact::SectionAllowance`) and `namespace_branch`
  refuses the Timoshenko contract (`FamilyFact::ShearModel`); both only narrow
  admission. The pressure prefix's changed lines are the per-region member body,
  the joint terms and `closure_source`, reached only through a pressure region
  (absent on L, explicitly empty on E). `fallback_circular_maximum` takes the
  basis and binds its area; `straight_stress_spans` and the two pressure helpers
  read the pressure bore from `ExactSectionBases`. The outside-caller inventory
  keeps every receiver (`.add(` on the span intensity's `ExactAccumulator`,
  `.divided(` on `CertifiedRow`, `.mul(` on `Scaled`). `ExactSectionBases` also
  defines `pressure_group_value` and `surface_stresses` as delegations, so those
  two keys name `SourceAnnulus::`'s unchanged definitions.
- **Moved lines with unchanged text** were re-keyed: the B7a/B7b audit, owner,
  error and helper lines; `formation_check.rs`'s reformation site
  (`FIRST_RETURN_LINE` 909); `retained_w1` (lib.rs 3474 → 3510); the T25
  callers (`source_receipt.rs` 646/732/748 → 662/748/764). The tool's own tests
  follow the same lines.

**Epoch source `addbdc030e`.** Beyond `433fc1f1e4` the epoch adds PP's
realized-arc marker and RD2's reader binding of it; each diff was read and the
priced change is exactly the following.
- **The marker (PP).** `lib.rs` gains `exact_pipe_sections`, `realized_arc` and
  `mark_realized_arc` after `straight_section_resultants`. The case solve's
  `pipe_sections`, `assemble_pressure_case`'s region geometry
  (`pressure_runtime.rs`) and T25's `physical_source_built` (`composite.rs`)
  call them in place of the same inline expressions. The marker
  (`member_kind = "realized_arc"`) is set only on a pipe that
  `built.curved_bend_elements` realizes, and `build_curved_bend_macro_elements`
  builds those only from `model.components` (P2: none on D1), so
  `realized_arc`'s `find` allocates nothing and no marker is set. Re-bound:
  the `lib.rs` and `pressure_runtime.rs` full-file pins, the
  `solve_prepared_load_case` body pins (`batch5_model`, `kd5_text_model`,
  `kd5_first_error_model`) and the pressure prefix. The diff adds no
  `.min(`/`.max(` call. Moved with unchanged text and re-keyed: `lib.rs` lines
  after the helpers (+36), `pressure_runtime.rs` after the arc lookup (−3) and
  `composite.rs`'s two T25 callers (731/982 → 726/977).
- **The readers (RD2).** `validate_physics_evidence_in`, reached on route E
  through `validate_pressure_evidence`, adds three sets: `region_members` (each
  region geometry entry), `case_arcs` (each marked section) and `marked_arcs`
  (one clone of the first case's `case_arcs`). Route E's regions are
  explicitly empty (D1.5) and no section is marked (above), so all three stay
  empty; an empty std `HashSet` and its clone hold no table. The new geometry
  loop sits in the region loop, which D1.5's region rule already zeroes. The
  new `require(..)?` checks (`ARC_UNSUPPORTED`, `ARC_MARKER`) fall under
  B7b-E5's at most one error per call. `section_basis::marked_section` reads
  the marker through `arc_marker`, whose Err (`"SOURCE_PHYSICS_MEMBER_KIND"`,
  26 B) is priced at its own site per call: 96 calls on route E, +4,992 B of
  TEXT. `bind_shear_model`'s new check is in `reader_verdict::route()`'s path,
  unreached from the retained roots (B7b-ZA). Reviewed input #1
  (`physics_source_recovery.schema.json`, +454 B: the marker's `member_kind`
  and its `if`/`then`) is a priced static input: STATICS rises by 228 B of
  text, 8 map nodes and 6 Value slots.
- **J5-T (test-only modules).** RD2's `arc_marker_tests.rs`, included by
  `physics_evidence.rs` only through `#[cfg(test)] #[path = ..] mod ..;`, calls
  `validate_physics_evidence_in` with `composite = true`. Test code is absent
  from every product build, so J5's caller inventory omits files the crate
  includes only that way. A `#[path]` module without `#[cfg(test)]`, or a
  plain `mod` named like such a file, stops the run.

No rule is pruned beyond the 31 below, and the tree differs from
`433fc1f1e4`'s only in STATICS and route E's TEXT atoms above and in U6-RB-V's
results capacity (below).

**Prunes.** The line map prunes 31 rules: B7b's 27 and four ID audits at basis
`lib.rs` 5508, 5509, 5562 and 5563. Those four were the two maxima's `Err`
arms (`unavailable_stress_maximum_members.push`, the source-recovery and
exact-stress maximum diagnostics). D-5 deleted them; their per-basis
replacements carry the same IN128 classes under U6-A and U6-K. No `site_from`,
`site_total`, `edge_per_call` or `loop_total` rule is pruned.

**U6-K (re-expressed basis keys).** D-5 replaced eight basis-keyed audit lines
of the case solve (lib.rs 5387, 5403, 5412, 5497, 5498, 5503, 5529, 5536 at
`2243be380b`: the two stress-finding loops and the two maximum rows) and
`composite_exact`'s 781/798. They leave `text_args.g4.json` and are added by
name: on earlier source with their original classes (the registered tree
reproduces), on D-5 source at the replacing sites with the same classes. The
arc branch's case-qualified exact-stress id is formatted once per basis; both
spellings end in the basis's static hyphen segment.

**U6-DPS.** D-5 admits a nonzero allowance, so `derive_pipe_section` loses the
nonzero refusal. B7b-DPS's checked premise (every `diag(` is a pushed
statement followed by `return None;`, and `diagnostics` is used nowhere else)
holds for the three remaining sites, so they stay one exclusive group: at most
one diagnostic per call. The corrosion-allowance ID audit keeps one format and
one refs site. Only D-5 source (`section_basis_row` present) may drop the
refusal; anything else stops the run.

**U8-MP.** `lib.rs member_prefix` is the one `format!("member:{}:{member}:",
member.len())`. `certified_publication`'s member records, `straight_stress_spans`,
`frame_member_identities` and `closure_source`'s straight operand label all use
it. Every caller passes a pipe's admitted id, so `member` is IN128 and its
length NUM; the prefix is at most 7 + 3 + 1 + 128 + 1 = 140 B.
`closure_source`'s arc label keeps KD5A3's spelling. Both labels are inside
the region path (KD5L10).

**U8-FB.** `formation_check.rs evaluate` adds one `straight_frames_bound(source)?;`
after `load_closure(..)?`. Its only allocation is one returned
`Failure::Closure(format!("{}: straight primitive frame is not its
formation-source frame", p.identity.source))`, which `detail` formats in its
existing Closure arm. The source is a `member_source` identity (≤ 268 B,
B7a's `MEMBER_SOURCE`). KD5's per-invocation caps are unchanged; the
`evaluate` pin is re-bound.

**Loops (U6-L1–U6-L6, U6-L, U6-LF, U6-EX).** Each bound is read from the code
it names, and each header is confined by `o10_check` to the functions listed in
`o10_rules.py`. Every constant's declared length is checked.

| Label | Header and bound | Code argument and guard |
|---|---|---|
| **U6-L1** | The case solve's two `for … in &stress_sites` loops: 5. Its `for (stress, basis) in bases`: 2 on E, 1 on L | `stress_sites` is a `vec!` of the two ends plus one push per station (at most 3, KD5L7). `bases` has one entry without a reduced stress and two with one. The reduced stress exists only for a member with an `exact_sections` entry, which `build_model_for_members` inserts only under `is_exact(model)`. Route L has no exact contract (D1.3). **Guard:** the run stops unless the `vec!`, the one push, the `bases` match and the exact-only insert are as written. |
| **U6-L2** | `push_basis_twins`' zip: 4 | It zips the rows one `push` call made on each basis, refusing unequal lengths. Its two callers (the case solve and `rows.rs`) append one stress site's rows through one helper: `append_stress_mpa`, `append_endpoint_stress_results` or `append_station_stress_results`, each pushing at most once for each of its four element-local stress kinds. |
| **U6-L3** | `case_observables`' headlines: 3 | The displacement headline plus one stress headline per entry of `stress_bases`, a slice literal of one (preview) or two (exact) entries. |
| **U6-L4** | `validate_physics_evidence_in`'s coverage zip: 2 | `section_basis::coverage` returns `[(Vec<&str>, bool); 2]`. |
| **U6-L5** | One case's or combination's rows (`rows_for`, `rows_all_for`, `g5c`, `g5b_exact_evidence`, `gates_after_g0`, `r_comb_1`, `prepared_alias`): P | `rows_all_for` filters `results` by the entry's `basis_ref`, so it is one block of the route's P rows, as `rows_for`'s was. The reduced twins are inside P (U6-RP). |
| **U6-L6** | `unbound_straight_primitive`'s scan and `certify_support_reaction`'s term loop: 12·l; its primitive filter: 1 | A formation source's loads and a formed force's terms are at most q = 12·l (B7a). One formed term carries one supplied primitive: `certified_loads::attach` with more than one source runs only in `push_exact_pressure_operands`, over `exact.assembled_operands`, which is empty on D1. **Guard:** the attach inventory is checked. |
| **U6-L** | Constant arrays: `PREPARED_SECTION_BASES`, `BASES`, `BOTH` (2), `ANNULUS_KEYS` (6); `lame`: Pall; `g3`'s combinations: z; `exact_mill_tolerances`: m | Each is the array's declared length, or one entry per validated Lamé row, per model combination (D1.4: z ≤ 2) or per captured pipe segment (D1.9). |
| **U6-LF** (placed first, `O10_LOOPS_FIRST`) | The case solve's three `for basis in pressure_exact::SectionBasis::BOTH`: 2 on E, 0 on L; `composite_exact`'s: 2; `exact_case_evidence`'s zip with `BOTH`: 2; `validate_physics_rows`' pipes × `BOTH`: 2·m | These headers name `pressure`, so D1.5's region zero rule (`pressure\|region\|aggregates`), which comes first, would zero them; they are not region loops. A new placed-first family in `sq2_rules.py` puts these exact headers before it, and `o10_check` confines them like `O10_LOOPS_LAST`. On L each of the three case-solve loops sits in an `is_exact(model)` arm (the two maxima) or under `exact_pressure`'s per-pipe state (the wall action), and route L has no exact contract. **Guard:** the run stops unless all three loops are in those arms. |
| **U6-EX** (route E) | `for ex in array(&case["pipe_stress_extrema"])?`: 2·m | The case solve pushes one extremum per basis in each maximum branch, and the reader refuses a repeated (pipe, basis) (`DUPLICATE_ID`). |

**U6-A (argument classes).** The D-5/D-6 sites' identifier-bearing arguments
take their classes by site. Static segments are bounded by their longest
literal, read at the snapshot:
- **Case solve.** The stress-finding warning id is formatted once per basis:
  the base id is at most 18 + 128 + 600 + 2 + 64 = 812 B (its audited parts),
  plus the segment. The maxima's ids keep B7b's classes. A row id that
  `section_basis_row` extends by ':' and a basis token (15 B) is TPLSITE
  322 + 16 = 338 B, for the headline `result_ref` and `row.id.clone()`. The
  per-basis unavailable pushes and diagnostic refs are IN128.
- **`section_basis_row`.** The id segment (`hyphen`/`token`) is ≤ 15 B, and
  the recovery-token suffix (`basis_suffix`) ≤ 31 B.
- **`validate_final_metadata`.** The expected recipe is at most the largest of
  B7a owner choices' bounds (338 B), plus the segment.
- **RE `section_basis`.** The private `require` codes are literals. A basis
  name or its hyphenated spelling is ≤ 15 B. A twin id's stem is a prefix of
  the row id (RES, 1,024 B).
- **RS `g5c`.** A reduced twin's class takes the twin's own published id (RES).
- **D-6.** The `exact_sections` key and the shear diagnostic's id and refs are
  IN128. FK `shear.rs`'s and `straight_pipe`'s error names are `&'static str`
  literals, each bounded by the longest; their f64s are Display (327 B).
  `straight_frames_bound`'s source is U8-FB's.

**U6-J4.** T4-U6 renamed J4's function to
`pressure_v3_formulation_basis_with_source`, with a `bool` `selected`. `identity`
and `source` each choose between two whole static literals and are bounded by
the longer, read at the snapshot. **Guard:** each `let` selects exactly two
literals.

**U6-RP (rows per member, route E).** Before D-5 an exact member publishes 12
end-force, 18 station-resultant, 8 endpoint-stress and 12 station-stress rows
and one maximum: 51. D-5 publishes the 20 element-local stress rows through
`push_basis_twins` and the maximum once per basis through `section_basis_row`:
72. Route E's premise becomes P_final = 7n + 72m + 8g + 3 and
R0 = 7n + 72m + g + 3, in place of 51m. Route L publishes no reduced row,
because its model has no exact contract, and keeps 51. The census (`lib/census_exact.py`, I95's B3-S method) measured
exactly 7n + 72m + 8g rows on every empty-region exact case of the 7
pressure-source-1 fixtures in both modes (for example `fields`: 102 =
14 + 72 + 16). The census is taken on the committed fixtures, regenerated on
the D-5 producer by their own writer (`SWBPIPE_PRESSURE_SOURCE_FIXTURES_WRITE`)
at `79bf8656dc`; it equals, fact for fact, D3's census of fixtures regenerated
at `20c6126a16` in a scratch tree. The census also gives
the D-5 per-case evidence: `pipe_stress_extrema` once per member and basis, the
nested section bases and two coverage lists. At the caps that is 3,118 array
slots, 868 objects, 4,029 entries and 414,656 string bytes (STR_CAP 304: the
elastic-maximum id 287 B plus ':' and a basis token).
`data/census_d5_79bf8656dc.json` holds it. **Guard:** route E stops unless the
census states 51 + 21 rows per member.

**Byte-preserving source changes (U6-DH, U6-SH, U6-RB).** Each removes a
resident copy without changing published bytes. Each is detected by its exact
source text; the earlier form keeps its earlier price, and any other form stops
the run (exit 4).
- **U6-DH.** `retained_wire.rs domain_hash` serializes the borrowed
  `DomainPayload { domain, payload }` through `domain_text` instead of copying
  the payload into a new `json!` Value. serde_json's object map orders keys,
  `domain` < `payload` is the field order, and each field serializes as the same
  string and the same borrowed Value, so the text and hash are unchanged.
  T16's three hashes (source identity, publication, receipt) price no wrapper
  copy. T17's reader (`source_blocks::domain_hash`) still copies.
- **U6-SH.** `source_receipt.rs hash` (its only definition in the crate;
  `source.rs` and `composite.rs` reach it through `use super::*`) serializes
  the same borrowed wrapper through `hash_text`. `hash_text` is `checked`'s
  body on the wrapper: the `to_string` text, the checked parse and the
  canonical output are unchanged and stay priced. Only T25's payload copy
  goes, at every T25 hash route: payload, plan, publication and body.
  `source_receipt/tests.rs`'s
  `receipt_hash_borrowed_wrapper_text_and_hash_are_the_former_objects`
  compares the text and hash with the former construction over every receipt
  domain, and reproduces a committed fixture's `receipt_sha256`. The new Err-only
  `e.to_string()` in `hash_text` keeps the chain's own multiplicity (15 calls at
  8,192 B on E).
  **Pin:** `u6_owners_model.receipt_hash_copies`.
- **U6-RB (route E).** The final aggregation clones each published row into
  `rows_by_base_id` only `if preview.is_some()`. `preview =
  (!pressure_runtime::is_exact(&model)).then(..)` is None on route E, so E's
  "Final aggregation" holds the results once, with no `rows_by_base_id` map
  nodes (a new `O10_TYPED_PATCHES` family rewrites that one `ordinary_caps.py`
  line on E). The `id_map` stays. **Guard:** the guarded block, the binding and
  its one consumer are checked.
- **U6-RB-V (route E, registration).** The one `results` Vec is push-built
  from `Vec::new()` across every case (checked: one `let mut results =
  Vec::new();`), so the rewrite prices it at its push capacity,
  `V("ResultItem", PA)`: 16,384 slots at PA = 8,361, not PA exact slots. Route
  L's doubled term covers its own push capacity (8,192 ≤ 2 × 6,345). The first
  rewrite was sufficient only while G-C still bounded route E's results by route
  L's 6,345 rows (capacity ≤ 8,192); route E's G-C now admits its own 8,361
  (below, "Gate bounds"). Cost: +8,023 × s(ResultItem) (296 B) = 2,374,808 B in
  every phase of the merged tree, where route E sets that coefficient.

**D-5/D-6 owners (U6-ES, U6-WS, U6-RS, U8-FM).** Four resident owners sit
outside the chain's TEXT inventory and are priced positively
(`base/u6_owners_model.py`). Each is found by its exact source text; a
half-changed form stops the run, and a snapshot without it adds nothing.
- **U6-ES.** `BuiltModel.exact_sections` holds one (pipe id,
  `ExactSectionBases`) entry per built pipe; `build_model_for_members` inserts it
  in its pipe loop. It is priced as a hash table of m entries plus the keys,
  beside the `DerivedSection` map, in the built resident and in T25's rebuilt
  model. The new in-build atom `s((String,ExactSectionBases))` is 272 B.
- **U6-WS.** For one member at a time, the case solve holds: the reduced end
  and station stresses with their children; the stations' Vec; the
  `stress_sites` Vec (pushcap(5) entries of ≤ 48 B); the stress-finding `bases`
  (≤ 2 × 16 B); and, on the exact maximum's reduced basis, a second span output
  (2l + 1 `EnclosedQuadraticStressSpan`) beside the held nominal spans. These
  are priced on both routes: conservative on L, which forms none of the
  reduced ones.
- **U6-RS.** The reader's `NumericCase` gains `all_rows`, a tenth field beyond
  `NumericCaseBorrowed`'s nine-field upper: one Vec header per numeric case
  and per Result carrier. It holds every published row of its case (≤ PA over
  all cases). `rows_for`'s transient full vector (≤ P) sits beside its filtered
  one, and `g5c`'s drained `bound` classes (≤ P) beside the rebuilt classes,
  with one `last` class clone (its id and `basis_ref`).
- **U8-FM.** `FormationSource.frame_members` holds one member prefix (U8-MP,
  ≤ 140 B, priced at `textcap`) per frame. It is filled from PP's
  `frame_member_identities` and copied wherever the frames are:
  - each assembly evidence: PP's Vec, `with_frame_members`' `to_vec` and
    `formation_source`'s clone;
  - each scaled copy;
  - `certified_publication`'s recovery source.

  The source-recovery assembly passes none.

**Gate bounds (registration).** G-C (`retained_memory.rs phase_caps_on`) must
admit what D1 produces and stay within what the profile prices. Before
registration its route-E bounds were still route L's 51-row P_final and I95's
pre-D-5 evidence census, so G-C refused the cap-maximal exact input in every
build (8,355 results against 6,345; `witness_tests::f1_g_c_admits_...`). On
route E the result rows and their capacity are now C_eq·P_final at route E's
own P_final = 7n + 72m + 8g + 3 (8,361 and 16,384 at the caps; U6-RP, U6-RB-V),
and the five contract-evidence bounds are C times the D-5 census at (n, m, g):
arrays 14 + 78m + 19n, objects 4 + 27m, entries 29 + 125m, string bytes
6080 + 12464m + 304n, key bytes 495 + 1286m (at the caps 9,354; 2,604; 12,079
after the node cut, so objects + ⌊entries / 5⌋ = 5,019 nodes; 1,243,968;
124,941). These equal route E's chain terms ("Final aggregation" and "Preview
tree final copy (envelope evidence)": 5,019·Node(String,Value) + 9,354·s(Value)
+ 1,368,909 B). The rows' text bound stays route L's 2·C_eq·P_final·Text(row) on
both routes: Text(row) bounds one row's text and route E's 8,361 rows stay below
route L's 12,690. Route L's bounds are unchanged.

**Current D3 binding.** At `addbdc030e` `run` exits 0: all four points are
complete, with no unmapped header or unclassified argument, and both O-10
checks pass. Against the registered tree the structure is equal: 1,128
coefficients equal, 294 higher and 34 lower, so the block must be
re-registered. `generate` gives 288 atoms, 98 forms and no Estimate (1,544
lines), keeps `threshold_bytes` at M, and `generate --check` reproduces it. All
70 evaluations pass at the registered build's in-build atoms. Required =
requested + moving + R and reserve = ⌈TAV/20⌉ (TAV_W for W1–W5, TAV_X for
X1/X2), against ⌊0.9 M⌋ = 11,596,411,699 B. The tightest headrooms:

| Tree | Mode | Phase | Headroom |
|---|---|---|---|
| merged | dense | W4 | 52,018,961 B |
| merged | dense | X1 | 77,525,843 B |
| merged | sparse | W4 | 111,150,305 B |
| merged | sparse | X1 | 136,657,187 B |
| L_30 | dense | W4 | 177,800,942 B |

T07 (the live source attempt) is a row of the ordinary base that every phase
holds, so every evaluation prices it; T25 (the receipt replay) is priced in X1
only, whose tightest evaluation is the merged dense one above. Against
`433fc1f1e4` the epoch's source costs 6,308 B in every merged evaluation (the
schema's STATICS; route E's +4,992 B of TEXT stays below route L's TAV_W), and
U6-RB-V 2,374,808 B. W4 (the precommit reader with the successor and the
invocation Value) is the in-build maximum in both modes (B7b's was W3).

**Registration.** `REGISTERED_PROFILES` keeps the build identity (rustc 1.97.1,
aarch64-apple-darwin, debug, opt-level 0, debug assertions, unwind, no
RUSTFLAGS, `open_pipe_stress_product_physics@0.2.0`), the four reader layouts
(56, 64, 96 and 16 B, align 8) and `threshold_bytes` = M. Six of the twelve
reviewed-input digests move with D-5's schemas and tables (#1, #7–#11). The law
tests pin the in-build phase record, the text atoms (D_env 28,127,
Text(diag_env) 262,735,773), the digests and the re-audited reader test
suffixes (`retained_precision.rs`, `physics_source.rs` and
`semantic_contract.rs` gain closed test code only). The block's basis line
names `addbdc030e`; the merged branch head, with FX's fixtures, prices the same
tree.
