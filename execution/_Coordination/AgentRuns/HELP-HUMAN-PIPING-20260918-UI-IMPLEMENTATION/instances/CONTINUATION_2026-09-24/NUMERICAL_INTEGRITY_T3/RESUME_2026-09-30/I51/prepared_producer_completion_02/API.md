# Closed API and one frozen candidate

Signatures below are the proposed typed contract, not compiled code. Public here
means only the Rust cross-crate seam; it grants no publication/provenance authority.
All named structs have private fields, no Deserialize, no mutable numeric access,
and no free constructors except those listed. Stage owners/proof drafts are not
Clone. Endpoints, matrices, general intervals and value/radius pairs do not cross
the production FK/PP interface. PP validates product semantics; FK validates its
source/owner/closed recipes. Neither substitutes for the other.

## 1. RV67-2: preparation and authenticated source

```rust
// FK retained_api; only scalar geometry, not product provenance.
pub fn prepare_product_annulus(diameter: f64, effective_wall: f64)
    -> AnnulusPreparationSpent;
pub struct PreparedAnnulus { /* input_bits:[u64;2], section_bits:[u64;5],
    algorithm: AnnulusPreparationVersion; constructors private to FK */ }
impl PreparedAnnulus {
    pub fn input_bits(&self) -> [u64;2];
    pub fn section_bits(&self) -> PreparedSectionBits; // Copy, read-only A,I,J,Z,c
    pub fn algorithm(&self) -> AnnulusPreparationVersion;
}
impl AnnulusPreparationSpent {
    pub fn work(&self) -> &SectionPreparationWork;
    pub fn into_parts(self)
      -> (Result<PreparedAnnulus, SectionPreparationError>, SectionPreparationWork);
}
```

The successful scalar result binds exact input bits and fixed algorithm/version.
Use the existing directed1024 geometry and fixed pi interval; identical finite
RN64 endpoint conversions establish A/I/J/Z. Check c, geometry, positivity and
the existing normal-primitive/range constraints; equal zero/overflow endpoints
do not pass. Errors are closed: InvalidInput/Geometry, AmbiguousRounding(property),
Conversion/PrimitiveRange(property), Arithmetic(original cause), Accounting,
CountLayout or Storage. Every exit owns entered arithmetic/conversion/capacity
work, including successful fields before a later refusal. No refinement.

```rust
// PP-private: captured hook validation, not a user-constructible credential.
struct ValidatedOrdinaryCase { /* actual capture/case/basis keys, old source,
    old facts/op checks, complete maps/ledger, normalized D/t/material facts,
    completed observations, work; private constructor after existing checks */ }
struct PreparedCase { /* one new PrimitiveSource; Vec<PreparedAnnulus>;
    new Vec<ProductMemberFacts>; new Vec<OperationalSpent>; checked product maps,
material/observation owners; old-to-new compact association; prep work */ }
fn seal_ordinary_capture(capture: ProductCapture, ordinary: &OrdinaryOwner)
    -> CaptureValidationSpent; // exactly one actual hook chain; no external factory
fn prepare_case(old: ValidatedOrdinaryCase) -> PreparedCaseSpent;
impl PreparedCase {
    fn source(&self) -> &PrimitiveSource;
    fn facts(&self) -> &[ProductMemberFacts];
    fn operational(&self) -> &[OperationalSpent];
    fn product_basis(&self) -> &CheckedProductBasis;
}
```

Old hook checks still compare actual normalized/resolved/old built operands with
each other, including the support bijection before canonicalization. The private
prepared probe suppresses only the old diagnostic native solve in `finish`;
default existing diagnostics and observer=None behavior remain unchanged. After
the actual final ordinary envelope establishes no exact-block selection and the
existing scope/custody permit, consume the validated capture to prepare the new
case. No preparation/solve/draft is constructed on an exact-block-selected path.

