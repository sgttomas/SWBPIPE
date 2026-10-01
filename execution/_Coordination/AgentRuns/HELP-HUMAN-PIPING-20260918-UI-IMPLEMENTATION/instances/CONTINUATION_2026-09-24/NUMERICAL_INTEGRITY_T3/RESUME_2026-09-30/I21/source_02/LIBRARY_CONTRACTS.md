# I21 source_02 — selected installed-library allocation contracts

Status: source derivation for the finite H/VR accounting domain. These selected
contracts are now bound to readable installed source. No numeric universal
E_max, private layout, final A1 owner union, compiled candidate or measurement
is approved by this packet.

## Source and version binding

The existing installed documentation identifies Rust **1.97.1
(8bab26f4f 2026-07-14)**; version_info links full commit
`8bab26f4f68e0e26f0bb7960be334d5b520ea452`.
The installed channel manifest, dated 2026-07-16, names that same version for
rust-docs, rustc and rust-std for aarch64-apple-darwin. Components lists rust-docs.
Every selected HTML page has the same rustdoc version/channel metadata.

VERSION_BINDING.json preserves those records and original file hashes.
DOC_BINDING.json and DOC_BINDING_EXTRA.json give each exact HTML origin/hash,
decoded hash, line count and metadata. The one-off Python stdlib decode kept
text inside pre.rust/code, omitted only numbered data-nosnippet anchors, decoded
HTML entities, and preserved code whitespace in UTF-8. All anchors were checked
contiguous from 1 and against decoded line counts. The command is recoverable
in RAW_COMMANDS.json; no reusable extractor was created.

Twenty directly relevant pages were decoded. Seventeen have historical
source hashes in the immutable response's SOURCE_INPUTS; all seventeen match
byte for byte. IntoIter, Result and BTreeSet have no historical comparator;
their null historical hash / false comparison field means **not compared**,
not a source mismatch. Their original HTML/version/decoded hashes are bound
here independently.

Initial location/version/decode binding completed at 05:34:00 UTC, 67 seconds
after recorded start 05:32:53 UTC, within the 10-minute allowance.
No source fetch/install/recovery or compiler call occurred. This is local
installed-documentation identity, not independent verification of a downloaded
archive or proof that a future binary used these standard-library bytes.
Final compiled candidate/toolchain identity still must be bound later.

All references below are original source line numbers, identical to decoded
line numbers. D=source_02/decoded; names such as alloc/raw_vec/mod.rs map to
D/alloc__raw_vec__mod.rs. Original HTML paths are recorded in DOC_BINDING.

## Contract types and metric

**API** means stated in the installed public documentation.
**Impl** means this pinned implementation, not a promise for future Rust.
**Derived** means a mathematical consequence of the displayed implementation
under the named finite construction path.

The metric is requested allocator bytes and the existing K6 growing-realloc
move counter. It is not allocator size-class consumption, RSS or footprint.
RawVec current_memory uses capacity*element size (raw_vec:633-645).
Global forwards equal-alignment grow/shrink to realloc (alloc.rs:236-325).
Both actual observation allocators charge new-minus-old to current bytes and
old+new to the growing move peak; their shrinking branch releases the difference
without adding an old+new move peak (H/alloc.rs:206-229;
VR/examples/vk_scale.rs:215-235). Those source files were hashed anew.

Let s=actual sizeof(T), a=actual alignof(T), nonzero type unless stated.
Let E(k,T)=k*s. Zero-length/zero-sized requests contribute zero heap.
Successful arithmetic/allocation paths are assumed; overflow/refusal never
licenses a wrapped small estimate.

## Vec / RawVec derivation

