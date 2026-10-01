# VR estimate context and five-field contract

Design proposal for ROOT/independent review, not an implementation grant. The
selected reference-profile ruling remains: these are conditional mathematical
premises, not current-executable attestation. Use the shared H kernel envelope;
remove the old VR kernel port when the complete adapter is ready. No numerical,
record, CLI, admission, test-roster or hosted-CI change is selected by this packet.

P = projects/chirality-piping; H = P/core/solver/performance_harness;
VR = P/validation/benchmarks/numerical_robustness. Product citations use immutable
40129a225d73860ac2a53da9a2fa73869df668f3. The complete API21 packet was seal-checked.

## 1. Minimum signature and ownership flow

Replace the insufficient estimate(Counts,Sizes) with one complete context call:

    VrEstimateContext::capture<'a>(case: &'a Case, model: &'a Model,
        source: &PrimitiveSource, counts: &Counts,
        input: &'a VrInputFacts, invocation: &'a VrInvocation)
        -> Result<VrEstimateContext<'a>, EnvelopeError>
    estimate(context: &VrEstimateContext,
        kernel_profile: &ReferenceKernelProfile,
        vr_profile: &ReferenceVRProfile) -> Result<Estimate, EnvelopeError>

These are proposed signatures, not code. The returned context owns only checked
scalar/fixed-array kernel facts and borrows Case/Model/input/invocation. It does
NOT borrow source or its children. Its lifetime is independent of the temporary
source borrow. Fields/fact constructors are private; no raw byte allowance input.
Input/constructor fact IDs must match the supplied immutable profile IDs; a
mismatch is an error, not an automatic conversion. Counts keeps its existing
storage meanings. Current Sizes::of_this_build remains
a separate public-type observation, not an implicit replacement for reference facts.

At examples/vk_scale.rs:398, after existing counts and checked K4SRC identity,
capture the context while source is live; at :399 drop source exactly as today;
at :401 call the shared estimate through that context. Required reference calls must succeed;
tests assert the Result before the unchanged ordering check. Missing/invalid data
or checked arithmetic/conversion failure emits no partial numeric estimate into
the CLI admission path. Source-backed facts are:
N,m,s,d,r,l,t,u, source construction VRModelV1, sum/max UTF-8 load IDs, original
source capacity classes and provenance, n,f,B,b,z,h,q,X. Borrow the public source
slices/getters for s/d/IDs/B; use existing Counts for f,z,h,b,q,X. Do not call
free_dofs(), layout(), encoding() or a graph helper again merely to describe them:
those methods allocate and their existing results/counts are already available.
Use source09 aggregate upper descriptors for component histograms, contribution
and queue populations, v<=min(l,n), J<=68v. Keep all cancellation/zero-load terms.
VR supports u=0 and zero prescribed values follow cases.rs:470–533; s/d never
inherit H/RF-LARGE zeros. Check model/source/count consistency and checked widths.
The already checked CLI K4SRC hash may be recorded as such; the test's expected
hash is an identity premise, not falsely labeled a newly computed hash.

Caller context borrows the actual Case/Model strings and arrays: standalone model
constructor branch; Case's embedded model; node/member/spring/omitted/load names;
rows with expected/class/optional scale; class scales; value versus outcome controls;
not-covered and optional s_full. Derive length/digit/exponent/class summaries via
existing descriptor equations, not numerical evaluation or observed capacities.
The source-backed capture must not clone Case, Model or raw input just to estimate.

## 2. Obtain input/launch history instead of guessing it

VrInputFacts has three named parts: selected-family load history, optional external
model read/parse/hash history, and expected-list input facts. It records input
identity and raw byte length, JSON shape/escape and String-construction descriptors,
typed-case construction/order aggregates, selected Case identity, and constructor
classes (parsed Case, exact embedded Model clone, parsed external Model). Actual
logical byte length never stands for a Vec/String capacity; source02/09 rules do.
Case alone cannot recover whitespace, ignored JSON fields or earlier family cases.

Recommend narrow descriptor-returning variants of the existing cases.rs loaders:
load_family_described returns the original Vec<Case> plus fixed-size family facts;
load_large_model_described returns the existing Model/hash plus fixed-size input
facts. At read_to_string/parse_case/parse_model (:222–292,:328–332), fold facts while
the existing raw String/Value and typed result are available, then drop those
owners on their original edges. No retained raw copy, new parser, per-case facts
Vec, runtime execution-tree lookup or second model parse is required. The facts
are private source-derived aggregates, not caller-provided peak numbers. A bounded
implementation must preserve the original allocation/collection classes and review
any added descriptor work; this design does not grant a zero-cost implementation.
This requires a narrowly authorized future cases.rs edit in addition to scale.rs
and vk_scale.rs; the earlier implementation fence did not itself grant that edit.
Expected-list facts may use the already bound immutable reference input identity;
its stable-content premise and later ordinary LIST initialization remain explicit.

