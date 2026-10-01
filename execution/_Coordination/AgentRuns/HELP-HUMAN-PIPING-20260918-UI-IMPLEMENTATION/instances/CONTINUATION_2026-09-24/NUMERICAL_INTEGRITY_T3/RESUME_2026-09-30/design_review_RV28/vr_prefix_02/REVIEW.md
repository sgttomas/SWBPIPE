# RV28 — VR prefix component review

**NOT VERIFIED as a complete counts/prefix owner union.** One SHOULD-FIX,
RV28-PREFIX-1, omits the fresh free-DOF vector in the Counts initializer.
The parser/read/raw-to-typed ownership and final cut-survivor composition are
otherwise source-consistent within the explicitly fixed successful path.
This return supplies component review, not complete E_max or checkpoint/admission
acceptance. No concrete runtime failure or numerical final-total undercount was
demonstrated.

TASK /root/rv28_a1_design, direct return to ROOT /root; native followup_task,
no delegation. Actual start 2026-10-01 19:36:25 UTC; deadline 19:56:25 UTC.
TIMING.json records completion. Raw commands, origins, hashes and checks are
under _run_records. The canonical independent K0 review was read **first**,
before inspecting or relying on this proposal's integration.

Aliases: P=projects/chirality-piping; FK=P/core/solver/frame_kernel;
VR=P/validation/benchmarks/numerical_robustness; R=this resumed run.

## Basis

Reviewed packet R/metric_design_08_vr_prefix at K6C
`5e967db01cc370a9dfd596b782a0b6147a6932af`, seal
`ab75fce30ff4be0716f2fbc35af17079294bde6f6c81f5c3e7245e918a7492a7`.
All payloads match. Brief is NUM
`a341d1cf9527fb2b9dfa2bb486f88057847aa94c`,
R/BRIEFS/K6C_PENDING_PROPOSAL_REVIEWS.md.

Product source is immutable
`40129a225d73860ac2a53da9a2fa73869df668f3`.
Six proposal product snapshots were independently matched to whole Git blobs;
the additional source.rs blob establishes the finding below. Thirteen
existing std HTML/snapshot hash pairs and five authenticated package snapshots
were checked; no package/archive/library research programme was reopened.

The family file is 986,182 bytes, SHA256
`16357afa5efeaaac3ab5798bb6f632104bec1e2dfada288f5b0936bfe6188759`.
All twelve existing external files matched their prior sealed input identities
and were unchanged across the read. The 24 family lines and 12 external
documents were rechecked for UTF-8, absent escape bytes, unsigned-u64 number
lexemes and empty spring-array syntax. No input JSON/typed-model/product parser
was executed; duplicate-key absence, full histograms and depth are reused from
the sealed fixed-input metadata with exact raw-byte identity, not claimed as a
fresh parser result. This preserves a narrow read/metadata check.

## RV28-PREFIX-1 — SHOULD-FIX: include the Counts free-DOF receiver temporary

**Location:** PREFIX_UNION.md:247–255, the HCounts equation.
Actual trigger: VR/src/scale.rs:145 evaluates

    free_dofs: source.free_dofs().len(),

before pattern_entries at :147, layout at :151 and encoding at :152.
FK/src/structural/retained/source.rs:629–633 defines free_dofs() as

    pub fn free_dofs(&self) -> Vec<usize> {
        (0..self.dof_count())
            .filter(|&g| self.constrained[g].is_none())
            .collect()
    }

This is an owned fresh filtered collection, not a borrowed cached list or a
scalar count. Its returned capacity follows the already reviewed lower-zero
collection rule G(usize,f), including its construction active-old term for the
move metric. SRC0 does not retain this caller-created free vector.

The .len() receiver is a temporary within the same Counts tail expression as
the layout and encoding receivers. The installed Rust Reference's temporary
scope rule supplies no smaller drop scope at a struct field; even the 2024
tail-expression narrowing ends after the **whole** tail expression, not after
the field. Therefore the safe source lifetime retains this vector through
the later pattern, layout and encoding phases. It drops before counts returns
and before tV, so it is absent from cut survivors but cannot be erased from
PrefixPeak.

**Impact:** the written HCounts does not enumerate a reached allocation or its
overlaps. The omission is real even if another, larger branch ultimately
dominates the corrected maximum. No such dominance warrant is supplied, and
this review does not infer an actual aggregate underestimate from the missing
term alone.

**Concrete repair:** add RFree=G(usize,f) and
HFree=RFree+epsilon*O_G(usize,f). A safe phase decomposition above the unchanged
outer SRC0/k4src/caller owners is:

    HCounts = max(
        HAdj,
        RAdj + HProfileCount,
        HFree,
        RFree + HPatternCount,
        RFree + HLayout,
        RFree + RLayout + HEncoding(X)
    )

