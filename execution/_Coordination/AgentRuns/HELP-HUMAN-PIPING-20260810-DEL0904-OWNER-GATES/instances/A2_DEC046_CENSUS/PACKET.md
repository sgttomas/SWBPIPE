# DEC-046 release-convergence value packet and blocked-mechanics census

**Packet status:** `OWNER-READY / NON-AUTHORITATIVE / RULING REQUIRED`
**Prepared by:** `A2-DEC046-CENSUS`, a fresh non-delegating ephemeral Agent 2
**Repository basis:** `c05fe2d6fbc3bd3d3b690f50075e2c878af0faf3`
**Date:** 2026-08-10
**Scope:** value preparation under D-19 / `DEC-046` and report-only census of
the 13 mechanics cases recorded blocked in the committed DEL-09-01 benchmark
bundle. Nothing in this packet is a ruling, threshold, repair, promotion,
acceptance, release, or edit to governed project state.

## 1. Executive finding and recommendation

There are two numerical axes in the cited sources, and they must not be
collapsed:

1. **Verification comparison** compares an observed benchmark value with a
   reference value. Mechanics and stress currently expose an internal,
   absolute assertion epsilon of `1.0e-9`; nonlinear runner regression uses
   exact equality for its recorded state/residual projection; sparse evidence
   records separate parity and residual metrics. D-04 / `DEC-026`, not
   D-19 / `DEC-046`, governs the class-tiered verification structure.
2. **Solver convergence** decides when an assembled nonlinear active-set loop
   stops. D-19 / `DEC-046` expressly governs this axis, separately from
   `DEC-024` / `DEC-026`. Its accepted current validation-seed record uses the
   integer `active_set_changed_support_count`, with relative field `0`,
   absolute floor `0`, and cap `4` for every named class.

The owner-facing DEC-046 recommendation is **Option O-B: promote the existing
accepted validation-seed convergence values unchanged into a new, separately
identified public-benchmark release-scope record**:

| nonlinear class | relative residual field | absolute residual floor | max iterations |
| --- | ---: | ---: | ---: |
| one-way | `0 count` | `0 count` | `4` |
| gap | `0 count` | `0 count` | `4` |
| lift-off | `0 count` | `0 count` | `4` |
| friction | `0 count` | `0 count` | `4` |
| multi-support / multi-DOF | `0 count` | `0 count` | `4` |

This is recommended over a measured-minimum cap because it retains one to two
iterations of deterministic headroom while preserving the essential invariant:
no active-set support may still be changing at convergence. It is recommended
over a cap of `6` because no release-scope evidence in the inspected corpus
requires iterations 5 or 6.

**Important limitation:** selecting O-B can rule the DEC-046 convergence
values, but it cannot by itself supply final mechanics/stress analytic
comparison pairs under DEC-026. If the intended owner act is instead a
verification-tolerance promotion across mechanics/stress quantities, the
lawful result of this packet is **DEFER that separate value act**: committed
evidence does not contain complete per-kind absolute floors, the 13 blocked
cases never reach comparison, and the current mechanics inventory has advanced
from 24 to 25 without a current whole-suite capture. An owner ruling should say
which axis it decides.

All 13 recorded mechanics blocks are classified **implementation** (high
confidence), with a secondary public-observation/runner-binding coverage gap.
They are not data failures and not tolerance failures: every case has recorded
expected data, but the runner returns `observed=null` and stops before any
tolerance predicate. Therefore **O-A, O-B, and O-C unblock zero of the 13**.

## 2. Authority boundary and the source contradiction

The accepted D-19 codification states that `DEC-046` creates a convergence
record keyed by gap, one-way, lift-off, and friction, with a relative residual
tolerance, absolute residual floor, and max-iteration cap. It also says the
record is separate from `DEC-024` / `DEC-026` verification tolerances and that
unmeasured entries remain `TBD`.

That boundary is explicit in:

- `execution/_Coordination/_DECISIONS/D-19_release_convergence_tolerance_policy.md`,
  Git blob `9275cd9c44f1af84b443910e08c8ba73aa0e8791`;