VrInvocation records complete argument count/UTF-8 totals (including argv[0] and
arguments later overwritten), retained final case/model-path Strings, compiled
manifest/cases path construction, normal/counts-only branch and intended normal
continuation. Fold raw argument scalars during the existing Args parse (:282–310),
without making a second argv Vec. Relative file identity/cwd is supplied by the
external launch binding; do not add an uncounted current_dir allocation. Dynamic
path lengths use the existing source path/format rules, not the frozen24 lengths.

For counts-only, keep reporting a complete normal-run estimate, not just its
shorter counts process. The minimum explicit continuation descriptor uses the same
case/model-path/manifest/executable and the existing runner's canonical normal argv
shape, with any canonical numeric heap-cap spelling bounded by20 digits under the
selected64-bit profile. Take the maximum of that normal prefix and the actual
counts prefix captured from argv. This covers the existing512MiB counts / 7.5GiB
normal pairing without inventing a new CLI flag or accepting unbounded future
paths. Argument duplicates or noncanonical numeric spellings on an actual normal
CLI call are covered by that call's own captured argv facts, not silently claimed
by the canonical continuation. Later changes to case/model-path/manifest/executable
require rebinding/recomputation. The existing half-cap decision still uses its
actual numeric cap; this descriptor changes neither predicate nor threshold.
Review this finite continuation rule against the existing binary_argv contract;
it is a proposed input binding, not permission to relabel a counts-only window.

The actual CLI at :350 loads only RF-LARGE. Do not extend its accepted family domain.
At tests/scale.rs:62 use an explicit SingleCaseFamilyReferenceV1 invocation: one
named committed family file, one selected Case, an independent exact standalone
Model clone, then the same single-case count/lane/report/RCM/parity source composition.
For non-RF-LARGE this is a mathematical composition of existing functions, NOT a
claim today's vk_scale accepts that ID. Use a named reference root `/reference`,
manifest `/reference/projects/chirality-piping/validation/benchmarks/numerical_robustness`,
executable `/reference/target/release/examples/vk_scale`, normal cap8053063680,
and the actual case ID. These are explicit reference byte strings, not host paths
or new files. Its selected family is c.family, not all ten simultaneously.

The test obtains the ten family fact summaries through an equivalent described
load_all path (original Case ordering/contents, fixed ten-entry summary array), or
an independently bound immutable descriptor catalog for those exact input bytes.
The loader route is recommended because it does not silently trust stale file facts.
Keep the existing committed-storage membership rule and every193 call/assertion.
It is legitimate to borrow c.model as the input for a hypothetical standalone clone's
lengths; do not perform that clone for the estimator. The actual test process retains
load_all and its other objects, but none is represented as this reference invocation's
heap window. No test-process memory bound or actual selected128 outcome is asserted.

## 3. Exact result meanings and ordering warrant

Let C_j be the corrected moving caller-only alternatives and A_s/A_1..A_4 the
solve/four-outcome addends (comparison, record1, expected initialization, nonselected
diagnostic). Let K_all/K_128 be complete moving kernel-solve envelopes and O_all/O_128
stationary requested-byte returned uppers. K_128/O_128 use the source06/H19 shortened
S/U128,256 and V256 roster; minimum attempt/state backing remains3328/64. They
retain every selected finish/certificate owner and all applicable error-prefix
alternatives. They do not mean retained selected-result size alone.

    max    = max_j(C_j, A_s+K_all, A_i+O_all for i=1..4)
    sel128 = max_j(C_j, A_s+K_128, A_i+O_128 for i=1..4)

Keeping the same complete caller alternatives in sel128 is explicit conservative
padding, including outcome branches unreachable on a selected128 trace. It avoids
predicting later sparse/class/record behavior. No late caller owner disappears just
because the kernel finishes early. One active old buffer remains the metric rule:
caller solve addends are retained; returned kernel is stationary in all four joins.

Define M = standalone typed Model heap-children upper (not its stack header);
P_pub = lane publication-map heap-node/row upper for q (not its inline header); C_cut = actual cut-survivor upper including M once;
E_lane = retained lane encoding; T_ctrl = retained control diagnostics;
L = persistent typed expected LIST78; B0 = source06 kernel Base.

    model = M + P_pub
    fixed = B0 + C_cut + E_lane + L + T_ctrl + P_pub
    decide = max over v=256,512,1024 of complete moving R7 transient,
             excluding Kpad, current Report and caller owners

