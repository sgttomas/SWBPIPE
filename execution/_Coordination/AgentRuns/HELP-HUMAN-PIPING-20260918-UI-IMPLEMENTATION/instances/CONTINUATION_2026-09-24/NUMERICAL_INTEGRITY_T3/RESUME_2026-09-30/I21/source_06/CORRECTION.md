# source_06 — RV30 correction and finite retained-prefix roster

TASK I21, parent WORKING_ITEMS recovery manager. Start 2026-10-01 09:50:57 UTC;
boundary 10:10:57 UTC. Source-only, frozen A1
40129a225d73860ac2a53da9a2fa73869df668f3. This additive packet overrides only
the equations/identity interpretation identified below in source_05.
source_05 remains sealed and is **not accepted as a closed ledger**.
All application terms remain A1-sensitive.

Aliases and shape/capacity operators are source_05's: K=retained source
directory; A=K/adaptive.rs; N,m,s,d,r,l,t,u,B,b,n,f,z,h and q are finite source
descriptors. w=48,80,144 for L=4,8,16. P_16(B)=0 at B=0,
otherwise max(4,next_power_of_two(B)); G_16(B)=16*P_16(B).
E_16(B)=16B. Requested bytes and active growing-realloc movement are distinct.

## F1: one resolution identity, corrected everywhere

Name RES_v the buffer created by verify.rs:289-317 and bound at :777, where
v is the current verification precision in {256,512,1024}. Its borrowed
fallible Result<Vec<[f64;2]>> collect uses the source_02 lower-zero capacity
rule. It is not an exact-size success collect and cannot reuse borrowed top.
Its successful retained request is G_16(B), not16B.

Replace source_05's equations with:

    Pass*_L = 3n*w + (7f+C)*w + (q+6m)*w + 50n
              + PrescribedChildren + G_16(B)
    Report*_L = 344 + 5q*w + (f+C)*w
                + b*(BlockCertificate_L+BlockNorms_L+w)
                + B*BodyReport_L + G_16(B)
    C = P_w(f)

The block coefficients remain976/1584/2800. The body terms are now
624B+G_16(B), 1040B+G_16(B), 1872B+G_16(B).
The old640/1056/1888 times B expressions are superseded; they were too small
when P_16(B)>B. This is a local owner correction, not proof about a full-process
peak. No unrelated padding is used to repair the retained Report term.

| Edge / phase | RES_v ownership and accounting |
|---|---|
| resolution_scale -> pass | Move its one G_16(B) buffer into the local resolution binding (:777); top drops. |
| early pass, shift, later report construction | Same RES_v persists. Include it once; construction alternatives below add only their distinct current helper. |
| pass -> report | verify.rs:1193 moves RES_v into VerificationReport, no clone. Arc construction at A:3892 adds only the derived344-byte Arc request, not another resolution allocation. |
| report summary | A:2398-2410 creates distinct theta8B, bound16B and exact cloned resolution16B. Summary children remain40B. Do not turn this clone into G_16(B). |
| before-R7 canonical validation | Current Report* retains RES_v alongside the temporary G_20(q) layout. |
| R7 | Current Report* retains RES_v. R7's own infallible borrowed hats collect at A:2095 is a **different** exact16B buffer; it is not resolution_hats below. |
| certificate canonical validation / draft / H-RU | Report* retains RES_v alongside Decision and the source_05 certificate alternatives. The repeated layouts and drafts do not clone RES_v. |
| selected finish | Report* retains RES_v through fresh summary/evidence/Box construction. A:4210 adds another exact40B summary; selected-record clone copies that record's summary children exactly. |
| rejection / terminal return / next iteration | Current Report drops at the loop/return edge; RES_v does not enter cache or pending. Pending/state/cache Arc copies do not retain a report. |
| selected result / selected deep Clone | RetainedSolve has no report. Evidence contains its own summary-derived rows; it does not acquire G_16(B). Its exact summary/evidence clones remain as in source_05. |

