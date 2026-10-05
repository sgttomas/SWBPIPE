# I32 C1 — logical receipt and reader contract draft

Status: proposed faithful realization for fresh independent review, not implementation authority or an identity reservation. Source is main `49034a940f3f8cd3f3da4d4cbc839943b808063d`; inspected NUM HEAD `6a964324bcdb33cccc722ce00f6d8f4be390d592` has no maintained P/{core,apps,schemas,fixtures,tests,validation} diff against it. P means `projects/chirality-piping`; FK means P/core/solver/frame_kernel. D1 means accepted r5a.2 with selected R7/A1 and subsequent ROOT amendments; D2 means r5b.2 as amended. The A1 correction and current source supersede older v1 claims.

## 1. Projection and conservation

Keep native physical records once, then put logical candidate attempts in the public `attempts` list. A native `VerificationThenCandidate` has two roles in the logical graph; copying it twice or filtering it away is wrong. Kernel `Accepted` is separate from facade selection.

For native record r define exact nonnegative integers:

```
W_r = r.work.limb_multiply_equivalents()
K_r = r.k4_work.limb_multiply_equivalents()
O_r = W_r + K_r
D_r = r.stop_rule_work
Q_r = r.verification_work
S_r = r.shared_work
V_r = r.verification_shared_work
b_r = int(r.shared_built_here)
v_r = int(r.verification_shared_built_here)
```

`D` includes candidate comparison and A1 publication certification. `Q` includes the verification pass on the already solved state. Both are subsets of O; Q and D are disjoint stages, so `0 ≤ D+Q ≤ O`. Define the physical solve/verification fragment B=(O−D,S,V), with subcounts solve-own O−D−Q and verification-own Q; define the candidate-decision fragment T=(D,0,0). Neither split separates W versus K internally: retain W,K and all stage totals in the one physical record; the split is at aggregate LME, which the native API actually provides.

Projection is deterministic in native record order:

1. A `Candidate` physical record creates a logical candidate, owning its B and T fragments. If its solve failed, its verification reference is null.
2. A `Verification` physical record belongs as B to the immediately preceding logical candidate that requested its precision 2p. Its T must be zero. A failed verification remains present with its failure reason; it is never a candidate merely because it has precision 256 or 512.
3. A `VerificationThenCandidate` physical record gives B to its earlier requesting pair and T to its own later logical candidate. The latter contains a backwards `reused_from` reference. Its earlier verification phase completed, although its final native outcome was overwritten by its later candidate outcome. Preserve the final native outcome verbatim as typed data; describe the earlier phase as `completed`, not as kernel `Verified`.
4. A successful pair's candidate record is Accepted and its verification record is Verified. The last *logical candidate* is therefore Accepted even though the last native record is Verification/Verified. A ceiling's final 1024 Verification/Solved is a completed verification of a rejected 512 candidate, never an accepted candidate at 1024.
5. Failed candidate solves may skip precisions. A candidate solve failure with Pivot/Condition/ResidualGate advances one candidate slot; a failed verification solve with one of those reasons advances two slots. Verification-pass failures are terminal. Other terminal reasons, pair-identity errors and certificate errors follow the actual schedule. Do not require a fictitious contiguous 128,256,512 attempt list.

Every B fragment belongs to exactly one logical candidate, and every T belongs to its own candidate (including zero T on a failed solve). Hence, without overflow/saturation:

```
case_charge = Σ_native (O_r+S_r+V_r) = Σ_logical case_charge
invocation_increment = Σ_native (O_r+b_r*S_r+v_r*V_r)
                     = Σ_logical invocation_increment
invocation_after = invocation_before + invocation_increment
```

The proof is a partition of additions, not a new work model. Shared reuse changes only the invocation coefficients. Do not deduplicate by digest: independent builds of identical sources incur separate charges. Existing native `shared_stages` combines S and V, so require its total to equal S+V; it is not an additional charge. Own stages sum to O. Failed/non-budget cached builds retain their costs and flags; budget failures are not cached. Preserve bound refusals even when a later stage stops.

Existing-record arithmetic in `_run_records/ARITHMETIC.json` establishes:

