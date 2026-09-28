# I8: implement slice K1 (W3 kernel sparse representation and sparse M03 gate)

This is an implementation TASK. Read `_COMMON.md` first; this brief overrides it where they differ.

## Purpose

K1 is the next kernel slice after K2a. The order is S11-K → K3a → K-D5 → K2a → **K1** → K2b → K5 (`DESIGN.md` revision 5a.2 §6, D-10).

Today's "sparse" mode is sparse only in its factor:
- the product assembles a dense n×n K;
- `AssemblyEvidence` keeps dense n×n `absolute_roundoff` and `operation_counts`;
- `sparse_direct::structural::factor_structural_ldlt` derives its adjacency and profile from the dense matrix (`adjacency_from_dense`, `from_dense_with_order`).

So memory grows as n² in both modes (M32).

K1 gives the kernel **one sparse representation** and **the same M03 gate rewritten over it** (§4.8 W3). It covers:
- a `SparsePattern` from element connectivity (12×12 blocks), springs, and user and curved blocks;
- values accumulated in today's dense element order, so every coalesced entry is bit-identical to the dense entry;
- the gate's validation, preparation, contribution audit, residual, pivot screens, rcond and negative witness over pattern entries, in O(nnz) memory.

K1 is kernel only. **The product keeps calling today's entries** until F1b switches it (see "F1b interface" below), so K1 changes no published byte.

## Basis (read in this order)

1. `T3/DESIGN_NUMERICS/DESIGN.md` revision 5a.2 (sha256 `fb62ef4a…`; hash-pinned, do not edit):
   - §4.8 "W3 — sparse assembly, reduction and reactions (M32)", all of it: one representation; the same gate in both modes; modes; the facade after T1; the parity protocol; the memory and runtime protocol;
   - the K1 row of the §6 slice table, and the K2b, K5, K6 and F1 rows (for what follows K1 and consumes it);
   - §7.1, for the frozen references N01–N09, R01–R07, NP-B, NP-D and the T0R references;
   - §7.3 mutation 8, and mutation 10 (label-dependent order).
2. `T3/ROOT_RULINGS_V1.md`:
   - the S11-K rulings (option (c): the nonlinear loop stays on the named `_binary64` kernel path, pinned by a test);
   - the K-D5 sections (the formation check, D5C-1 to D5C-5, R5-4, the M31b withdrawal);
   - the S11-G sections;
   - the F1 split (F1a merged; F1b after K1 and K2b);
   - "K2a product reach", corrections 1–3.
   - The standing lessons also apply: **enumerate every caller** (by lexer scan); a claim about product behaviour needs a product run; bounds and general claims must be derived and independently checked; performance claims come only from interleaved runs of archive-built probes.
3. **The code on your base:**
   - `FK/structural.rs`: `StructuralSystem`, `prepare_structural`, `finish_checked_factor` with K-D5's EF step, `negative_pair_witness`, `verify_negative_direction`, `solve_structural_dense*`;
   - `FK/structural/formation_check.rs` and `exact_boundary.rs`;
   - `sparse_direct/src/{lib.rs, structural.rs}`: `SymmetricProfileMatrix::from_entries_with_order` and `adjacency_from_symmetric_entries` already exist;
   - `nonlinear_integration/src/structural_adapter.rs` (`SA`): `AssemblyEvidence::new`, `with_force_terms`, `solve`, `solve_assembled`, `solve_assembled_with_formation_check`, `solve_binary64`, `solve_structural_sparse_binary64`, `curved_formation`.
   - The existing KREV-01 to KREV-05 regression tests are in `FK/structural.rs`, `FK/rigid_body.rs` and `sparse_direct/src/structural.rs`. Their origin is `SOLVER_MANAGER/NUMERICAL_INTEGRITY/KERNEL_REVIEW/RETURN.md`.
4. K2a's merged `IMPLEMENTATION/K2A/RETURN.md` once it exists, in particular its derivation section and its callers list.

## Base, worktree, branch, and when you are spawned

