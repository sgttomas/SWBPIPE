# source_10 — H caller composition (source proposal)

TASK I21; start2026-10-01 14:06:50 UTC; boundary14:36:50 UTC.
Frozen product40129; reviewed kernel source_05/06, queue source_07 and caller
descriptor source_09; exact type/request facts on Rust1.97.1/aarch64 release.
H only. No Rust, model, solver, runner, observer or measurement is executed.
No VR global-envelope change or prelaunch admission acceptance is proposed.

Aliases: H=projects/chirality-piping/core/solver/performance_harness;
M=H/src/bin/k6_observe/main.rs; HS=H/src/k6/w1/staged.rs;
HM=H/src/k6/models.rs; K=frame_kernel/src/structural/retained;
RUN=H/runner/k6_runner.py. Runtime library refs use the pinned installed pages
in RUNTIME_SOURCE.json. Their decoded text is evidence, not executed code.

## Finite runner and owner identities

RUN:365-368 selects the original33 models across W1-T1..T4. W1 run_parameters
has repeats=5 (:393-408); early budget/first-repeat/source stop can reduce that
number, never increase it. RUN:1061-1074 supplies --model, mode, numeric limits,
optional --counts-file; first W1 pass additionally supplies --dump-published
and --w1-prefixes. These are separate processes, not simultaneous models in
one allocator. This envelope is per W1 process, not multiplied by33 or passes.
RUN:300 fixes W1_PAIR=(w1a,sparse,w1a,sparse): two distinct W1 processes per
model,66 planned W1 processes for33 models, with sparse processes separate.
W1 time budgets are540 or1740 seconds and first-repeat limit600 (RUN:288-290,
405-408); these only reduce completed repetitions, not allocation-copy counts.

Args (M:145-168) is an inline stack struct. For this runner path only the model
id, optional counts path and first-pass published-row path persist as Strings.
model_file/dump_solution/dump_pattern are absent; no list/emit/noop branch.
Numeric options are scalars. Actual configured path bytes must be supplied
before launch; no maximum machine path length or hidden process baseline is
invented. The parsed Args id and K6Model id are distinct allocations.

The model/frame caller term C_model comes from source_09's actual builders:
RF-LARGE nodes<=G_48(N), members64m, restraintsG_16(R_nodes), loadsG_16(l),
their fixed-format/cloned String children, id/source Strings; DEC053's
owned-node-map bound uses actual size_of::<Option<[f64;3]>>() in a future
authorized estimator. Frames=G_136(m) from HM:85-96's fallible borrowed collect.
M:559/574 creates one model and frame Vec before the stages; both persist.

Writing the same retained model terms explicitly (R_nodes is the number of
restrained nodes, not constrained DOFs; each label length is prelaunch data):

    C_model_RF = G_48(N)+64m+G_16(R_nodes)+G_16(l)+G_136(m)
                 +sum_node_labels F(length)+sum_member_labels length
                 +len(model.id)+len(model.source)
    C_model_DEC = max(G_48(N),N*size_of::<Option<[f64;3]>>())
                  +64m+G_16(R_nodes)+G_16(l)+G_136(m)
                  +sum_node_labels F(length)+sum_member_labels F(length)
                  +len(model.id)+len(model.source).

G/F are source_09's pinned capacity operators. No additional200-byte
K6Model heap Box or24-byte per-String Box is added: those wrappers/headers
are stack fields or already within the measured Vec element strides.

Counts/W1Counts/SizeFacts, Observer, timers, W1Limits, first_digest and loop
scalars are inline, not additional Boxes. K6Counts:45-73 contains only scalars
and inline Option<W1Counts>; W1Counts:34-57 is scalar. Neither has heap children.
M:582-646's counts-file text/selected id or computed count scratch end before
Observer construction. Optional published dump writer exists only after the
first repeat's stage sample and drops before the next stage.

Define A_saved from the exact logical last-attempt clone at M:872:

    A_saved <=832*4+3*(40B+24*(2b)) =3328+120B+144b.

At most four AttemptRecords; at most three verification summaries and refusal
lists. Each fresh summary clone is16B resolution+8B theta+16B bound, not
G_16(B). Each refusal clone copies length<=2b, not old capacity.
AttemptRecord's scalar/work fields are in its832-byte element. No Shared/
Solved/VerifyShared/report/publication/radius payload is cloned by this
saved-attempt vector. More exact counts may tighten this finite ceiling.

Repeat0 has no saved clone. Subsequent repeats carry exactly one saved clone.
Assignment at M:872 constructs new clone while old saved clone and current
outcome are live, then drops the old clone; this is after the stage sample.
It is not two persistent saved copies in every later solve. The current
outcome drops at the end of its match arm after repeat_digest (:873).
No selected result or radius accumulates across five repeats.

