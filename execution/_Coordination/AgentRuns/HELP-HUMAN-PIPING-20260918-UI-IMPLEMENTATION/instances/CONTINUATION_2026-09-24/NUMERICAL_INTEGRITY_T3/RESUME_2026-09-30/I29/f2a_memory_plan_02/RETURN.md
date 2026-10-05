# I29 — F2a memory-admission checkpoint-zero plan

**Recommend M1: two-stage, source-derived admission for one grouped W1 batch, with a private publication draft
and explicit qualified-caller completion window.** Keep the existing numerical schedule and actual cache
semantics. Share the pure kernel resource algebra from its kernel owner; add product construction, aggregation
and caller terms only after independent derivation. Do not import H's allocator or single-case allowance as
product enforcement.

This is a revisable design plan, not a completed memory proof, implementation grant, adopted guard or
qualification. Basis: NUM 252e97404ba312b46fd85aef415729ba67cf652a; merged main
49034a940f3f8cd3f3da4d4cbc839943b808063d. W1_RESOURCE_POLICY_V1 selects 20,000,000,000 LME/case and
60,000,000,000 LME/actual invocation. Its 3.75-GiB technical target remains provisional. RV40-N1 supplies the
design obligation. The [ownership/interface table](OWNERSHIP_INTERFACES.md) gives exact current source anchors
and proposed changes; bracketed identifiers below refer to it.

## 1. Claim and entry boundaries

The proposed memory claim is a pair of source-derived upper expressions for requested-live bytes and
old-plus-new moving-reallocation bytes over a **named admitted-W1 request-owned interval**. It starts before
W1 census/source/count allocations and ends after the qualified caller's W1-related publication work. It
includes already resident ordinary/captured/caller objects throughout their actual overlap. It is not a bound
on earlier ordinary solve peaks, unrelated application state, allocator overhead, stack/RSS/footprint,
concurrent requests, or arbitrary later consumers of the returned result.

Qualify two finite caller contracts separately, not through a caller-supplied number presented as proof:

- **Direct captured Value call:** PP's raw capture, normalized typed model,
  basis states, ordinary case results, W1 preparation/solve/publication and
  returned MechanicsEnvelope through transfer to its caller [O1–O4,O12].
- **Headless captured Value call:** additionally retain solve_payload,
  actual_invocation JSON and RunnerRequest clones while PP runs, then mechanics,
  runner data, JSON/canonicalization/hash temporaries and the export document
  through PreviewRunnerOutput assembly [O14]. A PP-only expression is not this
  caller's expression. Transfer boundaries must name ownership passed onward.
- The legacy typed route has capture=None [O1,O14]. It remains ordinary/no W1.
  This plan adds neither typed reserialization nor a hypothetical custody API.

At headless entry, construct the finite caller census from borrowed request/payload before any **new W1-specific**
clone or planning buffer. Account existing ordinary clones from their actual retained objects or a qualified
clone-request bound; do not retroactively call those existing allocations guarded. Supply a bounded
caller-completion recipe before PP starts W1. An unqualified recipe declines W1 while ordinary processing
continues. A direct caller's untracked external copies are outside its named contract, not silently covered by
the direct profile.

If the resident ordinary base already exceeds a later adopted M, decline W1; do not block, shrink, rerun or
discard the ordinary result to satisfy M. Consequently the rule cannot promise that **every** invocation,
including its ordinary-only refusal path, fits M. A whole-application or guaranteed OOM-recovery promise would
require another architecture/owner decision and is not proposed.

## 2. Feasible mechanisms and recommendation

| Mechanism | Sharing and enforcement | Benefit / concrete cost |
|---|---|---|
| **M1 — admitted grouped batch, recommended** | Census/source/count preflight, then a complete aggregate phase reservation; opaque prepared groups feed the existing case schedule; one invocation meter; bounded draft/commit. Numeric work starts only after its entire reachable allocation envelope fits. | Preserves solve_cases ordering/reuse and existing combination-cache rules. Smaller ownership change than an incremental cache session. It can decline all W1 candidates when aggregate retained outputs do not fit even though individual cases would fit. |
| M2 — admitted incremental session | A new invocation-owned group/cache owner admits a case or combination at a time, reserves active plus live-returned storage, and releases groups/operands at a precomputed last-use point. | Potentially smaller peaks, but needs new lifetime APIs, combination scheduling and accounting proofs. Work order/rebuilds can change budget outcomes; publication must still be transactional. Independent numerical/order/work review is wider. |

