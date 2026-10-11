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
