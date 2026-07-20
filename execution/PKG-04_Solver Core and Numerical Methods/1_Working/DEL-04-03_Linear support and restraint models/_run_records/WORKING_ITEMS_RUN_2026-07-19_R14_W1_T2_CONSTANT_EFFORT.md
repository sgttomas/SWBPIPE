---
doc_id: WORKING-ITEMS-RUN-2026-07-19-R14-W1-T2-CONSTANT-EFFORT-DEL-04-03
doc_kind: working_items.run_record
status: draft
created: 2026-07-19
agent: WORKING_ITEMS
package_id: PKG-04
deliverable_id: DEL-04-03
tranche_id: R14-W1-T2
candidate_brief: CB-2026-07-19-T2-DEL-04-03-CONSTANT-EFFORT-001
decision_refs: [DEC-049, DEC-026, DEC-065, DEC-081, DEC-085, DEC-087]
---

# WORKING_ITEMS Run - R14 W1 T2 Constant-Effort Assembled-Solve Consumption

## Scope

Serialized T2 tranche of campaign run
`HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` (wave W1), executed by a
governed T2 executor under the sealed candidate brief
`execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T2_DEL-04-03_CONSTANT_EFFORT_SOLVE.md`
(v3 COMMIT-SAFE; verifier chain at
`execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T2/`).
Base commit `723c95b0f` on branch `claude/piping-r14-pkg04-mechanics`.
Selected work item: the sole DEL-04-03 `_STATUS.md ## Remaining` row directing
the DEC-049-anticipated deeper assembled-solve consumption for constant-effort
spring-hanger supports. Offline only; the executor did not commit.

## Implementation Evidence

- `core/product_physics/src/lib.rs`: ideal constant-effort element. A
  constant-effort support (no `nonlinear` field) meeting the consumption
  conditions — exactly one declared translational restraint DOF and a finite
  positive user-entered `hanger.constant_load` (already force-normalized) —
  contributes a constant nodal force of that magnitude at its node, along the
  positive axis of the declared DOF, in every solved load case. The force is
  added to the per-load-case assembled force vector before `reduce_system`
  (`add_constant_effort_support_loads`), the single seam consumed identically
  by the dense, sparse, and nonlinear active-set solve paths. Zero stiffness
  contribution; no restraint row (the existing `build_model` exclusion is
  unchanged).
- Data-driven opt-in, no hidden defaults: non-consuming shapes (zero declared
  translational DOFs — the accepted pinned-fixture shape — more than one,
  an unparseable declared restraint DOF, an unresolvable support node) stay
  review-only and emit one non-blocking `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED`
  warning naming the unmet condition. Missing/non-positive constant load keeps
  the pre-existing blocking `CONSTANT_EFFORT_LOAD_MISSING` validation
  behavior unchanged. A support with a `nonlinear` field keeps existing
  nonlinear-path handling. No accepted input shape became blocking.
- Result rows: per load case, each consuming support emits a
  `constant_effort_support_applied_load` row whose basis cites
  `dec_ref=DEC-049`, `mechanics_consumption=assembled_solve`, and
  `consumed_dof=<DOF>`, with the verbatim positive-axis sign convention. The
  existing `constant_effort_user_input_review` rows are preserved; their
  sign-convention disclosure now truthfully states the landed assembled-solve
  consumption semantics instead of the former "no global constant-effort
  load ... is claimed" clause.
- User-limit warnings: when the user entered `movement_limit` and/or
  `travel_range`, the computed displacement magnitude at the support node
  along the acting DOF is compared per load case and exceedance emits a
  non-blocking `SUPPORT_CONSTANT_EFFORT_USER_LIMIT_EXCEEDED` warning. The
  comparison uses user-entered values only; no software constant, threshold,
  tolerance, or acceptance criterion is introduced.
- Evidence artifacts: new hand-calc witness
  `validation/hand_calcs/mechanics/constant_effort_support_applied_load.md`
  (invented, project-original; superposition derivation and closed-form
  values); new suite fixture `MECH-CONSTANT-EFFORT-SUPPORT-APPLIED-LOAD`
  (`constant_effort_support_applied_load_fixture` +
  `solve_constant_effort_support_applied_load`) registered in
  `fixture_inventory()` with the conventional count-assertion bump (21→22),
  compared at the already-recorded DEC-026 analytic-class `1.0e-9` relative
  tier (`INTERNAL_ASSERTION_EPSILON` reuse; no new tolerance), plus exactly
  one additive inventory line in each mirror README
  (`validation/benchmarks/mechanics/README.md`,
  `validation/hand_calcs/mechanics/README.md`).
