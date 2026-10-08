# RV115 (RV-K) ADDENDUM_01: B3-K's kernel items in I96's B3 design (documents only)

TASK (Type 2), RV115, holding RV-K, for ROOT (HELP_HUMAN, Agent 0), who is the return path. I made no delegation. 2026-10-07 UTC. This addendum answers the coordinator's request, which follows RR "RV115 (RV-K) accepts B2-KD with amendments; R-1 to R-11 ruled" and RR "I96's B3-D design returned; RV116 (RV-D) reviews it; RV115 checks K3-2". It reviews only B3-K's kernel content (I96 §8, with §1.2 and §1.4 as context). RV116 reviews the rest of the design.

**The subject:** `R/I96/b3_d_01/DESIGN.md`, sha256 `ad7942f66c906270003c7c7fd98bb7c64900183dc47b31796844fb051e37b69b` (verified; its SHA256SUMS 11 of 11 OK).

**Method.** As in REVIEW.md: documents and code reading at NUM (HEAD `73b5d4e2a6`, whose `P/core`, `P/fixtures` and `P/schemas` trees equal main `2007709549`; `git diff --quiet`, checked), plus read-only Python with VENV in `WT/scratch/rv115_rvk/`:
- one script of my own;
- the committed oracle generator run on scratch copies, unchanged in verify mode and with K3-2's one-line change in `--write` mode. It wrote only beside the scratch copy.

No cargo, install or Git write. Placeholders as in REVIEW.md: FK = `P/core/solver/frame_kernel`, FKR = `FK/src/structural/retained`, FKT = `FK/tests/retained_k4`, PP = `P/core/product_physics`.

## Verdict

- **K3-1 (`ProductMaterial::BaseENu { e, nu }` → `MaterialOperands::ExactENu`): ACCEPT.** It is additive under both code constraints. PP does not match `ProductMaterial` exhaustively.
- **K3-2 (represented Z for every material): ACCEPT.** The finding is true, and the fix is sound. I recommend **option (a), `build_member` for every material, with ROOT's declared exception.**
  - The exception is exactly one generator line and two fixture lines, which I reproduced by regeneration in scratch.
  - It is the only FK test change.
- **Overall: ACCEPT WITH AMENDMENTS.** 0 BLOCKING, 2 SHOULD-FIX, 6 NOTE.
  - Nothing else is missing in FK for the exact route.
  - The 3–6 h estimate is right, and J2k should be its own merge point.

## Findings

