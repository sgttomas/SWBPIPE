# C3 — proposed prepared ordinary formation and receipt delta

**PROPOSED, NOT RESERVED OR INSTALLED.** This supplements corrected C1/C2; it does
not duplicate their native records, source maps, routing or carrier contracts.
TASK I52, same Type 2 under ROOT /root. Final source: CODE `922db9dce3`.
P = projects/chirality-piping; FK = P/core/solver/frame_kernel/src/structural/retained;
PP = P/core/product_physics/src. Line numbers below are at that immutable source.

## 1. Definition, identities and exact applicability

[DEFINITION.json](DEFINITION.json) is the exact proposed canonical registered
definition `RP-PREPARED-ORDINARY-DUAL-v1`, containing the preparation subdefinition
`RP-PREPARED-ANNULUS-v1`. Its contents bind fixed1024 preparation/proof/projection,
the exact pi limbs, operation order, both laws, same-draft source seed, row-family
formations, native-stop/floor scope and unchanged final predicates. Fixed facts
live there once. Receipt fields below hold actual varying evidence.

Proposed maintained home:
`P/fixtures/results/retained_precision_prepared_ordinary_v1.json`. The two pending
C1 successor tables retain their own identities/base hashes; the preview successor
table pins this definition id and domain hash in a closed
`product_formation_definitions:[{id,sha256}]` member. The exact successor table
does **not** claim this ordinary-only definition applies to exact-profile E/nu.
The shared facade policy `RP-FACADE-SI-v2` dispatches by the table-bound definition
and retains C1 native projection/work policies and method token. This is a scoped
addition; implementing promised exact-profile and source-compatible combination
routes remains necessary for full F2a. It grants no omission or availability exception.

| Identity/domain | Proposal |
|---|---|
| Existing C1 successor ids/profiles, `RP-LOGICAL-ATTEMPTS-v1`, `W1-LME-20B-60B-v1`, `RP-FACADE-SI-v2`, method `contribution_preserving_multiprecision_v1` | Reuse proposed spellings; still require ROOT reservation and atomic table/schema freeze. |
| Kernel `M03-INTEGRITY-MP-v2`, checked canonical profile and native K4 byte domains | Reuse actual existing meaning; no policy renaming or byte-domain change. |
| `RP-PREPARED-ORDINARY-DUAL-v1`, nested `RP-PREPARED-ANNULUS-v1` | New proposed names, no installed occurrence found in final maintained roots. |
| `retained_precision_formation_v1` | New H domain for the definition object. |
| `retained_precision_preparation_v1` | New H domain for the semantic preparation payload defined below. |
| C1 `retained_precision_receipt_mp_v2`, `retained_precision_publication_mp_v2`, `retained_precision_source_mp_v2` | Reuse proposed domains, with C3 covered payloads; no installed historical receipt is reinterpreted. |

Fresh collision results distinguish internal placeholders from public installation:
`contribution_preserving_multiprecision_v1` exists as adaptive.rs:57 METHOD_TOKEN,
whose source comment explicitly calls it a ROOT placeholder. `M03-INTEGRITY-MP-v2`
exists as the actual corrected kernel policy. The other searched C1/C3 proposed
public names have no maintained occurrence; historical execution-record mentions
are proposals, not registration. MANIFEST.json preserves the exact matches.

Use C1 H(d,p), checked canonical JSON of {"domain":d,"payload":p}, SHA256.
The definition file itself is compact sorted-key UTF-8 JSON without trailing
newline. All keys are ASCII, numbers exact small integers, and it contains no
floating JSON numbers; its sorted-key serialization is JCS-compatible.
Raw-file SHA256 and H-domain SHA256 are distinct and recorded in MANIFEST.json.
No runtime reads dated execution records. Rust embeds the maintained definition;
Python and TS consume the identical packaged JSON and table hash.

## 2. Added/changed closed fields

