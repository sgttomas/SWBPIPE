# RV65 dense observation delta

**CLEAR for ROOT selection of the exact seven-file interface delta below.** The
existing six-file fence is insufficient for immutable producer custody of the
dense ancillary and mode metadata. Add only the stated PP/src/lib.rs observation
capture seam. This is a reviewed proposed delta, not clearance of changing I50
source, numerical acceptance, or authorization to alter observation policy.

Basis: delta brief at `d87eb0fb59214a561f37a9ce5dcae4ba4c4bcf75`; unchanged public
producer `d0daa18717f8243a7232e898c9ef9b4f4d18d9e4`; immutable FIRST_RUN manifest
SHA-256 `1773eb7d47e432b8cb1440a95815938951591fd4352b2751909d56886ede7d0e`.
P=projects/chirality-piping; PP=P/core/product_physics;
FK=P/core/solver/frame_kernel. Origins/hashes and first-run facts are in CHECK.json.

## Existing classification and observed correction

T3 ROOT_SELECTION_DESIGNS.md selects D1 revision 5a.2 and D2 revision 5b.2.
D1 DESIGN_NUMERICS/DESIGN.md §4.1.6.1, line474, explicitly lists
`sparse_live_path_dense_parity_relative_delta` among exactly four non_quantity
kinds; D2 DESIGN_STANDING/DESIGN.md §4.9.10, line704, mirrors it. The row is an
existing observation, never a mechanical threshold/scale contributor or a rule
quantity. Baseline source_receipt/rows.rs 704–738 recognizes the same kind with
unitless, solver:sparse_direct, sparse_live_path/reduced_system, finite value >=0.
There is no authority gap and no new classification selection to make.

All twelve FIRST_RUN file hashes/sizes and the preserved manifest copy were
independently verified. The frozen pp_named_second run exited101 on the dense
98-row assertion. Its two I50_RECORD objects establish:

| Mode | Native Q | Final rows | Verdicts | Actual state |
|---|---:|---:|---:|---|
| sparse_interactive |58|98|98|capture/numeric_failure None, numeric_pass false, G5a and observables None, full_case false|
| dense_scrutiny |58|99|0|Association("ordinary sparse mode record"), no complete certificate/G5a verdict|

The dense mode row is index0/value2 (`4000000000000000`), followed by parity at
index1 with value bits `3e2c76280d1333f1` (3.31336034355847e-09). Sparse mode is
value1. Both runs have the three existing hooks once. The frozen test compares
the serialized observed envelope to the ordinary baseline before the failing
98-row assertion. Dense `g5a: None` after the binder error is **not** a performed
G5a check. The sparse false numeric verdict is not repaired by this delta.

**Correction to my sealed named_support_component_01 review:** its
mode-independent final98 census was wrong. It omitted the already existing dense
parity ancillary. The named sparse roster is97 quantity/mechanical rows +1 mode
=98; the observed dense roster is the same97 +1 mode +1 parity =99. Q58 and
3 Reaction/3 SpringAction/18 empty component counts do not change. In general,
the private final census adds the actually produced parity observation (0 or1)
and any separately required material record; dense mode alone does not prove
parity presence. The earlier review and seal remain unchanged and are not
retroactively credited with this check.

## Exact additional source seam

Add one call in PP/src/lib.rs, inside the existing case solve that already has
`mut product: Option<&mut ProductCapture>`, **immediately after** the complete
`if let Some(linear_solve)` mode/parity production block (baseline about3913),
and **before** the following selected_source branch and later row mutations:

```rust
if let Some(observer) = product.as_deref_mut() {
    observer.solver_observations(load_case, solver_mode, &results);
}
```

The method lives only in retained_product.rs with this signature:
`pub(super) fn solver_observations(&mut self, case: &PreviewLoadCase,
mode: PreviewSolverMode, produced_rows: &[ResultItem])`.
Its return is observer state, not a solver Result. Its failure cannot change the
ordinary envelope, diagnostics, observation calculation, branches, timings of
existing solves, or public routes. There is no extra parity invocation. With
product=None it does no capture work. No producer-helper signature or numerical
body change is required: the completed, same-case results prefix already owns
the actual immutable mode/parity fields at this boundary. Capture it before
PP's final qualification adds basis_ref and before the final binder sees rows.

