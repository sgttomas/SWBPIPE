# I96 B3-D: REVISION_01, RV116's amendments and ROOT's rulings

TASK (Type 2), I96, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC. Documents only, with the draft statics regenerated: no cargo, no install, no Git write. Git reads used `GIT_OPTIONAL_LOCKS=0`.

**What this amends.** `DESIGN.md` in this folder, sha256 `ad7942f66c906270003c7c7fd98bb7c64900183dc47b31796844fb051e37b69b`, which is left byte-identical. DESIGN.md plus this revision is the design. **Where they differ, this revision governs.**

**What it answers:**
- **RV116's review:** `R/REVIEW_RV116/b3_d_01/REVIEW.md`, sha256 `001b7a329c6bf20dd268011bd677fe6e01bba7de7418e796794b51528d37f51d` (verified; its SHA256SUMS 8 of 8 OK). ACCEPT WITH AMENDMENTS, 0/2/12.
- **RV115's addendum:** `R/REVIEW_RV115/b2_kd_01/ADDENDUM_01.md`, sha256 `501246079018ddbd510745e2ddbd9324003bf99628c0de67a7f1eff58dc55902`.
- **ROOT's rulings,** RR sha256 `5bdd3a8c…0a36` at my read:
  - "RV115's addendum accepts B3-K (K3-1, K3-2); the kernel items of B3-D ruled";
  - "RV116 (RV-D) accepts B3-D with amendments; B3-D ruled; I96 revises the draft statics";
  - the dispatch message for this revision.

**Placeholders and notation** are DESIGN.md's. **Basis:** NUM `d867caa1ee`. Its `P/core`, `P/fixtures`, `P/schemas`, `P/apps` and `P/tests` trees still equal main `2007709549` (`git diff --quiet`).

## 0. Results in brief

**The regenerated statics** (`_run_records/revision_01/b3d_statics_r1.py`, run twice, byte-identical):

| Static | v0 (DESIGN.md) | **Revision 01** |
|---|---|---|
| DEF-E raw sha256 | `6edae5ff…3879` (9,480 B) | **`71f63d3916fa37ad0021ffb6ad993760a274166fe7ef275d7435c6856ed5642e`** (9,733 B) |
| DEF-E H(`retained_precision_formation_v1`) | `9b66492e…693d` | **`5a3bac430df9bbc77484d5419c75880ad40ae209b439e5f928374458025281af`** |
| XTABLE sha256 | `5bf0d0dc…b5c7` | **`c4987e874889645ac315b5f55f58690082ad5e7745527f20e3e316efa3e70a3d`** (51,163 B) |
| SCHEMA, B3b's change only, against main | `9558a6a1…7b48` | **`0d5bb812ca812328f51c1e65cea0c9904005f6400a7a875e55270f7f37e85054`** (adds N-8's title and comment) |

- RV116's illustrative hashes (`6a45e9f0…`, `8b8743ff…`) are **not** these, because my wording differs from RV116's (§1).
- **Layout:** the revised drafts are in `statics/r1/`. The v0 drafts stay unchanged at their original `statics/` paths, so the original `SHA256SUMS` still verifies 11 of 11.