C1/C2 types U, B, H, Text, OwnerRef, SourceRef, RunRef, Dof and the existing error
unions retain their meanings. `SB` is finite binary64 bits including signed zero,
used only to preserve an actual conversion result; `B+` is finite nonnegative
bits with canonical +0. `P5=[B;5]` means [A,I,J,Z,c].
All objects below are closed; every listed member is required, even if null.
No ellipsis is a field. Array lengths/copies require the existing admitted counts.

| Containing object | Exact added member |
|---|---|
| C1 body | `product_attempts:[ProductAttempt]` in actual start order; empty if none. |
| Every C1 case entry | `product_attempt_ref:null|U`; null iff no C3 attempt entered. not_required requires null. |
| Every C2 CaseSource | `preparation:null|{attempt_ref:U,sha256:H}`; null for an unprepared source. |
| C2 CombinationSource | No new member; it already binds operand CaseSource identities. C3 grants no prepared-combination proof route. |
| Existing selected selection | Native p/P/stop/floor and operational section fields retain their names and native meanings. No proof1024 is written into p/P. |

```text
ProductAttempt = {
 id:U, definition_id:"RP-PREPARED-ORDINARY-DUAL-v1",
 owner_ref:{kind:"case",index:U}, ordinary_attempt_ref:U,
 material_basis_ref:U, source_ref:null|SourceRef, run_ref:null|RunRef,
 result: Ready | Unavailable,
 preparation:{members:[PreparedMember]},
 stages:{preparation:Stage,native:Stage,proof_start:Stage,projection:Stage,
         maxima:Stage,values:Stage,aliases:Stage,certificate:Stage,
         observables:Stage,g5a:Stage},
 proof:null|ProofTrace,
 adapter:AdapterTrace, operational:{old:[MemberOperational],new:[MemberOperational]},
 overlay_work:ScalarTrace, g5a_work:ScalarTrace
}
Ready = {kind:"ready"}
Unavailable = {kind:"unavailable",error:PublicFailure}
Stage = "not_entered" | "completed" | "failed"
PreparedMember = {
 member:U, old_source:[B;6], old_facts:[B;7],
 result:{kind:"prepared",section:P5} | {kind:"refused",error:SectionError},
 work:PreparationWork, conversions:[PreparationConversion]
}
PreparationConversion = {property:"area"|"second_moment"|"polar_moment"|"section_modulus"|"radius",
 endpoint:"lo"|"hi"|"exact",outcome:Conversion}
Conversion =
 {kind:"normal",value:SB} |
 {kind:"subnormal",value:SB,relative_precision:B+} |
 {kind:"underflow",negative:bool} | {kind:"overflow",negative:bool}
MemberOperational = {
 member:U,inputs:[B;10],
 result:{kind:"ready",length:B,axial_stiffness:B,torsional_stiffness:B,normalization:[B;3]}
       |{kind:"refused",error:OperationalError},
 work:ScalarTrace
}
ProofTrace = {
 lanes:[Lane], numeric:NumericTrace, comparisons_lme:Count,
 visits:Count, scalar_operations:Count, projection_conversions:Count,
 projection_outcomes:[{row_index:U,outcome:Conversion}],
 capacities:[U;7], prepared_capacity_bytes:[U;6],
 completion:{kind:"not_entered"} | {kind:"merged"}
           | {kind:"separate_failure",visits:Count,capacity_bytes:U},
 checks:{certificate:Check,observables:Check,g5a:Check}
}
Lane = {
 law:"admitted_k"|"annular_source",state:"completed"|"failed",
 error:null|BridgeError,work:LaneWork
}
Check = {kind:"not_entered"} | {kind:"passed"} | {kind:"failed",error:PublicFailure}
```

The old-source tuple is [E,G,A,Iy,Iz,J]; old-facts is [D,t,A,I,J,Z,c].
These are **actual retained old operands**, not recomputed old geometry.
Keep this compact direct source tuple; do not store an entire old Source, native
Run or duplicate map. C2 maps/section terms store the actual new operands.
PreparedMember order is canonical native member order, mapped to C2 full ids.
Only entered helper calls create entries: successful prefix, then at most one
refused entry. A failure before a helper has an empty/no-new-entry prefix, never
a fabricated failed zero-work conversion. Production capture must retain the
failed member's input/old tuple before helper entry; final source currently keeps
the successful association only.

