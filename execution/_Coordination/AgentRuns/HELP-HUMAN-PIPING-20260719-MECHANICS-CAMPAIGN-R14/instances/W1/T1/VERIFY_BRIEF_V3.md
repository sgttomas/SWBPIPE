# VERIFY_BRIEF_V3 — Third-Round Fresh-Context Adversarial Verification of CB-2026-07-19-T1-PKG04-PRODUCER-BINDING-001 (v3, post-executor-BLOCK amendment)

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W1 / T1
**Verifier:** fresh-context adversarial verifier, third round (governed Agent 2, non-delegating)
**Parent:** W1 PKG-04 package manager (WORKING_ITEMS)
**Objects reviewed (both at amendment v3):**
- `execution/_Coordination/CANDIDATE_BRIEF_2026-07-19_T1_PKG04_PRODUCER_BINDING.md`
- `execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14/instances/W1/T1/CURRENT_CANDIDATE_RATIONALE.md`
- preserved history (read-only): `instances/W1/T1/VERIFY_BRIEF.md` (v1 BLOCK),
  `instances/W1/T1/VERIFY_BRIEF_V2.md` (v2 COMMIT-SAFE, superseded),
  `instances/W1/T1/EXECUTE_RETURN.md` (executor BLOCK, claims E1–E15),
  `instances/W1/T1/CHECK_LOG.md`

**Verified against:** live tree at HEAD `6152908b3246df61150dc91e3558788b05dfb643`,
branch `claude/piping-r14-pkg04-mechanics` (confirmed by `git rev-parse HEAD` /
`git branch --show-current`; working tree clean except the lawful untracked R14
AgentRuns state and the candidate brief). All checks offline
(`CARGO_NET_OFFLINE=true`, `--offline`); no install, fetch, or network use. No
file modified other than this return. The pre-existing offline debug build of
`openpipestress-runner` at this HEAD was rebuilt (`cargo build --offline`,
no-op finish in 0.09s) and used for independent empirical reproduction.

---

## 1. Amendment-Cure Assessment

### 1(a) The executor's core blocking evidence is independently CONFIRMED in source and by empirical reproduction

- **Stiffness review-echo rows.** `core/product_physics/src/lib.rs` lines
  6795–6820: `component_user_stiffness_macro_element_review` rows are emitted
  with literal units `"N/m"` (axial, lateral) and `"N*m/rad"` (angular,
  torsional) for expansion-joint components;
  `spring_hanger_user_input_review` (line 6941) emits the user-entered
  variable-spring-hanger stiffness via `append_hanger_quantity_result`
  (line 7042, unit pass-through from the user quantity).
- **Nonlinear evidence-scalar rows.** `append_nonlinear_support_loop_results`
  (`core/product_physics/src/lib.rs` line 1540) unconditionally emits, on
  every successful nonlinear solve: iteration-count (`count`, line 1619),
  final-residual-count (`count`, 1637), converged-flag (`boolean`, 1653),
  per-support state-code (`state_code`, 1697), plus residual-observation
  rows (line 1665).
- **Closed DEL-08-04 vocabulary.** `core/reporting/result_export/src/lib.rs`
  lines 41–56: `DimensionId` has exactly 14 members with no stiffness,
  energy/work, count, or state identifier; lines 58–69: `ResultFamily` has
  exactly 9 members with no TBD/evidence member.
  `schemas/results.schema.yaml` `DimensionId` (14 values incl. `TBD`) and
  `ResultFamily` (9 values) enums match the crate exactly. `Tbd` is blocked on
  any exported value row (`RESULT_EXPORT_VALUE_METADATA_INCOMPLETE`,
  crate lines ~790–804 — read directly: `value.dimension == DimensionId::Tbd`
  triggers an export-blocking diagnostic).
- **Accepted D-01 classification.** `docs/SPEC.md` §4 (lines 173–184):
  "generic `stiffness` must be classified as `linear_stiffness` or
  `rotational_stiffness`"; `schemas/units.schema.yaml` lines 266–267 carry
  both identifiers; `execution/_Coordination/_DECISIONS/D-01_unit_catalog_acceptance.md`
  row E3 records the 30-identifier vocabulary as accepted. Neither identifier
  exists in the DEL-08-04 enums, so no truthful DEL-08-04 dimension exists for
  the stiffness echoes — E6/E11's substance stands.
