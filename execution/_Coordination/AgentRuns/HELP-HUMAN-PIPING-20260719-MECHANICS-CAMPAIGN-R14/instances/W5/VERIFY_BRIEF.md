# VERIFY_BRIEF — W5 Fresh-Context Adversarial Brief Verification

**Role:** Fresh-context adversarial brief verifier (bounded Agent 2 child of
the W5 wave manager), campaign run
`HELP-HUMAN-PIPING-20260719-MECHANICS-CAMPAIGN-R14` / W5
**Date:** 2026-07-20 (UTC)
**Target:** `execution/_Coordination/CANDIDATE_BRIEF_2026-07-20_DEL-09-04_CLEAN_REPRO_R14.md`
(`CB-2026-07-20-DEL-09-04-CLEAN-REPRO-R14-001`), adoption effect HELD
**Verified tree state:** branch `claude/piping-r14-w5-clean-repro`
(recomputed with `git branch --show-current`), HEAD recomputed with
`git rev-parse HEAD`:

```text
a5235340aae3c41cf227f5617e593b268936f6b3
```

byte-identical to the brief's pinned `SOURCE_COMMIT` (programmatic string
comparison of the recomputed HEAD against the 40-hex pin extracted from
the brief; first 12 hex = `a5235340aae3`, matching the proposed RUN_ID
suffix). Working tree clean apart from the lawful untracked W5 state (the
candidate brief and `instances/W5/**`, confirmed by
`git status --short`). Every claim below was recomputed or read directly
from the live tree by this verifier; no manager narrative was accepted on
trust.

## Inputs Read (live tree)

- `docs/validation_manual/headless_runner_reproduction.md` (full read; SHA
  recomputed)
- The candidate brief (full read), its rationale
  `instances/W5/CURRENT_CANDIDATE_RATIONALE.md`, and
  `instances/W5/W5_DISPATCH_TRANSCRIPT.md`
- Eight committed witnesses + two generators under `validation/witness/`
  (all digests recomputed); frozen solve input `supports[6]` parsed
- `execution/_Coordination/CANDIDATE_BRIEF_2026-07-18_DEL-09-04_CLEAN_REPRODUCTION.md`
  (R11 brief, §§2–3.6, 4.2, 5, 7, 8, 10) and the completed R11 bundle
  `validation/evidence/reproduction/REPRO_DEL0904_20260719T202023Z_23eeaabc9040/`
  (layout + checks inventory listed); the three earlier bundles' README/
  manifest statuses read
- DEL-09-04 `_STATUS.md` (full read); W3 `RETURN.md` rows 7–8; W4
  `instances/W4/T7/EXECUTE_RETURN.md` (full read) and `instances/W4/RETURN.md`
  W4-C7
- `_DECISIONS/D-52_four_lens_standing_approval_overlay.md` (§§1–4.1),
  `_DECISIONS/D-54_reasoned_discretion_standing_approval_refinement.md`
  (full read), `SOFTWARE_DECOMP.md` DEC-080/DEC-087 register rows
- `HELP-HUMAN-PIPING-20260719-DEL1005-RUNNER-PAYLOADS-R12/ENVIRONMENT_REPAIR_DISPOSITION.md`
  (full read)
- `software-workflow.json` (full read; SHA recomputed); root tools'
  existence verified (`run_registered_checks.py`, `validate_change_scope.py`,
  `tools/validation/*`); `tests/test_headless_runner_contract.py` and
  `core/runner/headless/Cargo.toml` existence verified
- `loop/LOOP_RECEIPTS.md` tail (Receipt-61/62/63) + receipt validator run
  (VALID); `ORCHESTRATION_PLAN.md` (full read);
  `loop/WORKPLAN_2026-07-18b_piping_loop.md` existence verified
- Read-only validator runs by this verifier: `validate_claims_language.py`
  (VALID, 262 files), `validate_path_anchors.py --text` (PASS, 700
  surfaces), `validate_piping_loop_receipts.py` (VALID)

## VERDICT: COMMIT-SAFE