For a successfully constructed new source define semantic payload exactly as:

```text
{
 definition_id:attempt.definition_id,
 definition_sha256:the table-bound H(definition),
 owner_ref:attempt.owner_ref,
 ordinary_attempt_ref:attempt.ordinary_attempt_ref,
 material_basis_ref:attempt.material_basis_ref,
 members:attempt.preparation.members.map(
   {member,old_source,old_facts,section:result.section})
}
```

Every member must be prepared and coverage complete before this hash exists.
CaseSource.preparation.sha256 = H(preparation domain, this payload).
CaseSource.preparation.attempt_ref = attempt.id and attempt.source_ref = that
CaseSource.index. C2's source identity continues to hash the exact source union
without its own index, **including this preparation reference/hash**. The semantic
preparation payload excludes source_ref/run_ref/work/hash fields: no hash cycle.
Receipt H(body) covers work, prefix data and all references. A changed old/new
association changes preparation and source identities; a changed work count
changes receipt identity without pretending the mechanical source changed.

Required equalities: old E/G = C2 selected material/new member E/G; old-facts D/t
= normalized actual request and preparation inputs; old A=old-facts A,
old Iy=old Iz=old-facts I, old J=old-facts J; prepared [A,I,J,Z,c] =
new C2 operational [area,I,J,Z,radius], new native A/Iy/Iz/J=[A,I,I,J].
Operational.inputs are [xi.xyz,xj.xyz,E,G,A,J], and their results match C2
length/axial_stiffness/torsional_stiffness. All unchanged maps, frame, individual
terms, prescriptions and support membership remain bound as C2 requires.
If source construction fails, source_ref is null, no CaseSource/preparation hash
is invented, and existing C2 source_decline retains the actual constructor error.

Case product_attempt_ref resolves once, agrees with owner, ordinary attempt and
material basis, and (when present) Run.origin owner/source. At most one C3 attempt
per owner and one proof start; C3 does not authorize retries. run_ref is null iff
no native call happened; it may reference a native selected run even when product
result is unavailable. Native work stays solely in that C1/C2 Run.

Lanes are an ordered entered prefix of [admitted_k,annular_source]. Missing means
not entered, never zero work. An entered failed lane remains with its cause/work;
no later lane follows a failed K. Two completed lanes are required for projection.
AnnularSource uses this same attempt's completed K seed; no unbound public seed.
Readout intervals/alpha/epsilon remain private. No lane or proof rerun is inferred
from final row values or an Accepted native record.

row_index addresses this case's **final** envelope rows in publication order
(after id qualification), derived from basis_ref; no separate row copy is added.
Successful projection outcomes cover exactly hull-projected rows (including
prescribed zero), excluding observed ancillary, support norms and maxima.
Failure retains the actual entered prefix. The conversion count equals outcome
count whenever exact; Normal permits normal or exact ±0, Subnormal requires actual
nonzero subnormal bits and the recorded conversion relative-precision metadata.
That metadata is not a final row error radius. Underflow retains its sign even
though actual product zero canonicalizes to +0; all final error tests remain.
For Ready, each outcome maps to that final row's raw value bits: Normal/Subnormal
preserve value except zero canonicalization, Underflow requires +0, and Overflow
is impossible. Unavailable attempts keep tentative outcomes without comparing
them to the preserved ordinary fallback rows. A complete prepared member's
endpoint outcomes equal its section bits; an ambiguous-rounding failure preserves
the distinct positive normal endpoint bits rather than inventing a section.
Preparation success has nine conversions, ordered A.lo/A.hi/I.lo/I.hi/J.lo/J.hi/
Z.lo/Z.hi/c.exact, with matching positive normal endpoints; failure keeps its prefix.

Stage values must come from entered/returned transitions, not expected control
flow reconstructed afterwards. Final source can run observables/G5a after a
numeric certificate refusal; retain those actual independent outcomes without
promoting the refused product. Ready requires every listed stage completed and
every proof check passed. Native stage completed means its actual selected Run;
native nonselected return makes that stage failed and is referenced, not copied.
A case becomes publicly selected only after C1 finalization, not merely Ready.