Store actual invocation mode in the existing `invocation` hook, only after its
CapturedInvocation mode check succeeds. Require one invocation/case/observation
completion for this existing one-case private witness. Compare the new call's
case id and mode to the previously captured actual case and invocation. The
observation state must distinguish **not captured** from **completed with no
parity**. Use a checked completion count plus independent `parity_produced`
presence recorded from the actual producer prefix and a separate optional owned
parity snapshot. The source presence fact must not be recomputed from that
optional snapshot or from final rows. Require `parity_produced == snapshot.is_some()`
when binding; it catches loss of both the snapshot and its final row. An outer
optional completed record distinguishes a valid absent state from no hook.
Publish completed state only after all validation/copies succeed. A missing,
duplicate, wrong-case or wrong-mode hook never means legitimate absence.

Scan only for the two closed ancillary kinds in the actual produced prefix;
validate their complete fixed fields and copy owned actual value bits and dynamic
metadata.basis bytes. Do not reconstruct the text from later envelope values,
parse-and-reserialize it, retain a mutable alias, or use only a prefix match.
Keep the record private to this observer/case; repeated reuse is refused. No
general observation framework or extra source snapshot is needed.

## Exact presence, fields and coverage

**Mode.** At this private ordinary nonfallback scope, exactly one mode row is
required: actual invocation sparse implies value bits1.0, dense implies2.0.
Keep code3/dense-fallback outside the previously admitted private scope; do not
coerce it to either mode or infer its producer from a string. Preserve the
existing refusal if it occurs and report it. Capture its actual dynamic basis
text for exact final comparison; fixed fields are:
id=result:solver-mode:linear-solve-basis; kind=linear_solver_mode_basis;
unit=mode_code; entity=solver:linear_static_preview; metadata component=
linear_solver_mode, coordinate_system=reduced_system, location=actual case id,
and the complete existing 1/2/3 sign-convention string. Producer basis_ref=None
and empty source_result_refs are checked. At final binding require the ordinary
qualified basis_ref `{ref_type:load_case, ref_id:captured case}` and unchanged id
for the existing single first/default case. All dynamic basis bytes and value
bits must equal the immutable producing capture, in addition to the mode rule.

**Parity.** Sparse forbids any produced or final parity row. Dense emits one only
when the actual existing outer condition has DenseScrutiny, Some(formed), no W2
publication, and the legacy-dense reduction/observation succeeds, followed by
successful sparse assembly, matching dimensions, sparse solve and finite metrics
in append_sparse_live_path_evidence (baseline5555–5682). The outer block also
requires a linear solve. A skipped outer condition or upstream legacy-dense
failure can produce no row; the helper's failures add the existing unavailable
diagnostic and return without a row. These public branches remain unchanged.

After the terminal hook, dense `completed/parity=None` is an explicit witnessed
absence, not evidence inferred from a missing final row or an unavailable
diagnostic. Require exactly0 final parity rows for that state. A captured parity
requires exactly1 final row, with exact actual value bits and dynamic text.
Removing both a final row and its snapshot must fail completion/presence custody,
not turn the state into an unobserved default. A failure during capture leaves
the sticky capture error and cannot publish completed absence.

Parity fixed fields: id=result:sparse-live:dense-parity-relative-delta;
kind=sparse_live_path_dense_parity_relative_delta; unit=unitless;
entity=solver:sparse_direct; component=sparse_live_path;
coordinate_system=reduced_system; location=literal `load_case` (not the case id);
sign_convention is exactly the existing full string beginning `unitless max
absolute dense-sparse solution delta...; no release threshold asserted`.
Producer basis_ref=None and source_result_refs empty; final basis_ref must be
the captured actual case, just as for mode. The value must be finite and >=0,
with actual bit equality. Preserve existing >=0 semantics: do not invent a new
negative-zero rejection for this non_quantity observation. Same numeric value
or similar text from a foreign case does not establish custody.

