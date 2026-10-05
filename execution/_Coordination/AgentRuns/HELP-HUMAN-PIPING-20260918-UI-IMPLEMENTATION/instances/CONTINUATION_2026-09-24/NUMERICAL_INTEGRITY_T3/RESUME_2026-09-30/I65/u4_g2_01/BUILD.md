# Build binding for the profile (T01, T10) and the D-6 build-identity check

**The profile is bound to its build in three layers:**
1. **Every stride the formulas use is taken in-build** with `size_of`/`align_of`.
2. **Private std and dependency layouts use source-derived upper formulas.** Allocation-Layout witness tests confirm them in the actual build.
3. **The owner-selected D-6 build check fails closed.** A PP build script records the compiler and target identity, compile-time layout witnesses guard the dependency features the formulas assume, and any mismatch makes the profile `Stale`, which sends the invocation to the ordinary path.

The consumer lock is a reviewed record. Basis: NUM `a2c26cc885`; the owner's D-6 = (a) ruling (RR "The owner decides D-3 and D-6").

## 1. Build identity facts the profile depends on

These hold for the D1 Direct consumer; ROOT verified the compiler identity before.

| Fact | Value | Source |
|---|---|---|
| Compiler | rustc 1.97.1, commit `8bab26f4f68e0e26f0bb7960be334d5b520ea452`, 2026-07-14, LLVM 22.1.6 | `rustc -vV` read in this grant (identity read only, no compilation); matches RR:5260 and I54 container COEFFICIENTS.md:7 |
| Host and target | aarch64-apple-darwin; 64-bit pointers and `usize`; little-endian | same |
| std source | the installed rustdoc source of that toolchain: btree/node.rs, raw_vec, string, hashbrown 0.17.1 inside std, NEON control group | I54 COEFFICIENTS.md:7–13 (RV73-accepted); this grant re-read btree/node.rs:43–111 |
| Direct consumer lock | `P/core/product_physics/Cargo.lock`, SHA-256 `f28eee2b6c039de6bbbf4bdf2e3e7138057cddbc73e3ce2676a632b68f5caac3`; 36 packages, 22 external | read in this grant |
| Relevant pins | serde_json 1.0.149 (features `std`, `float_roundtrip`; no `preserve_order`, `arbitrary_precision`, `raw_value` or `unbounded_depth`); serde/serde_core/serde_derive 1.0.228; ryu 1.0.23; itoa 1.0.18; zmij 1.0.21; sha2 0.10.9; memchr 2.8.0 | the lock and PP/Cargo.toml:26–27 |
| Crate features | PP has no `[features]`; FK `mutation-controls` is off; production `cfg(test)` is false | PP/Cargo.toml; FK/Cargo.toml:13–17 |
| Allocator | System malloc via std's default `Global`. M counts requested Layout bytes, not allocator overhead (I51 COMPOSITION §1) | I54 COEFFICIENTS.md:15 |

**The lock will change at U3.** Decision 5 adds a `result_export` path dependency to PP (RR:8862: U3 owns that runtime dependency). The registered lock hash is therefore re-pinned at G6 against the post-U3 lock; the hash above is the pre-U3 baseline.

**The headless runner's lock differs** (serde_json 1.0.151). That is one reason Headless is outside D1 (D-2).

## 2. The D-6 check: design

### 2.1 What the build script records

A new `P/core/product_physics/build.rs`. It is G5's implementation, under the D-5 fence.

The script runs once per build configuration and reads only cargo-provided environment variables plus the output of `$RUSTC -vV`. It emits one compile-time environment variable:

```
cargo:rustc-env=OPS_RETAINED_BUILD_IDENTITY=<canonical text>
cargo:rerun-if-changed=build.rs
cargo:rerun-if-env-changed=RUSTC
cargo:rerun-if-env-changed=RUSTFLAGS
cargo:rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS
```

The canonical text is newline-separated `key=value` lines in this fixed order:

| Key | Source |
|---|---|
| `rustc.release`, `rustc.commit`, `rustc.host`, `rustc.llvm` | `$RUSTC -vV`. Cargo sets `RUSTC` to the real compiler even under a wrapper |
| `target` | `TARGET` |
| `target.arch`, `target.pointer_width`, `target.endian`, `target.os`, `target.env` | `CARGO_CFG_TARGET_ARCH`, `_POINTER_WIDTH`, `_ENDIAN`, `_OS`, `_ENV` |
| `panic` | `CARGO_CFG_PANIC` |
| `profile`, `opt_level` | `PROFILE`, `OPT_LEVEL`. Stack depends on these (STACK_PLAN.md); layouts do not |
| `debug_assertions` | presence of `CARGO_CFG_DEBUG_ASSERTIONS` |
| `rustflags` | `CARGO_ENCODED_RUSTFLAGS`, escaped. A layout-changing flag such as `-Z randomize-layout` must therefore mismatch |
| `pkg` | `CARGO_PKG_NAME`, `CARGO_PKG_VERSION` |