## 3. Actual work, status and precise error maps

```text
Count = {kind:"exact",value:U} | {kind:"unavailable",fault:"overflow"|"inconsistent"|"both"}
NumericTrace = {wide_lme:Count,exact_sum_lme:Count,
 entries:[Count;7],f64_arithmetic:Count,sticky_status:Status}
Status = "exact"|"overflow"|"inconsistent"|"both"
ScalarTrace = {entered:U,checks:U,lost:bool}
PreparationWork = {numeric:NumericTrace,initialized_endpoints:Count,
 conversions:Count,checks:Count,endpoint_assignments:Count,layout_bytes:[U;8]}
LaneWork = {numeric:NumericTrace,point_lme:Count,
 view:{visits:Count,f64_operations:Count,prescribed_capacity:U,data_capacity:U},
 correction:{cast_lme:Count,factor_lme:Count,visits:Count,calls:Count,
             rhs_capacity:U,output_capacity:U,converted_capacity:U},
 visits:Count,member_builds:Count,frame_builds:Count,b_products:Count,d_products:Count,
 h_products:Count,capacities:[{name:CapacityName,capacity:U}],data_capacity:U}
CapacityName = "center"|"eta"|"residual"|"alpha"|"epsilon"|"midpoint"|"rows"|"reaction"
AdapterTrace = {counts:[U;10],fault:null|{kind:"overflow",event:AdapterEvent},
 prepared_capacity_bytes:[U;16],observation_capacity_bytes:[U;3],
 support_capacity_bytes:[U;7]}
AdapterEvent = "source_visit"|"row_visit"|"map_write"|"validation_entry"|
 "identity_byte_read"|"key_probe"|"allocation_request"|"library_boundary"|
 "requested_copy_bytes"|"rust_capacity_bytes"
```

Numeric entries order: add/sub/mul/div/r4/b64_up/sqrt. Use WorkTotal.exact()/status(),
never its legacy saturated value or Debug. Numeric status joins all its counts
and the actual NumericWork.status field exported as sticky_status;
lane status is the join of its counts. Ready requires exact statuses, no adapter
fault and no lost scalar collection. Scalar/adapter numeric arrays preserve their
actual retained prefix even when their separate lost/fault bit is set; they are
then not represented as an exact total. Values above safe-U range cause existing
whole-receipt receipt_encoding fallback; no truncation or invented zero.
Adapter counts are the cumulative prefix of the actual owning ProductCapture,
including its ordinary observation/capture entries. Old OperationalSpent work is
the actual earlier evaluator work. Neither is relabelled as an independent new
charge. Public multi-case caller integration must establish their actual owners
and prevent duplicate accounting; C3 does not sum these snapshots into a budget.

Proof capacities are element counts for laws/native-coverage/represented-intervals/
verdicts/derivative-coverage/support-slots/scales; Vec<bool> capacities are logical
boolean elements, not bytes. Prepared proof capacities are already bytes for
values/descriptor rows/maxima/completion flags/anchor/conversion records. Lane
capacity entries keep observed order and repeated residual entries, max9; unused
array slots are not emitted. View and correction capacities are typed element
counts. Preparation layout order is SectionPrepFrame, Endpoint, NumericWork,
WideContext<16>, ExactWideSum, PreparedAnnulus, Binary64Outcome sizes, then frame
alignment. These are actual local observations, **not** aggregate memory admission.

Proof visits/completion capacity already include successful ValuesCompletionWork
after certify_final or abandon_values. Emit completion.kind=merged without a
second additive count then. On ProductValuesFailure, emit separate_failure from
its own visits/capacity and retain earlier proof work separately. Never double
count either branch. No public all-in sum or priced product debit is introduced.

All errors below are tagged objects using `{kind,...payload}`; absent payload
means exactly {kind}. Text is the original typed String/&str payload, never Debug.
Imports C2 AttemptStop, SourceError and WideError retain their closed mappings,
with current AttemptStop CountRange{name:Text} and WorkAccounting{fault:Status
excluding exact} explicitly included. Out-of-range/native-unencodable payload
takes C1 receipt_encoding; private full error remains preserved.