M1's aggregate refusal is a **proposed ROOT design choice**, not existing behavior. If required availability
cannot be achieved under its derived expression, compare M2 or a revised allowance using the actual
composition; do not select a larger number merely from H headroom. No numeric alternative to 3.75 GiB is
justified yet. Existing 6-GiB dense/observation decisions and supported-machine policy remain with their
owners. A per-request allowance does not authorize concurrent requests to consume that allowance each as a
machine-memory guarantee.

- PROPOSAL: Select M1 as the basis for bounded independent design derivation
  - Evidence: W1_RESOURCE_POLICY_V1 “Memory remains a design obligation”; RV40-N1; [O1–O16].
  - Change: Derive and independently review the two-stage admission and finite caller windows below; require qualified preparation and execution permits, complete failure bounds and W1-only transactional publication before implementation reliance.
  - Why: Preserves current grouped work/cache semantics while giving every new allocation an earlier admission boundary and an explicit owner.
  - Risk: Aggregate reservations may reduce W1 availability; support groups, prescriptions, combinations and real caller copies invalidate direct transfer of H's formula. A proof may show that the provisional allowance is insufficient.
  - Status: PROPOSED

## 3. Proposed ownership and admission sequence

**S0 — ordinary work and routing facts.** Preserve the current ordinary/exact-block case pass, including W2.
At the existing boundary before results are consumed (PP lib.rs:2528 onward), all basis states and
LoadCaseSolve values are resident. Add only fixed route/basis/status fields to each case result while that
pass runs. Do not eagerly retain force/graph/source copies. Keep direct references or stable indices to
normalized model/basis data. An adapter may rebuild its needed nodal source later only under the preparation
reservation. The current force values are not a replacement for the term/source ledger [O2,O3,O5].

Determine invocation-wide coexistence before W1 allocations: any exact-block selection bypasses W1 and
preserves its existing identity/diagnostics/bytes. A nonlinear-support invocation remains ordinary with no W1
refusal of its result. Missing captured custody, an unsupported family or an ineligible ordinary failure
follows D1's existing selection rules; no memory policy expands W1a into W1b/c. I30's separate
input/publication plan must supply the precise supported adapter recipe; no unfinished peer proposal is
accepted by this plan.

**S1 — allocation-bounded census.** A borrowed visitor counts actual scalar populations, UTF-8/string bytes
and existing Vec/map capacities, including the caller/capture, ordinary results and failure diagnostics. Its
own fixed workspace, overflow checks and traversal stack need a declared finite bound. If that bound cannot be
established, return a fixed refusal capsule without building a source. A bounded visitor for existing values
is enough; no global parser rewrite. Do not call source.encoding(), layout(), free_dofs(), body_nodes(), an
adapter, graph builder or serialization merely to obtain a supposedly allocation-free count.

Before source validation, use raw populations including duplicate/invalid constraints, support children and
IDs. Do not infer free DOFs as 6n-r from an unvalidated list. With n nodes, m straight members, s axis springs
and d directional springs, current construction gives raw upper contributions 78m+s+6d and symmetric raw
positions at most 144m+s+9d; checked N = 6n and pattern≤N² are additional bounds [O6,O7]. These are
cardinality inputs, **not byte formulas or proofs**. Use N as the prevalidation free-DOF upper; bodies≤n and
blocks≤free DOFs are only population bounds. Any sum/product/cast overflow is a named W1 refusal.

**S2 — admit the preparation itself.** Derive a raw-source/preparation requested/moving envelope
without the final RCM profile, covering every constructor, validation, geometry, pattern/RCM,
ledger/layout/encoding and failure allocation in [O5–O8]. Include raw children before deduplication,
old/new overlap and previously prepared sources/groups. Reserve that whole stage against resident
ordinary/caller data **before** executing it.