- **Empirical fixture reproduction (independent).** I ran the offline-built
  runner on the pinned `tp_runner_015_final_cli_solve_input.json`: exit 0,
  stdout SHA-256 `738d3c074dd90ca97497f2710aac424385e0e85144e93bcee09ba6c2a0151614`
  — byte-identical to the executor's recorded §6 baseline, so the executor's
  baseline capture is exactly reproducible. The captured envelope carries
  **830 result rows across 44 distinct `(kind, unit)` pairs**, including
  exactly 2× `(component_user_stiffness_macro_element_review, N/m)`,
  2× `(…, N*m/rad)`, and 1× `(spring_hanger_user_input_review, N/m)` —
  E5 confirmed to the row count. `runner_result.diagnostics` is empty on this
  fixture (exit 0), so any §3.5-blocking treatment of those rows would have
  changed stdout and exit code, confirming E10/E11: the v2 predicate set
  (§3.2 map-every-row fail-closed + §3.5 + §3.7(b)) was jointly unsatisfiable
  on the live tree without inventing semantics. The executor's BLOCK was
  correct.

### 1(b) The v3 predicates are jointly SATISFIABLE on the live tree

- **§3.2 bounded-coverage floor is truthfully coverable.** The pinned
  fixture's straight-pipe element row classes all map truthfully into the
  existing vocabulary: `global_nodal_displacement_*`/`displacement_magnitude`
  (mm → length/Displacement), `global_nodal_rotation_*` (rad → angle/
  Rotation), `element_local_axial_force`/`element_local_shear_force_*`
  (N → force/Force), `element_local_bending_moment_*`/
  `element_local_torsional_moment` (N*m → moment/Moment), all
  `*_stress` rows incl. `pipe_section_pressure_hoop_stress` (MPa →
  stress/Stress), `reaction_resultant` (N → force/Reaction). The validator's
  extra metadata gate for Force/Moment/SectionProperty families
  (`RESULT_EXPORT_FORCE_MOMENT_METADATA_INCOMPLETE`, crate lines 806–820) is
  satisfiable: I checked the live fixture output — every `element_local_*`
  force/moment/stress row carries complete five-field metadata
  (component/coordinate_system/location/basis/sign_convention), and the
  `ResultMetadata` shapes in `core/product_physics` (line 647) and
  `core/reporting/result_export` (line 216) are field-identical.
  `reaction_resultant` rows lack complete metadata but the `Reaction` family
  is exempt from that gate. The disclosed classes (stiffness echoes,
  count/flag/state-code, mode-code, residual observations) genuinely lack a
  truthful representation, per 1(a).
- **§3.2 non-blocking per-row disclosure is representable without any schema
  or crate edit.** The DEL-08-04 `Diagnostic.code` is a free-form string
  (schema: `minLength: 1`, no enum); `DiagnosticClass` includes non-blocking
  members (`UnitWarning`, `ProvenanceWarning`, `AssumptionWarning`,
  `NonlinearWarning`); `DiagnosticSeverity` includes `Info`/`Warning`; and
  `validate_result_envelope` (crate lines 362–452, read in full) imposes no
  constraint on envelope `diagnostics` content — it blocks only on envelope
  metadata, boundary, provenance, reproducibility, rule-pack, and result-set
  defects. All `Diagnostic`, `QuantityResult`, `Provenance`, `ResultMetadata`
  fields are `pub`, so the producer can construct everything through the
  public API. A per-row vocabulary-boundary diagnostic with severity
  info/warning is validator-clean.
- **§3.3 nonlinear named metadata binds fully.**
  `core/solver/nonlinear_integration/src/lib.rs`: `pub assumptions` (311) /
  `pub limitations` (312), `pub fn assembled_loop_assumptions()` (506) /
  `pub fn assembled_loop_limitations()` (518), crate version `0.1.0`
  (`Cargo.toml` line 3); `grep` confirms zero consumers of either vector in
  `core/product_physics` (the drop is real). Solver version/name are plain
  strings on `ResultEnvelope`; `ASSUMPTION_WARNING`/`NONLINEAR_WARNING`
  classes carry the assumption/limitation rows non-blockingly; envelope
  `Diagnostic.provenance`/`source` carry crate-naming provenance. The
  count/flag/state rows are result rows, not the item's named metadata, so
  routing them through the predicate-2 disclosure leaves DEL-04-04 item 1's
  named content (solver version, warnings, assumptions, limitations,
  diagnostic provenance) fully bound.
