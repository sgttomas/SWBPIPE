# T08 Text at the D1 caps (replaces G2 RESIDUALS.md:146–193; RV83 B-2, S-6)

**Result.** Every text-producing site on the D1 call graph is covered by exactly one stated bound, or by a stated exclusion with a call-path argument. At the caps:

| Quantity | Caps | Milestone (sanity) |
|---|---|---|
| Total text allocation volume, requested (TAV_req) | **1,540,955,362 B** | 72,506,030 B |
| Moving: TAV_req plus the largest single text site | **1,543,555,324 B** | 72,708,116 B |
| Diagnostics D, every diagnostics vector on the graph, including the receipt replays' local ones (a fixpoint: the `diagnostics.iter().find` loops use D) | **25,544** | 993 |
| Diagnostics D_env, the ordinary envelope's own (the same budget without the receipt finalization and without the W1 root) | **11,030** | 423 |
| Retained diagnostic bytes (id, code, severity, source and message copies, plus refs); all vectors / the envelope's | **153,555,876 B / 77,812,844 B** | 6,953,671 B / 3,753,544 B |
| Sites inventoried / reached / with positive multiplicity | 2,563 / 2,381 / 1,223 | 2,563 / 2,381 / 1,210 |

`_run_records/text_budget.caps.out.json` reports **`complete: true`**: no unmapped loop header and no unclassified argument remains on a live path. Text needs no ASSUMED layout: every number here is bytes.

**The bound is total allocation volume, not peak.** Every live String was allocated by some site in the inventory, at most once per execution of that site, with capacity ≤ max(8, 2·length) (J9/V3). So the sum over sites of multiplicity × capacity bounds the live text at every instant. Moving text adds one old backing, because the invocation is single-threaded and only one reallocation is in flight at a time. The bound counts transient text as if it were retained, so it is loose by construction. A lifetime-aware model would tighten it; it is not needed for soundness.

## 1. Which sites (B-2)

**The rule.** A site is in scope if its function is reachable on the over-approximate call graph (`_run_records/callgraph.py`) from either D1 root:
- the retained Direct entry, `run_linear_static_preview_value_with_retained_direct` (PP/lib.rs:2184);
- the W1 preparation entry, `prepare_observed` (PP/retained_product.rs:3265). At this NUM revision the Direct entry does not yet call it: U3 wires it. It is included so that the W1 producer's text is covered now.

That is 1,881 reachable functions across 15 crates. Files come from the graph, not from a list.

**The lexicon.** Macro sites come from G2's 707-row inventory plus the G3 extension (1,147 rows: format!, write!, writeln!, format_args!, diag). The non-macro sites (`text_lexicon.py`) are:

| Kind | Rows | Meaning |
|---|---|---|
| `to_string` | 561 | a Display invocation, so a new String |
| `into_text` | 221 | `.into()` of a literal or of a text-named value |
| `clone_text` | 586 | `.clone()` of a receiver whose last path segment names text (`id`, `message`, `unit`, …) |
| `join` | 17 | `.join(` producing a String (77 status-lattice joins produce no String and are dropped) |
| `replace`, `to_owned`, `push_str` | 5, 5, 18 | string building |
| `diag_literal` | 3 | the `Diagnostic {` struct literals in self_weight.rs, which bypass `diag(` |

**Not text, and listed for review** (`text_lexicon.out.json`):
- 252 data `.clone()` sites. These are priced by their T05, T07 or T25 family; the PP ones are model, material, force, report and record clones.
- 48 `.into()` conversions of errors and Values.
- `json!` constructions, which are Value owners priced by T25 and T05.

**Implicit calls:**
- Display/Debug impls write into the caller's buffer, so the placeholder class at the caller bounds them (§3). Their own `write!` sites are also counted once (redundant, so conservative).
- `?`-operator `From` conversions in the reached crates construct no text: their error enums hold `&'static str` and scalars (§3).

**The call graph is an over-approximation.** Resolution is by name with these refinements:
- impl-owner and receiver-type inference;
- the crate-dependency closure;
- Rust's resolution of bare calls to the caller's own module (G3; E0255 makes a same-named import impossible);
- `module::name(` resolution, with multi-segment paths such as `a::B::f(` keeping their last qualifier (G3 fix: earlier versions dropped these calls entirely, which hid T25's finalize path and T07's `exact::Context::…` calls);
- locally bound closures shadowing same-named functions (G3);
- 47 adjudicated rules carried from STACK work (`callgraph_rules.json`).

**Residual risk:** an untyped method call fans out to every same-named method. That over-counts multiplicity but never under-counts it. The two fan-outs that mattered (`LoadLedger::push` and `force_scaled_term`, about 3·10⁸ name-matched calls) are replaced by explicit per-invocation totals (§4). The graph is a lexical tool, not a compiler; RV84 should spot-check it.