Expose an opaque prepared-group path around the kernel's existing preparation, returning exact scalar
pattern/profile/body/block facts tied to the same validated source and the owned prepared object [A2]. RCM's
skyline count is acquired without allocating the numeric skyline; its graph workspace still costs memory. Use
checked accumulation for profile sums and representational widths. Reuse this owned preparation in the solve;
do not count once, drop it and silently rebuild a second unaccounted graph. A failing source/geometry path
returns its own bounded refusal, not a fabricated zero-sized successful descriptor. Preparation findings remain deferred metadata: preserve the scheduled
case order and existing Invocation-budget/error precedence, rather than publishing a geometric
refusal for a case that the numerical schedule would never reach.

**S3 — admit the complete batch.** Using exact prepared facts, derive all reachable phase expressions for the
fixed ordered candidates, allowed combinations and caller publication schedule. Keep the chosen model case/combination order; grouping does not
authorize reordering numerical work under the 60B threshold. Do not predict Selected128
from the model name, past outcomes or the 20B limit: include all reachable precision/failure/certificate paths
unless a separately proved reachability restriction is adopted. The already allocated S2 state remains
resident in S3.

The composition is conceptually, for each phase j and metric k:
E(j,k) = resident ordinary/caller + owned W1 persistent allocations
         + new active/transient allocations + planned publication/completion.
Accept only when every checked E(j,requested) and E(j,moving)≤adopted M. This equation declares accounting
categories; it is **not a completed derivation**. Count each actual shared Arc payload once, but all owner
headers/Vec backings and deep-cloned failure records. Distinct allocations with identical values are not one
allocation. Never subtract one retained object's bytes without proving its last owner is dropped. Sum/max
alone over existing H single-call totals fails these conditions [O9,O10].

M1 reserves kept case/combination returns, maximum active scratch and later caller-output phases before
numeric execution, including ordinary and proposed new publication together until commit. If the full
aggregate fails admission, release S2 W1 objects and decline W1 for that candidate set; preserve the ordinary
base. Source-invalid/ineligible cases may be excluded by their own existing refusal semantics, not to hide
costs.

**S4 — execute under one private execution permit.** New internal admission APIs consume the prepared batch and
its source/profile/caller-bound execution permit. Production F2a does not call the unguarded retained entry
directly. Keep 20B per case and one 60B InvocationMeter through cases/combinations; charge actual shared
builds, failed work, A1 certificates and overshoot unchanged. Reservation release never refunds LME. No
implicit retry or lower precision is introduced by the memory policy.

Combinations retain selected operands and run their own exact-ledger schedule. Preserve current
GroupCache::merged scope: new combination builds do not populate other operands' caches [O10].
Count every actual new build/returned owner; a failed combination preserves operand standing.

**S5 — private publication, then transfer.** Keep ordinary rows and standing until the complete W1 replacement
has certified rows, unit/derived checks, receipt binding and reader-compatible identity [O11–O14]. Kernel
Selected alone does not authorize replacement. The default proof should budget an ordinary base plus the
complete W1 draft, including duplicate row maps/strings/JSON/canonical bytes where source holds them. Any
future view/streaming optimization requires its own lifetime derivation.

Commit the complete PP replacement only after its own row/receipt checks. A per-case or combination
failure keeps that ordinary item; independent W1 items survive only under the adopted S-G1 contract.
A PP-level W1 receipt/finalization failure discards its uncommitted overlay and returns the preserved
ordinary base without a solve rerun. Reconcile this scope with I30; do not alter old source-block
finalization. The ordinary base need not be retained after a successful PP commit solely for memory
admission, if S3 already reserves the complete qualified caller window.

**S6 — qualified caller completion, reserved upfront.** Keep the current committed MechanicsEnvelope
return. The caller memory context remains live through its known JSON/hash/export phases; do not release
its completion reservation at PP return. No new memory-admission decision is needed after W1 commit
when the upfront bound covers every completion branch. No library bounds arbitrary later application copies.