| Public union / exact tags and extra members | Actual source |
|---|---|
| SectionError: invalid_geometry; ambiguous_rounding{property}; primitive_range{property}; arithmetic{cause:NumericError}; accounting | product_certificate.rs:610–615. property is area/second_moment/polar_moment/section_modulus/radius, direct index0–4 map; no catch-all. |
| NumericError: arithmetic{cause:AttemptStop}; non_finite; nonpositive_source; invalid_geometry; invalid_material; material_bits; temperature_order; nonpositive_denominator; axis_bits; binary64_range | product_certificate.rs:103–115. |
| BridgeError: view{issue:ViewIssue}; numeric{cause:NumericError}; member_owner; unsupported_directional_spring; missing_radius{row:U}; missing_uniqueness_warrant{body:U}; row_identity{row:U}; count_range; storage; alpha_condition{block:U} | bridge.rs:23–35. Alpha's private alpha_hi Endpoint is deliberately not serialized or rounded to B; this names the required strict alpha<1 / positive one-minus-alpha condition and block, covering both actual return sites. Full original endpoint stays internal. It is not a public bound/radius or a lossless numeric dump. |
| ViewIssue: certificate{issue:CertificateIssue}; foreign_owner; unsupported_combination; verification_cache; ordering; body_bound; count_range; work{fault} | adaptive.rs:5249–5260. CertificateIssue exact tags shape/pair_identity/precision/row_identity/missing_field/negative_field/non_finite/non_canonical_zero/radius_class_mismatch; enum139–149. |
| ProductFailure: association{detail:Text}; numeric{cause}; native_source{cause:BridgeError}; work_accounting{fault}; count_range{detail:Text}; storage; g5a{detail:Text}; numeric_predicate{row:U,predicate}; numeric_helper{cause:HelperError} | final_case.rs:165–197. Predicate is absolute/sharper_exact/sharper_binary64/decimal_si/decimal_raw/input_derived. HelperError is arithmetic{cause:AttemptStop}/invalid_small_bound_input/binary64_range/invariant. |
| OperationalError: missing_or_foreign; input; degenerate; non_finite{operation,entered:U}; coefficient_range{coefficient:Text,operation}; accounting | PP retained_product.rs:2285–2298. operation add/sub/mul/div/sqrt; coefficient retains actual typed string, currently EA/L or GJ/L. |
| OriginError: count_range{detail:Text}; capacity; allocation; missing_selected_origin{operand:U} | origins.rs:17–22; missing_selected_origin remains outside this case-only prepared definition but preserves current enum mapping. |
| CaptureError: association{detail:Text}; count_range{detail:Text}; storage{detail:Text}; accounting{event:AdapterEvent}; source{cause:SourceError}; origin{cause:OriginError}; native_unavailable; prepared_arithmetic{cause:OperationalError}; prepared_proof{cause:ProductFailure}; prepared_attempt_consumed | PP:67–78. Original string errors stay strings; readers never interpret their prose as proof/trigger. |
| G5aError: accounting{event:AdapterEvent}; shape{detail:Text}; summary{detail:Text}; zero{row:U}; sanity{body:U,kind:U}; lower{member:U,kind:U}; operational{member_index:U,cause:OperationalError}; arithmetic{cause:OperationalError} | PP:2473–2492,2623–2635,2640–2672,2715–2744. kind is exactly0/1 for the force/moment resolution checks. Operational's member_index preserves native member ordinal; Lower's member is the native member id. |
| PublicFailure: preparation{capture:CaptureError,section:null|SectionError}; native{run_ref:RunRef}; capture{cause:CaptureError}; proof{cause:ProductFailure}; values{cause:ProductFailure,proof:ProductFailure}; abandoned{cause:CaptureError,proof:ProductFailure}; numeric{cause:null|ProductFailure}; observable{cause:CaptureError}; g5a{cause:G5aError} | PP PreparedCaseFailure and PreparedCandidateError:3112–3120,3285–3286. Observable/G5a unwrap their actually captured errors; Numeric preserves the actual optional numeric_failure, including null. No synthesized reason. |