**One line per amendment:**
1. **S-2:** DEF-E's `evidence` and `rows.maximum` now regenerate only the owner case's `exact_cases` entry, and state that every unselected entry is byte-identical. `pressure_regions` in `scope.excludes` becomes `nonempty_pressure_regions`. DEF-E and XTABLE are regenerated (§1).
2. **S-1:** the preparation payload's `definition_sha256` is the route's table-bound definition H. P-9 and G1 name the four hard-coded sites, and 07o gains the DEF-O-hashed preparation mutation, first failure G1 `RECEIPT_MISMATCH` (§2).
3. **N-4:** B3a's namespace predicate is type-strict. 07o gains `pressure_contract: false` on a 0.2.0 base, with the full falsy admission sets today: PY `{}`, `[]`, `""`, `0`, `false`; TS `""`, `0`, `false` (§3).
4. **N-5:** every B3b 07o first failure is restated with its gate and in-gate position. The G5c note is stated. The combination mutation's dependence on B2's G3 is stated. An S-C-only mutation is added (§4).
5. **N-6:** recommended. On the exact branch, G8 binds each evidence `pipe_materials[].G_pa` to the receipt's Ĝ bits, after S-C, with a "G_pa one ulp" entry (§5).
6. **N-7:** the results and stress-neutral `formulation_basis.profile_id` enums gain `exact_straight_retained_w1a_v2`, as `statics/r1/CARRIER_PROFILE_ENUMS.diff` (§6).
7. **N-8:** SCHEMA's diff now sets one title and one `$comment`. The J1 merged text is proposed. `OperandPreparation.definition_id` must never admit the exact id (§7).
8. **N-9:** TS's S-C exposure is exporting `validateAuthoredCaseFacts` unchanged. It already carries the setup (:301–328) and the thermal check (:358). PY also imports `_canonical_inputs` (§8).
9. **N-11:** B3b-P gains a pin for an exact invocation whose W1 ran and was abandoned: physics-1 plus the notice(s), accepted by physics-1's base readers in all three languages (§9).
10. **K3-2 and B3-K, as ruled:** option (a) with the three-line declared oracle exception; SA-2's and NA-6's K3-3 controls; NA-5's J2k acceptance; SA-1's re-base of B2-K; no kernel Ĝ check (§10).
11. **The other notes** (N-1 to N-3, N-10, N-12) are folded in without design change (§11).

**The estimates** move by +3–4 h for B3b, to 53–83 h agent; B3a's stays at 7–12 h (§12).

## 1. S-2: DEF-E's evidence wording, and the regeneration

### 1.1 The reworded members (verbatim, hashed)