For each mapped member, require prepared input bits equal its normalized D and
actual effective-t. Bind case, base/point/interpolated material identities and
actual selected/resolved E/G exactly as before. Keep nodes, y_reference, constraints,
springs, stations, supports and each ledger term/id; only prepared section fields
change in the genuinely new SourceParts. Build new facts from those results,
not old facts; check new source/facts/preparation equality in both directions.
Compute L/ka/kt with the existing binary64 operational evaluator on the new
source. Verify all its recorded operand bits. Preserve old-to-old checks and a
compact old/new/preparation record before dropping the old source/facts temporary.
All overlap is accounted. Failed new preparation preserves the ordinary envelope.
For the new prepared-probe mode, hooks retain ordinary produced facts as counted
routing-observation snapshots and perform old-to-old checks; SourceParts and any
old/new PrimitiveSource construction are deferred until the completed ordinary
envelope grants the no-exact-block permit. Thus rejecting that permit performs
no W1 source preparation. The legacy diagnostic mode retains its existing path.


The scalar helper authenticates no material/case on its own. `prepare_case`
supplies that association. New source encoding and preparation version/input/output
association enter private replay evidence. Once solved, `RecordedInvocation`
must establish the actual run/RetainedSolve/CasePrep Arc/source/cache relationship;
equal source digests in a foreign invocation are insufficient.

## 2. RV67-2: one proof draft through projection and final checking

```rust
pub struct ProductRowSpec<'m> { /* borrowed id/case/entity/metadata identity;
    closed kind/unit/body/recipe, ordinal; no mechanical value/interval/radius;
    only ancillary variants carry private authenticated ObservedBits */ }
// Closed validating constructors: native, support_component, component_stress,
// circular_maximum, mode, dense_parity, selected_material_record.
// They reject invalid kind/unit/component/site pairs; source binding is below.

impl RecordedInvocation {
    pub fn begin_prepared_product<'s,'m>(
      &'s self, run: usize, selected: &'s RetainedSolve,
      facts: &'s [ProductMemberFacts], specs: &'m [ProductRowSpec<'m>])
      -> ProductProofStartSpent<'s,'m>;
}
pub struct ProductProofDraft<'s,'m> { /* CheckedOwner<'s>, specs,
    LaneReadouts K + Source, unique proof anchor, owned ProductWork */ }
impl<'s,'m> ProductProofDraft<'s,'m> {
    pub fn project(self) -> ProductProjectionSpent<'s,'m>;
}
// Success extraction transfers to:
pub struct ProjectedProofDraft<'s,'m> { /* same owner/specs/lanes/anchor/work */ }
pub struct ProductValuesBuilder { /* Q f64 values; M pending-max bits;
    matching proof anchor; no public setters except complete_maxima */ }
impl ProductValuesBuilder {
    pub fn value(&self, ordinal: usize) -> Option<f64>; // non-max only
    pub fn complete_maxima(self, maxima: &[ProductMaximumValue])
      -> ProductValuesSpent; // exact max descriptor coverage, finite Pa values
}
pub struct FrozenProductValues { /* complete Q values, same proof anchor */ }
impl FrozenProductValues { pub fn value(&self, ordinal:usize) -> Option<&f64>; }
impl<'s,'m> ProjectedProofDraft<'s,'m> {
    pub fn certify_final(self, values: &FrozenProductValues,
                         rows: &[ProductFinalRow<'_>],
                         values_work: ValuesCompletionWork) -> ProductFinalSpent;
}
```

`ProductMaximumValue`
is a closed (member/row ordinal, Pa value) record, not an interval or certification
credential. PP creates it only from its actual candidate maximum. FK checks its
descriptor/owner coverage and finally its physical interval; PP checks its
coefficient provenance and midpoint. No other projected value is caller-mutable.