- `execution/_Decomposition/SOFTWARE_DECOMP.md` §12, row `DEC-046`; and
- the D-19 register row, which calls the subject the “solver-convergence axis”
  and distinguishes it from the analytic verification seed.

By contrast, DEL-09-04 `_STATUS.md` and TM-PIP-037 use the shorthand “final
public-benchmark release tolerances under the DEC-046 convention.” The
shorthand is not itself enough authority to move mechanics/stress numeric
comparison values onto the convergence axis. This packet resolves the phrase
by presenting an exact DEC-046 convergence ruling and fencing the distinct
DEC-026 verification values.

## 3. Current internal comparisons and committed observations

### 3.1 Comparison semantics actually implemented

| suite/evidence lane | current comparison operation | status of number |
| --- | --- | --- |
| Mechanics | finite operands and inclusive `abs(observed-recorded) <= 1.0e-9` | crate-internal absolute assertion constant; explicitly not release policy |
| Stress | finite operands and inclusive `abs(observed-recorded) <= 1.0e-9` | crate-internal absolute assertion constant; explicitly not release policy |
| Nonlinear runner regression | exact equality of residual count, support states, changed-support IDs, converged flag, and diagnostic codes | regression predicate; not the assembled-loop release convergence rule |
| DEC-046 assembled convergence | `active_set_changed_support_count <= effective_tolerance`, where current runtime computes `effective_tolerance = max(relative_residual_field, absolute_residual_floor)` | current validation-scope policy record; release scope not yet promoted |
| Sparse observation | recorded `sparse_dense_relative_delta`, absolute sparse/dense solution delta, absolute residual, repeat delta, and nonpositive pivots | DEC-050/053 bounded sparse evidence; expressly not a release/external-validation threshold |

For the active-set count basis, both stored tolerance members are in `count`,
despite one field being named “relative.” There is no reference magnitude and
no `rtol * scale + atol` operation in the live convergence consumer. Because
the residual is a nonnegative integer count, any effective tolerance in
`[0,1)` is behaviorally identical to zero; a value `>=1` would permit a still-
changing support to be declared converged. No inspected evidence supports that
semantic change.

For a future dimensioned relative-plus-absolute verification comparison, this
packet recommends an inclusive, finite rule of the form
`abs(observed-reference) <= max(absolute_floor_kind,
relative_tolerance_kind * abs(reference))`. That is a proposed algebra only;
the accepted sources establish the pair/floor structure but do not provide a
complete public per-kind value table. It must not be silently substituted for
the current absolute `1.0e-9` mechanics/stress helper.

### 3.2 Exact committed observations by suite

| suite | committed evidence | population | observed comparison result | maximum recorded discrepancy |
| --- | --- | ---: | --- | ---: |
| mechanics | `BENCHEVID_DEL0901_20260720T062342Z_e315fb8406d4/SUITE_RUN_MECHANICS.json` | 24 requested: 11 matched, 0 mismatched, 13 blocked; 91 numeric comparisons among matched cases | 11/11 observable cases match under absolute `1e-9` | max absolute `3.552713678800501e-15`; max nonzero-reference relative `1.7763568394002505e-15` |
| stress | R14 `outputs/del1005_benchmark_multi_case.json` | 3 requested/matched; 11 numeric comparisons | all exact in the capture | `0` |
| nonlinear | R14 `outputs/del1005_regression_full_suite.json` | 5 requested/matched; 5 residual comparisons plus exact categorical projections | all exact; 2 recorded converged, 3 recorded nonconverged, exactly as expected | `0` |
| sparse | `sparse_default_promotion_observation.dec053.json` | 9 observations | all repeat deltas `0`; all nonpositive-pivot counts `0` | relative sparse/dense solution max `7.060341894958857e-11`; absolute solution delta max `5.342535303043405e-10`; absolute residual max `1.0058283805847168e-7` |

Mechanics discrepancies by quantity dimension (matched cases only):

