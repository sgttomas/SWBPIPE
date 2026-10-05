# T20 Stack inventory under S1 (completes G2 STACK_PLAN.md; RV83 §7 items)

**Result.** No recursion on the D1 path is count-proportional. Every recursion is bounded:
- by the Value nesting of an input or product document (≤ 18 levels);
- by a fixed schema grammar (≤ 36, in the G4 reader);
- by the BTree height (≤ 7);
- or by a literal small factor (≤ 2).

So peak stack is a finite constant of the build for a given opt-level, and S1's structural premise holds. **R = 64 MiB and k = 16 are confirmed.** The witness at 4 MiB must exercise the chains in §3. G5 implements it; G6 records it.

## 1. The recursion inventory (W1 and ordinary paths)

**The method.** `_run_records/callgraph.py` builds an over-approximate graph: 2,982 functions and 10,610 edges across 15 crates, including the G3 resolution refinements and the multi-segment-path fix (TEXT.md §1). Tarjan's SCCs are taken over all of it. The G2 lead scan's 107 self-recursion candidates reduce to **18 genuine self-recursive functions**. There are **no mutual-recursion cycles**: the earlier 3-node `finite_tree` SCC is now three separate self-loops (`callgraph.out.json`).

| Recursion | Sites | Depth bound in D1 |
|---|---|---|
| Canonical JSON rendering and checked-value validation | canonical_json lib.rs:94 `validate_checked_value`, :125 `write_canonical` | one frame per Value level of the hashed document. Raw request ≤ 17 levels (census depth ≤ 16 counted from 0; G2_AMENDMENTS.md §2), +1 for the `json!` hash wrapper. The ordinary envelope, publication and receipt body are at most 12 levels: envelope → `source_block_recovery` → body → cases → case → supports → support → components → component → `action_terms` → term → leaf. The U1 successor's levels are fixed by the C1/C2/C3 grammar (G4). **≤ 18** |
| Result-export reader walkers (T17, G4) | `guard_json`, `same`, `finite_tree` ×3, `shape_in`, `floats`, `normalize`, `shape`, `encoding`, `negative_zero`, `objects`, `located`, `exact_work` (result_export, 14 functions) | the schema `$ref` graph has **no cycle** (`schema_depth.py`: 1,383 schema nodes); the longest same-value `$ref` chain is 6; total recursion from the root ≤ **36**; value depth reached ≤ 15 |
| Sparse assembly re-entry | FK sparse.rs:592 `assemble_sparse_stiffness` | recurses once to assemble the force-scaled primitives (W2), then returns: **≤ 2** |
| Self-weight basis | self_weight.rs:558 `retained_basis` | ≤ 2, and unreachable under the proposed D1.10 |

**Derive-generated and std recursion** (not visible to the lexical graph):

| Recursion | Bound |
|---|---|
| `Deserialize` of the typed request from the raw clone (`requested()`, the initial parse) | raw depth ≤ 17. `#[serde(flatten)]` on `MaterialRecordWire` (PP/lib.rs:192–198) and the internally tagged `LoadTargetInput` buffer a subtree into `Content`, then deserialize it again. Each buffering level is bounded by the same raw depth, and the buffer depth is at most the material record's or load target's own depth, ≤ 17 (RV83 §7) |
| serde_json `Value` Clone, Drop, PartialEq, Serialize and `to_value` | depth of the Value: ≤ 18 (raw plus wrapper) or ≤ 12 (envelope, body) |
| serde_json parser (`checked_parse`, `from_str` of schemas and self-weight provenance) | `remaining_depth` = 128 (serde_json 1.0.149 de.rs:63; `unbounded_depth` off). The inputs reached are ≤ 18 (generated text) and ≤ 15 (static schemas) |
| `BTreeMap::clone` → `clone_subtree` (RV83 §7) | recursion to the tree height: ≤ ⌈log₆(n)⌉ + 1 ≤ 7 for n ≤ 36,864 entries, the largest D1 map |
| Derived Debug and Display | by type nesting. Every reached type is finite and non-recursive except `Value`, which is covered above |
| `sort_unstable` (ipnsort) | depth ≤ 2·⌊log₂ n⌋ + O(1) ≤ about 32 for n ≤ 36,864 |
| `sort` / `sort_by` (driftsort) | iterative merge with an **on-stack scratch** of 4 KiB (std `smallsort`/`stable` `STACK_BUF` constant) plus heap scratch above it (I54 S1); no recursion beyond the quicksort fallback's logarithmic depth (RV83 §7) |
| Iterative by source | RCM BFS (I54 BOUND:65; FK factor.rs:298); union-find `root` loops (FKS/formation_check.rs:375–381); the census cursor array `[Option<Frame>; 64]` (PP/retained_memory.rs:100); hashbrown probing |

