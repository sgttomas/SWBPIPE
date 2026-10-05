# RS-F1 clarification — G4 does not inspect raw-row method tokens

Prospective supplement to reader_contract_seams_06 and its sealed RS-F1
correction reader_contract_seams_correction_07. Both sealed packets remain
unchanged. This clarification is part of the same bounded correction.

**Exact gate allocation:**

- G1 enforces 07's closed raw-row shape and optional direct string. Null,
  nonstring, unknown alternate row containers and unknown metadata keys fail
  G1. A prohibited method field on a case object is likewise a G1 closed-shape
  error; this proposal does not add it to an unavailable/not-required case.
- G2/G3 retain 07's earlier encoding and case/row coverage checks.
- G4 checks diagnostic exclusivity, codes and affected_refs against the actual
  case/combination dispositions. C3's older phrase “unavailable cannot carry
  selected method” does not authorize an earlier raw-row token check here.
- **Raw-row recovery_method presence, string value and selected-owner scope
  are checked at G6**, after all prior gates. G4 must not scan raw rows for
  that field or reject a well-shaped string on an unavailable/ordinary row.
  With all earlier gates valid, that string fails at G6 as 07 specifies.
- G7 removes only the direct member from rows already bound by successful G6.
  It never repairs an earlier shape, encoding, coverage or diagnostic defect.

The entire first-failure matrix in 07 is unchanged. In particular, a
well-shaped wrong/ordinary-row string plus a real diagnostic exclusivity defect
fails G4 for that diagnostic defect; the same row-token defect with valid
diagnostics and other earlier gates fails G6. A prohibited case-object method
field fails G1 even if the row-token and diagnostic defects also exist. Add these
dual-defect controls to the shared corpus; do not suppress an earlier gate to
force a later expected code.

No definition bytes, numerical predicate, public standing, coverage, identity,
raw-source custody, base metadata or owner-held meaning is changed. Independent
review and ROOT selection precede dependent implementation. This is an explicit
allocation of existing checks, not a new permissive parsing path.
