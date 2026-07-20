# T4 Fresh-Context Adversarial Brief Verification — Sub-Span Wind Exposure

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W2 / T4
**Verifier:** fresh-context adversarial brief verifier (bounded Agent 2, read-mostly; no prior context)
**Object:** `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_T4_PKG05_SUBSPAN_WIND.md`
(`CB-2026-07-20-T4-PKG05-SUBSPAN-WIND-001`) and
`instances/W2/T4/CURRENT_CANDIDATE_RATIONALE.md`
**Verified against:** branch `claude/piping-r14-pkg05-loads`, HEAD
`581a15b1c718fd444870f13e75fc7cd974518670` (worktree clean apart from the
untracked candidate brief and `instances/W2/**`, which is the brief's own
declared lawful pre-existing state). All checks were live-tree reads, greps,
and two read-only repo validators; no project file was modified other than
this return artifact.

VERDICT: BLOCK

One precise premise set is jointly unsatisfiable on the live tree; every
other enumerated claim survived refutation. The defect is narrow and curable
by a bounded brief amendment (§5 below). Per D-52 §4.2 gate 8, `EffectStatus`
must remain `HELD`.

## 1. The Refuted Premise (basis for BLOCK)

Brief §3.6 requires, conjointly:

- (P1) `WindEquivalentStaticInput`'s required set "is relaxed to require at
  least one marking form (via `anyOf` over `exposed_element_refs` /
  `exposed_spans`)"; and
- (P2) `tests/test_model_schema.py` … "gain additive assertions … no
  existing assertion is weakened", reinforced by fence §5 item 5:
  "(additive only; never weaken existing checks)".

Live tree fact (`tests/test_model_schema.py`, lines 450–458): the suite
asserts

```python
wind = defs["WindEquivalentStaticInput"]
assert wind["additionalProperties"] is False
assert {
    "pressure",
    "shape_factor",
    "direction",
    "exposed_element_refs",
} <= set(wind["required"])
assert wind["properties"]["exposed_element_refs"]["minItems"] == 1
```

Any implementation of P1 removes `exposed_element_refs` from
`wind["required"]` (that is what the `anyOf` relaxation means), so the
existing subset assertion fails and MUST be rewritten. Rewriting an existing
assertion is not "additive only", and the rewritten assertion individually
asserts strictly less than the current one (exposed_element_refs no longer
always-required). The alternative encodings do not escape: keeping all four
keys in `required` defeats the relaxation (a spans-only document stays
invalid, contradicting P1 and the §3.5 "at least one marking form" rule
mirrored at the canonical tier), and moving the variance to the
`EquivalentStaticGeneration.wind` use-site violates §3.6's own "no other
`$defs` change". Therefore {P1, P2, live assertion} is unsatisfiable: a
faithful executor either violates the fence condition or fails the schema
predicate, and under brief §7 must stop. This is exactly the class of defect
this verification exists to catch before dispatch.

Scope of the defect: it is confined to `tests/test_model_schema.py`. The
companion claim that every currently-valid document remains valid under the
`anyOf` relaxation is TRUE (all currently-valid documents carry
`exposed_element_refs`, satisfying the `anyOf` arm), and
`tests/test_physical_to_analytical_transform.py` is genuinely additive-safe:
its wind payload (line 553–557) uses `exposed_element_refs` and the
transform (`core/model_transform/physical_to_analytical/contract.py`,
`_copy_load_cases`, line 517: `load_cases.append(deepcopy(record))`) carries
load cases wholesale, so `exposed_spans` round-trips with no transform edit
and no fence gap. No defs-wide meta-assertion exists that the edit could
trip (only the `LoadRecord` `oneOf` list at line 384, untouched).

## 2. Claim-by-Claim Table (rationale §4, C1–C15)