Here RLayout=G(QuantityMeta,q) and HLayout includes its push construction and
active-old growth using the existing source02 operator. The separate layout
phase makes that growth explicit; an already proved domination by a different
branch may simplify the maximum, but must be stated, not assumed.
Do not add all these alternatives together, do not add RFree to C_tV, and do
not change maintained source to avoid the allocation under this review scope.
Use an additive proposal correction and the same reviewer backcheck.

The remedy uses existing public-type capacity facts and entails no private-node
probe, new tool, numerical/domain choice or extra human checkpoint. For scale,
when f=60,000, the source02 G(usize,f) on the named 64-bit target is
65,536*8=524,288 requested bytes. That is a conditional component amount, not a
claimed delta in the final E_max.

## Successful fixed-input parser/read path

The proposed successful parser branch is consistent with the inspected source.
serde_json StrRead borrows the family line/external text; unescaped
parse_str_bytes returns a borrowed slice without extending scratch. The
Deserializer starts with Vec::new and the fixed u64 integer tokens do not enter
float/long-integer scratch. Thus successful scratch allocation zero is a
specific reached-path consequence; it is not a malformed-input, arbitrary
number, recursion-failure or allocation-failure guarantee.

ValueVisitor copies string/key bytes into owned Strings, pushes array children
into Vec::new, and inserts object key/value pairs into the configured BTreeMap.
The whole-tree Jin padding correctly includes unattached current children and
completed siblings once; the single currently growing array needs one
active-old buffer. Unique-key metadata is needed to avoid separately discarded
overwrite histories. The no-arbitrary_precision/no-raw_value/BTreeMap feature
basis remains a final correspondence condition. Pushed Value arrays use G,
not source15's exact-length serialized-array E.

Hargv counts the initial exact argv Vec and each copied argument buffer once,
including argv[0]. Moved Args strings share those same underlying allocations
until the iterator and unconsumed arguments drop. The later id clone is a
separate allocation. Cap installation after args is not a pre-args bound.

The file-read bound is conservative under the stated stable-file/metadata
premise. CString path allocation at length>=384 dies after open, before the
String's read capacity is reserved. default_read_to_end's 32-byte probe is
stack storage. Each successful request need is at most F+32, and an old full
buffer is at most F, so ReadRet=max(8,2(F+32)) and the added move old<=F are
valid loose source uppers. Exact metadata can be tighter but is not required
for that bound. String UTF-8 validation uses the same byte Vec. try_reserve
conversion, open/read errors and panic/format leaves remain separately owned,
not zeroed.

Path joins, actual argv, CARGO_MANIFEST_DIR/cwd and future stable input identity
must be frozen for a final invocation. Current input hashes do not bound a
future growing/different file. A general file-size or path-limit policy is not
created by this review.

## Raw-to-typed construction and earlier freed peaks

The proposed ownership distinctions are sound within inherited source02/source09
constructor rules:

- Model parsing pushes the name/coordinate arrays and exact-collects the
  borrowed fixed-size member/spring/constraint/load/station arrays; its String
  children are new copies while raw Value still lives.
- Case parsing keeps rows/controls plus id/family/basis/unit/hash and map children.
  BTreeMap FromIterator has input-pair backing, sort scratch and bulk construction,
  so final node bytes alone cannot be its construction envelope.
- Whole family text and the outer growing Vec<Case> remain while one line is
  parsed and typed, alongside all previous typed cases. HFamily preserves that
  phase peak. IntoIter::find then moves the chosen Case and drops the remainder
  and backing. The discarded family owners are not cut survivors.
- Embedded Case.model and the standalone clone are distinct. Clone uses logical
  lengths rather than old spare capacities.
- For an external model, read text, raw Value and typed Model all coexist while
  the second tuple operand hashes the text. The SHA message clone/padding
  survives through hex collection. HHex/RHex stay separate; a 64-character
  logical digest does not alone prove an exact 64-byte String capacity.
- The start JSON construction/merge/stream happens before counts. Temporary
  JSON trees drop afterward; initialized stdout/lazy children remain.
  Source15/serializer and format/error owners remain explicit dependencies.

These formulas are conditional source unions, not measured prefix readings.
None may be replaced by a no-op baseline or the old fixed/model estimate.

## Counts helpers and added tree specializations

profile(&free_adjacency(model)) is a separate statement before the Counts
initializer. Its temporary adjacency lasts through profile; profile's local
vectors then drop at that statement boundary. The inspected K4 RCM returns an
order with exact f capacity; rank and block are exact f usize arrays. Marking
before DFS push bounds the stack's high-water population by f.
The proposal correctly uses K4's four-buffer eccentricity result and retains
input/output adjacency and potential inherited header capacity; it does not
substitute SD's six-buffer result or allocate a profile-value matrix.

pattern_entries constructs touched bool[N] and inserts ordered member pairs
into BTreeSet<(u32,u32)>, at most 2m distinct entries. This is an additional
reached specialization; the old seven-node list does not cover it by name.
The request parameter must bind its actual SetValZST/node construction, not an
invented equivalence to usize-key or unit-valued nodes. NODE_INTERFACE correctly
keeps it separate.

