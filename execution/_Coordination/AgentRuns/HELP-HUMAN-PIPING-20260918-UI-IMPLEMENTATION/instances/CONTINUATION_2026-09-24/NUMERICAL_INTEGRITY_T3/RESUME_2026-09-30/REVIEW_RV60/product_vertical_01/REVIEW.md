# RV60 — frozen product vertical implementation review

**BLOCKED for bounded implementation fan-in: two P2 validation findings.** The actual W0/W1 numerical results are honest complete private refusals and are independently reproduced. These findings concern the promised final decoder/metadata boundary; they do not invalidate the inspected original specimens' arithmetic or authorize changing their ordinary output.

Candidate CODE `52842022cc49d0c9dd8d0760e727963d29015842`, base `0fdd06a73389908fba87dfb0baef46dd443c4b8c`, maintained numerical basis `6ba653451f9fd27cdb852b7974753a3921f1c483`. P=projects/chirality-piping; PP=P/core/product_physics; FK=P/core/solver/frame_kernel; R is this packet's RESUME_2026-09-30 parent. All locations below use the frozen candidate unless a different basis is named.

## Findings

### RV60-F1 — enforce the closed final evidence shape (P2, blocking)

**Location:** PP/src/retained_product.rs:1010–1017, with the nested coverage and attribution reads at 1018–1037.

`observables` reads `preview_cases` but never requires the complete root key set or even the presence/type/emptiness of `combination_gates`. It also reads selected fields from each case, coverage record and support-attribution record without enforcing their closed key sets. A final snapshot can therefore pass the new observable gate after removing `combination_gates`, replacing it with a nonempty foreign object array, or adding unregistered case/coverage/attribution keys.

The selected I45 interface RETURN:29 requires final extrema/evidence with the fixed decoder; its RETURN:62 binds the final envelope after qualification/ancillary changes. The existing decoder at P/core/reporting/result_export/src/preview_physics_evidence.rs:257–263 requires exactly `preview_cases` and `combination_gates`; :356–357 requires CASE_KEYS; :409–420 and :485–487 require exact coverage/attribution keys. The first implementation's explicit no-combination scope makes an empty gates array the required admitted value, not an optional ignored field. This is a mismatch in the current private final-evidence check, not a request to activate readers or expand its domain.

**Independent evidence:** `_run_records/control_pp2.log` records `RV60_MISSING_COMBINATION_GATES_ACCEPTED true`, `RV60_FOREIGN_COMBINATION_GATES_ACCEPTED true`, and `RV60_EXTRA_CASE_KEY_ACCEPTED true`; the same record and `control_pp.log` record accepted extra coverage/attribution keys. These mutations keep all original mechanical rows and values unchanged. The neighboring extra-extrema-key control correctly refuses, so the gap is outside the already closed 13-field extrema record.

**Impact and repair:** The complete private gate can accept evidence that its selected fixed decoder rejects, including evidence from the excluded combinations surface. Check the exact root/case/coverage/attribution shapes and require the present empty combination-gates array before extracting the admitted fields. Preserve the current 13-field and numeric checks. Add discriminating refusal assertions for these exact mutations; rerun the original actual specimens and this review's controls on the repaired source. Do not change schemas, predicates or ordinary output.

### RV60-F2 — validate the solver-mode row's fixed sign convention (P2, blocking)

**Location:** PP/src/retained_product.rs:1232–1241.

The NonQuantity arm verifies the solver-mode row's kind, entity, component, coordinate system, case location and value, but omits its fixed `sign_convention`. Changing that field to `mode_code 1=dense_scrutiny` still binds as a valid sparse mode-code-1 row. NonQuantity correctly excludes it from numerical recipes, but must not bypass the promised final metadata association.

The selected I45 RETURN:45 requires full identities/metadata in the borrowed final projection and :64 preserves all emitted signs and metadata; :95 explicitly calls for sign/final-mutation controls. The actual producer at PP/src/lib.rs:5524–5537 defines value 1 as sparse interactive and emits the fixed string `mode_code 1=sparse_interactive, 2=dense_scrutiny, 3=dense_fallback_after_sparse_failure`. This finding concerns that fixed string, not reconstruction or parsing of the separate dynamic diagnostic `basis` text.