The inspected completion shape [O14,O16] supports deriving an upfront bound: fixed input/request
objects, bounded source/receipt rows, per-row target/disclosure/review and annotation/accounting/witness
arrays, then a document clone and canonical text/key-sort scratch. Project all allowed row-class branches
and string/number widths; P4 must supply the actual byte/capacity derivation. H supplies none of it.

Late source/mechanics digest, numerical-standing, finite/unique-row, metadata/witness and derivative-hash
checks require solved data. They cannot all move before solving, but are semantic/export checks,
not demonstrated unprojectable memory decisions. Immutable input/profile facts can be checked before W1
without changing ordinary validation precedence. Existing late export failure retains mechanics and sets
canonical_export_unavailability; it is not W1 memory refusal. IO is outside these in-memory builders.

No specific unprojectable late memory dependency was found in the inspected functions. Keep the current
return shape; no pending-publication handoff or cross-crate rollback framework is recommended. If P4 cannot
bound a transitive helper or future S-G1 branch, decline W1 for that caller upfront and return the missing
term to ROOT. Only demonstrated need warrants another architecture; a handoff would add ordinary-base/
W1-draft overlap through caller finalization and require independent proof.

## 4. Sharing and concrete API changes to derive

[A1] Move the allocation-free core algebra currently at H/src/k6/w1/envelope.rs into an FK-owned
resource-model module; H keeps a thin compatibility re-export and its H caller wrapper, VR consumes the same
definition. No PP→H observation dependency and no copied stale formula. Preserve existing H/VR named
proof/construction variants as historical/current contracts. Add a separately derived product
construction/profile, never pass support_groups=0 or nonzero_prescribed_terms=0 to bypass missing proof. Share
checked arithmetic, MetricBytes, phase vocabulary and reviewed kernel terms only where their owner and
premises remain valid; H/VR caller totals and compiler facts do not transfer.

[A2] Add internal raw-census/preparation-admission types and an opaque PreparedRetainedBatch/PreparedGroup
with identity-bound scalar count accessors; numeric execution consumes it, avoiding duplicate preparation.
Private owners expose capacity/fact summaries where PP cannot inspect nested ordinary SparseStiffness or
kernel data. No general heap walker or unsafe pointer inspection. Names are proposed interfaces, not current
APIs.

[A3] Add PP's narrowly scoped retained_memory/admission adapter: borrowed ResidentOrdinaryFacts,
ProductSourceRecipeFacts, qualified caller variant, raw-preparation/full-phase expressions and an unforgeable
execution permit. Add fixed route metadata to LoadCaseSolve, preserving original ordinary outputs. Use checked
arithmetic and a bounded refusal enum, not panic/unwrap for input size. New fallible reservations can handle
their own errors; existing infallible kernel allocations do **not** thereby acquire graceful OS-OOM recovery.

[A4] Coordinate I30's certified publication bridge and its borrowing/consuming lifetime [O9,O11].
Derive its allocations and last-use rules; avoid blind RetainedSolve clones. Do not expose naked radii,
private precision states or caller-fabricated certificates.

[A5] Add a narrow qualified-caller context to the captured-Value entry path and a matching headless
reservation owner; preserve the committed MechanicsEnvelope return. The existing direct captured
entry can construct its direct-return context; headless supplies its separately qualified bounded recipe
from actual borrowed caller objects, not a free-form caller assertion. Its census and any W1-added
clone/planning buffers are admitted before allocation. Missing/stale caller facts decline W1 upfront.
Keep completion ownership until the registered output is assembled; no generic host guard or new
typed custody API is needed.

## 5. Refusal and qualification contract

Proposed internal reasons: memory_proof_unavailable, memory_facts_invalid, memory_arithmetic_overflow,
memory_allowance_exceeded (phase/metric/estimate/M), and preparation_allocation_failed where an actual
fallible reservation exists. Distinguish these from SourceError, work Budget and numerical Ceiling. Use D1's
info RETAINED_PRECISION_UNAVAILABLE with case/entity/reason; preserve ordinary rows, diagnostics' prior
numerical meaning and Current eligibility. Nonlinear-not-selected and exact-block coexistence bypasses remain
their own paths. A memory miss never means ordinary SOLVER_SYSTEM_BLOCKED.