| Claim | Verdict | Evidence (live tree) |
|---|---|---|
| C1 | CONFIRMED | DEL-05-01 `_STATUS.md`: `Current State: IN_PROGRESS`; `## Remaining` has exactly one item, byte-matching the brief's quoted selected item |
| C2 | CONFIRMED | `_run_records/WORKING_ITEMS_RUN_2026-07-10_TP-PMM-P3-OCCLOADGEN-001.md` lines 114–115 carry the quoted boundary verbatim; `SOFTWARE_DECOMP.md` §12 row `DEC-068` item 2 (line 650) rules "user-entered wind pressure/shape parameters project onto exposed diameter per user-marked spans" with no whole-span restriction |
| C3 | CONFIRMED | `core/loads/primitive_loads/src/lib.rs`: `ElementExposedDiameter` (1730–1733) has exactly `element_index` + `exposed_diameter`; `generate_wind_equivalent_static_loads` (1859–1948) emits only whole-element `uniform_element_load` with id `{case_ref}:generated:wind:{axis}:element-{i}` (1934–1937); repo-wide grep finds no bare `PrimitiveLoad {` literal outside the crate (only helper calls; `PreviewPrimitiveLoad` is a distinct product_physics type); `prepare_lumped_nodal_loads` lumps `magnitude·span·0.5` to node_i and node_j (2045–2066) |
| C4 | CONFIRMED | `core/product_physics/src/lib.rs`: `WindGenerationInput` (443–456) has exactly the four named fields; `append_equivalent_static_generated_loads` (4815–5015) aggregates missing inputs (`EQUIVALENT_STATIC_INPUT_MISSING`), blocks unknown refs (`EQUIVALENT_STATIC_INPUT_INVALID`), computes `outside_diameter + 2·insulation` (4986), maps pipe index = element index (4956–4960); `add_uniform_element_loads` (5472–5553): straight `share = w·L/2` (5549), curved-bend `consistent_uniform_nodal_loads` with blocking `LOAD_INPUT_INVALID` fallback (5494–5536); DEC-018 normalization of `wind.pressure`/`wind.shape_factor` at 4413–4437 |
| C5 | CONFIRMED | `validation.rs` line 49: at-least-one-load check tests only `equivalent_static.is_none()`; lines 187–194: provenance required at block level (`"equivalent-static-generation"`); no wind sub-field enumerated anywhere |
| C6 | CONFIRMED | `schemas/model.schema.yaml`: `WindEquivalentStaticInput` (629–656) requires exactly the four keys, `additionalProperties: false`, `exposed_element_refs` minItems 1; `$defs/ElementLoadSpan` (943–957) = `{start_fraction, end_fraction}` over `FractionQuantity` (925), consumed by `ElementUniformDistributedForceLoadRecord.span` (780). Every currently-valid document remains valid under the `anyOf` relaxation. (The test-edit consequence of that relaxation is the §1 BLOCK, which C6 does not itself claim.) |
| C7 | CONFIRMED | Independent derivation in §4 below: exact statics; exact 50/50 reduction at `(0,1)`; disjoint-extent superposition exact by linearity |
| C8 | CONFIRMED | `grep -rn equivalent_static validation/witness/` returns nothing (exit 1); the five `del1005_payload_binding_*` input/generated pairs and three `tp_runner_015_*` witnesses are present as listed |
| C9 | CONFIRMED | `validation/benchmarks/mechanics/README.md` records the DEC-026 analytic-class `1.0e-9` relative tier with thresholds/tolerances `TBD`; suite readiness test embeds BOTH READMEs (`include_str!`, lib.rs 54–55) and checks fixture markers in each (597–599) plus per-fixture README containment (6131); fixture `MECH-TP-PMM-P3-OCCLOADGEN-EQUIVALENT-STATIC` listed (README line 47); count assertion `assert_eq!(fixtures.len(), 23)` at lib.rs 5958 is the conventional bump point |
| C10 | CONFIRMED | `Dependencies.csv`: `DAG-002-E0130..E0134` all `EXECUTION`/`UPSTREAM`/`CONSTRAINT`/`SATISFIED`; sole `PENDING` row `TP-DAG-004-DEL-05-01-E001` is `DOWNSTREAM`/`INTERFACE` toward DEL-05-02; legacy rows `RETIRED`; `execution/_DAG/_LATEST.md` → `DAG-007`, `approved_active_graph_authority` |
| C11 | REFUTED | Fence path coverage itself is complete (all §4 write targets, including the forced `ElementExposedDiameter` literal updates at product_physics:4984 and benchmarks lib.rs:6022/6056, fall inside §5; no fence line is materially broader than the tasks; `core/solver/diagnostics` needs no edit — see check 4(d)). But the fence's item-5 condition ("additive only; never weaken") is unsatisfiable together with §3.6 on the live tree (§1 above), so the §4 schema/test task is not executable as fenced |
| C12 | CONFIRMED | W1 wave at this merged head ran the identical surfaces clean offline: `CHECK_piping-pytest.json` et al. PASS; W1 T3 `EXECUTE_RETURN.md` records `cargo test --offline` PASS for product_physics (86), benchmarks (36), runner/headless; this verification re-ran `validate_claims_language.py` (262 files VALID) and `validate_path_anchors.py` (667 surfaces PASS) from `REPO_ROOT` |
| C13 | CONFIRMED | Brief §10 uses the D-52 §4.5 form extended by the D-54 §3.3 fields; `OwnerCaseSelection: NONE`; `EffectStatus: HELD`; adoption stated as the owner's conditional act; commit `f14fa77518a06f112ae72a8fcce4de0fab958d47` exists, is an ancestor of HEAD, and is the governance commit adding the four-lens standing approval |
| C14 | CONFIRMED | `ORCHESTRATION_PLAN.md` assigns W2 = PKG-05 manager, serialized chains, per-tranche commits, one PR per wave with HELP_HUMAN merging; the wave-level single-sweep refinement is recorded in W1 T1 brief §6 (lines 461–466, 525–534) and W1 `RETURN.md` ("DEC-025 treatment note for fan-in"), and the W1 wave was merged at PR #292 (this HEAD), so the recorded precedent stands; the brief attributes the W2 application to the controlling HELP_HUMAN W2 dispatch, a live-session fact this verifier cannot refute and which the brief cites with calibrated attribution |
| C15 | CONFIRMED | `_REGISTER.md` row D-45 `AWAITING_RULING` with packet `D-45_temperature_indexed_shear_modulus.md`; `instances/W2/T5/CONDITION_VERIFICATION.md` records `CANNOT_ADVANCE — CONDITION_NOT_MET` for the DEL-05-04 conditional row at this HEAD |