**Fail-closed rules:**
- The script never panics and never fails the build. Ordinary product builds must stay unaffected.
- Any read or parse failure emits `OPS_RETAINED_BUILD_IDENTITY=unavailable`.
- It writes no files, needs no network and adds no build-dependency: string comparison needs no hashing.

### 2.2 How a mismatch maps to `ProfileStatus::Stale`

In `PP/retained_memory.rs` (U4's fence, D-5):
- **The registered identities.** `const REGISTERED_BUILDS: &[&str]` holds the exact canonical texts qualified at G6. There is one per qualified (profile, opt_level), because the stack witness is per opt-level.
- **The build status function** returns:
  - `Registered` only if `env!("OPS_RETAINED_BUILD_IDENTITY")` equals one entry byte for byte **and** `LAYOUT_WITNESSES` (§2.3) is true;
  - `ProfileStatus::Stale` if a profile is registered but either test fails;
  - `ProfileStatus::Missing` when no profile is registered, as today (retained_memory.rs:230–234).
- **Admission:** D1.1 refuses on `Missing` or `Stale` (DOMAIN.md §3), so the ordinary path runs unchanged.

Comparisons are byte-exact; there is no prefix or semver matching.

### 2.3 Compile-time layout witnesses

Each witness is a `const bool` evaluated by the compiler, and `LAYOUT_WITNESSES` is their conjunction. A false witness makes the profile `Stale`. **It is never a compile error:** a `const` assertion would break ordinary builds on other targets or feature sets, which D-6's fail-closed rule forbids.

| Witness | Guards | Why the formulas need it |
|---|---|---|
| `size_of::<usize>() == 8` | 64-bit target | every count and stride formula |
| `size_of::<serde_json::Number>() == 16` | `arbitrary_precision` off | with that feature a Number owns a String the raw census does not count (T02) |
| `size_of::<serde_json::Map<String, Value>>() == size_of::<BTreeMap<String, Value>>()` | `preserve_order` off | the raw-backing formula uses the BTree node law (I54 BOUND:36–44); an IndexMap backing differs |
| `size_of::<serde_json::Value>() == 32` and `align_of == 8` | Value layout used by the qualified evaluation | T02, T03 |
| `size_of::<String>() == 24`, `size_of::<Vec<u8>>() == 24` | header layout | consistency check for the hand-checked G6 values; the formulas still take size_of directly |

Facts the build script cannot see are pinned by the compiler identity instead. Std is prebuilt per toolchain, so the rustc commit fixes:
- the std internals: BTree B = 6 and CAPACITY = 11 (btree/node.rs:43–44), hashbrown's group width, the RawVec growth law.

**The consumer lock is a reviewed record, not a mechanical check.** A build script cannot observe which `Cargo.lock` governs a downstream workspace. This is the stated residual: a foreign workspace that builds PP with a different serde_json patch release is not mechanically detected. It is mitigated by three facts:
- D1 admits `Entry::Direct` only;
- the desktop app is source-tested not to call the retained entries (PP/tests/retained_precision_admission.rs:217–222);
- the headless runner uses `Entry::Headless`, which is outside D1.

ROOT may accept this or ask for more.

## 3. T01: the stride roster and where each stride is taken

The rule is to take `size_of`/`align_of` in the crate where the type is nameable and export plain numbers across crates. Nothing is hard-coded except as a §2.3 consistency witness.

| Crate (evaluated in) | Types whose stride enters a D1 bound | Terms |
|---|---|---|
| PP (`retained_memory.rs`) | `serde_json::{Value, Number}`, `String`, `Vec<_>` headers; input model: `PreviewNode`, `PreviewPipe`, `PipeSectionInput`, `Quantity`, `PreviewSection`, `PreviewSupport`, `SupportStiffnessInput`, `MaterialInput`, `MaterialTemperaturePointInput`, `PreviewLoadCase`, `PreviewPrimitiveLoad`, `LoadTargetInput`, and the excluded-family element types whose (empty) capacities are still read: `PreviewComponent`, `PreviewCombination`, `Authored<Vec<ExpansionLawInput>>`, `usize`; ordinary output and helpers per I54 REQUIRED_FACTS (ResultItem, ResultBasisRef, ResultMetadata, Diagnostic, LocatedQuantity, NumericalCaseQuality, LoadCaseSolve, StationResultants, DerivedSection, preview `MemberRecord`, `RecoveryRecord`, `RowTreatment`, `FinalizedSourceBlockCase`, `OrdinaryAttempt`, and the hash tuples listed there); legacy source: `MemberRecovery`, `SpringAction`, `SupportActions`, `SelectedSourceRecovery`, `RecoverySummary`, `Identity` | T02, T03, T05, T07 |
| FK (new resource module, D-5 fence) | `Expansion` (pub(crate)), `StiffnessContribution`, `ForceContribution`, `exact_boundary::{Ratio, BlockWitness, Context, Response, Snapshot (private), RetainedResponse, RetainedProjection, QualifiedProjection, WorkReport}`, `functionals::{FunctionalDescriptor, FunctionalKey, FunctionalQuantity, AffineTerm, QualifiedFunctionalProjection, RetainedFunctionalProjection, RetainedFunctionalSet}`; FrameNode, FrameElement, StraightPipeElement, ContributionRounding, ResidualRow, PivotEvidence, ExactAccumulator, ForceTerm, FormationRecord, LoadFidelityRow, RecordOutcome, PublishedValue, `Wide<2>`, `Wide<16>`; the P3 kernel owners (G3 completes that list) | T05, T06, T07, T13 |
| SR | `elastic_extrema::Node` (private), `QuadraticStressSpan` | T14 |
| LS, PL | `LinearSupport`, `SpringEntry`, `SupportFinding`, `PrimitiveLoad`, `NodalLoadContribution`, `LoadFinding` | T05 |
| std / serde_json private | BTree `LeafNode<K,V>` and `InternalNode<K,V>`; hashbrown `RawTable` buckets and control bytes; serde_json `ErrorImpl` | §4 |

G3 and G4 append their own types (P3 owners, the C3 trace, U1's serializer, U3's transfer) under the same rule. A type missing from the roster is a missing term, never zero.

## 4. T10: private layouts by source upper plus in-build Layout witnesses

RV73 left the BTree leaf layout unresolved and declined to guess it as a sum of fields (I54 COEFFICIENTS row T1). The closure here keeps that discipline. Production code uses a **sound upper formula**, and a G5 test **measures the exact Layout in the qualified build** and requires measured ≤ upper. The test runs on the qualified build identity, so the measurement and the formula cannot drift apart unseen: a new compiler makes the profile Stale (§2).

**BTree nodes** (std 1.97.1 btree/node.rs:51–67 `LeafNode`, repr(Rust); :102–111 `InternalNode`, `#[repr(C)]`):
- Let A = max(8, align(K), align(V)).
- `Leaf_up(K,V) = up(8,A) + up(2,A) + up(2,A) + up(11·s(K),A) + up(11·s(V),A)`. Each field is padded to the maximum alignment; any field order fits inside this.
- `Internal(K,V) = up(up(Leaf,8) + 12·8, max(align(Leaf),8))`. This is exact because of repr(C), given the actual Leaf. Use Leaf_up in production.
- Check for `(String, Value)`: Leaf_up = 8+8+8+264+352 = **640**, Internal_up = **736**. The DWARF observation is 632/728 (I54 container RETURN:14), consistent with measured ≤ upper.
- The (K,V) roster from I54 REQUIRED_FACTS:
  - `(String, Value)`, `(String, Quantity)`, `(&String, SetValZST)`, `(String, Vec<ResultItem>)`, `(String, ResultItem)`, `(String, ())`, `(usize, Vec<&str>)`;
  - G3 and G4 add the factor, tracker and validator maps.

**Hash tables:** the accepted hashbrown formulas, `Buckets(s,k)` and `HashReq(s,a,b) = up(s·b, max(a,8)) + b + 8` (I54 COEFFICIENTS HB1–HB4; RV73), with s and a taken in-build for each tuple.

**Vec, String, VecDeque and BinaryHeap:** the accepted V1–V5, D1 and H1 laws (I54 COEFFICIENTS).

**serde_json:** J1–J10 (to_string starts at 128 bytes; canonical rendering scratch; Bigint 1,440 requested / 2,080 moving), pinned by the lock; and `ErrorImpl` by Layout witness.

**The G5 witness tests.** They use the existing test-binary allocator pattern: PP/tests/f1b_sparse_pattern_memory.rs:34–72 records peak bytes, and the same hook can record `Layout` values. This is a test-only global allocator, as the I29 memory plan's P6 allows. It is not a production guard and not new tooling.

1. **BTree:** build each roster map with 1 entry (one leaf), then with 12 (one internal node plus leaves). Record each allocated Layout and assert `Leaf_actual ≤ Leaf_up` and `Internal_actual = up(up(Leaf_actual,8)+96, …)`.
2. **Hash tables:** for each tuple in the roster and k ∈ {1, 3, 4, 7, 8, 14, 15, 28, 29, 56, 57, 112, 113, 192}, assert that `with_capacity(k)` requests exactly `HashReq(s, a, Buckets(s,k))`; then insert-only growth to the D1 high-water population.
3. **Vec and String growth, exhaustively over the D1 ranges.** For stride classes s ∈ {1, 8, 24, 32, 48, a type > 1024}, every h from 1 to the D1 maximum for that site, assert `capacity() == PushCap(s,h)` and that the requested Layout equals `s·capacity`. The site maxima are 192 typed elements, 16,384 raw values and 131,072 string bytes. The check is finite because D1 is bounded.
4. **serde_json:** the initial `to_string` request is 128 bytes, and `ErrorImpl`'s size and alignment are measured.

A failed witness test is a qualification failure at G6, not a runtime state. The runtime guard is §2.

## 5. Limits

- The compiler identity pins std. It does not pin a downstream workspace's lock (§2.3 residual).
- Layout witnesses cover features the formulas depend on; they are not a general dependency audit.
- All numbers in `_run_records/caps_arithmetic.out.json` marked ASSUMED are illustrative 64-bit values for order of magnitude only. G5 and G6 evaluate in-build.