**Ancillary numeric custody is explicit.** Mechanical RowSpec constructors take
no value. Only these three closed constructors carry observed numeric bits:
`mode(spec_identity, actual_mode)` derives the fixed mode code (1 or 2);
`dense_parity(spec_identity, observed_delta_bits)` requires finite >=0 while
preserving the actual zero sign; `selected_material_record(spec_identity)` fixes
presence to exact 1.0. PP invokes them only after checking exact metadata,
case/invocation/mode, expected presence and value bits against PreparedCase's
immutable completed observation/material snapshot (the existing I50 checks).
The private `ObservedBits` payload remains in the bound RowSpec. No arbitrary
record-kind constructor or mechanical-value setter exists. `project` copies those
bits unchanged into their Q value slots, without a hull or arithmetic. On final
checking, FK requires bit equality to the bound ObservedBits and the frozen slot;
PP rechecks the overlay against its original captured observation snapshot. The
mode/parity/material values are therefore supplied by their actual ordinary
producer, not invented from the new native owner. Missing/extra/relabelled records
or changed values/text fail even when the remaining mechanical rows pass.


Each Spent has private result payload plus owned work. Its consuming
`into_ready`/`into_parts` transition **moves** work into the next stage on success
or into `ProductFailureSpent` on failure; no successful draft and wrapper both
own/count the same work. Read-only work/error accessors are allowed. All callers
must immediately retain failure Spent in the private attempt record. Original
numeric cause and joined accounting status coexist; prior fault prevents work.
The concrete success transitions are StartSpent::into_ready -> Draft and
ProjectionSpent::into_ready -> (ProjectedDraft, ProductValuesBuilder), each with
the work moved into the draft. ValuesSpent owns only its newly entered completion
work. Its success extraction returns (FrozenProductValues, ValuesCompletionWork),
with both bound to the same private proof anchor. Final certification takes the
work by value, verifies that anchor, and collects it once; the frozen values do
not retain a second counted copy.
If PP maximum construction, values completion or another prefinal gate fails,
`ProjectedProofDraft::abandon(self) -> AbandonedProofSpent` drops its numerical
buffers while returning their owner/work/capacity record. The PP attempt failure
retains that record plus the original PP/values error and its separate work.
Abandon performs no numerical operation or new residual. FinalSpent owns all FK
work on either outcome; its success token has no borrowed row metadata.


`begin_prepared_product` validates recorded owner/run/source/cache, actual native
p/P and admission, new member/facts/material bits, full descriptor identity and
coverage, input-derived map and closed ancillary presence. PP first validates
all original row metadata/observations against its captured basis, then forms
the borrowed specs. FK constructs private ProposedMemberLaw values against this
owner; no caller-supplied law/operator is accepted.

Internal `enum ReadoutLaw { AdmittedK, AnnularSource }` is exhaustive. K uses
the four exact admitted products and exact-zero coefficient differences; source
uses its full selected coefficients in **both** residual and recovery. Preserve
the owner's scaling, beta=2B, strict alpha condition, individual-term/no-data/
zero-adjacency/anchoring premises and every missing-bound/cache refusal. Exactly
one residual call per lane, at most one cached-factor correction within each,
then its fresh residual and recovery. A zero-Delta lane is charged work.

Add the private consuming seam

```rust
ResidualSpent::into_readouts(self)
    -> (Result<LaneReadouts, BridgeError>, ResidualWork);
// LaneReadouts owns rows, data, alpha, epsilon; no borrow of ProposedMemberLaw.
// SourceBridgeView::into_data(self) privately moves data out; owner/scales
// borrows end. ProductProofDraft retains its separate checked owner lease.
```

The lane's temporary laws/center/rho/correction can then drop without self-reference.
Retain the moved data/alpha/epsilon vectors for both lanes through final checking;
their live dimensions are in DESIGN.md. Existing production result traces remain
disabled. First-lane success followed by second-lane failure returns K's whole
work/capacity prefix plus Source's entered prefix and original error; any first
lane rows dropped on failure are still recorded in the peak/lifetime account.

