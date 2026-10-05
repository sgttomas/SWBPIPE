# RV77 derivation: `rederive_coverage` against native `summary_coverage_data`

Source: candidate `c618675e84`, read from RV77's own `git archive` (sha256 of
`final_case.rs` 3986919726e962b5…, equal to I61's frozen bytes). FK means
`P/core/solver/frame_kernel/src/structural/retained`. Line numbers are at the
candidate.

## 1. Native function (FK/product_certificate/final_case.rs:1461–1515)

For body b, over the rows of `owner.prep.layout` with `meta.body == b`:

1. `present[k]` = some row of kind k exists.
2. `positive[k]` = some row of kind k has `!input_derived` and a nonzero
   verification value at P = 2p (`verification_nonzero`, :1423–1453). Call this A[k].
3. If `owner.prep.extents[b] != 0.0`: positive = [A0∨A1, A0∨A1, A2∨A3, A2∨A3].
4. If `evidence().floor` is `Some`: positive[2] |= floor_f > 0; positive[3] |= floor_m > 0
   (after the coupling; a missing body entry is an association refusal).
5. `stop[k]` = OR over rows of kind k of (positive[k] ∨ nonzero(row)), which is
   present[k] ∧ (positive[k] ∨ A[k] ∨ D[k]), D = some input-derived row nonzero.
6. hats = [E_f > 0, E_m > 0]; if extent != 0.0, hats = [h0∨h1, h0∨h1].
7. estimate = [present[2] ∧ hats[0], present[3] ∧ hats[1]].
8. charge = p == 512 ? [present[2] ∧ positive[2], present[3] ∧ positive[3]] : estimate.
9. has_data = OR over data blocks i of (data[i] ∧ blocks.body[i] == b).

## 2. Rederivation (final_case.rs:263–281) and its facts (:283–310)

Inputs: the compact payload {body, stop[4], has_data} copied from the proof's own
entry, and facts {present, extent_bits, resolution_bits, floor_bits, precision}
drawn from the same owner: `prep.layout` (presence), `adaptive::body_extent` of
the body's nodes (checked bit-equal to `prep.extents[b]`), `evidence().resolution_scale`,
`evidence().floor`, `selected_precision()`.

- estimate: identical expressions to steps 6–7 on identical inputs. The coupling
  test `f64::from_bits(extent_bits) != 0.0` equals `prep.extents[b] != 0.0` because
  the bits are required equal (and −0.0/NaN compare the same way in both).
- charge, p ∈ {128, 256}: estimate in both.
- charge, p = 512: native is present[2+j] ∧ positive[2+j]; the rederivation is
  stop[2+j] = present[2+j] ∧ (positive[2+j] ∨ A[2+j] ∨ D[2+j]). FK/recover.rs:101–198
  marks only the displacement/rotation DOF rows (`component_kind(.., true)`) input-
  derived, so D[2] = D[3] = false; and A[2+j] ⇒ positive[2+j] already at step 2
  (steps 3–4 only OR more in). Hence stop[2+j] = present[2+j] ∧ positive[2+j]. The
  floor is ORed into positive before stop is formed, so it is inside both sides.
- stop and has_data are carried, not rederived; the comparison checks them only
  through the p512 charge identity and the consistency refusals.

## 3. The extent recomputation

`CasePrep::with` (FK/adaptive.rs:1023–1032) computes `extents[b] =
body_extent(coords of source.body_nodes(b))`, and `body_nodes` (FK/source.rs:766–770)
is `(0..nodes.len()).filter(|n| body_of_node[n] == b)`, ascending. `coverage_facts`
builds exactly that sequence (same filter, same ascending order, same `source` via
`owner.source() == &prep.source`) and calls the same `adaptive::body_extent`
(adaptive.rs:321–335). The result is therefore bit-identical for every genuine
owner; the equality check can only fire on a foreign or altered owner. (min/max
with signed zeros cannot change the result because each d_a is squared.)

## 4. The added refusals hold for every genuine vector

| Refusal | Why every genuine vector passes |
|---|---|
| (128\|256, None) or (512, Some) only | `RetainedSolve` is constructed only in `finish_selected` (adaptive.rs:4919) after `certify_publication` accepted, which refuses `(candidate.precision()==512) != floor.is_some()` and a floor of the wrong length (adaptive.rs:3591–3594); `decision.floor` is set only when the verification precision is 1024 (adaptive.rs:2290–2307). |
| no stop on an absent kind | native stop[k] is only ORed inside the row loop for rows of kind k. |
| present force/moment with positive floor ⇒ stop | step 4 sets positive[2+j] for floor > 0, and step 5 ORs positive into stop for every present row of that kind. |
| no input-derived force/moment row (coverage_facts) | recover::layout, as above. It is also necessary: RV77's third test shows the p512 identity breaks if such rows existed. |
| resolution/floor entry for the body | native performed the same lookups for every body before the vector was assigned. |

## 5. Executed check

`rv77_enum.rs` (reviewer-owned, included only in RV77's copy) transcribes the native
function at row level and runs the real `rederive_coverage`:

- genuine domain: 16×16 displacement/rotation row-type subsets × 4×4 force/moment
  subsets × 3 extents (0, 2, 5e−324) × 4 E patterns × 6 (p, floor) modes × has_data
  = 589,824 cases; each must not be refused, must equal the native model bit for bit,
  and every single estimate/charge flip must differ;
- full payload domain: every stop pattern × has_data × 16 presence patterns × 3
  extents × 4 E patterns × 12 (p, floor) pairs, including invalid ones, against
  RV77's own statement of the refusal contract and output formula;
- input-derived force/moment rows: counts the p512 identity breaks the coverage_facts
  refusal prevents.

Results are in ENUMERATION.txt.