- Unit tests added in `core/product_physics` (eight): superposition identity;
  direction convention (declared-DOF positive axis); two-load-case
  application; every §3.2 non-consumption shape including the
  empty-restraints pinned shape and unknown node (warning, no force, no
  blocking); preserved blocking for missing/non-positive constant load plus
  direct classifier assertions; user-limit warning emission and quiet path;
  nonlinear coexistence and nonlinear-field precedence; no-op guard for
  models without constant-effort supports; review-row text/count updates.

## Validation (brief §6, in order)

- `cargo fmt --manifest-path core/product_physics/Cargo.toml --check`: PASS.
- `cargo test --offline --manifest-path core/product_physics/Cargo.toml`:
  PASS (83/83).
- `cargo fmt --manifest-path validation/benchmarks/mechanics/Cargo.toml
  --check`: PASS.
- `cargo test --offline --manifest-path
  validation/benchmarks/mechanics/Cargo.toml`: PASS (34/34).
- Mirror inventory check: both READMEs list
  `MECH-CONSTANT-EFFORT-SUPPORT-APPLIED-LOAD`; `git diff --numstat` against
  `723c95b0f` shows exactly one added line and zero removed lines in each.
- `cargo test --offline --manifest-path core/runner/headless/Cargo.toml`
  (read-only downstream regression): PASS (23 lib + 1 + 15 bin), including
  the T1-landed DEL-08-04 result-envelope producer tests; the new
  `constant_effort_support_applied_load` kind flows through the
  vocabulary-boundary disclosure path.
- del1005 byte-identity: all five `del1005_payload_binding_*` cases rerun
  before and after the change; outputs byte-identical to the committed
  witnesses in `validation/witness/generated/` in both runs (exits
  0/0/0/1/1 as documented).
- Pinned-case capture (`tp_runner_015_final_cli_solve_input.json`, solve):
  exit 0 and `COMPLETED` before and after. Ephemeral stdout captures
  (outside durable paths) with SHA-256:
  - before: `738d3c074dd90ca97497f2710aac424385e0e85144e93bcee09ba6c2a0151614`
  - after: `b3cd85af85655eadb827f366457494387ba4b58807fd5608c676958b37168613`
  Structural diff: one added non-blocking diagnostic
  (`SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` for `support:CE-120`, whose
  `restraints` list is empty), the two `constant_effort_user_input_review`
  rows carrying the updated truthful sign-convention text, and the
  consequent result-envelope checksum value; result-row count unchanged
  (830/830); no row added or removed; `runner_result` otherwise identical;
  `CliOutput` shape unchanged. The committed historical witnesses were not
  edited.
- `python3 tools/validation/validate_claims_language.py --repo-root .`
  (repo root): PASS.
- `python3 tools/validation/validate_path_anchors.py . --text` (repo root):
  PASS.
- `git diff --check`: PASS.
- JSON parse of new/changed `.json`: only
  `instances/W1/T2/CHANGE_SCOPE_CONTAINMENT.json` (parsed on write); no
  project `.json` file changed.
- Changed-path containment
  (`tools/software_workflow/validate_change_scope.py` against base
  `723c95b0f`): PASS; JSON stdout persisted to
  `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T2/CHANGE_SCOPE_CONTAINMENT.json`.

Branch-level registered checks run once at W1 wave closeout per the
controlling W1 dispatch (not part of this tranche).

## Follow-On For HELP_HUMAN (docs lane; not fixed here)

The reproduction-manual case-1 documented solve expectations
(`docs/validation_manual/headless_runner_reproduction.md`, Part 1 fixture
table) are stale for post-tranche sources: the same frozen command now also
emits the non-blocking `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` warning and
the envelope carries the updated review-row disclosure text, while exit 0 /
`COMPLETED` and the empty request/result validation posture are unchanged.
Per the R12→R13 case-3 precedent this needs a dated note through a later
HELP_HUMAN docs-lane selection. The committed
`validation/witness/generated/tp_runner_015_*.json` witnesses and prior
reproduction bundles remain truthful for their pinned pre-#287 commits and
were not edited in this tranche.

## Boundaries

No catalog sizing, manufacturer table, protected standards value, hidden
default, gravity-coupled direction inference, schema/input-shape change,
variable-spring behavior change, nonlinear constant-effort behavior
(DEL-04-04 owner surface), threshold/tolerance creation or promotion
(DEC-046 state untouched; DEC-026 tier reused as recorded), witness or
reproduction-bundle edit, docs write, lifecycle/stage/release/issuance act,
push/PR/merge, or professional approval, certification, sealing,
authentication, or code-compliance claim.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
