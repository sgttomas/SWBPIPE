# Independent review: minimal precision-1 gap publication containment

No actionable defect found in the complete frozen tests-only change plus
REPAIR_01. This candidate is suitable for manager fan-in and the exact postfix
integration suite followed by full affected checks. This is static review
clearance, not a test pass, merge clearance, engineering acceptance, or Current
qualification. No production or test source was changed by this reviewer.

Reviewed candidate, relative to REPO_ROOT:

- `projects/chirality-piping/core/product_physics/src/lib.rs` SHA-256
  `37d19221af9900e40d500370c30c156e49f5469a905f20acb3b2e3db6ae54d80`;
  before `e215487709e06275bb92796455560e1bea16253c1c4fbf614f182b7da96d2df2`.
- `projects/chirality-piping/core/product_physics/tests/represented_gap_publication.rs`
  SHA-256 `1c33c4f0604b3eb0e31e689f3f965e65fd5d832377737f366b1b25e038fc1b78`.

[CHECKS.json](CHECKS.json) records independent hash, exact diff reconstruction,
explicit-path scope validation, analytical preconditions, and baseline-log
checks. Both frozen diffs reconstruct exactly. Production has precisely three
hunks; the new integration test is unchanged. Comparing all 55 baseline source,
manifest, lock and literal-include targets against the reviewed checkout finds
only the declared lib.rs repair. This establishes this slice's scope; it is not
an independent whole-checkout Git-status assertion. No Git command was run.
Actual origins, hashes, delegation, restrictions and preserved context bytes are
in [_run_records/ORIGINS.json](_run_records/ORIGINS.json). The root, TASK,
Piping project, LOOP_INIT and selected project skill bodies were read from the
assigned checkout. No other role or workflow was activated; no child delegated.

The source trace supports the intended containment. At lib.rs:1721 the branch
uses the selected final iteration's `StrictGapEvidence::Qualified`, after the
existing convergence and selected-state checks. The original ordinary report
remains a preprojection precondition. The nonlinear adapter replaces selected
global displacements and reactions with binary64 projections derived from its
retained exact ratios; it separately evaluates equilibrium on the projected
displacement. Product member recovery still consumes that projected displacement
(lib.rs:1915 and following). Therefore the warning correctly withholds mixed
field-recovery qualification and does not turn the preprojection report or
selected-public-state equilibrium into a proof of complete exact-ratio recovery.

The new warning uses the existing case-specific integrity ID and the actual
load-case ID in affected_refs. The assessment at lib.rs:853 maps that evidence
to numerically_unresolved / unresolved / not_assessed / unresolved and binds its
ID in the case evidence_refs. The existing worst-case aggregate becomes
unresolved. Preliminary solve diagnostics do not create a competing passing
integrity diagnostic. Failed initial solves, unavailable/nonconverged selected
states, and unqualified strict-gap evidence retain their blocking paths before
ordinary publication. The warning itself preserves computed rows and mechanics
inspection; no-gap cases still take the untouched ordinary report branch. Other
nonlinear states retain their prior path. No computation, convergence limit,
protected numerical criterion, historical 0.05 oracle, schema, source capture,
persistence requirement, or broad gap-family ban was added or changed.

The public tests preserve the real invented fixture's version and metadata while
explicitly replacing the physical entities and loads. The one-metre annular
elastic member has one free axial tip DOF, explicit material values, a fixed
root and no pressure, thermal or combination loading. The anchor-family subset
is supported by the actual adapter and linear-support restrictions; it makes no
standard support-product claim. Independently, A=pi*t*(OD-t)=0.0014922565104551519
m² and FL/(EA) give 0.08376575952205018 mm at 25 kN and 0.3350630380882007 mm at
100 kN, bracketing the 0.1 mm stop. The peak ordinary axial strain is about
0.000335. Both signs, modes and seeds are exercised. The four no-gap solves use
the retained 1e-9 relative displacement criterion and a solver-independent
expected value. The gap tests prove intended supported execution first, then
require the specific case-bound nonpassing reason; arbitrary input rejection
cannot satisfy them. They are containment tests, not gap accuracy qualification.

The observed baseline log is independently consistent with the sealed test:
all 16 gap cases panicked at the specific missing-containment assertion on line
119 after loop/contact prerequisites, with MECHANICS_SOLVED and an ordinary
CHECKS_PASSED report. The no-gap companion passed. The baseline run returned 101
and its before/after snapshots match. These are pre-repair observations. This
review ran no Rust/Cargo/build/npm/native operation and claims no postfix result.

Existing precision-1 contracts admit the new combination of already-supported
status values. Rust `semantic_contract::numerical_use_standing` and TypeScript
`numericalResultStanding` reject unresolved quality. The latter gate feeds
currentSolvedResult, the RuleCheckPanel input, Current result export and the
stress-neutral export path. Thus source inspection supports refusal of the same
raw precision-1 result without a precision-2 persistence prerequisite. An actual
postfix raw result passed through those consumer routes remains verification
work for the owning managers. The separately owned TypeScript aggregate issue
still labels a consistent unresolved case aggregate contradictory in the reviewed
consumer bytes; it does not grant eligibility and is outside this producer diff.
Its authoring-owned correction requires its own review and checks.

Proceed with the parent's frozen `represented_gap_publication` postfix command
under the approved resource lease, then the affected product/consumer checks.
Preserve failures and diagnose any protected-criterion conflict rather than
weakening an oracle. Previously reported serialization/frame/nonlinear passes
are earlier evidence, not postfix validation from this reviewer. Whole-gap
capability, complete mixed-field accuracy, engineering Current, and any required
native/practitioner witness remain open at their existing owners.

Reviewer: TASK `/root/solver_manager/gap_publication_review`, parent
`/root/solver_manager` WORKING_ITEMS, delegated-harness-native fresh child; no
descendants. ROOT's minimal solver-first precision-1 scope was relayed in the
parent's brief; this return does not assert personal owner review.