## 3. Cross-Cutting Checks

1. **Attribution (D-52/DEC-085, D-54/DEC-087).** PASS. §10 keeps
   `OwnerStandingApproval` (DEC-085/D-52 §2 as refined by DEC-087/D-54 §1)
   distinct from `AgentClassification: CLASSIFY_ELIGIBLE` and
   `AgentJudgment: SELECT_AND_ADVANCE`; `OwnerCaseSelection: NONE`;
   `EffectStatus: HELD`; `AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL`
   SHA-bound to the verified ancestor commit `f14fa775…`. Nothing in the
   brief performs or presupposes an owner-only act: no execution writes are
   authorized while held, no lifecycle/stage/release/acceptance act appears,
   and every D-52 §4.1 / D-54 §3.1 owner class is preserved in §9/§10. The
   D-52 and D-54 packets and their §12 codifications (DEC-085 line 667,
   DEC-087 line 669) are committed at HEAD.
2. **Ten-class screen truthfulness, especially class 4 (scope).** PASS,
   item-by-item against D-52 §4.1 as refined by D-54 §3.1. On class 4:
   the selected item is the sole recorded `## Remaining` row of DEL-05-01
   (no `(gated:)` suffix), placed there 2026-07-10 by the owner-adopted
   consolidation and reconciled 2026-07-12 (PDU-054); the landing tranche's
   run record names sub-span marking as the deliberate residual; DEC-068
   item 2's ruling language is "per user-marked spans" with no whole-span
   restriction. Extending the marking granularity inside that ruled class,
   with fraction vocabulary reused from accepted `$defs/ElementLoadSpan`,
   is recorded-scope work, not a scope/boundary act. Class 9 was probed
   hardest: every "verified live at HEAD `581a15b1c`" premise in the
   rationale was independently re-verified true here (§2 table). No class
   is hit; the screen's PASS is truthful.
