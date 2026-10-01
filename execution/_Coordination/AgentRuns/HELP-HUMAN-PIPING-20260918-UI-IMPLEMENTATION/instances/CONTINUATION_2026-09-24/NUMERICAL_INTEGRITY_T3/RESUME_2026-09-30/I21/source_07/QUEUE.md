# Output 1 — pinned RCM VecDeque binding

TASK I21; source-only start 2026-10-01 10:19:56 UTC, deadline10:39:56 UTC.
Frozen product source40129a225d73860ac2a53da9a2fa73869df668f3.
RV30 kernel_01/backcheck_01 closes F1/N1 and the finite-roster delta within
its conditional boundary; its seal is verified in SOURCE_BINDING.json.
No Rust compiler, solver, model or allocator probe was invoked here.
K=projects/chirality-piping/core/solver/frame_kernel/src/structural/retained.

## Population from the actual calls

K/factor.rs:226-242 bfs_reachable creates marked[f], marks the seed before
enqueue, and marks every discovered neighbor before reachable.push and
queue.push_back. Therefore each component vertex is enqueued at most once.
A popped vertex is not enqueued again, including duplicate adjacency entries.

K/factor.rs:324-337 creates a second queue only after pseudo_peripheral_start
returns. It marks start and each neighbor before enqueue in the same way.
Each queue is fresh for one connected component; it is dropped at that
component's loop end. Previous component capacities do not accumulate.

For component cardinality c>=1, queue length is initially1. After the seed
is first popped, at least one of the c vertices is permanently outside the
queue, and marking prevents duplicate enqueues. Thus high-water Q(c) is
bounded by max(1,c-1). Across the free graph use c<=f:

    Qmax(f)=0 if f=0, otherwise max(1,f-1).

A tighter connected-component descriptor may replace f, but is not needed
to close this queue term. This is a source count bound, not a queue trace
obtained by running the model.

## Installed implementation binding

The existing installed VecDeque source page identifies Rust1.97.1
(8bab26f4f 2026-07-14), matching the source_02 full commit
8bab26f4f68e0e26f0bb7960be334d5b520ea452 and layout_04 target.
DOC_BINDING.json records its original HTML SHA and one-off decoded-source
SHA. Decoded lines preserve original numbered source lines.

- new at vec_deque/mod.rs:828-830 uses RawVec::new, zero len/head/capacity.
- capacity at1052-1053 is RawVec capacity for non-ZST usize.
- is_full at236-238 compares len==capacity. There is no extra reserved
  sentinel slot; the entire capacity can hold live elements.
- push_back at2277 delegates to push_back_mut:2295-2303. It grows only
  when full, then writes one element in the same buffer.
- grow at2794-2803 calls RawVec::grow_one once. Source_02 RawVec:158-165,
  186-188,490-531 gives first capacity4 for8-byte usize, then doubling
  on these full-buffer grow-one calls.
- pop_front:2136-2147 updates head/len and reads the element. It neither
  reserves nor shrinks.
- handle_capacity_increase:671-720 rearranges the shorter ring segment
  inside the grown backing via ptr::copy / ptr::copy_nonoverlapping
  (:336-382). No auxiliary heap buffer is allocated for wrapping.
- Each call owns exactly one RawVec backing. usize elements have no child
  allocations; queue metadata is stack storage.

These are **pinned implementation facts**, not an API promise of future
VecDeque growth strategy. Source_02's RawVec request/bookkeeping and existing
observer semantics apply; no allocator size-class inference is used.

Let P8(q)=0 when q=0, else max(4,next_power_of_two(q)).
Let OQ(q)=0 for q<=4, else4*P8(q) bytes. Then:

    QueueRet(f) <= 8*P8(Qmax(f))
    QueueMove(f) <= QueueRet(f)+OQ(Qmax(f))

OQ is one current old-buffer request at a possible growing realloc. The
first allocation has no old-buffer surcharge. A queue which later empties
retains its capacity until that local queue drops. Ring movement does not
add another buffer identity.

Pure integer spot checks (not runs): component c=1 gives Q<=1, retained32,
old0; c=5 gives Q<=4, retained32, old0; c=6 gives Q<=5, retained64, old32;
c=10 gives Q<=9, retained128, old64. f=0 creates no queue. Arithmetic must
remain checked in any later estimator; overflow is not a small bound.

## RCM phase placement and other live Vecs

Only one of the two queue sites is live at once. bfs_reachable returns before
the first bfs_eccentricity call at:277. bfs_eccentricity uses current/next
Vecs, not a VecDeque (:244-265). During later eccentricity calls, component,
saved last_level, current level and next_level may coexist, plus marked[f].
All of those pseudo-peripheral locals drop before the final RCM queue at:326.

Keep the source_05/source_06 RCM neighbors/degrees/visited/order/caller
adjacency prefix. Above that prefix the relevant alternatives are:

    Reach_R <= f + G_8(c) + QueueRet(c)
    Reach_M <= Reach_R + max(OR(c),OQ(Qmax(c)))
    Ecc_R <= f + 4*G_8(c)
    Ecc_M <= Ecc_R + one active Vec grow's old request
    MainQueue_R <= QueueRet(c)
    MainQueue_M <= MainQueue_R + OQ(Qmax(c))

Here QueueRet(c) means the formula above using component count c, not
population q. reachable starts as vec![seed] with capacity1. Its retained
upper G_8(c) is conservative; OR(c)=0 for c<=1,8 bytes for2<=c<=4,
and4*P8(c) for c>4. reachable.push precedes queue.push at:236-237,
so their growing old buffers are alternatives, not simultaneous charges.
The existing neighbor-sort alternative remains separate. Complete
neighbors/visited/order padding is allowed exactly as declared in source_06.

The former Qdeque parameter in C1/K07 is therefore replaced by this finite
pinned-source expression. Input array/String capacity C1, T1/T2, C2,
final-A1 O2 and H/VR W1 remain open. This queue closure is not full E_max,
admission, fixture narrowing or acceptance of later source revisions.