| Immutable record | Logical candidate case charges | Case and invocation increment |
|---|---|---:|
| current VR RF-RANGE-CHAIN-L-240, Selected128/256 | p128: 7,009,209 | 7,009,209 |
| current VR RF-RANGE-THIN-A, Ceiling with reused256/reused512 | p128: 658,778; p256: 1,375,193; p512: 2,408,105 | 4,442,076 |
| existing H T4 247 CHAIN-AX prefix1, Budget(Case) | native per-attempt breakdown not in this prefix record | limit 2,407,264,229; spent 2,407,292,969; overshoot 28,740 |

All shared flags in those two VR examples are built=true. The equations prove the false-flag case algebraically; a shared-cache runtime witness remains a future fixture. The stopped H summary proves its stated charge/overshoot only; it cannot establish a missing fragment breakdown. Historical unlimited case limits are not substituted for the selected 20B/60B policy. No solves ran.

**Required source prerequisite:** `adaptive.rs:3556,4168` returns Refused without attempts; `finish_terminal` can discard charged records for NegativeEnergy/Structure. `combine.rs:129` also drops Refused evidence. Preserve the actual attempts at those return seams, including empty vectors for genuinely pre-schedule refusals. Never reconstruct them from the meter difference, a Debug string, or an assumed zero. See SOURCE_COMBINATIONS.md for the narrow API change.

## 2. Counters and stop thresholds

Runtime work policy proposal `W1-LME-20B-60B-v1` pins Lc=20,000,000,000 for each case **and each mechanics retained combination**, and Li=60,000,000,000 for one actual invocation. One meter spans all calls, retries if separately authorized, and combinations. It is not reset on fallback, refusal or facade certification failure. No pricing is added for unmetered source/preparation/graph/encoding, ordinary/arbitration/caller or new facade arithmetic. Those have separate finite-cost/memory obligations.

The native checkpoint uses `used > room`, Case before Invocation if both exceed, and `solve_cases` refuses subsequent cases when `charged >= limit`. Thresholds admit recorded overshoot; they are not hard maxima. The receipt records actual charges, before/after values and terminal scope, not clipped limits. Reader G5 accepts an honest unavailable overshoot; selected cases must have a schedule and final totals consistent with passing the actual final guard. A previous selected case is not retroactively invalidated because a later unavailable case overshoots Li.

All successor integer counters must lie in [0,2^53−1]. Rust derives sums/differences with checked u128/u64 conversion; Python uses int with explicit bounds and rejects bool; TS uses validated Number.isSafeInteger inputs converted to BigInt for all accounting and then range-checks. Never aggregate charges through binary64 Number arithmetic. Counts/indices and byte lengths use the same checked policy; a native `usize::MAX` per-block refusal row is encoded as the tagged string `block_step`, not a huge JSON number. Other rows use `{kind:"row",index:U}`.

The kernel's Work/ SumWork/InvocationMeter totals saturate at u64::MAX, but upstream SumWork components, some aggregate expressions and stage accumulation use ordinary additions. Returned small, internally consistent counters and checked projection sums do not prove that an upstream counter never wrapped. Exact-charge reliance requires independently checked **upstream no-wrap evidence**, bound before execution to the actual admitted source/count/build profile. P1–P5 must cover cumulative component increments, aggregate and stage additions, maximum unguarded segments, whole attempts/invocations, reuse and every stopped/error path. The 20B/60B intermittent thresholds and provisional byte target alone supply no such proof. Without that established admission premise, decline W1 before execution and preserve the ordinary transaction; a retrospective equality check cannot grant it. A separately authorized checked/sticky-overflow seam is an alternative only after its own bounded design, maintained write-set and review, not an implicit change in this contract.

With that upstream premise established, the projector additionally requires nonsaturated, internally consistent counters and checked sums. An observed u64::MAX cannot distinguish exact equality from saturation; mark `saturation_not_excluded`, never invent lost counts. A checked inconsistency, max counter, sum beyond u64, or count beyond the safe JSON range prevents selected successor emission. Unexpected missing no-wrap evidence must never be labelled exact. Native arithmetic failures/panics are not repaired by JSON projection. The upstream premise is an open implementation-blocking proof obligation, not a result established by this draft.