## 2. Multiplicity

For each function f, M(f) is computed by a DP over the graph's condensation in topological order: M(f) = Σ over call sites of M(caller) · Π(loop bounds enclosing the site). A site's multiplicity is M(f) times the bounds of its enclosing loops. Loops are brace loops and iterator-adapter closures (`loopscan.py`).

**Loop bounds** (`loop_bounds.json`, 113 rules): each is a regex on the loop header with a cap expression and a source reason.
- Option and Result adapters count once.
- Array-literal headers count their elements, ahead of the keyword rules (G3 fix).
- Joint pairs are three rules: supports × restraints → r; nodal × primitive loads → l·l; the free-DOF triangle → F(F+1)/2.

**Exclusions, each a call-path argument:**
- **File exclusions (5):**
  - source_receipt/composite.rs: `composite` is false; `physical` is never Some for pre-0.4 input;
  - pressure_material.rs (`!is_exact`);
  - case_state/ (no load-state document);
  - result_export/: T17, priced in G4;
  - canonical_json rendering: the hash-route law prices it wherever text is hashed.
- **Function exclusions (32):**
  - self_weight's validation module, under the proposed **D1.10** (G2_AMENDMENTS.md §5);
  - `resolve_shared_sections` (no `section_ref`);
  - `validate_profile`, `build_pressure_case_with_members` and `append_exact_pressure_results` (no exact pressure);
  - `validate_document` (no load-state keys);
  - `add_uniform_element_loads` and `build_thermal_element_loads` (nodal loads only);
  - `build_nonlinear_supports` and its two field-diagnostic helpers;
  - the three spring-hanger validators and `spring_hanger_diag`;
  - `append_equivalent_static_generated_loads` (`equivalent_static` is None).
- **Site exclusions (27):** the bodies of `diag()` and of the excluded self-weight branch, doc comments the macro lexer matched, and data clones.
- **Caps on M:**
  - `solve_load_case_observed` ≤ cases, because its three call sites are mutually exclusive per case;
  - `blocked_envelope` ≤ 1, because it is returned;
  - linear_supports `SupportFinding::new` ≤ 4·(g + r) and `add_restrained_dof` ≤ 4·r: per `prepare_boundary` call at most one finding per support plus one per raw restraint, and `prepare_boundary` has multiplicity 4.
- **Edge exclusion (1):** `load_row_finding` → `decide_row` (nodal-only: no formation records).
- **Once-only edges (2):** the exclusive branches of `normalize_quantity`, and the load-state fallback.

Every exclusion's reason is stored with it in `loop_bounds.json` or `text_args.json`.

**Explicit per-invocation totals**, replacing M × size where name fan-out or error paths inflate M:
- `LoadLedger::push` and `push_unique` source copies: ≤ 5 ledgers × l ids each;
- `force_scaled_term`'s source clones: ≤ 4·l;
- **receipt error paths.** All 127 `bad(` calls in source_receipt.rs, rows.rs and source.rs build an error that is returned at once. The receipt has three catch points, so at most 4 errors exist per invocation, and 8 are charged (source_receipt.rs:28, :31, :55);
- **commitment pointer strings** (source.rs:147): `function()` runs once per retained descriptor, so the terms total ≤ the 16,384-unit limit over all calls;
- lib.rs:1266/1270 (load-fidelity row strings): one call covers all N rows, and each identified source appears in exactly one row (a nodal term sits on one DOF), so the sources Debug totals l·(6·128+4) per call rather than per row.

## 3. Spelling classes (S-6 included)