- **§3.5 + §3.7 are jointly satisfiable with the disclosure present.** The
  joint predicate set pins the disclosure onto the non-serialized surface: a
  disclosure appended to `runner_result.diagnostics` would change `CliOutput`
  bytes and the exit code (the `solve` verb derives both from
  `preview.runner_result.diagnostics` — bin lines 432–441, read directly),
  violating §3.7(b); a disclosure carried in the envelope document riding the
  serde-excluded `PreviewRunnerOutput` field changes nothing observable on
  any pinned surface. Since §3.5 declares the disclosure non-blocking with no
  exit-code change and §3.5/§3.7 must hold simultaneously, only the second
  implementation is lawful, and it exists. Satisfiable.
- **§3.7 CLI stability verified live.** I independently re-ran all five
  `del1005_payload_binding_*` cases at HEAD: exit codes 0/0/0/1/1, stdout
  byte-identical to the five committed witnesses (`cmp`). The three
  tp_runner_015 committed witnesses are historical, exactly as §1/§2 state:
  the committed `validation_blocking` witness has zero `suite_run`
  occurrences while live `CliOutput` carries the field (bin line 87); the
  committed benchmark-stub witness records
  `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` while live
  `RunBenchmark` emits `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` (bin line
  299). The §3.7(b) pre-tranche-baseline mechanism is demonstrated workable:
  my fresh capture reproduced the executor's recorded baseline SHA exactly.

### 1(c) The `headless_preview_runner` discovery (E14) is cured by the serde-exclusion clause

`core/runner/headless/src/bin/headless_preview_runner.rs` lines 92–96 print
`serde_json::to_string_pretty(&PreviewRunnerOutput)` directly — the
tp_runner_014 witness surface (committed at
`validation/witness/generated/tp_runner_014_headless_entrypoint_preview_run.json`,
confirmed present). `PreviewRunnerOutput` derives `Serialize` **only**
(`core/runner/headless/src/lib.rs` line 669: `#[derive(Debug, Clone,
Serialize)]`) — there is no `Deserialize` derive anywhere on the struct, so a
`#[serde(skip_serializing)]` field carries no deserialization hazard, and the
cure is complete for every serialization surface: the tp_runner_014 bin
serializes the whole struct (cured by skip), and the DEC-065 bin is shape-safe
by construction because `base_output` field-accesses
`preview.runner_result`/`preview.mechanics_envelope` (bin lines 464–467)
rather than exhaustively destructuring. The cure suffices.

**Amendment-cure verdict: the v3 amendment cures the executor's BLOCK.** The
blocking condition was a predicate-design defect, exactly as rationale §3
item 7 records; the bounded-coverage mapping removes it without touching any
fence-external surface.

## 2. Per-Claim Verdict Table (C1–C18)

