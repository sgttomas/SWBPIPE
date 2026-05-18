---
amendment_id: SCA-001
doc_kind: scope_change.propagation_plan
created: 2026-04-30
status: executed
---

# Propagation Plan

## Executed Steps

1. Amend `SOFTWARE_DECOMP.md` from revision `0.3` to `0.4`.
2. Add SCA-001 architecture-basis register `AB-00-01` through `AB-00-08`.
3. Add decision log entries `DEC-008` through `DEC-012`.
4. Update scope/register notes for resolved and remaining-TBD architecture choices.
5. Inject SCA-001 architecture basis into all 65 downstream `_CONTEXT.md` files.
6. Update mutable coordination state to point at revision `0.4`.
7. Preserve lifecycle state and production four-document kits.

## Package-Level Context Injection Map

|Package|Context Count|Applicable Basis IDs|
|---|---:|---|
|PKG-01|4|AB-00-01, AB-00-02, AB-00-06, AB-00-08|
|PKG-02|5|AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08|
|PKG-03|8|AB-00-01, AB-00-02, AB-00-04, AB-00-06, AB-00-07, AB-00-08|
|PKG-04|6|AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08|
|PKG-05|5|AB-00-01, AB-00-02, AB-00-03, AB-00-06, AB-00-08|
|PKG-06|5|AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08|
|PKG-07|7|AB-00-01, AB-00-02, AB-00-03, AB-00-05, AB-00-06, AB-00-07, AB-00-08|
|PKG-08|5|AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08|
|PKG-09|5|AB-00-01, AB-00-02, AB-00-06, AB-00-08|
|PKG-10|5|AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08|
|PKG-11|5|AB-00-01, AB-00-02, AB-00-06, AB-00-07, AB-00-08|
|PKG-12|5|AB-00-01, AB-00-02, AB-00-03, AB-00-04, AB-00-06, AB-00-07, AB-00-08|

## Follow-On

Use `plans/SCA-001_DOWNSTREAM_FOUR_DOC_REFRESH_PLAN.md` for any downstream four-document refresh workflow.
