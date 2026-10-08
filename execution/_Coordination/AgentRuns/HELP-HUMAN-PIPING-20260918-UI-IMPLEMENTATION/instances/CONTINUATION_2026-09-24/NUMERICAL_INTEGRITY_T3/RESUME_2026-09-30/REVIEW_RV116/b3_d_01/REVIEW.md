# RV116 (RV-D): independent review of I96's B3-D design (documents only)

TASK (Type 2), RV116, holding RV-D for B3, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I am a fresh instance, wrote none of the design, and made no delegation. 2026-10-07 UTC.

**The brief:** `R/BRIEFS/RV116_RVD_DESIGN_REVIEW.md`, sha256 `1bb840b636327bd85138822f5ece040f67e6666db657f2be829ce32cf2a07533`, verified before reading. I read NUM's `AGENTS.md` (`f96feb19…`) and `agents/AGENT_TASK.md` (`1a13a5b0…`) first.

**The subject:** `R/I96/b3_d_01/DESIGN.md`, sha256 `ad7942f66c906270003c7c7fd98bb7c64900183dc47b31796844fb051e37b69b` (verified); its SHA256SUMS 11 of 11 OK.

**Method.** Documents and code reading at NUM (HEAD `73b5d4e2a6` at dispatch, `d674763934` at the end, records only between them; `P/core`, `P/fixtures`, `P/schemas`, `P/apps` and `P/tests` equal main `2007709549` at both). Read-only Python with VENV in `WT/scratch/rv116_rvd/`: I96's generator, unchanged, and my own exact-arithmetic and hash checks. No cargo, native job, install or Git write; Git reads used `GIT_OPTIONAL_LOCKS=0`. `evidence/RUN.md` has the commands.

**Notation** is I96's (DESIGN.md header): PP, FK, FKR, FKT, RE, RS, PY, TS, PLAN, REV, KD, STUDY, DESIGN_v2, C2, C3, DN, D2, DEF-O, DEF-E, PTABLE, XTABLE, P1TABLE, SCHEMA.

## Verdict

**ACCEPT WITH AMENDMENTS.** **0 BLOCKING, 2 SHOULD-FIX, 12 NOTE.**

The design is sound, and its load-bearing claims reproduce independently:
- the generator rebuilds all three drafts byte for byte, DEF-E's H is `9b66492e…693d` by my own canonicalizer, and DEF-O's pinned `a7ed7ca0…0349` is the control;
- SourceAnnulus's published sections are not correctly rounded: one ulp in I, J and Z on the committed n05/n06 section, and 74 % of 3,000 random sections differ (I96: 75 % of 2,000);
- Ĝ equals binary64 `e/(2*(1+nu))` in every sampled case, and the equality is in fact provable for every normal Ĝ (N-1);
- I95's two zero rules hold by code reading (N-2), so B3b's admission can land at J2.

Two amendments are needed. S-2 changes hashed static text, so it should be settled before J1. S-1 is a producer and reader instruction, fixable at implementation.

## Findings

