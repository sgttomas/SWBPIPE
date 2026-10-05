# I29 P1 — RV48 repair return

**RV48-1/2/3 repairs are prepared for RV48 backcheck.** No roster acceptance,
numeric memory allowance, full memory/work proof or implementation qualification
is claimed. The corrected [count/ownership component](COUNTS_AND_OWNERSHIP.md)
retains C1/C2's source, combination, Call and native branch distinctions.

The source repairs are explicit:

- **RV48-1:** combination import owns a new GroupCache. For v256/v512/v1024,
  first occupied slot wins in authored operand order. Each chosen nonbudget
  failed slot adds its own copied Vec<BlockRefusal> backing:
  Delta_import=sum_j V_BlockRefusal(r_j), at most three vectors.
  Read r_j from the actual borrowed slot before cloning; r_j≤f_origin_j follows
  from its stopped shared-build Uc refusal inventory. Original vectors remain
  owned in P_prev. New copies survive to actual cache Drop or execution transfer.
  Arc-sharing successful payloads, seven origin refs and inline status size
  cannot hide these dynamic allocations. No numerical reachability witness or
  emptiness invariant is invented.

- **RV48-2:** RCM now carries S_RCM=max_i Sort_U(degree_i) at the actual
  sort_by_key phase while adjacency, neighbors and degrees are live.
  degree_i≤max(F−1,0) after deduplication; sequential sorts use their maximum,
  not sum. Before degrees exist, a qualified profile supplies an upper over
  that range. No constructor-scratch transfer or Vec-slack absorption is assumed.

- The corresponding complete phase terms are H'_import=H_import+Delta_import
  (plus any actual new heap capsule holding the cache header) and
  H'_RCM=H_RCM+S_RCM. Apply the conditional pair R0+H'_j / R0+2H'_j.
  The current inline/stack cache header and any future containing heap stride
  retain their own binding obligation. Coefficients remain unbound.

- **RV48-3:** the fresh checker validates every scalar and list element before
  multiplication, subtraction or summation. It rejects the review's Boolean
  node, negative constraint and masked negative-load cases, plus every
  validated scalar position, combination/list cases and a zero-product masking
  case. Width/layout inputs are checked too. All51 corrected checks pass;
  the original16 selected control records are exactly equal, including their
  expected rejection results.

The original _run_records/count_checks.py, COUNT_CHECKS.json, ORIGINS.json and
SESSION.json remain byte-identical to original P1 at8eaca35bdc. The new checker,
results, source/change origins and preservation record are only under
[_run_records/correction_02](_run_records/correction_02). Reviews are unchanged.

**Free-block sentinel clarification:** bound.rs:90–106 reserves u32::MAX as
unvisited. Each new block id requires positions.len()<u32::MAX; a final
f≤u32::MAX (or prior F≤u32::MAX) is sufficient. On a≤64-bit target the current
stronger premise that the unreduced F*(F+1) product fits usize already implies
that bound. Checking only the reduced triangular value would require the
explicit sentinel premise. RV48 did not prove a counterexample to the stronger
current guard. This is representation safety, not a new numerical domain choice.

The preparation component still covers borrowed census and its own workspace,
raw/validated counts, exact source/ledger/combination encoding lengths,
SourceParts/PrimitiveSource/CasePrep, actual graph/order/blocks, maps/registries
and partial failure owners. It creates no rounded combined PrimitiveSource,
Source/Run for pre-source refusal, free allocating census or unaccounted rebuild.
Reached case preparation keeps exhaustion→group→CasePrep order. Prepared ordinary
operands do not acquire a graph/cache/solve.

- PROPOSAL: Backcheck the repaired P1 roster and helper validation
  - Evidence: RV48 RETURN at44a6f7d9cf; native cache/sort/block sources; correction_02/CORRECTION.json and COUNT_CHECKS.json.
  - Change: Review the new import/sort terms, their requested/moving inclusion, copied-vector handoff and per-input validation while preserving original evidence.
  - Why: Supplies the two omitted allocation owners and the checker correction without changing methods or byte coefficients.
  - Risk: Actual type/capacity/sort/build/format profiles and later numeric/caller/suffix owners remain unbound. Source allocation coverage is not a numeric allowance or runtime guarantee.
  - Status: PROPOSED

MISSING: RV48 backcheck; reviewed actual strides/capacity/sort/tree/Deque/Arc/
String/format profiles; changed I34 status/type effects; exact caller R0 and
ordinary suffix; numeric/aggregate/certificate/native-completion composition.
Later obtain_verify and finish_selected failure-vector copies remain later
owners, not silently covered or removed by this preparation repair.

NEEDS_HUMAN_RULING: no new owner question, byte allowance, domain or code choice.
ROOT disposition and independent backcheck precede reliance.

DEPENDENCY_NOTES: Original accepted count/encoding lemmas retain their scope.
Complete memory and work-exactness obligations remain separate. No implementation,
runtime, Git/index/API writes, delegation or follow-up is commissioned.

Repair receipt2026-10-02 21:54:42 UTC; new-analysis cutoff22:09:42;
hard return22:19:42. Direct native TASK /root/i29_w1_limits under ROOT /root.
Only the three authorized revisable files and three new correction_02 files
were written. WRITE_INVENTORY.json is informational, not an acceptance seal.
