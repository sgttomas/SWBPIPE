# layout08 concrete source isolation and review boundary

Source-only preparation, TASK I21, native parent /root/t3_recovery_manager.
Start13:15:00 UTC; deadline13:35:00 UTC. No compiler/probe or source execution.
The complete diff is OVERLAY.diff, exactly two modified original files and
one new example; target type definitions/numerical code are unchanged.

## Call path and source window

The standalone example's main computes five constant size/align pairs in a
fixed stack array, then calls the appended fixed-purpose hook with the
unchanged allocator's current/calls function pointers. No CLI/file/env parsing,
logger, worker pool, thread creation, test harness, model, solver or framework
entrypoint is called. The hook uses three direct concrete blocks, not a generic
probe or macro framework. The result is stack arrays and booleans, not a heap
report. H's unchanged library/dependencies do not receive a numerical call.

Each block's twelve exact operands are built on the stack first. The tracker
constructor stores scalars and two empty Vecs; no offer occurs. The first
operand is extracted before baseline0. The next ten are moved individually
into the existing leaf, then the twelfth operand is extracted before baseline2.
Array IntoIter owns its stack array; exactly1+10+1 next calls consume exactly12
elements. unwrap's empty branch is unreachable under this fixed source count;
no error formatting is executed in the trial. Fixed insertion keys are
distinct and comparison uses scalar/derived enum/tuple ordering.

Within baseline0 through after-drop:
- current/calls callbacks only perform separate SeqCst atomic loads;
- the first insert allocates one actual LeafNode;
- ten fitting inserts allocate nothing;
- the twelfth allocates one LeafNode plus one InternalNode;
- black_box receives a live reference, and the container remains used;
- iteration checks all keys/values after post-insert sampling, without
  assertions, formatting or new heap data;
- drop releases the three actual nodes; empty tracker children release none;
- after-drop current and calls are sampled before any output.

The three trials run sequentially. No diagnostic text is produced between
them. The first println is after the hook has returned all three results.
Content failure is a stored bool; the caller performs checked subtraction/
multiplication and aggregates validity before output. A failed check prints
NO_BINDING and returns exit1 only after all trial windows have ended.

K6Alloc is unchanged and process-global. current/calls are distinct reads,
not a transactional snapshot or a per-site counter. Matching values cannot
prove the absence of concurrent allocation/deallocation or compensation.
The release condition is **no concurrent users of this registered allocator
for every baseline-to-drop interval**. The reviewed standalone source path
creates none. A later run must also preserve that execution boundary; if
external/runtime behavior prevents establishing it, no binding is allowed.
Environment thread-count variables and counters alone are not substitutes.
No monitor, allocator replacement or thread-detection framework is proposed.

Allocation elimination/stack promotion is permitted by Rust. black_box and
content checks are best effort; they do not force allocator calls as a semantic
theorem. Missing, extra or optimized-away requests, changed configuration,
failed content or cleanup, overflow, unexpected activity, tool failure or
offline dependency failure means no binding. No unsafe assumption is used.

## Exact sample interpretation

Rows0/1/2 are geometry map, tracker map, holding set. Columns0/1/2/3/4 are
baseline, after first, before twelfth (after eleventh), after twelfth, after drop.

Required per row:
- calls1-calls0=1; L=bytes1-bytes0>0.
- calls2=calls1 and bytes2=bytes1.
- calls3-calls2=2; I=(bytes3-bytes2)-L>0, checked.
- bytes3-bytes0=2L+I, checked.
- calls4=calls3 and bytes4=bytes0.
- twelve distinct expected contents pass; normal non-test release cfg.
These reject invalid witness behavior; they do not alone prove isolation.
Printed REQUEST lines are candidates subject to source/isolation/build
review; RESULT COUNTERS_AND_CONTENT_PASS is deliberately not E_max acceptance.

The holding specialization uses BTreeSet, hence real private SetValZST.
No mirror struct/unit-map substitute is instantiated. current measures bytes,
not alignment. If reviewed runtime later supplies L and I for each exact
specialization, source_02's requested-byte node formula can use max(L,I)
without private alignment. Node alignment stays UNBOUND.

## File, dependency and release controls

Archive: <I21_LAYOUT08_SCRATCH>/archive. source.tar is a read-only Git archive
of frozen40129's nine existing H dependency packages; no Git checkout/index
was made. All227 original files are inventoried. Exactly adaptive.rs appended
hook and structural.rs retained_api export differ; exactly one new H example
exists. Eighteen Cargo manifest/lock files and the existing allocator are
byte-identical; all20 manifest path-dependency edges resolve inside the archive.
SOURCE_BINDING records their hashes. No cfg/features/types or maintained
source are edited.

The sibling future target is <T3_HOST_ROOT>/k6c-layout08-target and is empty.
An initial misunderstood target directory <K6C_WT>/k6c-layout08-target was
created empty, then left empty when the manager clarified the host-root
convention. No deletion/archive occurred. Both empty directory writes are
reported in the inventory; the prospective command uses only the sibling.
Logs are placed at scratch root, never in immutable source directories.

RUNTIME_COMMAND.txt freezes the exact prospective payload and cwd/PTY/log/
target settings. It is not run or executable authority. ROOT and RV30 must
review the concrete overlay, source isolation, target/lock/toolchain/config
binding and manager containment check before any later slot. Existing M5
guard/operator supervision remains mandatory and unchanged. G1 has numerical
priority; this prep consumed no host runtime slot.

T1/T2 remain unmeasured; C1 source input capacities, C2 finite descriptors,
O2 final-A1 and W1 H/VR consumer composition remain open. No E_max, admission,
fixture change, product merge, standard-library edit, host change or further
continuation follows from this source packet.