The spring-only BTreeSet<(u32,usize,usize)> is initially empty and receives
entries only in the model.springs loop. Exact byte/metadata binding confirms
zero springs for all 24 actual RF-LARGE models, including all 12 external
inputs. Therefore it allocates no node on this roster. A wider API still
requires its distinct I_spring3 term; fixture emptiness does not narrow the
API or prove a generic zero.

The first source.encoding() buffer is present during its hash then dropped;
k4src survives. Counts later creates a second encoding buffer and a layout
temporary, not a cached reuse of the first. RV28-PREFIX-1 adds the earlier
free-DOF temporary to that initializer's owner map. All three die before tV;
returned Counts and Sizes have no heap children.

## Runtime clarification requested by ROOT

After the initial review, ROOT directed a check against source12 H_LEAVES:79–114.
I read that source result plus independent h_leaves_09 and the additive
h_request_bindings_10 closure. The latter closes the former Mutex/Once facade
gap; it must not be treated as still pending.

**The successful direct-entry source warrant is already enough for its
registered pre-args peak, not just post-entry survivors.** On the stated direct
single-main/no-foreign-initializer/non-unwinding path, the only registered entry
owners are at most the 4-byte name Box, one registry mutex child and the first
ThreadInfo leaf. They accumulate and persist until main; there is no tree
replacement, competing OnceBox loser or growing registered buffer in that
traced construction. The other named entry work is scalar/static/stack or OS
memory outside the registered metric. Consequently the entry phase can use

    EntryPeak <= M_stack + L_info + 4

with beta=1 conservatively, and zero growing-realloc old for those particular
constructors. Combine that prefix with Hargv by identity, not by an extra
unknown startup allowance.

After first stdout initialization, its separate buffer/mutex are added under
their already reviewed source conditions. Their request values remain the
known qualified artifact facts plus final VR correspondence; this review
does not promote 64/544 from a historical artifact into final-build acceptance
or reopen them as unknown layout facts.

Thus PREFIX_UNION residual #2 should be **narrowed** in integration: there is
no identified missing successful direct-entry temporary edge requiring another
generic runtime audit. The additional warrant needed is applicability to the
final VR entry/std/cfg/allocator/artifact and preservation of the same direct
single-main path. Failed startup, foreign initialization, panicking/unwinding
and any additional reached lazy/error behavior remain outside that source
claim and require their own supplied bound or explicit registered qualification.
A completed-H-only assumption must not silently become a bound for every
prelaunch failure. H's initialized raw-byte dropped-error writer result likewise
does not prove the wrapped serde writer/error route; those leaves remain with I21.

This is use of previously supplied successful-path source facts, not a new
whole-runtime proof or a request to broaden scope.

## Cut survivors, global window and residuals

The proposed cut-survivor identity union is correct:

    RuntimeRet_after_start + ArgsRet + separate id clone
      + selected Case children + standalone Model
      + optional external model hash + k4src hash.

There are three independent id Strings: Args.case, id and Case.id (the last
already in RCase). All family text, discarded Case children/vector backing,
raw Value, path, source, source encoding, adjacency/profile/pattern/layout and
free-DOF temporaries are gone by the cut. Their earlier peaks still matter.

Preserve

    max(PrefixPeak, max_later(surviving cut identities + later owner union))

for both requested-byte and growing-move counters. Current-at-cut or observed
stage prefixes cannot replace global history. Admission remains before launch;
no new metric/window/no-op value is authorized.

Outstanding categories are precise:

1. Repair RV28-PREFIX-1 and independently backcheck the corrected counts union.
   Bind the added pair-set request through its owning node evidence; this review
   does not inspect or accept the concurrently assigned node artifact packet.
2. Compose actual HHex, JSON start/stream/format/IO/error/panic leaves and all
   required failure routes. The fixed successful parser/read path is not their
   substitute.
3. Bind final launch/input/path/feature/std/source/allocator/request-site
   correspondence and checked implementation, including any future estimator
   allocation. Successful direct-entry source facts are supplied within the
   scope above; arbitrary startup failure/foreign/panic behavior is not.
4. Final complete same-binary totals, admission replay, required measurement
   and full K0/E_max acceptance remain later obligations.

No other actionable defect was found in the inspected component. No closed
numerator, sparse, queue or unrelated library theorem was reopened.

## Execution

One evidence check completed with **387 passing source/hash/lexical assertions**,
covering 36 fixed input documents, six immutable product snapshots, thirteen
std snapshot/original bindings and five package pages. Raw results are under
_run_records. This count is not a runtime/test count.

No production/parser/model/solver runtime, build, probe, new library programme,
maintained edit, Git/index mutation, allocator/guard change or delegation occurred.
The only writes are this review subtree. Prior seals are unchanged.

