# T4 Fresh-Context Adversarial Brief Verification — Round v2 — Sub-Span Wind Exposure

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W2 / T4
**Verifier:** fresh-context adversarial brief verifier, round v2 (bounded Agent 2,
read-mostly; no prior context; NOT the v1 verifier; no v1 conclusion inherited)
**Object:** the AMENDED (v2) candidate brief
`execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_T4_PKG05_SUBSPAN_WIND.md`
(`CB-2026-07-20-T4-PKG05-SUBSPAN-WIND-001`) and the amended rationale
`instances/W2/T4/CURRENT_CANDIDATE_RATIONALE.md` (claims C1–C16).
**Verified against:** branch `claude/piping-r14-pkg05-loads`, HEAD
`581a15b1c718fd444870f13e75fc7cd974518670` (the PR #292 merge commit —
post-wave-1 main with the merged R14-W1 PKG-04 tranches and Receipt-61,
commit `03d0188cd` an ancestor of HEAD). Worktree clean apart from the
untracked candidate brief and `instances/W2/**` (the brief's declared lawful
pre-existing state). Every check below is my own live-tree read, grep, git
query, or read-only validator run; no cargo build/test was run (offline
cargo evidence is cited from the committed W1 check records where noted); no
project file was modified other than this return artifact.

VERDICT: COMMIT-SAFE

The v2 amendment cures the v1 BLOCK: the amended premise set
{v2 §3.6, v2 §5 item 5, live `tests/test_model_schema.py`} is jointly
satisfiable, the specified single assertion replacement is equal-or-stronger
in the precise sense C16 states, the v1 BLOCK is preserved and truthfully
characterized in all three amendment records, and every claim C1–C16
survived my independent refutation attempts. `EffectStatus` progression
remains the W2 manager's act under the R14 campaign-plan execution rules;
this artifact performs no adoption, lifecycle, release, or acceptance act.

## 1. Claim-by-Claim Table (C1–C16)