| Contract | Evidence | Consequence for source_01 |
|---|---|---|
| Exact reservation request (API) | vec/mod.rs:385-392 explicitly guarantees vec![...], vec![x;n], with_capacity(n) request exactly n elements, though allocator may provide more | E is justified for these request sites. No power-of-two rounding for explicitly reserved block/row arrays. |
| No automatic shrinking (API) | vec/mod.rs:367-378; growth strategy explicitly unspecified at :380-383 | Dedup/retain/drain/pop/clear do not license reducing capacity. General Rust compatibility cannot assume doubling. |
| Exact local capacity bookkeeping (Impl) | raw_vec:446-481,772-778 stores the requested capacity and allocates array Layout; :633-645 forms capacity*s | On this implementation the source recurrence tracks requested bytes directly. |
| Minimum first grow (Impl) | raw_vec:153-165 | mu(s)=8 for s=1; 4 for 1<s<=1024; 1 for s>1024. ZST does not allocate. |
| Amortized grow (Impl) | raw_vec:502-531 | On an actual grow: c'=max(2c,len+additional,mu). Otherwise no allocation. Record the actual sequence, not only final length. |
| Exact reserve (Impl) | raw_vec:785-803 | reserve_exact's grow requests len+additional, not max(2c,...). This can produce non-power-of-two starting capacities. |
| Shrink (Impl) | raw_vec:829-861 and vec/mod.rs:1604-1609 | Successful shrink_to_fit requests len; zero deallocates. An old table remains live while a different kept Vec is shrunk and assigned. The move metric does not add shrinking old+new. |
| Fresh lower-zero collect (Impl) | spec_from_iter_nested:18-42; vec/mod.rs:4022-4043 | First nonempty allocation mu; at full capacity reserve(lower+1)=reserve(1). Capacity is P(k)=max(mu,next_pow2(k)), k>0; P(0)=0. |
| TrustedLen / exact lower-hint collect (Impl) | spec_from_iter_nested:46-61; spec_extend.rs; generic first capacity lower+1 at :27-30 | TrustedLen uses E(k). A fresh borrowed exact-size chain with the exact remaining lower hint also initially reserves k and need not grow. Each site still must be classified correctly. |
| Fresh clone (Impl) | slice.rs:396-445, especially :425; Vec clone delegates to slice machinery | Clone copies len, not old capacity. Child Vec/String clones are additional allocations. clone_from is different and can retain/reuse capacity (:813-833). |
| Consumed input lifetime (Impl) | vec/into_iter.rs:541-559 | IntoIter owns the backing allocation until dropped, even as elements move. Count tagged input backing during construction of destination structures. |

**Derived scalar operators.** For a fresh push/lower-zero collection with final
or branch upper count k, G(k,T)=P(k)*s. Its growth-move envelope is
MG(k,T)<=1.5*G(k,T), conservatively also valid before the first grow.
For a nonempty fresh collection with a valid positive lower hint, or a fresh
resize/extend sequence whose maximum required length is K,
A(K,T)=max(mu,2K)*s is a safe retained upper. Every grow has old c<required<=K,
so c'<=max(mu,2K); the active old+new move is <=1.5*A.
For a preexisting capacity c0 use the actual recurrence, or
max(c0,mu,2K)*s (and its conservative 1.5 multiple), not A(K) alone.
Retained storage after a shrink must use its new actual c before later grows.

A next-power-of-two model after shrink is false: shrink to 10, then need 11,
gives capacity 20; it can still hold length 13. The generic high-water bound
max(mu,2R) is safe when R bounds the required length throughout that tracker
table's history. It does not replace old-table + new-kept coexistence.

These bounds are source deductions; no observed capacity was used as proof.

## Fallible collect and owned-buffer reuse

Result FromIterator calls try_process then collect (core/result.rs:2155-2156).
GenericShunt reports lower hint 0 (core/iter/adapters/mod.rs:180-186), and
does not implement TrustedLen in the selected source. Its SourceIter/
InPlaceIterable forwarding (:208-232) means an owned-source chain can still
take the in-place specialization. Therefore the precise rule is:

- Borrowed slice/range fallible collection: no owned Vec to reuse; use P,
  including on a successful all-Ok result. In particular K/factor.rs:667-676
  has this path for scaled and final output vectors.
- Borrowed filter/filter_map: size_hint lower 0 (filter.rs:118-121,
  filter_map.rs:126-129); use P unless a separately proved count specialization
  applies. Do not infer ExactSize from the input array alone.
- Owned Vec IntoIter directly recollected can reuse its original capacity
  (spec_from_iter.rs:37-62). A partially consumed direct IntoIter reuses only
  under the stated half-capacity predicate; otherwise the fallback allocates.
- Owned in-place adapter eligibility requires non-ZST types, equal alignment,
  and sizeof(SRC)*MERGE_BY >= sizeof(DEST)*EXPAND_BY
  (in_place_collect.rs:168-245). Destination capacity is
  floor(source_capacity*sizeof(SRC)/sizeof(DEST)) (:254-264).
  Only a byte remainder causes shrink (:301-327). Result/fallible wrappers
  do not by themselves rule out this path.
- When relevant private layouts are unbound, safely permit both source backing
  and generic destination construction. After return, retain the maximum of
  inherited-source byte capacity and generic destination capacity. Counting
  only the generic destination after deleting the source identity is unsafe.

