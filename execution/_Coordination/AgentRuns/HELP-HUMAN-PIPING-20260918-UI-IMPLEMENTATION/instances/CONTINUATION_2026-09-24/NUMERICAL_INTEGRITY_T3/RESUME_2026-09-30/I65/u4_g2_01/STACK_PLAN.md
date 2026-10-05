# T20 stack under S1: recursion bound, reservation, witness and margin

**The owner chose S1** (RR "The owner decides D-3 and D-6"). Stack is qualified by three things together:
1. a structural recursion bound from source;
2. an explicitly reserved stack, whose size **R is the priced term**;
3. a witness test at R/k, recorded as **measured evidence, not a proof**.

**Proposal:** R = 64 MiB and k = 16, so the witness runs at 4 MiB. G3 completes the derivation; G5 writes the code, with I61 as integration owner for the `PP/lib.rs` site (D-5). No host tooling is involved.

## 1. Placement

1. G-A runs on the caller's thread exactly as today: parse, census, the D1 predicate and the gate check.
2. **If the permit is refused, nothing changes.** The ordinary route runs once on the caller's thread.
3. **If the permit is granted,** the retained Direct entry runs everything that follows on one scoped thread, built with `std::thread::Builder::new().stack_size(R)` and `spawn_scoped`. That covers the observed ordinary run, G-B, G-C, preparation, native, proof, serializer, precommit validation and the transfer.
4. **The scoped thread borrows the request and the capture.** Nothing is cloned to cross the thread boundary.

**Failure handling:**
- **Spawn failure** is the refusal `StackReservation` (DOMAIN.md §3). It happens before any W1 owner exists, and the ordinary route then runs on the caller's thread.
- **A panic** on the scoped thread is collected by `join` and re-raised on the caller's thread with its original payload (`std::panic::resume_unwind`). The caller sees the same panic as before. A panic loses the ordinary result with or without the hop, so this is not a new failure mode.
- **The spawn's own heap allocations** (the thread handle, its optional name, and the boxed closure and result packet) are small and are priced in the Direct caller term (G4). The stack itself is an mmap reservation, not a heap request.

**Thread-local state must not change behaviour across the hop.** A scan of PP's non-test source for this grant found two thread-locals:
- `historical_pressure_reference::ACTIVE` (PP/historical_pressure_reference.rs:10). It is private and test-only: no request field, environment variable or public symbol selects it (:1–2). It is outside D1 because pressure is excluded, and it is already asserted not to propagate to spawned threads (PP/lib.rs:14446–14457).
- The `cfg(test)` dense-ceiling override (PP/lib.rs:2989–2994).

A scan of every P/core crate's non-test source (performance_harness excluded) found no other `thread::spawn`, `thread::scope` or `rayon` use. G3 completes the thread-local inventory for the FK, LS, PL, SR, canonical_json and result_export paths.

## 2. The recursion-bound argument from source

The claim: on the D1 W1 path, every recursion has a depth bounded by a D1 cap or a fixed grammar. Peak stack is therefore a finite constant of the build for a given opt-level. G2 supplies the structure; G3 completes it as a call-graph read.

**(a) Explicit recursion.** A lead scan found directly self-recursive candidates (`_run_records/recursion_scan.py`, output in `recursion_scan.out.json`, 107 candidates). Most are delegation to a field's same-named method, which is not recursion. The genuine recursions are JSON-tree walkers:

| Site | Recursion | Depth bound in D1 |
|---|---|---|
| canonical_json `write_canonical` and `validate_checked_value` (src/lib.rs:94, 125); `binary64.rs:179, 263, 320` | one frame per nesting level of the Value | raw request: census depth ≤ 16. Envelope and receipt: fixed by the C1/C2/C3 grammar and the envelope schema; G3 computes the maximum |
| result_export reader `objects`, `located`, `encoding` and `shape` (retained_precision.rs:360, 400, 1694, 1722); `finite_tree` (load_reference.rs:306, physics_evidence.rs:77, preview_physics_evidence.rs:126); `shape_in` (source_blocks.rs:87); `guard_json` (derivative.rs:13) | Value depth. `shape` also follows schema `$ref` | value depth times the longest `$ref` chain that does not descend into the value. G3 must show there is no such cycle |

**(b) Derive-generated recursion** (not visible to the scan):
- **Deserialization** of the typed request from the raw clone: bounded by the raw depth.
- **serde_json `Value` Clone, Drop, PartialEq and Serialize:** bounded by the depth of the value concerned.
- **serde_json's parser** is limited to 128 levels (cached serde_json 1.0.149 `src/de.rs:63`; `unbounded_depth` is off, BUILD.md §1).
- **Debug and Display of error enums:** bounded by the type nesting. These types are finite and non-recursive apart from `Value`.

