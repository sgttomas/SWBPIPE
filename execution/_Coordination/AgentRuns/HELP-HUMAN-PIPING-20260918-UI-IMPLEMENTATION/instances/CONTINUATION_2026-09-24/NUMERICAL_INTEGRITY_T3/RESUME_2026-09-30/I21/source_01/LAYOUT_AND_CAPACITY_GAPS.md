# I21 source_01 — exact remaining bindings

This list distinguishes compiler layout facts from unavailable library-source
contracts and run-input values. No witness/overlay was written or run.

## Layout facts genuinely requiring an actual build binding

Use actual types at the final A1 candidate, target and features. Measure the
allocation-bearing aggregate or element, not every intermediate stack struct.

| Group | Required sizes/alignments | Why |
|---|---|---|
| Retained numeric array strides | Wide/OptionWide for L4,L8,L16; MemberOperators, BoundedCoefficients, DirectionalBlock, PivotScreen, BlockBound, BlockCertificate, BlockNorms, BodyReport; residual tuple | Direct Vec element allocation strides; A1-sensitive |
| Retained control strides | Contribution; tagged tuple; ledger-build/ledger-entry; lazy/table/fallback rows; Option<Option<GateRatio>>; Option<(bool,Tracker)>; start/result tuples; OptionBoundRefusal; PrecisionState; GroupEntry | Mixed-enum/tuple padding and nested-option niches are not proved by mirrored structs |
| Owned aggregate requests | Arc payloads CasePrep, GroupPrep, Shared(4,4)/(4,8)/(8,16)/(16,16), Solved4/8/16, VerifyShared(4,8)/(8,16)/(16,16), VerificationReport4/8/16; Box<RetainedSolve> | Account header/padding and inline Vec/enum headers exactly once; children counted separately |
| Exported/consumer strides | Source element types, QuantityMeta/PublishedRow/AttemptRecord/BlockRefusal/BodyGeometry, Expansion/[Expansion;6], H K6Model tuples/FrameElement, VR Case/Model/Row/Control/Member/SpringSpec, Value/Map/String, sparse report types and helper tuples | Existing public size_of can cover many without exposing FK private internals; witness scope should distinguish these |
| Actual tree node request layouts | Geometry u32->usize; tracker-key->Tracker and holding key set; usize set; QuantityId->PublishedRow; String->(f64,f64), String->String, String->Value; borrowed &str->&Row; sparse pair->(f64,usize) | Need actual leaf/internal request sizes or a proved conservative node request envelope. sizeof(BTreeMap) is only its inline header |

Not needed solely for this table: sizeof(ShiftedFactor), sizeof(ScaledProfile),
sizeof(RetainedFactor), sizeof(SparsePositiveFactor) as separate heap requests:
their stack wrappers are not independently boxed. Their vector element buffers
and containing Arc payloads are already named. Avoid resurrecting an indiscriminate
126-type overlay programme. The actual list can be reduced by equivalent generic
instantiations only after checking type/layout identity.

## Unavailable source contracts (not solvable by a size-only witness)

The standard rust-src path for installed 1.97.1 was absent. The response
SOURCE_INPUTS pins historical hashes; those source bytes were not requalified
on this host. No installation/recovery followed. Exact needed assertions:

1. RawVec reserve recurrence/minimums and the requested Layout for with_capacity,
   vec![x;n], clone and reserve/resize. A single observed capacity is not a proof
   for all finite shapes/branches.
2. Vec TrustedLen versus GenericShunt/fallible/filter collection, owned-map
   reuse, iterator buffer destruction, shrink behavior, stable-sort scratch,
   VecDeque growth. Conservative alternative: permit input+output and old+new,
   retain prior high-water capacity; no next_pow2 assumption after shrink.
3. BTree insertion/split/high-water/empty-root, from_iter vector+sort+bulk,
   clone topology. Conservative alternative is an independently justified
   insertion request-volume envelope, not amortized '32 bytes per key'.
4. Arc requested header/padding and Box layout; actual request identity is
   needed separately from child payload.
5. Finite String/format Debug growth and numeric grammar; stdout retained
   buffer; Unix argv/path/file-read and exact-file EOF behavior.
6. Locked serde Value parse/clone/to_value writer ownership, parsed String
   scratch and map representation. Manifests request float_roundtrip;
   no compiled feature claim was made.

These prevent a numeric universal requested-heap bound. They do not prevent
reviewing the source owner/lifetime formulas now. A bounded compiled witness
can bind actual layout; it cannot replace missing capacity/source reasoning.
ROOT can supply/rely on an authenticated existing library-source package in
a later bounded grant, but this TASK requests no installation or tooling lane.

## Finite values / source checks still needed

- Enriched descriptors for the original 33 H fixtures and actual VR CI models:
  degrees/pre-dedup counts, source string capacities, loaded-DOF multiplicities,
  geometry NotAssessed children, actual array capacities by construction path.
  The equations preserve springs/bodies; the 193-case assertion is not weakened.
- Bind planned argv, paths, stable input F/hash and full large-model bytes.
  Only the family/expected-list file byte/hash facts were freshly read here.
- Validate current JSON shape/string histograms and all exact comparison
  decimal/exponent descriptors. Previous response values are candidate inputs.
- Finish numeric request composition for each actual emit/error JSON object,
  parse scratch, path/read/argv and runtime-owned baseline. H only needs its
  staged reason plus retained caller owners; VR needs its whole finite window.
- Verify the decision-rule offer/key bounds and complete stop-decision summary/
  floor child union at the final A1 source. K19 gives correct lifetime
  alternatives; imported 768/4608/2816 caps and narrowed key counts still need
  full review/binding, not an unexamined constants copy.
- Validate kernel shape/capacity terms in FORMULAS against all source entry
  branches; especially append/reuse capacities, geometry successful witness,
  refusal/report/evidence child clones, and verification operands Vec.
- Independently review the whole final formula, then implement/check/replay
  admissions and W1-T4 only under subsequent ROOT grants.

## Smallest remaining decisions

1. Retain the concrete shared H-library API/dependency proposal or choose
   another explicitly shared definition. The preferred proposal preserves
   all existing count/test shapes and separates binary metrics.
2. Select an authenticated capacity/library basis and narrowly scoped actual
   layout witness after final A1. Neither source absence nor numeric gap grants
   tooling work; compilation remains ungranted here.
3. Bind a finite runtime/argv/input envelope for the existing VR global metric
   and H staged metric. An observed baseline may support a run-specific
   observation, but cannot silently establish a universal constant.

A1-sensitive: every retained numeric/control/Arc layout, helper path, phase
maximum, refusal/summary/result child, geometry text and serialized outcome
shape. Consumer-only parser/dependency algorithms need recheck if their sources,
features or inputs change; they are not assumed numerically affected by A1.