For a representable unavailable case in an otherwise selected successor, retain exact safe counters in its `run`. For an unencodable/saturation-ambiguous run, abandon successor finalization transactionally and return the preserved base ordinary publication with `RETAINED_PRECISION_UNAVAILABLE`, reason `receipt_encoding`, detail `work_counter_range|work_counter_inconsistent|saturation_not_excluded`. Existing diagnostic evidence text may retain decimal native counts labelled `exact` only when known, otherwise `saturated_lower_bound`; no new base top-level member, unsafe JSON integer, fake exact total or infinity is emitted. Keep full internal/run evidence where the caller's admitted custody permits it. Publication-hash failure similarly uses `publication_hash_range`, unchanged base rows and spent ledger. Such fallback must be reserved before allocations (memory dependency).

## 3. Proposed identities and hash scopes

ROOT reserves these names only after review/collision recheck. Maintain constants/schema/table content in the named product/reader files; never read a dated AgentRuns path at runtime.

| Proposed identifier | Purpose |
|---|---|
| `openpipestress.result_semantics/0.3.0/preview-physics-retained-1` | Preview successor; inherits exact preview base table hash |
| `openpipestress.result_semantics/0.3.0/physics-retained-1` | Exact successor; inherits exact physics base table hash |
| `product_preview_retained_w1a_v2`, `exact_straight_retained_w1a_v2` | Distinct profiles; no 0.4.0/F3 identity |
| `M03-INTEGRITY-MP-v2` | Already selected corrected kernel policy; v1 is not relabelled |
| `RP-LOGICAL-ATTEMPTS-v1` | This physical-record/logical-candidate projection |
| `W1-LME-20B-60B-v1` | Current work thresholds and native checkpoint/cache accounting |
| `RP-FACADE-SI-v2` | Proposed final-row normalization/certificate contract; contents wait for I31 review |

Use `H(d,p)=sha256(UTF8(JCS_checked({"domain":d,"payload":p})))`, exactly the existing checked `openpipestress_jcs_ijson_v1` pattern. Proposed domains: `retained_precision_receipt_mp_v2` over body; `retained_precision_publication_mp_v2` over the **final successor envelope with only retained_precision removed**; `retained_precision_source_mp_v2` over the source binding described below. Publication hash binds final row ids, diagnostics, ordinary quality, contract evidence, identity and profile. Do not hash the base projection instead.

Invocation uses the existing `source_blocks_invocation_v1` hash of `{request:actual_raw_request,solver_mode:actual_mode}`; the receipt names algorithm/profile/scope, and the readers recompute it when custody is supplied. Typed reserialization never qualifies. Kernel bytes already have versioned domains: raw SHA256 of K4SRC\x01 or K4CMB\x01 source bytes, K4LED\x01 ledger bytes and K4RST\x01 state bytes. Preserve the exact existing byte encodings, lengths and endianness; the combination encoding preserves ordered factor bits and operand encodings, not a netted/rounded sum. Product source binding is the closed object `{kernel_source_sha256,stiffness_sha256,basis_ref,material_basis_ref,id_maps,body_membership,layout,stations,supports,section_terms}`. Its `source_identity_sha256` is H(`retained_precision_source_mp_v2`, source_binding); stiffness_sha256 is raw SHA256(K4STF\x01 bytes). Bind the bounded canonical arrays directly; never hash Rust memory layout or Debug text. The exact id/material mapping arrays are I30's required shared interface, frozen before schema implementation; the intended entries and checks are specified below. Memory admission counts those arrays and hash scratch.

Policy/table hashes bind these stable contents and limits, not the path of the historical decision record. No byte allowance or byte-policy id is selected here.

## 4. Closed field contract

This is a field specification for the future JSON Schema, not a schema installation. All objects are closed (`additionalProperties:false`), enums closed, arrays bounded by admitted counts, and absent/null distinguished as stated. `U` means safe nonnegative JSON integer; `Bits` means exactly 16 lowercase hex digits with finite decoded binary64; `Hash` means 64 lowercase hex; `Ref` is `{ref_type,ref_id}` with the route-specific fixed ref_type. Nonnegative quantities require canonical +0, never negative zero. Signed source operands preserve their actual finite bits.