This closes the collection-rule gap for the actual borrowed retained solves.
H Builder node maps and other owned transformations remain conditioned on
their actual layouts or use the explicit two-buffer/inherited-capacity upper.
VR Nat u64->u32 map cannot meet equal alignment on the eventual usual target
facts; until those facts are bound, the conservative two-buffer path suffices.

## Finite capacity instantiations

Committed H counts were rehashed to
fab0466a846ef26b1c530a261a6f1d134fc32ba1d70f57682010877a52373c05.
All 33 fixtures have f>8, so P(f)=next_pow2(f) regardless of the nonzero type's
minimum-capacity class (mu is at most8).

For K/factor.solve, above the caller RHS, scaled holds C=P(f) wide slots;
solve_scaled holds two exact f-slot vectors; final fallible collection holds
old+new destination alongside scaled and the returned f-slot y.
A conservative move-peak coefficient is:

```text
max(1.5C, C+2f, C+f+1.5C) = f+2.5C  [wide slots]
returned capacity = C; fresh clone of returned values = f
```

The exact upper is not claimed attained on every error path. Multiply by actual
Wide stride later and add the correct caller owner union. This is not an E_max
delta: previous broad phase sums may contain other overcounts.

| Family (AX and ROT) | Members | f | C | returned slots above f | helper peak wide slots |
|---|---:|---:|---:|---:|---:|
| CHAIN/TREE | 10 | 60 | 64 | 4 | 220 |
| CHAIN/TREE | 100 | 600 | 1,024 | 424 | 3,160 |
| CHAIN/TREE | 1,000 | 6,000 | 8,192 | 2,192 | 26,480 |
| CHAIN/TREE | 10,000 | 60,000 | 65,536 | 5,536 | 223,840 |
| CONT | 10 | 45 | 64 | 19 | 205 |
| CONT | 100 | 450 | 512 | 62 | 1,730 |
| CONT | 1,000 | 4,500 | 8,192 | 3,692 | 24,980 |
| CONT | 10,000 | 45,000 | 65,536 | 20,536 | 208,840 |

CAPACITY_ROWS.json contains all33 records and the exact pure JavaScript
arithmetic used. No Rust model, solver or capacity probe ran.

Apply this result to source_01 K12/K13 and verification delta in K16/K18.
Fallback row list grows with the same P and has MG=1.5C slot upper.
Evaluated-state clones remain exact f (not C). Shift nl_pass at/bt/ct are
vec![...;f], reserved f and an exact clone; retain the earlier exact 3f term.
Do not spread the fallible-capacity correction into those exact vectors.

## BTree node and construction contracts

BTreeMap invariants: non-root nodes have at least5 entries, internal root at
least1; empty map can retain an empty leaf (map.rs:29-39). Node capacity is11,
B=6 (node.rs:43-48). Leaf/internal allocation uses Box<actual node type>
(node.rs:85-127). LeafNode has default Rust representation; InternalNode is
repr(C) with a LeafNode followed by twelve edge pointers (:51-110).

**Private node bytes remain unmeasured.** The source proves element/node
counts and what Layout is requested, not an invented leaf size.
If actual leaf size/alignment SL,AL and pointer size/alignment SP,AP are bound,
repr(C) gives:

```text
AI=max(AL,AP)
SI=round_up(round_up(SL,AP)+12*SP, AI)
```

Thus the smallest private node witness can bind actual LeafNode per K,V and
alignment, then derive InternalNode; a full-node witness can additionally
backcheck it. Do not substitute sizeof(BTreeMap), an amortized per-key constant,
or a mirrored repr(C) leaf for the actual Rust-layout LeafNode.

### Repeated insertion

For population k>0, Nnodes<=1+floor((k-1)/5). Budget
TI(k,K,V)=Nnodes*max(SL,SI)+key/value child heaps.
At k=0 a never-populated new map uses no node; a previously populated empty
map may retain a leaf. Retain high-water nodes during consumption/removal.

The transient insertion bound uses the completed insertion's population:
split allocates a right node (node.rs:1253-1262,1288-1306), then recursively
installs it (1058-1086). New split/root nodes survive into the resulting tree;
there is no second disposable copy of the complete tree. The count argument
does not use the temporarily underfull intermediate nodes as final invariants.

BTreeSet uses BTreeMap<T,SetValZST> (set.rs:82,900-904). Use the actual marker
type in any private-layout binding; no assumption that a guessed unit-valued
mirror has an identical layout.

### FromIterator / bulk