All legacy fields are byte counts; max/sel128/decide use moving as stated. model
and fixed are retained-owner uppers with no additional active-grow surcharge.
model preserves the old Model-plus-publication meaning. fixed is precisely the
comparison-interface retained baseline under existing finite Base padding, NOT
an assertion these owners stay live throughout the entire global history. Base's
prepaid/dropped source/cache scaffolding remains declared upper padding. fixed is
a diagnostic component and is NOT added again to the already composed max/sel128.
No global total is increased merely to enforce the field order.

The ordering follows from explicit component premises, to be checked in the
instantiation/implementation:

1. C_cut >= M, and B0>0 (its named3328/64 header backings already suffice), so
   fixed-model = B0+(C_cut-M)+E_lane+L+T_ctrl > 0.
2. The comparison addend is >= C_cut+E_lane+L+T_ctrl+P_pub: its diag contains
   T_ctrl and compbuild retains P_pub. O_128 >= B0+S128+U128, with positive named
   Shared/Solved requests from the selected reference profile. Therefore that
   selected comparison join is strictly greater than fixed, and sel128 is too.
3. The full finite roster contains the shortened roster and every shortened
   phase/returned alternative with nonnegative additional owners. Thus K_all>=K_128
   and O_all>=O_128; the identical caller alternatives prove sel128<=max.

These are source-owner subset arguments, not +1, forced sorting or an added max
with fixed. They remain conditional on the generic caller implementation preserving
those identities. All193 checked numeric tuples must still demonstrate the relations;
any failing premise is a real defect/missing cell, not a reason to weaken the test.
Do not set an impossible/unknown conditional path to zero or infer actual selection.

## 4. Bounded remaining instantiation, not new generic derivation

ROSTER_CONTEXT.json reads original metadata/storage only: exactly193 required cases,
12 already in fixed24 and181 outside it, spanning all ten families. Existing records
(records.rs:53–57) provide only z/h/limbs, not B/b. source09 provides raw N/m/s/d/r/l/t,
IDs and scalar metadata, not the missing body/component counts. At the actual two
call sites Counts already supplies B,b,z,h,q,X. A source-only preflight can use the
proved explicitly tagged B<=N,b<=f and other input uppers where the reused equations
are monotone; do not pretend these are measured exact counts or substitute W1 h for
SD's independent skyline. A numeric table using uppers need not equal a later tighter
actual-count result; verify dominance or use the same tagged policy for comparisons.

| Cell to commission narrowly | Required deliverable before full adapter release |
|---|---|
| KERNEL-193 | Instantiate existing generic source05/06/09 terms with actual s/d, zero prescribed VR children, all loads/IDs and exact or explicitly upper structural fields; full/selected128 solve and returned unions, B0 and R7 in both metrics. No new geometry/solver execution or kernel formula port. |
| CALLER-FAMILY10 | Bind each actual family raw/typed prefix, parser escape descriptors, selected Case/Model retention and loader-fact aggregation, plus planned/reference launch byte strings. RF-LARGE's986182-byte family is not every family's input. |
| CALLER-193 | Instantiate existing Exact/control/diagnostic/JSON/records/floor/RCM/sparse equations for all strings, outcome controls, row-specific cancellation scales, omitted springs, zeros/multiple loads/bodies and directional branches. Preserve constructor partial-failure and returned-error scopes and identify any unsupported finite-parser premise. No observed slack or numerical-outcome prediction. |
| REACHED-TYPES | Resolve the existing untouched-node spring-set reach condition in scale.rs:71–88: if nonempty, its type is BTreeSet<(u32,usize,usize)>, not the bound pair set. Prove it empty on the relevant fixed inputs or supply its specific missing request fact; do not infer layout. This packet does not assert it is reached or commission library research. Directional binary64_model returns its existing String at parity.rs:78 before later arrays; retain that branch. |
| RESULT-5-CHECK | Evaluate all193 five-field tuples, validate the three subset premises above and max/selected/returned relationships, checked widths and one-active-grow placement. Reuse/compare the existing12 overlap plus external12 normal CLI rows. Keep all storage equalities and n==193. |
| CONTEXT-TRANSLATION | Review constant-size/borrowed descriptor plumbing and any exact allocations it actually adds; bind/check the canonical counts-to-normal continuation rule. Authorize the narrow cases.rs/Args/test-call migrations separately; no silent historical record-schema change. |

No complete193 numeric table or final artifact is produced here. Missing facts must
remain named until supplied; returning MissingProof for a required193 call is truthful
but does not satisfy release, and stale/partial estimates are prohibited. ROOT can
commission only these warranted instantiations, then independent review and maintained
implementation. Ordinary CLI,213 source-domain coverage including existing refusals,
all193 successful reference calls, counts-only behavior, protected checks and external
artifact/admission/measurement obligations remain. No automatic follow-on.