| Claim | Verdict | Evidence (my own live-tree checks) |
|---|---|---|
| C1 | CONFIRMED | DEL-05-01 `_STATUS.md`: `Current State: IN_PROGRESS`, `Last Updated: 2026-07-12`; `## Remaining` holds exactly one item, byte-matching the brief §"Selected work item" quotation |
| C2 | CONFIRMED | `_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-PMM-P3-OCCLOADGEN-001.md` lines 114–115 carry the quoted boundary verbatim ("…deliberately not invented here"); `SOFTWARE_DECOMP.md` line 650 `DEC-068` item 2 rules "user-entered wind pressure/shape parameters project onto exposed diameter per user-marked spans" — no whole-span restriction in the ruling text |
| C3 | CONFIRMED | `core/loads/primitive_loads/src/lib.rs`: `ElementExposedDiameter` (1730–1733) has exactly `element_index` + `exposed_diameter`; `generate_wind_equivalent_static_loads` (1859–1948) emits only whole-element `PrimitiveLoad::uniform_element_load` with id `{case_ref}:generated:wind:{axis}:element-{i}` (1934–1937); repo grep finds no bare `PrimitiveLoad {` literal outside the crate (all external hits are the distinct product_physics `PreviewPrimitiveLoad` type; construction elsewhere is via the five in-crate helpers, 888–950); `prepare_lumped_nodal_loads` lumps `half_total = magnitude·span·0.5` to `node_i` and `node_j` (2045–2066) |
| C4 | CONFIRMED | `core/product_physics/src/lib.rs`: `WindGenerationInput` (442–456) has exactly `pressure`, `shape_factor`, `direction`, `exposed_pipe_refs`, all `#[serde(default)]`, no `deny_unknown_fields`; `append_equivalent_static_generated_loads` (4815–5015) aggregates missing wind inputs into one `EQUIVALENT_STATIC_INPUT_MISSING` (4925–4937), blocks unknown pipe refs via `EQUIVALENT_STATIC_INPUT_INVALID` (4964–4978), computes exposed diameter `outside_diameter + 2·insulation` (4980–4987), maps pipe index → element index one-to-one (4956–4961); `add_uniform_element_loads` (5472–5553): straight `share = w·L/2` to both end nodes (5549–5551), curved-bend `consistent_uniform_nodal_loads` with blocking `LOAD_INPUT_INVALID` fallback and no silent drop (5494–5536); DEC-018 normalization of `wind.pressure` (Pressure) and `wind.shape_factor` (`normalize_dimensionless_quantity`) at 4413–4437 |
| C5 | CONFIRMED | `core/product_physics/src/validation.rs`: at-least-one-load check tests only `equivalent_static.is_none()` (line 49); provenance required at block level (`expect_public_preview_provenance("equivalent-static-generation", …)`, 187–194); no wind sub-field enumerated anywhere in the file |
| C6 | CONFIRMED | `schemas/model.schema.yaml`: `WindEquivalentStaticInput` (629–656) requires exactly the four named keys, `additionalProperties: false`, `exposed_element_refs` `minItems: 1`; `$defs/ElementLoadSpan` (943–957) = `{start_fraction, end_fraction}` over `$defs/FractionQuantity` (925–942), consumed by `ElementUniformDistributedForceLoadRecord.span` (780); every currently-valid document carries `exposed_element_refs` (it is currently required), so each satisfies that `anyOf` arm and remains valid under the relaxation |
| C7 | CONFIRMED | Independent re-derivation in §4 below: exact elementary statics; unique two-end-force reduction; exact 50/50 reduction at `(0,1)`; disjoint-extent superposition exact by linearity |
| C8 | CONFIRMED | `grep -rn equivalent_static validation/witness/` exits 1 (no match — this also covers the `equivalent_static_generation` spelling as a superstring); the five `del1005_payload_binding_*` input/generated pairs and the three `tp_runner_015_final_cli_*` witnesses are present exactly as listed |
| C9 | CONFIRMED | `validation/benchmarks/mechanics/README.md` records the DEC-026 analytic-class `1.0e-9` relative tier (line 71) with thresholds/tolerances/CI policy `TBD` (lines 14, 16, 82); suite embeds BOTH READMEs via `include_str!` (lib.rs 54–55), enforces TBD markers and forbidden-reliance absence in both (596–600), and per-fixture inventory containment in both (6131, 6143–6145); `MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC` listed in the benchmark README (line 47) and hand-calc README (line 67); count assertion `assert_eq!(fixtures.len(), 23)` at lib.rs 5958 is the conventional bump point (W1 T2/T3 fixture-addition precedent at this HEAD) |
| C10 | CONFIRMED | `Dependencies.csv`: `DAG-002-E0130..E0134` all `EXECUTION`/`UPSTREAM`/`CONSTRAINT`/`SATISFIED`; the sole `PENDING` execution row `TP-DAG-004-DEL-05-01-E001` is `DOWNSTREAM`/`INTERFACE` toward DEL-05-02 (boundary-preservation obligation, honored by not touching load-case algebra); `execution/_DAG/_LATEST.md` → `DAG-007`, `approved_active_graph_authority` |
| C11 (v2) | CONFIRMED | Fence↔task coverage complete both directions (§3.5 below); forced literal-construction sites all in-fence (`ElementExposedDiameter` literals: in-crate 3852/3856/3890/3904 = item 2, product_physics 4984 = item 3, benchmarks 6022/6056 = item 7; `PrimitiveLoad` and `ElementUniformLoadContribution` literals in-crate only, 2332); `core/solver/diagnostics` needs no edit (§3.4(b)); with the v2 single named-replacement carve-out the §3.6/§5-item-5 constraints are jointly satisfiable on the live tree (§2 below) |
| C12 | CONFIRMED | The four W1 branch-level checks at this merged HEAD are recorded PASS (`instances/W1/CHECK_{piping-pytest,evidence-sweep,harness-pytest,harness-self-check}.json`, each `"status": "PASS"` — parsed here); `software-workflow.json` registers exactly those four checks; `tests/test_headless_runner_contract.py` present; I re-ran both read-only validators from `REPO_ROOT`: claims-language `VALID` (262 files), path-anchors `PASS` (668 surfaces) |
| C13 | CONFIRMED | Brief §10 carries the union of the D-52 §4.5 form (packet lines 192–207) and D-54 §3.3 fields (packet lines 124–137), every field present; `OwnerCaseSelection: NONE`; `EffectStatus: HELD`; adoption stated as the owner's conditional act; governance commit `f14fa77518a06f112ae72a8fcce4de0fab958d47` exists and `git merge-base --is-ancestor` confirms it as an ancestor of HEAD; `DEC-085` and `DEC-087` codified at `SOFTWARE_DECOMP.md` §12 lines 667 and 669; both decision packets present in `_DECISIONS/` |
| C14 | CONFIRMED | `ORCHESTRATION_PLAN.md` assigns W2 = PKG-05 manager for exactly T4 + the DEL-05-04 condition verification (lines 22–24), serialized author→verify→execute→verify chains, per-tranche commits, one PR per wave, HELP_HUMAN merge/receipt (35–48); the wave-level single-sweep refinement of the plan's per-tranche-sweep sentence is durably recorded in the W1 T1 brief §6 (lines 461–462, 525–534) and the W1 `RETURN.md` "DEC-025 treatment note for fan-in" (45–51), and that W1 wave was merged at PR #292 = this HEAD; the brief attributes the W2 application to the controlling HELP_HUMAN W2 dispatch — a live-session fact this verifier cannot refute from disk, cited with calibrated attribution |
| C15 | CONFIRMED | `_REGISTER.md` line 79: D-45 `AWAITING_RULING`, packet `D-45_temperature_indexed_shear_modulus.md` present; `instances/W2/T5/CONDITION_VERIFICATION.md` records `CANNOT_ADVANCE — CONDITION_NOT_MET` for the DEL-05-04 PDU-037 conditional row at this HEAD |
| C16 (v2) | CONFIRMED | See §2.2: the current assertion set (lines 450–458) verifies exactly four facts; the specified replacement preserves three of them verbatim ((i) three-key subset, (iii) `exposed_element_refs` `minItems == 1`, (iv) `additionalProperties` False), drops only the one fact the authorized schema change deliberately falsifies (`exposed_element_refs` membership in `required`), and adds verification the current suite cannot express ((ii) exact `anyOf` marking rule; (iii) `exposed_spans` `minItems == 1` + `ElementLoadSpan`-based item shape). Total verification strength against the amended target schema is preserved-or-increased |

