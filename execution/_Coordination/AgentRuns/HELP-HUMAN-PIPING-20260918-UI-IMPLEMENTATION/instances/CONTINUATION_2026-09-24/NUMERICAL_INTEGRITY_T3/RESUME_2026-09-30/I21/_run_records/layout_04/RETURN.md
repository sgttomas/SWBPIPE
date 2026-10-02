# I21 layout_04 — bounded actual-layout return

**Two authorized diagnostics completed; actual layouts bound, full E_max open.**

Frozen source: `cb13fcf3ea560c0d07ad9ac02dac78e9d2e06e00`.
Retained production source is byte-identical to source_03's A1 basis.
Existing TASK `/root/t3_recovery_manager/i21_k6c`, parent
`/root/t3_recovery_manager`; no delegation or maintained-code/Git/index writes.
Start 08:12:46 UTC, common deadline 08:52:46 UTC; final verification records finish.
Initial grant e30e94090ab1f671ae3a1a6a09f47732624c18fd, SHA256
8145f6a6302b3bc38b9ed04d1a977ec8780e3387f5c9e977f7be6bf3063d253a;
the existing-package clarification was read at71bd33e89041bf71062672c688d3b20ad7b739b5.
metric_design_01/PROPOSAL was read and its unchanged-admission boundary retained.

## Execution and prebuild checkpoints

| Run | Fence/release | Execution and result |
|---|---|---|
| VR | Manager independently checked 189 original archive files, three appended hook/export changes plus one example; released 08:23:11 UTC | PTY 21772, exit 0. Log-open 08:24:35.014827 UTC is a **filesystem launch proxy**, not a directly sampled launch clock; last write 08:24:46.571192; completion observed 08:25:17. 151 layout rows. |
| H | Separate original dependency archive,227 originals unchanged, one 24-line example; separately released 08:28:02 UTC after VR finished | Direct prelaunch clock 08:28:41 UTC, PTY 31638, exit 0. Log-open 08:28:41.461360; last write 08:28:54.442295; completion observed 08:29:38. Eight rows. |

No compiler repairs (0 of maximum 2). No solver/model construction, corpus,
scale, timing benchmark or mutant ran. Both diagnostic jobs are stopped.
Existing memguard PID 5387 was verified before each job; it was not modified.

Installed rustc 1.97.1, full commit 8bab26f4f68e0e26f0bb7960be334d5b520ea452,
LLVM 22.1.6; target aarch64-apple-darwin, release. Commands used offline/locked,
-j4, incremental 0, test threads 2, auto-install 0 and distinct target/vr,target/h.
Inherited mutation/Rust flags/wrappers were cleared as released.
Verbose rustc invocations and runtime metadata show normal production cfg:
no test/seeded features; debug_assertions false. serde_json uses its existing
default/std/float_roundtrip features; no preserve_order/arbitrary_precision.
Original manifests/locks are unchanged before and after builds.
BUILD_IDENTITIES.json binds executable hashes, locks, target/profile and
effective command/config evidence. Post-run Cargo config search found none
at inspected current-directory ancestors or default Cargo home; actual compiler
arguments remain the effective-build evidence.

VR binary SHA256:
abd8f47b2d0378de3c6c916aa907915755de5ca67eb03ed866d1c950638e9cb5.
H binary SHA256:
2c966300f2f3211264b328a943e7872a4234bc579927a08c128348ddc9dc0df7.

The exact PTY pipelines are in RAW_COMMANDS. /usr/bin/time -l wrapped env/cargo;
tee was outside that timed command. Its raw rusage is supervision evidence,
**not a rustc/process-tree peak, kernel observation, or E_max calibration**.

## Source-bound facts

LAYOUTS.json joins all 159 reported rows to actual type_name, sizeof, alignof,
and the exact label/type/source map. Three rows intentionally cross-check FK
nominal types across the separately built packages; all agree.

| Actual type / instantiation | Size / alignment (bytes) |
|---|---|
| RetainedSolve; AttemptRecord; PublishedRow |1976/8;832/8;64/8 |
| QuantityMeta; QuantityId |20/4;12/4 |
| Wide and Option<Wide>, L4/L8/L16 |48/8;80/8;144/8 for each corresponding pair |
| BlockBound L4/L8/L16 |208/8;336/8;592/8 |
| BoundRefusal; Option<BoundRefusal>; BlockRefusal |16/8;16/8;24/8 |
| ExactWideSum; tracker lazy row; tracker table entry |2144/16;4304/16;40/8 |
| Tracker key; BoundedExtremeTracker |8/4;88/8 |
| CasePrep; GroupPrep; GroupCache; group Vec entry |416/8;352/8;1360/8;1464/8 |
| AttemptStop; AttemptReason; AttemptOutcome; CaseOutcome |24/8;32/8;40/8;80/8 |
| H PrecisionWork; Segment; K6Model |336/8;32/8;200/8 |
| serde_json Value; Number; Map wrapper |32/8;16/8;24/8 |