| dimension | comparisons | max absolute delta | max relative delta for nonzero reference |
| --- | ---: | ---: | ---: |
| dimensionless | 9 | `0` | `0` |
| displacement | 6 | `1.734723475976807e-18 m` | `1.2390881971262908e-16` |
| force | 42 | `1.7763568394002505e-15 N` | `4.440892098500626e-16` |
| length | 7 | `0 m` | `0` |
| linear stiffness | 2 | `0 N/m` | `0` |
| moment | 20 | `3.552713678800501e-15 N-m` | `1.7763568394002505e-15` |
| rotation | 5 | `8.673617379884035e-19 rad` | `2.0016040107424698e-16` |

The 13 nonzero mechanics deltas occur in four cases:
`MECH-TP-PHYS-004`, `005`, `006`, and `007`. Exact numeric equality would
therefore admit 7 of the 11 observable mechanics cases and exclude 4; the
internal absolute `1e-9` predicate admits all 11.

Sparse raw observations sufficient to recompute the maxima:

| fixture | relative sparse/dense delta | max absolute solution delta | max absolute residual |
| --- | ---: | ---: | ---: |
| invented-cantilever-chain-8 | `1.170938346284331e-14` | `1.2490009027033011e-15` | `1.5279510989785194e-10` |
| invented-cantilever-chain-24 | `1.8417674793016713e-11` | `5.304290340291118e-11` | `1.1175870895385742e-8` |
| invented-cantilever-chain-48 | `2.318808725225492e-11` | `5.342535303043405e-10` | `1.0058283805847168e-7` |
| invented-grid-frame-4x3 | `1.003337665119566e-13` | `4.811147140404426e-18` | `4.3655745685100555e-11` |
| invented-grid-frame-6x8 | `5.529307789126628e-13` | `7.402390332644782e-17` | `1.0260237104375847e-10` |
| invented-grid-frame-7x8 | `3.393440690140549e-13` | `3.813681141717762e-17` | `1.2168044349891716e-10` |
| invented-grid-frame-5x5 | `2.6781827357134942e-14` | `2.3310346708438345e-18` | `5.820766091346741e-11` |
| invented-cantilever-chain-32 | `7.060341894958857e-11` | `4.819860066618276e-10` | `3.073364496231079e-8` |
| invented-grid-frame-5x6 | `8.327116333388005e-14` | `9.378348791999613e-18` | `7.295852810784709e-11` |

Every sparse row also records repeat-solution delta `0` and nonpositive pivot
count `0`. DEC-050's bounded generated-grid policy uses relative limit `1e-9`,
absolute sparse residual limit `1e-6`, repeat delta `0`, and nonpositive pivot
count `0`; these values admit all nine DEC-053 observations, but the records
expressly exclude release/external-validation thresholds.

## 4. DEC-046 numeric options

All options retain effective residual threshold `0 count`. The only materially
distinct value is the class cap. A positive fractional tolerance below one is
not a distinct behavior for an integer count; a tolerance at least one is not
offered because it would accept a changing active set.

### O-A — observed-minimum class caps

| class | relative | absolute floor | cap | derivation |
| --- | ---: | ---: | ---: | --- |
| one-way | 0 | 0 | 2 | accepted seed observations complete in at most 2 |
| gap | 0 | 0 | 2 | accepted seed observations complete in at most 2 |
| lift-off | 0 | 0 | 2 | accepted seed observations complete in at most 2 |
| friction | 0 | 0 | 2 | accepted seed observations complete in at most 2 |
| multi-support / multi-DOF | 0 | 0 | 3 | the cascade fixture records 3; the other 12 accepted fixtures record 2 |

This is the tightest observed envelope. It admits all 9 accepted assembled
seed cases and all 13 accepted multi-support cases in current committed source.
It excludes any future case requiring one extra stabilization iteration. It is
more brittle than needed for a public release gate.

### O-B — promote the accepted validation values unchanged (**recommended**)

Every class uses `{relative=0, absolute=0, max_iterations=4}`. This admits the
same 22 current accepted assembled fixtures and leaves deterministic headroom:
two iterations above the one-way/gap/lift-off/friction maxima and one above the
multi-support maximum. It preserves the existing record's failure boundary and
does not loosen the changed-support criterion.