**Private typed coverage.** Add one distinct `ProductRecipe::DenseParityObservation`
(name may follow local style), using the existing private ProductUnit::Record
representation after PP has validated the public unitless unit. `NonQuantity`
continues to mean the one mandatory mode record at this facade; it is not a
catch-all bucket. FK run_case tracks a separate at-most-one parity flag and
requires Record, finite nonnegative value and valid row/case/body association.
Use ancillary body0 consistently with existing mode/material rows. Its verdict
has class=None, scale_bits=0, predicates allNone and unchanged value bits, just
as the other typed non_quantity records; this says association succeeded, not
that a numerical bound for parity was proved. It contributes no native,
support-component or derivative coverage and no mechanical scale or G5a work.

PP owns expected presence from the completed producer capture and requires exact
set equality independently of FK's maximum coverage. FK still requires the
mode row; parity cannot substitute for it, nor can a mode/material record be
retagged as parity. Binding retains final row order and one verdict per row,
including parity, without filtering or sorting it out. All97 quantity verdicts,
native Q58, mandatory mode/material laws, support coverage, numerical predicates,
floors, scales, G5a and observable checks remain unchanged.

## Work, controls and exact amended fence

Use existing checked AdapterWork calls for every entered hook/source-row visit,
validation/key/byte comparison, map write, dynamic string copy and capacity.
Check the new hook count and length/capacity arithmetic before mutation/casts;
use fallible reserve/copy. Capture has at most two ancillary snapshots; missing
allocation capacity or duplicate state is not successful absence. Record new
owned strings/containers and successful temporary allocations on failure with
the same actual-capacity conventions; no allocator/RSS/whole-profile claim.
In FK charge the actual new branch/coverage entries using its existing work
discipline. Stop after a prior fault, preserve first error and exact entered
prefix; separately state any intentionally counted call-entry fact. No new tariff,
budget, retry, or independent observation computation is authorized.

Required focused controls under the author's later runtime grant:

- Both actual modes: unchanged ordinary serialized None-path envelope; mode1/2
  exact capture; sparse98 and dense99 for this named successful observation;
  complete per-row output including class=None parity and unchanged97 quantities.
- Explicit completed dense absence (outer skipped and helper unavailable branches
  exercised through a narrow synthetic capture control if a natural fixture is
  unavailable), versus missing capture. Synthetic records are not actual runtime
  absence witnesses. Sparse present parity refuses. Never fabricate public inputs
  or weaken a criterion to force an unavailable branch.
- Missing/duplicate mode, wrong mode bits, fallback3, foreign mode/case capture,
  missing/duplicate completion, mismatched captured mode, missing/extra/duplicate
  parity, and removal of both expected row and snapshot all refuse at the intended
  custody/presence boundary.
- One-bit parity value mutation, negative/nonfinite value, altered dynamic basis
  (same length, whitespace, UTF-8), every fixed field/source refs/basis_ref,
  equal-valued foreign case, missing metadata, wrong type/unit and mode/parity
  substitution. Duplicate parity with a distinct id independently fails typed FK
  coverage; wrong Record unit and missing mode fail without a generic row-id catch.
- New counter overflow/storage/count controls preserve first-failure prefixes,
  don't copy/commit after the fault, and report owned capacities. A no-parity
  completed state and a zero-valued captured parity remain distinguishable.

The full amended maintained fence is exactly:

1. PP/src/retained_product.rs — invocation-mode custody, one terminal ancillary
   capture, complete final binding/presence and checked work/storage, plus the
   previously selected support-component changes.
2. **PP/src/lib.rs — only the one observer call at the producing boundary above.**
3. FK/src/structural/retained/product_certificate/final_case.rs — distinct typed
   parity maximum coverage and ancillary verdict dispatch, retaining mode coverage.
4. PP/src/retained_product_tests.rs — mode-aware census, custody/failure/None-path
   controls and both-mode complete observations.
5. FK/tests/retained_k4/product_final_case_tests.rs — isolated typed parity controls.
6. P/fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json — unchanged
   exact shared input from the original grant.
7. PP/tests/formation_check_runtime.rs — only the previously granted literal
   extraction; existing both-entry/mode Sensitive assertions remain protected.

No source_receipt, semantic table, public schema, numerical producer, kernel
algorithm, routing, origins.rs signature, dependency or observation-policy edit
is required. ROOT may select this concrete delta before implementation. Frozen
implementation/evidence review, including prior RV65-1/2, still follows.
