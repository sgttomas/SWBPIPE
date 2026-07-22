# N3E Return — Candidate v5 independent refutation

**Verdict:** `BLOCK`  
**Owner Gate-3 readiness:** `NOT READY`

The CAEPIPE correction passes. The frozen panel has exactly the Harness JSON
and Parser CSV sinks; `DOTH-CAEPIPE-LOCAL-006` is exclusively `local_private`,
absent from `DOTH-FORMAT-003`, and consistently carries wrapper-owned intent
plus whole-member/no-`href` gating. The accepted contract and parity behavior
remain intact.

One current-attribution defect remains. The accepted DEC-076 bounded
re-extraction changes `SURF-011` from historical `NONE_FOUND` to `DEL-10-04` in
`R7_DEC076_PDU077/IMPLEMENTATION_SURFACES.csv`. The owner map still calls the
build-readiness surface unattributed. Correct current attribution is
`PKG-10 / DEL-10-04` with no state/handoff effect. Candidate §6.1 must also
change its count from two to one `NONE_FOUND` surface.

Additionally, Amendment v5's claim that every RouteID appears exactly once in
the owner map is false because the map intentionally records multiple exact
route/path associations. The verified statement is: every RouteID has exactly
one verification disposition and at least one owner-map path; every intended
route/path association is explicit and non-duplicated.

Authority, parity, preservation, N4→N5→W3 sequencing, and closeout gates
otherwise survive. Fresh refutation is required after correction.

