# I57 — exact native summary coverage for the prepared receipt

Status: PROPOSED for fresh independent design review and ROOT disposition. No
schema, maintained source, reader eligibility or public producer is selected here.
This is a new recovery assignment under ROOT HELP_HUMAN /root, TASK Type 2 through
native delegation. Receipt 2026-10-03T19:05:51Z. P means projects/chirality-piping;
FK means P/core/solver/frame_kernel/src/structural/retained; PP means
P/core/product_physics/src. Immutable source is fc23cff95f. Full source identities,
read origins and source-level check results accompany this file.

- PROPOSAL: Carry the irreducible native coverage outcome once in its proof owner.
  - Evidence: FK/product_certificate/final_case.rs:1326–1450, 1191–1195,
    1573–1579, 1789; PP/retained_product.rs:2759–2842, 3362–3366,
    3494–3536; C1 §§4/6, C2 §§3/4, C3 §§2/4 and F1/06/07/08.
  - Change: Add the closed nullable summary_coverage member specified below to
    C3 ProofTrace. Encode the native stop flags and has_data from the exact
    ProductCertificateSpent; rederive estimate and charge as specified. Validate
    the complete reconstructed coverage against the producer's actual typed
    vector before encoding, and use that coverage for exact reader cardinalities.
  - Why: Published binary64/final dual-readout rows do not reveal private q_P
    nonzero or native block-data facts. The proposed field closes that omission
    without changing required summary coverage or exporting a private value.
  - Risk: Treating digest-bound flags as an independently reproduced solve would
    overstate reader assurance. Missing owner/prefix custody would allow a flag
    swap; copying the product adapter's partial vector would forge completeness.
  - Status: PROPOSED

## 1. Scope and one closed shape

This is the completion for the selected ordinary-prepared C3 route
RP-PREPARED-ORDINARY-DUAL-v1. It inherits C1/C2 and corrected C3/F1/06/07/08
unchanged except for this explicit field. It does not select a nonprepared,
exact-profile or prepared-combination implementation. Those routes cannot borrow
this proof's coverage or become eligible by a Cartesian/final-row fallback.
Their independent producer/caller obligations remain; this packet makes no claim
that their complete F2a implementation is finished.

Every non-null C3 ProofTrace gains exactly this required member:

```text
summary_coverage: null | [CoverageBody]
CoverageBody = closed {
  body: U,
  stop: [bool;4],
  has_data: bool
}
```

Array order is ascending native body id, exactly 0 through body_count−1, including
isolated/zero/no-data bodies. Four stop indices are translation, rotation, force,
moment. A complete source has at least one body; no_nodes is already a source
refusal. There is no empty-array representation of complete coverage, no omitted
member, and no per-body null/default. Existing U rules and admitted body/copy/
encoding bounds apply. No new policy id, signature or hash domain is introduced.

`null` means **no complete coverage vector was successfully retained by this
proof**. It does not say whether the internal helper was entered or failed, and
is not zero coverage. Existing typed stage, failure and work evidence states the
actual prefix; do not parse an Association string to manufacture a finer state.
If ProductAttempt.proof itself is null, there is no nested coverage object.

The selected case already reaches this single snapshot by
case.product_attempt_ref → ProductAttempt.proof. There is no second mutable copy
in Selection, Run, source_binding, final rows or a new registry. A Ready attempt
with later receipt/publication failure still keeps its proof-owned snapshot even
though there is no selected public case. Unavailable attempts can retain a
complete snapshot without a Selection; that does not promote them.

This is the smallest body-fact payload needed by the existing typed coverage
contract after eliminating publicly redundant estimate/charge flags: four native
stop outcomes and one native data outcome, with an explicit body identity. It
exports neither per-row nonzero bits nor the private values producing them.
Encoding all nine native booleans would be faithful but redundant; omitting
stop or replacing has_data with a check on published values is not faithful.

## 2. Exact source derivation

Use the bound **new native source and selected native Run**, not old operational
facts or final projection values. Let p be selected native precision and P=2p;
fixed proof/projection 1024 never changes this p/P or triggers a p512 floor.
For body b and kind k let:

- present[k] mean that the exact source layout contains a row of that body/kind.
- non_input_present[k] mean at least one such row has input_derived=false.
- A[k] mean the private predicate: some such non-input row's recovered value in
  owner.state(P) is nonzero.