3. **Physics.** PASS — independent derivation in §4 confirms brief §3.3
   exactly; consistency with both live seams confirmed (C3/C4 evidence).
4. **Satisfiability on the live tree.**
   - (a) Additive fields: construction-site inventory — `PrimitiveLoad`
     struct literals exist only in-crate (all external construction is via
     the four helpers, verified in product_physics and
     `validation/benchmarks/mechanics/src/lib.rs`);
     `ElementUniformLoadContribution` literals only in-crate (lib.rs 2332);
     `ElementExposedDiameter` literals at product_physics:4984 and
     benchmarks lib.rs:6022/6056 — every file that MUST change is inside
     the §5 fence (items 2, 3, 7). No exhaustive destructuring patterns of
     these structs exist outside the crate; no external consumer parses the
     `generated:wind` id shape. `WindGenerationInput` is plain serde
     `Deserialize` with per-field defaults (no `deny_unknown_fields`), so
     the additive `exposed_spans` field is compatible. PASS.
   - (b) Schema `anyOf` plan vs. live test assertions: FAIL — the §1
     refuted premise (`tests/test_model_schema.py:450–458`).
   - (c) Witness inputs: PASS — no `equivalent_static` anywhere under
     `validation/witness/` (C8).
   - (d) `core/solver/diagnostics`: PASS — `load_finding_diagnostic_code`
     (lib.rs 723–745) is an exhaustive wildcard-free match over
     `FindingCode`; the brief introduces no new variant (§3.2 routes
     invalid extents through existing `InvalidGenerationInput`), so no
     diagnostics edit is needed, and the §6 read-only regression run of
     that crate is the correct guard.
   - (e) `validation.rs`: PASS — checks are block-level only (C5); the
     §4.2 strict-necessity carve-out is correctly expected to be unused.
5. **Fence completeness vs. §4 tasks.** Path coverage complete in both
   directions: every §4/§6 durable write (hand-calc witness, two crate
   sources, validation.rs carve-out, schema, two Python test files,
   benchmark suite + two README mirror lines + count bump, DEL-05-01
   state triple, `instances/W2/T4/**` incl.
   `CHANGE_SCOPE_CONTAINMENT.json`) is a §5 line; no §5 line authorizes
   anything the tasks do not need (Cargo manifest/lock lines are
   strictly-required-conditional). The sole defect is the item-5
   condition, per §1. FAIL only in that respect.
6. **Dependency and gate posture.** PASS — C10/C15 evidence; DAG pointer
   approved `DAG-007`; D-45 `AWAITING_RULING`; T5 record present and
   truthful at this HEAD.
7. **Campaign lawfulness.** PASS — the W2 tranche chain
   (author → this fresh-context verify → execute → verify → manager
   commit) matches `ORCHESTRATION_PLAN.md` execution rules and the W1
   precedent artifacts (per-tranche `VERIFY_BRIEF*`/`EXECUTE_RETURN*`/
   `VERIFY_IMPL`, manager commits, single wave sweep, no push/PR/merge/
   receipt at manager level; W3 is read-only and its early `RETURN.md` is
   plan-conformant concurrency). The brief's §5 item 10 / §6 closing
   paragraph mirror the recorded W1 refinement exactly.
8. **Claims language and warrant.** PASS with one systemic exception
   already counted: the §3.6 sentence "no existing assertion is weakened"
   is unsatisfiable as a predicate (§1) — as a claim about the future run
   it exceeds what the live tree permits. All other "current at HEAD"
   facts in the brief and rationale were re-verified true (HEAD, branch,
   Receipt-61 cursor, W1 product_physics diff disjoint from the wind path
   — zero hits for the wind generation symbols in
   `git diff 96563e8e0...581a15b1c` over that file, GUI-emit whole-span
   field path live at `operation_applier/src/lib.rs:713`, active workplan
   `loop/WORKPLAN_2026-07-18b_piping_loop.md` present). Repo claims
   validator: VALID over 262 files.

## 4. Independent Derivation — Lever-Rule Statics (attempted refutation of C7/§3.3)