No BLOCKING defect. Two Low defects and three INFO observations recorded
below; none invalidates a predicate, the write fence, the containment list,
the failure disposition, the governance classification, or the HELD status.

---

## C1 — Per-case exits, diagnostics, and evidence vs the live manual

Read the manual in full and compared every P1–P12 pin verbatim.

| Brief pin | Manual anchor | Match |
|---|---|---|
| P2 case 1: exit 0; `COMPLETED`; `request_validation.diagnostics` and `result_validation.diagnostics` empty; `result_refs` non-empty; one non-blocking `SUPPORT_CONSTANT_EFFORT_NOT_CONSUMED` warning for `support:CE-120`; NOT diagnostic-clean | Part 1 case-1 row + dated 2026-07-20 note ("still exits 0 ... `COMPLETED` ... empty request/result validation diagnostics ... non-empty `result_refs` ... one non-blocking warning ... for `support:CE-120`") | YES |
| P2 "exactly one in the entire document, located in `mechanics_envelope.diagnostics`, severity `warning`" | Manual says "solve result envelope"; the exact location/count pin is warranted by W4 T7 `EXECUTE_RETURN.md` live-run evidence ("exactly one ... in the whole output, located at `mechanics_envelope.diagnostics[4]` (the solve result envelope), severity `warning`"), which the brief cites | YES (warranted, not overclaimed) |
| P3 case 1: regenerated does NOT byte-match witness `c406d9c2...`; recorded, both digests | Dated 2026-07-20 note: "regenerated output no longer byte-matches the committed ... witness" | YES |
| P4 case 2: exit 1; `HEADLESS_RUNNER_LOAD_BASIS_MISSING`; no solver result | Part 1 case-2 row, verbatim | YES |
| P5 case 3: exit 1; `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` (post-#287); stub-witness mismatch EXPECTED | Part 1 case-3 row + dated 2026-07-19 note (stub code `HEADLESS_RUNNER_OPERATION_STUB_REQUIRES_DOWNSTREAM_PAYLOAD` is historical pre-#287 only) | YES — brief correctly does NOT expect the stub code |
| P7 mechanics, `MECH-TP-PHYS-004-LOAD-TO-RESULTANT`, 1/1, diagnostics empty, exit 0 | Part 2 table row 1 | YES |
| P8 stress, 3 named cases, 3/3, diagnostics empty, exit 0 | Part 2 table row 2 (all three case IDs match verbatim) | YES |
| P9 nonlinear, `whole_suite_default_applied` true, 5/5, diagnostics empty, exit 0 | Part 2 table row 3 | YES |
| P10 exit 1, `HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING` | Part 2 table row 4 | YES |
| P11 exit 1, `HEADLESS_RUNNER_REGRESSION_PAYLOAD_MISSING` | Part 2 table row 5 | YES |
| 0/0/0/1/1 pattern | "Expected exits are 0, 0, 0, 1, 1 respectively" — verbatim | YES |
| P6 generator scope quote | Manual Part 2 intro — verbatim ("writes only `del1005_payload_binding_*` input files and does not touch the frozen `tp_runner_015` surfaces") | YES |
| P12 review checks: headless cargo test + `python3 tests/test_headless_runner_contract.py`, both exit 0 | Review Checks section (brief adds the offline posture from R11 §3.4 — stricter, not weaker) | YES |
| §5.2 output filenames (8) | Manual Part 1 + Part 2 procedure `--output` basenames | YES, all eight |

Case 1 is pinned NOT diagnostic-clean; case 3 is pinned to
`HEADLESS_RUNNER_BENCHMARK_PAYLOAD_MISSING`, not the stub code — both
exactly as the C1 mandate requires. No exit-code, diagnostic-code, or
evidence-cell mismatch found. One verbatim-quotation defect in a §2
narrative citation (not a predicate) — see defect D2.

## C2 — Pinned SHA-256 recomputation at HEAD

`git rev-parse HEAD` (verbatim tool output):
`a5235340aae3c41cf227f5617e593b268936f6b3` — compared programmatically
(shell string extraction + equality) against the brief's pinned
`SOURCE_COMMIT`; identical. The dispatch's short form `a5235340aae3`
prefixes it.

All twelve content digests recomputed with `shasum -a 256` at this HEAD;
every one matches the brief's pin exactly:

| File | Recomputed SHA-256 | Brief pin |
|---|---|---|
| `docs/validation_manual/headless_runner_reproduction.md` | `fa714cf44d5c3e8a54ff6e2f6883676b81e01755e2e07d36a5bd118576b299c1` | match (§1) |
| `software-workflow.json` | `123249634475e87207cd75740dc25e5061c08cc7a1708aa239105b27e30c9c2f` | match (§1) |
| `.../tp_runner_015_final_cli_solve.json` | `c406d9c2d8b6e739cd8faf86fcd67ff8f685342f9ee056b5544685a769705188` | match (P3) |
| `.../tp_runner_015_final_cli_validation_blocking.json` | `0d707ee26f4bbdd979f06580b46c658a7e10fdcde7f7833585bb15f2863dd1c1` | match (P4) |
| `.../tp_runner_015_final_cli_benchmark_stub.json` | `420484c82e798b9f0bb12d17d4381dab18b17f0c7468aa6110607c16ded94d4d` | match (P5) |
| `.../del1005_payload_binding_benchmark_single_case.json` | `813a702b74be74a88755626d5b4530716d4fd5e27a1b988e48f3da3be3306728` | match (P7) |
| `.../del1005_payload_binding_benchmark_multi_case.json` | `8feb3d25e50e78dcd7fcc85e2253021610faa971a37837eefb63df5cea456d68` | match (P8) |
| `.../del1005_payload_binding_regression_full_suite.json` | `2f89adce9e4d6250280cee347822a567f4405eafbb8bc666483c6ce4cbd87593` | match (P9) |
| `.../del1005_payload_binding_benchmark_payload_missing.json` | `9596c052c76a178e13bcf29faa5841848df6d9453983b184ebff3fd5fc2449a4` | match (P10) |
| `.../del1005_payload_binding_regression_payload_missing.json` | `61cba4f28bcf109510489125b1de11e44796a25285ca64bf9c0714e870d9f518` | match (P11) |
| `validation/witness/inputs/generate_tp_runner_015_inputs.py` | `5a9f29b7a24163706c78c0ce412b4fa270aafdcd19f44a3d4edba2de84feae68` | match (§3.4) |
| `validation/witness/inputs/generate_del1005_payload_binding_inputs.py` | `4c81b645723495dee04a5ad854d7274b6e863883ed382d3117e728585c062b9f` | match (§3.4) |

Frozen solve input parsed: `supports` has 7 entries; `supports[6]` is
`id: support:CE-120`, `family: constant_effort_support`, `restraints: []`,
`hanger.constant_load` 375 N — exactly the non-consuming shape the brief
and the dated note describe. No digest defect.

## C3 — Write fence completeness/minimality and §6 containment list

Fence items (§5): (1) the brief itself (status record only); (2) one
bundle `validation/evidence/reproduction/<RUN_ID>/**`; (3) one
`validation/evidence/sweeps/SWEEP_*.json`; (4) DEL-09-04 `_STATUS.md` +
`MEMORY.md` + one named run record; (5) `instances/W5/**`; (6) NO receipt,
no executor commit. Everything the execution must durably write maps into
this set: all bundle contents incl. `checks/*` (item 2), the sweep (item
3), state files (item 4), `EXECUTE_RETURN.md`, `VERIFY_*`, manager
closeout `CHECK_*.json` (item 5), status flip (item 1). Nothing forbidden
is admitted: no `loop/LOOP_RECEIPTS.md` (correctly absent — the R11 §4.2
list had it; this brief's divergence is deliberate and stated twice),
no code/test/schema/witness/fixture/docs path, no other deliverable.

§6 `validate_change_scope.py` `--allowed` list has exactly seven entries:
brief file, `$RUN_ID` bundle dir, `$SWEEP_FILE`, the three DEL-09-04 files
(`_STATUS.md`, `MEMORY.md`,
`_run_records/WORKING_ITEMS_RUN_2026-07-20_R14_W5_CLEAN_REPRO.md`), and
the `instances/W5` dir — a one-to-one image of the fence, no receipts, no
extras. Run-record filename is identical in P15, §5 item 4, and the
`--allowed` list. Pre-existing untracked W5 state (this file, the
rationale, the dispatch transcript, the brief) is inside allowed paths, so
the untracked-inclusive containment check cannot be tripped by lawful
state. The ephemeral-writes clause is bounded to the `mktemp -d` root plus
the §4.4 ignored-paths copy inside the clone. No defect.

## C4 — Byte-comparison policy calibration

- Five del1005 cases (P7–P11): byte-equality REQUIRED. Warranted: the
  manual Part 2 names the five committed witnesses as the expected
  evidence, records them "reproduced against the committed witnesses"
  (R12 N3), and the dated 2026-07-20 currency note states "the five
  committed `del1005_payload_binding_*` witnesses are byte-identical at
  head". The W5 dispatch independently requires it.
- Cases 1 and 3 (P3, P5): byte mismatch EXPECTED and recorded, never a
  predicate pass/fail — exactly what the two dated notes declare
  ("regenerated output no longer byte-matches"; stub witness "records the
  historical pre-#287 stub diagnostic").
- Case 2 (P4): observation only. The manual's case-2 row states diagnostic
  evidence only and no dated note declares either identity or divergence;
  requiring equality would exceed warrant, forbidding it would discard
  data. Recording the observation is the calibrated middle.

The split matches the manual exactly; no overclaim found. No defect.

## C5 — Failure disposition

§7 is fail-closed: expected exit-1 cases count only with the matching
diagnostic; witness mismatch (P7–P11), schema drift, frozen-byte change,
check/hash failure, unclean state, or containment violation = `FAIL` and
stops closeout; missing prerequisites outside the §4.4 named class =
`BLOCKED` with no install/fetch; the truthful failed/blocked bundle is
preserved (R11 §7 precedent confirmed equivalent); rerun = new `RUN_ID`;
no repair lane in-tranche; `Remaining` and lifecycle unchanged in EVERY
outcome (§4.7 + P15 + §7). Confirmed against live `_STATUS.md`:
`IN_PROGRESS`, exactly two Remaining bullets, both owner-gated per W3
RETURN rows 7–8, and NO bullet names this rerun — so strike-no-rows is
correct and an R11-style bullet closure would have been wrong. The brief
hard-requires `## Remaining` byte-identity. No defect.

## C6 — Independent D-54 §3.1 ten-class screen (run by this verifier)

1. Personal preference/identity — not touched; shape fully derived from
   manual + R11 precedent + DEC-080. NO HIT.
2. Purpose/scope boundary — re-executes the already-documented procedure;
   nothing enters or leaves scope. NO HIT.
3. New normative content / acceptance criteria — every predicate restates
   the manual's own tables/notes or committed witness bytes; the
   strongest candidate hit (P7–P11 byte-equality as a "new criterion")
   fails: the witnesses are the manual's stated comparison basis and the
   result is labeled evidence, never a criterion; DEC-046 untouched. NO
   HIT.
4. Professional/safety/legal/fiduciary — bundle restates that acceptance
   and professional judgment remain with the responsible engineer. NO HIT.
5. Spending/procurement/external commitment — offline, local-only. NO HIT.
6. Publication/release/issuance — no push/PR/merge/receipt by any W5
   node. NO HIT.
7. Protected data — invented public-metadata fixtures only. NO HIT.
8. Destructive/irreversible — new immutable bundle; cleanup restricted to
   the validated task-created temp root; no history rewriting. NO HIT.
9. Lifecycle/promotion/acceptance — `INTERNALLY_VERIFIED` only on PASS;
   no Remaining strike; every named gate preserved (§9/§10). NO HIT.
10. Stale evidence/authority conflict/claim beyond warrant — predicates
    re-anchored to the live manual precisely BECAUSE the R11 pins went
    stale; digests recomputed by this verifier; the one head-behavior
    expectation carries W4 T7 live-run evidence. The strongest candidate
    conflict — D-54 §2 / DEC-087 "DEL-09-04 is prospectively excluded" —
    was examined and is not a hit: that exclusion protects the R11-era
    candidate, evidence, records, receipts, terminal FAIL, and Remaining
    item from being modified/rerun/reinterpreted BY the D-54 refinement;
    this brief leaves all of them byte-untouched and is a NEW instrument
    whose rerun obligation issues from the R11 brief's own §8 clause. The
    landed, receipt-recorded precedents (R13 refresh; R14-W4 T7, both
    DEL-09-04 tranches under D-52+D-54 with fresh verifiers, Receipts
    60/63) settle the accepted-practice reading; additionally the
    rationale's §4 alternatives are each refuted or forbidden rather than
    left surviving-defensible, so the selection also survives the
    stricter DEC-085 exactly-one-outcome test. NO HIT — recorded as INFO
    I1 so the disposition is visible. Ambiguity resolved by live
    artifacts, not preference.

No fast-reject hit; nothing in the brief authorizes acceptance, promotion,
lifecycle, receipt, release, or network acts. The rationale's screen
glossed nothing material; its §6 adversarial self-check attacks match the
three strongest attacks this verifier found independently.

## C7 — RUN_ID convention and profile-check division

- Form `REPRO_DEL0904_<UTC-YYYYMMDDTHHMMSSZ>_a5235340aae3`:
  `a5235340aae3` is exactly the first 12 hex of `SOURCE_COMMIT` (verified
  character-by-character). Matches R11 §3.1
  (`REPRO_DEL0904_<UTC>_<SOURCE_COMMIT-12>`) and the three most recent
  bundle names (`..._525ef0903e68`, `..._89a93d7ca21d`,
  `..._23eeaabc9040`). The oldest bundle's 11-hex suffix
  (`..._f14fa77518a`) is a pre-existing historical anomaly, not this
  brief's doing (INFO I3). DEC-080 register row confirms the ruled home
  `validation/evidence/reproduction/<run-id>/`.
- `software-workflow.json` recomputed and read: `validation/**` selects
  `piping-pytest` + `evidence-sweep`; `execution/**` selects
  `harness-pytest`; `harness-self-check` is the always-check. The tranche
  writes both surfaces, so the full selected set is exactly those four.
  Brief division: executor runs `evidence-sweep` (with one-file-delta
  proof); manager runs `piping-pytest`, `harness-pytest`,
  `harness-self-check` at closeout — union covers the full selected set
  with no omission; matches the W4 precedent (branch-level checks at wave
  closeout) and the W5 dispatch. No defect.

## C8 — §4.4 contingency boundedness

The trigger is a named failure class only ("if — and only if — an offline
cargo command fails specifically because of this missing
gitignored-lockfile/ignored-build-state class"); the authorized ACTION is
narrower still: copy already-present gitignored `Cargo.lock` files only,
from the primary `REPO_ROOT` tree, nothing tracked, explicitly "no
`node_modules`", "no network, registry, download, or install", with each
copied path + triggering failure evidence + precedent citation recorded in
`commands.jsonl`, the README, and the run record, cleanliness proofs still
required, and "any other missing prerequisite is a truthful `BLOCKED`
result — no provisioning". Compared against R12-ENVREPAIR-01: the
precedent covered ignored-path copies of `Cargo.lock` AND `node_modules`;
§4.4 adopts a strict subset (lockfiles only) and adds recording duties, so
it sits inside the precedent and cannot be stretched into general
provisioning — the action clause, not the trigger clause, bounds the
writes, and the action clause names one file kind. Primary-tree
preconditions verified live: per-crate `Cargo.lock` files exist (e.g.
`core/runner/headless/Cargo.lock`) and `git check-ignore` confirms they
are gitignored. Fail-closed otherwise: confirmed (§4.2, §4.4 last
sentence, §7 bullet 3). Trigger-clause wording slightly broader than the
action (INFO I2). No defect.

## C9 — HELD status and chain conformance

- Frontmatter `status: proposed_effect_held`; body status line `PROPOSED —
  ADOPTION EFFECT HELD PENDING FRESH-CONTEXT INDEPENDENT REFUTATION`; §5
  first line "While the adoption effect is held: no execution writes are
  authorized"; §10 `EffectStatus: HELD`, `IndependentVerifier: PENDING`;
  closing paragraph "No execution is released by this document in its
  current state." Genuinely HELD — no execution authority exists now.
- §10 conforms to the D-54 §3.3 attribution template: all nine template
  fields present (`OwnerStandingApproval` citing DEC-085/D-52 §2 as
  refined by DEC-087/D-54 §1; `AgentJudgment: SELECT_AND_ADVANCE`;
  `SelectedOutcome`; `JudgedBy`; `OwnerCaseSelection: NONE`;
  `RejectedAlternatives`; `RationaleArtifact`; `IndependentVerifier`;
  `EffectStatus`; `PreservedGates`) plus the D-52 classification fields.
  `AdoptionAuthority: HUMAN_OWNER_BY_STANDING_APPROVAL ... SHA-bound at
  governance commit f14fa77518a06f112ae72a8fcce4de0fab958d47` — matches
  the R11 brief's recorded durable landing commit and D-52's own record.
  Adoption correctly attributed as the owner's conditional act; the agent
  only classifies/selects.
- Chain (verifier → EFFECTIVE → executor → fresh implementation verifier
  → manager commit; no push/PR/receipt by W5; HELP_HUMAN fan-in) matches
  both the W5 dispatch transcript and the R14 `ORCHESTRATION_PLAN.md`
  execution rules ("fresh-context adversarial verifier before effect,
  executor, fresh-context implementation verifier before commit").
  Receipt cursor recomputed: `Receipt-63` newest; receipt validator VALID.
  No defect.

## C10 — Internal consistency, paths, claims language, rationale quality

- Path existence spot-checked for every file path the brief cites: the
  manual, both generators, all eight input fixtures, all eight witnesses,
  R11 brief + bundle (layout matches §5.2 exactly, including the
  `checks/` inventory), R12 disposition, `_STATUS.md`/`MEMORY.md`/
  `_run_records/` home, `software-workflow.json`,
  `tools/software_workflow/{run_registered_checks,validate_change_scope}.py`,
  `tools/validation/{validate_claims_language,validate_path_anchors,validate_piping_loop_receipts}.py`,
  `tests/test_headless_runner_contract.py`,
  `core/runner/headless/Cargo.toml`,
  `loop/WORKPLAN_2026-07-18b_piping_loop.md`, `ORCHESTRATION_PLAN.md`,
  the W1–W4 instance dirs, `docs/PRD.md`. All exist.
- Cross-references verified: PR #287 commits (`60841413a` impl,
  `45ec0524d` merge) match the manual; `faee4faed`/`581a15b1c`/
  `e315fb840` match Receipts 61–63 and the manual's notes; W4 T7 digest
  `b3cd85af8565...8613` and 830 `result_refs` match
  `instances/W4/T7/EXECUTE_RETURN.md`; W4-C7 frozen-surface byte-identity
  claim present in `instances/W4/RETURN.md`.
- No contradiction found between §§1–10: predicates, fence, containment
  list, bundle layout, and run-record name are mutually consistent; the
  doc_id is consistent between frontmatter, dispatch, and rationale.
- Claims language: no acceptance/certification/promotion claim; explicit
  "closes NO Remaining row ... accepts nothing"; `PROVER_CORRELATED`/
  `ENGINEER_ACCEPTED` expressly forbidden (P14); the standard claim fence
  sentence is present (§9 end) and required in the bundle README (§5.2).
  `validate_claims_language.py` and `validate_path_anchors.py` both PASS
  on the tree carrying the brief (noting the claims lint's scan set does
  not cover coordination briefs; the manual inspection above is the
  operative check, consistent with prior-brief practice).
- Rationale genuinely addresses NO-OP (§4 item 5, refuted with the three
  fired rerun rows and the absence of any current-head bundle — verified:
  newest completed bundle is pinned to `23eeaabc9040`, pre-#287) and five
  material alternatives with reasons; the adversarial self-check (§6)
  attacks real failure modes.

Defect D1 (Low) is the one C10-class finding.

---

## Defect List (severity-ranked)

**BLOCKING:** none.

**MEDIUM:** none.

**Low:**

- **D1 — "Three prior completed bundles" miscount/ambiguity.** Brief §2
  bullet 3 ("Three prior completed bundles plus one truthful
  blocked/failed lineage live under `validation/evidence/reproduction/`")
  and rationale §3 Ontology ("Three prior completed bundles establish the
  kind"). Recomputed live statuses: `..._f14fa77518a` FAIL,
  `..._525ef0903e68` BLOCKED, `..._89a93d7ca21d` FAIL,
  `..._23eeaabc9040` PASS — i.e., ONE completed-PASS bundle, two FAIL,
  one BLOCKED. The sentence is defensible only under the reading
  "completed = finalized to a terminal bundle" (2 FAIL + 1 PASS) with the
  BLOCKED run as the "lineage", and misleading under the natural reading
  "completed = PASS". No predicate, fence entry, or execution step
  depends on the count, and the brief separately (and correctly) commits
  to editing none of them; narrative-accuracy defect only. Anchor: brief
  §2 third bullet; rationale §3 first lens.
- **D2 — Misquotation of the manual's Rerun Consequence.** Brief §2
  bullet 1 quotes the manual as "any subsequent clean-checkout
  reproduction executes from a post-refresh source commit under a fresh
  run ID and a new immutable bundle"; the live manual (## Rerun
  Consequence, line ~213) reads "...executes from a **post-#287** source
  commit...". Semantics are unaffected (`a5235340aae3` is both post-#287
  and post-refresh) and the trigger stands independently on R11 §8, but
  the words are inside quotation marks and do not match the source
  verbatim. Anchor: brief §2 first bullet vs manual "Rerun Consequence".

**INFO:**

- **I1 — D-54/DEC-087 DEL-09-04 prospective-exclusion clause considered
  and dispositioned** (see C6 class 10). Not a defect: the exclusion
  guards the R11-era candidate/evidence/records, all left byte-untouched;
  new-brief-under-D-54 practice for DEL-09-04 is settled by the landed
  R13 and W4-T7 precedents (Receipts 60/63); the selection also survives
  the stricter DEC-085 test because every §4 alternative is refuted or
  forbidden rather than surviving-defensible.
- **I2 — §4.4 trigger-clause phrase** "gitignored-lockfile/ignored-
  build-state class" is marginally broader than the authorized action
  (lockfile copies only). The action clause is what bounds writes, and it
  is unambiguous; no cure required.
- **I3 — Historical bundle-name anomaly**: the oldest bundle dir carries
  an 11-hex suffix (`REPRO_DEL0904_20260718T215424Z_f14fa77518a`); the
  brief's proposed RUN_ID conforms to the 12-hex convention. Pre-existing
  tree fact, not a brief defect.

## Per-class "no defects" statement

C1: no defects. C2: no defects. C3: no defects. C4: no defects. C5: no
defects. C6: no defects (I1 recorded). C7: no defects (I3 recorded). C8:
no defects (I2 recorded). C9: no defects. C10: D1, D2 (both Low).

Verdict restated once: **COMMIT-SAFE** — no BLOCKING defect; the two Low
defects are narrative-accuracy items the W5 manager may cure editorially
before or at the status flip without re-verification of any predicate,
digest, fence, or governance conclusion (neither touches an executable
instruction), or may leave recorded here.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