| Claim | Verdict | Evidence (live tree, this round) |
|---|---|---|
| C1 | **CONFIRMED** | `DEL-04-02 …/_STATUS.md`: `IN_PROGRESS`; `## Remaining` holds exactly one item, verbatim match to the brief's selected item 1 (re-read this round). |
| C2 | **CONFIRMED** | `DEL-04-04 …/_STATUS.md`: `IN_PROGRESS`; Remaining item 1 verbatim match; items 2–6 re-read — PDU-035 REVIEW/dimensional hold, friction "(gated: new D-XX ruling…)", three threshold-promotion rows (DEC-046 / PRD D6/D9 / DEC-052/DEC-054, one stage-gated R5) — all excluded by §9. |
| C3 | **CONFIRMED** | Re-ran `grep -rn "ResultEnvelope {" core --include="*.rs"` excluding `ResultEnvelopeRef`: constructors only at result_export lib.rs 941/1000/1226 (crate test module) and `report_package/tests/container.rs` 155. No non-test constructor at HEAD. |
| C4 | **CONFIRMED** | `validate_result_with_optional_envelope_payload` at headless lib.rs 563; `Some(...)` callers only at 1126/1149 (test module) and a `None` caller at 1164; none in `src/bin/`. Validator content as described (v1/v2 line-level evidence spot-confirmed unchanged; tree identical at same HEAD). |
| C5 | **CONFIRMED** | `pub assumptions`/`pub limitations` (nonlinear_integration lib.rs 311–312), public assembled-loop accessors (506/518), crate `0.1.0`; `grep -c "assumption\|limitation"` over `core/product_physics/src/lib.rs` returns 0 — the vectors have no consumer. |
| C6 | **CONFIRMED** | `MechanicsEnvelope` carries `results`/`diagnostics`; empirically the pinned solve envelope carries 830 rows and 30 diagnostics with review/provenance codes; the brief maps, never recomputes. |
| C7 (v2) | **CONFIRMED (independently reproduced)** | I re-ran all five del1005 cases at HEAD: stdout byte-identical to committed witnesses, exits 0/0/0/1/1. tp_runner_015 historical status verified directly (witness lacks `suite_run`; benchmark code drift; consistent with the R13-refreshed manual). Amended §3.7 predicate satisfiable — baseline mechanism proven by exact SHA reproduction of the executor's capture. |
| C8 | **CONFIRMED (v3 re-derived)** | The v3 §4 additions (disclosure logic + mapping table in the binding module, `#[serde(skip_serializing)]` field in headless lib.rs, out-of-vocabulary unit tests, follow-on recording in run records/instance dir) require only fence items 2–4 and 6–8. No `core/reporting/**`, `schemas/results.schema.yaml`, `validation/**`, or `docs/**` write is needed; DEL-10-05 `_STATUS.md` Remaining still carries the open `export-results` bullet (re-read), so the CLI deferral remains truthful. |
| C9 | **CONFIRMED (descriptor defect carried)** | Both `Dependencies.csv` files re-parsed this round: DEL-04-02 eight EXECUTION UPSTREAM rows all `SATISFIED`; DEL-04-04 E001–E005 `SATISFIED`, E006–E009 `PENDING` with committed implementation evidence live for each target. Carried defect: E009 is `DependencyType` CONSTRAINT, so §2's "non-constraint" descriptor remains wrong for that one row (V3-D2). |
| C10 | **CONFIRMED** | No predicate/task in v3 introduces a tolerance, threshold, acceptance criterion, or release/CI vocabulary; the mapping table is expressly forbidden to introduce identifiers; PDU-035/DEC-046/threshold holds preserved (§9/§10); lifecycle stays `IN_PROGRESS`. |
| C11 | **CONFIRMED** | Rationale §1 ten-class screen checked one-to-one against ratified D-52 §4.1 (lines 85–119, re-read this round); my independent re-screen (§3 below) reaches PASS on the v3 brief, including the deep class-4 and class-9 probes the parent tasking required. |
| C12 | **CONFIRMED** | Offline rebuild finished as a no-op at this HEAD (cargo 1.92.0); runner executed on eight witness inputs; python3 validators ran (claims-language VALID, 262 files; path-anchors PASS, 643 surfaces — both re-run this round with the v3 text in the tree). |
| C13 | **CONFIRMED** | §10 v3 record keeps owner standing approval (DEC-085/D-52 §2 at `f14fa775…` — re-verified an ancestor of HEAD) distinct from agent classification; `OwnerCaseSelection: NONE`; `EffectStatus: HELD (v3)`; `IndependentVerifier: PENDING (v3)` names this artifact and truthfully preserves the full history (v1 BLOCK → v2 COMMIT-SAFE superseded → executor BLOCK). D-54 landing `8825065d5` re-verified an ancestor. |
| C14 | **CONFIRMED** | `ORCHESTRATION_PLAN.md` re-read: version 1, MIXED, HUMAN selection authority, W1/PKG-04 T1 named as exactly this combined tranche, serialized author→verify→execute→verify chain, per-tranche commits, one PR per wave, HELP_HUMAN-only merge/receipt. Branch consistent with "one branch per package wave" (plan still does not literally name it — unchanged informational caveat). |
| C15 (v2) | **CONFIRMED** | §6 sweep treatment unchanged from the v2-verified form (controlling HELP_HUMAN dispatch quoted; refined plan sentence named — re-verified present in the plan; pre-push gate semantics intact per `docs/BUILD_AND_RELEASE.md` §5.1 as v2 verified; refinement surfaced to fan-in). §5 item 9 now reads "per the controlling HELP_HUMAN W1 dispatch (§6)" — the v2-D1 vestige is cured. Dispatch durability caveat carried (V3-D4). |
| C16 (v3) | **CONFIRMED** | Empirical + source + schema + D-01: the pinned fixture produces the stiffness echoes (`N/m`, `N*m/rad`) and count/flag/state-code/residual rows in the stated counts; the DEL-08-04 enums (crate and `schemas/results.schema.yaml`) contain no truthful identifier for those classes; D-01/SPEC §4 mandates `linear_stiffness`/`rotational_stiffness` classification. |
| C17 (v3) | **CONFIRMED** | Joint satisfiability established in §1(b): straight-pipe classes all truthfully representable (incl. the Force/Moment metadata gate — checked against live fixture metadata); nonlinear named metadata fully representable; disclosure is validator-clean, non-blocking, and pinned by §3.5+§3.7 to the serde-excluded surface, so pinned-fixture stdout (SHA-verified) and exit code are unchanged; no predicate requires an out-of-vocabulary export. |
| C18 (v3) | **CONFIRMED** | The v3 amendment adds no family/dimension identifier, no schema edit, no coerced classification (coercion expressly rejected, rationale §3 item 9); the disclosure records the existing vocabulary boundary as fact; the extension need is routed to HELP_HUMAN as a reported follow-on, not resolved in-tranche. |