- D[k] mean the same predicate restricted to input-derived rows.
- L be the native body extent, recomputed by adaptive::body_extent from mapped
  node coordinates in native order, using its binary64 subtract/multiply/add/
  sqrt order. A geometrical distinct-node test is not a substitute for L != 0.
- E be the native selected verification resolution_scale force/moment values;
  floor be the actual selected native p512 floor, otherwise null.

The actual function first takes positive=A. If L != 0 it replaces positive by
[A0∨A1,A0∨A1,A2∨A3,A2∨A3], then ORs positive2/3 with (floor.force>0)/
(floor.moment>0) when floor exists. Its exact stop formula is:

```text
stop[k] = present[k] AND (positive[k] OR A[k] OR D[k])
```

A and D are explanatory private variables, never values obtained from published
rows. ProductSummaryCoverage reads owner.state(P).recovered.values[index].is_zero
(final_case.rs:1334–1362). C3's final values are later hull projections of two
separately retained readouts. Even native binary64 publication cannot safely
replace the private Wide nonzero predicate.

Reconstruct the omitted native flags exactly:

```text
hats = [E.force > 0, E.moment > 0]
if L != 0: hats = [hats[0] OR hats[1], hats[0] OR hats[1]]
estimate = [present[force] AND hats[0], present[moment] AND hats[1]]
if p in {128,256}: charge = estimate
if p == 512:      charge = [stop[force], stop[moment]]
```

The p512 identity is a source derivation, not a proposed approximation.
recover::layout:101–195 marks only constrained displacement DOFs input-derived;
all force/moment rows are non-input-derived. Thus for these two kinds D=false,
positive already includes A, and stop[k]=present[k] AND positive[k], exactly the
source charge expression at final_case.rs:1437–1438. G5a must derive the canonical
layout/flags from the bound source maps and reject a purported force/moment
input-derived layout; final G8 still independently binds those maps to invocation.
The accompanying 648-case boolean enumeration checked this identity for every
presence/nonzero/coupling/floor pattern consistent with nonzero implying presence.
It is a source algebra check, not an executed solver witness.

Finally, has_data is ANY(data[i] AND group.blocks.body[i]==b). The block data
comes from the same owner-bound source bridge view. bound::fill_data_blocks:
162–201 and ledger::nonzero_term_spent:149–164 inspect original nonzero ledger
contributions, private verification
u[g], and adjacency to nonzero prescribed DOFs. A netted load or final zero row
is not the data definition. Actual cancelled nonzero terms are the decisive
counterexample (PP/retained_product_tests.rs:1442–1500).

Producer encoding must reconstruct all nine flags from this payload and public
inputs and compare body/stop/estimate/charge/has_data bit for bit with the actual
ProductSummaryCoverage accessor. Any mismatch is an association/encoding failure
under the existing ordinary transaction, not a license to alter the vector.
This also guards the compact p512 derivation against a later source/layout change.

## 3. Custody, lanes and unavailable prefixes

The authoritative snapshot is ProductCertificateSpent::summary_coverage()
(final_case.rs:301), or CertifiedProductProof::summary_coverage() (1758), read from
the exact work owner already supplied to PP/retained_receipt::project (99–123).
ProductProofTrace currently omits that slice (final_case.rs:232–239,280–289).
The narrow future source seam is a borrowed typed coverage slice in that trace,
then the closed public mapping above. No new solve, residual, nonzero scan or
coverage computation runs during projection/serialization.

Do not source the field from ProductCapture.summary_coverage. Its later
prepared_verdict_copy clears/reserves/copies entries with fallible accounting
(PP:3362–3366); an interrupted copy is a different fact from the complete proof
vector. The proof remains available on actual proof/values/abandoned failures
and certified later refusals through typed_trace (PP:3619–3636). If the canonical
proof owner is unavailable, preserve the existing receipt-encoding fallback;
never fill from the adapter, a sibling proof or the final row inventory.

Coverage assignment is atomic in the actual source:
`spent.coverage = summary_coverage_data(owner,data,spent)?` (1195). Its local
vector is returned only after every body completes. At this source, an empty
proof vector is therefore mapped to null; a nonempty vector must cover all bodies
or projection refuses. Private failed helper visits remain in proof work, not a
fabricated public body prefix. This interpretation depends on this source rule;
a future incrementally assigned vector must revise the mapping explicitly.