| Path | Revision 01 text |
|---|---|
| `evidence.pipe_sections` | "in the owner case's contract_evidence.exact_cases entry (matched by load_case_id) only: regenerate pipe_sections[].As_m2,I_m4,J_m4,Z_m3 as the prepared A,I,J,Z bits; outside_diameter_m, effective_wall_thickness_m, ro_m, ri_m, Ai_m2, geometry_basis, pipe_id and order unchanged" |
| `evidence.pipe_stress_extrema` | "in the owner case's entry only: the maximum row family's eight regenerated numeric fields; every other key unchanged" |
| `evidence.unchanged` | "pressure [] and connector []; in the owner case's entry, load_case_id, profile_mode, material_basis, pipe_materials, pressure_rhs_assembly and stress_maximum_coverage (complete) byte-identical to the ordinary exact envelope; the entry of every case that is not selected byte-identical to the ordinary exact envelope; no recovery_method member" |
| `rows.maximum` | DEF-O's recipe, whose evidence clause now reads "regenerate the eight numeric fields of pipe_stress_extrema in the owner case's contract_evidence.exact_cases entry (matched by load_case_id)" (v0: "… of contract_evidence.exact_cases[].pipe_stress_extrema") |
| `scope.excludes` | `pressure_regions` → **`nonempty_pressure_regions`** (RV116's optional rename, adopted): beside `scope.pressure` ("explicitly_empty_pressure_regions_and_no_pressure_primitives") it no longer reads as excluding `[]` |

**Why "the owner case":**
- DEF-E, like DEF-O, is the formation of one product attempt, whose owner is one load case (`scope.owners: load_case`). So each selected case's attempt regenerates its own entry, and no other.
- With c ≤ 3 and several selected cases, each attempt regenerates its own owner's entry.
- An unselected case (`not_required` or `unavailable`) has no product formation to apply, and its entry is byte-identical to the ordinary envelope.
- The `[]` path syntax that read as "every case" is gone.

**Nothing else changed.** `paths_changed_from_v0` lists exactly these five paths (`b3d_statics_r1.out.json`). The preparation sub-object is still byte-identical to DEF-O's. DESIGN.md §1.4's meaning is unchanged; the static now says it.

### 1.2 Regeneration and controls

- **The generator** `b3d_statics_r1.py` imports the unchanged v0 generator and applies only §1.1's five edits, plus §6's and §7's diffs.
- **Its controls:** DEF-O's canonical form, DEF-O's pinned H `a7ed7ca0…0349`, and the v0 draft's raw sha256 and H (`6edae5ff…`, `9b66492e…`), all re-derived. v0's PTABLE control (`c74742ce…`) is unchanged, because the table function is v0's own.
- **The results:**
  - **DEF-E:** 9,733 B; raw sha256 `71f63d3916fa37ad0021ffb6ad993760a274166fe7ef275d7435c6856ed5642e`; **H `5a3bac430df9bbc77484d5419c75880ad40ae209b439e5f928374458025281af`**. ASCII only, no JSON floats.
  - **XTABLE:** 51,163 B; **sha256 `c4987e874889645ac315b5f55f58690082ad5e7745527f20e3e316efa3e70a3d`**. It differs from v0 only in `product_formation_definitions[0].sha256`. It round-trips through `json.dumps(indent=2, ensure_ascii=True)` plus a newline.
- **Collisions** at NUM `d867caa1ee` (outside `P/execution`): `nonempty_pressure_regions` 0 hits (2 inside, RV116's review); the three new hash prefixes 0 hits.
- **Where the new hashes go at J1:**
  - DEF-E's H goes into the readers' `EXACT_DEFINITION_HASH` and PP's exact definition constant;
  - XTABLE's hash goes into the readers' XTABLE constants (RS, PY `compatibility.py`, TS `numericalResultQuality.ts`), RE `semantic_contract.rs`, and lane T's carriers: `analysis_run.v0.3.schema.json`'s `SemanticContract.sha256` enum and its three branch consts, and `stress_neutral_export.v0.3.schema.json`'s branch const;
  - `REVIEWED_INPUTS` (B3D-18) takes the two new files' bytes.
- **Any later wording change regenerates both** with this script.

## 2. S-1: the preparation payload's definition hash is the route's

**The rule** (C3 §2: "definition_sha256: the table-bound H(definition)"):
- the preparation payload's `definition_sha256` is the H of the definition bound by the route's table;
- on the exact branch that is DEF-E's H (`5a3bac43…`), which G0 step 4 has already checked against XTABLE's `product_formation_definitions` and the reader's constant;
- on the preview branch it stays DEF-O's.

**Producer and readers take it from the route descriptor** (DESIGN.md §6.1), never from a module constant. **The four hard-coded sites today:**

| Site | Use | Constant |
|---|---|---|
| PP `retained_wire.rs:1306` (`preparation_payload`) | `"definition_sha256":DEFINITION_SHA256` | `:41–42` (and `:40` `DEFINITION_ID`, written into each attempt at `:1295`) |
| RS `retained_precision.rs:487` (`preparation_payload`), checked at G1 (`:612–625`) | `"definition_sha256":DEFINITION_HASH` | `:10–11` |
| PY `retained_precision.py:364` (`_preparation_payload`) | `"definition_sha256": DEFINITION_HASH` | `:22` |
| TS `retainedPrecision.ts:170` (`prepPayload`) | `definition_sha256: PREPARED_DEFINITION_HASH` | `:16` (and `:15` `PREPARED_DEFINITION_ID`) |

**DESIGN.md amended:**
- **§5 P-9** gains: "the attempt's `definition_id` (`retained_wire.rs:1295`) and the preparation payload's `definition_sha256` (`:1306`) are the route's: DEF-E's id and H on the exact route."
- **§6.2's G1 row** becomes: "SCHEMA (shared; `definition_id` enum); receipt and publication hashes; **the preparation hash, whose payload carries the route's definition H (RS `:487`, PY `:364`, TS `:170`), not DEF-O's** → `RECEIPT_MISMATCH`."

**07o** (in §4's table, entry 11):
- an exact-base mutation whose `CaseSource.preparation.sha256` is computed over a payload carrying DEF-O's H;
- the source identity, receipt and publication hashes are recomputed;
- expected first failure: **G1 `RETAINED_PRECISION_RECEIPT_MISMATCH` in all three**.
- **For SC2's writer:** the rehash tool must compute every other preparation hash with the route's H, or every exact base fails G1.

## 3. N-4: B3a's predicate, type-strict, and the falsy-value mutation

**Today's admissions of `model.pressure_contract`**, read from code:

| Reader | Predicate | Admits |
|---|---|---|
| RS `:3450` | `is_null()` | absent, `null` |
| PY `:1423` | `not model.get("pressure_contract")` | absent, `null`, `{}`, `[]`, `""`, `0`, `false` |
| TS `:1116` | `!model.pressure_contract` | absent, `null`, `undefined`, `""`, `0`, `false` |

**DESIGN.md §6.3's predicate, implemented type-strictly in all three:**
- **branch L** (0.1.0 or 0.2.0): the key is absent or its value is JSON `null`;
- **branch L3** (0.3.0): a JSON object with exactly the two keys `version` and `mode`, whose values are exactly the strings `"1.0.0"` and `"legacy_pressure_v1"`;
- **on the exact branch** (§4): the same shape with `"2.0.0"` and `"exact_straight_pressure_v2"`;
- anything else → G8 `INVOCATION_MISMATCH`.

**07o, B3a list, added:** on a 0.2.0 base, `pressure_contract: false` (invocation rehashed) → G8 `INVOCATION_MISMATCH` in all three. Today RS refuses it, while PY and TS admit it.
- **Optional siblings** of the same entry, for SC2: `[]` (PY only today), `""` and `0` (PY and TS).
- DESIGN.md's `{}` entry stays.

**As ruled (B3D-10):** the tightenings are adopted if the census over 07n shows zero changed outcomes; otherwise they are declared.

## 4. N-5: 07o's first failures, stated precisely

### 4.1 The G5c note

- **No reader implements D2 §4.9.3 G5c item 3**, the pressure condition: neither RS (`g5c`), PY nor TS. The exact branch **does not evaluate it either**.
- Under D1.5-exact it is vacuous: every case's `pressure_regions` is `[]`, and no primitive is a pressure load.
- G8 enforces `[]` per source.
- **So a region added to the invocation first fails at G8.** If a reader implemented item 3, a region naming a member with a maximum row would fail first at G5c `RETAINED_PRECISION_CLASSIFICATION_MISMATCH`, because the maximum would become `not_covered`. That would be a declared first-failure difference until all three implement it.

### 4.2 G8's order on the exact branch

Deterministic, first failure wins. It replaces DESIGN.md §6.2's two G8 rows where they differ.
1. **Invocation shape and hash; mode; project id.** Then the **namespace:** schema `0.3.0`; the exact contract, type-strict (§3); `combinations` empty; `components` empty; no `reference_configurations` → `INVOCATION_MISMATCH`.
2. **Ids** unique and non-empty, as preview → `PREPARATION_MISMATCH` (as every step below).
3. **Per case:** the selector must be `{kind: base}`; the ordinary attempt's mode and basis; the mode and parity rows.
4. **Material bases:** selector; material indices; per material, `elastic_modulus` = bits(E), `shear_modulus` = bits(e/(2·(1+ν))) and normal, `shear_origin` = `{derived_e_nu, bits(ν), homogeneous_isotropic_E_nu_v1}`, `selection` `{kind: base}`.
5. **S-C:** physics-source-1's `actual_materials` over every `exact_cases` entry; S-C's own code is detail only.
6. **N-6** (if adopted, §5): each entry's `pipe_materials[].G_pa` equals the receipt's `shear_modulus` for that material.
7. Topology, supports and stations, as preview.
8. **Per source:** owner and case binding; **`pressure_regions` present and `[]`**; `equivalent_static` absent; no `analysis_state`; maps.
9. **Members:** `geometry.route == "exact"`, OD, wall, radius, A/I/J; then loads, layout and C3's preparation binding, as preview.

### 4.3 The B3b 07o list, restated

Every entry is a rehashed mutation of the exact successor base, both modes. "Invocation" entries recompute the receipt's invocation value.

| # | Mutation | First failure (gate, code, where) |
|---|---|---|
| 1 | identity → the preview id | preview branch G0 step 1 (exact profile ≠ preview profile): `SOURCE_PRODUCER_CONTRACT_UNSUPPORTED` |
| 2 | profile → the preview profile | exact G0 step 1: same code |
| 3 | one attempt's `definition_id` → the ordinary id | exact G0 step 9: same code (G1 would admit it, but G0 runs first) |
| 4–8 | `projection_policy`, `work_policy`, `canonicalization`, `work.case_limit`, `work.invocation_limit` each changed | exact G0 step 8: same code (before G1's schema `const`) |
| 9 | the preview milestone successor relabelled physics-retained-1 (identity and profile) | exact G0 step 9 (its attempts carry the ordinary id): same code |
| 10 | the exact successor relabelled preview-physics-retained-1 | preview G0 step 9 (its attempts carry the exact id): same code |
| 11 | **S-1:** the preparation hash over a payload with DEF-O's H | **G1 `RETAINED_PRECISION_RECEIPT_MISMATCH`** (the preparation-hash check) |
| 12–14 | owner entry's `pipe_sections[m].As_m2`, `Z_m3` or `I_m4` one ulp | G5b evidence cross-check: `RETAINED_PRECISION_SECTION_MISMATCH` |
| 15 | owner entry's `pipe_sections[m].ro_m` one ulp | G5b (`actual_radius`): `SECTION_MISMATCH` (G7's ro = OD/2 would also refuse it, later) |
| 16 | `connector` non-empty | G7: physics-1's `CONNECTOR_UNSUPPORTED` (RS `SOURCE_PHYSICS_CONNECTOR_UNSUPPORTED`; PY and TS their own spellings) |
| 17 | an entry's `pipe_materials[m].G_pa` three ulps | G7: physics-1's `MATERIAL_G_BINDING` (> 2 ulps) |
| 18 | `recovery_method` added to an `exact_cases` entry | G7: physics-1's `CASE_SHAPE` |
| 19 | invocation contract → `{1.0.0, legacy_pressure_v1}` | G8 step 1: `INVOCATION_MISMATCH` |
| 20 | invocation schema → 0.4.0 | G8 step 1: `INVOCATION_MISMATCH` |
| 21 | invocation `pressure_contract: false` (N-4 on this branch) | G8 step 1: `INVOCATION_MISMATCH` |
| 22 | **a combination added to the invocation** | **depends on B2-C's gate placement.** If, as PLAN §1.2.5 puts it, G3 checks one coverage entry per model combination, the first failure is **G3 `RETAINED_PRECISION_COVERAGE_MISMATCH`** (the receipt's `combinations` is empty). Ruling 4's G8 refusal (step 1, `INVOCATION_MISMATCH`) is then pinned by a reader-local unit test of the exact branch's namespace function in each language. If B2-C keeps that coverage after G8, this entry expects G8 `INVOCATION_MISMATCH` directly. SC2 records which |
| 23 | a case naming `modulus_basis_ref` | G8 step 3 (selector not base): `PREPARATION_MISMATCH` |
| 24 | a material's `shear_origin` → `{kind: explicit_g}` | G8 step 4: `PREPARATION_MISMATCH` |
| 25 | a material's `shear_modulus` one ulp | G8 step 4: same |
| 26 | `shear_origin.poisson_ratio` bits changed | G8 step 4: same |
| 27 | authored ν changed in the invocation | **G8 step 4** (the receipt's ν no longer equals the authored ν), not S-C: same code |
| 28 | **S-C only:** an entry's `pipe_materials[m].nu` one ulp, every hash recomputed | G7 passes: with the committed ν = 0.25, 1 + ν still rounds to 1.25, so physics-1's G binding is unchanged; SC2 checks this on the actual base. Step 4 passes: the receipt and invocation are unchanged. **G8 step 5 (S-C, `ACTUAL_SELECTED_MATERIAL` as detail): `PREPARATION_MISMATCH`.** `pipe_materials` is never overlaid, so the entry may be the owner's or any other |
| 29 | **N-6:** an entry's `pipe_materials[m].G_pa` one ulp | G7 passes (≤ 2 ulps); S-C passes (it does not read G); **G8 step 6: `PREPARATION_MISMATCH`** (only if N-6 is adopted) |
| 30 | a case's `pressure_regions` → `null` | G8 step 8: `PREPARATION_MISMATCH` |
| 31 | a case's `pressure_regions` → one region | G8 step 8: `PREPARATION_MISMATCH` (§4.1's G5c note) |
| 32 | a member's `geometry.route` → `preview` | G8 step 9: `PREPARATION_MISMATCH` |

**A limit, stated (not a finding):** an unselected case's `exact_cases` entry is bound only by physics-1's base checks and S-C's OD, wall and E/ν checks, as for any physics-1 envelope. Its section values are not cross-checked, because no receipt section terms exist for it. A reader cannot prove "byte-identical to the ordinary envelope" without that envelope.

**The B3a list** is DESIGN.md §6.4's, plus §3's falsy entry.

## 5. N-6: binding the published G (recommended)

- **The check:** on the exact branch, G8 step 6 (§4.2) requires each `exact_cases[c].pipe_materials[m].G_pa` to equal, bit for bit, the receipt's `shear_modulus` for that `material_id` → `PREPARATION_MISMATCH`.
- **The 07o entry:** §4.3's entry 29; entry 17 (three ulps) stays at G7.
- **Why after S-C, not in G5b:** placed in G5b, it would pre-empt physics-1's own G7 refusal for entry 17. Placed in G8, the base validator keeps its 2-ulp tolerance as its own statement, and the stricter binding is the successor's.
- **Why recommend it:** S-C already binds E and ν exactly. Ĝ is a function of both, and the receipt's Ĝ is bound exactly (B3D-7). Leaving the one published G at physics-1's tolerance is the only inexactness left in the material evidence. It costs one loop in each reader, about 0.5 h across the three.
- **DEF-E's hashed text is not changed for it.** `trust.G8` already states the material binding at the receipt level, and no row depends on `G_pa`. So adopting or declining N-6 needs no re-hash.

## 6. N-7: the carrier profile enums

`statics/r1/CARRIER_PROFILE_ENUMS.diff` (generated; each file round-trips through its own JSON form, so each diff is exactly the enum change):

| File | Path | main sha256 | With the enum only |
|---|---|---|---|
| `P/schemas/results.v0.3.schema.yaml` (≈ :1004–1012) | `$defs/ResultEnvelope/properties/formulation_basis/properties/profile_id/enum` | `eb21b496…5eda` | `952b1deb71326dbeb2830e28e4e3c9d288fa313d5ee226c3f028a668f46e4232` |
| `P/schemas/stress_neutral_export.v0.3.schema.json` (≈ :360) | `properties/formulation_basis/properties/profile_id/enum` | `6355640a…7d06` | `05a588ed29110bff4ce65b90191f8544497805860e0bbdb9b555591fe6601b40` |

- Each enum gains `"exact_straight_retained_w1a_v2"` after `"product_preview_retained_w1a_v2"`.
- **Lane T owns both files** (REV §1.3). These diffs join DESIGN.md §7's list: the identity enums, the new `oneOf` branches and the hash constants are still lane T's, written at B3b-T.
- `analysis_run.v0.3.schema.json` has no profile enum.

## 7. N-8: SCHEMA's text and `OperandPreparation`

**B3b's draft diff** (`statics/r1/SCHEMA_ENUM.diff`, against main) now also sets:
- **`title`:** "Standalone ordinary prepared C1/C2/C3 receipt" → **"Standalone prepared C1/C2/C3 receipt"**;
- **`$comment`'s first sentence:** "Ordinary-prepared scope only; exact and prepared-combination admission require their later selected extension." → **"Prepared ordinary and exact formations, selected by ProductAttempt.definition_id; the exact route's constraints are reader gates (G0, G5b, G8), not schema branches; prepared-combination admission requires its later selected extension."** The rest of the comment is unchanged.
- **`ProductAttempt.definition_id`:** `const` → `enum [ordinary, exact]`.
- The draft is still a valid 2020-12 schema (`RUN_R1.md` R2).

**At J1, merged with B2-C** (I-A writes it; RV-Q2's round-1 completeness check). One title and one comment, for example:
- **`$comment`'s first sentence:** "Prepared ordinary, combination and exact formations, selected by definition_id; route constraints are reader gates (G0, G5b, G8), not schema branches."
- **`ProductAttempt.definition_id`:** `enum [ordinary, combination, exact]`.
- **`OperandPreparation.definition_id` (B2's):** never admits `RP-PREPARED-EXACT-DUAL-v1`. The exact route has no combinations (ruling 4), and KD §4 binds every operand source to DEF-O (`operand_definition`). So it is DEF-O's id, which B2-C states. PLAN §1.4 item 5's "and `OperandPreparation`'s" is withdrawn for the exact id.

## 8. N-9: S-C's exposure, per language

S-C's gate is unchanged: every physics-source-1 suite and corpus outcome stays identical in all three.

| Language | Exposure | What it carries |
|---|---|---|
| RS | `physics_source::actual_materials` (`:896`) → `pub(crate)` | Called per `exact_cases` entry with the request's case matched by `load_case_id`, as `physics_source::validate` does at `:1131–1138` |
| PY | import `_actual_materials` (`:409`) **and `_canonical_inputs`** (`:311`) | `_actual_materials` takes the canonical-input helper that `validate_physics_source` builds at `:433` |
| TS | **export `validateAuthoredCaseFacts` (`physicsSourceRecovery.ts:301`) unchanged** | It already contains: the setup (`:302–328`: request materials, the units engine, `convert`, `normalizeMaterial`, `normalized`, `validPair` at `:324–328`); the per-entry loop (`:329–360`); and the recovery-method thermal check (`:358`), which is vacuous on a successor because its evidence carries no `recovery_method`. The base call at `:294` is unchanged. It is async (the units engine), as the retained TS reader already is |

DESIGN.md §6.2's "the per-case loop … is extracted into an exported function" is replaced by this. No extraction is needed in TS: exporting the existing function carries N-9's setup and thermal check by construction.

## 9. N-11: the abandoned-W1 pin (to B3b-P; P-13 extended)

- **The input:** the exact successor input in both modes, with an existing fault hook armed after W1 work starts. For example, the serializer fault (`after_serialize`), the staging fault, or the precommit corruption (`rebind_next_precommit_invocation` or `corrupt`).
- **Expected output:**
  - the ordinary exact envelope byte for byte (physics-1), plus T-12's N1 notice (`RETAINED_PRECISION_UNAVAILABLE`, the fixed text) for each case in A, in request order;
  - with the receipt-encoding detail only for a serializer failure that C1:68 names.
- **Expected acceptance:** physics-1's base readers accept that envelope with its invocation, in RS, PY and TS.
- **This pin is new for physics-1:** T-12's base-reader test so far covers preview-physics-1 only. If a physics-1 base reader refuses the notice, that is a finding and a stop; it is not fixed by declaring it.
- **Estimate:** +0.5 h in B3b-P.

## 10. B3-K, as ruled (replaces DESIGN.md §8's open points)

- **K3-1:** `ProductMaterial::BaseENu { e, nu }` → `MaterialOperands::ExactENu { e, nu }`. `ProductMaterial` stays 96 bytes (NA-4). K-14's check runs at J2k.
- **K3-2 by option (a),** with **the declared exception, exactly:**
  - the generator's line 68 drops `if mode else (F(),F())`;
  - the fixture's line 22 becomes `("+ep2", "+fp2")` (the hull [7, 7.5]) and its line 122 `("+8p-1", "+8p0")` (the hull [0.5, 1]);
  - the fixture's sha256 moves from `8cbe5d32ea21a3ced7d5dc5cf9ccde256d01a6d546e94406a43a5d46b3a39a37` to `467f881181efb35ca11e8303dc212e3f90e00dd394571cb26f67b8504745f72c`, and the generator's from `88fc0765…` to `d9618a32…`;
  - **any other byte change in the oracle is a stop.**
  - Option (b) is rejected. `product_certificate_tests.rs`'s `unwrap_or` fallback stays unchanged (NA-6), to keep the exception minimal.
- **K3-3, with SA-2's and NA-6's controls:**
  - `ExactENu` with I_y ≠ I_z refuses with `AxisBits`, keeping its work prefix;
  - a `BaseENu` stress row and a maximum row certify in both lanes, and fail with `bad("represented Z")` under a test-only mutation restoring the material gate;
  - through the public variant: an E-bit mismatch gives `MaterialBits`, and ν outside (−1, ½) gives `InvalidMaterial`;
  - the six non-`ExactENu` oracle vectors keep their bytes and pass;
  - K3-3's own ν = 0.3125 control (it lived only in RV56's review fixture).
- **J2k's acceptance (NA-5):**
  - K3-3;
  - FK's suite with only the declared oracle lines changed;
  - PP's compile and suite;
  - every FK dependent compiled;
  - `profile_in_build_record` in the registered build.
- **SA-1:** B2-K's brief re-bases its stop S-11 and its byte-identity test K-13 on J2k's oracle.
- **NA-3:** no kernel check of Ĝ. G8 binds Ĝ, and the certificate refuses if α ≥ 1.
- **N-10:** B3-K's and B2-K's briefs cite RR "RV115's addendum accepts B3-K …" as the resolution of PLAN risks 4 and 9.
- **The estimate stands:** 3–6 h agent, RV-K 1–2 h.

## 11. The other notes

| Note | Effect |
|---|---|
| N-1 | DESIGN.md §13's "Ĝ's equality is sampled" limit is withdrawn. RV116 proves Ĝ = RN64(E/(2·RN64(1+ν))) for every normal Ĝ. P-3's second refusal clause stays as defensive code |
| N-2 | I95's zero rules C-1 and C-2 hold by RV116's code reading, and P-4 holds on main and on `b1`. RV-Q2 confirms G5's use at SQ2 (ruled) |
| N-3 | P-2's pin discriminates: n05's raw output publishes `work.limit = 8000000`. No change |
| N-10 | §10 |
| N-12 | `W1Route` is an internal identifier (0 hits) and needs no reservation. `SOURCE_PHYSICS_RETAINED_TABLE_{HASH,IDENTITY}` are packaging-defect strings, as the preview's are, so "no new failure code" holds |

**Decisions:** B3D-1 to B3D-18 are accepted as ruled. This revision adds no new decision. It records two choices of my own for RV116's confirmation:
- the wording of §1.1, which differs from RV116's illustration;
- §8's TS exposure by export rather than extraction.

## 12. Estimates, restated

| Slice | DESIGN.md | Revision 01 | Why |
|---|---|---|---|
| B3b-P | 14–22 | **15–23** | S-1's two producer sites; N-11's pin |
| B3b readers | 17–25 | **18–27** | S-1's route H at three sites; N-6's check; type-strict namespace |
| 07o (B3b part) | 3–5 | **4–6** | 32 entries (DESIGN.md: about 27) |
| Others (A, statics, K, T, SQ2) | unchanged | unchanged | N-7's diffs are generated; B3-K as ruled |
| **B3b total** | 50–79 | **53–83** | |
| B3a | 7–12 | **7–12** | N-4 adds about 0.5 h within the range |

Review is unchanged (17–27 h for B3b), plus RV116's confirmation of this revision, about 0.5–1 h.

## 13. Records, execution and limits

**The folder now holds:**

| Path | What |
|---|---|
| `DESIGN.md` | unchanged (`ad7942f6…b69b`) |
| `REVISION_01.md` | this file |
| `statics/r1/retained_precision_prepared_exact_v1.json` | DEF-E, revision 01 (`71f63d39…642e`) |
| `statics/r1/semantic_contract_v0_3_physics_retained_1.json` | XTABLE, revision 01 (`c4987e87…0a3d`) |
| `statics/r1/SCHEMA_ENUM.diff` | SCHEMA, B3b's change only, with N-8's title and comment |
| `statics/r1/CARRIER_PROFILE_ENUMS.diff` | N-7 |
| `statics/` (top level) | the three v0 drafts, unchanged at their original paths (`6edae5ff…`, `5bf0d0dc…`, `165c9d74…`) |
| `_run_records/revision_01/` | `b3d_statics_r1.py`, its output, `RUN_R1.md` |

**The seals:**
- **`SHA256SUMS.revision_01`** covers every file in the folder except itself, DESIGN.md and the original SHA256SUMS included.
- **The original `SHA256SUMS` verifies 11 of 11:** the v0 drafts were not moved, so every sealed record keeps verifying in the committed tree (RR's committed-tree rule; erratum E-7).

**Read for this revision:**
- RV116's REVIEW in full, and its S-2 illustration script;
- RV115's ADDENDUM_01 (SA-1, SA-2, NA-1 to NA-6);
- RR's two ruling sections;
- at NUM, the S-1 sites (PP `retained_wire.rs:38–44`, `:1295–1308`; RS `:478–490`, `:612–625`; PY `:360–368`; TS `:14–17`, `:166–172`);
- N-4's predicates (PY `:1423`, TS `:1116`, RS `:3450`), RS `g3`, `g4` and `g5b`;
- the S-C sites (RS `physics_source.rs:896`, `:1128–1140`; PY `physics_source.py:311`, `:409–436`; TS `physicsSourceRecovery.ts:290–360`);
- the carrier profile enums.

**Executed:** `RUN_R1.md`. The generator ran twice with byte-identical outputs; one `jsonschema` meta-validation; `git grep` and `git diff --quiet` reads.

**Limits:**
- **Nothing was compiled or run** beyond that Python and Git reads.
- **§4.3's first failures** are derived from RS's present gate order and from DESIGN.md's and this revision's exact-branch order. The three readers' runs at SC2 establish them.
- **Entry 22 depends on B2-C**, and entry 28 on the committed ν keeping physics-1's G binding.
- **B2-C is unwritten.** §7's merged SCHEMA text is a proposal for J1.