## Prefix list and repeated invocation lifetime

HS:292-325 creates one solve segment per attempt, at most three verification
segments and at most three preceding decision segments: S<=4+3+3=10.
HS:346-351 returns all but the last, J<=9. Future prefix limits depend on
completed call work, but storage follows finite source slots without
predicting that work or allocating a model.

Retained prefix-list backing is exact32J (actual (String,u64) stride32);
cloned label buffers request logical lengths. Labels are
solve_128/256/512/1024, verify_256/512/1024, decide_128/256/512,
all<=11 ASCII bytes. Thus PrefixList<=32J+11J<=387 bytes.

This is a source-derived count/length upper, not an allowance. Temporary
segments has G_32(S) and original formatted labels. It overlaps destination
prefix-list construction, then drops before the list returns. Construction
precedes prefix stage resets and is not charged anew to every measured solve.
The iter() temporary in M:955 retains one prefix-list across the whole loop.

M:951 borrows the one A_saved as full. Per prefix, adapter::source at:956
finishes before Observer::begin: retained SRC0 is live at reset, but dead
constructor transients are outside this prefix sample. One prefix outcome
survives through outer sample and prefix_line, then drops at iteration end.
No two prefix outcomes overlap and borrowing full makes no clone.

prefix_line/prefix_matches run after M:965-966's outer read. Their temporary
segments/formatting values do not persist into the next reset. Persistent
stdout/runtime state is carried separately, not declared zero because those
temporary values drop.

## Composition by identity, with runtime cells explicit

Let C=C_model+C_Args+C_stdout_buffer+R_live. R_live is the unresolved
source-named runtime/lock/TLS owner set below, not a guessed constant.
S_r=0 for repeat0, otherwise one A_saved. K_phi is source_06/source_09's
finite kernel roster, including SRC0 once. Outcome is the actual returned
terminal/selected union, not a second K_phi clone.

    H_source(r) = C+S_r+max(source/adapter construction phases,
                            refused-source-error formatting)
    H_solve(r) = C+S_r+max_phi(K_phi and named construction extras,
                              returned Outcome+ErrFmt)
    H_prefix(j) = C+A_saved+PrefixList
                  +max_phi(K_phi under finite prefix budget,
                            returned Outcome+ErrFmt,
                            returned Outcome+ErrString+StageEmit)

Source construction and solve are successive measured stages; do not retain
a second SRC0 when entering K_phi. Source refusal selects an edge, not a
domain deletion. Original source is consumed; Selected's PREP source clone
is already in the kernel identity union. R_live remains an explicit blocker
to a numeric prelaunch upper, not accepted unexplained padding.

All equations are requested-byte unions. For growing-realloc movement add
only the current old buffer. Distinct String/Vec copies are already real
retained owners; no blanket safety factor.

## Observer edge that changes the prefix formula

M:281-290 emits stage_begin, then resets peaks. Its Line/integer temporaries
drop before reset. stdout was already initialized by start.emit at M:558.

HS:57-63 constructs SourceError text before obs.end; HS:77-82 constructs
Refused(...) or Unresolved(...) before obs.end. M:298-305 samples the inner
peaks before constructing the stage Line. Inner source/solve repeat metrics
include the error String construction, not the later stage-line emission.

For prefixes M:965-966 re-reads after w1_solve returns. That includes
Observer::end's stage-line construction/emission with returned outcome alive.
The input error String stays alive while Line copies/escapes it: two distinct
owners. StageEmit belongs in the outer maximum. prefix_line at:967 is after
the outer read and is not added there.



## Direct input/runtime warrants and precise limits

C_Args is now source-bound for the valid UTF-8 argv produced by the fixed
runner. On Apple, std/sys/args/unix.rs:152-189 reads the OS argv pointers;
args():21-61 reserves argc OsString slots and copies each argument's bytes
into a fresh exact-length Vec. sys/args/common.rs:6-31 wraps its IntoIter.
env.rs:875-876 and ffi/os_str.rs:242-243 consume each OsString; Unix
sys/os_str/bytes.rs:95-96 passes the same Vec through String::from_utf8.
alloc/string.rs:569-573 returns String { vec } on valid UTF-8, without copying.
Thus the Args String children retain exact logical byte lengths.

    C_Args = len(model_id) + [counts_path supplied]*len(counts_path)
             + [first W1 pass]*len(published_rows_path).

The argc backing and all flag/numeric/executable-argument strings are dropped
when parse_args finishes. They are not a permanent process allowance.
The temporary argv population is15 tokens plus2 if counts-file plus3 on the
first W1 pass, hence<=20. This is a source count, not a second persistent Vec.
Actual path lengths remain required prelaunch inputs, not unknown future
heap capacities. The first-pass path is record_dir joined with run_id+".rows";
RUN:425 gives run_id from finite schedule order/model/mode. No path ceiling
is imposed. Invalid UTF-8 takes startup failure, outside a completed stage.

