# I65 U4 G7: return (Pass A)

**Status.** Pass A is complete. **The registered entry stays byte-identical, so no `registration.diff` is needed.** No stop condition was met:
- the integrated build is Registered;
- the margin holds;
- no source change is needed.

The record is `QUALIFICATION_G7.md`, an addendum to G6's QUALIFICATION.md. Pass B is `_run_records/g7_pass.sh`.

**Basis:** `ba1faa1c858ce3630a22767677310b1902a14b83`, the merge of NUM `f172f86abe` and memory `0c7827b6ad`. My copy equals the tree blob for blob: 2,949 of 2,949 files.

## In short

| Item | Result |
|---|---|
| 1. Delta inventory | 40 changed files.<br>– **3 production Rust files, 24 hunks.**<br>– `derivative.rs` (13 hunks): unreachable.<br>– `retained_precision.rs`: F5 is **new and priced**; one hunk is a comment; one is `#[cfg(test)]`.<br>– `semantic_contract.rs` (8 hunks): the `is_retained` branches are dead on D1, because the reader's projection sets the preview-physics-1 id; the base-branch checks are allocation-free; the standing, binding and classification fns are unreachable.<br>– **37 other files** (TypeScript, Python, Rust integration tests, fixtures, schemas): not on PP's D1 path. None of the 14 reviewed inputs changes |
| 2. Identity and statics | **Registered.** The identity, all 14 reviewed-input hashes and the 4 reader layouts equal the registered entry; `build_status() == Ok(0)`; 42 of 42 law tests pass.<br>**One new static** (`semantic_contract::preview_physics_retained_contract`'s `OnceLock` + `include_bytes!`). It is reachable only from the dead `is_retained` branches, so it is not reached from `validate` on D1. (Bound if it were ever initialized: 286,836 B) |
| 3. The new maxima | **Unchanged.** Sparse 3,575,778,286 B = 0.8881 M, and dense 3,595,488,734 B = 0.8929 M; that is 48,100,370 / 28,389,922 B under 0.9 M.<br>F5's two `Vec<&Value>` (counted at pushcap(D_env) + D_env slots) add +205,960 B to T17's V4 stage. V4 stays 1,131,825,961 B below T17's largest stage, V2_hash, so T17, every phase and the pinned record are unchanged |
| 4. TEXT and the audit | **Complete.** TAV 2,150,800,830, TAV_W 1,570,041,862 and TAV_X 1,440,401,002 all equal G6.<br>2,807 of 2,807 G6 rows are equal in multiplicity, bytes and requested bytes in all four runs; the 7 new rows all have multiplicity 0. The profile tree is G6's, plus F5 in `T17_V4`.<br>The audit is enforced: no `id-unaudited`, `stale-key` or `stale-audit-entry`. 11 of 11 controls pass.<br>**§11 is discharged by re-running the sweep.** The 410 non-candidates are exactly the 410 RV87 read |
| 5. Witnesses and suites | All 9 witnesses pass, and so does the challenge (peaks 3,541,898 / 2,252,863 B).<br>PP: 699 passed, 1 failed (t13), 10 ignored. runner/headless: 85 passed, 2 failed.<br>Both are **outcome-identical to the `0c7827b6ad` registered baseline.**<br>The differences from NUM `f172f86abe` unregistered (PP 660/1/1) are 48 memory-branch tests NUM lacks: 46 `retained_memory::`, the challenge, and U3 1d's `u3_unfired_hooks_…`. runner/headless is identical |
| 6. Pass B script | `g7_pass.sh <basis> <rev> <tag>`.<br>It stops if the build is not Registered (exit 3) or a rule's line was edited (exit 4). Otherwise it reports byte-identical outputs, or the exact deltas, against Pass A.<br>Self-tested on Pass A's basis: every output identical |
| 7. Output | Registered entry byte-identical; QUALIFICATION_G7.md |

## For ROOT

1. **A TEXT-tool finding, fixed in G7's records-only tool. Please route it to review.**
   - `text_budget.py` propagated multiplicity only between condensation components.
   - U6 creates two lexical cycles (`validate` ↔ `for_source`, and `validate_transport_metadata` ↔ `for_source_metadata`). The G6 tool silently zeroed everything below `for_source`, giving TAV −112,852,224 B, with no completeness signal beyond one unclassed argument.
   - The G7 tool forms the condensation without `edge_zero` edges, fails on any multi-member cycle (`scc`, control c9), and reproduces G6's outputs exactly.
   - G6's graph had no such cycle, so G6's numbers stand.
2. **Proposal, optional, a source change G7 does not make** (`pass_a/proposal/t17_v4_f5.diff`, one generated line). Regenerating the profile with F5 changes only `T17_V4`'s `s(&Value)` coefficient (57,880 → 83,625). No maximum, record, identity or test moves.
3. **§11:** discharged at this basis by the mechanical sweep. RV87's explicit-row rule (240 bare-local rows) stays the durable closure, for its own grant.

## Execution record

- **Who and when:** I65, TASK (Type 2) under ROOT, no descendants; 2026-10-04, about 09:12–09:45 MDT.
- **Memory guard:** PID 5387 was running, and every cargo job and run checked it.
- **Cargo:** the default toolchain (rustc 1.97.1), `--locked --offline`, `CARGO_BUILD_JOBS=4`, `RUST_TEST_THREADS=2` (1 for the witnesses and the challenge), one cargo job at a time; targets in WT/targets/i65_g7/.
- **Writes:**
  - this folder;
  - WT/scratch/i65_u4_g7_01/: `work/` and `num/` copies, `s/`, `p/`, `rr/`, logs, and the `pass_selftest/` copy;
  - WT/targets/i65_g7/.
  
  The basis extract was only read. WT/f2a-memory and I61's files were not touched.
- **Not run:** no Git writes or index operations (reads, `git archive` and `ls-tree` used `GIT_OPTIONAL_LOCKS=0`), no installs, no new tooling, no native, solver-at-scale or DEC-025 jobs, and nothing in the system temp directory.
- **Records:** placeholder paths only (WT). `SHA256SUMS` covers this folder.
