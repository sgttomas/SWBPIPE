# D2 — T3 result standing, source envelopes and transport design

HELPS_HUMANS-style design TASK. Read `_COMMON.md` first.

## Purpose

Write the T3 design for the items that decide what a retained-source or joined result may be used for, how its envelope is finalized and carried, and how full precision survives transport and display. It is a proposal until ROOT selects it after independent review.

Most of this work lands in files T1 is changing. Design against T1's candidate `f3270ea79` as if it were merged, and mark every place where T1's final merge could change your design.

## Scope

1. **Composite finalization (`STAGE0_MAP.md` §2.6).** Replace the pre-0.4 `Err("SOURCE_BLOCKS_FINALIZATION_FAILED")` (`PP:1249-1253`; T1's copy also names `load-reference-source-1`). Evaluate porting T1's SF-1 republication (`LOAD_STATE_IMPLEMENTATION/CP4_WIRE_ADDENDUM.md` §1.2) to source-blocks-1 and physics-source-1 against the alternatives, such as a mixed envelope with per-case standing. The standard: no invocation loses a correct result because another case failed, and no failed or Sensitive case gains standing. Say how T1's characterization test `the_pre04_composite_finalization_failure_is_unchanged_by_this_fallback` is replaced and recorded. Reproduce the trigger from T1's review probes P3 and P12 (`REVIEW_CHECKPOINT_3/_run_records/`, read with `git show f3270ea79:`).
2. **Joined eligibility (`STAGE0_MAP.md` §2.7 item 1).** Design the reader-side re-derivation of the resolved 0.4.0 case (thermal laws, fit, material selection), captured like physics-source-1's `actual_materials`, in Rust and Python, with the TS position stated. Say exactly what grants `numerically_eligible`, and what stays `needs_recompute`.
3. **Binding route (§2.7 item 2).** A canonical-document or binding route for joined results through `result_envelope_binding`, with `QualifiedPreviewEvidence`, replacing the test-side `derive_document` as the only source of joined carriers. Desktop export stays T6.
4. **Selected-UNAVAILABLE alignment (§2.7 item 3).** Compare the joined reader's S13 refusal with physics-source-1's acceptance of the same fact. Recommend a direction, with the evidence: what the diagnostic means on a selected case, whether any producer can emit it there, and what each direction changes in already qualified readers and corpora.
5. **Source-blocks-1 re-homing (§2.8).** The T0R carries: the selected-plus-ordinary `needs_recompute` reason, and the all-selected abs-sum summary refused for rule binding. Options: republish source-blocks-1 cases under preview-physics-1 semantics (signed six-component reactions, circular maximum) as a successor identity, or retire source-blocks-1 for fresh solves in favour of D1's general method. Historical source-blocks-1 bytes, schemas and fixtures stay readable and unchanged either way. Update the user-facing `N_SB` and `N_SB_MIXED` texts consistently.
6. **Transport and display (§2.3 items 2–4).**
   - Cross-unit display representability: the `QuantityReadout → displayQuantityService → operation_applier/display_units → units::convert_for_dimension` seam, for subnormal and zero projections.
   - Scientific transport: whether and where the product adopts `openpipestress_jcs_binary64_v1`, and T1's note on the `openpipestress_jcs_ijson_v1` 2^53 − 1 integral limit. Say what belongs to T3 and what to T6.
   - A comparison policy for new T3 cases. `STAGE0_MAP.md` §5.3 recommends leaving every protected predicate unchanged. If you conclude a protected predicate must change, give ROOT concrete options with a recommendation and name the approving instrument you believe applies.
7. **Standing across languages.** Every change to `numerical_use_standing*` (Rust, Python, TS) and to the static fresh-identity sets, with the T0R and T1 standing edits as the baseline. Readers must stay in outcome parity across languages; say how that is tested.
8. **Slices and file plan.** Use `STAGE0_MAP.md` §3. Mark what can land before T1 merges (disjoint readers `source_blocks.rs`, `physics_source.rs`, `source_blocks.py`, `physics_source.py`, the display services, units and transport crates) and what waits.
9. **Verification plan.** Reader mutation and tamper controls, positive controls, cross-language parity, the probe reproductions, and which existing tests change (each change stated and justified; none deleted to make something pass).

**Out of scope:** the numerical method, range scaling, sparse assembly and VP-ROBUST (D1's). Where your items depend on D1's evidence (for example, what a general-method receipt contains), state the assumption in an interface section and keep the design valid if D1 chooses differently.

## Basis to read

`STAGE0_MAP.md`; `DEFAULT_ROUTE_DESIGN/{DESIGN,ROOT_SELECTION,ROOT_RULINGS}.md` (§5.7 in particular); `CORRECTNESS_DESIGN/COMPOSITE_ENGINE/**`; `ENGINE_INTEGRATION/RETURN.md`; `SCIENTIFIC_TRANSPORT_FOUNDATION/RETURN.md`. From T1 at `f3270ea79`: `LOAD_STATE_IMPLEMENTATION/{T1_WAVE1_RULINGS,CHECKPOINT_4,CHECKPOINT_5,CHECKPOINT_6,CP4_WIRE_ADDENDUM,CP2_WIRE_ADDENDUM_2}.md`, `T1_WP1_JOINED_READERS/RETURN.md`, `T1_WP4_HEADLESS/RETURN.md`, `REVIEW_CHECKPOINT_3/`, `REVIEW_CHECKPOINT_4/`. Product source: the readers and standing functions in `P/core/reporting/result_export/src/`, `P/core/analysis_runs/`, `apps/desktop/src/features/results/`, `P/core/runner/headless/src/result_envelope_binding.rs`, the facade entry in `PP`, and `source_receipt*`, on main and on T1.

## Probes

Probes run only after the manager says ROOT has released the host, in `<scratch>` with `<t3-target>`. A probe of T1's candidate builds from an exported copy of `f3270ea79` in `<scratch>` (`git archive f3270ea79 P/core ...`), never from T1's worktree. Record probe sources with `.txt` suffixes.

## Write set

`T3/DESIGN_STANDING/**` only: `DESIGN.md` (the return), and `_run_records/` with probe sources, outputs and `SHA256SUMS`.

## Return

`T3/DESIGN_STANDING/DESIGN.md`: recommendation in brief; current state with citations on main and T1; options and the recommended design per item; the interface assumptions on D1; slices, order and T1 serialization; verification plan; what T3 completes and what remains; decisions for ROOT, with any owner-level question separated and given options plus a recommendation; sources, probes and limits. Then send the manager a SendMessage summary.