Setup: straight span of length `L` between end nodes i and j; uniform
transverse intensity `w` (force/length) acting over the sub-interval
`x ∈ [aL, bL]`, `0 ≤ a < b ≤ 1`, along one global axis.

- Resultant: `W = ∫_{aL}^{bL} w dx = w·(b−a)·L`.
- Centroid: `x̄ = (aL + bL)/2 = c·L` with `c = (a+b)/2`.
- Statically-equivalent end forces `R_i` (at x = 0) and `R_j` (at x = L)
  must preserve force and moment resultants:
  force: `R_i + R_j = W`; moment about node i: `R_j·L = W·x̄ = W·c·L`
  ⇒ `R_j = W·c`, `R_i = W·(1−c)`.
  Moment about node j checks: `R_i·L = W·(1−c)·L = W·(L−x̄)` ✓ — both
  force and moment resultants are preserved exactly about every point, so
  the two-force system is the exact statically-equivalent reduction. This
  is elementary statics with no approximation at this tier (what it omits
  — fixed-end moments — is precisely what the existing tier omits).
- Whole-span reduction: `(a,b) = (0,1)` ⇒ `W = w·L`, `c = 1/2` ⇒
  `R_i = R_j = w·L/2` — exactly the live `prepare_lumped_nodal_loads`
  half-total (`magnitude·span·0.5`, lib.rs 2045) and the live
  `add_uniform_element_loads` straight-span `share = w·L/2` (5549). The
  claimed consistency with the existing whole-span treatment is not
  refutable: both existing seams are the `c = 1/2` special case.
- Superposition: the assembled force vector is linear in applied loads;
  for disjoint extents `[a₁,b₁], [a₂,b₂], …` on one span, each load's
  exact shares add, and the sum preserves the combined resultant and
  centroid moment because both are additive integrals. Exact.
- Refutation attempts that fail: (i) intensity-scaling `w·(b−a)` over the
  whole span preserves `W` but places the centroid at `L/2 ≠ c·L` for any
  asymmetric extent — the rationale's rejected alternative 1 is correctly
  rejected, and the lever rule is the unique two-end-force reduction;
  (ii) claiming the treatment under-represents bending: true but
  tier-consistent — the existing whole-span path already applies pure
  force lumping with no `wL²/12` fixed-end moments, and the brief
  expressly stays on that tier (§3.3 "No fixed-end moment is introduced
  anywhere"), with the work-equivalent upgrade recorded as a rejected
  out-of-scope alternative.

## 5. Required Curing Amendment (for a superseding candidate)

The refuted premise is confined to the §3.6/§5-item-5 test-edit
constraint. A curing amendment must:

1. In §3.6, replace "gain additive assertions/round-trip coverage for the
   new field; no existing assertion is weakened" with an explicit,
   equal-strength update authorization for the ONE existing wind
   required-set assertion in `tests/test_model_schema.py` (currently lines
   450–458): the amended test must assert (i)
   `{"pressure","shape_factor","direction"} <= set(wind["required"])`,
   (ii) the exact `anyOf` shape enforcing at least one of
   `exposed_element_refs` / `exposed_spans`, (iii) `exposed_element_refs`
   retains `minItems == 1` and the new `exposed_spans` carries
   `minItems == 1` and the `ElementLoadSpan`-based item shape, and (iv)
   `additionalProperties` remains `False` — so total verification strength
   is preserved or increased while the single obsolete subset assertion is
   replaced, and record that replacement as a named non-additive edit.
2. Mirror the same carve-out in fence §5 item 5 (e.g., "additive only,
   except the single named wind required-set assertion update in
   `tests/test_model_schema.py`, which must be equal-or-stronger; never
   weaken any other existing check").
3. No other change is required by this verification; all remaining
   predicates, fence lines, screens, attribution, and evidence plans were
   confirmed against the live tree as recorded above.

All findings above are stated to the warrant of the reads, greps, git
queries, and the two read-only validators executed at HEAD `581a15b1c`;
no cargo build/test was run by this verifier (offline evidence was taken
from the committed W1 check records where cited). This artifact is
verification evidence only; it performs no adoption, lifecycle, release,
or acceptance act.

VERDICT: BLOCK

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
