# RV70 same-reviewer backcheck — P02-1 closed

**Close P2 RV70-P02-1 at I53 `first_party_profile_correction_03/ADDENDUM.md` hash 1234487f0e, seal 2ede21ecb9, NUM d85300e1d7.** All three correction payload hashes match, and the addendum bytes match the stated Git revision. Both earlier I53 seals and the first RV70 seal remain unchanged. This backcheck is within the original 25-minute RV70 continuation window; no extension was requested.

ROOT supplied the correction location and requested confirmation through a collaboration message. The same reviewer read the complete addendum and manifest, compared each change against the existing source/accepted I30 clauses, and verified the seal independently. The original finding and corrected interface remain in RETURN.md.

| Required correction | Result |
|---|---|
| Preserve the job result after body transfer | U_J explicitly carries producer/completion and resident R_J until actual last-owner Drop. Worker completion, final poll, registration, JS completion, cancellation request and UI detachment do not retire it. This matches native publication/poll behavior and I30's independent resident lease. |
| Separate caller window from resident lifetime | W_call,J is only the included caller-profile domain. Each reply keeps its own transfer/Drop boundary. Outside it the caller overlay makes no wider promise and is not asserted zero; U_J remains governed by its own ownership. Detached reply copies survive independently if the table drops. |
| Avoid second R and input/response double counting | A_call,J contains B_call, O_call, C, L and D, with no R. Composition is U+A at actual overlap. C owns original request/context allocations; O only otherwise uncharged ordinary response work; L/D only W1 response allocations. Repartitioning I30's old D removes duplicated request/id/header costs while preserving separately allocated response IDs. |
| Preserve per-invocation scope | Shared backing is attributable and charged once; unrelated jobs, transferred output, TS and opaque native heaps remain outside. Resident lifetime does not bring all later framework work into the caller claim. |
| Do not turn logical start into one native producer | The addendum explicitly binds reservations to each actual admitted invocation/job, recognizing possible fetch-to-postMessage resubmission without asserting observed duplicate execution or requiring deduplication. |
| Preserve open qualification cells | The finite included native-context/backing premise, per-phase capacities, serializer/emergency and build/layout obligations remain open. No H, δ, K, allowance, finite-use restriction, raw-IPC expansion, mandatory cleanup, availability policy or native activation is selected. |

The original s/p/c inequalities, canonical 228/325-byte envelope bounds and conditional formulas remain valid with the corrected partition. They are logical/source facts and conditional upper expressions, not exact allocated-byte totals or native population measurements. In I51, ownership moves must update which term carries an allocation; source lengths alone do not determine String/Value/header capacities.

**Disposition:** no unresolved review finding. Use profile 02 together with correction 03 as a conditional source/interface input to I51; do not describe it as complete native resource qualification. The exact included context lifetime/backing warrant remains the external prerequisite identified in RETURN.md. No runtime, source change, install, tooling or vendor investigation was needed for this correction.