Failed check errors use the appropriate PublicFailure wrapper: certificate
proof{cause}, observables observable{cause}, G5a g5a{cause}. Observable/G5a require
their actual captured cause, because their source branches test it before return;
unexpected absence is an encoding/association defect and ordinary fallback.
The payload-free Rust Numeric variant is different: its captured numeric_failure
may be absent and the proposed wire states that honestly as null. Readers do not
infer a missing proof failure from the name. All nested payloads are finite/size-admitted.
Bridge missing_radius/row_identity indices address C2 native layout; ProductFailure
predicate.row and G5a zero.row address the derived final case row order. They are
not interchangeable. Body/block/member/operand identities retain their actual
source index domains. Work fault payloads exclude Status="exact".

## 4. Reader ordering, trust and D2 cross-reference

Keep C1 order G0,G1,G2,G3,G4,G5,G5a,G5b,G5c,G6,G7,G8. Within a gate use this
deterministic order, then ascending attempt/member/lane/row index. First failure
wins. Schema shapes remain G1 before encoding G2, as C1 specifies.

| Gate | New check and first-failure code |
|---|---|
| G0 | Known table/definition id and pinned H, allowed table/scope dispatch. Unknown definition uses existing unsupported-contract outcome. Known definition with wrong fixed contents/hash: proposed RETAINED_PRECISION_FORMATION_MISMATCH. |
| G1 | Closed C3 objects; preparation/receipt/publication/source hash integrity. Use existing proposed RETAINED_PRECISION_RECEIPT_MISMATCH. |
| G2 | U/SB/B/Count/enum/tuple representation, conversion tag/value range, counts encoding. Use RETAINED_PRECISION_ENCODING_MISMATCH. |
| G3 | Case→attempt bijection, attempt id/order/owner, member-prefix and row-index coverage. Use RETAINED_PRECISION_COVERAGE_MISMATCH. |
| G4 | Existing selected/unavailable diagnostics; unavailable cannot carry selected method. Unchanged RETAINED_PRECISION_DIAGNOSTIC_MISMATCH. |
| G5 | First existing native schedule/origin checks; then C3 run/source/ordinary references and allowed stage/lane sequence; then typed check/result consistency; then work/status/conversion-prefix/merge equations. New proposed RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH for C3 execution association; existing RETAINED_PRECISION_WORK_MISMATCH for work. Native p/P/stop stay native. |
| G5a/b/c | Existing exact summaries, final SI scales/section values/classes/absolute/NotCovered/DOF checks. New prepared operational fields feed existing section mismatch checks; p1024 proof cannot trigger p512 floors. Existing adopted codes. |
| G6 | Existing per-row method scope, including ancillary rows of a selected case. Unchanged ROW_METHOD code. |
| G7 | Literally unchanged preview/physics base projection; do not erase sections, changed row values or old recipe failures. Preserve support norm guards and actual coefficient maximum/midpoint/coverage. |
| G8 | Existing raw invocation/mode/project/material/order/family/pressure checks, then independently normalized D/t/E/G and unchanged maps; then old↔new tuple/C2 operand equalities and semantic preparation binding. Existing invocation failure for raw custody; proposed RETAINED_PRECISION_PREPARATION_MISMATCH for a validly encoded, rehashed but false association. Unknown/excluded scope retains C1 standing rule. |

Cross-language G8 must rederive selected preview independent E/G and their actual
base/point/interpolation operands with current unit/selection semantics. Exact
profile retains the existing selected common E/nu/G promise. G8 does not recompute
old binary64 A/I/J/Z; G5b reads actual operational fields. Reusing physics_source's
narrow authored-material helper must not call its historical stress recipe.
Preparation rounding, actual execution/lane work and direct certificate remain
digest-bound producer attestations audited by source review/independent Rust replay,
not three-language proof replay or authenticated producer origin. Rehashed semantic
mutations need equality/coverage checks; changing all self-authored facts consistently
cannot be disproved merely by their unkeyed hashes.