### O-C — six-iteration headroom

Every class uses `{relative=0, absolute=0, max_iterations=6}`. This admits the
current 22 and any otherwise-stable case that first reaches zero changed
supports in iterations 5 or 6. It is not recommended: raising the cap is
explicitly a new governance event, and no inspected accepted release-scope
observation needs the extra two iterations. A larger cap can mask iteration
churn and increases bounded work without evidence of benefit.

### O-D — defer release-scope promotion

Retain the validation-scope records and `TBD` release scope. This is the only
lawful selection if “final public-benchmark tolerances” is intended to include
mechanics/stress verification values or sparse release thresholds, because
those axes lack a complete current public per-kind evidence set.

### Admits/excludes matrix

| evidence population | O-A | O-B | O-C | reason |
| --- | --- | --- | --- | --- |
| 9 accepted assembled nonlinear seed cases | admits 9 | admits 9 | admits 9 | observed max 2, final changed count 0 |
| 13 accepted multi-support nonlinear cases | admits 13 | admits 13 | admits 13 | observed max 3, final changed count 0 |
| future assembled case completing at iteration 3 | only multi-support class admits | admits | admits | class caps differ |
| future assembled case completing at iteration 4 | excludes | admits | admits | cap boundary is inclusive through loop budget |
| future assembled case completing at iteration 5–6 | excludes | excludes | admits | O-C only |
| any case with changed-support count 1 at final evaluated iteration | excludes | excludes | excludes | effective threshold remains 0 |
| 11 observable mechanics cases | no effect | no effect | no effect | verification axis, not DEC-046 convergence |
| 13 recorded blocked mechanics cases | unblocks 0 | unblocks 0 | unblocks 0 | runner never produces observed counterparts |
| 3 stress cases | no effect | no effect | no effect | verification axis |
| 5 legacy nonlinear runner regression cases | no promotion effect | no promotion effect | no promotion effect | exact recorded-outcome regression, not assembled convergence acceptance |
| 9 sparse observations | no effect | no effect | no effect | sparse parity/residual policy is separate |

## 5. Exact census of the 13 recorded mechanics blocks

### Root cause common to all 13

`core/runner/headless/src/benchmark_binding.rs` maps 11 fixture IDs to public
observed-value functions. Every other mechanics ID reaches the catch-all
`CaseEvaluation::NotReusable`. `compare_case` then emits all recorded expected
values with `observed=null`, `delta=null`, and `within_recorded_basis=null`.
The committed whole-suite record repeats the same fail-closed reason for every
blocked case: the suite does not expose a public value-addressable observed
counterpart for every recorded value, so the runner refuses to re-encode the
crate-internal check or invent a tolerance.

That proves the primary class is **implementation**, specifically runner/public
observation binding. “Fixture” is a secondary contributor only in the narrow
sense that some fixture APIs expose a boolean validator or partial solve result
rather than a complete named value projection. The expected fixture data are
present. No comparison predicate is reached, so “tolerance” is not the cause.