| ID | Severity | Where | Finding | Required change |
|---|---|---|---|---|
| **S-1** | SHOULD-FIX | §1.3 ("the preparation hash … unchanged"), §5 P-9, §6.1 ("G1 shared and unchanged"), §6.4 | **The preparation payload's `definition_sha256` is route-specific, and the design does not say so.** C3 §2 defines it as "the table-bound H(definition)", so on the exact route it is DEF-E's H. But every site hard-codes DEF-O's: PP `retained_wire.rs:41, :1306`; RS `retained_precision.rs:10, :487` (checked at G1, `:619`); PY `retained_precision.py:22, :364`; TS `retainedPrecision.ts:16, :170`. Implemented "unchanged", producer and readers agree with each other, every pin passes, and every exact preparation hash binds the ordinary definition. | **P-9 and §6.1/§6.2's G1 row:** the preparation payload's `definition_sha256` is the route's definition H (DEF-E's on the exact branch), in the producer and all three readers.<br>**07o:** add one exact-base mutation whose `CaseSource.preparation.sha256` is computed with DEF-O's H (source, receipt and publication hashes recomputed). Expected first failure: G1 `RECEIPT_MISMATCH` in all three. |
| **S-2** | SHOULD-FIX | DEF-E `evidence`, `rows.maximum` (hashed text); §1.4 | **DEF-E's `evidence` member does not scope the regeneration to the selected case.** It names `contract_evidence.exact_cases[]` and "each case's … `stress_maximum_coverage` (complete)". That reads as every case. §1.4 says the opposite: unselected cases keep their ordinary evidence, so their coverage need not be complete, and a mixed successor can carry two section values for one pipe. The static should say what the design means, because changing it after J1 is a re-registration. | **Reword `evidence.*` and the path in `rows.maximum`** to the owner case's `exact_cases` entry (by `load_case_id`). **State** that an unselected case's entry stays byte-identical to the ordinary exact envelope.<br>**Optional, same regeneration:** rename the `scope.excludes` token `pressure_regions` to `nonempty_pressure_regions`, which reads more clearly beside `scope.pressure`.<br>**Then I-A regenerates DEF-E's H and XTABLE's hash** with `b3d_statics.py` at J1. One verbatim wording is in §1.3 below. It gives H `6a45e9f0…50ff` and XTABLE `8b8743ff…d9e3` (`evidence/rv116_s2_illustration.out.json`; illustrative only). |
| N-1 | NOTE | §1.2, §13 limits, P-3 | **Ĝ = RN64(E/(2·RN64(1+ν))) holds for every normal Ĝ, not only the sampled cases** (proof in §1.2 below). My exact-rational model of `Scaled` gives 250,009 equal and 0 different, with 8 pairs non-normal on both sides (`evidence/rv116_checks.out.json`). So P-3's second refusal clause is unreachable. Keep it as defensive code. | None. The design's "sampled" limit can be withdrawn |
| N-2 | NOTE | §4.4 ruling 2; §5 P-4; I95 C-1, C-2 | **I95's two zero rules hold by code reading** (§5 below):<br>• **C-1:** under `Some([])`, only `EXACT_PRESSURE_MATERIAL_AMBIGUOUS` can fire in `build_pressure_case_with_members`, at most once per material;<br>• **C-2:** `finish_source_groups` fires nothing.<br>**P-4 holds on main and on `b1` `603e238517`:** no `retained_*.rs` file calls a pressure-runtime builder; they reference only `is_exact`. Without these credits the exact route needs 11.25 GiB (I95's er: X1 10,427,541,576 B, above 0.9 M at 10.5 GiB). So B3D-17 rests on them. | None. RV-Q2 still checks G5's use of the rules at SQ2 |
| N-3 | NOTE | §5 P-2 | **P-2 is real, and its pin discriminates.** `permitted_run` builds `SourceRecoveryBudget::default()` (4,000,000 per case) on main (PP `lib.rs:2990`) and on `b1` (`:2995`). `ordinary_dispatch` gives the exact route 8,000,000 (`lib.rs:2307–2310`). physics-source-1 publishes the limit (`source_block_recovery.body.cases[].work.limit = 8000000` in n05's raw output), so the n05/n06 Direct-against-ordinary byte pins fail if P-2 is missing. For pre-0.4 models, the two entries differ only in this budget and the passive observer (`run_linear_static_preview_captured` reduces to `observed(…, None)`). Work limits are operation counts, and I95's memory chain does not read them. | None |
| N-4 | NOTE | §6.3, §6.4 (B3a) | **The latent first-failure difference is wider than PY's `{}`.**<br>• PY's `not model.get("pressure_contract")` (`retained_precision.py:1423`) admits `{}`, `[]`, `""`, `0` and `false`.<br>• TS's `!model.pressure_contract` (`retainedPrecision.ts:1116`) admits `""`, `0` and `false`.<br>• RS (`is_null()`, `retained_precision.rs:3450`) admits none of them.<br>The proposed predicate closes all of these if each language implements it type-strictly. | Add a falsy-scalar mutation (for example `pressure_contract: false` on a 0.2.0 base) to the B3a 07o list, expected `INVOCATION_MISMATCH` in all three |
| N-5 | NOTE | §3 (G5c), §6.4 (B3b G8 row) | **Two 07o first failures need a sharper statement.**<br>• **(a) "regions with one region":** G8 is the first failure only because no reader implements D2 §4.9.3 G5c item 3's pressure condition; none does today (RS `g5c`, PY, TS). If one did, a region naming a member with a maximum row would fail first at G5c `CLASSIFICATION_MISMATCH`.<br>• **(b) "authored ν changed in the invocation (S-C)":** this first fails at G8's material-base binding (`shear_origin.poisson_ratio`), not in S-C, so it does not exercise S-C. | **(a)** State that the exact branch does not evaluate item 3: it is vacuous under D1.5-exact, and G8 enforces `[]`.<br>**(b)** Add an S-C-discriminating entry: an unselected case's evidence `pipe_materials[].nu` changed by one ulp. G7's 2-ulp G tolerance passes it, and the material base never reads it, so only S-C refuses it (G8 `PREPARATION_MISMATCH`) |
| N-6 | NOTE | §6.2 G8, B3D-7 | **The evidence's `pipe_materials[].G_pa` is held only to physics-1's 2-ulp tolerance.** G8 binds the receipt's `shear_modulus` exactly, and S-C checks E and ν but not G. So a 1–2-ulp edit of the published G passes every gate. No row depends on it, because rows use the receipt's Ĝ. | **Optional:** on the exact branch, require each `exact_cases[c].pipe_materials[m].G_pa` to equal the receipt's `shear_modulus` bits for that material, with a 07o "G_pa one ulp" entry |
| N-7 | NOTE | §7 | **Two carrier sites are not listed.** The top-level `formulation_basis.profile_id` enums of `results.v0.3.schema.yaml` (about :1004–1012) and `stress_neutral_export.v0.3.schema.json` (about :360) need `exact_straight_retained_w1a_v2`. §7 lists only the identity enums. Lane T's schema tests would catch it. | Add both to §7's list (lane T) |
| N-8 | NOTE | `statics/SCHEMA_ENUM.diff`, B3D-9 | **Two SCHEMA points for J1.**<br>• **The text:** B2-C also rewrites `$comment`'s first sentence, and SCHEMA's `title` still says "Standalone ordinary prepared C1/C2/C3 receipt". J1's merge should leave one coherent comment and title.<br>• **The narrowing:** `OperandPreparation.definition_id` (B2's) should not admit the exact id, because the exact route has no combinations (ruling 4). PLAN §1.4 item 5 had named both enums. | State it at J1 (I-A; RV-Q2's completeness check) |
| N-9 | NOTE | §6.2 S-C (B3D-12) | **S-C is minimal, and in RS and PY it cannot change an outcome:** RS makes `actual_materials` (`physics_source.rs:896`) `pub(crate)`, and PY imports `_actual_materials` (`physics_source.py:409`).<br>**In TS the extracted function has more to carry.** It must take the setup above the loop (`normalized` and `validPair`, `physicsSourceRecovery.ts:320–328`) and the recovery-method thermal check (:358), and stay called from the same place. | S-C's gate as stated: physics-source-1 suites and corpus identical in all three |
| N-10 | NOTE | §8; PLAN §6 risks 4 and 9 | **Two of PLAN's stop rules fired.** Risk 4's last sentence ("An FK need beyond an annulus version is a stop") fired for K3-1 and K3-2, and risk 9 for the oracle change. ROOT's ruling on RV115's addendum (B3D-6, B3D-16) resolves both. | Cite that ruling as the stop's resolution in B3-K's and B2-K's briefs |
| N-11 | NOTE | §5 P-13 | **No pin covers an exact invocation where W1 ran and was abandoned.** Such an invocation publishes physics-1 plus the N1 notice (`RETAINED_PRECISION_UNAVAILABLE`), and that envelope must still pass physics-1's base readers in all three languages. P-13 pins refusals, which take plain ordinary bytes, but not this case. | Add one pin using an existing fault hook |
| N-12 | NOTE | §5 P-1, §6.4, §10 | **Two small points:**<br>• `W1Route` (P-1) is not in §10. It is an internal identifier with 0 hits, so it needs no reservation.<br>• The RS table-pin strings `SOURCE_PHYSICS_RETAINED_TABLE_{HASH,IDENTITY}` are reached only by a packaging defect (`expect` at init), as the preview's are. They are not document first-failure codes, so "no new failure code" holds. | None |

## 1. The definition `RP-PREPARED-EXACT-DUAL-v1`

### 1.1 Is preparing on the exact route necessary (B3D-3, B3D-4)? Yes

**Reproduced independently** (`evidence/rv116_checks.py`, written fresh):
- **SourceAnnulus in plain binary64** (in the normal range each `Scaled` step is the binary64 step) reproduces all 18 published `pipe_sections` in the committed physics-source raw outputs, bit for bit. All 18 share D = 0.2 m and t = 0.01 m (n05, n06 and the other fixtures).
- **The correctly rounded annulus** uses my own π enclosure (Machin with integer fixed point and explicit error bounds, width < 2^-1300). That is a different construction from I41's and DEF-O's limbs. No rounding was ambiguous.

| D = 0.2, t = 0.01 | A | I | J | Z |
|---|---|---|---|---|
| SourceAnnulus (published) | `3f7872fa3a37ac13` | `3efc52664442210b` | `3f0c52664442210b` | `3f31b37feaa954a7` |
| Prepared (correctly rounded) | `3f7872fa3a37ac13` | `3efc52664442210a` | `3f0c52664442210a` | `3f31b37feaa954a6` |
| Preview `derive_pipe_section` | `3f7872fa3a37ac15` | `3efc52664442210e` | `3f0c52664442210e` | `3f31b37feaa954a8` |

- **I, J and Z each differ by one ulp; A is equal.** I96's three J representations (`…210e`, `…210b`, `…210a`) are confirmed. The prepared Z is FK's final-case test fixture's `0x3f31b37feaa954a6` (FKT `product_final_case_tests.rs:400`).
- **A random sweep with a different generator and seed** (D log-uniform in [0.01, 3] m, t/D in [0.005, 0.25], 3,000 sections) gives 2,228 sections that differ in at least one property (74 %): A 1,161, I 1,634, J 1,634, Z 1,668. The largest distance is 2 ulps in A and 3 in I, J and Z, with 0 ambiguous. That agrees with I96's 75 %.

**Why prepare.** Three reasons, beyond the reproduced bits:
- **C3's receipt shape already requires a preparation stage.** `ProductAttempt.preparation`, `PreparedMember` and the preparation hash all assume one. An unprepared exact formation would need a new receipt branch and new G5 and G8 paths.
- **SourceAnnulus's errors are the RV66 class** that I51 removed on the preview route, and the committed exact pair (n05 and n06) is a torsion pair.
- **"Source OD/effective wall define the section"** is physics-1's own first limitation. The correctly rounded annulus is the faithful realization of it.

**I agree with B3D-3.** KD's and RV115's "no exact annulus version" also holds: `prepare_product_annulus(D, t_eff)` takes exactly the exact route's normalized OD and effective wall (`build_model`, PP `lib.rs:7135–7155`).

### 1.2 Ĝ

The derived G of the producer and the readers' expression are identical for every normal Ĝ (N-1). The producer's path is `IsotropicENu::new`, PP `pressure_exact.rs:237–253`, with `Scaled` at `:13–158`.
- **`1 + ν`.** `Scaled(1)` has mantissa ½ and exponent 1. For |ν| < 1 the add aligns ν's mantissa to exponent 1 and rounds once in binary64. That gives RN64(1+ν)/2 at exponent 1, which is exact as a scaled pair because the sum is near ½.
  - When |ν| < 2^-1021 the aligned term may round to a subnormal first. That cannot change the result, because RN64(1+ν) = 1 already for |ν| < 2^-54.
- **The doubling** (`Scaled(2)·`) is a mantissa product by ½ with no rounding, so the denominator is exactly 2·RN64(1+ν), which is binary64's `2.0*(1.0+nu)`.
- **The quotient.** It divides the two mantissas (each in [½, 1)) in binary64, so the result rounds to 53 significant bits once. Exponents are carried separately.
  - `to_f64` multiplies by a power of two, which is exact whenever the result is normal. So Ĝ = RN64(E/(2·RN64(1+ν))) = binary64 `e/(2*(1+nu))` for every normal Ĝ.
  - For a subnormal Ĝ the two can differ, by double rounding. P-3 already refuses that case.

My exact-rational model of this path (RN53 with unbounded exponent, then the final conversion) agrees with binary64 on 250,009 normal cases. These are random E in [1e-300, 1e307] and ν in (−1, ½), ν drawn by bits, and 17 edges: subnormal E, ν at −1 + ulp, at ½ − ulp, at ±5e-324 and at ±2^-53, and E at the maximum. Eight cases are non-normal on both sides. 0 differ.

**B3D-7 (exact bits in G8) is right**, and stricter than physics-1's 2-ulp `MATERIAL_G_BINDING` (RE `physics_evidence.rs:103–124`). N-6 covers the one published G it leaves at tolerance.

### 1.3 The new `evidence` member against D2 G5b

**Sound in substance.** D2 §4.9.3 G5b cross-checks receipt section terms against "the exact route's `contract_evidence.exact_cases[].pipe_sections`: A_s and Z, bit for bit". The selected rows are computed from the prepared section, so the published evidence has to state it.
- **The design's G5b also binds the geometry fields:** I_m4, J_m4, `ro_m`, OD and wall. That is a consistent widening: `ro_m` = OD·0.5 equals the prepared c bits (`3fb999999999999a`).
- **physics-1's base validator is unaffected.** It checks only positivity and the ro/ri relation (RE `physics_evidence.rs:137–155`). Its other cross-case checks compare member coverage, and the extrema bounds and headline, never section values. The only section-value comparison is region geometry against `pipe_sections` (`:1085–1105`), and it is vacuous with no regions.
- **The preview overlay has no section evidence to rewrite** (`preview_cases` has none; PP `retained_product.rs:2010–2020`), so the `pipe_sections` overlay is genuinely new. P-8 and P-12's hook cover it.
- **The exact route's ordinary extrema have the same 13-key shape as the preview route's** (`physics_evidence.rs:453–509`). So "the eight regenerated numeric fields" are `PREPARED_MAX_KEYS` (PP `retained_product.rs:3475–3476`), and `EXTREMA_BOUNDS` holds by construction.

**S-2: the static's wording.** These are the texts I used for the illustration. Their meaning is §1.4's; the final wording is I-A's.
- `evidence.pipe_sections`: "for the owner case's contract_evidence.exact_cases entry (matched by load_case_id): regenerate pipe_sections[].As_m2,I_m4,J_m4,Z_m3 as the prepared A,I,J,Z bits; outside_diameter_m, effective_wall_thickness_m, ro_m, ri_m, Ai_m2, geometry_basis, pipe_id and order unchanged".
- `evidence.pipe_stress_extrema`: "for the owner case's entry: the maximum row family's eight regenerated numeric fields; every other key unchanged".
- `evidence.unchanged`: "pressure [], connector [], and the owner case's load_case_id, profile_mode, material_basis, pipe_materials, pressure_rhs_assembly and stress_maximum_coverage (complete) byte-identical to the ordinary exact envelope; an unselected case's exact_cases entry is byte-identical to the ordinary exact envelope; no recovery_method member".
- `rows.maximum`: "… of the owner case's contract_evidence.exact_cases[].pipe_stress_extrema …".
- **Optionally,** `scope.excludes`: `nonempty_pressure_regions`.

Adopted verbatim, these give 9,680 B, raw sha256 `efbf221f…6705`, H `6a45e9f09c72f144249a2bc6f491cb071356e9ad11620ca1db55714751fb50ff`, and XTABLE sha256 `8b8743ff9aee5b8872e4ec9f3deeaa17f55b93ceb6ca2dd0353640d196b9d9e3`.

### 1.4 Regeneration and H

`b3d_statics.py`, run unchanged in scratch, rebuilds all four outputs byte for byte (`evidence/regeneration.txt`):
- the definition `6edae5ff…3879`;
- the table `5bf0d0dc…b5c7`;
- `SCHEMA_ENUM.diff` `165c9d74…99ba`;
- the out file `8eed3327…a98`.

It also reproduces its own controls: DEF-O canonical, DEF-O's H the pinned `a7ed7ca0…0349`, and PTABLE rebuilt to `c74742ce…`.

**My own canonicalizer** sorts keys by UTF-16 code units, uses JCS string escaping and admits integers only, and is independent of the generator's `json.dumps`. It gives:
- **H(`retained_precision_formation_v1`, DEF-E) = `9b66492e56a25a10939973a9ceb060a538769b0cd8f0da52962d5ec90f68693d`**, as claimed;
- DEF-O's pinned H as the control;
- DEF-E's raw bytes equal to their canonical form.

**What differs from DEF-O.** DEF-E's preparation sub-object equals DEF-O's. Its only new top-level member is `evidence`, and it differs from DEF-O at 14 member paths.

No reader or producer parses a definition's members; they only hash it (grep of RS, PY, TS and PP). So the new member has no parse consequence.

## 2. The table (`5bf0d0dc…`)

- **The construction re-runs.** The PTABLE control reproduces `c74742ce…` from preview-physics-1's table.
  - P1TABLE round-trips byte-identically through `json.dumps(indent=2, ensure_ascii=True)` plus a newline.
  - XTABLE differs from P1TABLE in exactly three changed lines (identity, inherited hash, profile) plus the appended block. It is ASCII, and its four JSON floats are P1TABLE's own.
- **The changed and appended members** are as stated:
  - changed: `semantic_contract_id`, `inherited_semantic_contract_sha256` = `9a2cf6268b57bd5265a1a115497c07450819dd4d03cd5ab618097bd9d19da8cc` (the raw sha256 of P1TABLE, verified), and `formulation_profile_id`;
  - appended: `receipt_policy`, `receipt_schema`, `product_formation_definitions`, `accuracy_classification`, `formation_warrant` and `receipt_bindings`.
  
  `receipt_policy`, `receipt_schema` and `formation_warrant`'s clauses equal PTABLE's. `accuracy_classification` equals PTABLE's except `scope`, and `formation_warrant` names the exact id.
- **`reserved_inactive_successors` is P1TABLE's,** following the retained precedent of DESIGN_v2 decision 14. No runtime code reads it (`git grep` over `P/core` and `P/apps/desktop/src`: no runtime use).
- **`receipt_bindings` binds exactly RV78-N1's set** (REV §2): the projection and work policies, the 20B and 60B limits, the method token and the canonicalization.
  - Each key matches the receipt body's path and the body's SCHEMA `const` (`Body.projection_policy`, `work_policy`, `work.case_limit`, `work.invocation_limit`, `canonicalization`).
  - `method` matches `Case.method`'s `const` and the rows' `recovery_method`.
  - The limits are exact integers below 2^53.
- **G0** (§2.4) reads the table with the constants as a cross-check, as decision 31 and N-12 require:
  - its step order mirrors RS's present G0 (`retained_precision.rs:509–566`);
  - step 6 goes beyond REV's list, cross-checking `receipt_policy` and `accuracy_classification.policy` too.
  
  **The one open item is the shared spelling of `receipt_bindings` with B2-C.** B2-C has not been dispatched (RR: the next unused id is I97), so it belongs in B2-C's brief (B3D-8).

## 3. The D1 texts

**B3a's D1.3 (branch L3) and N-11's reading: agreed.**
- **The producer facts hold:**
  - `validate_profile`'s 0.3.0 arm admits exactly the two contracts (PP `pressure_runtime.rs:128–145`);
  - `PressureContractInput` denies unknown fields (`:25–30`);
  - `is_exact` and `is_load_state` are false on L3;
  - `mechanics_producer_for_model` gives preview-physics-1 (PP `lib.rs:990–998`);
  - `source_recovery` refuses the legacy namespace for 0.3.0 (`source_recovery.rs:604–608`), so T-3 (c) cannot fire.
- **N-11's reading** matches DN §4.3, "0.3.0 `legacy_pressure_v1` with zero pressure". A zero-magnitude legacy pressure load is refused by D1.7 (`LoadTarget` or `LoadDimension`) and published ordinarily. A zero-valued node-targeted force labelled `pressure` stays a nodal term (`validate_profile` refuses it only when non-zero, `:213–226`).

**B3b's D1.3, D1.4 and D1.5 (branch E): agreed.**
- **No combination on the exact route** (ruling 4, which `validate_profile` mirrors, `:201–204`).
- **c ≤ 3 with no route cap** (ruling 3).
- **`pressure_regions == Some([])`:** a `None` is also blocked by the ordinary route (`EXACT_PRESSURE_REGIONS_REQUIRED`, `:242–245`).
- **Every case's E/ν is the base selection,** as B1's D1.5 already keeps for every route (DESIGN_v2 decision 7).
- **Every refusal uses an existing `FamilyFact`** (`b1` `retained_memory.rs:573–595`): `SchemaVersion`, `PressureContract`, `Combinations`, `Components`, `PressureRegions`, `EquivalentStatic`, `ModulusBasisRef`, `ModulusBasisTemperature`, `AnalysisState`, `LoadTarget` and `LoadDimension`.

## 4. The readers' `<physics-retained>` branch

- **Dispatch and gates.** One dispatch on the identity, with route-specific functions behind their own call edges. G0 to G8 keep RS's present order (`validate`, `retained_precision.rs:4272–4322`).
  - **No new document failure code** (N-12 on the two packaging strings).
  - **G7's projection** mirrors the preview's (`project`, `:4246–4266`), with physics-1 and `exact_straight_pressure_v2`.
- **G8's predicates are right:**
  - the exact namespace;
  - combinations as `INVOCATION_MISMATCH`;
  - `[]` per source;
  - `derived_e_nu` with the exact Ĝ bits;
  - `geometry.route == exact`;
  - S-C over every `exact_cases` entry.
- **First failures in §6.4.** The ones I checked against the gate code agree, except N-5's two:
  - B3a: contract mode, version, removal, extra key, 0.2.0 with a contract, and the element pressure load;
  - G0: identity, profile, every bound member, both relabels (step 9);
  - G5b: the A, Z, I and `ro_m` ulps;
  - G7: connector, G_pa three ulps, case `recovery_method`;
  - G8: namespace edits, `shear_origin`, `shear_modulus`, receipt ν, `geometry.route`, `modulus_basis_ref`.
  
  S-1 adds one entry, and N-4 and N-6 suggest two more.
- **B3a's G8 change** closes PY's `{}` difference, and N-4's wider set too. 07m has only 0.1.0 and 0.2.0 invocations with no contract (corpus census: 15 and 2), so the tightening should census clean on 07n.
- **S-C's minimal exposure** is the minimal change. In RS and PY it cannot change any base outcome; for TS see N-9. It touches base-reader files, so ROOT rules it (B3D-12).

## 5. The producer requirements

- **P-2:** real and pinned (N-3). Physics-source-1 can select differently on the Direct entry today only through this budget. With P-2 the two entries match for pre-0.4 models.
- **P-4 and I95's two zero rules,** checked by code reading as RV83 and RV84 checked G4's:
  - **C-1** (`build_pressure_case_with_members` → `problem` = mats):
    - with `pressure_regions == Some([])`, `REGIONS_REQUIRED` cannot fire (`pressure_runtime.rs:439–447`);
    - the region loop is empty (`:474–747`);
    - every cap and eigen term list is empty, so `exact_sum` returns `Ok(+0.0)`. `exact_rounded_sum` over no values projects a zero accumulator to `Ok(0.0)` (FK `exact_sum.rs:160–163, :303–309`). So the per-DOF `_ => problem` (`:748–763`) cannot run;
    - all vectors are +0.0, so the non-finite site (`:766–778`) cannot run;
    - only `EXACT_PRESSURE_MATERIAL_AMBIGUOUS` (`:461–468`) can fire, at most once per duplicate material id.
  - **C-2** (`finish_source_groups` → `problem` = 0):
    - `groups` is empty, so both in-loop sites (`:845–885`) cannot run;
    - every `by_dof` list is empty, so `exact_sum` returns `Ok(0.0)` and the per-DOF site (`:886–895`) cannot run;
    - `max_rhs` = 0 and every list is empty, so `maximum_ratio` = 0, the screen is 0, and `CANCELLATION_UNRESOLVED` (`:922–925`) cannot run.
  - **P-4 itself:** the retained modules call no pressure builder (`retained_product.rs:357` reads `is_exact` only; the same on `b1`). The builders run only in the ordinary run, once per case, which C-1 and C-2 price.
- **P-1, P-3, P-5 to P-13: agreed,** with S-1 for P-9 and N-11 for P-13.

## 6. B3-K and J2k

**RV115's addendum and ROOT's ruling settle the kernel content:** K3-1 and K3-2 by option (a), the declared three-line oracle exception, and J2k as its own merge point.

**As B3 needs it, I agree:**
- K3-2 is necessary. The K lane needs `represented_z` for every `Stress` and `CircularMaximum` row (FKR `final_case.rs:748–752`), and `build_member` gates it on the material (FKR `product_certificate.rs:544, 582–592`).
- The oracle's mode-0 hulls are [7, 7.5] and [0.5, 1]. I checked them against the generator's inputs (FKT `product_certificate_vectors.py:39, 44, 68`).
- DEF-E's `rows.component_stress` ("admitted-K bending Z is hull(actual Z, I_K/c)") then holds on the exact route.

N-10 records the PLAN stop rule this resolves.

## 7. The decisions B3D-1 to B3D-18

| # | Decision | RV116 | Reason |
|---|---|---|---|
| B3D-1 | A new definition `RP-PREPARED-EXACT-DUAL-v1` | **AGREE** | DEF-O's hash and its cascade stay put. The scopes differ in materials, source and evidence |
| B3D-2 | Reuse `retained_precision_formation_v1` | **AGREE** | The domain names the object type; the payloads differ. Consistent with R-10 for DEF-C |
| B3D-3 | Prepare with the identical `RP-PREPARED-ANNULUS-v1` | **AGREE** | §1.1: the bits reproduced, C3's shape kept, the RV66 class removed |
| B3D-4 | Regenerate the selected case's section evidence and extrema | **AGREE** | D2 G5b requires it. The static's wording needs S-2 |
| B3D-5 | The base common E/ν only | **AGREE** | It equals B1's D1.5 domain (DESIGN_v2 decision 7). Points and interpolation are a later version |
| B3D-6 | B3-K with K3-2 and the declared exception | **AGREE** | Already ruled (RV115 addendum). N-10 |
| B3D-7 | G8 binds Ĝ exactly | **AGREE** | Provable for every normal Ĝ (N-1). N-6 is optional |
| B3D-8 | `receipt_bindings` in XTABLE v1 and in PTABLE's B2 revision | **AGREE** | Path-for-path with the body. ROOT must put the spelling in B2-C's brief |
| B3D-9 | One SCHEMA with a `definition_id` enum | **AGREE** | N-8: the comment and title at J1; `OperandPreparation` without the exact id |
| B3D-10 | The exact namespace predicate in all three readers | **AGREE** | Add N-4's falsy-scalar mutation; census 07n first |
| B3D-11 | Keep the legacy `pressure_regions: []` leniency | **AGREE** | The producer cannot emit `Some` on L or L3. A tightening would buy no reliance |
| B3D-12 | S-C's minimal exposure | **AGREE** | Minimal, and outcome-preserving in RS and PY; in TS with N-9's care |
| B3D-13 | S-C failure as `PREPARATION_MISMATCH` with detail | **AGREE** | Avoids three spellings of S-C's codes; keeps first-failure parity |
| B3D-14 | `retained_physics` and `N_RETAINED_PHYSICS_OUTPUT` | **AGREE** | Follows `retained_preview_physics`. Avoids T6S's `physics_retained` example (`outputPolicy.test.ts:74–75`) |
| B3D-15 | The results carrier's `limitations` pinned as `const` | **AGREE** | As the preview branch pins its seven strings (`results.v0.3.schema.yaml:1675–1686`). Add N-7's profile enums |
| B3D-16 | J2k for B3-K | **AGREE** | Already ruled |
| B3D-17 | B3b's admission at J2 under the interim | **AGREE** | §8 below |
| B3D-18 | Append DEF-C, DEF-E and XTABLE (14 → 17) | **AGREE** | Existing positions keep their meaning |

**The names (§10)** recheck clean (`evidence/collisions.txt`, at `d674763934`, outside `P/execution`): every new name has 0 maintained hits, except the substrings and existing keys I96 lists. That includes the full identity and profile, the definition, the file names, `receipt_bindings`, every DEF-E token, `retained_physics`, `BaseENu` and the code constants. DEF-E's H and XTABLE's hash appear nowhere.

## 8. B3b's admission timing (B3D-17): recommend J2, under the interim

- **REV §1.4's condition is met.** B3-S shows the fit, and the fit rests on the two zero rules, which hold (N-2).
  - **The interim's under-pricing:** S3's profile under-prices the exact route at C = 3 by 152,426,210 B (9,900,151,888 against 9,747,725,678).
  - **The margin:** the real need stays 246,708,348 B under 0.9 M at 10.5 GiB (10,146,860,236 B).
- **No product caller exists,** nothing reaches NUM before SQ2, and SQ2's per-route registration supersedes the interim, as at I2.
- **Waiting would invert the order.** B3b-P's must-pass successor and coexistence pins (P-2, P-13) need a permitted Direct call. So waiting for SQ2 would put SQ2 before the code it registers is tested.
- **Conditions:**
  - RR records the 152 MB disclosure in the J2 interim entry;
  - RV-Q2's J2 round reads B3b-A's clauses;
  - RV-Q2 at SQ2 confirms G5's use of C-1 and C-2.

## 9. What ROOT must rule on

1. **The amendments:**
   - **S-1:** the route's definition H in the preparation payload, at four sites, plus one 07o mutation;
   - **S-2:** DEF-E's `evidence` and `rows.maximum` wording, scoped to the owner case, with H and XTABLE regenerated by I-A at J1. The illustrative hashes are `6a45e9f0…` and `8b8743ff…`.
2. **B3D-1 to B3D-18** (6 and 16 are already ruled). RV116 agrees with all eighteen.
3. **B3D-8:** put `receipt_bindings`'s name and shape into B2-C's brief.
4. **P-2** into B3b-P's brief, with the n05 and n06 Direct-against-ordinary byte pins. They discriminate (N-3).
5. **B3D-17:** J2 under the interim, with the 152 MB disclosure, as recommended.
   - Ruling 2's review is satisfied at code-reading level (N-2).
   - RV-Q2 confirms G5's use at SQ2.
6. **B3D-10's tightenings,** with N-4's mutation, if the 07n census is clean.
7. **B3D-12:** S-C's exposure in the base `physics_source` readers.
8. **Optional NOTEs to adopt:** N-5, N-6, N-7, N-8, N-9 and N-11 (briefs for lanes A, P, T, the readers and SC2), and N-10 (the stop's resolution, cited in the B2-K and B3-K briefs).

## 10. What I read, execution record and limits

**Read** (sha256 prefixes; placeholders):

| Input | sha256 |
|---|---|
| The brief; I96's brief | `1bb840b636327bd8`; `08b34b12833157ed` |
| DESIGN.md (whole), its statics and `_run_records/` (SHA256SUMS 11 of 11) | `ad7942f66c906270` |
| PLAN (§1.3, §1.4, §5, §6); REV (whole) | `e1147dbda247c470`; `63abb73fe85c8dbc` |
| RV114's REVIEW (as cited); RV115's REVIEW (as cited) and ADDENDUM_01 (whole) | `bc6918ce474a188d`; `0f7ab77d37b0ec45`; `501246079018ddbd` |
| KD (§8, §10); STUDY (§0, §2, ruling inputs) | `4e8c33a45616d4f0`; `8975948a1d6f919e` |
| DESIGN_v2 (§5, §8); C2 (§3); C3 (§1–§2) | `5933b90b8c323199`; `923da0b97eb5beca`; `fd00d2c1e8f4f7e2` |
| DN (§4.3); D2 (§4.9) | `fb62ef4a8282e53f`; `993f5f3ab4bd768b` |
| RR (the sections from "B2/B3's plan dispatched as I93" to "RV115's addendum accepts B3-K"), at `d674763934` | `7e367a8704ddc05a` |
| DEF-O; PTABLE; P1TABLE; preview-physics-1's table; SCHEMA; CORPUS 07m | `3e0779a45a74cf0b`; `c74742ce6a936384`; `9a2cf6268b57bd52`; `ae55503d44a47507`; `07951edacfedd410`; `c21112fdbfad37dd` |
| PP `lib.rs`; `pressure_runtime.rs`; `pressure_material.rs`; `pressure_exact.rs`; `pressure_exact/source_geometry.rs`; `annulus_geometry.rs`; `source_recovery.rs`; `pressure_sum.rs`; `retained_product.rs`; `retained_wire.rs` | `4c33c25031622db8`; `5a4f07f40ea3b08e`; `a7d63cedda6040e0`; `b85630992db2e3a8`; `a023170798a85650`; `98a781f126af9d8e`; `af58eaf7a1810dc7`; `bfcde7d240fa6837`; `f536bfe786684baa`; `4f383c04752baaa8` |
| `b1` (`603e238517`) `lib.rs` (`permitted_run`, `ordinary_dispatch`) and `retained_memory.rs` (`family_clauses`, `FamilyFact`) | read copies in scratch |
| FK `exact_sum.rs`; FKR `product_certificate.rs`; `product_certificate/final_case.rs`; FKT `product_certificate_vectors.py`; `product_certificate_tests.rs` | `3a847f8dbaa74768`; `64e09224aacdc328`; `d4dbaf3cb455dd50`; `88fc0765939e9dff`; `94ff617d7055c767` |
| RE `retained_precision.rs`; `semantic_contract.rs`; `physics_evidence.rs`; `physics_source.rs`; `source_blocks.rs` | `4722b505279810ce`; `fbbc1a165d200827`; `f17c0f4cfa62070f`; `c4e380ee7c01eb15`; `a9ee998ad4b3925b` |
| PY `retained_precision.py`; `physics_source.py`; `physics_evidence.py` | `9d1156ed6dacce33`; `81487a87d5e170b9`; `40013aa421adf9f2` |
| TS `retainedPrecision.ts`; `physicsSourceRecovery.ts`; `outputPolicy.ts` and its test; `numericalResultQuality.ts` | `7f9b47a99e67d78f`; `ef7ab4c9505825df`; `712941315278fc92`, `ee51a26d2542145d`; `af72a4d5c4527daa` |
| `results.v0.3.schema.yaml` (identity, profile and preview branch); the committed physics-source fixtures | `eb21b496f5ec8435` |

**Executed** (`evidence/RUN.md`): I96's generator once, unchanged; `rv116_checks.py` once; `rv116_s2_illustration.py` once; the `git grep` collision recheck once. All exited 0, in seconds.

**Limits:**
- **Nothing was compiled or run** beyond that Python and Git reads. Every statement about producer, kernel and reader behaviour comes from source.
  - S-1's sites and N-4's differences are read from code; B3b's and B3a's tests establish them.
  - N-3's discrimination rests on the published `work.limit` field.
- **The 07o first failures** I list are derived from the present RS gate order and code; the three readers' runs at SC2 establish them.
- **The section sweep** covers the committed section and 3,000 random sections. Preparation's own `ambiguous_rounding` refusal remains the guard at D1's extremes.
- **B2-C is unwritten.** `receipt_bindings`'s shared spelling and J1's merged SCHEMA text are therefore checked only as proposals.
- **Exact-route verdicts** (which inputs select; whether a mixed exact base exists) are B3-W's, as I96 states.