- **Write sets.** K2a writes `FK/lib.rs` (`local_stiffness`; `FrameKernelError::NumericalRange`), `diagnostics/src/lib.rs`, and new test files. K1's write set, below, is `FK/structural.rs`, new `FK/structural/sparse.rs`, `sparse_direct/src/{lib.rs, structural.rs}` and `SA`.
- **The two write sets do not overlap,** so by ROOT's rule **K1 is spawned when K2a's PR is open,** not after K2a merges.
  - ROOT creates `<k1-worktree>` on a new branch from **main at or after `134eefc24`** (F1a merged). It contains S11-K, K3a, S11-F, S11-G, K-D5 and F1a.
  - **Do not base K1 on K2a's branch.**
- **Two conditions follow from spawning early:**
  1. **K1 must not edit `FK/lib.rs`.** If you find you need to (for example a block-assembly helper next to `assemble_global_stiffness`), put it in `FK/structural/sparse.rs`. If that is impossible, **stop and tell the manager.** The write sets would then overlap, and ROOT's rule moves K1 to after K2a's merge.
  2. **The K2a-interaction tests need K2a's code.** When K2a merges, the manager or ROOT merges main into K1's branch. You then add those tests (see "Tests"). **K1's PR cannot merge before K2a's,** and K1's review covers the tree with K2a in it.
- K1 lands as **its own PR to main with full gates:** a complete-diff independent review, hosted CI with the surface-4 dispatch, a clean DEC-025 sweep, and the committed-fixture diff with its stop rule.
- Make no Git writes and no index operations. The manager commits.

## Write set (from the K1 row; re-locate every line on your base)

