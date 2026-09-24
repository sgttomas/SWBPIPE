# Independent repaired-candidate backcheck

TASK `/root/solver_manager/input_review` returns to `/root/solver_manager` using delegated-harness-native execution. Same independent reviewer, no implementation authorship or product source writes. The prior complete review remains in REVIEW.md; this backcheck covers the complete delta from its preserved SOURCE_DIFF.patch and affected consequences.

**Final disposition: suitable for manager fan-in for this bounded two-file input-contract slice. No unresolved actionable findings.** R1–R3 and V1 are closed against the repaired hashes below and actual manager-run evidence. This does not clear broader integration, native, engineering-acceptance or release gates.

Reviewed repaired source identities:

| Logical source | SHA-256 |
|---|---|
| `projects/chirality-piping/core/product_physics/src/lib.rs` | `904f72876024943778788e56fd4fe7dac03387350282cec215d1be0cf3f129b5` |
| `projects/chirality-piping/core/product_physics/src/validation.rs` | `4573e181828d84f7fa385796162bb3bd2c9908ad32d29a7f193673125d4d67c1` |

The manager temporarily swapped working source for the baseline. Review used immutable `{REPAIR_LIB_SNAPSHOT}` and `{REPAIR_VALIDATION_SNAPSHOT}`, whose hashes match this table; working files were not assumed to represent the candidate during that interval. REPAIR_DIFF.patch preserves the entire reviewed repair delta. BACKCHECK_ORIGINS.json records origins, parentage and limits.

## Finding dispositions

- **R1 repaired:** constant-effort records are excluded from rigid/contact comparison using the same effective classification that causes the builder to omit them from stiffness/constraints. The new constant-force/contact test checks the co-located same-axis case with both inactive and active gap, explicit consumed force and signed contact reaction, in sparse and dense modes.
- **R2 repaired:** separate spring identities can act in parallel with contact. Same-record spring/nonlinear overlap remains blocked, preventing the current support-result overwrite ambiguity. The new spring/contact test checks both open and closed contact, elastic displacement, spring magnitude, root magnitude and signed contact force; it also exercises the same-record negative control. The actual supported spring axis follows stiffness, with the same fallback to first restraint as construction. Genuine rigid/contact overlaps remain blocked by the retained check.
- **R3 repaired:** nominal `family="spring"` with effective constant-effort hanger semantics emits `SUPPORT_FAMILY_HANGER_CONFLICT`; constant-effort records also cannot pass the consumed-spring stiffness guard. The new regression covers the contradictory payload both without and with stiffness; the latter must also emit `SUPPORT_STIFFNESS_FAMILY_UNSUPPORTED`, and both return no results.
- **V1 addressed:** a proper orthogonal skew rotation and all three local-axis couples, each positive and negative, are authored through three global `rotation_x/y/z` records. The test checks independently rotated global rotations, cross-product translations, signed local i/j end moments and zero resultant support force. It runs both modes, uses exact annulus inertias and allows only envelope rounding plus small arithmetic allowance. This closes the specific requested skew-moment verification gap.

No other production changes occur after the reviewed freeze: lib.rs adds four regression tests and their helper; validation.rs refines classification/overlap. M04 provenance, M21 hydrotest containment, M35 parsing and the initial disjoint guide preservation remain as previously reviewed. Full signed support-moment publication and native persistence witnesses remain programme work.

## Reference and test adequacy

The skew columns are orthonormal and right-handed (determinant +1), independently checked by lightweight Python arithmetic. The force/moment transformation and sign expectations agree with the prior independent end-couple derivation and current-source comparison in REVIEW.md; no new constitutive theory is introduced.

For the repaired axial controls, A=0.0035405749205956985 m² and EA/L=354057492.059570 N/m at L=2 m. With g=0.05 mm, the 1 kN input stays open and the 50 kN input closes the gap for both controls. Independently calculated closed-state values are:

| Parallel record | Root reaction N | Parallel spring reaction N | Contact reaction N | Applied total N |
|---|---:|---:|---:|---:|
| +1500 N constant force | -17702.874603 | 0 | -33797.125397 | +51500 |
| 100 MN/m spring | -17702.874603 | -5000 | -27297.125397 | +50000 |

Signed forces balance. Root/spring publication is magnitude-only, so the tests appropriately compare their magnitudes and use the signed nonlinear row for contact; this is not misrepresented as full signed reaction publication. BACKCHECK_REFERENCE_CALCULATIONS.json preserves both open/closed cases and the rotation checks. Inputs are explicit synthetic small-strain cases, not catalog/material defaults, pressure validity, or professional qualification. Existing protected test tolerances were not changed.

## Execution evidence

The baseline raw log was reviewed: exactly three new defect regressions fail on prior validation, and the skew companion passes. R1/R2 failures show the old incorrect conflict diagnostic; R3 shows the incorrectly solved contradictory family. This is consistent with the source diagnosis, rather than accepting any generic test failure as reproduction.

Repaired execution evidence has now been reviewed: focused `input_contract_` run passes 9/9; complete product crate passes 184/184 unit tests with zero doc tests, both exit 0. The full log contains 184 named passing test rows. All three raw log SHA-256 values match CANDIDATE_RUN.json. Restored working lib.rs and validation.rs hashes match the immutable candidate above. The baseline source witness in BASELINE_RUN.json retains its prepared-state wording; completed outcomes and execution authority are recorded in CANDIDATE_RUN.json and its hashed logs.

No Cargo/build/native/browser/npm workload was run by this reviewer. Tests are explicitly manager-run evidence, now checked against the reviewed candidate hashes; no broader sweep, native, engineering or release gate is implied. No OS isolation, model diversity, human acceptance, release or native witness is claimed. Source review is complete and the freeze is handed back to the manager; subsequent source changes require affected backcheck coverage.

## Portable view custody

This record is a portable view; the original observed bytes remain at `projects/chirality-piping/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260918-UI-IMPLEMENTATION/instances/CONTINUATION_2026-09-24/SOLVER_MANAGER/_run_records/portability/INPUT_REVIEW/BACKCHECK.md` (repository-relative), SHA-256 `d6190dd4f7a1337b75dd3531db57a679636f228c675d32a1c07ed6a72a056843`. Pre-existing hashes identify original observations, not this rewritten view; no rerun is implied.

- `{REPAIR_LIB_SNAPSHOT}`: Historical external immutable lib.rs review snapshot; exact filename is retained at the indicated raw record locations. Existing source hash identifies its bytes. Raw locations: line:14.

- `{REPAIR_VALIDATION_SNAPSHOT}`: Historical external immutable validation.rs review snapshot; exact filename is retained at the indicated raw record locations. Existing source hash identifies its bytes. Raw locations: line:14.