**Tally: 18/18 CONFIRMED, 0 REFUTED.**

## 3. Independent 10-Class Fast-Reject Re-Screen (D-52 §4.1, re-derived item by item against the v3 brief)

1. **Irreducible owner preference / two defensible outcomes remaining:** not
   hit. The bounded-coverage selection is project-grounded (accepted D-01
   vocabulary vs live DEL-08-04 enums determine the boundary; the floor is the
   deliverables' own Remaining text); the material alternatives (park; extend
   vocabulary here; coerce) are rejected on recorded project grounds, not
   preference. The vocabulary-extension decision itself is routed to the
   owner chain, not taken.
2. **Professional/safety/legal/fiduciary accountability:** not hit. Envelope
   vocabulary pins `HUMAN_REVIEW_REQUIRED` and the professional boundary,
   enforced by the validators the tranche must pass; no reliance outcome.
3. **Conflict/contradiction ruling not determined by the authority chain:**
   not hit. The v2-resolved sweep-attribution matter is unchanged in v3 (and
   the §5.9 vestige is now cured); no accepted artifact requires every solve
   row to be exported, so bounded coverage contradicts nothing; the
   brief/rationale/live-tree/manual all now agree on witness currency.
4. **Scope/boundary change, new normative content, new acceptance criteria
   (deep probe per tasking):** not hit. (a) The mapping table is an
   implementation artifact connecting two already-accepted vocabularies; it
   may use only existing identifiers, is forbidden to invent or coerce, and
   its truthfulness is checkable row-by-row against D-01 — it creates no
   normative content. (b) The per-row disclosure records an existing
   vocabulary boundary as fact with a free-form diagnostic code on an open
   code surface (schema `minLength: 1`, no enum) — ordinary development
   vocabulary, not normative content, and expressly non-blocking so it gates
   nothing. (c) Refusing to export out-of-vocabulary rows changes no accepted
   scope: I verified empirically that the disclosed classes are component/
   support review echoes and solver evidence scalars — none is a straight-pipe
   element displacement/rotation/force/moment/stress/reaction result
   (DEL-04-02's named content) and none is the DEL-04-04 item-1 named
   metadata (solver version, warnings, assumptions, limitations, diagnostic
   provenance), all of which bind fully. Both Remaining items' recorded scope
   is discharged, the boundary is disclosed rather than silently narrowed,
   and the extension need returns to HELP_HUMAN. (d) No acceptance criterion
   is created: the §3 predicates are run criteria derived from accepted
   sources (existing validators, existing witnesses, existing D-01 text).
5. **Lifecycle/stage/issuance/release/acceptance/evidence-posture act:** not
   hit. `IN_PROGRESS` preserved; only the two named Remaining rows struck on
   success; every promotion/acceptance gate expressly preserved.
6. **Third-party/procurement/spending/publication/external action:** not hit.
   Offline, local-only; push/PR/merge/receipt reserved upward.
7. **Merge/integration authority over an accepted baseline, destructive
   action:** not hit. Ordinary local commits by the W1 manager after a fresh
   implementation verification; frozen witnesses read-only with byte-stability
   predicates; nothing rewritten.
8. **Protected/private data exposure:** not hit. Invented fixtures only;
   F-PIP-1/F-PIP-4 undisturbed.
9. **Evidence unavailable / stale basis / claim beyond warrant (deep probe per
   tasking):** not hit. Every outcome-determining v3 premise was re-verified
   this round against the live tree, including empirical reproduction of the
   executor's blocking evidence (row counts, units, enums, D-01 text, exact
   baseline SHA) and of the del1005 byte-currency. The v3 brief's premises
   about the executor return match the preserved `EXECUTE_RETURN.md` without
   softening. No premise found stale; no claim found stronger than its
   warrant (C16–C18 are each narrower than the evidence gathered here).
10. **Protected domain-engine paths, prover/tool activation, higher-order
    human boundaries:** not hit. No `_DomainEngines/**`, no prover, no
    accountable-party assignment.