| ID | Severity | Where | Finding | Required change |
|---|---|---|---|---|
| **SA-1** | SHOULD-FIX | I96 §8 (B3D-6, B3D-16); B2-K's brief (KD S-11, K-13) | **B2-K's stop list is written against J0's FK.** KD's S-11 stops on "any changed byte in … `product_certificate_vectors.rs`", and K-13 pins case-path byte identity. With J2k first in lane K, B2-K rebases onto a fixture that K3-2 changed. Unless its baseline moves, S-11 fires on the rebase, or the exception goes unrecorded in B2-K's stops. | **State the exception exactly,** as in §2.3: the generator's `if mode else (F(),F())` clause; fixture lines 22 and 122; resulting fixture sha256 `467f881181efb35ca11e8303dc212e3f90e00dd394571cb26f67b8504745f72c`; any other byte in that fixture remains a stop.<br>**Re-base B2-K's S-11 and K-13 on J2k.** |
| **SA-2** | SHOULD-FIX | I96 §8, K3-3 | **K3-3 does not test two of K3-2's behaviour changes.**<br>• **The axis-bit refusal.** K3-2 also extends the Iy = Iz axis-bit refusal (`NumericError::AxisBits`) to `ExactENu`, because the same flag gates it (FKR/product_certificate.rs:582–585). I96 states the change, but K3-3 has no control for this new refusal path.<br>• **K3-2's discriminator.** K3-3 also lacks a test showing a `BaseENu` stress row fails without K3-2 and certifies with it. | **Add to K3-3:**<br>• `ExactENu` with Iy ≠ Iz refuses with `AxisBits`, keeping its work prefix;<br>• a `BaseENu` stress row and maximum row certify in both lanes, and fail with `bad("represented Z")` under a test-only mutation that restores the material gate;<br>• through the public variant: an E-bit mismatch gives `MaterialBits`, and ν ∉ (−1, ½) gives `InvalidMaterial`;<br>• the six non-`ExactENu` oracle vectors keep their bytes and pass. |
| NA-1 | NOTE | K3-2 | The hull is the right K-lane representation on the exact route.<br>• **What the exact route publishes.** Its bending stress is M/Ẑ: `stress_recovery` `divide_optional` with `section_modulus` (P/core/loads/stress_recovery/src/lib.rs:505–517). The exact branch of `build_model` sets `derived.section_modulus` from `SourceAnnulus` (PP/src/lib.rs:7137–7152), and B3D-3 makes it the prepared Z.<br>• **Where the material gate came from.** I41 scoped represented Z to "ordinary" without a material reason (I41 RETURN line 28). DEF-O's `rows.component_stress` is material-agnostic ("admitted-K bending Z is hull(actual Z, I_K/c)"). | None |
| NA-2 | NOTE | K3-2 option (b) | Option (b) is feasible but not recommended:<br>• it duplicates the hull and the axis check inside `recipe`;<br>• `MemberEnclosures.represented_z` stays `None` for `ExactENu`, a trap for any later reader;<br>• it adds a route-specific certificate path to review. | None |
| NA-3 | NOTE | I96 §8 (optional Ĝ check) | I agree not to add a kernel check that Ĝ equals RN64(E/(2·RN64(1+ν))). Today `ExactENu` checks E's value only (product_certificate.rs:416–431).<br>The dual readout holds for any admitted Ĝ:<br>• \|Ĝ·J − G·J\| enters as `coefficient_differences`, then η, then α, and α ≥ 1 refuses;<br>• G8 binds Ĝ. | None |
| NA-4 | NOTE | K3-1, PRODUCT_MEMBER_FACTS atom | `ProductMaterial`'s size cannot change.<br>• No field in any variant has a niche (f64, usize), so the enum carries a separate tag.<br>• `Interpolated`'s 88-byte payload governs the size (96 bytes).<br>• A 16-byte variant uses one more tag value.<br>K-14's check still runs at J2k (R-4's ruling). | Run `profile_in_build_record` in the registered build at J2k |
| NA-5 | NOTE | B3D-16, J2k | J2k needs no PP change: PP constructs only `Base`, `Point` and `Interpolated` (PP/src/retained_product.rs:461–465, :1409), and its three test matches end in wildcard arms. | **J2k's acceptance list:**<br>• K3-3;<br>• FK's suite, with only the declared fixture lines changed;<br>• PP's compile and suite;<br>• every FK dependent compiled (as ruled for J3);<br>• `profile_in_build_record` in the registered build |
| NA-6 | NOTE | K3-3 | **The ν = 0.3125 control.** I96 cites it as "RV56's control", but it lived in RV56's review fixture, not in a maintained test, so K3-3 must add its own.<br>**The test's fallback.** `unwrap_or(Enclosure::point(Endpoint::ZERO))` (FKT/product_certificate_tests.rs:91) becomes inert for every material. A regression to `None` would still fail `covers`, because the new references are positive. | Leave the test code unchanged, to keep the exception minimal |

## 1. Is K3-2's finding true? Yes

**Represented Z is gated on the material kind.**
- `material()` returns a third value: `true` for `Ordinary` and `Interpolated`, `false` for `ExactENu` (FKR/product_certificate.rs:410–477).
- `build_member` computes `represented_z` (the Iy = Iz bit check, then hull(I_K/c, Ẑ)) only when that value is true. Otherwise it returns `None` (:544, :582–592).

**Both certificate paths need represented Z in the K lane.**
- Both call `recipe(…, represented = true)`:
  - `check_intervals` (FKR/product_certificate/final_case.rs:1366–1414), from `run_case` (`certify_product_case`);
  - the prepared `project` (:1809).
- For every `Stress` and `CircularMaximum` row, the K branch takes `sec.represented_z.ok_or_else(|| bad("represented Z"))` (:748–752).

**So K3-1 alone fails every exact stress row.**
- `BaseENu` maps to `ExactENu`, so every exact-route stress or maximum row would fail its certificate with an `Association` failure.
- Under decision 7, that makes the case `unavailable`, with a facade certificate failure.
- Today nothing reaches this path: no public `ProductMaterial` maps to `ExactENu`, and the only maintained `ExactENu` users are tests.

**The oracle pins the placeholder.**
- `product_certificate_vectors.py` line 68 sets `rz = … if mode else (F(),F())`.
- The committed fixture holds `("Z+", "Z+")` at lines 22 (`exact_normal`) and 122 (`ratio_amplification`), the two mode-0 (`ExactENu`) vectors.
- `product_certificate_fraction_geometry_material_and_coefficients` checks `represented_z.unwrap_or(point(0))` covers it (FKT/product_certificate_tests.rs:66–92).

