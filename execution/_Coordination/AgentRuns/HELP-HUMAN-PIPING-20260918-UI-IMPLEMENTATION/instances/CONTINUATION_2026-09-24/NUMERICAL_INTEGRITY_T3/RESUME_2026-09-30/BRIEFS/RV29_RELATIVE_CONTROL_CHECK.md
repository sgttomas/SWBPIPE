# RV29 — check ROOT's concrete relative-radius fixture

ROOT's proposed isolated control is in verification/ROOT_RELATIVE_RADIUS_CONTROL.json.
It addresses your test gap for admitted H whose RU64(H) exceeds the sharper exact
non-binary64 allowance while remaining within its representable binary64 allowance.
Independently verify the exact dyadic algebra, five-step binary64 rounding, actual
relative-class membership and representability of H in a P256 report. Prefer
integer rounding/source math; no observed solver/helper counter is an oracle.
If correct, state the compact certificate fixture requirements and bounds; if not,
return the precise defect. Do not author maintained test code or a new model.
Write only source_review_RV29/relative_control_03, preserve older seals. No Rust
or runtime slot. Ten-minute bounded check; report unresolved points, not a search.
The initial failed adjacent candidate is disclosed in the proposal and is not
part of the accepted fixture. I22 has not been given this fixture as truth yet.