## 2. Does the v2 Amendment Cure the v1 BLOCK? (refutation target 2)

### 2.1 Joint satisfiability of {v2 §3.6, v2 §5 item 5, live test file}

I independently re-established the v1 defect from the live tree before
assessing the cure: `tests/test_model_schema.py` lines 450–458 bind
`wind = defs["WindEquivalentStaticInput"]` and assert (a)
`additionalProperties is False` (451), (b)
`{"pressure","shape_factor","direction","exposed_element_refs"} <=
set(wind["required"])` (452–457), (c)
`wind["properties"]["exposed_element_refs"]["minItems"] == 1` (458). Any
implementation of the §3.6 `anyOf` relaxation removes
`exposed_element_refs` from `required`, so assertion (b) fails and must be
rewritten — the v1 "additive only / no assertion weakened" constraint was
genuinely unsatisfiable. The v2 text no longer asserts that constraint:
§3.6 (v2) authorizes exactly ONE named, non-additive replacement of that
assertion, specified by four mandatory sub-assertions (i)–(iv), and §5
item 5 mirrors the carve-out ("additive only, EXCEPT the single named wind
required-set assertion replacement … equal-or-stronger; never weaken any
other existing check"). Satisfiability of the amended set:

- Repo-wide, `exposed_element_refs` appears in code/test surfaces at
  exactly three places: `tests/test_model_schema.py` 456/458,
  `tests/test_physical_to_analytical_transform.py` 557, and the schema
  itself. The transform-test occurrence is a payload literal (a valid
  document that stays valid under `anyOf`), not an assertion about
  `required`; the transform carries load cases wholesale
  (`core/model_transform/physical_to_analytical/contract.py`
  `_copy_load_cases`, `load_cases.append(deepcopy(record))` at 517), so
  additive round-trip coverage needs no transform edit and
  `core/model_transform/**` is correctly outside the fence.
- No defs-wide enumeration assertion exists that the schema edit could
  trip: line 225 is a subset check (`REQUIRED_DEFS <= set(defs)`); the
  only `==`-style property enumerations bind `LoadCase.allOf[0].not.required`
  (431) and seismic `g_factors` (449) — both untouched by a wind edit.
  §3.6 adds no new `$defs` (the `exposed_spans` items reference
  `$defs/ElementReference` and `$defs/ElementLoadSpan`, both existing), so
  "no other `$defs` change" is satisfiable.
- Therefore a faithful executor can: replace the single named assertion
  with the (i)–(iv) set, keep assertions (a) and (c) verified (they are
  literally part of (iv) and (iii)), leave every other assertion in both
  files untouched, and add only additive coverage. The amended predicate
  set is jointly satisfiable on the live tree. CURE CONFIRMED.

One precision note, recorded as non-blocking: the brief cites the target
as "the wind required-set subset assertion (currently lines 450–458)".
The subset assertion itself is lines 452–457; 450–458 brackets the whole
wind assertion block (binding, `additionalProperties`, subset, `minItems`).
The target is uniquely named in prose ("the wind required-set subset
assertion" — there is exactly one such assertion in the file), and the
(i)–(iv) replacement spec subsumes every fact in the bracketed block, so
both the minimal reading (replace only 452–457) and the block reading
land on an equal-or-stronger surviving assertion set. No material
ambiguity; not a defect.

### 2.2 Equal-or-stronger (C16)

Facts verified by the current block: F1 `additionalProperties` is False;
F2 the three parameter keys are required; F3 `exposed_element_refs` is
required; F4 `exposed_element_refs` has `minItems == 1`. The replacement
verifies F1 ((iv)), F2 ((i)), F4 ((iii)) and adds F5 (exact `anyOf`
at-least-one-marking-form shape) and F6 (`exposed_spans` `minItems == 1`
and `ElementLoadSpan`-based item shape). Only F3 is dropped — and F3 is
precisely the fact the authorized schema change makes deliberately false;
retaining it would contradict §3.6 and defeat the selected capability (a
spans-only document would remain invalid). Relative to the amended target
schema, verification strength is preserved-or-increased; the rationale's
C16 states this honestly (it does not claim strength against the old
schema). CONFIRMED.

### 2.3 v1 BLOCK preserved unsoftened and truthfully characterized

- `instances/W2/T4/VERIFY_BRIEF.md` is present and carries
  `VERDICT: BLOCK` (twice), the refuted premise, and a §5 cure
  prescription. The v2 §3.6 spec implements that prescription's items
  (i)–(iv) and fence mirror exactly.
- Brief header amendment record, §10 `IndependentVerifier` line, and the
  rationale amendment note each state the v1 BLOCK, its location, and the
  refuted premise accurately. The header's "All other v1 claims …
  confirmed" is consistent with the v1 table: v1's sole REFUTED row (C11)
  failed only on the named premise, with its path-coverage component
  confirmed. The §10 line records `PENDING (v2)` with the BLOCK history —
  a truthful intermediate state; `EffectStatus: HELD` keeps the posture
  fail-closed pending this round. The phrase "cured by the v2 amendment"
  is a forward claim, but it matches the v1 verifier's own prescribed cure
  and is exactly what this round independently confirms; no softening.
- Warrant limit, recorded: `instances/W2/**` is untracked, so git history
  cannot attest byte-preservation of the v1 artifact since it was written.
  The preservation finding rests on the document's internal coherence (a
  complete, self-consistent BLOCK record whose prescribed cure the v2
  amendment implements verbatim) and the absence of any indication of
  alteration. Within that warrant: preserved, unsoftened. CONFIRMED.

## 3. Cross-Cutting Findings (refutation targets 3–7)

### 3.1 Ten-class fast-reject re-screen (D-52 §4.1 / D-54 §3.1), my own, item by item

1. Irreducible owner preference / owner-only choice: NOT HIT. The
   Remaining row names the work verbatim; under D-54 §3.2 the surviving
   design multiplicity (extent representation, lumping tier, bend
   handling) is bounded reasoned selection, and no engineering-method fork
   is opened — the lever rule is the exact evaluation of the tier the
   preview path already applies (verified against both live seams, §4).
2. Professional/safety/legal accountability: NOT HIT. Preview mechanics
   from user-entered inputs on invented fixtures; claim posture and
   `TBD` reliance boundaries unchanged (README markers verified live).
3. Conflict ruling not determined by the authority chain: NOT HIT.
   DEC-068 item 2's own language ("per user-marked spans", line 650) rules
   the class; the whole-span limit is the landing tranche's recorded
   deliberate residual; no accepted artifact defends whole-span-forever.
4. Scope/boundary change, new normative content, new acceptance criteria
   — the class the v2 carve-out most plausibly touches, screened hardest:
   NOT HIT. The sub-span capability itself is recorded deliverable scope
   (ungated Remaining row + Receipt-6/DEC-068 lineage). The non-additive
   assertion replacement is truth-maintenance of a verification surface
   forced by the authorized schema change: it creates no criterion,
   threshold, or normative content; its protective content is preserved
   (F1/F2/F4 retained, F5/F6 added, §2.2); every other check in both
   files is fenced additive-only. A verification mirror that must track
   an authorized schema edit is not a guard-weakening or scope act when
   strength is preserved-or-increased and the edit is singly named,
   specified, and independently verified — which is this case exactly.
5. Lifecycle/stage/release/acceptance/evidence-posture act: NOT HIT.
   `IN_PROGRESS` unchanged; DEC-026 tier reused as recorded; the sweep
   treatment follows the recorded W1 refinement (durable precedent, not a
   posture change).
6. External/procurement/publication: NOT HIT. Offline; no
   push/PR/merge/receipt inside the tranche.
7. Merge authority / destructive action: NOT HIT. Wave-branch commits by
   the manager only after independent implementation verification; frozen
   witnesses protected by byte-identity predicate; replacing a live test
   assertion does not touch ruled history (the prior text remains in git
   history; tests are living verification code, not ruled records).
8. Protected/private data: NOT HIT. Code-book wind content expressly
   excluded (§9); elementary statics and invented geometry only.
9. Evidence unavailable / stale basis / claim beyond warrant: NOT HIT.
   Every outcome-determining premise was re-verified live at HEAD in this
   round (§1 table); the one over-warrant claim v1 found was removed by
   the amendment; the derive-first gate (§4.2) and del1005 byte-identity
   predicate keep future claims within warrant; live-session citations
   (W2 dispatch) are attributed as such, not asserted as tree facts.
10. Protected domain-engine paths / prover / higher-order boundaries:
    NOT HIT. `_DomainEngines/**`, prover, and app-dev surfaces untouched
    and outside the fence.

Screen result: PASS — the v2 carve-out does not cross any class.

### 3.2 Physics (target 4)

PASS — independent derivation in §4; both live application seams verified
as the `c = 1/2` special case; tier consistency confirmed (no fixed-end
moments anywhere in the existing generated-load path, none introduced).

### 3.3 Predicate satisfiability sweep (target 5)

- (a) Additive-field construction sites: `ElementExposedDiameter` struct
  literals exist at exactly five sites — primitive_loads in-crate tests
  3852/3856/3890/3904 (fence item 2), product_physics 4984 (item 3),
  benchmarks lib.rs 6022/6056 (item 7, expressly authorized as "minimal
  literal-construction updates"). `PrimitiveLoad` literals: in-crate only
  (five helper constructors; all external `PrimitiveLoad {`-pattern hits
  are the distinct `PreviewPrimitiveLoad` type).
  `ElementUniformLoadContribution` literal: in-crate 2332 only. Every
  file that MUST change is inside the fence. `WindGenerationInput` is a
  plain serde `Deserialize` with per-field defaults and no
  `deny_unknown_fields`, so the additive `exposed_spans` field is
  deserialization-compatible. PASS.
- (b) No-new-FindingCode: `core/solver/diagnostics`
  `load_finding_diagnostic_code` (723–745) is an exhaustive,
  wildcard-free match over `LoadFindingCode`; §3.2 routes invalid extents
  through the existing `InvalidGenerationInput` family, so no variant and
  no diagnostics edit is needed; the §6 read-only regression run of that
  crate is the correct guard. PASS.
- (c) Witness inputs: no `equivalent_static` (nor, a fortiori,
  `equivalent_static_generation`) anywhere under `validation/witness/`;
  the del1005 five-case byte-identity predicate is satisfiable and no
  witness-facing solve change is expected. PASS.
- (d) `validation.rs`: both relevant checks are block-level only (line
  49; 187–194); the §4.2 strict-necessity carve-out is correctly expected
  to be unused. PASS.
- (e) Benchmark suite: fixture-count assertion (5958) is the conventional
  bump point; both README mirrors are enforced by `include_str!` marker
  and per-fixture containment checks (596–600, 6131, 6143–6145), so ONE
  additive inventory line per mirror satisfies the enforcement without
  touching claim-posture/`TBD`/note text. The §3.8 recorded-fallback path
  is also lawful (fmt/test still run per §6). PASS.
- (f) Schema/tests: per §2.1, jointly satisfiable with the v2 carve-out;
  every currently-valid document remains valid; `del1005` predicate per
  (c). PASS.

### 3.4 Attribution and effect discipline (target 6)

PASS. §10 uses the D-52 §4.5 form extended by the D-54 §3.3 fields with
no field missing; `OwnerStandingApproval` (DEC-085/D-52 §2 as refined by
DEC-087/D-54 §1) is kept distinct from `AgentClassification:
CLASSIFY_ELIGIBLE` and `AgentJudgment: SELECT_AND_ADVANCE`;
`OwnerCaseSelection: NONE`; `EffectStatus: HELD`; adoption is stated as
the owner's conditional act under the standing rule. Governance commit
`f14fa7751…` exists and is an ancestor of HEAD (verified by
`git merge-base --is-ancestor`). `DEC-085`/`DEC-087` §12 rows live at
SOFTWARE_DECOMP lines 667/669; both packets present. Nothing in the brief
performs or presupposes an owner-only act; no execution writes are
authorized while held.

### 3.5 Fence-vs-task coverage, both directions (target 7)

PASS. Forward: every §4/§6 durable write maps to a §5 line — hand-calc
witness (item 6), primitive_loads (2), product_physics lib +
conditional validation.rs (3), schema (4), the two Python test files with
the single named carve-out (5), benchmark suite + two one-line README
mirrors + count bump + forced literal updates (7), DEL-05-01 state triple
(8), `instances/W2/T4/**` including `CHANGE_SCOPE_CONTAINMENT.json` (9).
Reverse: no §5 line authorizes more than the tasks need — item 1 is the
brief's own governed status record, Cargo manifest/lock lines are
strictly-required-conditional, item 10 is an explicit absence (single
wave-level DEC-025 sweep at W2 closeout, matching the durable W1
refinement records). The exclusion list (§5 tail, §9) closes every
adjacent surface (solver crates, user_loads, runner, model_operations,
witnesses, docs, other schemas, registers, governance).

### 3.6 Campaign-plan conformance and claims language (target 7 cont.)

PASS. The W2 chain (manager-authored brief → this fresh-context v2 verify
→ executor → implementation verifier → manager commit) matches
`ORCHESTRATION_PLAN.md` execution rules; W1-then-W2 serialization is
honored (W1 merged at PR #292 = this HEAD; W1 T1–T3 instance directories
carry the full per-tranche artifact chains as precedent; W2 holds T4/T5
only, matching the plan's W2 assignment). Claims-language validator:
`VALID` (262 files); path-anchors: `PASS` (668 surfaces); the amended
brief and rationale use calibrated language throughout — the v1-refuted
over-claim is gone, live-session facts are attributed, and both artifacts
close with the standard claim fence.

## 4. Independent Derivation — Lever-Rule Statics (my own, refutation target 4)

Setup: straight span of length `L` between end nodes i (at `x = 0`) and j
(at `x = L`); uniform transverse intensity `w` (force/length) over
`x ∈ [aL, bL]`, `0 ≤ a < b ≤ 1`, along one global axis.

- Resultant: `W = ∫_{aL}^{bL} w dx = w·(b−a)·L`.
- Centroid: `x̄ = (aL + bL)/2 = c·L`, `c = (a+b)/2`.
- Static equivalence of end forces `R_i`, `R_j` requires force and moment
  preservation. Force: `R_i + R_j = W`. Moment about node i:
  `R_j·L = W·x̄ = W·c·L ⇒ R_j = W·c`, hence `R_i = W·(1−c)`. Cross-check
  about node j: `R_i·L = W·(1−c)·L = W·(L − x̄)` ✓. Two equations, two
  unknowns — the two-end-force reduction is unique and exact; matching
  the brief §3.3 closed forms exactly.
- Whole-span reduction: `(a,b) = (0,1) ⇒ W = w·L, c = 1/2 ⇒
  R_i = R_j = w·L/2` — byte-for-byte the live
  `prepare_lumped_nodal_loads` half-total (`magnitude·span·0.5`,
  primitive_loads lib.rs 2045) and the live `add_uniform_element_loads`
  straight-span `share = w·L/2` (product_physics lib.rs 5549). Both
  existing seams are the `c = 1/2` special case; the claimed whole-span
  invariance is structurally consistent.
- Superposition: for disjoint extents on one span, resultants and
  first moments are additive integrals and the assembled force vector is
  linear in applied loads, so per-extent exact shares sum exactly.
- Attempted refutations that fail: (i) intensity-scaling `w·(b−a)` over
  the whole span preserves `W` but places the centroid at `L/2 ≠ c·L`
  for every asymmetric extent — rejected alternative 1 is correctly
  rejected; (ii) "the treatment under-represents bending" — true but
  tier-consistent: the existing whole-span path applies pure force
  lumping with no `wL²/12` fixed-end moments, and the brief stays on
  that tier explicitly, with the work-equivalent upgrade and partial-arc
  bend integration excluded and preserved as future lawful selections;
  (iii) a partial extent on a curved-bend macro span has no exact
  treatment in the live arc machinery (it integrates full uniform
  intensity only) — the brief fails closed there rather than
  approximating, which is the only truth-preserving option at this tier.

## 5. Required Cure

None — no refuted premise remains. (The single non-blocking precision
note in §2.1 — the 450–458 line-range bracketing versus the 452–457
subset assertion — requires no amendment because the replacement target
is uniquely named and the specified replacement subsumes the bracketed
block's facts either way.)

All findings above are stated to the warrant of the reads, greps, git
queries, JSON parses, and the two read-only validators executed at HEAD
`581a15b1c`; no cargo build/test was run by this verifier (offline cargo
evidence cited from the committed W1 check records); the byte-preservation
finding for the untracked v1 artifact carries the §2.3 warrant limit.
This artifact is verification evidence only; it performs no adoption,
lifecycle, release, or acceptance act, and effect progression remains the
W2 manager's act under the campaign plan.

VERDICT: COMMIT-SAFE

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
