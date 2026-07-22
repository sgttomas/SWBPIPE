# N3F Return — Candidate v6 independent refutation

**Verdict:** `COMMIT-SAFE`  
**Owner Gate-3 readiness:** `READY TO REQUEST OWNER DECISION`

Evidence:

- `SURF-011` correctly overlays the historical register with accepted DEC-076
  attribution `PKG-10 / DEL-10-04`; `SURF-050` is the sole remaining
  `NONE_FOUND` surface.
- The route matrix has 31 unique RouteIDs. Dispositions cover all 31 exactly
  once. The owner map has 54 valid rows and 87 unique `(RouteID, Path)`
  associations; all 31 routes have at least one explicit path.
- Every owner-map attribution equals the original concordance register plus
  the SURF-011-only R7 overlay.
- `DOTH-CAEPIPE-LOCAL-006` remains distinct, fixed `local_private`, limited to
  Harness JSON and Parser CSV, absent from `DOTH-FORMAT-003`, and consistently
  requires wrapper-owned intent plus whole-member/no-`href` gating.
- Prior cures remain intact: fixed contexts; payload-intent stripping;
  unchanged unknown-value `warning_only` behavior; parity-corpus preservation;
  exact ownership; specialized-format withholding; complete route
  verification; native-GUI limitation disclosure; and N4 → N5 → W3 closeout.

Candidate effect remains held. Gate 3 may ask the owner to adopt, reject, or
amend v6. Adoption must expressly authorize `AFFECTED_OWNER_MAP.csv`, N4 as the
sole serialized integration owner, fresh N5 verification, and DEL-12-02-only
W3 closeout after N5 `COMMIT-SAFE`. No adoption, implementation, state,
lifecycle, receipt, or Git effect is implied by this return.