Proposed exact successor cross-reference text (add to the registered definition/
successor table documentation; **do not edit sealed D2 bytes**):

> For RP-PREPARED-ORDINARY-DUAL-v1 only, D2 §4.9.10's “formed at p” descriptions and
> the related historical §5 I-9 description at line934 are
> replaced by the registered row-family formation: fixed1024 dual-readout projection
> for displacement magnitudes and actual binary64 component hypot for support
> magnitudes. Existing k=1, scales, classes, predicates and observable checks remain.
> Native p/P and stop entries concern native q_p/q_P only. For these final rows,
> D2 §4.11.2's historical operational-convergence description of the basis of b
> is replaced by the direct final-certificate warrant against both required
> readouts, using the same b and unchanged acceptance inequalities. This does not
> activate S-I or change pre-S-I quantity/headline binding refusal. Other routes
> and historical receipts retain their original warrants and meanings.

This includes RV69-N1's §4.11.2:741 and §5 I-9:934 finding; no interval-binding grant,
larger b, source-meaning choice or public radius follows.

## 5. Minimal examples, actual ownership and required seams

The examples below are **symbolic proposed shapes, not producer evidence**. Bind
C,O,M,S,R,Q to an already valid C1/C2 fixture's case, ordinary attempt, material
basis, newly prepared CaseSource, actual native Run and complete final row list.
A is the next actual ProductAttempt index. PM,OPold,OPnew,AD,OV,PK,PS,N,Cmp,V,Sc,
Cap,PCap,Conv,GW are actual typed snapshots specified in §2–3. GW is the actual
ProductCapture.g5a_work ScalarWork. Hprep is the computed
semantic preparation digest, not an arbitrary placeholder accepted by a reader.
The complete referenced C1/C2 fixture and these snapshots are required dependencies.

```text
selected case delta: product_attempt_ref=A
CaseSource S delta: preparation={attempt_ref:A,sha256:Hprep}
ProductAttempt A = {
 id:A, definition_id:"RP-PREPARED-ORDINARY-DUAL-v1",
 owner_ref:{kind:"case",index:C}, ordinary_attempt_ref:O,material_basis_ref:M,
 source_ref:S,run_ref:R,result:{kind:"ready"},
 preparation:{members:PM},
 stages:{preparation:"completed",native:"completed",proof_start:"completed",
 projection:"completed",maxima:"completed",values:"completed",aliases:"completed",
 certificate:"completed",observables:"completed",g5a:"completed"},
 proof:{lanes:[{law:"admitted_k",state:"completed",error:null,work:PK},
               {law:"annular_source",state:"completed",error:null,work:PS}],
 numeric:N,comparisons_lme:Cmp,visits:V,scalar_operations:Sc,
 projection_conversions:{kind:"exact",value:len(Conv)},projection_outcomes:Conv,
 capacities:Cap,prepared_capacity_bytes:PCap,completion:{kind:"merged"},
 checks:{certificate:{kind:"passed"},observables:{kind:"passed"},g5a:{kind:"passed"}}},
 adapter:AD,operational:{old:OPold,new:OPnew},overlay_work:OV,g5a_work:GW
}
```

Failed-prefix shape uses the same dependencies through new native-selected R.
K completed one correction; Source entered and failed ViewIssue.body_bound
after its actual view allocations. It therefore has **two** lane entries, keeps
PK and PSfail (including view capacities and zero source correction calls), and
never starts projection. Replace the selected shape's result by
`{kind:"unavailable",error:{kind:"proof",cause:{kind:"native_source",cause:{kind:"view",issue:{kind:"body_bound"}}}}}`;
stages proof_start=failed and projection/maxima/values/aliases/certificate/
observables/g5a=not_entered; lanes second={law:annular_source,state:failed,
error:{kind:view,issue:{kind:body_bound}},work:PSfail}; completion=not_entered;
checks all not_entered; projection count exact0/outcomes[] and their unallocated
capacities0. N/Cmp/V/Sc/Cap/PCap/AD/OV/GW are the actual failed-prefix snapshots,
**not copied from success or assigned zero wholesale**. Case status is unavailable,
its Run remains genuinely selected, and ordinary rows/quality remain unchanged.
If no other case selects, C1 returns the base ordinary envelope and preserves
this internal trace rather than emitting an empty-success successor.

