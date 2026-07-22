# N5B Return — Fresh attempt-2 verification

**Verdict:** `BLOCK`  
**Tool policy:** `PASS`  
**ApplyEdits:** false

All five original N5 findings are closed. All 31 routes reconcile; candidate
§6 containment, protected/state gates, recorded checks, and both immutable
sweep dispositions validate.

One new material safety defect remains. Both route projectors derive a
payload-wide public basis from root privacy/provenance metadata, then promote
every otherwise-unmetadataed sibling leaf to
`public_metadata/public_permissive`:

- Python: `core/security/redaction/route_control.py` around lines 234–295;
- TypeScript:
  `apps/desktop/src/features/redaction-controls/redactionExportControls.ts`
  around lines 712–850.

A public top-level provenance record plus an unmetadataed `opaque_leaf` can
therefore expose that leaf as public. This violates adopted candidate §3:
metadata is leaf-explicit, containers carry structure only, and missing
metadata remains unknown.

Required remediation: remove or leaf-scope the payload-wide public basis in
both languages and add Python/TypeScript regressions containing public
envelope provenance with an unmetadataed sibling. W3 remains held pending a
new bounded attempt and fresh verification.

No edits, sweep, state, receipt, or Git effect was performed by N5B.