For an explicit nonduplicating construction formula, set

    PassRest_L = Pass*_L - G_16(B) - (f+C)*w
    ReportBuild_R <= Kpad + PassRest_L + Report*_L
                     + CallerShiftControls + SpentRefusals

PassRest removes RES_v, r_hat and delta, because those three buffer identities
are already in Report*. The five report row buffers include the three early
row arrays; do not add another3q*w here. CallerShiftControls and SpentRefusals
are the existing source_05 finite start/data/s_refused/shifts and G_24(b)
owners. Report*'s344 bytes are explicit complete-result padding before Arc
creation; they are one allocation after creation. The construction phase's
active-grow surcharge applies only to the buffer growing then.

For earlier shift calculations retain Pass* + early3q*w + profile/factor/
controls, with the existing source_05 alternatives. After the pass closure,
PassRest and caller shift controls drop; Report* alone carries RES_v onward.
Every source_05 occurrence of current Report in K19/K20/canonical/certificate/
finish is overridden by Report*. Do not also add the local RES_v delta there.

## N1: explicit top and resolution_hats alternatives

These are heap buffers. Neither is absorbed into stack arithmetic or unnamed
future-vector padding. Kpad is defined by the finite roster below; it excludes
all pass/report/decision/certificate temporary owners.

Name TOP_v the exact vec! buffer at verify.rs:297, requesting2B*w.
Name HATCHECK_v the distinct borrowed fallible result at :340-360.
HATCHECK_v has retained G_16(B), and is discarded by the statement at :789.
TOP_v has already dropped before that statement. w_abs=n*w and e_rows=q*w
are the only pass vectors already created at these two helper calls
(:761-777); prescribed arrays, r_hat/delta and later full-DOF arrays come later.

Let O16(B)=0 for B<=4, otherwise8*P_16(B). This is the largest old request
on a possible growing realloc of this fresh16-byte borrowed fallible collect.
The first allocation is not a realloc. The following are direct phase uppers:

    Scale_R = Kpad + n*w + q*w + 2B*w + G_16(B)
    Scale_M = Scale_R + O16(B)

    HatCheck_R = Kpad + n*w + q*w + G_16(B) + G_16(B)
    HatCheck_M = HatCheck_R + O16(B)

The first G in HatCheck is retained RES_v; only HATCHECK_v grows. TOP never
overlaps HATCHECK. Output conversion failure may stop at a shorter prefix,
bounded by these B-count envelopes; partial output drops on Err. Failure of
the later finite-value check or HATCHECK also drops RES_v and e_rows/w_abs
when the pass returns Err; successful VerifyShared cache payloads can remain.

Take the maximum of these direct alternatives with formation_scale, later
Pass*, shift and ReportBuild alternatives. No simultaneous top+hats union and
no extra G_16(B) on all later phases. This resolves N1 without a domination
argument using unconstructed vectors.

## Finite Kpad roster for the original one-case schedule

K_phi is no longer an arbitrary runtime allocation-address input. Use the
following finite source-site roster. “pad” means deliberately retaining that
listed complete owner even before construction or after its actual drop;
this is a source-based upper, not a claim about actual overlap. A tighter
version uses the source control-flow slot indicators. No observed heap or
future allocation identities enter either version.

Original comparison calls use solve_case -> solve_cases with one source and
a fresh group/cache (A:4342-4408). The original caller's model/args/runtime
owners remain the separate H/VR caller term; they are not invented inside
Kpad. Source array/String capacity descriptors remain C1.

