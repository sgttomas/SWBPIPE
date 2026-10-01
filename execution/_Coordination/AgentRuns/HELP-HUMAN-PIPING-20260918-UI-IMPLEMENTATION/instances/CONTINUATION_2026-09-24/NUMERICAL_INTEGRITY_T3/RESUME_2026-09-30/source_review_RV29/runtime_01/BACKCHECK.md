# RV29 compiler-repair backcheck

The exact core delta from3cf296e36645d97e4c657c8ad1a6322bc4163f16 to
dd1f70d8ba85b19f7d948bca6ee08a44bbb12ae1 contains only the three authorized edits:
explicit Wide::<4>::from_f64 and Wide::<4>::ZERO in the new publication tests,
and one distinct PublicationEnclosure diagnostic arm in method_tests.rs naming
the layout index and predicate. No existing matcher arm, fixture, expected
outcome, tolerance or assertion changed. The complete delta is COMPILE_REPAIR.diff.

The candidate's adaptive.rs and three accepted price primitives are byte-equal
to candidate_01; the independent ledger remains applicable to these helpers.
The new source tests I22 is separately authoring are not in this runtime archive.

The default-target cargo command compiled successfully and began executing tests
on this immutable candidate. That closes RV29-C1/C2's compiler defects for dd1,
not any numerical/protected-outcome failure of the resulting suite. Runtime
result is reported separately only after the one authorized run completes/stops.