**Independent evidence:** `control_pp.log` shows altered mode metadata accepted. The isolated `rv60_g5a_join_mutation_controls` in `_run_records/rv60_pp_controls2.rs` changes only this sign after constructing an explicitly synthetic all-positive-zero snapshot. It passes `bind_rows`, `certify_product_case`, G5a and observables and records `RV60_SYNTHETIC_FULL_GATES_ACCEPT_WRONG_MODE_SIGN true` in `control_pp2.log`. The positive-zero snapshot is solely a discriminator that removes the known earlier W0 G5a refusal; it is not actual ordinary output or product availability evidence. The actual W0 negative zeros remain untouched.

**Impact and repair:** A complete final snapshot can carry a contradictory meaning for its required mode-code ancillary row while every gate component passes. Require the unchanged producer sign string in this closed NonQuantity arm and add the isolated negative control. Do not relabel the ancillary row as mechanical or adjust any numerical value to obtain a pass.

## Independent numerical and custody result

I read the complete ten-path maintained diff and all four new files, then traced the actual parse/normalization/resolver/build/load preparation/ordinary solve/render/final-hook path and its native join. The actual observer is optional, owns its records per invocation, and is absent from public ordinary callers; there is no proof-registry allocation on their None path or global/TLS recorder. The final hook follows replacement/qualification/ancillary append/finite checks; no applicable ordinary wrapper mutates rows afterward. The borrowing FK result is dropped before the unchanged ordinary envelope moves outward. Retained side reports are snapshot records, not authority over later mutated output.

Actual material selection captures successful resolver ordinals/pointers and source fields without a resolver replay. The one-case source adapter maps normalized node/model member/support identities and individual prepared primitive occurrences into canonical source order. A fresh external control reverses authored load order and verifies original/prepared/canonical occurrence custody; another exercises original model materials instead of request override. Both pass. Actual selected-point/interpolation capture tests remain resolver-capture evidence, not generalized mechanical availability.

The native join checks the selected registry Arc, run, source encoding and finish slots; the checked source view supplies source/member pointer identity, source/ledger bytes, P=2p state, publication metadata/radii, data/block maps and factor owner. Fresh distinct-invocation and invalid-run controls reject. Existing independently rerun bridge controls reject foreign source, missing/extra/invalid/excessive radius, wrong precision and missing warrant. No endpoint/radius/free-arithmetic authority is exported.

Ordinary direct/magnitude rows use the explicit completed native/source hull. Stress and circular-maximum rows finish represented and source recipes separately before hulling outputs. Represented bending includes both actual Z_hat and I_K/(D/2); source section quantities use exact normalized geometric/material primitives. Signed i-end, j-end and quarter/mid/three-quarter identities are preserved. InputDerived is exact and receives no invented radius. Every native and derivative row plus the solver-mode row is covered; failed rows are retained as failures.

The fresh `_run_records/rv60_exact.py` imports only Python standard library. It independently forms exact binary64 rational inputs, Machin-series pi bounds, integer-square-root rational intervals, closed cantilever mechanics, both represented bending meanings, final raw/SI conversions, all row scales/classes/absolute bounds, and all required predicate decisions. It uses actual freshly captured request, normalized operands and emitted rows rather than producer arithmetic or another agent's oracle. It also reconstructs complete G5a shape/bounds/zero/sanity/lower checks, including original-operand coupling of the resolution scales.

| Actual ordinary case | Mechanical rows | Numeric result | Complete G5a | Overall |
|---|---:|---|---|---|
| W0 | 73, plus one NonQuantity row | 73 pass | Refuses row 35: negative zero, associated E=+0 | Refused |
| W1 | 73, plus one NonQuantity row | 45 pass, 28 fail | Pass | Refused |