| # | fixture | recorded values / kinds | primary | secondary contributor | confidence | effect of O-A/O-B/O-C |
| ---: | --- | --- | --- | --- | --- | --- |
| 1 | `MECH-CANTILEVER-TIP-FORCE` | 2: length, moment | implementation | public solver returns tip displacement but not a complete named projection including fixed-end moment | high | remains blocked under all |
| 2 | `MECH-STRAIGHT-PIPE-WEIGHT-RECOVERY` | 2: force/length, force | implementation | crate exposes a boolean validator, not value-addressable runner output | high | remains blocked under all |
| 3 | `MECH-SUPPORT-BOUNDARY-MIXED` | 3: count, stiffness, rotation | implementation | boolean preparation validator; heterogeneous outputs not projected | high | remains blocked under all |
| 4 | `MECH-PRIMITIVE-LOAD-PREP` | 3: force, force/length, length | implementation | boolean preparation validator; named outputs not projected | high | remains blocked under all |
| 5 | `MECH-FIXED-FIXED-THERMAL-AXIAL` | 2: ratio, force | implementation | crate-internal formula assertion only; no runner observation binding | high | remains blocked under all |
| 6 | `MECH-IMPOSED-DISPLACEMENT-SPRING` | 1: force | implementation | boolean support-preparation validator; no named observed value | high | remains blocked under all |
| 7 | `MECH-INCLINED-MEMBER-TRANSFORM` | 2: ratios | implementation | boolean transform/symmetry validator; no named observed projection | high | remains blocked under all |
| 8 | `MECH-EXPANSION-LOOP-CURVED-BEND-THERMAL` | 21: displacement, force, moment | implementation | solver/validator exists but runner has no 21-value mapping; fixture-local model uses separate measured comparison constants | high | remains blocked under all |
| 9 | `MECH-CURVED-BEND-DISTRIBUTED-FIXED-END` | 42: force, moment | implementation | solver and normalized-deviation accessor exist, but runner has no 42-value mapping | high | remains blocked under all |
| 10 | `MECH-CURVED-BEND-PRESSURE-THRUST-ARC` | 12: displacement, force | implementation | solver and normalized-deviation accessor exist, but runner has no value mapping | high | remains blocked under all |
| 11 | `MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC` | 6: mass/length, force/length, force | implementation | fixture/test calculations exist but no runner observation mapping | high | remains blocked under all |
| 12 | `MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE` | 8: force/length, force | implementation | fixture/test calculations exist but no runner observation mapping | high | remains blocked under all |
| 13 | `MECH-CONSTANT-EFFORT-SUPPORT-APPLIED-LOAD` | 5: force, length, moment | implementation | public solve exists but runner has no complete named mapping | high | remains blocked under all |

### Data/tolerance/fixture/implementation totals

| primary class | count |
| --- | ---: |
| data | 0 |
| tolerance | 0 |
| fixture | 0 |
| implementation | 13 |

This is a report-only classification, not a defect disposition. Repair would
require a separately authorized runner/public-observation tranche and must
preserve the suite crate as the source of fixture identity, expected values,
and predicates.

## 6. Currency caveat: “13 currently blocked” is not proven at this base

The bundle is immutable and truthful for its base
`e315fb8406d44dce684cbec091f3174c261efee4`: 24 fixtures, 11 matched, 13
blocked. Current mechanics source has Git blob
`eb65e53075110995a4ddcd93b4181b15392f91d5`, asserts an inventory of **25**,
and adds `MECH-TP-DEC092-TEMPERATURE-INDEXED-SHEAR-MODULUS-TORSION` in commit
`c394365ca72b8383c7d7203ce5be2cb9ea67d508` (2026-08-03). The runner binding
blob remains `75fa69df616dd803ebd8409683d2468536c4b6ac` and has no arm for that ID.

Therefore:

- fact: the exact committed census contains the 13 rows above;
- fact: the current inventory contains one additional fixture;
- inference: a current whole-suite run would route the DEC-092 fixture through
  the same not-reusable catch-all and would likely record 14 blocked;
- unknown: no committed current-base whole-suite capture proves the present
  aggregate count.

No owner ruling should call 13 the current complete inventory without either
constraining the claim to the July 20 bundle or authorizing a fresh evidence
capture. This caveat does not alter the requested 13-row census.

## 7. Risks and unresolved ambiguity

1. **Axis ambiguity:** “under DEC-046” cannot lawfully promote mechanics/stress
   verification values. Those belong to DEC-026. The owner ruling must name
   convergence or verification explicitly.
2. **Scope expansion:** existing DEC-046 JSON records say validation seed only
   and “not a release.” A release-scope value must be a new derivative/governed
   record, not a silent status rewrite of historical evidence.
3. **Residual limitation:** changed-support count proves active-set stability,
   not a dimensioned force, displacement, work, or energy residual. Existing
   policy files expressly exclude external validation and release scope.
