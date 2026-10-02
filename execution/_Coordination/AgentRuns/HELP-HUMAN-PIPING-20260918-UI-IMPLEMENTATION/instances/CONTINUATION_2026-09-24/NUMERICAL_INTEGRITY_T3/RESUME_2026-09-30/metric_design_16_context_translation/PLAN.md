# CONTEXT-TRANSLATION preparation

Proposal for the next bounded implementation grant. Entry/count/loader/test source
is pinned to K6C8b6b4db5aa5ee8a3a3db149ff36b9418910bc9a3 after A1 integration;
this packet does not inspect or change I21's active shared-kernel module. API13 is
accepted; join15 remains independently reviewing. Historical reference inputs and
storage hashes remain their immutable origins, not this newer worktree's hashes.

P=projects/chirality-piping; H=P/core/solver/performance_harness;
VR=P/validation/benchmarks/numerical_robustness. Signatures below are specifications,
not code. No source algorithms or tests were executed.

## Data representation and incremental heap

Use inline scalar/enum/fixed-array facts and borrowed views. Kernel counts and
phase/result arrays are owned inline; Case/Model/Args and existing profile constants
are borrowed. No Box, Vec, owned String, retained raw Value/text or dynamic closure
is needed by these descriptor/result objects. Returning/moving an inline tuple or
array does not by itself request heap. This is a design invariant to verify in the
actual implementation, not a finding that unpublished code is cost-free. Added
stack/CPU work is not automatically zero RSS/time or waived measurement.

| Translation choice | Incremental registered heap / accounting rule |
|---|---|
| Counts/scalar summaries, [FamilyFacts;10], [LaunchSpec;2], fixed phase arrays, borrowed strings/profiles | No new heap owner if implemented literally without allocating helper calls; code review must establish this |
| New Vec of per-case facts, Vec<(Case,Facts)>, boxed callback, cloned model/Case/raw input, second argv | Avoid; these introduce real new buffers/children, alter original strides/lifetimes and require new source-bound terms |
| New hash-to-hex String, formatted identity/path/error, Vec-backed decimal/JSON traversal | Avoid for normal descriptor capture; if introduced, identify actual F/E/G request and live overlap, then update the relevant window before reliance |
| New descriptor error emission | Prefer scalar error enum/static code until the existing boundary; an owned error String/JSON copy/Line remains a real allocation with its template length and output overlap. No blanket zero-error claim |
| Existing raw/typed loader owners kept longer for facts | Do not extend them; if the actual implementation does, preserve their capacity/children in the added interval, even if the field values are unchanged |

Use checked integer folds over existing slices/borrowed Value iterators; do not
collect tokens, histograms or paths. A borrowed recursive JSON-shape walk uses call
stack, not a new heap stack; avoid Vec-based traversal. Preserve input depth/valid
parser premises. Decimal descriptors inspect borrowed lexical bytes/integers, never
run Exact values/predicates. External identity/build/launch qualification stays
external: no attestation, sidecar parser or new runtime domain guard. A family
name alone is not content authentication; private same-load provenance plus the
existing external exact-input binding supplies the claim. Reuse existing hashes;
do not add an encoder/hash String solely to manufacture an estimator identity.

## VR: exact hooks and preserved lifetimes

1. VR/src/cases.rs: factor parse_case's existing body into an internal described
   path which still returns Case, updating a borrowed scalar FamilyCapture. While
   the original line/raw Value and typed Case coexist, fold actual raw JSON shape,
   UTF-8/escape/number descriptors and typed constructor facts. parse_model similarly
   exposes facts from its existing Value/result without a second parse or clone.
   Keep original field/Case order, iterator item type Case, and constructor classes.
   Do NOT collect (Case,Facts) tuples and then map them back; that changes the outer
   allocation. Existing parse/load APIs may remain wrappers discarding inline facts.
2. load_file/load_family_described returns (Vec<Case>,FamilyInputFacts). Preserve
   read_to_string -> lines/filter/map -> collect, source order and original drops.
   The closure captures &mut scalar accumulator; it is not boxed. The rolling
   previous-case sum/current raw+typed maximum gives the accepted family union.
   load_all_described keeps the original flat_map/Case ordering and fills a fixed
   ten-slot summary array by FAMILY_FILES index; no per-case metadata Vec/map.
3. load_large_model_described returns existing Model/hash plus inline facts. Keep
   raw text, Value and typed Model through the existing hash tuple operand, then
   drop them on the original edges. The existing hash String is moved, not cloned.
   Embedded selection still makes exactly its existing standalone Model clone.
