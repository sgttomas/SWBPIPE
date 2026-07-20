# T4 Fresh-Context Adversarial Implementation Verification — Sub-Span Wind Exposure

**Run:** `HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W2 / T4
**Verifier:** fresh-context implementation verifier (bounded Agent 2,
read-mostly, adversarial; no prior context; NOT the brief verifier and NOT
the executor; no executor claim inherited — every result below was
re-established by my own reads, diffs, recomputation, and check re-runs)
**Object:** the UNCOMMITTED working-tree implementation on
`claude/piping-r14-pkg05-loads` at base HEAD
`581a15b1c718fd444870f13e75fc7cd974518670`, verified against the EFFECTIVE
(v2) brief `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_T4_PKG05_SUBSPAN_WIND.md`
(`CB-2026-07-20-T4-PKG05-SUBSPAN-WIND-001`) §3 predicates 1–11, §5 fence
(v2, single authorized non-additive replacement), §6 checks, §7
dispositions, and the chain artifacts under `instances/W2/T4/`
(rationale; `VERIFY_BRIEF.md` v1 BLOCK, preserved; `VERIFY_BRIEF_V2.md`
COMMIT-SAFE; `EXECUTE_RETURN.md` claiming PASS).
**Date:** 2026-07-20
**Offline discipline:** all cargo runs used `CARGO_NET_OFFLINE=true` +
`--offline`; no install, fetch, or network action; the only file written
by this verifier is this artifact.

VERDICT: COMMIT-SAFE

Every §3 predicate was independently confirmed; every §6 check was
independently re-run and passed with outcomes matching the executor's
tally; the working-tree change set is fully contained in the §5 fence;
the hand-calc physics was independently recomputed and is exact; the
deliverable state updates are truthful and claim-calibrated. One
precision note on the executor's pytest tally decomposition is recorded
in §5 below; it resolves to a truthful reading endorsed by the record's
own clarifying clause and does not refute any operative claim.

## 1. Containment (independent re-run and comparison)

`git status --porcelain` and `git diff --stat` from REPO_ROOT show 10
tracked modified paths and 4 untracked entries (the run record, the
`instances/W2/` tree, the candidate brief, the new witness) — nothing
else. HEAD is the base commit `581a15b1c` (no commit, push, branch, or
merge was performed).

My own containment run
(`python3 tools/software_workflow/validate_change_scope.py "$REPO_ROOT"
--base 581a15b1c718fd444870f13e75fc7cd974518670 --allowed <the §5 fence
paths, with the instance root broadened to `instances/W2` to admit the
lawful pre-existing R14 chain state>`) swept ALL 19 changed/untracked
paths — the executor's 14 declared writes plus the 5 lawful pre-existing
artifacts (candidate brief; T4 rationale, `VERIFY_BRIEF.md`,
`VERIFY_BRIEF_V2.md`; T5 `CONDITION_VERIFICATION.md`) — and returned
`status: PASS, violations: []`.

Comparison with the executor's persisted
`instances/W2/T4/CHANGE_SCOPE_CONTAINMENT.json`: the executor used the
`--path` enumeration form over its 14 writes with the T4-scoped allowed
root (the recorded W1 T3 precedent, for the reason its return states:
the parent-owned T5 artifact sits outside a T4-only fence), also
`PASS/0 violations`. The two runs are consistent: the union of my
full-tree sweep equals the executor's 14 writes plus exactly the lawful
pre-existing state, and no path outside the §5 fence (as extended by
that lawful state) appears anywhere. The tracked `git diff --name-only`
set is exactly the 10 tracked paths of the 14-write list. Explicitly
verified untouched: `core/solver/**` (including `diagnostics`),
`core/product_physics/src/validation.rs` (0 changed files under both),
`core/loads/user_loads`, `core/runner`, `core/model_operations`,
`validation/witness/**`, all Cargo.toml/Cargo.lock files, docs, and
every other deliverable folder.

## 2. Per-Predicate Findings (brief §3.1–§3.11)

| Predicate | Verdict | My independent evidence |
|---|---|---|
| §3.1 Extent vocabulary exists once, validated | CONFIRMED | Diff of `core/loads/primitive_loads/src/lib.rs`: public `LoadExtent { start_fraction, end_fraction }` (Copy) with validating `LoadExtent::new` — both fractions finite (else `PrimitiveLoadError::NonFiniteInput`), `0 <= start < end <= 1` (else new Display-only error variant `InvalidExtentBounds`, not a finding code). `ElementExposedDiameter` gains `exposed_extent: Option<LoadExtent>`; `PrimitiveLoad` and `ElementUniformLoadContribution` gain `extent: Option<LoadExtent>`; all four existing helper constructors set `extent: None` (completeness compiler-enforced); `partial_uniform_element_load` is the sole extent-bearing constructor. No `LoadTarget` variant added; no existing helper signature changed. |
| §3.2 Deterministic partial-extent generation | CONFIRMED | `generate_wind_equivalent_static_loads`: the `None` match arm is the byte-identical pre-existing `uniform_element_load` call with the unchanged id `{case_ref}:generated:wind:{axis}:element-{i}`; `Some(extent)` emits one load per entry with id suffix `:extent-{start}-{end}` (shortest-round-trip f64 Display — deterministic for identical inputs, injective for distinct f64 extents; rerun-equality unit-tested). Invalid extents (reversed, negative, >1, NaN, empty) block via the existing `FindingCode::InvalidGenerationInput`; five invalid shapes unit-tested. NO new `FindingCode` variant anywhere in the diff; `core/solver/diagnostics` has zero diff and its 24 tests pass (mapping still exhaustive). |
| §3.3 Lever-rule static equivalence, existing tier | CONFIRMED | Both seams implement exactly the §3.3 closed forms. `prepare_lumped_nodal_loads`: `None` arm computes literally `magnitude * span.span * 0.5` to both nodes (pre-tranche math, same finiteness check semantics); `Some` arm computes `resultant = w * span * (b−a)`, `centroid = (a+b)/2`, `value_i = resultant*(1−centroid)` at `span.node_i`, `value_j = resultant*centroid` at `span.node_j`. `add_uniform_element_loads` straight branch: `None` arm literally `magnitude * length / 2.0`; `Some` arm identical lever-rule forms with `i = node_index(pipe.from)`, `j = node_index(pipe.to)` — fractions measured from the from-node in both seams, consistent orientation (moment about i gives `R_j = W·c`; re-derived in §4). `(0,1)` reduction exact in floating point (×1.0 and ×0.5 exact), asserted with `==` in both the unit test and the benchmark test. No fixed-end moment introduced anywhere in the diff. |
| §3.4 Curved-bend macro spans fail closed | CONFIRMED | In `add_uniform_element_loads`, `load.extent.is_some()` on a macro-realized span pushes blocking `LOAD_INPUT_INVALID` (`diagnostic:curved-bend:{case}:{load}:partial-extent-load`, message directs whole-span marking or separate pipes) and `continue`s — no chord approximation, partial-arc invention, or drop. The whole-span arc-consistent path is byte-unchanged. `curved_bend_uniform_intensity_by_pipe` skips extent-bearing loads. Consumer sweep: `element_uniform_loads` has exactly two production consumers in product_physics (both extent-aware/blocking) and one in primitive_loads (`prepare_lumped_nodal_loads`, extent-aware); `assemble_solver_load_vector` consumes post-lumping nodal contributions only — no seam silently ignores an extent. Integration test `subspan_wind_on_curved_bend_macro_span_is_blocking` passes (MODEL_INCOMPLETE, empty results). |
| §3.5 Additive fail-closed preview input | CONFIRMED | `WindGenerationInput` gains serde-default `exposed_spans: Vec<WindExposedSpanInput>` (all fields Option + serde-default). DEC-018: `normalize_model_units` normalizes `start_fraction`/`end_fraction` via `normalize_dimensionless_quantity` exactly mirroring `shape_factor`. Marking rules, all blocking on existing codes: no marking form at all (MISSING; message now names both forms — see §5 note), per-span missing fields (MISSING), unknown pipe ref (INVALID), same pipe in both forms (INVALID, "exactly one form"), overlapping extents on one pipe (INVALID; sorted adjacent-pair interior-intersection check — mathematically sufficient for pairwise interval overlap given validated `start < end`; endpoint-touching extents allowed, tested solving), invalid fractions (INVALID via `LoadExtent::new`). Exposed diameter uses the identical section basis (OD + 2·insulation). Seismic generation untouched. Each rule has a passing blocking test. |
| §3.6 Canonical schema + tests | CONFIRMED | Schema diff is exactly two hunks inside `WindEquivalentStaticInput`: required relaxed to `pressure`/`shape_factor`/`direction` + `anyOf` over `exposed_element_refs`/`exposed_spans`; additive `exposed_spans` (minItems 1, items `additionalProperties: false`, required `element_ref`+`extent`, `$ref` → `$defs/ElementReference` and `$defs/ElementLoadSpan`); block-level `additionalProperties: false` preserved; `exposed_element_refs` `minItems: 1` retained; no other `$defs` change anywhere in the diff. Currently-valid documents (all carry `exposed_element_refs`) satisfy the first `anyOf` arm — the unchanged round-trip test passes. `tests/test_model_schema.py`: exactly ONE existing assertion modified — the wind required-set subset assertion loses only the `"exposed_element_refs"` member; the replacement set verifies (i) three-key subset, (ii) exact `anyOf` equality, (iii) retained `exposed_element_refs` `minItems == 1` plus `exposed_spans` `minItems == 1` and the `ElementLoadSpan`-based item shape, (iv) `additionalProperties is False` (pre-existing assertion retained verbatim). Equal-or-stronger: the old schema would fail the new assertion set; no fact asserted at base is un-asserted at head. Every other assertion in both Python files untouched; `tests/test_physical_to_analytical_transform.py` diff is +84/−0: two additive tests (spans-only round-trip with schema validation; four rejection cases). |
| §3.7 Derivation witnessed | CONFIRMED | NEW `validation/hand_calcs/mechanics/tp_pmm_p3_subspan_wind_exposure.md` records: partial intensity `w = p·G·D_exposed` over `[a,b]`; `W = w(b−a)L`, `c = (a+b)/2`; the two-equation lever-rule derivation with both moment cross-checks; exact `(0,1)` reduction (analytic and floating-point); disjoint-extent superposition by linearity; explicit curved-bend fail-closed boundary (no partial-arc form invented); the fixture closed forms. Elementary statics and invented non-engineering numbers only; no code wind content. Independently recomputed in §4 — every value exact. |
| §3.8 Benchmark evidence | CONFIRMED (fixture path; fallback not needed) | New fixture `MECH-TP-PMM-P3-SUBSPAN-WIND-EXPOSURE` (family `EquivalentStaticGeneration` reused; provenance cites the new witness) with eight expected values computed from the witness closed forms; inventory registration; count assertion 23→24 (base grep: 45/86/36 tests and count 23 confirmed at `581a15b1c`); two new tests exercising generator + `prepare_equivalent_static_loads` + extent-aware lumping at the 1.0e-9 internal epsilon already used by the adjacent occloadgen tests; `tolerance_policy: None` on every expected value; NO new tolerance constant, threshold, or policy JSON anywhere in the diff (repo-wide diff grep: the only "threshold" match is boundary prose stating none was created). Both README mirrors: `git diff --numstat` = `1 0` each — exactly one additive inventory line, no other text changed. |
| §3.9 Existing behavior preserved | CONFIRMED | Whole-span arms are the literal pre-existing code (ids, math, diagnostics); seismic untouched; all pre-existing tests pass unchanged in every touched and regression crate (49/94/38/24/23+1+15 — decompositions 45+4, 86+8, 36+2 verified against base by `#[test]` count); no existing expected value or fixture body modified (benchmark diff is additive + count bump + two forced `exposed_extent: None` literal insertions); integration evidence (a)–(d) present and passing, with (d) the unchanged `wind_equivalent_static_generation_matches_authored_distributed_load`; del1005 five-case byte-identity independently re-established (§3 tally row 12; my own build + run + `cmp`, exits 0/0/0/1/1, SHA-256 equal to the executor's recorded values); `grep -rl equivalent_static validation/witness/` exits 1 (no witness uses the surface); `CliOutput` and runner untouched (zero diff; contract guard passes). |
| §3.10 Bounded state update | CONFIRMED | `_STATUS.md`: exactly the sole Remaining row removed, `## Remaining` header retained — the repo-wide empty-section convention (verified against DEL-04-02/04-03/04-06); exactly one new History entry (2026-07-20, cites the brief, records the GUI-emit follow-on, closes with "no review, validation, issuance, or lifecycle ruling was made"); `Last Updated` 2026-07-20. `MEMORY.md`: exactly one new dated section, truthful, naming the curved-bend fail-closed boundary and the GUI-emit follow-on to HELP_HUMAN; all cited test counts verified. One NEW run record `_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W2_T4_SUBSPAN_WIND.md`, claim-calibrated, no lifecycle/acceptance language. No other deliverable touched (containment sweep). |
| §3.11 Checks | CONFIRMED | Full §6 sequence independently re-run; tally in §3 — every check passed, all counts match the executor's. |

## 3. Independent Check Tally vs Executor (brief §6, in order)

| # | Check | My result | Executor | Match |
|---|---|---|---|---|
| 1 | fmt --check primitive_loads | PASS | PASS | ✓ |
| 2 | cargo test --offline primitive_loads | 49 passed, 0 failed | 49 (45+4) | ✓ |
| 3 | fmt --check product_physics | PASS | PASS | ✓ |
| 4 | cargo test --offline product_physics | 94 passed, 0 failed | 94 (86+8) | ✓ |
| 5 | fmt --check benchmarks/mechanics | PASS | PASS | ✓ |
| 6 | cargo test --offline benchmarks/mechanics | 38 passed, 0 failed | 38 (36+2) | ✓ |
| 7 | Two-mirror inventory check | Both READMEs list the fixture (hand-calc README also names the witness file); numstat `1 0` each | same | ✓ |
| 8 | cargo test --offline solver/diagnostics (read-only) | 24 passed; no new finding code | 24 | ✓ |
| 9 | cargo test --offline runner/headless (read-only) | 23 + 1 + 15 passed | 23+1+15 | ✓ |
| 10 | pytest test_model_schema + test_physical_to_analytical_transform | 22 passed | 22 passed | ✓ (decomposition note, §5) |
| 11 | `python3 tests/test_headless_runner_contract.py` | PASS (exit 0) | PASS | ✓ |
| 12 | del1005 five-case byte-identity | Runner built offline at implementation head; all five stdouts `cmp`-identical to committed witnesses; exits 0/0/0/1/1; SHA-256 `813a702b…0728`, `8feb3d25…d68`, `2f89adce…7593`, `9596c052…49a4`, `61cba4f2…0518` | identical SHAs | ✓ |
| 13 | validate_claims_language | `VALID … 262 files scanned; DEC-081 registry taxonomy satisfied` | 262 | ✓ |
| 14 | validate_path_anchors | `PASS … 671 live path-anchor surfaces` | 671 | ✓ |
| 15 | `git diff --check` | clean | PASS | ✓ |
| 16 | JSON parse of new/changed .json | Only `instances/W2/T4/CHANGE_SCOPE_CONTAINMENT.json`; parses; no project .json changed | same | ✓ |
| 17 | Changed-path containment | My full-tree sweep PASS, 0 violations (19 paths); executor's persisted 14-path run PASS, 0 violations; consistent (§1) | PASS | ✓ |

Note: checks 10–14 were re-run by me AFTER this verdict artifact did not
yet exist; this artifact is a coordination-instance file outside the
claims-language registry and containment fence concerns (fence §5 item 9
covers `instances/W2/T4/**`).

## 4. Independent Physics Recomputation

From the witness's stated inputs only (p = 480 Pa, G = 0.7, D_o = 0.2 m,
t_ins = 0.025 m, L = 3.0 m; extents A = [0.2, 0.7], B = [0.8, 1.0]):

- Intensity: `w = 480 × 0.7 × (0.2 + 2×0.025) = 480 × 0.7 × 0.25 = 84.0 N/m` — matches.
- Lever rule (re-derived): resultant at `x_c = cL` from node i; force
  balance `R_i + R_j = W`; moment about i: `R_j·L = W·c·L ⇒ R_j = W·c`,
  hence `R_i = W(1−c)`; unique two-force reduction. Orientation matches
  both implemented seams (share `(1−c)` at the from-node).
- Extent A: `W_A = 84×0.5×3 = 126.0 N`, `c_A = 0.45`,
  `R_iA = 126×0.55 = 69.3 N`, `R_jA = 126×0.45 = 56.7 N` — match.
- Extent B: `W_B = 84×0.2×3 = 50.4 N`, `c_B = 0.9`, `R_iB = 5.04 N`,
  `R_jB = 45.36 N` — match.
- Superposition: `R_i = 74.34 N`, `R_j = 102.06 N`, `W = 176.4 N` — match.
- Moment cross-check: `R_j·L = 102.06×3 = 306.18 N·m` =
  `126×0.45×3 + 50.4×0.9×3 = 170.1 + 136.08 = 306.18 N·m` — exact.
- `(0,1)` reduction: `W = 252.0 N`, `c = 0.5`, `R_i = R_j = 126.0 N =
  w·L/2` — exact, including in floating point (`×1.0` and `×0.5` exact).
- Mixed-marking test values (P-200, extent [0.5, 1.0]): `W = 126 N`,
  `c = 0.75` → `31.5 N` / `94.5 N` — match the integration test.

The benchmark fixture computes its eight expected values from these same
closed forms (`subspan_wind_lever_rule_shares`, shared occloadgen
constants), and its tests tie the generator + lumping output to them at
the recorded 1.0e-9 internal epsilon with `tolerance_policy: None` —
the recorded DEC-026 analytic-class tier, no new tolerance constant.

## 5. Executor Return Integrity (EXECUTE_RETURN.md vs my findings)

Every operative claim in `EXECUTE_RETURN.md` was re-established
independently and none was refuted: base-commit freeze facts, per-predicate
statuses, the 14-write list, the del1005 SHAs, the check outcomes, the
recorded design choices (interior-intersection overlap rule; extent id
Display formatting; the `InvalidExtentBounds` error-variant/no-new-
FindingCode distinction; the minimal reading of the single authorized
replacement), and the follow-on report. No softening or over-claim of
substance was found; the v1 BLOCK is preserved and truthfully
characterized throughout the chain.

One precision note (resolved, non-blocking): check-tally row 10 states
"22 passed (19 pre-existing test functions + 3 new across the two files;
the one authorized replacement sits inside the pre-existing schema test
function)", echoed compressed in the run record. My independent count:
base = 20 collected test functions (5 + 15), head = 22 (5 + 17), i.e.
2 NEW test functions and 20 pre-existing, of which 19 are untouched and
1 (the schema-contract test, via its helper) carries the authorized
replacement plus additive assertions. The record's own clarifying clause
and its §3.6 row ("gains two additive tests") state exactly this, so the
"19 + 3" decomposition reads as 19 untouched + 3 carrying new-or-replaced
coverage — truthful under the clarification it itself supplies, and the
operative facts (22 passed; exactly one replacement; exactly two additive
transform tests; every other assertion untouched) are separately stated
correctly and verified. This mirrors the v2 brief-verifier's precedent
for a uniquely-resolvable imprecision (its §2.1 line-range note). Not a
refutation; the W2 manager may optionally have the two tally sentences
tightened before commit.

Non-blocking observation (pre-existing, not a §6 check): non-test builds
of the mechanics-benchmarks crate (as a dependency of the headless
runner) emit a pre-existing `unused_imports`/`dead_code` warning pair for
cfg(test)-only-used items; the tranche appended `LoadExtent` to the same
already-warned import list. `cargo fmt --check` and all test builds are
clean; no §6 check gates on non-test warnings; recorded for completeness.

## 6. Defect List

None blocking. (§5 precision note and observation recorded above.)

## 7. Scope of This Artifact

This verification performs no adoption, lifecycle, release, or acceptance
act; commit authority remains with the W2 manager under the R14 campaign
chain. All statements above are made to the warrant of my own reads,
diffs, greps, recomputation, and offline check re-runs at the working
tree state described in §1.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
