# RV46 — independent checked-work design review

**CLEAR for fan-in as a bounded specification. No actionable finding in the frozen eight-file packet. It is not ready for maintained implementation or exact-receipt reliance: the explicitly unprovided placement, caller, scalar, layout and qualification work remains blocking.**

Author candidate `49d7dee436f9b595fa274084fa7b2bdab397a006`, exactly R/I34/f2a_work_exactness_design_01. Source main `49034a940f3f8cd3f3da4d4cbc839943b808063d`. No compiler, solver, numerical model, product runtime or host probe ran.

## Independent basis and scope

SOURCE_DERIVATION.md was frozen at 2026-10-02T21:16:24.870046Z, SHA256 `fd6a1c434210ea67fb92b9d92928ff6fbf68f726d4f3a0c69cc95f66b28f2937`, and notified to ROOT before any I34 design/control read. ROOT's initial message had already authorized Stage 1 at the candidate. Stage 0 independently traced the actual raw/weighted/stage/delta mutations, unpriced scratch and tracker drops, cached success/failure, terminal losses, scalar/count trust timing and carry mutation ordering; it consulted RV43-F1/corrected C1 and RV44's limited review.

The complete candidate diff is only the eight declared packet files. All seven inventory payload hashes/lengths match. Maintained P/{core,apps,schemas,fixtures,tests,validation} at the candidate is unchanged from the pinned source. Source references are immutable Git objects in origins records, not copied trees. Read-only source checks support this design assessment; they do not replace later compilation, full changed-call coverage or runtime witnesses.

## Design conclusions

| Area | Independent assessment |
|---|---|
| Exactness algebra | DESIGN:20–64 correctly separates exact E, overflow O, inconsistency I and their union. Checked raw increments, weighted multiplication, intermediate addition and owned chronological subtraction prevent prior loss from being reclassified through small totals or MAX−MAX. Exact equal snapshots can yield exact zero; foreign equal snapshots cannot. Unknown provenance remains unknown rather than an invented state. |
| Prices and closure | The existing eight L4/8/16 prices match wide/multi.rs:986–995; operation charge precedes numeric success/failure (:1153–1225). Unpriced old scratch remains unpriced. StageWork's ordinary adds and stopped saturating remainder, aggregate W+K, compound deltas and stop-rule accumulation all require checked state, as enumerated. Budget-room clamp and honest overshoot remain exact budget semantics, not accounting underflow. |
| Ownership and drops | Per-movable-owner state plus enclosing status is sufficient in principle. DESIGN:98–115 correctly requires a join before every local early return, prune, collapse or discard; generic inherited clones cannot use the constant-size escape. Tracker refusal alone is insufficient because later key pruning can erase it. A successful value copy does not absolve the producing scope. Actual Rust transfer coverage is still unprovided. |
| Cache/invocation | Both successful Arc owners and non-budget failure tuples need origin state. A reused slot charges the case fully and invocation zero new build cost while importing dependency status. Budget failures remain uncached; finish-time snapshots and first-filled combination slots preserve their actual provenance. A later tainted run cannot restore an exact aggregate invocation, even if prior numerical selections remain standing. |
| Terminal/legacy | DESIGN:201–221 handles the real losses at adaptive.rs:4179 and combine.rs:131: retain physical records and invocation state before legacy projection. A legacy Refused view may remain incomplete only as a labelled compatibility view. Existing u64-only accessors cannot qualify history. No duplicate solve/counting pass is proposed. |
| C1 | The induction is sufficient only after every required transition/owner is implemented and qualified. C1's existing upfront no-wrap obligation is still governing until explicitly reconciled. Conservation, chronology, complete source provenance and safe-JSON checks remain. The proposed narrowing of `saturated_lower_bound` is necessary: a refused prospective update does not prove actual spent work exceeded MAX, and an arbitrary lost payload is not a proved lower bound. |

### Carry lemma and mutation ordering

