# G3 repairs routed to G4 (RV83 R-1 to R-3; RV84 S-1 to S-4, S-7, N-2, N-3, N-7, N-11, N-13), and the T08/T20 reruns

**Each item is reported separately.** S-5 is in COMPOSITION_G4.md and S-6 in API_G4.md.

**Basis:** NUM `b1f80234dc`, read from a `git archive` snapshot. The G3 rules were carried from `5ae5fe4f0f` through `3260d7809e` to `b1f80234dc` by `linemap.py`. That script re-points a line key only when the old line defines the same function (or carries the same text site) and the new line does too, and stops on any key it cannot map. Its logs are `linemap_*.out.json`.

**Tools:**
- `callgraph_g4.py` (with `CG_REPAIR=0 CG_LEXER=regex` it reproduces G3's graph edge-for-edge; checked at `3260d7809e`);
- `loopscan.py`, `text_lexicon.py`, `text_budget.py`, `template_inventory.py`;
- `run_text_g4.sh`;
- `recursive_types.py`.

## 1. RV83 R-1: the call graph

### 1.1 The repairs, one per pattern (`callgraph_g4.py` header)

Each pattern below either dropped calls or dropped functions. Where it **added** edges, the added edges are over-approximate name fan-out.

| Id | Pattern | What G3 did | Repair |
|---|---|---|---|
| R1a | **Generic, trait and alias receivers** (RV83 R-1(a)): `equations: &R`, `impl Trait`, `dyn Trait`, type aliases, `Self::X`/`T::X` paths, and `self` in a trait's default body | resolved to a "type" that owns no methods, so the edge was dropped | fan out to every same-named method in the crate-dependency closure |
| R1b | **Blanket impls** (`impl<T..> Tr for T`) | their methods matched no receiver | they are targets for every receiver |
| R1c | **Chained calls** (RV83 R-1(b)): `f(x).g(`, `x?.g(`, `x.unwrap().g(`, `}.g(`, a `.g(` after a line break | matched neither `CALL` nor the receiver pattern; `METHOD` was defined but unused | every `.g(` the receiver pass missed is handled (see R1l) |
| R1d | **Turbofish and trait-qualified paths**: `Type::<T>::f(`, `<X as Tr>::f(` | dropped (the `CALL` lookbehind excluded `:`) | `Type::<..>::f` resolves on Type; `<X as Tr>::f` on every impl of Tr (any trait impl if Tr is external); any other `<..>::f` fans out |
| R1e | **Generic, trait or alias path qualifiers**: `G::f(` | resolved on a type with no methods | fan out |
| R1f | **Implicit calls** (`CG_IMPLICIT=1`, used for the recursion check only): `?`/`.into()` reach user `From::from`; formatting reaches user `fmt`; serde entry points reach custom Deserialize/Visitor/Seed; `+`/`-` near WorkTotal reach its Add/Sub; the stress-recovery heap reaches Node's Ord/Eq; Deref/Drop | not modelled | §3 |
| R1g | **String and char lexing** (found by my own audit) | the regex blanker failed on a `\`-newline continuation (`.` does not match a newline), so string and code regions inverted. It also cut a string at a `//` inside it, and misread a `'"'` char literal. Two non-test files were mangled: nonlinear_integration lib.rs from :2450 (`nonconverged_exit_diagnostic` lost all edges) and canonical_json binary64.rs from :170. PP lib.rs's damage was confined to its test module | a single-pass lexer (`loopscan.blank_rust`) that handles line and nested block comments, escapes including an escaped newline, raw strings with any `#` count, byte strings, and char literals versus lifetimes, and keeps every newline |
| R1h | **A call right after a single `:`** (= RV84 S-1): `"k":f(x)` in compact `json!`, `field:Vec::new(`; also after `..` (`..Default::default()`) | dropped by the lookbehind | accepted when the preceding `:` is not part of `::` |
| R1i | **Array types in signatures** (my audit): a `;` inside `[T; N]` in the parameters or return type was taken for a body-less declaration | **194 functions** were not nodes at all (their own calls were lost, and calls to them dropped), e.g. FK `FrameNode::new`, `element_dof_map`, `FrameElement::new` | the first `{` or `;` at bracket depth 0 after the parameter list decides |
| R1k | **Test modules** (my audit): G3 cut each file at its first `#[cfg(test)] mod {` | dropped production items after a mid-file test module: FK adaptive.rs after :5152 (`SourceBridgeView`, `SourceCorrectionWork`, …). **At `b1f80234dc`, U3 adds `retained_tests_hooks` at PP lib.rs:3002, which would have dropped 10,000 lines of PP lib.rs** | every `#[cfg(test)]` / `#[cfg(any(test, ..))]` item is blanked in place, wherever it sits |
| R1j | (refinement) **Visibility** | — | a crate-local fn (no plain `pub`, and not a trait method) is never a target in another crate |
| R1l | (refinement) **Chain types** | — | a chained `.g(` resolves on the union of the declared return types of every candidate of the call just before it (after `?`, `unwrap`, `expect`, `clone`, `as_ref`). It fans out when any candidate's type is generic, a trait, an alias or unknown, or when the head is a local closure. The candidate set over-approximates, so the union contains the true type |

**The audit.** The output records, for every call-shaped `name(` whose name is defined in the scanned set, which pass handled it and how many edges it produced (`callgraph_audit.out.json`; `unresolved_call_tokens` = 5,123).
- **After the repairs, no token is "NOT MATCHED BY ANY PASS"**: before R1h there were 203.
- **The remaining zero-edge tokens are std or external calls:**
  - receivers of std `Vec`, array or `HashMap` types;
  - `Type::f` on types without that fn, i.e. derived `Default`, external `Vec::new`;
  - method names that exist only as free fns;
  - local closures.
- **The repaired graph:** 3,320 functions, 13,112 edges, 71 adjudication rules (24 new in G4, each citing the receiver's actual type).

**RV83's evidence, re-checked on the final graph** (Direct root, `b1f80234dc`):
- **All 64** generic-receiver-hidden functions are reached.
- **33 of 36** chained-hidden definitions are reached. The other three:
  - primitive_loads `positive` is called only from FK, which does not depend on primitive_loads;
  - adaptive.rs's two `into_legacy` are not the target of combine.rs:116. That receiver is `RecordedCombination` (`solve_recorded`'s return type), so R1l resolves to combine.rs:91.
- endpoint_maximum.rs:283 `add`, which RV83 also listed, is a free fn, so `.add(` cannot call it.
- **FKS/formation_check.rs is reached:** `check` and `detail` with its 5 `format!` sites.

### 1.2 The stated method, replacing "never under-counts" (TEXT.md:54)

**What the graph is.** A **name-based, crate-scoped static graph**:
- every non-test `fn` with a body is a node, in the 15 crates PP links (plus `result_export`, now a runtime dependency);
- an edge is added for every lexical call whose name matches a definition the caller's crate can reach;
- resolution narrows only where Rust's rules make the narrowing certain: the declared receiver or parameter type, same-module bare calls, `module::f`, inherent-before-trait path resolution, visibility, and one-step chain return types.

**Known limits.**
1. **Implicit calls are not edges in the TEXT graph.** These are operators, `Deref`, `Drop`, `?`'s `From`, formatting's `Display`/`Debug`, `Iterator::next` in `for`, and serde-derived code.
   - They are covered by separate arguments: §3 for recursion; TEXT's Display placeholder rule; the `from_literal` lexicon kind (§8, N-2) for `From<&str>` text.
2. **Calls through function pointers stored in data, or through `dyn Fn`,** are covered only where the fn's name appears as a value. RV84 found no text behind any.
3. **Macro-generated calls** count once per invocation site (RV84 §3).
4. **Name fan-out over-approximates.** Multiplicities are upper bounds, which can be loose (§2).
5. **Adjudication rules** replace a call's targets on a cited reading of the receiver type. A wrong rule is the one way to under-count, so every rule carries its evidence (`callgraph_rules.g4.json`).
6. **Tests and `bin/` targets are excluded by construction.** `bin/` targets are not linked into the library.

### 1.3 Reachability, and what the repairs exposed

**From the Direct root at `b1f80234dc`** (which now reaches W1, the serializer and the reader): 2,698 functions. G3 reached 1,881 at `5ae5fe4f0f` with two roots.

**What the repairs exposed:**
- T25's source-commitment text (`source.rs`; S-1): 71.2 MB of TAV;
- the U1 capture and serializer;
- the precommit reader (§2);
- functions G3 never saw (R1i, R1k).

## 2. T08 rerun on the repaired graph (TEXT, replacing TEXT.md's figures)

### 2.1 Configuration

**The run** (`run_text_g4.sh`, reproduced by `sens.py`):
- Root: the Direct entry only (D-2).
- PP → result_export is a runtime dependency (`CG_EXTRA_DEPS`, decision 5; merged in U3).
- G3's rules, carried by line map, and the rules added in G4 (`loop_bounds.g4.json`, `text_args.g4.json`):
  - loop bounds for every loop on newly reached functions: the T25 commitment, the U1 serializer and capture, the reader, rigid-body and formation-check;
  - `loop_total` rules for the functional-unit totals: ≤ 16,384 units on the selected path, from functionals.rs:409–470;
  - `site_from` rules for error-path allocations (each bounded per call of a named owner);
  - `edge_per_call` for R-2;
  - edge zeros for exact-only and non-preview reader arms;
  - `fn_cap` for one ordinary run per invocation (S-7).
- `result_export` is no longer file-excluded: its text is the reader's.
- The diagnostics count D is iterated to its fixpoint (`TB_D`).

### 2.2 Results (complete: no unmapped loop, no unclassified argument)

| Quantity | Caps | Milestone | G3 (caps) |
|---|---|---|---|
| TAV, whole invocation | **2,144,966,676** | 109,697,874 | 1,540,955,362 |
| TAV_X (branch X: retained_w1 returns at :2955) | **1,445,193,238** | 81,102,794 | — |
| TAV_W (branch W: no selected finalization) | **1,583,158,378** | 79,456,986 | — |
| Largest single site | 2,599,962 | 202,086 | 2,599,962 |
| D (every diagnostics vector) | 17,574 | 658 | 25,544 |
| D_env (the envelope's own) | 11,408 | 423 (= G3) | 11,030 |
| Text(diag_env) | 78,860,051 | 3,590,229 | 77,812,844 |
| Text(diag_total) | 109,065,359 | 4,616,149 | 153,555,876 |
| Sites inventoried / reached / positive | 2,712 / 2,631 / 1,372 | | 2,563 / 2,381 / 1,223 |

**Why the figures moved.**
- **TAV rises** with the newly reached text: the reader (276 MB, TAV-counted), S-1's commitment text, S-2's real loops, R-2, and R-1's functions.
- **It falls** with the single Direct root (S-7) and the D fixpoint.

**The per-branch totals (§2.3) are what the composition uses.**

### 2.3 Per-branch text (new)

Branches X and W are exclusive per invocation (one case, D1.4):
- **TAV_X** zeroes `retained_w1` (lib.rs:2950), which returns at its coexistence check on X (:2955–2957);
- **TAV_W** zeroes the selected finalization: `FinalizedSourceBlockCase::exact` (source_receipt.rs:636) and `FinalizedSourceBlockReceipt::finalize`/`finalize_composite`/`finalize_for` (:839–867). These run only for a selected source (lib.rs:2852–2862, :5223–5231).

Both remain whole-invocation totals for their branch.

## 3. T20 rerun: recursion on the repaired graph (RV84 N-3; RV83 R-1)

**Explicit graph** (`callgraph.out.json`):
- **22 cyclic components, every one a single self-recursive function. There is no mutual recursion.**
- 19 are reachable from the Direct root:
  - G3's 18;
  - retained_wire.rs:1310 `safe_integers` (U1), a recursion over the body Value, ≤ 12 levels.
- 3 newly visible (R1g) and unreachable: canonical_json binary64.rs `validate_value`, `write_value` and `Parser::value`, each also guarded at depth ≤ 128.

**The cycles the repairs created by name fan-out were adjudicated** with 24 cited rules. Examples:
- `.collect()` on std iterators;
- `WorkTotal::status`/`add`/`mul` (RV84's traced collision);
- `WidthWork::checked_lme`;
- `BodyEvidence::geometry`;
- `FrameElement::length`/`local_stiffness`/`global_stiffness` behind `self.frame_element()?`;
- `Option::<T>::deserialize`;
- the `SelectedCandidate` delegations in retained_wire.rs:1485–1497, where an inherent method takes precedence in path resolution.

**Implicit calls** (`CG_IMPLICIT=1`; `stack_implicit_candidates.json`). Modelling implicit calls bluntly (any formatting reaches every user `fmt`, any `?`/`.into()` every user `From`, …) adds **18 candidate components**:

| Candidate | Adjudication |
|---|---|
| 13 `fmt` self-loops and pairs (error Displays and Debugs: LinearSupportError/SupportApplicationError, PrimitiveLoadError/BoundaryMetadataError, CurvedBendError, StraightPipeError, StressRecoveryError, SparseDirectError, DiagnosticsError, NonlinearSupportError, NonlinearIntegrationError, UnitError, CaptureError, AlgebraError, …) | false. Each body formats only its own fields (primitives, `&'static str`, Strings or other error types) or delegates `{self:?}` to Self's derived Debug. None formats a value of its own type, and no involved type is recursive: `recursive_types.py` over 862 types finds only `NonlinearFrameSolveResult ↔ ProductCompletion`, depth ≤ 2 by construction ("the nested output snapshot has no receipt", nonlinear_integration lib.rs:349–360) |
| The 60-function FK cluster (exact_sum helpers, LoadLedger/AssembledForce/FormationRow/StageWork/AttemptRecord/WidthWork/SumWork/ExactWideSum/WorkTotal/Wide Debugs, structural error Displays) | false. The Debug impls call leaf helpers (`round`, `finish`, `is_exact`, `sig`). The helpers' `?`/`expect` convert within `SumError` (no user From is involved) or format `SumError`'s Debug, which calls nothing |
| `Ratio::eq` ↔ `same_terms` (exact_boundary.rs:157–165) | false: `same_terms` compares `f64` bits |
| elastic_extrema `Node` `eq`/`partial_cmp`/`cmp` (:120–137) | false: `eq` and `partial_cmp` call `cmp`, which compares `f64`/`usize` fields |
| adaptive.rs:5601 `From<WideError> for SourceCorrectionError` | false: its `.into()` converts WideError into AttemptStop, a different impl |
| canonical_json `Seed::deserialize` ↔ `ValueVisitor::visit_some`/seq/map (lib.rs:45–90) | **genuine**: the checked parse's recursion. Bounded by serde_json's `remaining_depth` (128) and the input's depth (≤ 18), as G3's STACK_INVENTORY row "serde_json parser" states |
| PP `Authored<T>`, `PreviewModel`, `LinearStaticPreviewRequest` custom `deserialize` | **genuine nesting, bounded**: the request's Deserialize descends the typed request's non-recursive types through the generic `Authored<T>`, so depth ≤ the raw depth (17) |

**R and k are unchanged:** 64 MiB and 16.
- The deepest explicit chain from the Direct root is 39 frames (G3: 34). It now includes the W1 dispatch and the reserved-thread closure.
- With 18 Value levels and 36 reader schema levels at ≤ 16 KiB per frame, the total is still under 1.6 MiB.
- G3's open item on the reader's deepest schema chain stays with G5's witness (PUBLICATION_READER.md §3).

## 4. RV84 S-3: the hash route's per-string temporary

**The repair.** The canonical render (CJ lib.rs:132, :155) allocates `serde_json::to_string(text)` for each string and key, one at a time. Its capacity is ≤ max(128, 2·(ε·L + 2)).
- The route (`t25_g4.py`, `g4_caps.py`) now adds that temporary at the hashed Value's **L_max**, plus the parser's escape scratch, ≤ 2·L_max.
- **L_max per hashed Value:**
  - the publication: the integrity message, 2,549,385 B (composite_text `message_int`);
  - the T25 payload: the source identity JSON, 1,797,413 B (T07);
  - plans, bodies and sources: the longest id or literal class, 2,173 B, or a diagnostic id ≤ 2,330 B.
- **The effect** at ε = 2 is about 10.2 MB for the publication route and 7.2 MB for the payload route. T16 and T17 reuse the corrected route.

**parsed()** now prices push-built arrays as 6 × slots. Only non-empty arrays allocate, and max(4, 2h) ≤ 6h. This replaces G3's 4 × objects stand-in, which RV84 found could under-count.

## 5. RV84 S-4: T25's commitment coefficients (`t25_g4.py`)

**Added to the payload:**
- the `source_identity` string (1,797,413 B; source.rs:376);
- `stiffness_aggregate_bits`: N × N strings in N + 1 arrays;
- `force_aggregate_bits`: N strings.

**Added to the functions:** one `{kind, source_path, bits}` factor object per functional atom, ≤ 16,384 units (functionals.rs:409–470), in one array per product.

**T25 at the caps:**

| | Requested | Moving | G3 requested |
|---|---|---|---|
| ε = 2 | 1,475,185,857 | 209,737,314 | 1,439,324,474 |
| ε = 6 | 3,147,084,185 | 622,613,126 | 3,074,072,410 |

The peak stage is unchanged: I1, the body `json!` with `hash(publication)`. S4 is now 0.69 GB at ε = 2, and I1 1.42 GB.

N-6 (selection is infeasible at the full caps) is a lever for X only; X passes the margin rule (COMPOSITION_G4.md §4).

## 6. RV83 R-2: `validate_profile` priced

**The exclusions are removed:**
- the `fn_zero` for `validate_profile` is dropped;
- `build_pressure_case_with_members`'s exclusion is replaced by edge zeros for every callee after its `!is_exact` return (pressure_runtime.rs:436–438);
- `check_suffixes` is zeroed with its citation: it is called only inside `if exact`.

**What is priced.** The edge `validate_profile → problem` runs ≤ l times per call (`edge_per_call`). Under D1.3 only the `!exact` arm runs: PRESSURE_MODEL_REAUTHOR_REQUIRED once per pressure-category load (:207–225).

**The cost: pressure_runtime.rs is now 25,622,784 B of TAV** at the packet's byte classes (diag 10,240, join 8,320, format 2,330 per firing). RV83's estimate at real spellings is about 0.4 MB.

**No new D1 clause** (ruled).

## 7. RV83 R-3: O-N's provenance-parse term

**Added to ORDINARY's O-N** (`ordinary_caps.py`): the per-load `serde_json::from_str::<Value>(provenance).ok()` transient (self_weight.rs:796–799), one load at a time. Under D1.10 a provenance that parses is a non-object Value of ≤ 128 bytes:
- ≤ 64 values and ≤ 64 arrays (each takes ≥ 2 bytes with its separator);
- ≤ 42 objects nested in arrays, with ≤ 25 entries in all;
- ≤ 128 string and key bytes.

**The term:**
- s(Value)·(4·64 + 2·64), push-built arrays;
- + Node(String, Value)·48;
- + 384 B (string and key bytes, plus one boxed `Error` on a failed parse).

**It is 48,000 B at the illustrative strides,** against RV83's "under about 30 KB" at Leaf_up 640. The "about 40 B" text in G2_AMENDMENTS §5 is corrected by this row.

## 8. RV84 N-2: `?` into `CaptureError` (TEXT.md:44 corrected)

**The claim "?-operator From conversions construct no text" is false.** `impl From<&str> for CaptureError` (retained_product.rs:3150–3154) allocates the literal. So does std's `From<&str> for String` for any `?` into a `String` error.

**The repair.** The lexicon adds the kind `from_literal`: an `ok_or("…")?`, `ok_or_else(|| "…")?` or `map_err(|..| "…")?` in a function whose return type's error is `String`, `CaptureError` or `Box<dyn Error>`.
- 85 rows, 52 with positive multiplicity, 7.8 MB of TAV at the caps.

## 9. RV84 S-2: the broad zero rules

**The three broad rules are anchored:**
- `components` becomes `model.components` and its iterator forms;
- `wind` becomes the whole word, so it no longer matches `windows`;
- `intensity` becomes `uniform_intensities`.

**Specific rules come first** for the real D1 loops RV84 found:
- the 6-component tables at lib.rs:11156 / 11246 / 11321;
- the 4-component stress tables at :12609 / :12702;
- retained_product.rs:2162's 3-array;
- `boundaries.windows(2)` (:9986, :10109), bound 1 (D1.7);
- the 3-accumulator zip (:10012).

**The self-check.** `text_budget.py` now lists every header a zero rule matches (`zero_matched_headers`, 57 at the caps). Reading that list found **one more**: preview_physics.rs:222–224 `ambiguous_supports`. Its adapter header spans the preceding loops and contains "nonlinear". It is now bounded by r.

Every other listed header iterates a collection D1 forces empty: components, combinations, nonlinear, curved, thermal, pressure, load-state, wind, user elements, directional springs or gates.

**These loops add 97.3 MB of TAV at the caps.**

## 10. RV84 S-7: the double-counted ordinary route

**Resolved in the text model.** The root is the Direct entry only. `run_linear_static_preview_observed` is capped at 1 (`fn_cap`), on U3's structural proof of one ordinary run per invocation (TRANSFER_COMPLETION.md B-1). The per-case caps (`solve_load_case_observed ≤ cases`) are now consistent with that premise.

## 11. Wording corrections

- **N-7** (RESIDUALS_G3.md:64). "26 `checked_mul`" should read: "26 `checked_mul` across four PP files: retained_product.rs 15, case_state/temperature.rs 8, source_recovery.rs 2, retained_memory.rs 1".
- **N-11** (COMPOSITION.md §3). G3's W moving term carried an undocumented 2 MiB candidate. G4 removes it and enumerates the candidates (COMPOSITION_G4.md §1). T12–T15's largest single old backing is below the 8,388,608 B maximum-helper growth, on RV84's reading.
- **N-13** (TEXT.md §2 top-ten). The listed multiplicities are **over-counts** of the true executions:
  - mutually exclusive `match` arms in `validate_final_metadata` are counted together;
  - support × restraint products are counted per support;
  - `normalize_quantity`'s diagnostic runs at most once per call.

  They are sound upper bounds, not estimates.
- **S-6(d)** (COMPOSITION.md:84). The bound 155,625,688 is **2·Text(diag_env)**, not 2·Text(diag_total). At G4's atoms it is 2·78,860,051 = 157,720,102 (API_G4.md).