4. VR/examples/vk_scale.rs Args: route every existing iterator next through a tiny
   borrowing/scalar observation step. Include argv[0], value tokens and overwritten
   values in argc/total bytes before drop/move. Keep final owned case/path strings
   as today. No second env::args collection, second argv Vec or path rendering.
   Borrow env!(CARGO_MANIFEST_DIR) and final argument strings; compute join lengths
   arithmetically. Do not allocate current_dir. Preserve accepted flags/errors.
5. At V398 capture API13 context from Case, standalone Model, source and Counts;
   own only source scalars, borrow the surviving inputs, and preserve V399 drop.
   Reuse Counts.f/z/h/b/q/X and source slices/B/IDs; no additional free_dofs/layout/
   encoding/graph call. At V401 compose the shared kernel and VR caller once.
   Emit/check the same result, with checked u64 conversion, before the existing
   half-cap predicate. No stale copied kernel loop or subtotal fallback remains.
6. Preserve actual RF-LARGE-only load_family selection. Counts-only uses the
   accepted same-input canonical normal continuation (cap spelling <=20 digits),
   maxed with its actual counts prefix. Different later paths/context require
   rebinding. The test uses SingleCaseFamilyReferenceV1, not the heap of load_all.
   Expected LIST initialization is not moved earlier; its fixed reference facts
   are borrowed from the accepted profile, while actual LIST/raw/scratch lifetimes
   remain unchanged. Named profiles do not certify the running executable.

## H: one composed result through every entry

H needs its own model/launch capture; VR's context is not an H default.
HModelFacts borrows actual id/source/node/member-label strings and carries the
trusted constructor recipe and raw restraint-row/bit populations. Sum masks with
borrowed iteration, not a new restrained_dofs Vec. Do not use family/id alone as
proof of construction capacity or inspect observed capacities as an allowance.

- H/src/k6/models.rs: have the actual model dispatch/build/conversion branches
  return or expose inline construction facts (RF/ceiling builder, from_fixture
  including grids/DEC). Reuse their existing source09/10 capacity contracts and
  distinguish exact clones, formatted labels and owned-map backing. Keep public
  model(id) as a wrapper if needed. No second model build is needed at main.
- H/src/k6/canonical.rs and main::load_model: add an inline origin/facts return
  for the existing canonical parse. Its successful node/member/load counts equal
  the with_capacity reservations; restraints are fresh pushes. Retained parsed
  model heap is E48(N)+E64(m)+G16(restraint_rows)+E16(l)+all actual owned id/source/
  node/member-label bytes, excluding stack headers. Existing temporary text,
  counts/section tokens, line-token Vec and roundtrip serialization drop before
  H measured stages; do not silently retain them or add their whole prefix to H's
  staged window. This formula needs source-site checking in the granted adapter
  review; no new library or observed-capacity premise is proposed.
- Preserve the already-created frames from main:576; include their existing
  fallible-collect retained envelope in H caller facts. Never call model.frames
  again merely to estimate. Preserve every owned Args String, including model_file,
  counts_file, dump_solution/dump_pattern/dump_published even when a flag is unused
  by a particular W1 branch. Arg parsing temporaries are not retained H-stage
  owners. Inline launch views borrow those strings and carry repeat/prefix/pass
  semantics without materializing a path or launch Vec.

H/src/k6/w1/counts.rs compute_described is the capture seam. Keep its existing
source, node_graph, free Vec, structural profile, encoding and layout calls once.
Capture checked source scalars while source exists, before returning; do not return
source references. Existing compute can discard the inline capture for consumers
that only need counts. Preserve the current refusal result/error and raw-input
facts separately from a successful kernel descriptor; zero failed counts are not
valid successful-solve facts.

For main's --counts-file branch (:583–605), no source is present. Build context
from the existing loaded Model/construction facts, parsed W1Counts, actual launch
and existing model-FNV validation. Store scalar FNV/origin/line identity; borrow
model.id, not the soon-dropped parsed id or raw file text. Do not rebuild source,
encoding or graph to estimate. Existing FNV verifies the model link, NOT arbitrary
z/h truth: retain the supplied-count provenance/external record binding, plus
checked scalar consistency. It is not a new runtime attestation. No raw counts
text is kept through W1 stages solely for provenance.

Compute one HComposedEstimate whenever counts.w1 is Some, even in a counts-only
or other-mode record that emits W1 fields. Keep it inline and thread it through:

| Current site | Required migration |
|---|---|
| main::counts_line (:343–435; call sites :603,:636) | Add Option<&HComposedEstimate>; serialize its complete moving fields and shared/state/solve/verify/pass arrays, never call the old two-argument estimate |
| H/src/k6/counts.rs admission_estimate_bytes (:308–324) | Accept explicit optional composed W1 result and return a checked Result; W1a requires it and returns its same max. Four K6 modes keep their existing formulas. Delete W1SizeFacts/old-estimate fallback branch |
| main backstop :659 | Use the SAME composed result/context used by counts_line; preserve cap/2, override cap restrictions, named refusals and all non-W1 rules |
| H/src/k6/w1/counts.rs estimate :411 | Replace the stale formula with H context -> shared kernel -> H-window composition. No old port for missing context |
| tests/k6b_w1.rs :635–729 | Pass explicit H reference ModelRecipe/launch-pair facts into estimate and independent hand equations; recompute all33 estimate rows without building huge models merely for metadata. Preserve exact non-estimate counts/identity/storage assertions and all33 cardinality |

