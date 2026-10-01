# Output 2 — minimal prospective kernel layout witness

**Proposal only. No overlay applied; no build, probe or measurement authorized
or run.** Start10:19:56 UTC; boundary10:39:56 UTC. Immutable product40129,
installed-source1.97.1, normal production configuration. All previous seals
remain unchanged. This is limited to three kernel BTree specializations and
five omitted accessible actual types; no consumer/IO or general layout work.

P=projects/chirality-piping; FK=P/core/solver/frame_kernel;
H=P/core/solver/performance_harness; K=FK/src/structural/retained.

## Exact targets

| Label | Actual type / source |
|---|---|
| geometry nodes | BTreeMap<u32,usize>, matching factor.rs geometry node index |
| tracker nodes | BTreeMap<(RuleTest,u32,Kind),BoundedExtremeTracker>, matching adaptive.rs:809-812 / rule:2136 |
| holding nodes | BTreeSet<(RuleTest,u32,Kind)>; internal value is the actual std SetValZST, never a substitute unit type |
| NotAssessed element | (u32,SpringKind), using retained_api::SpringKind exported from source.rs:104 |
| input-derived DOF | retained_api::Dof, source.rs:63 |
| resolution/floor evidence | actual tuple (u32,u64,u64), adaptive.rs:3407,3418 |
| theta evidence | actual tuple (u32,f64), adaptive.rs:3413 |
| certified-bound evidence | actual tuple (u32,u64), adaptive.rs:3416 |

The last five use ordinary sizeof/alignof of these actual types, as in
layout_04. Dof and SpringKind are already exported by structural.rs:29-32.
No mirror structure, copied enum or guessed repr(Rust) padding is permitted.

## What the existing observer can and cannot establish

H/src/bin/k6_observe/alloc.rs remains byte-for-byte unchanged.
current():42-44 reads requested live **bytes**. alloc/alloc_zeroed:168-199
charge Layout::size; dealloc:201-203 releases that size. calls():59-60
counts successful alloc/alloc_zeroed/realloc calls. Both are atomic reads and
allocate nothing. No requested-alignment observation exists. Pointer
alignment, pointer trailing zeros, allocator size classes and copied structs
are not alignment evidence.

A source-isolated first-node request can bind leaf request bytes L. A
source-isolated first root split can then bind internal request bytes I.
Obtaining both L and I for the **same three exact specializations** is enough
for requested-byte TI=[1+floor((k-1)/5)]*max(L,I), with the existing empty-root
and child-heap rules. It does not bind private node alignments or a general
private-layout interface. Alignment remains explicitly unknown.

## Complete proposed allocation windows

Use ordinary insert, ascending12 distinct scalar keys, no FromIterator,
cloning, bulk build, remove or balancing exercise beyond the first root split.
Construct all twelve keys and values in fixed stack arrays before sampling.
Tracker values are BoundedExtremeTracker::new(Direction::Up), with empty
Vecs and inline fields only (adaptive.rs:677-685); do not call offer.
Choose (RuleTest::Disagreement, body0..11, Kind::Force) for the two private
key types. Geometry keys are u32 values0..11, values the corresponding usize.
Key comparisons are scalar/derived enum/tuple ordering and allocate nothing.

The witness keeps a concrete container live through each after-sample, passes
a reference to std::hint::black_box, and checks its len/key contents only after
sampling. Formatting/output happens only after all three trials and five
sizeof/alignof values have been captured. Fixed arrays of sample integers,
iterator state and callback pointers are stack data, not heap logs.

| Step / samples | Every allocation / ownership fact |
|---|---|
| Prepare | Empty container has no root allocation; preconstructed keys and empty tracker children own no heap. Extract each insertion operand before its window. Record m0=current(), c0=calls(). |
| Insert key0 | Exactly one Box<actual LeafNode<K,V>> allocation. Record m1,c1 while container remains live. Require c1-c0=1 and L=m1-m0>0. |
| Insert keys1..10 | These fit the existing11-entry leaf. Outside the narrow first/root-split windows, require m11=m1 and c11=c1 afterward. No child allocation, replacement/drop or new node. |
| Insert key11 | Full root leaf splits: one additional leaf request L and one new internal-root request I. Original leaf remains a child; no node is freed. Record m12,c12. Require c12-c11=2 and derive I=(m12-m11)-L>0 with checked subtraction. |
| Keep observable / drop | After samples verify all12 keys are present; then drop container outside windows. Empty tracker children have no allocations to free. Require current() returns to m0; this checks the final live total2L+I. Output only afterward. |

No assertion/panic formatting, stdout initialization, String, Vec report,
model, solver, source construction, Arc, tracker.offer or unrelated allocation
belongs inside these windows. No new thread is started. Standard-library
BTree operations use the default Global allocator. The existing allocator
forwards to System but counts the requested Layout bytes, not System overhead.

Source proof:
- map.rs:651-652 new has root=None.
- map.rs:1045-1057 insert chooses VacantEntry for these distinct keys.
- newly bound map/entry.rs:381-402 creates one new_leaf on the first insert;
  subsequent insertion calls insert_recursing and pushes a root level only
  if a split reaches the root.
