# I34 return — checked/sticky work exactness design

**Bounded question answered; stop for fresh independent source/design review.**
The explicit-state alternative is coherent at specification level. It can replace
the missing global cumulative no-wrap assumption for exact receipts only after
complete implementation/source correspondence, review, qualification and C1's
explicit adoption of that alternative. This packet does not lift today's blocker.
No API, code change, byte policy, numerical criterion or product release is selected.

DESIGN.md gives E/O/I/OI transitions, exact-zero versus loss semantics, raw through
weighted/stage/aggregate/delta propagation, clear/reset/clone/error custody,
cache and cross-case handling, terminal records and C1 projection. State is kept
per independently movable owner, with a small enclosing status for unpriced locals;
existing prices and physical/logical ownership are unchanged. A MAX sentinel alone
cannot retain history through subtraction or terminal omission.

The separate raw-safety construction uses exact cumulative term_limbs to bound
completed nonzero raw additions. A prospective base-charge check before any
anchor/magnitude mutation, with that charge reserved during carry checks, keeps
T+1<2^64 and hence the 8128+64-bit headroom invariant. It does not charge a
conservative 256 allowance or introduce another work model. Explicit carry/index
containment and poisoned-value/full-reset discipline remain required. This is a
source/design lemma needing independent checking, not an implemented guard.

Upfront raw count/index/encoding and scalar predicates remain necessary, including
6n, preparation prefixes/profile bounds, residual m/64m, factor multipliers,
tracker sequencing/capacities and ceil_sqrt r*r. Status cannot repair a mechanics
input that already wrapped. No new bound on unguarded runtime or full memory is
claimed.

SOURCE_MANIFEST.md enumerates the affected source owners, caches, scopes, local
drop/prune edges, public/legacy consumers, targeted tests and K6c/H/VR/F2a memory
consequences. It includes the separate K3a WorkCounter and legacy ExactAccumulator
boundaries so they are not silently repriced. Added logical status need not be
serialized into legacy exact observations; existing clean numerical values,
classes, work and output views must remain bit-identical. New concrete layouts
and size diagnostics cannot be qualified by old profiles or by an assertion that
padding will absorb fields. Literal identity of changed size reports is not
promised; implementation must choose the compatibility representation explicitly.

Finite standard-library exact controls: **15 passed**, including 19,683 clean width
price vectors, 1,024 reduced-capacity add/delta pairs, 64 state-join triples and 56
reduced headroom cases. Named controls cover overflow, pending carry capacity,
MAX−MAX, inconsistent/foreign delta, partial error/stopped closure, cached success
and non-budget failure, zero-coefficient reuse, cross-case sticky state, missing
legacy terminal vector, clean C1 partition and safe-JSON provenance/range. These
are abstract controls only; no Rust/compiler/solver/model/runtime/host witness.
The first script invocation failed on an unsupported evaluated type annotation;
adding postponed annotation evaluation corrected the script, and the next run
passed. Both the initial failure and final result are recorded in SESSION.json.

Unresolved finite choices: Rust state placement/accessors and terminal accounting
reason; ownership-constrained snapshots; legacy/recorded wrapper; exact partial
mutation/error mechanics; diagnostic mapping (unknown/O/I are not automatically
saturated lower bounds); and new layout/profile facts. C1 §2 and W09 need a bounded
reconciliation when the alternative is adopted. Normal independent review and
actual-candidate qualification precede maintained implementation/reliance.

Receipt 2026-10-02 20:52:37 UTC; new-analysis cutoff 21:27:37; deadline 21:37:37.
Actual completion time is in SESSION.json. Native delegated TASK
`/root/i34_f2a_work_exactness` under ROOT `/root`; no descendants. Only
R/I34/f2a_work_exactness_design_01 was written. Origins, checks and informational
inventory are supplied; no acceptance seal, Git/index/API write, maintained edit
or broader authority claim follows.
