# Integrated review v8 — N2 reopening finding

Status: `BLOCKING PRODUCTION DEFECT / AMENDMENT 3 REQUIRED`

`buildReportPackageRequest` verifies `inputManifest` but compares the separately supplied model/result only by project ID. A same-ID stale or different model can drive report rendering while the manifest/hash evidence attests a different model payload.

Disposition: add a fail-closed production canonical payload identity gate before rendering/package assembly; cover same-ID/different-payload rejection, canonical object-order equivalence, and the positive current-session path; rerun focused checks; freeze the complete N2 diff from original basis; and obtain fresh governed read-only review.

No N1/N3, Git, receipt, PR, owner-ruling, schema, dependency, or policy expansion is authorized.
