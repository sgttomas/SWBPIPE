# K-D5 member-relative reference

`notice_reference.py` independently forms the round-to-nearest 128-bit frame
stiffness and loads using exact fractions and integer square-root rounding, then
solves all twelve free equations exactly. `request.json` is the complete invented
thermal L control consumed directly by the PP notice regression. The generator
checks its geometry, member topology, section, material, strain, fit, support and
rotation leaves against the independent calculation.

Run `python3 notice_reference.py` to verify `oracle.json`; use `--write-oracle`
only when intentionally replacing the checked oracle. The frozen oracle binds the
complete request bytes with SHA-256 and its actual normalized section leaves.
Its binary64 norm upper bounds are established by exact rational squared-norm
comparisons, including upward correction. Decimal norms are display values only.
The PP test checks the request binding and actual normalized leaves before
comparing both notice bounds to those independent upper bounds.

The real Euler–Bernoulli field has zero mechanical actions. The p-rounded
intended system instead has zero force groups and tiny nonzero moment groups;
the reference reports their maximum. This test-only code has no product-reader
role and no scratch-file dependency.

`two_body_request.json` is the maintained negative control: two copies of the
thermal/rigid-rotation body, with a resolved 1e-7 N nodal force on the second
body. `two_body_reference.py` reuses the independently reviewed RN128 operations
and exactly solves both twelve-free-DOF systems. Run it to verify
`two_body_oracle.json`; `--write-oracle` intentionally replaces that oracle.
The full request is byte-bound, the normalized sections are explicit, and each
reported norm upper bound is proved by an exact rational squared comparison.
The loaded anchor force is exactly minus the binary64 nodal input. This exposes
why cancellation-scale smallness alone cannot establish a stress-free notice:
every group must also be zero-compatible with its own member bound.