- **New `FK/structural/sparse.rs`,** declared by one `mod sparse;` in `FK/structural.rs`. It holds:
  - **`SparsePattern`:** built from element connectivity (12×12 blocks per frame, user and curved element), springs (diagonal), and prescribed and free partitions. It is independent of labels: the order must not depend on node or element labels (mutation 10). It has deterministic storage counts: nnz and profile entries.
  - **Coalesced values** accumulated in **exactly the element order the dense assembly uses today** (`assemble_global_stiffness_with_user_elements` and `SA`'s curved and spring additions). Each coalesced entry must be **bit-identical** to the dense entry. Bitwise equality is the parity basis, with no new tolerance.
  - **The sparse gate,** the same M03 checks over the pattern:
    - `validate`: symmetry over pattern pairs;
    - `prepare`: scaled values, and `K_fc u_c` from sparse columns;
    - the contribution audit, with Expansions per pattern entry, so memory is O(nnz);
    - the residual and intended action over sparse rows;
    - `rcond`: the norm from the pattern;
    - `negative_pair_witness` and `verify_negative_direction` over pattern pairs only, in O(nnz).
- **`FK/structural.rs`:**
  - the `mod sparse;` declaration;
  - a sparse system type beside `StructuralSystem`;
  - the entry points that run the shared gate stages (`finish_checked_factor`, K-D5's EF step, the S11-K exact right-hand side and audit) on either representation.
  - **The dense `StructuralSystem` API and every existing entry stay unchanged,** for auxiliary callers and dense scrutiny.
  - Refactor shared stages behind the representation only where needed. Every dense call must stay byte-identical.
- **`sparse_direct/src/{lib.rs, structural.rs}`:** a factor built from pattern entries with `SymmetricProfileMatrix::from_entries_with_order`, ordered by `adjacency_from_symmetric_entries`, beside today's `factor_structural_ldlt`/`solve_structural_sparse`, which stay unchanged.
  - For the same K, the pattern path must produce the same order and profile as today's dense-derived path. Adjacency from coalesced pattern entries must equal `adjacency_from_dense` on the materialized matrix; prove it by test.
  - Watch explicit zeros. A structurally present entry whose value is exactly 0 is in the pattern but absent from `adjacency_from_dense`. Decide the rule, keep the order identical to today's on every fixture, and state the rule in RETURN.
- **`SA` (`nonlinear_integration/src/structural_adapter.rs`), for the sparse `AssemblyEvidence`:**
  - pattern-indexed `absolute_roundoff` and `operation_counts`, instead of n×n;
  - S11-K's force terms (`with_force_terms`);
  - K-D5's formation primitives, unchanged in content;
  - the geometry, qualified-family and rigid-body evidence, unchanged in content;
  - storage counts;
  - **new** pattern-taking solve entries beside the existing ones: plain, and with the formation check. `solve`, `solve_assembled`, `solve_assembled_with_formation_check`, `solve_binary64` and `solve_structural_sparse_binary64` stay unchanged.
  - `SA` lives in T5's `nonlinear_integration` crate. **Edit only `structural_adapter.rs` there.** The nonlinear loop (`nonlinear_integration/src/lib.rs`) is not in K1's scope: it stays on S11-K's option (c) `_binary64` path, pinned by the existing test, and moves to sparse in F1b with T5.
- **Allowed only if strictly needed, and declared in CHANGE_RECORD with the reason:** representation plumbing in `FK/structural/formation_check.rs` or `exact_boundary.rs`, so that K-D5's re-formation and S11-K's exact boundary accept the sparse system. They must have no behaviour change for dense.
- **Not in scope. Stop and ask before touching any of these:**
  - `FK/lib.rs` (K2a);
  - `PP`, `source_recovery.rs` and the facade (F1b);
  - `nonlinear_integration/src/lib.rs`;
  - `curved_bend`;
  - `diagnostics`;
  - formation-time scaling (K2b);
  - the W4 witness and curved screen (K5);
  - the performance harness (K6);
  - the dense-scrutiny and sparse resource-guard ceilings (ROOT picks them from measurement; F1b wires the refusal).
- **Enumerate every caller** of the entries you add or refactor, and of `solve_structural_dense*`, `solve_structural_sparse*`, `factor_structural_ldlt`, `negative_pair_witness` and `verify_negative_direction`, by lexer scan, into `_run_records/callers.txt`. Show that no existing caller changes behaviour.

## Interactions with merged and parallel slices

- **K2a (checked formation feeds sparse assembly).**
  - Sparse assembly must form element stiffness through **the same formation calls** as dense assembly (`FrameElement::local_stiffness`/`global_stiffness`, K2a-checked once merged; the user and curved element paths as today). Never through a separate formula.
  - A formation refusal (K2a's `FrameKernelError::NumericalRange { name }`) must propagate **identically in both representations:** the same error, the same element, the first failing element in the same order, before any pattern value is accumulated.
  - After K2a merges and main is merged into K1's branch, pin this. RF-RANGE LEF-small and K2a's reach_zero and reach_lef kernel cases must be refused with the same named error in both representations. A normal formation must give bit-identical coalesced values.
- **K-D5.**
  - The formation check (EF after the residual gate) must run in the sparse representation with the same inputs. ρ is still one `ExactAccumulator` sum per free row, from the pattern's contributions.
  - `FormationCheck` records and the Passed→Sensitive outcome must be **identical** between representations. Tests:
    - P1's `RF-SKEW-T-CANT-OFF-122-r1e-04` must demote in both;
    - R5-4's E1 and E6 must not demote in either;
    - the skew-plane elbow must demote in both;
    - a `formation_check_unavailable` seed must behave the same in both.
  - The existing pin (no formation check reached from the nonlinear loop's `_binary64` targets) must keep passing.
- **S11-K.** Its audit with exact per-DOF force terms, and the KS1–KS3 exact right-hand side and residual numerators, run in both representations with identical outcomes (the K1 row: "S11-K's audit in both representations").
- **S11-G and F1a** are facade only, and K1 changes no `PP` code.
  - For F1b, the `StructuralReport` produced through the pattern path must be **byte-identical in `Debug`** to the one produced today through the dense-derived sparse path, for every kernel fixture you run. S11-G's guard and F1a's evidence line read that report and the `FormationCheck` record, so any difference would move product bytes when F1b switches. Test it.

## F1b interface (what the next slices consume)

F1b rewires `PP` onto K1:
- assembly at the design's `PP:1620`/`:1751`, per modulus basis;
- partition maps at `:2330-2342`;
- reactions from sparse rows at `:2697`;
- `solve_preview_reduced_system` taking the pattern;
- the nonlinear loop with T5.

K2b adds a formation-time scale b to the same assembly entry. Design K1's public API for these consumers, and document it in RETURN's "F1b interface" section with the exact signatures:
- **`SparsePattern`** and its builder from frames, users, curved elements and springs, per modulus basis;
- **the sparse `AssemblyEvidence`** (`with_force_terms`, formation primitives, storage counts);
- **the pattern-taking solve entries,** plain and with the formation check, returning today's `StructuralSolution` and report types;
- **reduction with prescribed displacements, and partition maps,** over the pattern;
- **reactions from sparse rows;**
- **a dense view materialized from the same values,** for dense scrutiny (today's dense Cholesky and its labels) and for `source_recovery`'s n ≤ 256 case if needed;
- **deterministic storage counts** (nnz, profile entries, contributions), for the resource guard F1b wires.

The assembly entry should take its options in a form that lets K2b add b without changing every caller, for example an options struct. Do not implement b.

## Tests (all required)

- **Existing suites** of frame_kernel, sparse_direct and nonlinear_integration (and every crate the caller scan reaches) are unchanged and passing.
- **Committed fixtures:** the committed-fixture diff (T9) is **byte-identical**, as expected, since the product calls no new entry. **Any committed-byte change stops the work,** and you report it to the manager with its site before regenerating anything.
- **Bitwise K parity:** the coalesced pattern values equal the dense assembly entry for entry, bit for bit, on every kernel fixture and on generated product-shaped models covering frames, user elements, curved elements (realized bends), springs, prescribed motion, and two modulus bases, each on its own pattern.
- **Order and profile parity:** the pattern path's adjacency, RCM order and profile equal today's dense-derived ones, including any explicit-zero cases.
- **KREV-01 to KREV-05 on sparse:** each existing regression, run through the pattern path.
- **The O(nnz) negative witness:** `negative_pair_witness` and `verify_negative_direction` visit only pattern pairs. Show it with a deterministic count (pairs visited = pattern pairs), not with timing, on a model whose dense pair count is much larger.
- **Dense and sparse outcome-class parity** on N01–N09, R01–R07, NP-B, NP-D and the T0R references, per §4.8 protocol item 2. A divergence near a screen boundary is recorded, never tuned.
- **Relabelling and permutation** (§4.8 item 6; mutation 10): the same answers against the references.
- **S11-K's audit in both representations,** with identical outcomes, including KS1–KS3 prescribed cases.
- **The K-D5 parity cases** listed above, plus the nonlinear-loop pin.
- **Report byte-identity** between the pattern path and today's dense-derived sparse path, as above.
- **The K2a parity cases** listed above, added after K2a merges.
- **Mutation 8:** omit one pattern entry, such as a spring, in sparse mode only. It must fail a behavioural assertion. Add your own mutants:
  - accumulate in a different element order;
  - drop `K_fc u_c` from the sparse `prepare`;
  - let the order depend on labels;
  - scan dense pairs in the witness;
  - skip a coalescing step.

  Use a clean target per mutant, with a no-mutation control first.
- **The gate:** K1 changes no product path, so the both-entry gate is expected to be unchanged. Whether to run it is ROOT's call, as for F1a; record the call in RETURN.
- **Storage counts** (nnz and profile entries) are recorded for the RF-LARGE chain and tree at 10, 100, 1,000 and 10,000 members, at kernel level, as deterministic counts. **Make no timing or memory-growth claim.** Performance observations belong to K6, under the interleaved, archive-built method.

## Disclosure and return

- **`T3/IMPLEMENTATION/K1/CHANGE_RECORD.md`,** following `.agents/skills/chirality-change/SKILL.md`. It states:
  - that no product path changes, and no value or byte changes;
  - which new entries exist, and who will call them (F1b, K2b);
  - the explicit-zero rule;
  - any use of the conditional files (`formation_check.rs`, `exact_boundary.rs`);
  - the fixture result.
- **`RETURN.md`,** with logs under `_run_records/`, SHA256SUMS and no machine paths. It covers:
  - the files and line counts;
  - each write-set item and test;
  - the callers;
  - the per-crate counts;
  - the parity results;
  - the mutation table;
  - the storage counts;
  - the "F1b interface" section with signatures;
  - the K2a-interaction results (after K2a merges);
  - what was not done.
- Send the manager a SendMessage summary. Message at once if:
  - the stop rule triggers;
  - bitwise parity fails anywhere;
  - `FK/lib.rs` or `PP` would need an edit;
  - a design item cannot be implemented as specified.

## Running things

- `RUSTUP_TOOLCHAIN=1.97.1`, `RUSTUP_AUTO_INSTALL=0`, `CARGO_INCREMENTAL=0`, `--offline --locked`, and your own `CARGO_TARGET_DIR` under `<wt>`.
- **The cargo token is the manager's;** ask before any build. K2a's implementation may hold it. One heavy job at a time. Hold cargo while a DEC-025 sweep runs.
- Wait and kill patterns must be python-anchored (see `_COMMON.md`) and must not match your own shell.
- Keep free disk above about 8 GB.
- The authority targets are prerequisites, never scratch.
- **Never rewrite committed, hash-bound evidence;** add new files instead.
- Skip no tests and raise no timeouts.

## Addendum 1 (2026-09-28): spawn and gate rulings

This follows ROOT_RULINGS_V1, "K1: spawn timing and no both-entry gate (ROOT)".
- **Spawned now,** in `<wt>/k1` on `codex/piping-k1-20260928` from main `134eefc24`, with target `<wt>/k1-target`. Conditions 1 and 2 above stand.
- **K2a-interaction tests:** draft them read-only against K2a's branch, and land them after K2a merges and main is merged into your branch.
- **No both-entry gate.** The parity tests and T9 are the evidence. Record this ruling in RETURN, in place of "ROOT's call".
- **Cargo:** in your slot, build only the named crates you need. Check free disk before each build and prune afterwards.

## Addendum 2 (2026-09-28): the S11 site table

This follows ROOT_RULINGS_V1, "K1: the S11 site table for sparse.rs and formation_check.rs (ROOT)".
- **sparse.rs:** add it to `frame_kernel/tests/s11_site_table.rs` SOURCES, additively, under the five conditions there.
- **formation_check.rs:** add its rows as a separate change on K1's PR, so it lands as a separate commit. Stiffness and re-formation sites are exemptions, with reasons, and a binary64-fold mutant there must be killed.
  - **A plain binary64 load, force or RHS accumulation in formation_check.rs stops the work.** Report it as a K-D5 finding before touching anything.
- At your clean point, tell the manager which hunks belong to which change, or keep the formation_check.rs rows as a patch file in your records.
- Declare both in CHANGE_RECORD and RETURN as separate items.

## Addendum 3 (2026-09-28): the K-D5 and option-(c) source pins

This follows ROOT_RULINGS_V1, "K1: extending the K-D5 and option-(c) source pins for the sparse siblings (ROOT)".
- **Edit `nonlinear_integration/src/s11k_tests.rs` additively,** as that ruling states:
  - blanking and definition counts scoped by impl block;
  - tokens matched on identifier boundaries;
  - the binary64 legacy bodies checked as today;
  - zero crate calls of the siblings, and one product call until F1b.
- **Required:** the behavioural loop pin, and the three new mutants.
- **Prove there is no weakening:** re-run K-D5's E4 (and E1–E3 where relevant) and the option-(c) mutants from S11-K's and K-D5's records against the extended pins. List each with its kill site in RETURN. **A mutant that is no longer killed stops the work;** report it to the manager.
- Declare it in CHANGE_RECORD and RETURN as a pin extension.

## Addendum 4 (2026-09-28): the KERNEL list, and the handoff

This follows ROOT_RULINGS_V1, "K1: the S11-F site test's KERNEL list, and the handoff (ROOT)".
- **Add sparse.rs to `product_physics/tests/s11f_site_test.rs`'s KERNEL list** as its own hunk, under the site-table conditions. The WIP carries it as a patch, `IMPLEMENTATION/K1/kernel_list_s11f.patch`.
- **The C3-detect helper:** see WIP_STATE.md for whether it was pre-existing. If it was, it is restored and routed to ROOT.
- **K1 continues on the owner's Mac,** from the WIP commit and `IMPLEMENTATION/K1/WIP_STATE.md`, with a fresh implementation TASK working under this brief and addenda 1–4. `HANDOFF_2026-09-28_TO_LOCAL.md` §5 lists the first steps, including platform calibration before any comparison.