A typed W1 result error must not become u128::MAX as a fake computed envelope,
zero, or old estimate. Existing missing-W1-count validation remains. Preserve
source-refusal semantics using the existing source/error window contract; do not
reinterpret failed-source W1Counts as a valid full kernel descriptor.

H library results keep fixed arrays for all legacy fields; matching shared module
exports are consumed only after I21's return/review. Never place estimator capture
inside Observer begin/end, stage resets, solves or prefix iteration. In particular
preserve the inner post-Observer sample versus outer prefix sample before
prefix_line, and the next adapter outside the new w1_solve stage.

## H launch/test contract requiring an explicit binding

The33 committed counts are reference estimates for the accepted maximum of both
original planned H passes, not the shorter counts-only argv. Materialize those
named launch facts as compiled borrowed/fixed data alongside the reference profile,
not a runtime execution-tree/sidecar parser. Tests use that explicit pair and
source-backed model recipe; actual CLI uses its actual launch facts. Counts_line
and backstop must agree on their selected context, including when counts came
from a file. Retaining only Counts/Sizes cannot supply this choice.

**H-LAUNCH-REUSE remains a concrete owner decision:** how a counts-only invocation
without future counts-file/dump paths declares the later H launch envelope while
preserving the33 reference rows and ordinary arbitrary path spellings. The accepted
original two-pass plan closes the governed33 run, but does not bound unspecified
future path lengths. Recommend explicit library contexts: OriginalK6bPair for
reference/count-artifact regeneration, ActualNormal for a real W1 run, and an
explicit same-input CountsContinuation for ad-hoc counts. A policy taking max of
named reference pair and actual invocation is possible, but is NOT silently
selected here. No unbounded universal path allowance, new CLI flag, rejection of
existing model IDs or hidden runtime sidecar machinery resolves it automatically.
ROOT must choose the actual counts CLI binding before that entry migration ships.

**H-ORIGIN/FAILURE-COVERAGE:** confirm the above canonical/extra-model constructor
recipes and the existing source-refusal result mapping before generic H entry
release. The33 successful frozen rows do not themselves prove coverage of every
accepted canonical model/extra grid path. Do not turn this into a whitelist or
stale fallback. This is a bounded caller-contract check, not a new kernel/library
proof programme. Preserve all supported ordinary CLI paths while closing it.

## Next grant, integration order and checks

1. Receive/review I21's shared-kernel public interface; retain the accepted source
   equations/reference meanings. Resolve H-LAUNCH-REUSE and source-refusal mapping.
2. Grant narrow descriptor/profile and described-loader/model/Args edits above;
   include H models.rs/canonical.rs and VR cases.rs explicitly. Review field,
   closure, collection specialization and destructor edges for real new owners.
3. Migrate BOTH VR calls (tests/scale.rs62 and vk_scale.rs401) and ALL H sites in
   the table as one integration. Update only authorized estimate fields in counts
   plus named compile-time reference launch facts; preserve original case/files/order.
4. Tests retain VR193 membership, storage equalities and strict RESULT5 ordering;
   compare described/ordinary loads' Case order/content, raw whitespace/escape facts,
   exact original input identities, and current public-type/allocator checks.
   Exercise normal/counts canonical argv, duplicate/overwritten/long valid arguments,
   arithmetic overflow and source-drop lifetime at the API boundary. No graph replay
   just for descriptors and no ignored/early-success tests.
5. H keeps all33 committed-count recomputations and independent hand/subterm checks,
   source/adapter identities, debug subprocess success/parity/prefix/dump behavior,
   counts-file identity failures and named half-cap refusal. Existing phase-dominance
   expectations must be reconciled with the accepted corrected equations by an
   explicit reviewed test change, never merely loosened after failure. Keep all
   four K6 mode checks, runner tests, separate k6_alloc and --all-targets scope.
6. Check matched source profile/capacity/request assumptions and actual descriptor
   costs before final artifact qualification/replay. Historical numeric tables are
   not silently revised to include new implementation allocations. Any added heap
   either gets an explicit bounded owner in its real window or is removed/reworked.

No maintained code, test, record or CLI behavior changed. No runtime, compiler,
probe, parser/model/graph/count/solver/encoder execution, new host tool, Git/index
mutation or delegation occurred. No automatic follow-on; return for ROOT review.
