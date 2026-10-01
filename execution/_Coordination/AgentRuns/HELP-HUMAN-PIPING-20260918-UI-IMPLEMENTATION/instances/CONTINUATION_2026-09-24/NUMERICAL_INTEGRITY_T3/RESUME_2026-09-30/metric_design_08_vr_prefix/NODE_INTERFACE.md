# Counts node interface — additional explicit specialization

scale.rs61-68 builds BTreeSet<(u32,u32)> on actual RF-LARGE member pairs.
It is additional to the historical VR-NODES-7 list. Use its own leaf request
size/alignment and derived internal request; no equality with usize-key nodes
is inferred. Constructor population<=2m is source-derived, with source02's
insertion/split bound. The actual requests remain symbolic with the node owner.

The untouched-spring BTreeSet<(u32,usize,usize)> at scale.rs70 has a separate
reach premise. All24 actual RF-LARGE models (12 embedded plus12 existing external)
have zero springs, freshly confirmed by fixed metadata. It allocates no node
here. The broader counts API retains that distinct specialization when reached.

ROOT's in-turn message supplied the sealed vr_nodes_source_16 conclusion:
LeafNode has default Rust layout; public component facts do not yield its exact
request. InternalNode is repr(C), so the reviewed target formula derives its
request from actual leaf size/alignment. This packet did not read/rederive that
node proof or perform a layout/artifact probe. No free node allowance is introduced.