| Object | Exact members / meaning |
|---|---|
| `retained_precision` | `body`, `receipt_sha256:Hash` |
| `body` | `receipt_version:1`, `policy:"M03-INTEGRITY-MP-v2"`, `projection_policy`, `work_policy`, `facade_policy`, `canonicalization`, `invocation`, `publication_sha256`, `work`, `cases`, `combinations` |
| `invocation` | `algorithm:"sha256"`, `profile:"openpipestress_jcs_ijson_v1"`, `scope:"actual_request_and_solver_mode"`, `domain:"source_blocks_invocation_v1"`, `value:Hash` |
| `body.work` | `case_limit:U=20B`, `invocation_limit:U=60B`, `charged:U`, `execution_order:[{kind:"case"|"combination",index:U}]`. Entries cover every metered run exactly once in actual order. Nonexecuted entries need not appear. `charged` equals final after, with initial before=0; case-order arrays do not imply combination order. |
| case common | `basis_ref:{ref_type:"load_case",ref_id}`, `status`, `ordinary:{quality_index:U,diagnostic_refs:[string]}`. Ordinary reference binds the unchanged numerical_quality entry and exact existing diagnostics, including a real failed-attempt representation supplied by I30. |
| `not_required` case | Common members only. No attempts, selected summaries, token or made-up zero-work solve. This branch means ordinary pass, not unsupported scope or W1 failure. |
| `selected` case | Common + `method`, `run`, `source_binding`, `source_identity_sha256`, `selection`. `method` is exactly `contribution_preserving_multiprecision_v1`. |
| `unavailable` case | Common + `reason:{code,phase,detail}`, `diagnostic_ref`, `run:null|Run`. Phase is `routing|preparation|kernel|facade|receipt`. No method token or selected payload. `run:null` is legal only when no kernel schedule ran; preparation can still cost resources. |
| `Run` | `kernel_terminal:{kind:"selected"|"unresolved"|"refused",reason:null|Reason}`, `records:[PhysicalRecord]`, `attempts:[LogicalAttempt]`, `case_charge:U`, `invocation_before:U`, `invocation_increment:U`, `invocation_after:U`. A facade refusal can have kernel_terminal selected and a final logical Accepted while public status is unavailable. |
| `PhysicalRecord` | `index:U`, `precision:128|256|512|1024`, `role:"candidate"|"verification"|"verification_then_candidate"`, `outcome:Outcome`, `residual_basis:U`, `corrections:U`, `pivot_margin_min:null|Bits`, `rcond:null|Bits`, `residual_worst:null|Bits`, `gate:null|Gate`, `work:Work`, `storage:{pattern_entries:U,profile_entries:U,limbs_per_entry:4|8|16}`, `verification:null|Verification`, `bound_refusals:[BlockRefusal]`. These are snapshots, not a public radius or report. |
| `Work` | `wide_lme:U`, `exact_sum_lme:U`, `own_lme:U`, `own_stages:Stages`, `shared_lme:U`, `shared_built_here:bool`, `shared_stages:Stages`, `stop_rule_lme:U`, `verification_lme:U`, `verification_shared_lme:U`, `verification_shared_built_here:bool`. own=wide+exact_sum; stages/subset equalities as §1. |
| `Stages` | Exactly the 19 native names: formation, assembly, residual_formation, factor, condition, rhs, solve, refinement, recovery, stop_rule, bounded_gate, scale, estimate, charge, bound, shift, bounded_formation, wide_formation, uc; each U. |
| `LogicalAttempt` | `precision`, `candidate_record:U`, `origin:{kind:"fresh"}|{kind:"reused_verification",attempt:U}`, `verification:null|{record:U,precision:U,phase:"completed"|"failed",reason:null|Reason}`, `outcome:Outcome`, `charges:[{record:U,part:"solve_and_verification"|"candidate_stop"}]`, `case_charge:U`, `invocation_increment:U`. Fragment amounts are derived by §1, not separately attested mutable numbers. Every B/T exactly once. |
| `Outcome` | `{kind:"accepted"|"verified"|"solved"}` or `{kind:"rejected"|"failed",reason:Reason}`. Logical outcome permits only accepted/rejected/failed; physical preserves native final role/outcome. |
| `Gate` | `{kind:"coalesced"}` or `{kind:"bounded",state:U,evaluated:U}`; preserve selected best-state meaning. |
| `Verification` | `resolution:[{body:U,force:Bits,moment:Bits}]`, `theta:[{body:U,value:Bits}]`, `bound:[{body:U,value:null|Bits}]`, `data_blocks:U`, `shift_factorizations:U`, `uc_missing:null|U`, `g_max:U`, `g_violation:null|U`. Exact current native summary, body-indexed without pretending absent B is zero. |
| `BlockRefusal` | `block:U`, `bound:"uc"|"s"`, `kind:"span"|"exponent"`, `pass` from current BoundPass, `location:{kind:"block_step"}|{kind:"row",index:U}`. Preserve array order/duplicates as recorded; do not drop an earlier refusal because later work succeeds. |