Projection calls the closed recipes on those two stored readout vectors. It does
not run another residual. `certify_final` consumes that same draft, checks the
proof-anchor identity and equality of each supplied final-row value to the frozen
value slot, then applies unchanged gates/scales/G5a-summary prerequisites using
the same lane vectors. It must **not** call today's `certify_product_case` or
`run_case` residual-building entry. The old entry remains for old diagnostic tests.
There is no external endpoint accessor or new radius constructor.

Successful final proof drops both lane vectors/spec borrows and returns an owned
`CertifiedProductProof`: closed verdicts/summary/work plus an owner stamp cloned
from the verified CasePrep Arc/run/cache/source identity and the projection anchor.
It contains no reference to candidate values or old row strings. Arc ownership
prevents address reuse; all header/counter operations and retained prep lifetime
are counted. Read-only private diagnostics may record lane widths while live;
there is no public serialized interval/radius API.

## 3. RV67-3: freeze, identical consumers, consuming transfer

Choose one **frozen overlay**, with the original envelope held in
`OrdinaryOwner { envelope, unique Arc<OrdinaryAnchor> }`. No mutator or clone of
that owner is exposed to candidate code. A PP-only one-attempt token permits the
prepared probe after the ordinary result and coexistence/scope checks. It does
not claim invocation-wide numerical budget admission.

| State | Owned/borrowed data and permitted next step |
|---|---|
| OrdinaryReady | Own the sole untouched MechanicsEnvelope and anchor; PreparedCase and recorded new native solve live separately. Failure returns this exact ordinary envelope. |
| Projecting | Borrow its immutable row metadata for Q specs; one FK draft owns both lane readouts. Receive independent ProductValuesBuilder. Borrow identified projected endpoint actions to build each actual maximum; no ordinary maximum value is used. |
| CompletePayload | Own completed FrozenProductValues, M numeric maximum patches, recomputed coverage, two new summary aliases and work. Each max patch identifies the existing per-member evidence object and contains **all eight newly formed numeric fields as ready JSON Numbers**. Its five identity/scope strings remain borrowed from the validated original. No copied evidence tree or whole envelope. |
| FrozenOverlay | Read-only view borrowing OrdinaryOwner and CompletePayload. No numeric, order, metadata, maximum or alias mutation is possible. All consumer views below read this same pair. |
| CheckedPayload | FK consumes draft to certify frozen rows; PP observes metadata/complete coverage, norms/maxima/headlines and actual new-K G5a through the same frozen view. All pass tokens bind the ordinary anchor and projection anchor. End every borrowed-row/spec/view lifetime. |
| CertifiedCandidate | Move the original owner and complete payload into one PP-private object with all pass tokens and owned CertifiedProductProof. Its only outward operation is a consuming private commit. No arbitrary envelope can be substituted. |
| PrivateCommitted | Move the checked values/number patches/aliases into the original envelope, without allocation or fallible checks. Return an opaque PrivatePreparedCandidate plus complete work; this is not public method selection or an ordinary identity available for export. |

The overlay is the closed enum/view `ProductCaseView::Ordinary(&MechanicsEnvelope)`
or `ProductCaseView::Prepared(FrozenOverlay)`, not a trait callback. Its accessors
are `row_count`, `row(i)`, `maximum(i)`, `coverage`, `headline(kind)` and
`ancillary(i)`. `row(i)` returns original identity/kind/unit/entity/basis/metadata
references with **the new value reference**; maximum numeric fields come from
the patch; aliases come from the new LocatedQuantity values. Strings and ancillary
records come from the actually validated ordinary/capture owner.

Adapt PP bind_rows, observables and G5a to consume that closed view (retain old
ordinary wrappers). FK final rows borrow its new value references. The PP adapters have these closed inputs (Spent returns retain their own work):

```rust
fn bind_final_rows<'v>(view: FrozenOverlay<'v>, prepared: &PreparedCase,
    owner: &RetainedSolve) -> RowBindingSpent<'v>;
fn check_observables(view: FrozenOverlay<'_>, prepared: &PreparedCase)
    -> ObservableSpent;
fn check_g5a(view: FrozenOverlay<'_>, prepared: &PreparedCase,
    owner: &RetainedSolve, certified: &CertifiedProductProof) -> G5aSpent;
```

