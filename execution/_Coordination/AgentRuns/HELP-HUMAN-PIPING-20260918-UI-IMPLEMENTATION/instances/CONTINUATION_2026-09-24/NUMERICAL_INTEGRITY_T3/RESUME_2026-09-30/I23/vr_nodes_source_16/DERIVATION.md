# VR-NODES-7: exact source result and missing fact

The source establishes the actual request types, but does not determine any of the seven exact leaf size/alignment pairs. This is a private representation gap, not a deferred arithmetic evaluation of public component sizes.

Pinned alloc/collections/btree/node.rs43-67 defines B=6, CAPACITY=11 and `LeafNode<K,V>` with fields `parent: Option<NonNull<InternalNode<K,V>>>`, `parent_idx: MaybeUninit<u16>`, `len: u16`, `keys: [MaybeUninit<K>;11]`, and `vals: [MaybeUninit<V>;11]`. There is no repr attribute on LeafNode. InternalNode102-110 alone is `repr(C)` with `data: LeafNode<K,V>` followed by twelve `MaybeUninit<BoxedNode<K,V>>` edges; BoxedNode137 is the thin `NonNull<LeafNode<K,V>>` alias.

The installed Rust Reference explicitly leaves default-Rust field ordering, padding and additional layout guarantees unspecified. In particular, the leaf alignment is at least its fields' maximum, not proven equal to that maximum. Treating it as a C struct, sorting fields by alignment, assuming minimal padding, or inheriting the containing InternalNode's C representation would invent the missing premise. A field-compatible mirror or a different kernel specialization supplies no such premise.

Actual allocation route: LeafNode::new85-93 and InternalNode::new120-127 call Box::new_uninit_in for Self. Pinned boxed.rs606-616 requests Layout::new::<MaybeUninit<T>>() through the allocator for non-ZST T. MaybeUninit source214,243-244 guarantees the same size/alignment as T. Both node types are non-ZST because the leaf contains a nonzero-sized parent field. BTreeMap's default allocator is Global (map.rs189-200); Global forwards the same nonzero Layout to alloc (alloc.rs202-215). Thus the node request is the actual node's size/alignment, with no inferred map-header or allocator-size-class adjustment. Separately owned key/value children remain separate.

For each of the seven exact K,V specializations, let L=size_of(actual LeafNode<K,V>) and A=align_of(actual LeafNode<K,V>). Let P=size_of::<NonNull<()>>() and Q=align_of::<NonNull<()>>(). These public pointer expressions legitimately stand in for the same-sized thin BoxedNode edge pointers; they do not stand in for leaf layouts. The C-layout offset arithmetic is:

1. data offset=0; after data=L.
2. edges offset=align_up(L,Q); edges size=checked_mul(12,P), alignment=Q.
3. end=checked_add(edges offset, edges size).
4. internal alignment=max(A,Q); internal size=align_up(end, internal alignment).

align_up(x,a) adds zero when x mod a=0, otherwise checked_add(x,a-(x mod a)). Alignment must be a nonzero power of two. Refuse any overflowing product/sum or invalid Layout; do not wrap. Size is a multiple of its alignment.

On the bound layout04 target, thin pointers have usize's size/alignment, 8/8. NonNull is transparent and Option<NonNull<T>> has NonNull's size/alignment; MaybeUninit and arrays retain their element alignment. Hence A>=8, L is already a multiple of8, and twelve edges occupy exactly96 bytes. The exact conditional pairs reduce to:

    leaf request    = (L, A)
    internal request= (align_up(checked_add(L,96),A), A)

No A=8 is assumed. In particular, L+96 without final rounding is not asserted for an unbound A. These equations apply separately to all seven entries in `_run_records/SEVEN_REQUESTS.json`.

| Consumer | Actual K,V | Previously bound public K size/align | Previously bound public V size/align |
|---|---|---|---|
| JSON | String, serde_json::Value |24/8|32/8|
| Borrowed control lookup | &str, &cases::Row |16/8|8/8|
| Publication | QuantityId, PublishedRow |12/4|64/8|
| Floor | String, (f64,f64) |24/8|16/8|
| Adjacency | usize, actual SetValZST |8/8|Actual private marker; no compiler size/alignment pair added here|
| Sparse allowance | (usize,usize), (f64,usize) |16/8|16/8|
| Case strings | String, String |24/8|24/8|

The actual set marker is verified from pinned alloc/collections/btree/set_val.rs1-6: its own fieldless `pub(super) struct SetValZST`, explicitly described as the internal ZST used instead of unit to distinguish BTreeSet from user BTreeMap<T,()>. BTreeSet's field is BTreeMap<T,SetValZST,A> (set.rs82). No unit-valued substitution or guessed marker alignment is made. A real LeafNode<usize,SetValZST> size/alignment witness subsumes its marker field; a separate marker measurement is not required by the conditional internal derivation.

JSON's selected serde_json1.0.151 map implementation is BTreeMap under not(preserve_order), matching the already bound default/std/float_roundtrip build basis. No preserve_order/arbitrary_precision behavior is imported. Public component values above are the actual layout04 diagnostic facts at its cb13 source/normal release/aarch64/Rust1.97.1 basis. Final build/source/feature correspondence and checked public sizeof/alignof evaluation remain separate obligations; they cannot alone close the private Rust-layout fact.

Exact missing artifact fact: the actual requested size/alignment (or actual type size/alignment) of each of the seven selected private LeafNode instantiations on the bound final compiler/target/source/cfg. Seven real leaf pairs suffice for this representation reduction; internal pairs then follow from the pinned C layout above. No new witness run, probe, build, mirror, compiler-layout survey or alternate metric is commissioned by this return. The source02 insertion/bulk/clone contracts remain intact. No whole E_max, K0, admission or implementation acceptance follows.