| Finite site ID | Slots / bytes or referenced finite equation | Actual lifetime / copy rule |
|---|---|---|
| SRC0 | One input PrimitiveSource child heap; source_05 K01 array/String/support-id/prescribed/body-index formula with input capacity descriptors | Source borrowed by solve_cases; lives through its return, then drops in solve_case. Its shallow stack header is not a heap allocation. |
| PREP | One Arc432 + exact-length source clone + retained ledger(48v+v+8J) + prescribed32r+16r + SRC encoding + G_20(q) layout +8B extents | A:908-977,4385-4386. Single-case factors Vec empty. Source clone is separate from SRC0; children reside behind the432-byte Arc payload. |
| GROUP | One Arc368 + geometry + pattern + ordering + free-block children | A:4310-4337. No additional Box per inline GroupCache or GroupPrep field. |
| GROUP.geometry | 24B + sum_c G_sigmaNS(k_c), k_c<=2N_c | Original geometry in GROUP; sigmaNS remains T2. |
| GROUP.pattern | 8(n+1)+A_8(z)+8z+8(z+1)+8U, U=78m+s+6d | Retained rowstarts/columns/transpose/starts/items; source_05 K05/K06 transient inputs are separate phase helpers. |
| GROUP.order | G_8(f)+8n+24f | Returned free/position/order/rank/first arrays as source_05 K07; adjacency/BFS/Qdeque are construction helpers. |
| GROUP.blocks | 4f+G_24(b)+sum_c G_8(f_c)+4b | Source_05 K08 returned component owners; DFS stack is construction helper. |
| CALL | GroupEntry1464 + Outcome Vec80 + STF encoding | A:4347-4370. GroupEntry contains inline1360-byte GroupCache; do not add another1360-byte heap. STF constructed before group. |
| GEOM_COPY | One 24B + sum_c E_sigmaNS(k_c) | Schedule clone at A:3944 moves into terminal/selected outcome. Group error/outcome clone paths reuse this two-copy upper with GROUP.geometry; they are not extra simultaneous three-copy geometry slots. |
| ATTEMPT_HEADERS | One G_832(4)=3328 Vec backing | At most four attempts. Current stack record moves into the Vec; no extra832-byte heap per local record. |
| STATE_HEADERS | One G_16(4)=64 Vec backing | At most four state handles. Candidate/verification/pending handle copies are shallow, stack/inline references to the same Solved Arc. |
| ATTEMPT_KIDS(v) | Three slots v=256,512,1024, each padded40B + G_24(2b) | At most one verification summary and refusal Vec per verification attempt. The summary's resolution clone is exact16B.128 is never verification. Generic shorter/error paths fit these padded counts. |
| S(p) | Four complete Shared_p equations from source_05: p=128:(L,R)=(4,4),256:(4,8),512:(8,16),1024:(16,16) | A:3603-3640 caches success even if own solve later fails. Arc720 included once per slot. Inline shared-error values have no added heap children. |
| U(p) | Four Solved_L=104+(n+6m+q)*w: p128/L4,p256/L4,p512/L8,p1024/L16 | Created only after successful own solve at A:3724-3731. At most one per precision. U128 and U256 are distinct payloads despite equal layout. |
| V(v) | Three complete VerifyShared_LW equations: v256=(4,8),512=(8,16),1024=(16,16) | A:3783-3832. Derived Arc560/592/656 included once. Shared/cache/consumer handles do not multiply payloads. |
| VE(v) | Three explicit padded cached failed-verification refusal clones, each E_24(b) | Non-budget failed build clones spent refusals into cache at A:3821-3827. Success V(v) and failure VE(v) in the same slot are alternatives in real control flow; padding both is declared conservative. Original spent refusal moves into its attempt slot; current pass-refusal transfer can have a separate G_24(b) transient. |

For the original one-source call define the fully padded persistent prefix:

    Base = SRC0 + PREP + GROUP + CALL + GEOM_COPY
           + G_832(4) + G_16(4)
    Kpad = Base + sum_{p=128,256,512,1024}[S(p)+U(p)]
                 + sum_{v=256,512,1024}[V(v)+40B+G_24(2b)+E_24(b)]

This is a finite substitution expression. It deliberately pads all four/three
slots, including mutually exclusive successful/failure storage and owners not
yet built. Thus it can be used without predicting numerical branch outcomes.
It is conservative, not tight. The unbound source capacity/tree/tuple/queue
parameters are the named existing cells, not arbitrary future owners.