**Re-screen result: PASS — no class hit on the v3 brief.**

## 4. Remaining Defects (none blocking)

- **V3-D1 — MINOR.** The v3 amendment record lists §3.2/§3.3/§3.5/§4 as
  amended, but the v2→v3 diff also includes the §5 item 9 attribution
  correction (the v2 verifier's recommended V2-D1 cure: "per the campaign
  plan" → "per the controlling HELP_HUMAN W1 dispatch (§6)") and the §10
  status/verifier-pointer progression. "No other section changed in meaning"
  is defensible — neither edit changes operative content, and both were
  v2-recommended progression acts — but the record understates the textual
  diff. Recommend the manager's progression note name both edits.
- **V3-D2 — MINOR (carried V2-D4).** Brief §2 and rationale C9 still label
  `TP-DAG-004-DEL-04-04-E009` "non-constraint"; its `DependencyType` is
  CONSTRAINT (re-parsed this round). Row identity, `PENDING` status, live
  evidence, and no-resolution treatment remain exact.
- **V3-D3 — MINOR (carried V2-D3).** §6's quotation of the plan's
  execution-rules sentence still omits the trailing "(code-touching)"
  qualifier without ellipsis; meaning unchanged (this tranche is
  code-touching).
- **V3-D4 — INFORMATIONAL (carried V2-D5).** The controlling HELP_HUMAN W1
  dispatch quoted in §6 has no durable artifact in the tree; it is validated
  by its own author at fan-in before any push. Unchanged posture from v2.
- **V3-D5 — INFORMATIONAL (carried V2-D6).** §7's "any committed-witness byte
  drift" phrase persists; the adverse reading remains foreclosed by §3.7(b)'s
  explicit "NOT the comparison target".
- **V3-D6 — INFORMATIONAL.** The brief does not name the surface on which the
  vocabulary-boundary disclosure rides. The joint predicates pin it: a
  disclosure in `runner_result.diagnostics` would change pinned stdout/exit
  (§3.7 violation, fails closed), so the only lawful placement is the
  envelope-document/serde-excluded surface, which exists and is
  validator-clean. Determinate by constraint; the implementation verifier
  should confirm placement explicitly.
- **V3-D7 — INFORMATIONAL.** For in-dimension rows whose family agreement is
  judgment-laden (e.g., `nonlinear_support_final_reaction` [N],
  `expansion_joint_pressure_thrust_load_review` [N],
  `open_formula_stress_summary` [MPa]), the executor's table may lawfully
  either export (where D-01 and the family semantics genuinely agree) or
  disclose (where they do not); the floor (straight-pipe element classes must
  export), the no-false-mapping rule, and the no-silent-drop rule bound this
  discretion and make every table row checkable against D-01. This is bounded
  implementation judgment, not semantics delegation.

## 5. Verdict

The v3 amendment cures the executor's BLOCK on its actual ground: the
blocking evidence (E5–E11, E14) is confirmed in source and by exact empirical
reproduction, the bounded-coverage predicate set is jointly satisfiable on
the live tree without inventing semantics or touching any fence-external
surface, the serde-exclusion clause fully cures the `headless_preview_runner`
discovery (the struct derives `Serialize` only), and the disclosure path
cannot change pinned stdout or exit codes on any lawful implementation. All
eighteen enumerated claims withstood independent refutation, the independent
ten-class re-screen finds no hit — including the deep class-4 probe (no new
normative content, no accepted-scope change; the disclosed classes are
verifiably neither straight-pipe element results nor the DEL-04-04 named
metadata) and class-9 probe (no stale premise survives) — and the remaining
defects (V3-D1..D7) are minor or informational, none outcome-determining,
none authority-converting, each durably surfaced here for the W1 manager and
HELP_HUMAN fan-in.

**VERDICT: COMMIT-SAFE**

Precisely: no acceptance predicate is unsatisfiable; every v3 premise matches
the live tree, the preserved executor evidence, and the accepted record; no
fast-reject class is hit; the DEL-08-04 vocabulary-extension need is routed,
not resolved; and every owner and HELP_HUMAN gate (PDU-035, DEC-046,
thresholds, lifecycle, sweep-before-push, fan-in, merge, receipt) remains
intact downstream of this verdict.

No release-readiness, acceptance, review-disposition, or reliance conclusion
is expressed or implied by any statement in this return; all verdicts are
bounded to the refutation task defined by the parent brief.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
