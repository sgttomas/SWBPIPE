# WORKING_ITEMS Run — D-41 R5 T3 / PDU-016

- Date: 2026-07-12
- Role: owning privacy/redaction pilot
- Ruling: DEC-074 O7 before E5; evidence requirement only

Changed the CAEPIPE external-run evidence default from invented-public classification to `private_user_controlled` while preserving local-only and telemetry-disabled controls. Added a partial-override no-bypass test. Existing explicit unsafe embedded-payload flags remain blocking.

Focused T3 suite passed 18 tests. This covers only the DEL-17-05 user-output seam; DEL-12-01/DEL-12-03/DEL-13-01 consumer/runtime reach is not claimed. Lifecycle remains `IN_PROGRESS`.

## Fan-in correction

Independent T3 fan-in identified that the initial merge-based normalization still allowed callers to override `classification`, `local_only`, or `telemetry_allowed`. The owning pilot corrected the seam: those three O7-controlled defaults are now preserved, every conflicting override emits blocking `CAEPIPE-RUN-PRIVACY-DEFAULT-OVERRIDE-BLOCKED`, and no public-rights override path was invented. Focused schema-valid negative coverage for all three fields passed: `10 passed` in `tests/test_caepipe_external_run_package.py`; `git diff --check` passed.