K6Counts is entirely scalar plus inline Option<W1Counts>
(H/src/k6/counts.rs:45-73); W1Counts is scalar
(H/src/k6/w1/counts.rs:34-57). Therefore its retained heap-child term is zero.
The separate old counts file's String and selected id are local to M:582-605
and die at the branch edge. That conclusion does not declare the separate
counts construction/global process metric bounded.

A **single1024-byte stdout buffer** is source-warranted:
stdio.rs:645 stores the outer OnceLock/ReentrantLock/RefCell/LineWriter inline
in static storage; stdout():717-721 creates LineWriter::new; linewriter.rs:88-110
chooses1024; bufwriter.rs:121-122 requests Vec::with_capacity(1024).
bufwriter.rs:270-279,452-466 writes only within spare capacity, never grows
this buffer. Cleanup can replace it with capacity0 (stdio:724-741).
The start line initializes it before model/stage execution; counting1024
during stages is valid for this buffer. It is **not the whole stdio/runtime
envelope**. Platform lock children and write/error path transients are
separate unresolved cells below.

The outer ReentrantLock is static, but its sys::Mutex child is opaque at the
selected boundary (reentrant_lock.rs:81-85,232-238,284-299). No sizeof of that
static wrapper is an extra heap allocation, and no zero-heap assumption for
the platform mutex is made. rt.rs:111-119 delegates to sys::init; the Unix path
calls standard-fd sanitization, SIGPIPE setup, stack_overflow::init and args
initialization, and names the Apple main thread (sys/pal/unix:25-51).
Only the args copy/drop path above is closed here. The remaining initializer/
TLS/backend allocation ownership has not been given a complete byte bound.

thread/current.rs:289-300 shows that a reached current() initialization creates
a Thread and retained TLS reference, while drop_current:324-329 releases it.
This is a **conditional runtime site**, not a claim that the H path necessarily
creates a Thread handle. ReentrantLock uses current_id, not current, so these
must not be conflated. Its reachability or child allocation bounds need the
specific target runtime path, rather than an observed baseline.

## Finite format grammars and outer stage-line construction

Use F(L)=max(8,2L) for a nonempty formatted String, as source_09's pinned
String/growth derivation; exact cloned strings use L. SourceError and
UnresolvedReason contain fixed enums and bounded integer fields, not model
Vecs/Strings. Non-pretty derived Debug gives the finite token schemas.
GRAMMAR.json records each source variant and literal/digit calculation:
u32<=10 decimal bytes, usize/u64<=20, u128<=39, Dof Debug<=39 bytes,
MemberProperty name<=15; SourceError Debug<=75 bytes.
Thus the source-stage error destination is <=F(75)=150 bytes; it is created
after a refused PrimitiveSource constructor has dropped its partial fields.

WideError has maximum32 Debug bytes (Accumulator(AccumulatorOverflow));
CertificateIssue names max19. UnresolvedReason Debug<=88, including the
new PublicationCertificate { index: Some(usize), issue: ... } case.
The full HS:80 Unresolved(...) error has D<=100, destination<=F(100)=200.
These grammar byte calculations are source arithmetic, not sampled output;
independent review of the pinned derived-format schema remains appropriate.

Refusal's two nontrivial format leaves are kept explicit:

    D_Refusal <= max(71+6*D64_Debug,
                     49+D_GeometryError_Debug,
                     67, 51, 9).
    D_ErrorSolve <= max(9+D_Refusal,100).