| Class | Bytes | Source |
|---|---|---|
| f64 Display / Debug / LowerExp | 327 / 24 / 24 | core/fmt/float.rs (G2, confirmed by RV83 N-4) |
| integers | 20 (usize, u64, i64); 11 (i32); 39 (u128) | decimal digits plus sign |
| `{:016x}` / `{:032x}` / a SHA-256 digest `{:x}` | 16 / **32** / **64** | hex width; FK/wide.rs:218 `{:032x}` (S-6); `format!("{:x}", Sha256::digest(..))` (source_receipt.rs:36) |
| identifier, or any input string value | 128 | D1.9 text cap |
| **str/String Debug** | **6·len + 2 = 770** for 128 B | `escape_debug` emits ≤ 6 output bytes per input byte (`\u{1f}`) plus 2 quotes (S-6; G2's 2 + 10·len is withdrawn) |
| literal used as text | ≤ 426 measured on the reached graph; class `static` = 2,173 (the longest literal in the four core files) | `text_lexicon.py` raw measurement |
| `&'static str` constant | ≤ 339 | the longest `const …: &str` in PP and FK |
| composite id | 600; result id 1,024 (the longest reached template evaluates to 872) | template inventory |
| **error Display/Debug** | **8,192** | every D1-reachable error type (below) |
| composite Debug | StructuralReport 2,227,765; `integrity_dof_map` 252,288; the integrity message 2,549,385; sources 246,912; evidence line 50,576; joined list 320,556 | `composite_text.py` (derived Debug, structural formulas over the caps) |

**Error Display/Debug ≤ 8,192 at the caps.** The largest is `StructuralError::NegativeEnergy` Display, which embeds Debug of `direction: Vec<f64>` (FKS/structural.rs:275–300): 22 + 28 + (2 + 192·24 + 191·2) + 10 + 24 + 13 + 24 + 2 = **5,115**. The others:
- `Mechanism` is smaller;
- `UnitError` ≤ 128 + 78 + 327 + 78 = **611** (units/src/lib.rs:1083–1124; the longest units literal is 78 B);
- `StraightPipeError` and `FrameKernelError` hold only `&'static str` and scalars, so ≤ 600;
- `ExtremaError` Debug holds scalars and a `CertifiedStressMaximum` of scalars, so ≤ 600;
- `serde_json::Error` ≤ 400: the message quotes at most one 128-byte input string;
- `RecoveryError` Debug wraps `exact::Error::Arithmetic(StructuralError)`, so ≤ 5,115 + 60;
- `AttemptStop`, `CaptureError` and `HelperError` are static labels and scalars.

**Constructor parameters** (`impl Into<String>`: `SupportFinding::new`, `LoadFinding::new`, `bad`, `FormationCheck::unavailable`). Every caller passes a literal (copied, ≤ 426 B), a `format!` result or a `to_string()` result. The last two are moved without allocation and counted at their own site.

## 4. The largest contributors (caps)

| Function | TAV bytes | Note |
|---|---|---|
| retained_product.rs `validate_final_metadata` | 196.8 M | W1: per final row (2·P_final), the expected id and metadata strings are formatted and compared |
| PP `normalize_quantity` / `unit_conversion_diag` | 196.3 M / 123.3 M | per quantity, over 4 normalizations (ordinary, T25 rebuild, …); ids are eagerly formatted |
| PP `prepare_rigid_support`, `build_model_for_members` | 76.6 M, 74.1 M | per support × per restraint: the joint crosses a function boundary, so this is over-counted |
| T25 source.rs `function` | 57.1 M | per retained descriptor (Fn = 1,760) |
| PP `integrity_diagnostic_id` | 56.9 M | one id formatted for every diagnostic compared (`assessed_numerical_quality` × D) |
| PP `solve_load_case_observed`, `support_contribution_summary`, `range_scaling_evidence_line`, preview `render`, `check_support_maps`, PL `LoadFinding::new`, `g5a`, `append_integrity_report` | 30.8–52.6 M each | the composite 2.6 MB messages and per-row texts |

## 5. Text atoms for the other records (`t08_closure.py`, `text_closure.caps.json`)

| Atom | Caps | Meaning |
|---|---|---|
| Text(row) | 11,474 B | string content of one ResultItem: id and entity_ref ≤ 1,024; kind, unit, ref_type, component and coordinate system ≤ 426; ref_id ≤ 128; location, basis and sign convention ≤ 1,024; ≤ 4 source refs ≤ 1,024 each |
| Text(diag_total) | 153,555,876 B | the content of every diagnostics vector (all, including the replays') |
| D | 25,544 | bound on the length of every `Vec<Diagnostic>` |
| Text(diag_env), D_env | 77,812,844 B; 11,030 | the ordinary envelope's own diagnostics, used for the T25 publication Value and for CompleteFacts |
| Text(err), Text(audit_error), Text(formation_detail) | 16,384 / 16,384 / 16,426 B | one retained error String's capacity (2 × 8,192, and so on) |
| Text(sym) | 368 B | the symmetry-basis String (2 × 184) |
| Text(recovery_finding) | 10,466,306 B | I54's RecoveryFinding polynomial, 1022 + 16·F_len + 2·old message + 8·s(String), with capacity 2× on the variable part |

T05 and T25 use these for **copies** of rows and diagnostics: Value conversions and clones, which are not text sites. Those copies are added on top of TAV. The originals are already in TAV, so the T05 diagnostics row prices only the `Vec<Diagnostic>` backing.

## 6. What remains, and how the bound can be tightened

**Nothing is open at the derivation level.** The bound is sound and complete for the D1 call graph at NUM `5ae5fe4f0f`, given:
- the proposed D1.10. Without it the same method gives about 10.6 GB, and the module's own loop headers then need rules;
- the call graph's over-approximation;
- the review of each exclusion's stated argument.

It is loose because it counts transient text as retained. If ROOT wants headroom, a lifetime-aware text model would cut it substantially: frame-local temporaries die at frame exit, and the giant composite messages are built once. That would be a separate grant.