`Reason` is a tagged structural translation of every current AttemptReason/AttemptStop/UnresolvedReason/Refusal/CombinationReason variant, with its exact quantity/body/kind/member/DOF/issue/scope payload. Do not serialize Debug strings. Freeze its enum mapping directly against `adaptive.rs:145–211,2477–2530,2585–2630`, `bound.rs:222–290`, `combine.rs:44–67` and the referenced `WideError`, `StructuralError`, `LedgerRefusal` enums. Expose only the variants reachable in the admitted W1a source contract; unknown variants fail encoding to the named ordinary fallback, not an arbitrary `other` reason. This mapping is an implementation prerequisite, not permission to discard a new variant.

`selection` exact members: `precision`, `verification_precision`, `ledger_sha256`, `retained_state_sha256`, `stop_rule:[{body,kind,value:Bits}]`, `floor_ratio:Bits`, `body_scales:[{body,translation:Bits,rotation:Bits,force:Bits,moment:Bits}]`, `input_derived_dofs:[{node_id,component}]`, `section_terms:[{member_id,area:Bits,section_modulus:Bits,length:Bits,axial_stiffness:Bits,torsional_stiffness:Bits}]`, `absolute_verified:[{result_id,bound:Bits}]`, `not_covered:[result_id]`, `pivot_margin_min:Bits`, `rcond:Bits`, `rcond_label`, `residual_worst:Bits`, `corrections:U`, `resolution_scale:[{body,force:Bits,moment:Bits}]`, `verification_estimate:[{body,kind,value:Bits}]`, `verification_charge:[{body,kind,value:Bits}]`, `theta:[{body,value:Bits}]`, `certified_bound:[{body,value:Bits}]`, `floor:null|[{body,force:Bits,moment:Bits}]`. `kind` in these summaries follows native coverage (four kinds for stop, force/moment for estimate/charge). `floor` is present as a non-null array only at p512, including zero entries; B entries exist only for bodies with data-bearing blocks. RCOND_LABEL is pinned to current source. No structural_zero exemption is implemented in this slice; reject that member. No twist/extension public scales.

Source binding contains the exact proposed common material-basis reference, bijective product↔kernel id arrays, body membership, quantity layout, stations (id/member/fraction bits), supports (id/node/restraint vector/spring ids), section terms and kernel source/stiffness digests. Prefer sorted canonical arrays with explicit original case/basis ids; source constructor sorting cannot be used to lose authored combination term order. Layout identities are structural QuantityId objects, never coincident numeric indices. All arrays need checked population limits from the memory design. Readers verify derivable facts and equality to selection section terms; remaining source digest facts stay at the accepted physics-source trust level, with Rust replay as a validation audit.

## 5. Separate combination coverage

`combinations[]` has exactly one entry per invocation model combination in authored order, separate from numerical_quality.cases. Every combination-basis result id belongs to exactly one entry's `result_ids` (result publication order); sets are disjoint and exhaustive. Entries hold `{basis_ref:{ref_type:"combination",ref_id},expression,disposition,result_ids,diagnostic_refs}` plus branch members below.