DESIGN:125–165 agrees with the independently derived sufficient condition. Each completed nonzero raw addition contributes b=limbs+1>=2 to exact lifetime term_limbs. Clear/reset preserve that count; clone preserves both it and the value. Before changing anchor or magnitudes, t+b<=2^64−1 implies the next live raw-term count is <2^64. Each incoming term is strictly below 2^8128 at the prospective common anchor, so either sign magnitude is strictly below 2^8192. Applying the span check at a lowered anchor preserves that inequality during shift.

For scaled donors, add_scaled forms each complete sign magnitude times the u64 factor, and add_raw computes its high bit from that complete integer. Accumulated donor carry therefore becomes part of the incoming term's measured span; it is not hidden behind the donor's original high metadata. add_product_of nets its mutable b and sends each nonzero limb through these same scaled insertions. Donor status must transfer before zero/empty shortcuts. The proposed pending-b reservation during carry charges is necessary; the current main charge occurs after writes. Shift charge checks and explicit carry/index containment must precede affected writes. Partial failure poisons the value until full reset, and accounting poison remains terminal after reset; numeric Span/Exponent with otherwise exact accounting preserves the existing block-local reset path. Mapping accounting failure into a swallowable A2 refusal would violate this design.

This is a conditional source lemma. It proves neither an unimplemented Rust ordering nor safe arbitrary fabricated raw histories. The retained explicit containment check is appropriate even after the mathematical lemma.

### Scalar, consumer and layout limits

The manifest preserves a distinct upfront raw-size/index/encoding/scalar obligation. A work flag cannot repair node×6, count prefixes, narrowed source encodings, residual 2count+2/64m, factor operations, tracker sequence/capacity arithmetic or ceil_sqrt's r*r. The stated F<=2^32−2 sufficient restriction also keeps the relevant small count-to-binary64 conversions exact; target usize expressions still need their own checked admission. No new accepted model cap or generic preprocessing proof follows.

Actual consumer consequences are present: H staged.rs currently performs saturated work closure and ordinary stage merges; H and VR hide Refused traces, and VR records serialize work without status. These consumers need qualified E gating or the recorded adapter. The current 2144-byte ExactWideSum, 4304-byte tracker/fallback entry, 40-byte table entry, nested AttemptRecord/Shared/VerifyShared/cache layouts and source40129 profiles cannot be assumed unchanged. H wrapper profiles and VR's 19-field stages/18-member attempt memory expression depend on those facts. New terminal ownership may increase lifetimes even if a flag fits existing padding. SOURCE_MANIFEST correctly requires exact-candidate layout/profile and P1–P5/caller recomposition; output equality alone proves none of those byte claims.

## Finite controls

Independent exact-integer checks reproduced the reported 15 abstract control rows' arithmetic/state expectations, 64 join triples, 1,024 add/delta boundary pairs and 19,683 price vectors. An additional 22,410 reduced headroom states cover variable prior term work, live-term counts, pending base charges and lowered-anchor bounds. C1's synthetic partition gives case 117 / invocation 90.

Only after those checks, the inspected author's standard-library script was replayed in an owned subdirectory. Exit0, stdout `15/19683/56`, and byte-identical CONTROLS.json. The author script at its own path was neither run nor modified. These controls are algebraic constructions: the cache, prune and terminal examples do not execute the production edges, and no source/model reachability, real partial-array mutation or compiled clean-output parity is claimed.

## What ROOT can carry forward

The state algebra, status-versus-price discipline, carry lemma, conditional C1 replacement and source/consumer consequence inventory are design-ready. Before implementation or reliance, freeze the concrete state/total accessors, same-lineage snapshot interface, accounting stop/precedence, recorded-to-legacy terminal seam, full-reset/partial-write mechanics and diagnostic mapping. Then authorize the complete maintained manifest and qualify every producer/consumer transition, scalar admission, near-MAX/checked/optimized behavior, unchanged clean numerical and observation views, actual layout/H/VR correspondence, whole memory/caller composition and C1's three readers. These are acknowledged unprovided artifacts, not defects concealed by this clear design disposition.

No global no-wrap proof, counter implementation, ordinary-availability proof, byte allowance, API adoption, numerical qualification, engineering acceptance or release is supplied by RV46.
