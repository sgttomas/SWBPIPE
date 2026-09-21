# Focused diagnosis

01-tsc reproduces TS2367 in ModelTree disclosure test ID. A repeated text substitution duplicated the Sections arm, making its second comparison statically unreachable. Removed only the duplicate condition; no criterion changed. Rerun TypeScript plus affected table tests below will check correction.

Rejection clarification read before EngineeringTable changes; existing result.rejected behavior retained exactly.

02-tsc: new fixture changes an optional OD property without asserting the fixture's declared presence. TS2532 reproduced at line 62; adding only the fixture nonnull assertion keeps runtime/oracle intact. 03-focused passes all 79 tests in four files.

04-raw-enum-indicator reproduces a missing retained-draft disclosure (expected 1, got none). changedGridCells deliberately excludes unsupported enums for Queue, so it cannot count all retained raw review text. Fix uses a Section-only raw draft count; Queue eligibility and Node/Materials counts are unchanged.

07-app-section failed because JSDOM returns zero geometry and the common persistent editor correctly remains visibility:hidden until positioned. Added test-local restored DOMRect allocation, matching the existing MaterialTable test harness; did not alter production visibility, accessible query, or sole supported option oracle.

11-cargo new test at checkpoint 54176: wrong-order geometry rejection and rollback passed; right-order succeeded; assertion then differed only serde_json Number(60.0) vs Number(60). Exact expected JSON literals corrected to 60.0/20.0 under manager instruction. No value, tolerance, geometry, diagnostic or publication assertion weakened; not a product defect.