`expression` is `{kind:"mechanics",terms:[{case_id,factor:Bits}]}`, `{kind:"result_state_subtraction",minuend_id,subtrahend_id}` or `{kind:"range_envelope",operand_ids:[sorted ids],mode}`. Derived from the actual invocation, preserving product ordering and meaning. Mechanics terms keep repeats, zeros and factor bits; do not infer source equivalence from result ids.

| disposition | Additional members and rule |
|---|---|
| `retained_selected` | `method`, `run`, `source_binding`, `source_identity_sha256`, `selection`, `operand_cases:[{case_id,status,source_identity_sha256}]`, `cache_inputs` (below). Only same-source-compatible mechanics use this branch. Own solve/certification/classes; no maxima or intensified rows. |
| `retained_unavailable` | `reason`, `run:null|Run`, `operand_cases`, `cache_inputs`; no token on published rows. Whether unchanged ordinary rows can survive is an explicit routing transaction decision, never implicit omission. The ordinary envelope is the availability baseline. |
| `ordinary` | `reason:"no_retained_mechanics"`; no run/token/class payload. Existing subtraction/range and ordinary-only mechanics keep their base contract; this is coverage, not a W1 certificate. |
| `base_withheld` | `reason` must equal the existing base gate/reason and corresponding diagnostic; result_ids empty. W1 cannot invent a new base gate. |

`cache_inputs:[{operand_index:U,source_case_id,slots:[{slot:"s128"|"s256"|"s512"|"s1024"|"v256"|"v512"|"v1024",state:"success"|"nonbudget_failure",build_ref:{run_kind,run_index,record_index},work:U}]}]` is a bounded provenance inventory of actual snapshots at call entry, in operand order. It gives no access to private cache payload. No snapshot for a prepared ordinary source means an empty slots array. First occupied slot wins in that order, matching GroupCache::merged; newly built combination slots stay owned by that combination and are never retroactively inserted into operands. Build references can point only to actually incurred earlier builds with the same stiffness, slot, totals and failure, or (for independent historical API callers) remain unavailable for production invocation binding. F2a production uses only same-invocation origins.

Do not attach W1 classes by inheritance to subtraction/range outputs or pretend a range is an own solve. Existing base-algebra coverage and operand standing remain enforceable. Any requested numerical reliance enhancement for those ordinary combination rows needs its specified recipe/reader contract; it is not created by this receipt. Source-compatible mixed mechanics are addressed in SOURCE_COMBINATIONS.md.

## 6. Three readers, carriers and standing

All readers execute G0→G8 in the same order and consume one shared positive/mutation corpus. Schema validation and header admission alone never establish standing. Proposed new general failure codes below need ROOT's reservation alongside table/schema freeze; keep already adopted scale/section/classification/input-DOF codes unchanged.