A pre-admission refusal reports no numeric attempts and zero new LME; it is not zero preparation
CPU/allocation. A later refusal preserves all attempts and spent charges. Refusal has a cost: plan fixed
status cells and a bounded notification reserve before W1 work; render after releasing rejected W1 buffers.
Include ID strings, diagnostic-Vec growth and caller serialization. If the existing ordinary base itself
exceeds M, the ordinary/refusal return is not claimed≤M. A source-proved admission gate is the proposed
enforcement mechanism, not a sampling watchdog or allocator exception catcher. Qualification must show every
covered allocation is dominated by the correct check. There is no guaranteed recovery from external memory
exhaustion and no RSS guarantee.

## 6. Remaining bounded proof and acceptance work

1. **P1 — adapter/census identity:** close eligible input origins with I30, per-basis source recipes,
   raw/canonical population differences, groups, prescriptions, combinations, raw JSON/capacity traversal and
   finite census workspace. No rounded force net or typed-reserialized custody substitute.
2. **P2 — preparation:** independently derive every source/geometry/graph/RCM/ ledger/layout/encoding
   allocation and error lifetime before a complete descriptor exists; prove checked count accessors and no
   duplicate preparation.
3. **P3 — aggregate kernel ownership:** re-derive all precision, active/failed, cache-snapshot/Arc,
   retained-state, combination and release terms from source. Rebind the accepted kernel subterms to the
   proposed API without silently transferring H's base/caller or unsupported populations.
4. **P4 — publication/caller:** join I30's actual row/unit/derived/receipt flow and each named caller through
   output transfer; include old and new outputs, maps, validation/replay if required,
   canonicalization/serialization and PP publication rollback.
5. **P5 — profile and decision:** independently verify actual source, Rust/std/ dependency/target
   layout/capacity/request premises for each qualified build. Ordinary, cfg(test) and mutation-controls
   constructor paths are distinct (SourceParts clones differ). size_of alone proves no Vec
   capacity/private-container or caller lifetime. Unsupported build/caller remains W1-unavailable. ROOT then
   selects M and refusal/commit scope; supported-machine/concurrency interpretations stay owner-held.
6. **P6 — bounded implementation qualification:** only after that review/grant, test M−1/M/M+1 phase
   boundaries, overflow/missing/stale facts, constructor and preparation failures, many
   cases/bases/bodies/groups/long IDs, shared successes and cached failures, work stops, combination releases,
   PP publication rollback and separately bounded late export failure, exact-block byte identity and
   typed/no-custody bypass. Exercise direct and headless captured calls, including caller serialization and
   ordinary preservation. Use bounded allocator observations in isolated test binaries to challenge the
   derived requested/moving windows and over/under-count mutations; do not add a production H allocator.
   Required numerical predicates remain unchanged. Complete the normal F2a/S-G1 candidate checks and review.
   Later V-P measures product performance and may confirm/revise policy; it is not a prerequisite for creating
   F2a or a substitute for the new guard's own qualification.

No fresh historic-equality, F17, KF3-B1 redesign, KF2 dense-screen or owner-held 6-GiB/PHYS-R4 decision is
raised. Retain all eight RSS misses and unavailable precision-isolated peaks as limits; neither selects a
multiplier or a new test.

MISSING: P1–P5 derivations and adopted mechanism/M/refusal scope; P6 execution.

NEEDS_HUMAN_RULING: No new owner prompt now. ROOT's checkpoint-zero selection is pending independent
re-derivation; any new blocking-ordinary or deployment promise must be presented to its owner as a distinct
concrete decision.

DEPENDENCY_NOTES: I30 is an independent parallel plan, not accepted authority. F2a/S-G1
publication/readers must merge together. V-P follows F2a under amended Q5. Stop at this plan; no follow-up
work is commissioned.

Receipt 18:05:42 UTC; new-analysis cutoff 18:55:42; return deadline 19:05:42, 2026-10-02. Native TASK
/root/i29_w1_limits directly under ROOT /root. Only R/I29/f2a_memory_plan_02 is written. Origins/session and
informational inventory accompany this revisable packet; no acceptance seal is implied.