## 2. Is the fix sound, and which option?

### 2.1 Soundness

**Represented Z is a section and recipe quantity, not a material one.**
- `ExactENu`'s E and G enter only the stiffness coefficients, and through them the solved actions M.
- The section modulus is independent of the material.

**The K lane's readout covers both candidate stresses.**
- The K lane divides its action enclosure by hull(I_K/c, Ẑ), using signed corners over a positive interval.
- So it encloses both M_K/Ẑ (PP's published recipe applied to the K law's exact action) and M_K·c/I_K (the K law's own bending stress).
- This holds for any material, because neither denominator depends on it (evidence §3).

**The G lane is untouched.**
- It keeps its own geometric denominators (`sec.a`, `sec.z = i/c`, `sec.j`, `sec.c`), which are material-independent.
- For `ExactENu` it uses the exact G = E/(2(1+ν)), enclosed outward. This is positive, because E > 0 and RD(1+ν) > 0 for ν > −1.

**Every certificate premise still holds:**
- positive source laws;
- finite coefficient differences;
- α < 1 or refusal;
- the hull and DEF-O's predicates, which are unchanged.

**On the exact route specifically** (NA-1): the published Z is the prepared Ẑ, and I_K is the prepared I.
- G5b binds the area and Z bits to the evidence.
- `run_case` binds A, I, J and the radius to the source member (final_case.rs:1052–1068).

So the hull's two endpoints are the exact route's own represented quantities.

### 2.2 Recommendation: option (a), with the declared exception

- **(a) is right.** It makes `build_member` agree with DEF-O's material-agnostic definition: one place, one formula, and the axis check with it.
- **(b) is worse.** It keeps a latent `None` and duplicates logic in the certificate (NA-2).
- **The exception strengthens the oracle.** It replaces a placeholder zero with a real enclosure that the actual interval must cover.

### 2.3 The exception, exactly

I ran the committed generator on a scratch copy in verify mode. It reproduces the committed fixture (exit 0, sha256 `8cbe5d32…a39a37`). I then removed only the `if mode else (F(),F())` clause and ran it with `--write` on a second scratch copy (evidence `k3_2_regeneration.txt`).

**The whole difference is three lines:**
- generator line 68: `rz=(min(kk[4]/c,kk[6]),max(kk[4]/c,kk[6]))`;
- fixture line 22 (`exact_normal`): `("Z+", "Z+")` becomes `("+ep2", "+fp2")`, the hull [7, 7.5] from I_K = 15, c = 2 and Ẑ = 7;
- fixture line 122 (`ratio_amplification`): `("Z+", "Z+")` becomes `("+8p-1", "+8p0")`, the hull [0.5, 1] from I_K = 1, c = 2 and Ẑ = 1.

**The resulting hashes:**

| File | Now | After the exception |
|---|---|---|
| Fixture | `8cbe5d32ea21a3ced7d5dc5cf9ccde256d01a6d546e94406a43a5d46b3a39a37` | `467f881181efb35ca11e8303dc212e3f90e00dd394571cb26f67b8504745f72c` |
| Generator | `88fc0765…` | `d9618a326ed1051c2471a3ddce8262b8cbd2bafbd89666a27c840f6affe03eda` |

**Independent cross-check.** My own token encoder (evidence §1–§2) gives the same tokens. They also equal the committed tokens of `interpolated_negative_e` and `interpolated_wide_span`, which have the same K primitives. Both mode-0 vectors have Iy = Iz bits, so the extended axis check passes them.

**It is the only FK test change.** No maintained test asserts `represented_z == None`, and the fixture hash is pinned nowhere outside FK's tests (grep).
- **The other `ExactENu` tests** refuse in `material()` before the new code: `product_certificate_tests.rs:226` (ν = −1) and `source_bridge_tests.rs:380` (ν = ½).
- **The member-arithmetic test's work assertions** are relational: R4 is 0 off mode 2, B64U and f64 counts are 0, and TwoProduct calls = Mul + Div − exact-zero products. They still hold with the added divisions.
- **`product_final_case_tests` uses `Ordinary` only.**

B3-K's runs establish this; it is not established by reading alone.

## 3. Is K3-1 additive? Yes

**`ProductMaterial`'s reach** (FKR/product_certificate/final_case.rs:20–43):
- it is re-exported by `origins.rs:805` and `FK/src/structural.rs:19`, so no re-export changes;
- FK's only exhaustive match on it is its own `operands()` (:44–70), which gains the arm.

**No existing signature changes.** The new variant is the only addition.

**The size is unchanged** (NA-4). `ProductMaterial` is stored inline in `ProductMemberFacts` (atom `PRODUCT_MEMBER_FACTS`).

**PP does not match `ProductMaterial` exhaustively.**
- Non-test PP only constructs it (`retained_product.rs:461–465, :1409`) and stores it (`MaterialSelection.selected`, :17).
- Its three test matches end in wildcard arms (`retained_product_tests.rs` near :1262, :1670 and :1700).
- No other workspace crate references it (grep over `P/core` and `P/validation`).

## 4. Is anything else missing in FK for the exact route? No

I checked the exact route's certificate path against B3-D's G5b section-bit check and G8's exact-G binding.

**Facts.** `run_case` and `begin_prepared_product` bind A, Iy = Iz, J and D/2 bits to the source member. These are material-independent, and prepared bits pass.

**Material.** `ExactENu` checks E's value against the admitted E and −1 < ν < ½, and encloses G outward. G8 binds Ĝ in the readers (NA-3).

**Preparation.** `prepare_product_annulus(D, t_eff)` is material- and route-independent. G5b compares receipt and evidence bits, a reader and producer obligation with no FK hook.

**Rows.** Stress and maximum rows are covered by K3-2. With D1.5-exact (`pressure_regions` empty), no pressure row is published, so no FK recipe is needed (I96 §3). Native, support and magnitude rows are owner-generic.

**The source lane.** It uses `law.material` through the same `build_member` (`source_residual.rs` `coefficients`), so `BaseENu` reaches the G lane with no further change.

## 5. The estimate and J2k

**3–6 h agent is right:**

| Item | Hours |
|---|---|
| K3-1 | about 0.5–1 |
| K3-2 (a): code plus a mechanical regeneration | about 0.5–1.5 |
| K3-3 with SA-2's controls | 1.5–3 |
| Runs | about 1 |

RV-K needs 1–2 h.

**J2k should be its own merge point.**
- Without it, B3b-P's certificate stage waits for J3: B2-K's 19–30 h plus its RV-K round. That puts B2-K on B3b's critical path.
- B3-K shares `final_case.rs` and `product_certificate.rs` with B2-K, but not their functions:
  - B3-K touches `ProductMaterial`, `operands()` and `build_member`;
  - B2-K touches `product_owner`, the `row_scales` branch and the residual's two load sites.
- Taken first in lane K, B3-K's conflict risk is small, provided B2-K is re-based on J2k (SA-1) and J2k's acceptance is NA-5's list.

## 6. What I read, execution record and limits

**Read** (sha256 prefixes):

| Input | sha256 |
|---|---|
| I96 DESIGN.md (§0, §1.2–§1.4, §3, §8, §9, §11) | `ad7942f66c906270` |
| I41 RETURN | `a12592f535c32e38` |
| RR's two sections named above | — |
| FKR `product_certificate.rs` | `64e09224aacdc328` |
| FKR `product_certificate/final_case.rs` | `d4dbaf3cb455dd50` |
| FK `src/structural.rs` | `39164ca2c866fc38` |
| FKT `product_certificate_tests.rs` | `94ff617d7055c767` |
| FKT `source_bridge_tests.rs` | `85a79deac6cc1ccc` |
| FKT `product_certificate_vectors.py` | `88fc0765939e9dff` |
| FKT `product_certificate_vectors.rs` | `8cbe5d32ea21a3ce` |
| PP `src/retained_product.rs` | `f536bfe786684baa` |
| PP `src/retained_product_tests.rs` | `6ff9fd75df696dd3` |
| PP `src/lib.rs` | `4c33c25031622db8` |
| `P/core/loads/stress_recovery/src/lib.rs` | `38c66ecd2bcb9656` |

**Executed** once each, with VENV's Python 3.13.14 (`addendum_01/RUN_ADDENDUM.md`, placeholder paths):
- `rv115_addendum_checks.py` (mine): the represented-Z hulls and tokens for all eight oracle vectors, and the corner-division enclosure of both quotients;
- the committed generator on a scratch copy, in verify mode;
- the same generator with the one-line change, in `--write` mode, writing only beside the scratch copy.

All three exited with status 0. Outputs are in `addendum_01/`.

**Limits:**
- **No code was compiled or run.** The claims about FK test outcomes (§2.3) are read from source and established by B3-K's runs.
- **Kernel content only.** I reviewed B3-K's kernel content. The rest of I96's design is RV116's.
