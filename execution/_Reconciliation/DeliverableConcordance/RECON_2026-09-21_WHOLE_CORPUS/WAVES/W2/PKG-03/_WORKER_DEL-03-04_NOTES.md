# Worker carry-forward notes — W2 PKG-03 group G2 (DEL-03-04, DEL-03-05, DEL-03-06)

Running record of recurring-situation judgments, so the three ledgers treat the
same situation the same way. Agent judgments only; not owner rulings.

1. **Product-path slice missing from the SOW.** All three SOWs (migrated
   2026-07-14) describe only the component schema, fixture and schema test,
   although the 2026-06-21/22 DEC-045 app-absorption tranches (D2 branch,
   D3 rigid, D4 expansion joint) landed product code for each deliverable.
   Recorded once on the SOW SURFACE row as `STALE_REVIEW_OR_EVIDENCE ·
   DOC_BEHIND_CODE · LOCAL_DESIGN · RECORD · NO` with a FindingGroup, except
   DEL-03-06 where the SURFACE row carries CP-04 (rename residue) and the
   product-path lag is carried by the item rows. Item rows are stale only when
   their text *limits* evidence to the schema slice ("only", "limited to",
   "bounded to", "excludes product implementation code") or lists as TBD
   something the code now settles. Neutral evidence lists stay `ALIGNED`.
2. **Scope blocks** that limit the specification to schema evidence →
   `STALE_REVIEW_OR_EVIDENCE · DOC_BEHIND_CODE` (origin 1b62eb5b8 in all
   three).
3. **CP-01 four-document residue.** Origin by `git log -S` on the deliverable
   folder: 7bee9ae41 text → `STALE_SETUP_SPECIFICATION`; 1b62eb5b8 text →
   `STALE_REVIEW_OR_EVIDENCE`.
4. **Revision 0.7 pins in References** → CP-02. PDU-055 declarations pinning
   0.8/DAG-007 → CP-02 fields, CanonicalSituation CP-02 (CP-03 directs CP-02).
5. **Parity (CP-09).** DEL-03-04 and DEL-03-06: no PASS record matches the
   frozen SOW → matrix OUT-001 and VER-001 both `STALE_REVIEW_OR_EVIDENCE ·
   EVIDENCE_OVERTAKEN`. DEL-03-05: matching PASS → both `ALIGNED`.
6. **Protected-content review named by a claim but not located** → `UNKNOWN ·
   EVIDENCE_NOT_LOCATED · INVARIANT · IP_DATA;RECORD · REVIEW`, following the
   W1 resolution for DEL-07-02 CLM-026. The keyword denylist test is not a
   review.
7. **No-bypass requirements.** Where the basis names OPS-K-IP-3 (DEL-03-04
   RQ-006): `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · INVARIANT · IP_DATA`.
   Where the basis is AB-00-02/AB-00-07 only (DEL-03-05 R07, DEL-03-06
   R-008): `PARTIALLY_IMPLEMENTED · PARTIAL_SLICE · PROJECT_BASELINE ·
   BASELINE`.
8. **Invented public example values** (preview components C-120, C-130,
   C-150) are permitted by DEC-045 ("public examples remain schema-shape-only
   or invented"). A setup-era condition forbidding invented weights/COGs
   (DEL-03-05 CLM-004) → `STALE_SETUP_SPECIFICATION ·
   SCOPE_REDIRECTED_BY_RULING · LOCAL_DESIGN` (CS-05-style catch-up; IP
   boundary named in Notes under F8).
9. **CONTEXT surface.** Same treatment in all three: SURFACE stale
   (BASIS_POINTER_STALE); `.s01` PKG-00 SEMANTIC_READY statement and `.s02`
   Still-TBD list as sub-claims, both `STALE_SETUP_SPECIFICATION` (7bee9ae41
   origin). Context Envelope: NON_NORMATIVE when the note is pure sizing;
   DECLARED_STATE when it states a constraint ("no data tables").
10. **D-41 declarations (CP-03)** that are true as written → `ALIGNED`;
    Remaining holds point `OPEN_ACTION` at the governing non-aligned row.