| New field family / producer owner | Missing narrow seam and decisive shared mutation |
|---|---|
| Preparation old_source/old_facts/section | PP PreparedAssociation:3093–3100 exists. Capture failed-member tuple before prepare call, preserve wrapper cause. Mutate old J while keeping new source unchanged and rehash all affected digests. |
| Nine preparation conversion outcomes + PreparationWork | FK SectionPreparationWork:617–659 counts calls but drops outcome; retain actual ordered outcome before match, expose typed NumericWork snapshot. Mutation omits a failed conversion or swaps property/end. |
| New source/ref/operational | PP prepare_owned_case:3125–3218 and evaluate_operational:2416–2470; bind into C2 registry/native origins without reserializing old source. Mutation copies old operational J or old/new source digest. |
| Proof/lane/stage causes and counters | FK final_case:208–288 and source_residual:48–119 currently partly private/Debug-only. Add consuming/borrowed typed read-only summaries and actual lane-terminal capture before extraction, without exposing endpoints. Mutation deletes K-only/Source-failed work or swaps run/seed law. |
| Projection outcomes / completion merge | final_case:1545–1572,1624–1630,1712–1720 already retain actual conversion and completion branches. Typed serialization maps exact variants; mutation turns underflow into normal-zero or double-adds completion work. |
| Adapter/old/new operational/overlay/check errors | PP ProductCapture:80–131, ScalarWork:2300–2399, PreparedCandidateError:3285–3302, project_candidate:3405–3499. Replace diagnostic-string access with typed captured snapshots; no Debug parser. Mutate same-source foreign ordinary owner or lost/fault flag. |
| Maxima and support formation | final_case:1643–1696; PP prepared_maxima:3336–3384. Definition dispatch + inherited base checks; mutate support hull norm, wrong support slots, old f5 stress, stale maximum midpoint/headline, or proof1024→native floor. |
| Receipt definition/carriers/reader cache | Planned retained_receipt and three successor modules plus existing carrier hooks. Drop definition/attempt association during AnalysisRun copy or save/reopen; require same first failure in all readers. |

Exact maintained delta additions to I30's already proposed package:
`P/fixtures/results/retained_precision_prepared_ordinary_v1.json`;
`P/core/solver/frame_kernel/src/structural/retained/product_certificate.rs`,
`.../product_certificate/final_case.rs`, `.../product_certificate/source_residual.rs`,
`P/core/solver/frame_kernel/src/structural.rs` for necessary closed exports, and
`P/core/product_physics/src/retained_product.rs`.
No native arithmetic/factor/cache algorithm change is implied. Read-only mappings
of existing origins/adaptive/work types suffice unless implementation demonstrates
a specific missing path. Existing planned `retained_receipt.rs`, three successor
reader modules, tables/schema/shared-corpus/carrier files receive the C3 changes;
the concrete expanded manifest records their exact names. Parent routing and
resource owners retain their scopes. No additional host tool/library is proposed.

## 6. Selection boundary and remaining qualification

No new owner-reserved numerical meaning decision is demonstrated. The proposed
definition realizes ROOT's selected dual-readout ordinary process and RV69's
warrant clarification. ROOT must independently confirm and reserve this **concrete**
definition/delta before public implementation. Exact-profile/combination formation
completion, full invocation/work/resource/caller admission, both-entry/coexistence/
pressure/ordinary-availability controls, native Current and final review/CI remain.
This contract is not an all-in accounting proof or public milestone pass.

Typed source accessors/capture gaps above are implementation work, not permission
to populate unavailable facts. If exact registered error/counter encoding cannot
be obtained, use the existing ordinary transaction/receipt-encoding refusal while
preserving spent evidence; no fabricated receipt, new refusal policy or public
availability exception is selected by this proposal.