Prepared proof start obtains admitted_k then annular_source through the same
anchor, and requires equal data arrays and row cardinalities (1573–1579).
certify_final authenticates frozen owner/values/rows before calling
check_intervals with admitted_k.data (1780–1789). Therefore:

| Actual retained path | Required public coverage |
|---|---|
| Preparation/native failure; no proof | No ProofTrace and no coverage object |
| Proof-start/lane failure, projection failure, values/aliases abandonment before final certificate | ProofTrace with summary_coverage=null; actual entered lane/work prefix retained |
| Certificate entered but fails before complete summary assignment | null; preserve its actual typed failure/work, never claim all bodies false |
| Summary assignment completed, then row-scale/numeric/accounting/certificate failure | Complete actual array, even though product is unavailable |
| Certificate succeeded, later adapter-copy/observable/G5a/commit refusal | Complete proof-owned array if its proof work remains; do not substitute partial adapter contents |
| Ready product, including later outer receipt failure | Complete actual array; all original stage/check and transaction rules continue |

Non-null coverage requires the same ProductAttempt.source_ref and run_ref, a
selected native Run, two completed lanes in their declared order, completed
proof_start/projection/maxima/values/aliases, and certificate entered
(completed or failed). A completed certificate or passed G5a requires non-null
coverage. Null is allowed on a failed certificate, never on Ready. Never infer
certificate success merely from complete coverage.

Binding is the existing C2/C3 graph: case owner/basis/id ↔ unique ProductAttempt
↔ ordinary_attempt/material_basis ↔ newly prepared CaseSource/preparation hash
↔ Run.origin(source_ref,owner_ref,call,position) ↔ the selected logical native
candidate and its precise verification record P=2p. Same stiffness, same numeric
body id, same value, equal digests or another complete proof cannot replace that
identity chain. No public proof pointer authenticates an actual in-memory anchor;
source review/replay is still required to establish that producer custody.

## 4. Public checks and first failure order

Keep G0→G1→G2→G3→G4→G5→G5a→G5b→G5c→G6→G7→G8. Process attempts in existing
order, then bodies ascending. Earlier original checks still win.

| Gate | Added obligation / existing failure code |
|---|---|
| G0 | Existing known C3 definition/table/scope. No new identity or changed numerical definition. |
| G1 | Exact required closed member, null-or-array, closed body objects, four booleans and has_data boolean; existing receipt/source/preparation/publication hash validation. RETAINED_PRECISION_RECEIPT_MISMATCH. |
| G2 | Body U safe integer encoding under existing rules. RETAINED_PRECISION_ENCODING_MISMATCH. |
| G3 | Non-null array cardinality and ordered unique body ids equal the source inventory already associated through this attempt. RETAINED_PRECISION_COVERAGE_MISMATCH. Existing owner/attempt bijection errors remain here. |
| G4 | Unchanged diagnostic scope/exclusivity; do not inspect flags or row method tokens here. |
| G5 | Existing schedule first; then same-source/Run/proof binding and above availability/stage/lane implications in the C3 association pass. RETAINED_PRECISION_PRODUCT_ATTEMPT_MISMATCH. Work/status remains the later original WORK_MISMATCH pass. |
| G5a | Exact reconstructed coverage, public constraints and numerical summary checks below. RETAINED_PRECISION_SCALE_MISMATCH. |
| G5b/c | Existing scale/section/class/DOF checks unchanged; no final-row values are inputs to native nonzero coverage. |
| G6/G7/G8 | Original method, literal base projection and raw invocation/material/source obligations unchanged. |

At G5a, first verify existing summary encodings/ranges and actual native p/P/floor
rules, then canonical source layout and extent, then the compact-flag consistency,
then exact summary cardinalities/numeric limits. Require selected p512 floor to
contain exactly one force/moment pair per body including zeros; other native p
requires null. Use the actual existing floor derivation/equalities and G5b checks;
this addendum supplies no replacement phi formula or permission for proof1024 to
create a floor.