- node.rs:43-46 has capacity11; :85-94 allocates actual LeafNode via
  Box::new_uninit_in; :120-127 allocates actual InternalNode likewise.
- source_03's authenticated boxed.rs:574-618 routes new_uninit_in through
  try_new_uninit_in to one allocator request for Layout<MaybeUninit<T>>;
  MaybeUninit preserves T's size/alignment. No separate Box header request.
- node.rs leaf insertion/split and insert_recursing route one full leaf
  through one new right leaf and one new internal root at the twelfth insert.
  Existing old leaf is moved under the new root, not deallocated.
- set.rs:900-904 delegates to the same map.insert using actual
  SetValZST::default(); set_val.rs:1-6 shows the private zero-sized marker.

This is a proposal for an actual-type request witness, not calibration from
a model peak. The sampling equalities are necessary rejection checks, not
permission to waive the source-isolation argument.

## Minimal prospective overlay and command fence

Only an owned archive of the exact accepted source would be changed after a
new ROOT grant. Proposed touched files:

1. FK/src/structural/retained/adaptive.rs: append one temporary, fixed-purpose
   report hook. It instantiates only the three containers above and returns
   stack sample arrays. It can name private RuleTest/BoundedExtremeTracker
   directly; their definitions and visibility stay unchanged.
2. FK/src/structural.rs: one temporary retained_api re-export of that hook.
3. H/examples/i21_kernel_layout.rs: new tiny caller, importing the existing
   unchanged allocator file by path and registering that existing K6Alloc.
   It captures five actual sizeof/alignof pairs, calls the hook, then emits
   finite labeled records after every sampling window has ended.

Proposed exact hook interface (diagnostic samples, not a replacement type):

    pub fn i21_node_requests(
        current: fn() -> usize,
        calls: fn() -> u64,
    ) -> ([[usize; 5]; 3], [[u64; 5]; 3])

Rows are the three concrete container specializations; columns are empty,
after first insert, after eleventh insert, after twelfth insert, after drop.
Three explicit specialized blocks use the window sequence above; no public
generic probe, custom allocator, replacement observer or reusable framework.
The five sizeof/alignof labels are produced in the H caller from actual
retained_api/tuple types. No target type definition is edited.

The exact prospective type expressions are
sizeof/alignof::<(u32, retained_api::SpringKind)>(),
sizeof/alignof::<retained_api::Dof>(),
sizeof/alignof::<(u32,u64,u64)>(),
sizeof/alignof::<(u32,f64)>(), and sizeof/alignof::<(u32,u64)>().
Here sizeof/alignof denotes the two ordinary std::mem::size_of / align_of
calls; it is notation, not a new helper or type.

Example registration is the existing allocator, not a new implementation:

    #[path = "../src/bin/k6_observe/alloc.rs"]
    mod alloc;
    #[global_allocator]
    static ALLOC: alloc::K6Alloc = alloc::K6Alloc;

No manifest/lock/dependency/config/cfg/feature/allocator/guard change is proposed.
Use the original H package/dependencies/lock and exact frozen FK source. H's
manifest already depends on frame_kernel. The diagnostic archive should
contain only the existing H dependency closure, as in layout_04; no third
package or expanded framework is needed.

Prospective payload command, **not run or granted now**:

    RUSTUP_AUTO_INSTALL=0 CARGO_INCREMENTAL=0 RAYON_NUM_THREADS=2 OMP_NUM_THREADS=2 \
      cargo +1.97.1 run --release --offline --locked -j4 \
      --manifest-path <ARCHIVE>/projects/chirality-piping/core/solver/performance_harness/Cargo.toml \
      --example i21_kernel_layout --target-dir <ROOT_GRANTED_TARGET>

The caller must use ROOT's existing guarded non-timed M5/PTy/time-l/operator
supervision envelope and cleared inherited mutation/RUSTFLAGS/wrapper
overrides, with no invented cap or deadline. Those are later release
conditions, not an allocation-window operation. ROOT must review the full
concrete overlay, file fence and label/type map before any compile slot.
This packet supplies the minimal prospective file/interface/window design;
it does not claim a compiled or already-applied overlay.

## Binding checks and stop boundary

Before any later release, bind source tar/overlays, unchanged manifests/locks/
allocator and target definitions, installed1.97.1 full rustc commit/target,
normal production cfg/features and the authenticated std source origins.
No cargo/test feature may alter these target types. Confirm the private hook
really instantiates the same exact key/value types, including the set marker
through BTreeSet; a fake unit map is rejected.

The later run must satisfy every call-count/live-byte/content/drop equality,
with positive checked L/I derivations. It must keep containers observable
through samples. Unexpected allocation/deallocation, overflow, optimized-away
allocation, zero/missing sample, changed source/compiler, offline dependency
failure, or inability to isolate the window means **no binding**. Stop and
return that gap; do not install, fetch, adjust allocator/observer, inspect
pointer alignment, introduce a mirror, or engineer an alternate probe.

Even a successful later witness supplies only six actual request-byte facts
(three leaf and three internal) plus five actual size/align pairs. It does
not establish full E_max, caller capacities, descriptors, H/VR consumer
composition, final A1 reconciliation or admission. No measurement is claimed
in source_07; T1/T2 stay open pending the separately granted witness/review.
