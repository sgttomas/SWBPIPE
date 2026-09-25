# Original eleven-path findings

## OPA-01 — P2: retain the authoritative finite denominator on malformed submission

`tools/validation/qualification_gate.py:166–211` predeclares/summarizes only submitted cases/assertions. The fixed first-static inventory is enforced later at452–465 and in prepare_case, so a missing case is blocked but disappears from the ledger's “required” denominator. The pure probe gives1case/73scalar/10structural when the selected second case is omitted, and2cases/145scalar/20structural when one scalar assertion is omitted. Neither passes; the defect is lost mandatory coverage in refusal evidence, not a demonstrated false solver pass.

For the ordinary finite profile, predeclare the checked two-case/73-ID-per-case inventory, retain submitted inventory separately and associate by explicit case/assertion identity. Missing, duplicate or extra submissions must not remove authoritative obligations or make zip position define identity. Keep the legacy caller-selected inventory contract unchanged. Preserve the nested nine section subchecks as unavailable/error details when applicable rather than inferring successful observations. Add negative controls asserting146scalar/20structural per mode after missing-case, missing-assertion and malformed-checker refusal.

## OPA-02 — P2: reject contradictory new-method metadata on legacy dispatch

`tools/validation/qualification_gate.py:267–268` checks only schema_version0.1 for the legacy branch. An otherwise valid legacy wrapper is accepted when it additionally carries source_block_recovery:null, carrier_evidence:null, or a producer claiming physics-source-1. The ordinary branch is correctly closed, but the other branch silently treats these contradictory carriers as old raw0.1. This defeats the intended distinct dispatch/namespace interpretation.

Reject the presence of recognized newer-method identity/evidence fields on the legacy route, including producer, numerical_quality, formulation_basis and the physical/recovery/foreign evidence namespaces, rather than inspecting their truthiness or ignoring them. Preserve a genuine header-free raw0.1 positive control; retain old sparse mode and comparison semantics. Add contradictory-presence controls across both dispatch boundaries. This is a synthetic parser counterexample, not a genuine newer-method solve or a demonstrated completed-case pass.

`FOCUSED_BOUNDARIES.json` and `focused_boundaries.py` retain exact observed pure-function results and the original gate hash. No solver, helper subprocess, pytest or real case was executed. Both findings were promptly reported to Root and the implementation manager, who accepted coherent repair while keeping this original source freeze intact. No third actionable finding was found in the complete assigned diff.