| Gate | Exact obligation | Failure |
|---|---|---|
| G0 | Known identity/profile/table/inherited hash, corrected v2 and all registered policy ids/thresholds; refuse v1 relabels | existing unsupported-contract code |
| G1 | Closed shapes, receipt H(body), final publication H(envelope minus receipt), only checked canonical profile | `RETAINED_PRECISION_RECEIPT_MISMATCH` |
| G2 | Canonical finite Bits, safe integers, tagged sentinel locations, hashes; no -0 counter or NaN/Inf | `RETAINED_PRECISION_ENCODING_MISMATCH` |
| G3 | Case order/unique ids equals request and unchanged quality; separate exhaustive combination and row coverage; execution-order bijection | `RETAINED_PRECISION_COVERAGE_MISMATCH` |
| G4 | Exactly one selected/unavailable diagnostic per corresponding case, exact affected_refs=[case id]; analogous combination refs; no selected+unavailable for same basis, no SOURCE_BLOCK_RECOVERY_SELECTED anywhere, no source-unavailable naming a retained-selected case | `RETAINED_PRECISION_DIAGNOSTIC_MISMATCH` |
| G5 | Actual logical/native schedule, reuse links, all partition and stage equations, built provenance, 20B/60B checkpoint meaning, selected terminal and summaries; ordinary refs resolve | `RETAINED_PRECISION_ATTEMPT_MISMATCH` or `RETAINED_PRECISION_WORK_MISMATCH` before numerical checks |
| G5a | Nonnegative finite summary bits and exact body/kind coverage; stop≤2^-64; estimate≤1/4; charge≤1; theta≤1/2; positive finite B only where applicable; resolution zero/sanity/lower tests from R7 §6.3 on normalized SI values, A3 coverage amendments | adopted `RETAINED_PRECISION_SCALE_MISMATCH` |
| G5b | Recompute bodies/L, SI normalization, original-max coupling, same E/ê/Φ at p512, section truth and k=1/upward sqrt(2)i/2sqrt(2)/4; exact scale bits | adopted scale/section mismatch codes |
| G5c | Exact absolute/not_covered sets with no overlaps, no InputDerived/NonQuantity in either; R bits, small-scale A1 bound, pressure/member rules; rederive restrained DOFs with hanger rules | adopted class/input-DOF mismatch codes |
| G6 | Every selected case or retained-selected mechanics-combination row has exactly the method token; no ordinary/unavailable row has it; no private radius field | `RETAINED_PRECISION_ROW_METHOD_MISMATCH` |
| G7 | Project away only receipt, bound method evidence, and bound identity/profile/policy constants; run unchanged preview or exact base evidence validator, including T0R combination gates, magnitude/identity/reference constraints | existing base failure codes |
| G8 | Actual raw invocation+mode hash, project/model ids, source/maps/sections/material basis, case+combination expressions and ordinary eligibility; no 0.4 extension | `RETAINED_PRECISION_INVOCATION_MISMATCH`, or existing scope result |

G5 work checks use wider exact arithmetic independently in all three languages. Digest-bound reports are attestation, not independently reproduced solves; readers cannot prove a producer's actual cache history just from a self-authored ledger. Actual provenance, source-mapped implementation and Rust replay/mutation tests establish that trust boundary. A self-consistent forged hash never authenticates a producer.

Final publication uses actual normalized n: mm divide by 1000, kN/kN·m multiply by 1000, MPa multiply by 10^6; SI identity otherwise. Absolute b is RU64(2^-64 S), except 0<S<2^-988 uses upward `RU64(RU64(2^-64 S)+RU64(2^-53 |n|)+2^-1074)` and S=0 gives b=0. The facade must prove H_n≤b or both sharper relative allowances plus the decimal predicate; raw relative y also needs 10^9 H_U≤|y|. These certificates remain private; G5a is still an explicit producer preflight, not an automatic-pass theorem. No radius/report serialization.

Numerically eligible requires all applicable G checks with invocation, MECHANICS_SOLVED, requested load-case refs/order and each case selected or ordinarily eligible not_required, with applicable base combination standing. An unavailable case, missing invocation or scope yields needs_recompute; malformed evidence yields unsupported. A retained-unavailable published mechanics combination cannot gain reliance from selected operands. It needs explicit refusal through the existing basis/binding surface; if that cannot be represented without new standing semantics, use the preserved ordinary transaction and return the conflict to ROOT. Row classes do not alone demote the envelope: before S-I, absolute and not_covered rows/headlines refuse binding with RULE_QUANTITY_BELOW_VERIFIED_FLOOR / RULE_QUANTITY_NOT_COVERED. Relative/InputDerived keep the point route. Show b in SI around n, or outward-converted raw endpoints; never y±b_SI labelled raw units.

Rust `result_export` owns full G validation, standing, binding and derivative copy/disclosures; Python `analysis_runs` independently performs identical checks; TS performs async full validation during direct/job registration, bound to immutable actual request/mode/result fingerprints, and caches only that validation result for synchronous standing. Adding an id to sourceContract/FRESH_CONTRACT_IDS is insufficient. AnalysisRun copies the complete receipt; save/reopen validates it. Canonical derivatives copy it and all absolute/not_covered disclosures. Transport without raw rows executes only G0–G2 plus existing base metadata checks and is never eligible. Schemas add explicit successor branches in results, AnalysisRun and stress-neutral carrier; that schema support does not lift the T6 desktop/stress-neutral export refusal.

Independent review of this wire/source return and the reconciled routing/certificate/memory interfaces precedes any maintained implementation. No producer-only merge follows.