4. **Comparison incompleteness:** 13 historic mechanics cases and likely the
   new DEC-092 case have no runner-observable projection. A tolerance cannot
   cure missing observations.
5. **Fixture-specific constants:** some mechanics fixtures encode separate
   comparison constants (for example the expansion-loop approximation uses
   measured relative and absolute bounds). They are not automatically governed
   public release values and may conflict with a blanket `1e-9` verification
   table.
6. **Unit basis:** current mechanics data use an explicit fixture-local
   N–m–rad–K basis; project-grain unit-catalog acceptance remains a separate
   hold. Absolute floors must carry quantity units and cannot be one
   dimensionless global epsilon.

## 8. Exact owner ruling form

The owner can copy one of the following without changing the axes.

### Recommended DEC-046 ruling

> **DEC-046 RELEASE-CONVERGENCE VALUE RULING — O-B.** Promote into a new
> public-benchmark release-scope governed record the active-set changed-support
> count convergence values `{relative_residual_field=0 count,
> absolute_residual_floor=0 count, max_iterations=4}` for one-way, gap,
> lift-off, friction, and multi-support/multi-DOF. The comparison is inclusive
> and requires final changed-support count zero. This ruling does not select or
> promote mechanics/stress verification tolerances under DEC-026, does not
> promote sparse release thresholds, does not repair or unblock the 13 recorded
> mechanics cases, and creates no acceptance, lifecycle, reliance, or release
> act. Apply only through a subsequent bounded implementation and evidence
> tranche.

### Alternatives

> **DEC-046 RELEASE-CONVERGENCE VALUE RULING — O-A.** Select the same zero
> count threshold with caps one-way=2, gap=2, lift-off=2, friction=2,
> multi-support/multi-DOF=3, with the same fences.

> **DEC-046 RELEASE-CONVERGENCE VALUE RULING — O-C.** Select the same zero
> count threshold with cap 6 for every class, expressly accepting the unevidenced
> additional iteration headroom as a new governance event, with the same
> fences.

> **DEC-046 RELEASE-CONVERGENCE VALUE RULING — DEFER.** Make no release-scope
> value promotion. Retain the existing validation-scope records and all
> release/external-validation TBDs.

If the intended act is the separate verification axis, use:

> **DEC-026 PUBLIC-BENCHMARK VERIFICATION VALUES — DEFER.** The current packet
> does not establish a complete per-kind relative-plus-absolute public value
> table; prepare a current 25-fixture observable sweep and a governed per-kind
> value packet before ruling.

## 9. Conditional application mechanism

Only after an owner ruling, a bounded managed application should:

1. preserve the existing validation records byte-identically and create a new
   release-scope governed record with a new record ID and explicit source
   pointers;
2. bind only the ruled convergence fields and classes, without changing
   relaxation, line search, sparse selection, fixture data, or case pages;
3. execute a fresh current-base assembled nonlinear evidence capture and prove
   every governed case reaches changed-support count zero within the ruled cap;
4. keep mechanics/stress verification tolerances, sparse thresholds, and every
   unmeasured quantity `TBD` unless separately ruled;
5. route any runner-observation repair as a separate bounded implementation
   tranche; and
6. return the resulting evidence to the owner before any TM-PIP-037 register
   disposition, DEL-09-04 Remaining edit, promotion, lifecycle, or release act.

## 10. Committed source identity index