G5a uses the **new** PreparedCase operational/facts records and the exact returned
final verdicts; it first verifies their projection/ordinary anchors and row
normalized bits against this overlay. It never consults stale ProductCapture
operational fields or old verdicts. Final summary coverage is derived from the
already owned native data mask and owner evidence, without constructing another
residual-producing view. The two stored lane masks must agree with that same
native owner. Recompute all norm guards, scales/classes, G5a and aliases from
that same overlay. A maximum
view never falls back to old numeric extrema. Reject missing/duplicate patches,
partial coverage or any source/owner/metadata mismatch before freeze.

**Actual maximum construction:** instantiate one prepared recovery pipe/section
with the same source coordinates/frame and checked new property bits; take its
identified projected end actions with the existing sign convention. Call the
unchanged `exact_straight_summary_extrema` algorithm with empty loads and no
pressure. Keep its actual result, location/tie and midpoint. A narrow adapter may
expose this existing helper; do not change its arithmetic. The eight patch fields
are station_fraction, span_index, local_fraction, value_lower_pa, value_upper_pa,
global_upper_bound_pa, certified_gap_pa and subdivisions. All are regenerated.
The five reused strings are pipe_id, result_id, approximation, coefficient_basis
and enclosure_scope. Their exact values must match the new descriptor and current
contract. Independently recompute complete member coverage and support attribution;
unchanged empty coverage lists may be retained only after exact equality to that
fresh result is checked. Select both headlines from candidate rows with existing
tie rules and preallocate their three strings each.

**Commit contract:** `CertifiedCandidate::commit(self) -> PrivatePreparedCandidate`
is PP-private and consuming. Before certification, validate existing target slots,
field shapes, all indices, move counts and serialization-number representability;
preconstruct JSON Numbers and aliases and charge/check the entire fixed move plan.
After every borrowed overlay/spec/final-row reference has ended, transfer only:
Q f64 values to matching existing rows, 8M Numbers to existing fixed JSON slots,
and two LocatedQuantity aliases to existing summary fields. Map keys/strings,
row order and capacity do not change. Use private invariant-checked slot replacement,
not insertion, serialization, cloning, formatting, allocation or a fallible API.
No Result-producing operation follows the first mutation. A programmer invariant
violation is a defect, not a recoverable candidate refusal.

CaptureValidationSpent and PreparedCaseSpent likewise have private result/work
fields and consuming into_ready accessors returning Result<State, FailureSpent>.
Success moves preparation/capture work into the next private state; failure owns
its full prefix. AnnulusPreparationSpent alone uses the shown scalar into_parts,
whose returned work is immediately moved into the PP preparation ledger. No generic caller can
construct ValidatedOrdinaryCase, CheckedProductBasis or a pass token.

All precommit failures return the untouched ordinary owner plus complete Spent.
Borrow lifetimes prevent concurrent mutation; private owning states and two
non-reused Arc anchors preserve identity across the later move. Any deliberate
post-freeze mutation requires discarding all pass tokens and rebuilding/rechecking;
none is exposed by this private API. There is no self-referential struct: specs
and row views are temporary stack-scope borrows, and successful final proof owns
its identity stamp before those borrows end and OrdinaryOwner moves.

PrivateCommitted remains opaque and cannot be returned by a public entry or
serialized as an eligible product. Later public integration must pre-stage all
identity/receipt/diagnostic changes, bind/hash **this exact certified candidate**,
and finish all fallible hash/encoding/resource checks before an analogous
consuming commit. If a later receipt operation would mutate covered rows or
metadata, invalidate the certificate and recheck; never patch after certification.
That future transaction must also count envelope/output-buffer/caller overlap and
invocation-wide debit. This packet does not implicitly redesign or activate it.
