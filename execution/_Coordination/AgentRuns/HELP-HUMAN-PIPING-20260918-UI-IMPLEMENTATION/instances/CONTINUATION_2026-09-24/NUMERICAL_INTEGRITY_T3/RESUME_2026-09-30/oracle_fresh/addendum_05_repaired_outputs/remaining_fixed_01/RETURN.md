# Remaining fixed repaired outputs — independent claim check

**18 selected outputs pass; one source-bound output is nonselected.** All 19
released raw SHA256 values match ROOT's exact release. The unchanged sealed
bare-b checker verifies complete source/stiffness/selected-source identities,
corrected policy, row/layout sets, canonical finite encodings and floor shape.
No numerical claim failure or invalid binding was found.

| Cases | Outcome | Rows checked |
|---|---|---:|
| B03, B04, B09, B10, EXTRA-FM-01, EXTRA-MF-01 | Selected 128 / verified 256; pass | 228 |
| B01, B02, B05, B06, B07, B08, B11–B16 | Selected 512 / verified 1024; pass | 444 |
| EXTRA-ZR-01 | Unresolved(Ceiling); exit 3; no accuracy pass | 0 |

The 672 checked rows comprise 198 exact InputDerived, 471 AbsoluteVerified and
3 Unpublishable rows. All 669 value-bearing rows satisfy their accepted claims;
the worst absolute error/b is 1/12 at B01 D:9. All direct-rounded bits match
frozen exact truth. There are no RelativeVerified rows in these 19 outputs, so
no relative guarantee is exercised by this particular return.

The exact-consistent Underflow rows are B15 D:9, B16 D:6 and B16 M:1. No overflow
outcome occurs. All 12 selected-p512 records carry both required finite,
nonnegative force/moment floors and pass interface checks. This validates floor
encoding/presence, not the numerical construction of the floors.

EXTRA-ZR-01 has matching primitive/stiffness source identity but no publication;
`numeric_accuracy_pass` is null. Its Unresolved(Ceiling) result is not counted as
a pass, a false publication, or proof of source reachability exclusion.

Authority: ROOT's REPAIRED_FIXED_OUTPUT_RELEASE.json at NUM feef1005ca, preserved
verbatim as RELEASE.json. RUNS.json carries its full commit/path/hash and every
actual `<VENV>/bin/python -B` argv, exit, stdout/stderr and raw hash. Each case
subdirectory retains unchanged raw TSV, full comparison JSON and checker output.
SUMMARY.json records per-case precision, row/class counts, floors, range outcomes
and failures. All reported SOURCE_COMMIT fields match the release's dd1f70d8
label; that consistency is not proof of binary execution or source provenance.

Actual order: 3dbc4f read the pinned release; 428d8c rechecked the comparator
seal and sequentially rehashed/compared exactly the 19 files; e32f28 summarized
the new results with exact Fraction arithmetic. C17 was not reread for numerical
comparison or repeated. Its existing seal was checked for preservation only.
No other case, solver run, Rust command, source edit, expectation change,
Git/index mutation, private-radius or certificate-H inspection occurred.

All prior seals remain unchanged. This return covers raw output claims only;
binary/source/probe integrity, bound/floor construction, protected availability,
complete A1 repair acceptance and product/native/F2a qualification remain separate.