Every W1 numeric refusal is independently a source/output miss, not solely interval width. The first is row 2, `result:disp:node-N-DEC092-TIP`: SharperExact and SharperBinary64 fail. Its source-error lower bound is about 9.98266 times the exact sharper allowance. All four per-row predicate decisions agree in fresh debug and optimized runs; the full independent result objects agree. The original observed/no-observer serialized envelopes are byte-identical in each build.

The newly evaluated operational helper follows the literal length/normalize/repeated-norm and separately rounded EA/L and GJ/L sequence: 23 entered scalar operations and 39 explicit checks for each actual specimen. It is not historical section_terms capture or stiffness replay. Individual coefficient intermediate range checks and failures are retained. Actual G5a scalar prefixes are 9 for W0's first zero-rule refusal and 35 for W1 success. W1 lower bounds are 295.6593001841618 and 1.3809523809523807; the independently coupled upper bound is 976.3093505533737 for both kinds. Missing/foreign operational operands and lost accounting refuse in fresh controls.

Native source-residual/view/cast/factor work and new final arithmetic remain distinct from the invocation's 20B/60B limits. Producing scalar/helper/exact-sum numeric failures survive collection loss; non-exact status prevents success. Adapter counts are named entered events and capacity observations, with stated auxiliary library/allocator/formatting/serde costs unqualified. They are not an adopted total cost, peak memory or caller profile. No contrary resource claim is made by this review.

## Scope and verification evidence

`SEAL_AND_SCOPE_CHECK.json` independently verifies all 98 I45 sealed payload hashes and exactly ten maintained paths. Supplied RETURN, FINAL_SOURCE_INVENTORY and SEAL hashes match. Maintained core between 6ba6534 and base 0fdd06a is unchanged. The actual selected I45/RV59 original/G5a correction/brief pins are checked in `AUTHORITY_PINS.json`. Actual consulted instruction/source origins and SHA-256 values are retained in `ORIGINS.json`; no unrelated role/workflow was loaded.

The six existing-file hunks are observer plumbing, narrow fixed exports/registration, and S11 registration/body retargeting. Scanner logic, assertions, criteria, old oracle/test bodies, dependencies and feature definitions are unchanged. PP S11 lookups now inspect the actual observed producer bodies, with compatibility wrappers forwarding None. `SOURCE_BOUND_COMMAND_AUDIT.json` validates repaired PP records against the final ten-file inventory and correctly limits historical FK/ordinary records to their unchanged relevant sources.

Fresh runtime checks: candidate PP 7/7 debug and 7/7 optimized; candidate FK certificate/final controls 15/15; candidate existing bridge controls 12/12; FK S11 3/3; independent appended FK controls 2/2; independent appended PP controls 4/4 (the two accepted gaps are explicitly recorded diagnostics). The new exact verifier covers all 73 mechanical rows per case in both builds, with zero disagreeing predicates and matching complete G5a outcomes/prefixes. Preserved controls and exact replay instructions are in `_run_records/BACKCHECK_CONTROLS.md`. There were no failing compilation/test commands in this review; unsuccessful file-location reads and the corrected empty-directory setup mistake are disclosed in execution records.

## Limits and return

The `CapturedInvocation` guard checks mode, not arbitrary request/capture pair equality. An intentionally mismatched internal pair is accepted by that guard. The actual sole `observed(raw)` harness parses and forwards its matching pair correctly, and this review traced that callsite; no public receipt/raw-identity token is exposed. This remains an explicit internal-caller and future C2 boundary, not a current failed original-specimen warrant or a required redesign. Dynamic solver `basis` text is also not independently parsed by this review; F2 is only the fixed sign convention.

No value/sign/predicate edit is authorized for a real pass. No production W1 projection/routing/rollback, C2/readers/receipts, p512 product availability, point/interpolation mechanical availability, combinations, pressure/SIF/directional/nonlinear families, tariff/permit, full memory/work profile, protected availability, practitioner/native acceptance, integration, merge or release follows. ROOT owns the narrow repairs, unchanged same-reviewer backcheck, integration and acceptance. Candidate source and prior sealed records remain unchanged.