For every complete payload, public stop consistency can be checked without a
solve by this exact small Boolean feasibility rule: enumerate the sixteen A
vectors; discard any with A[k] true where non_input_present[k] is false. In this
C3 scope every input-derived row is a prescribed displacement with exact +0,
so D=[false;4] (rederive that canonical layout/prescription relation, do not trust
a mutable input_derived flag). Form positive and stop by §2 using the actual
p512 floor presence/positivity. At least one permitted A must produce the four
attested stop bits. No A vector is serialized or claimed as the actual one.
This is a necessary public consistency condition, not reconstruction of q_P.
In particular, absent kinds cannot require summaries, positive floors force
present-kind coverage, and zero extent does not create coupling.

For data, directly derivable constraints are also mandatory: no free DOF in the
body implies has_data=false; a nonzero original admitted nodal contribution at
a free DOF implies has_data=true. Preserve separate contributions, including
cancellation, as C2 requires. Remaining data facts stay attested. No claim that
zero net loads or zero final values imply has_data=false is permitted.

For every selected case, reconstruct ProductSummaryCoverage and enforce the
actual PP validate_summary_shape contract exactly:

1. stop_rule has exactly one entry per true stop bit, none per false bit;
   all four kinds are allowed, with canonical nonnegative finite value ≤2^-64.
2. verification_estimate/verification_charge have exactly one entry per true
   rederived estimate/charge force/moment bit, none per false bit; no translation/
   rotation entries, with value ≤1/4 and ≤1 respectively.
3. resolution_scale and theta each cover every body exactly once; theta is
   canonical nonnegative finite ≤1/2; resolution retains its original nonnegative
   and subsequent zero/sanity/lower checks.
4. certified_bound has exactly one finite positive entry iff has_data=true and
   no entry otherwise; no duplicate or foreign body. Bind this roster and its
   bits to the exact selected verification record's non-null bound entries,
   and resolution/theta to that same record under existing native consistency.
   A no-data body has verification bound=null and theta=+0; data_blocks=0 iff
   no body has_data, and otherwise data_blocks is at least the true-body count.

The selected verification metadata is a second consistency relation, not an
independent proof of the same private data fact. Dropped/extra entries fail even
when all remaining numeric inequalities pass. Do not demand a Cartesian roster
or remove zero-valued required summaries.

Unavailable attempts keep their actual coverage when present but have no
invented Selection or successful public numeric claim. Structural, ownership,
stage and direct source-consistency checks still apply. Do not apply selected
summary pass conditions to unavailable native snapshots whose actual G5a failed:
preserve the failing prefix/cause. If a public check is recorded passed, its
prerequisites, including complete coverage, must hold. There is no validation of
absent unpublished final bytes from this metadata alone.

## 5. Hash scope, controls and decision boundary

The new field belongs only to C3 ProofTrace inside body.product_attempts.
H(retained_precision_receipt_mp_v2, body) therefore binds every body id, stop bit,
data bit, null state and the existing attempt/source/Run references. Do not add
this execution outcome to mechanical SourceBinding or the semantic preparation
hash: doing so changes their meaning and risks a cycle. The existing final
publication hash still covers the final envelope with retained_precision removed;
a receipt-only flag mutation changes receipt hash, not publication hash. Rehash
all scopes actually affected by each control and preserve the frozen definition.
The pending closed schemas/table implementation bytes change upon selected
integration; DEFINITION.json's numerical contents/domain hash do not change.
Admit and account for the actual new borrowed typed-view layout, body-array
projection/copy/encoding and hash scratch under their existing owners before
use. Re-run affected source layout/trace-accounting and partial-failure controls;
a five-flag payload is not a complete memory bound or zero-cost projection.
No new all-in debit, byte allowance or M qualification is claimed.
Historical sealed receipts and records are untouched.

Required controls for producer source review/replay and the shared three-reader
corpus (synthetic until actual production is witnessed):