**(c) Numerical and std code:**
- the exact preparation helpers are "finite and nonrecursive" (R/I51/prepared_producer_implementation_03/C0_HELPER_LAYOUT.md:49);
- RCM is an iterative BFS (I54 BOUND:65);
- the formation union-find `root` is a loop (FKS/formation_check.rs:375–381, a `while` loop);
- BTreeMap and hashbrown operations are iterative;
- the unstable sort's recursion depth is logarithmic in the slice length, and the slice length is capped in D1.

G3 must also inventory the retained kernel (FK/retained), the product certificate and U1/U3's code. U1 and U3 come in by design-to-budget.

**(d) Known large fixed frames,** which are why the witness is informative:
- `Element = [[Wide2;12];12]`: 144·s(Wide2), which is 4,608 bytes at s = 32 (FKS/formation_check.rs:460);
- `M6`: 36·s(Wide2) (:462);
- the 27-Endpoint preparation frame: 3,888 bytes;
- ExactWideSum: 2,144 bytes;
- Magnitude[128]: 1,024 bytes (all three from C0_HELPER_LAYOUT.md:13–16);
- ExactAccumulator: two inline `[u64;68]`, 1,088 bytes (FK exact_sum.rs:45–48);
- the census iterator array `[Option<Frame>;64]` (PP/retained_memory.rs:100).

## 3. Reservation size and margin

**Proposed reservation: R = 64 MiB (67,108,864 bytes).**
- **The existing observation it rests on.** Experiment 03 ran the actual facade for the milestone in both modes: ordinary run, capture, producer, certificate and emitter. It ran as a `cargo test --lib` test (R/I61/receipt_experiment_03/_run_records/run_iter.sh:9–11), a debug build, with no `RUST_MIN_STACK` set, so each test thread had the harness's default 2 MiB stack. That is an observation for one small case, not a bound, and it did not include the precommit reader.
- **Headroom.** R is 32 times that observed reservation.
- **Cost.** R is reserved address space. The OS commits only touched pages, but the priced term is the whole of R.
- **Against M.** R is about 1.7% of the 3.75 GiB target.

**Proposed margin: k = 16, so the witness runs at R/16 = 4 MiB.** The margin has to cover two things:
- **chains the tests cannot drive.** G3 must show that each deepest chain is either exercised by a witness input or bounded by an exercised chain plus a source-bounded delta;
- **the difference between opt-levels.** Each qualified (profile, opt_level) build identity (BUILD.md §2.1) gets its own witness run.

**How R enters M.** The owner made R a priced term; ROOT's D-7 keeps M a heap threshold with no stack claim. **Decision S-1 for ROOT:** report R separately and also check `E_mov,max + R ≤ M`. This is conservative and keeps a single comparison.

## 4. Witness design (G5)

A unit test inside PP, so that a `cfg(test)` override of R is reachable. The precedent is the dense-ceiling override at PP/lib.rs:2989–2994. The override is read on the caller's thread before the spawn. The test calls the real retained Direct entry with R_test = R/16. Each input must complete, and the result is recorded as measured evidence for that build identity.

| # | Input (both modes unless noted) | Path it exercises |
|---|---|---|
| W1 | the milestone fixture | the selected path: native, proof, serializer, precommit reader |
| W2 | a cap-maximal synthetic D1 model: a 32-node, 32-member chain; 32 supports, some springs; 192 nodal loads; 128-byte identifiers; raw depth 16 | the largest counts and the deepest raw JSON |
| W3 | an axis-aligned in-domain model whose legacy exact source recovery reaches `Context` and selects | exact-block selection and W1 bypass; the exact_boundary and functional path |
| W4 | the existing preparation-refusal case | the refusal and fallback path |
| W5 | dense mode of W1 and W2 | the dense parity tail |
| W6 | a W2-triggering in-domain case, if G3 finds or builds one | the ordinary W2 branch |
| W7 | U3's fault-injection controls, if maintained tests can drive them | the receipt-failure fallback |

**What the test does not do:**
- It does not search for the smallest passing stack. An overflow aborts the test process, so the test runs once, at R/16.
- It records the pass, the build identity text and the inputs.
- A failure is a G6 qualification failure, never a runtime state.

## 5. Which grant does what

| Grant | Work |
|---|---|
| **G3 (derivation)** | the complete W1-path call-graph recursion inventory, including mutual and trait recursion and the receipt/envelope grammar depth; the deepest chains and their witness coverage; confirmation or revision of R and k; the thread-local inventory |
| **G5 (code)** | the R constant and its `cfg(test)` override in `retained_memory.rs`; the `StackReservation` refusal; witness tests W1–W7; the thread placement at the retained Direct dispatch, written by I61 as `PP/lib.rs` integration owner to this specification |
| **G6 (record)** | the measured witness evidence per build identity, and an explicit statement of the S1 standard |

## 6. Limits

- The witness is measured evidence for the inputs and builds it runs, not a proof of the maximum stack.
- R bounds only the scoped thread. The caller's thread runs the unchanged pre-admission code as today.
