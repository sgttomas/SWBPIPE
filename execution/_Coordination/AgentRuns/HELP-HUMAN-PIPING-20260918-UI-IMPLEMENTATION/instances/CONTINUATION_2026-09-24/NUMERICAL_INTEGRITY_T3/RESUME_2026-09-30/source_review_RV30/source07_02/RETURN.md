# RV30 source07_02 — queue derivation and prospective node requests

**Disposition: source queue derivation checks out; witness proposal is ready for concrete-overlay preparation and review. No blocking design finding in QUEUE.md or PROPOSAL.md.** This is not implementation review, compile/runtime authorization, a request-byte measurement, full E_max, or acceptance. Actual request binding still requires the proposal's source-isolated windows, reviewed concrete overlay and all rejection checks.

TASK Type2 RV30, direct native child `/root/rv30_k6c_kernel` of ROOT `/root` HELP_HUMAN Agent0; no delegation. The direct parent follow-up granted20 minutes, source only, and this additive write subtree. Actual start2026-10-01 13:09:48 UTC; deadline13:29:48 UTC. Completion is recorded in VERIFICATION.json after the review and checks. Same Root/TASK/Piping/COMMON/software-code-review instruction origins and hashes as kernel_01/INSTRUCTION_BINDING.json; no new workflow or role body selected.

Review basis:
- K6C head `1908c542966a11d3f64ad875ec2dfe4188a5e772`; targeted working diff for QUEUE.md and PROPOSAL.md is empty.
- Immutable product `40129a225d73860ac2a53da9a2fa73869df668f3`.
- R/I21/source_07 seal `1b23b5cea2d6beb42fc608afb50fd802f118f9d497ccfa40e8593d2b8dc7b1a7`; all nine payload hashes independently verified.
- QUEUE.md hash `d9bd10a1bd4af61ce0025a6510b8031deb164324401df766c8cf96bc15864058`.
- PROPOSAL.md hash `94eaa581fa54ab25d8edf2151cc79ffe94fcbeb84d2bc8d3b7a9cf6f17125268`.

Only the two requested proposal artifacts were reviewed as deliverables. Referenced product/library sources and their binding metadata were read to check their claims. No other implementation undertaking was reviewed.

## Queue result

QUEUE.md:12-31 correctly derives a source population bound. Both K/factor.rs queue sites mark before enqueue. Each component vertex enters that queue at most once. After the first pop, at least the popped seed is outside the queue permanently, so for component size c>=1 the maximum live queue population is at most max(1,c-1). Substituting c<=f gives Qmax(f); f=0 never enters either queue site.

The pinned implementation supports the capacity recurrence:
- VecDeque::new is an empty RawVec (source_07 decoded vec_deque:828-830).
- is_full is len==capacity, with no sentinel slot (:236-238).
- push_back_mut grows only when full (:2295-2303); grow invokes RawVec::grow_one once (:2794-2803).
- RawVec's sizeof(usize)=8 minimum is4 slots; full-buffer grows double (source_02 raw_vec:158-165,186-188,490-531).
- pop_front only adjusts head/len and reads the element (:2136-2147). Ring rearrangement copies within the grown buffer (:671-720); it creates no separate heap buffer.

Thus QueueRet<=8*P8(Qmax) and QueueMove<=QueueRet+OQ(Qmax) are valid requested-byte source uppers on the named implementation. The old-buffer charge is zero through the first4-slot allocation, then4*P8(q) for a possible last doubling. Emptying a queue does not shrink it. Recomputed integer examples match: c1/c5 =>32 retained, no old; c6=>64+32; c10=>128+64. These are source arithmetic, not queue runs or capacity probes.

QUEUE.md:80-102 also places overlaps correctly:
1. Reachability's marked[f], returned reachable backing and its local queue coexist. Reachable begins at exact capacity1; its first grow's old request is8 bytes. OR(c) correctly preserves this exception rather than treating it as a fresh zero-capacity Vec.
2. That queue dies before eccentricity. Later eccentricity may retain component, saved last_level, current level and next_level plus its marked[f]; four G_8(c) buffers are a conservative upper. Only the active Vec grow adds an old-buffer movement charge.
3. All pseudo-peripheral locals die before the final RCM queue. Main RCM order was reserved for f, so pushing one occurrence per vertex does not introduce another order growth.
4. Neighbor sorting is a separate transient alternative. Previous connected components' queue capacities do not accumulate.

The queue subcell may therefore replace symbolic Qdeque in K07/C1, conditional on this frozen product/std/target basis. This does not close the remaining input-capacity portion of C1. The three source_07 installed HTML pages were independently hashed and decoded in memory; all decoded bytes match the sealed extracts, not merely their reported line references.

## Node-window source result

The proposed three exact specializations match the kernel:
- geometry: BTreeMap<u32,usize>;
- tracker: BTreeMap<(RuleTest,u32,Kind),BoundedExtremeTracker>;
- holding: BTreeSet<(RuleTest,u32,Kind)>.

The private tracker value's constructor has only inline fields and empty Vecs (A:677-685); no offer means no lazy/table child heaps. RuleTest and Kind derive scalar ordering. BTreeSet::insert uses the real private SetValZST through map.insert (source_02 set:900-904; source_07 set_val:1-6). A map with unit value would be a different specialization and is correctly excluded.

