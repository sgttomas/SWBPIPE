# RU return

Status: DIAGNOSIS_CONFIRMED

The two-project browser failure is caused by stale test sequencing. Canvas placement deliberately leaves node provenance blank; the submission validator requires nonempty provenance; Add therefore remains disabled exactly as the product contract requires. The repeated disabled DOM state is fully explained by that missing input and is not a timing failure.

The active browser scan found the same obsolete assumption in r2-smoke.spec.ts's fillNodeDraft helper. The sealed candidate corrects both consumers in the same file, preserves and strengthens the readiness assertion, and uses provenance already supplied by the rehearsal fixture.

Candidate binding:

- preimage: 3f500eb5e061ab0581123d8cc28d581fa4a17b654afc247d624f60af4dfe6f17
- postimage: 5a8af00f21a2618c6429d73737926d9371954ab7ebe69ec513f88ef493319287
- patch: 48295ed5a031fe2405e8189acb1a627d9d191538b6de85c2a20d84c0708c1445

The candidate is unapplied. The current owner act authorizes the prior numeric-only postimage, so this interaction-test correction requires an amended bounded authorization and independent review before application. No live source, test, fixture, Git state, build, browser, or test run was changed by RU.