| source | Git blob | SHA-256 | role |
| --- | --- | --- | --- |
| `D-19_release_convergence_tolerance_policy.md` | `9275cd9c44f1af84b443910e08c8ba73aa0e8791` | `82dc283c740dc1d3cc83399bbce3b7a45b40d2f2c57c28a7f9417d47aa1c01c7` | DEC-046 axis, CV-B structure, fences |
| `D-04_tolerance_coverage_thresholds.md` | `c6b12371a8473724d0d0e5d8e6be2d26e4e9f8e3` | `2f4b4cc44e7c74d60640f80b13cb1a46b91677903b56e6b1d45a4adfc5115b4a` | DEC-026 verification axis and `1e-9` analytic seed |
| `validation/benchmarks/nonlinear/convergence_policy.dec046.json` | `d3850f5becda48c523dfb15cd5d87aabe54220c0` | `bcf6a0a3afc02e12b08b82a790150373973091625d06ab8fd470d6f30efdc552` | current four-class values |
| `validation/benchmarks/nonlinear/multisupport_convergence_policy.dec046.json` | `a6f43a50aa7682d85409f1d25b8982aeff7bd6a9` | `0ed5025fc48077b669e4a3984f492c2e2645172051743ce6fac9f80d640e4508` | multi-support values and 13-fixture evidence set |
| `validation/benchmarks/nonlinear/src/lib.rs` | `37d19abf27a17c1d1333a81732d0b413e4ca2880` | `ff3f318224fa2c55392bfa17569182768453b3157cf35fc20d763f54bf02fd12` | observed iteration counts and exact residual assertions |
| `core/solver/nonlinear_integration/src/lib.rs` | `36062a62a95da7efe2bf801a7dad8f3c398f6c9d` | `9ae6eb2cab93ed034b394fc12d99e6dd7f555359b77bb5eafb9d7ccc9fa4bd93` | live effective-tolerance operation |
| `validation/benchmarks/mechanics/src/lib.rs` | `eb65e53075110995a4ddcd93b4181b15392f91d5` | `8f015dfeeb1fd670065f467335419ab0da8b0c9a4e6d6b3fb565ceea2d6f6a26` | current `1e-9`, current 25-fixture inventory, fixture APIs |
| `validation/benchmarks/stress/src/lib.rs` | `201f5d84d1a666975000d07fb8e21900b88f9807` | `ad7239a073f5ca6c161ee3f63642d487414efed6e3176ae86a2af7979b73210c` | current `1e-9` absolute predicate |
| `core/runner/headless/src/benchmark_binding.rs` | `75fa69df616dd803ebd8409683d2468536c4b6ac` | `1dadf7636f99b9a1931e76daf28bfcf3c49e502cecef9b51c4fd351711050d39` | mapped cases and fail-closed catch-all |
| mechanics evidence `MANIFEST.json` | `c325e8ee8da95a0b2bda499ba4c013060cf362b5` | `32e3943047e18b2b289beeda9b369369599e42c79d2007fe03aaa3b5ccc18804` | bundle identity and 24/11/13 summary |
| mechanics `SUITE_RUN_MECHANICS.json` | `c0fee1b6581169e4427686e5c1932ec6176f5ceb` | `d1e8c76b1c7c162fba1d746f57c593c874629e7b6b0175fdbb4f63fd5afbd761` | exact 13-case census and 91 comparisons |
| R14 stress output | `2feb1b9cf0abc0f978457a6e1564db4596239a22` | `8feb3d25e50e78dcd7fcc85e2253021610faa971a37837eefb63df5cea456d68` | 3 stress cases / 11 comparisons |
| R14 nonlinear output | `ce8dc556b66dc4384b87acc73829129f1b70b719` | `2f89adce9e4d6250280cee347822a567f4405eafbb8bc666483c6ce4cbd87593` | 5 exact regression cases |
| sparse DEC-053 observation | `ad57a55ca33e9b3f7f1a57eb196db52adc49484c` | `d49c7923c224fccdbe0c8885bd19564cdc5a78bfefbf1b6e83d2f027cad71f24` | nine sparse observations |
| Task Management `REGISTER.csv` | `4d19f55bd90dac938b12b970abf4d3729daa0154` | `2175d2c4db7a480cd6ff77b9964d3815ff7558361df3a132838763d49a49ebfe` | exact deferred TM-PIP-037 trigger |

## 11. Explicit fences

This packet is report-only. It performs no repair; no data, fixture, suite,
runner, solver, policy, case-page, manual, evidence bundle, register, status,
receipt, decision, lifecycle, reliance, acceptance, promotion, threshold,
release, or Git effect. It does not address GUI-workflow validation evidence,
the export/CAEPIPE TBD cluster, D-61 reliance posture, lifecycle transitions,
or release acts.