The complete table includes all selected Shared/Solved/VerifyShared/report
instantiations, shifted-factor control tuples, fallback option types, source
elements, sparse-tail records, VR Case/Model/Row/Control types and exact H tuple
field types. H tuple rows are those actual anonymous Rust tuple types,
**not** substituted nominal H layouts. Reported headers/components are charged
only when their actual owner allocates them; sizeof(K6Model/CaseRun) is not
their dynamic heap and children remain separately accounted.

ARC_DERIVED.json has 15 **derived**, not directly measured, Arc allocation
requests. Authenticated installed alloc/sync.rs:387-405 gives
repr(C,align(2)) ArcInner with two Atomic<usize> fields and T; the diagnostic
reports AtomicUsize 8/8 and actual payload sizes/alignments. The formula is:

```text
offset = align_up(2*sizeof(AtomicUsize), alignof(T))
alignment = max(alignof(AtomicUsize), alignof(T), 2)
request = align_up(offset + sizeof(T), alignment)
```

Examples: Arc<CasePrep> request 432/8; Arc<GroupPrep> request 368/8.
These exclude separately owned child buffers. The original HTML hash,
byte-matching decoded source and version are in ARC_SOURCE_BINDING.json.

## Formula implications and exact remaining cells

- Replace the old copied-struct Option premise with actual matching Wide/
  OptionWide sizes for this source/toolchain/target. OPTION_EXTRA is0 for
  these instantiations.
- BlockBound's actual strides exceed the legacy 4*w+8 by 8 bytes per element
  at all three widths. Its refusal field needs the measured stride.
- Source_03's fallible publication collection uses 64-byte actual rows.
  Keep P(q), growing old+new capacity and its phase/lifetime distinctions;
  do not turn logical q into capacity. RetainedSolve's 1976-byte Box payload
  includes the radius handle, not its separate 8q array.
- Use actual changed enum/table/attempt strides and derived Arc requests in
  the existing owner ledger. Actual lazy/fallback inline elements are not
  a new heap allocation for stack-local H accumulators.
- H-prefix nuance from metric_design: the outer prefix read at main.rs:965-966
  occurs after Observer::end's stage-line emission. The prefix envelope must
  include that stage-line construction/emission, while the timed repeat's
  inner stage snapshot precedes it. prefix_line remains after the prefix
  sample. Earlier statements excluding stage-line output apply to timed
  repeat snapshots, not this outer prefix reread.
- Preserve source-derived prelaunch composition, both existing H-stage /
  VR-global metrics, chronological admission and backstop rules. No same-run
  observed-cut replacement or new observer is selected.

Still unclosed:

1. Actual std-private **repr(Rust) BTree LeafNode** layouts for the required
   key/value pairs (geometry; tracker/holding; free-adjacency set; published
   map; String→String/Value/floor pair; borrowed row map; sparse allowance map).
   Actual key/value components are reported, but map wrappers and components
   do not measure private nodes. InternalNode repr(C) remains conditional on
   the unresolved real leaf size/alignment. No mirror leaf was built.
2. Inaccessible std-private I/O Custom/StringError allocation layouts and
   remaining finite formatting/serde/runtime/input/argv source envelopes.
   No struct recreation, allocator interception or library patch was used.
3. Application capacity/lifetime/aliasing union, descriptor completeness,
   all prior U3–U7 remaining source cells, final-A1 reconciliation and fresh
   independent full-bound review. These sizeof facts alone establish none.
4. Shared estimator implementation, required suites/mutants, additive admission
   replay and W1-T4 measurements still need their separate grants/basis.

## Preserved evidence and write inventory

Complete approved overlays and exact label maps: OVERLAY.diff,
OVERLAY_FILES.json, LABEL_TYPE_MAP.json; H_OVERLAY.diff,
H_OVERLAY_FILES.json, H_LABEL_TYPE_MAP.json. Both post-build archive comparisons
confirm the same approved fences and untouched type definitions/numerical flow.
The archive source tar hashes and executable IDs are in BUILD_IDENTITIES.

Original logs are preserved byte-for-byte in owned scratch:
`<I21_LAYOUT_SCRATCH>/layout_04_cb13fcf_20261001/raw/BUILD_VR.original.log`,
SHA256 5fd6e8cd93fb0fbf916dcb7a493f269f41c4366fa6d1abbdfb36a95bde6098c5;
`<I21_LAYOUT_SCRATCH>/layout_04_h_cb13fcf_20261001/raw/BUILD_H.original.log`,
SHA256 65876ccfe41ce17fe21693bd7966fbbd9c3f4c08c402a298d42e165d6e8274bc.
Committed BUILD_VR.log and BUILD_H.log are deterministic path-only portable
exports; LOG_TRANSFORM.json records both hashes and mapping. No other log
bytes were rewritten.

All new writes are within layout_04, owned i21-layout scratch archives and
the granted k6c-layout-target subdirectories. Prior seals remain unchanged.
The final SHA256SUMS seals this finite packet; the scratch/target inventory
records reproducible source images, overlays and normal build artifacts.
**No diagnostic, solver or experiment remains running.**