For source/group/prep constructors and S/U/V construction, replace the
corresponding padded returned child by the source_05 construction envelope.
Equivalently retain its complete result early and add **only the named helper
extras not already in that result**. In particular do not add another whole
S/U/V allocation when building its roster slot. Add source_05 K03–K08
geometry/pattern/order/blocks helpers to the respective complete GROUP padding;
ledger build and extent scratch to PREP padding. This explicit early padding
also covers partial failure unwind, without asserting successful owners exist
on an error trace.

Finite current-phase owners and transfer aliases (outside Kpad except the
explicit SUMMARY_BUILD alias):

| Site | Maximum live copies / edge |
|---|---|
| PASS_v / RES_v / TOP_v / HATCHECK_v | One current verification pass, direct formulas above. Pass moves report children once. |
| REPORT_v | One current Report*, created only on pass success; not part of U or V caches. |
| SUMMARY_BUILD_v | One fresh40B summary during A:3893; **alias of the padded ATTEMPT_KIDS(v) summary slot**. Its theta/bound/exact-resolution children are already covered by that slot during construction and transfer; add zero extra40B to Kpad. |
| REFUSALS_SPENT_v | One G_24(b) transient while attempt refusal Vec extends (A:3889); input backing drops after transfer. Cached failed-build original instead moves into attempt. |
| DECISION_v | One source_05 Decision; R7 trackers/scales/skip/hats drop before certification; only decision summaries/floor escape. |
| CANON_v | One G_20(q) temporary at a time; pre-R7 and certificate calls are sequential. |
| CERT_v | One current publication draft/radius set; rejected/Err draft drops before next precision. |
| FINISH_v | One selected finish, with source_05 finite evidence/summary/selected-record-child clone/1976-byte Box terms. REPORT_v and DECISION_v survive through this call. |
| terminal outcome | Attempts/geometry move as applicable; Refused may discard attempts. PREP/GROUP/cache/state owners eventually drop, except any explicitly selected retained result. No extra heap is created by the fixed enum alone. |

The schedule proof is A:3936-4165: candidate failure increments c by1;
failed verification solve increments by2; successful rejected verification
moves into pending and becomes the next candidate without solving again.
Therefore each p has at most one solve; four attempt/state slots and three
verification slots suffice. verify_precision failure is terminal (:4014-4019).
A selected result from a fresh cache therefore has no previous failed V slot;
VE padding above remains safe without relying on that tightening.

Multiple-source invocations, arbitrary selected clones and combinations do
not silently fit this one-case prefix. Source_03 describes their ownership.
Their future bounded composition must enumerate a finite number of source/
group/prep/operand/clone slots and alias successful Arcs by those source sites.
Current original H/VR domains and schedules are unchanged; this is no new
restriction or acceptance of an arbitrary multi-case bound.

## Arithmetic/regression obligations and remaining cells

ARITHMETIC.json records pure source integer evaluations. Required later
implementation regression cases include B=1 and B=5 (non-power-of-two), across
all three verification widths. The additional B=0,3,4,7,8,9 rows are arithmetic
boundary samples only; they introduce no model or fixture.

For B=1: retained resolution64 versus old16; deficit48. Its first allocation
has no old realloc request; Scale helper contributes2w+64, HatCheck128.
For B=5: retained resolution128 versus old80; deficit48. Active old request64;
Scale helper10w+128 retained /10w+192 move; HatCheck256 retained /320 move.
These numbers describe the named helper owners, excluding Kpad+n*w+q*w.
They are source arithmetic, not tests, observed capacities or global peaks.

Carry T1/T2/C1/C2/O2/W1 unchanged: actual private tree leaves; omitted actual
nominal/tuple layouts; queue/input capacities; concrete all-fixture descriptors;
final A1 reconciliation; H staged/prefix and finite VR global caller/consumer
composition. O1 remains independent review of the repaired identity union.
No full E_max, estimator implementation, W1 admission replay, new observer,
fixture narrowing or source_05 closure is accepted. ROOT will route this exact
additive correction to the same RV30 for backcheck.