Map FromIterator collects a Vec<(K,V)>, stable-sorts it, then bulk-builds
(map.rs:2539-2553). Set FromIterator likewise collects Vec<T> and stable-sorts
(set.rs:1470-1487). Key duplicates can lower final population, but the input
Vec/sort term must retain the original input count k.

For a sorted unique stream of k>0, bulk_push fills11 keys in a leaf; the next
key is promoted to an ancestor and a new right subtree is created
(append.rs:19-57). Immediately before right-border repair:

```text
leaf_count = 1 + floor(k/12)
internal_count at height j>=1 =
    1 + floor(k / 12^(j+1)), if k>=12^j; otherwise 0
```

These are base12 carry counts, including temporarily empty right-spine nodes
when the last input ends at a promotion. At k=12 there are two leaves plus
a root, even though the right leaf is initially empty. At k=144 there are
13 leaves, two height1 nodes and one height2 root. Right-border repair steals
existing entries/edges and creates no node (fix.rs:105-119).

TBnodes is leaf_count*SL + sum internal_count*SI. A safe construction envelope:

```text
children_of_input
+ max(input_vec_growth,
      input_vec_retained + Sort(k,input_element),
      input_vec_retained + TBnodes(k))
```

Input backing remains while IntoIter is consumed. Moving String pairs into
nodes does not make another String copy. A separate deep clone does:
map.rs:226-306 recursively clones corresponding subtree structure and key/value
children, overlapping the original. An empty clone may omit a retained empty
root, so retaining the source topology is a conservative clone upper.

These close the source count/phase contracts for source_01 geometry/tracker
trees, VR raw Value/maps, borrowed-row controls tree, floor member map,
publication map and sparse allowance maps. Actual node layouts and any
unbound consumer key/population descriptors remain cells.

## Stable sort allocation

alloc/slice.rs:854-871 supplies Vec<T> as core's BufGuard; with_capacity
therefore requests exact scratch-element count. Normal driftsort uses

```text
a=max(ceil(k/2), min(k, floor(8_000_000/s)), 48)
```

(core stable/mod.rs:111-128; smallsort.rs:180-188 gives32+16=48).
When its4096-byte stack buffer holds a elements, no scratch heap is needed;
otherwise it allocates a elements. The optimize-for-size branch asks floor(k/2)
and has its own stack branch; 16-bit targets use that heap branch
(stable/mod.rs:42-68). Normal k<=20 uses insertion sort without a scratch heap.

A conservative target/branch-independent heap scratch bound for these sources
is Sort(k,T)=0 for k<2 or ZST, else max(k,48)*s.
Do not replace it with k*s for all small large-element rows.
Only one scratch buffer is owned by this sort call; recursive algorithms
receive the supplied slice. Comparator-owned allocations, if any, are separate.
The existing primitive/field/String comparisons must be checked at their
application call sites; this formula bounds algorithm scratch alone.

For tracker prune, preserve the source_01 lifetime split:
taken lazy exists during table extension and drops before prune; stable-sort
scratch ends before drain-to-kept; old table and growing kept coexist; shrink
then assignment drops old table. P(current_table_len) is not valid after a
prior shrink, while max(mu,2*offers_so_far) remains conservative.

## What this closes; exact remaining cells

The prior **missing standard source** blocker is closed for the selected
Vec/RawVec/fallible-collect/BTree/sort contracts. The source-location claim is
corrected additively; old sealed bytes remain unchanged.

Still open, without implied permission to investigate or implement:

- Actual private numeric/control/Arc/leaf-node layouts and target/compiler
  binding. Page identity is not a compiled layout witness. The InternalNode
  repr(C) reduction above can make the later witness smaller.
- Exact per-site classification and full application owner union at final A1:
  typed iterator reuse, decision summaries/keys, caller state, all report/end/
  refusal owners. Library facts do not establish those application facts.
- VecDeque, Arc request layout, String/float formatting, stdout, argv/path/read
  and locked serde parser/to_value contracts outside this finite page selection.
  No source-recovery work is requested; installed pages may support later
  narrowly authorized work if needed.
- Current finite input/model/decimal/JSON descriptors, runtime baseline and
  actual future argv/path/file values, including large-model bytes.
- Final H staged / VR global composition, independent review, implementation,
  tests/mutants, chronological admission replay and W1-T4 under ROOT grants.

Conservative alternatives remain: retain both sides when reuse is unbound;
carry high-water capacity and old/new replacements; count complete persistent
payload during partial build; use tree node-count uppers with symbolic real
node sizes. No measured slack, guessed layout or changed domain/metric is used.
Every application term depending on retained code remains A1-sensitive.