The ordinary insertion path supports the intended identities:
- Empty map has root=None (map:651-652).
- First VacantEntry insertion calls new_leaf (entry:381-402); leaf allocation calls Box::new_uninit_in for the actual LeafNode<K,V> (node:85-94).
- Leaf capacity is11 (node:43-46). Distinct insertions2 through11 use insert_fit without a new node (node:947-980).
- Insertion12 splits the full leaf into the existing left leaf plus a new right leaf (node:1253-1262), then pushes one new internal root (node:1058-1086,598-605 and entry:394-400). No old node is deallocated in this path.
- InternalNode allocation uses Box of the actual internal type (node:120-127). The Box implementation requests Layout<MaybeUninit<T>> once, with no second heap header (source_03 boxed:574-618).

Under the explicit isolation and observed-call conditions, m1-m0=L and (m12-m11)-L=I bind actual requested bytes for that specialization. The retained request after insertion12 is2L+I. Measuring both requests directly avoids needing a private leaf alignment to derive the internal request. The insertion-node upper [1+floor((k-1)/5)]*max(L,I) can consume these request sizes without an alignment inference; the existing empty-root and child-heap rules still apply.

The five accessible targets are exact types, not mirrors: (u32,SpringKind), Dof, (u32,u64,u64), (u32,f64), (u32,u64). Their source locations match the evidence/geometry fields, and retained_api reexports the nominal SpringKind and Dof. size_of/align_of of these exact type expressions can bind these five facts under the later pinned build. A node byte delta supplies **no** private node alignment, regardless of pointer properties or request divisibility.

## Isolation, optimizer and thread boundary

No current claim of actual node requests is warranted: no overlay or binary exists in this proposal. Its source design is sufficient to prepare the concrete overlay, with the following strict interpretation of its existing conditions.

**Optimizer:** I checked the installed1.97.1 core source documentation. GlobalAlloc explicitly permits allocation elimination/stack promotion despite explicit source allocations and allocator side effects (core/alloc/global.rs:98-117). black_box is best-effort, not a correctness guarantee (core/hint.rs:314-344); input expressions can still be optimized (:461-484). Passing a live concrete container to black_box and checking content afterward are sensible witness design measures, but are not proof that any request occurred.

The proposal does not rely solely on those hints: it requires1 and2 successful allocator calls for the two narrow windows, positive byte deltas, no additional calls through insertion11, correct contents, and the final drop delta; any optimized-away/missing request means no binding (PROPOSAL:172-178). This is an empirical rejection boundary for the fixed compiled witness, not a Rust semantic theorem that allocation counts must occur. Keep this distinction in later evidence; do not use unsafe assumptions or a forced assertion that allocations necessarily happened.

**Process-wide observer:** H K6Alloc.current()/calls() are SeqCst reads of separate process-wide atomics, not a transactional pair and not per-thread/site counters. CALLS counts successful alloc/alloc_zeroed/realloc, not deallocations. Therefore matching deltas cannot alone prove there was no unrelated activity or compensating allocation/deallocation. A retained baseline m0 does not exclude concurrent changes to that baseline.

PROPOSAL:71-75 and :94-96 expressly require source isolation beyond the equalities. For the concrete standalone example, this must mean **no concurrent users of this registered allocator for the full baseline-to-drop interval**, not merely no thread created inside the two insert statements. The standalone cargo-run example avoids a libtest worker harness; the reviewed main/hook/callback path must also contain no worker setup or other heap work. Environment thread-count values are not evidence of single-thread isolation. This is an outstanding concrete implementation/release check, not a reason to add a thread monitor or change K6Alloc. If isolation cannot be established, return no binding as the proposal already requires.

**Observable lifetimes and operands:** Concrete overlay review must verify that operands are extracted before each baseline, comparisons and empty-tracker drops allocate nothing, all post-samples occur while the same container remains live, and output/check failure formatting is outside the windows. Keep raw before/after samples and the checked-subtraction results. The after-drop call count should remain unchanged because the source-only drops create no heap objects; an extra successful call there is unexpected activity under the proposal's stop rule. The drop-current equality by itself is not a substitute for this source reasoning.

Given these constraints, I find no design-level blocker to the finite actual-request witness. A later success can support six request-byte facts for the exact source/type/toolchain/configuration tuple. It cannot support a universal allocator contract, private node alignment, future source/layout identity, or all-process bound. This review grants no runtime work and proposes no workaround.

## Readiness and remaining work

Ready only for preparing the stated three-file overlay and label/type map for its required concrete review. The proposal itself preserves the separate ROOT release boundary at :155-178. Because no implementation was supplied, I did not verify compilation, cfg propagation, actual instruction ordering/allocation survival, execution-thread isolation, sample values or cleanup behavior in a binary. Those are later checks, not silently passed here.

Preserve T1/T2 pending the separately granted actual witness and review; C1 input arrays/Strings/children; C2 finite fixture descriptors; O2 final-A1 reconciliation; and W1 staged-H/prefix and VR global caller/consumer composition. The queue portion of C1 is now source-bound conditionally, but no complete E_max, estimator, admission replay or W1 acceptance follows. The planned witness does not investigate consumer/IO private layouts.

All writes are additive under R/source_review_RV30/source07_02. No Rust, probe, build, model, solver, network, installation, reusable tool, observer/allocator change, maintained edit, Git/index mutation or delegation occurred. Read-only Git used GIT_OPTIONAL_LOCKS=0. Standard-library documentation was read locally; one-off in-memory decoding/hashing and integer calculations produced evidence only. Existing seals remain unchanged.