**The retained kernel and the product certificate** (FK/retained/*, the product_certificate modules) have **no self or mutual recursion** in the graph. The exact helpers are "finite and nonrecursive" (I51 C0_HELPER_LAYOUT.md:49).

## 2. Deepest chains

The longest acyclic call depth over the condensation, counting each recursive SCC once, is **34 frames** from `run_linear_static_preview_value_with_retained_direct` and **31** from `prepare_observed` (`callgraph.out.json` `root_depths`). Add the recursion bounds above where a chain passes through a recursive function.

| Chain | Frames (over-approximate) | Large fixed frames on it |
|---|---|---|
| Ordinary T25: finalize → `hash(publication)` → checked → `canonical_json_checked_v1_text` → `write_canonical` × ≤ 18 | ~34 + 18 | escape temporaries; canonical number scratch (I54 J8 inline buffers) |
| Ordinary request rebuild: `requested()` → `from_value` → Deserialize with flatten buffering | ~25 + 2·17 | none large |
| W1 native: retained Direct → ProductAttempt → solve_cases → adaptive schedule → verify_state → formation check → `frame_matrix` and `invert6` | ~34 | `Element = [[Wide2;12];12]` = 144·s(Wide2); M6 = 36·s(Wide2); the 27-Endpoint frame (3,888 B); ExactWideSum (2,144 B); Magnitude[128] (1,024 B); ExactAccumulator (1,088 B) (C0_HELPER_LAYOUT.md:13–16) |
| G4 reader (T17): `retained_precision::validate` → `shape` → `encoding` → … | ~9 + 36 | schema OnceLock first-use parse (below) |

**Debug-build frames** (RV83 §7). Large `json!`- and `format!`-heavy functions get big frames in debug builds: `run_linear_static_preview_observed`, `solve_load_case_observed`, source.rs `commitment`, retained_product's capture functions. Frame size depends on opt-level, so each qualified (profile, opt_level) identity gets its own witness run (BUILD.md §2.1). The witness inputs below drive exactly these functions.

**The panic and backtrace path** (RV83 §7). A panic on the scoped thread runs the panic hook on that thread: message formatting, and backtrace capture and symbolication when `RUST_BACKTRACE` is set. That stack is used on top of the frame that panicked. R = 64 MiB leaves headroom far beyond the observed 2 MiB. The witness does not drive a panic. **This is stated as a residual of the S1 evidence, not measured.**

**First-use OnceLock parses on the scoped thread** (RV83 §7, N-13). result_export holds 13 process-global `OnceLock` caches: physics_source.rs:39, source_blocks.rs:71, retained_precision.rs:326/335/344, load_reference.rs:334, semantic_contract.rs:5/109/118/142/155/164/173/570. Whichever thread uses one first parses its static schema or contract with serde_json (depth ≤ 15). They are statics, not thread-locals, so their contents do not depend on the thread. Their heap is process-lifetime and belongs to T17 (G4). Their parse stack falls in the reader chain above.

## 3. Witness coverage (refines STACK_PLAN.md §4)

| Witness | Inputs | Covers |
|---|---|---|
| W1 | milestone, both modes | W1 native, proof, serializer and reader chains (the selected W1 path) |
| W2 | cap-maximal synthetic D1 model: 32 nodes, 32 members, 32 supports, 192 loads, 128-byte ids, **raw depth 16**, every string with a quote or backslash | the deepest raw Value (Deserialize, flatten buffering, Value Clone and Drop, canonical rendering of the raw request), maximal counts, maximal escaping under D1.11 |
| W3 | an axis-aligned, in-domain model whose legacy exact recovery **selects** | **T25**: `requested()` ×3, the commitment and the publication and body hashes at envelope depth 12; T07 Context and functionals |
| W4 | preparation refusal | the refusal and fallback chain |
| W5 | dense mode of W1 and W2 | the dense parity tail |
| W6 | an in-domain W2 (force-scaled) case | the `assemble_sparse_stiffness` re-entry (≤ 2) |
| W7 | U3 fault-injection controls, if drivable | the receipt-failure fallback |

**Chains no witness drives, and how each is bounded:**
- the panic hook (above);
- envelopes larger than W3's. A deeper envelope is impossible: the depth is fixed by the grammar, so only the counts grow, and frames do not scale with counts;
- the reader's 36-level schema walk. G4 must confirm that W1 reaches the deepest schema chain, or add a reader witness.

**The witness binary** (RV83 §7) is a `cfg(test)` build of PP: a unit test that reaches the `cfg(test)` R override (STACK_PLAN §4). Its D-6 identity text matches the production build's when profile, opt-level, debug assertions, rustflags and target are the same: `cfg(test)` is not an identity key, because a build script cannot observe it. So the witness evidence is recorded against the production identity text. The test harness's extra code is not on the measured thread's chain, apart from the test function frame itself.

## 4. Thread-local inventory (completes STACK_PLAN.md §1)

A scan of the non-test source of the 15 linked crates for `thread_local!`, `static mut`, `OnceLock`, `LazyLock`, `OnceCell`, `lazy_static!` and atomic or cell statics found:

| Crate | Item | Guard | Effect across the thread hop |
|---|---|---|---|
| PP | `retained_memory.rs:348` DISPATCH_COUNT; `lib.rs:2990` DENSE_SCRUTINY_CEILING_OVERRIDE; `source_receipt/composite.rs:1168` WORK_TRACE | `#[cfg(test)]` | none in production |
| PP | `historical_pressure_reference.rs:10` ACTIVE | the module is `#[cfg(test)]` (lib.rs:105–106) | none in production; already asserted not to propagate (lib.rs:14446–14457) |
| FK | `retained/verify.rs:1243` NO_SHIFT; `retained/adaptive.rs:5159` SEED and `:5190` ROWS | inside `#[cfg(test)] mod hooks` and `mod seed` | none in production |
| FK | `retained/seeded.rs:87` SELECTED (OnceLock) | compiled only under `cfg(any(test, feature = "mutation-controls"))` (seeded.rs:4–8), and the feature is off in D1 | none |
| result_export | 13 `OnceLock` caches (above) | process-global statics, not thread-local | none: the value is identical on every thread |
| LS, PL, SR, canonical_json, units, sparse_direct, straight_pipe, nonlinear_integration, curved_bend, nonlinear_supports, diagnostics, load_case_algebra | none | — | — |

std's per-thread state still differs across the hop (RV83 §7): the hashbrown `RandomState` keys, and the thread name in the panic hook's output. Product outputs do not depend on HashMap iteration order: ordered outputs come from BTree maps or explicit sorts. Panic payloads are preserved through `join` and `resume_unwind`.

## 5. R and k

**R = 64 MiB (67,108,864 B) is confirmed.** It is reported separately and checked as E_mov,max + R ≤ M (COMPOSITION.md). Nothing in the inventory is count-proportional.

The deepest structural chains are about 34 frames plus 18 levels of Value recursion plus about 36 reader levels (G4). Even at a generous 16 KiB per debug frame, that is under 1.5 MiB. The witness at R/16 = 4 MiB checks this per build identity, and **k = 16 is confirmed**.

**The limits are as in STACK_PLAN §6.** The witness is measured evidence, not a proof. The panic hook's stack is a stated residual.