The constants count actual variant/field/list punctuation plus bounded
u32/usize/SumError tokens. The six f64 rigid parameters and the reachable
StructuralError formatting are not silently truncated. The known geometry
entry sites include InvalidInput("rigid geometry"), Range("relative
coordinates"), Range("nonfinite characteristic length") at rigid_body.rs:42,
50,56; a complete bound for propagated reachable error leaves remains a
named cell. D64_Debug and any floating Debug heap scratch are also still
unbound here. The 240-character truncation in bin/w1.rs occurs only in
post-sample outcome/prefix output; it does **not** bound HS:79's in-stage
Refused(...) formatting.

M:306-317's outer prefix stage Line has11 fields. Using exact ASCII keys,
repeat<=5 (one decimal digit), prefix stage label<=11, boolean<=5,
elapsed_ns<=39 digits, four heap counters plus alloc_calls<=20 digits each:

    L_stage_no_error <=315 bytes, including closing brace/newline.
    L_stage_with_error(D) <=313+6D bytes.

M:71-84 escapes a char using at most6 output bytes per input UTF-8 byte.
This deliberately covers the control-char path; it does not assume error
text is unescaped. Its one temporary "\\u{:04x}" String has length6,
hence capacity<=F(6)=12. Every Line::n temporary is an unsigned integer
String, at most39 digits; retained<=F(39)=78. The pinned ordinary unsigned
ToString path uses a stack digit array and exact str clone
(alloc/string.rs:2984-3004); the generic small-code path starts String::new
(:2917-2930), also covered by F(39). No signed/float Line::f field appears
in this particular stage line. Temporary number and control strings occur
sequentially, so use78 as their maximum, not their sum.

For an input error text of logical length D with retained cap Eerr<=F(D):

    StageEmit_R(D) <= F(L_stage(D)) + 78 + T_stdio_emit
    outer_end_R <= Outcome + Eerr + StageEmit_R(D)
    outer_end_M <= outer_end_R
                   + max(F(L_stage(D))/2,39,6,O_stdio_emit).

Only the active Line or temporary grows; already-created ErrString does not
grow here. For absent error Eerr=0 and L_stage=315. T_stdio_emit is the exact
missing heap-transient bound for the pinned LineWriterShim/StdoutRaw write
and error paths, not a positive invented allowance. It cannot be set to zero
merely from the fixed1024 buffer. Error-format construction is an earlier
alternative: Outcome+F(D)+T_Debug(D), with only its active old buffer added
for movement, bounded by max(F(D)/2,O_Debug(D)) when the formatter helper's
own growth is not yet bound. O_stdio_emit and O_Debug are the corresponding
unresolved active-old-request premises, not zero assumptions. Likewise any
untraced runtime-owner growth must supply O_runtime in the whole-stage move
maximum. Nonfloating primitive/derived fields write into the supplied
String; any remaining floating formatting scratch must be warranted at its
actual leaf, not copied from a tracker allocation.

The source-bounded numeric non-error Line term is630+78=708 requested bytes,
with active Line-old upper315. This is one declared branch of the caller
formula, not the complete prefix or runtime bound.

## Remaining cells and prelaunch consequence

| Cell | Exact missing premise / consequence |
|---|---|
| H-R0 target startup/live TLS | Complete registered-allocator owner set for std::rt -> selected Unix/macOS init, stack-overflow/thread-local setup and any reached Thread-handle initialization; actual private child requests/lifetimes. No value is assigned from another process or an observed612 baseline. |
| H-R1 stdout platform lock | Selected sys::Mutex's child allocation/lifetime under the static ReentrantLock, plus current_id/TLS initialization behavior. Static wrapper storage is excluded; the1024 byte buffer is included separately. |
| H-R2 emission transients | Direct LineWriterShim::write_all/flush -> StdoutRaw/macOS raw writer/error path heap behavior. Fixed buffer capacity alone does not prove no additional allocation. |
| H-F0 floating/geometry Debug | D64_Debug and any heap scratch for six rigid parameters; complete reachable StructuralError leaf/message grammar. These affect the actual in-stage Refused String and its escaped outer prefix copy. No generic formatter audit or guessed bound was substituted. |
| H-I0 concrete argv/model identity | Exact configured counts_path/record_dir/model id bytes and pass flag before launch; preserve33 fixed models, W1-only paths and repeats<=5. Paths are inputs, not measured capacities. |
| H-C0 checked implementation / review | Actual accessible sizeof expressions and checked products/additions; independent verification of grammar and owner union; final A1 source reconciliation. No estimator/API/record/admission edit occurred. |

R_live in the composition means an as-yet-unproved source upper across H-R0/
H-R1's live owners over the finite windows. It is **not an estimator input a
caller may choose arbitrarily**, and this packet does not assert completeness
of that runtime owner set. If later initialization changes the set, it must be
included before claiming a constant bound. T_stdio_emit/H-F0 cover their named
extra transient sites separately. Until these premises close, the formula is
conditional and cannot justify numeric prelaunch admission.

A same-process cut at Observer::begin after reset can be written conditionally:
current-at-cut plus newly allocated identities after the cut, retaining
preexisting allocations conservatively and charging the active old grow.
For solve/prefix the cut already includes SRC0; do not add it again. Such a
cut still needs a warranted bound on later untraced runtime/format allocations.
It is a diagnostic conditional relation, not a replacement for the requested
prelaunch estimate or the existing admission decision.

All first-repeat dump/attempt/digest/old-new-save and prefix_line/segment
comparison allocations remain outside these chosen sample edges; their
persistent consequences are not presumed zero. Nothing here substitutes a
whole-process H peak or VR global envelope for H's original stage/prefix
metrics. Return the exact gaps rather than start another runtime/IO programme.