| Control / rehashed mutation | Required disposition |
|---|---|
| Zero/no-data body, complete native source | Complete body entry, all source-derived flags as actual; resolution/theta still one each; no B. No empty array or invented nonzero summaries. |
| +x and −x individual free-DOF loads with zero net/final values | has_data=true; dropping B or changing only has_data=false fails G5a. Existing PP cancelled control is a source witness, not a new runtime result here. |
| Nonzero private q_P with zero or different native/final binary64 row | Actual stop is retained; final-row mutation cannot recompute/remove it. Producer control obtains the nonzero fact from the actual owner; a synthetic flag is labelled synthetic. |
| Body/kind absent; L=0 versus L!=0 | Exact Boolean feasibility and estimate derivation; no absent-kind entry or invented cross-kind coupling. |
| Native p512, actual zero and positive floors; p128/p256 with fixed1024 product proof | p512 charge equals force/moment stop; lower p charge equals estimate. Actual positive floor forces relevant coverage; proof precision alone creates no floor. |
| Drop required or add forbidden stop/estimate/charge/B; duplicate numeric entry | G5a SCALE_MISMATCH even if value=+0 and every bound passes. |
| Remove required field, wrong tuple size, numeric/null flag or unknown field | G1 RECEIPT_MISMATCH. |
| Duplicate/missing/swapped body identity with other fields unchanged | G3 COVERAGE_MISMATCH. |
| Swap compact flags between bodies with distinct presence/floor/load constraints | G5a SCALE_MISMATCH after receipt rehash. If bodies have indistinguishable public constraints, that swap may not be publicly detectable; producer custody/replay must catch it. |
| Change only attempt owner | Existing G3 owner coverage mismatch. Change only source_ref/run_ref to another valid owner while preserving attempt owner | G5 PRODUCT_ATTEMPT_MISMATCH after affected hashes are correctly recomputed. |
| Copy flags from another complete proof; rehash receipt | Partial source/Run/owner changes fail existing G3/G5; a fully consistent forged receipt is not authenticated by these checks. Source-bound producer mutation/replay must cover the actual swap. |
| K fails or source lane fails after K; missing proof/earlier value failure | Exact actual lane/work prefix; summary null; no selected eligibility or fabricated complete false vector. |
| Summary completes, later certificate fails; adapter copy fails partway | Retain complete proof-owned flags, actual typed refusal and spent work; never emit adapter prefix. |
| Ready or completed certificate with summary null; non-null summary before certificate | G5 PRODUCT_ATTEMPT_MISMATCH; earlier G1/G3 defects still win. |
| Flag/coverage defect plus wrong row recovery_method string | G5/G5a as applicable before G6; null/nonstring row token still fails earlier G1 per 07/08. |

The original numerical guarantee is unchanged: the actual producer performs its
native stop/verification checks, direct final certificate against both physical
readouts, exact native summary-shape preflight, and final gates. Readers establish
closed encoding, hash binding, owner/reference consistency, rederivable facts,
exact **attested** summary coverage and public bounds. An attacker can consistently
rewrite flags, native summaries and all unkeyed hashes; no added Boolean field
proves origin or reproduces a private solve. C1 §6 and C3 §4/DEFINITION.trust already
make this same producer-attestation/review/Rust-replay boundary explicit.

Recommendation: ROOT select this completion only after fresh design review
confirms the compact derivation and custody, then authorize the narrow trace/
producer/schema/reader/corpus changes and actual producer/Rust replay controls.
Keep the affected reader eligibility closed until those dependencies are met.
If the intended requirement instead is for every reader to independently prove
actual private nonzero/data facts against a malicious fully rehashed receipt,
that is a different assurance requirement: this representation is insufficient.
ROOT must take that concrete choice to the owner; possible routes are an
explicitly authorized private-state proof/replay or authenticated producer
attestation, each separately designed. Replacing actual coverage with a Cartesian
roster or final-value rule would weaken/change the contract and is not proposed.

MISSING: Fresh independent design review; ROOT selection; maintained typed trace
mapping; atomic pending schema/table/corpus/three-reader integration; actual
producer custody and partial-failure replay; full native/public finalization.

NEEDS_HUMAN_RULING: None demonstrated within the already explicit attestation
boundary. If independent proof of private flags against a fully forged receipt
is required, the assurance choice above is owner-held; do not silently claim
this completion satisfies it. ROOT owns all decisions and escalation.

DEPENDENCY_NOTES: No mathematical definition change; no private endpoints,
radii or solve replay exported. No native-only/exact/prepared-combination route
is qualified by this C3 completion. No instructions, protected oracles, public
activation, maintained source, Git/index/API state or runtime were changed.
