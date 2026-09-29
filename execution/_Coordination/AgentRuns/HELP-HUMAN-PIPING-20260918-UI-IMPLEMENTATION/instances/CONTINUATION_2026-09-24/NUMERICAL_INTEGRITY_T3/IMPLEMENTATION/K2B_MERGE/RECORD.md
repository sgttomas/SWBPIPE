# K2b merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1040.
  - ROOT (HELP_HUMAN) merged it on 2026-09-28 as `e7d930d493bf5b2fad00284727974cb3a22d0caa`: a merge commit with `--match-head-commit 33e33c723`, under the owner's standing Git authorization.
  - Main was `57617b0fb` (K3), an ancestor of the head, and the merge state was clean.
- **Candidate head:** `33e33c723a974a86947348156d7917953612730d`, on branch `codex/piping-k2b-20260928`.
- **Where it ran:** the owner's Mac (`aarch64-apple-darwin`, rustc 1.97.1). ROOT dispatched the implementer (I10) and the reviewer (RV11) directly.
- **Scope:** W2's exact force-radix scaling, and the kernel half of formation-time scaling (SCALE-W; DESIGN.md §4.7).
  - It is kernel only: no product path calls a new entry, and existing entries are byte-identical at b = 0. PP's wiring and the `range_scaling:` evidence line are F1b's.

## The chain (base main `eb52114e9`)

| Commit | Content |
|---|---|
| `6ce4d694b` | Checkpoint A. I10 stopped here on the spring-carried and partial-underflow cases (rulings A–C) |
| `70828d4d6` | Checkpoint B: rulings A–C |
| `8e6698282` | Checkpoint C's assertion groups |
| `b461dfc0f` | The b-rule's documented-limitation pin (the "third attempt" counterexample) |
| `ca20b9eca` | Records |
| `087b3a088` | A merge of main `98b1723b1` (the skew M03 pin) |
| `bf4647c21` | RV11's fixes: reactions and member actions fail closed; exact scaling only |
| `14f9b093f` | Tests for RV11's fixes |
| `f385a8bc8` | Records addendum 1 |
| `f9a15fbd6` | A checked spring-action helper (RV11D-1) |
| `85ae94b41` | Tests for RV11's delta findings; the scaled helpers on the pin list |
| `112c1729d` | Records addendum 2 |
| `33e33c723` | A merge of main `57617b0fb` (K3) |

## Gates

- **Independent review, RV11** (`REVIEW/K2B_REVIEW.md`; numerics `833d69b96`, `90ab6f1f9`, `255fce346`, `cf44386ae`):
  - **FAIL** at `087b3a088`, with 1 BLOCKING finding. RV11-1: `force_scaled_reactions` published an unchecked K′·u. RV11's probe published 0 labelled Normal where the truth was ±1.38e-300 N.
  - **PASS** on the delta at `f385a8bc8`. RV11-1 to RV11-4 were fixed, with 146,602 values across the forced-b grids and 0 wrong. It raised RV11D-1 (the spring-action recipe) and RV11D-2.
  - **PASS** on the second delta at `112c1729d`. Everything was resolved: the spring probe gave 537 solves and 0 wrong.
  - **Confirmed** at `33e33c723`: the merge adds exactly main's delta, K2b and K3 are file-disjoint, and the b = 0 probe gives 439 of 439.
  - Across the review, RV11 checked the kernel-only claim (a lexer scan, and the probe at 439/439 against every head), the 13-step even-b derivation, the census and the b-rule pin, and ruling B.
- **Hosted CI on `33e33c723`:**
  - pull_request run **36416556943**: success. Its Numerical cargo suite took about 11.1 minutes.
  - the full-SHA dispatch **36416551310** (target_base `57617b0fb…`): success.
  - The final PR state: 12 checks passing, 0 failing.
- **T9 (Mac-only):** 112 of 112 (I10's RETURN, at checkpoint B and again on the RV11 fixes). The base equals the Mac calibration hashes.
- **DEC-025,** under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28), **on the final head `33e33c723`**; the evidence is in `dec025/`:
  1. **The sweep invocation** failed at the cargo surface on product_physics's platform test `t13` (fail-fast). The summary is `SWEEP_20260928T113531Z_33e33c723a97.json`, sanitized; the sha256 of the original is `350a3d8331aa60c42eeec1f3deb8e910d4670163eceb53dc88d60958b95abe4a`.
  2. **All 39 manifests** were run with `--no-fail-fast`. Against the Mac run of main's tree, only frame_kernel (227 → 249) and nonlinear_integration (102 → 120) change, by K2b's tests. [Correction (RV13-N1): the baseline is the K3 candidate `b7e93650e`'s Mac run, that is main `57617b0fb` less K3's two follow-up tests, and 2 of FK's 22 added tests are those. By `#[test]` counts, K2b adds 20 FK and 18 NI tests. The driver is `M03_SKEW_PIN_MERGE/dec025/dec025_mac.sh.txt`.] The failing tests are exactly the three Mac platform tests (`suites_vs_baseline.txt`).
  3. **Surfaces 2, 3 and 5** all exit 0:
     - pytest: 3023 passed, 32 skipped;
     - vitest: 134 files, 2822 of 2822;
     - the production build.
  - **Disclosure: the earlier head `087b3a088`** (`dec025/earlier_087b3a088/`). Its vitest surface failed one test: `App.test.tsx`'s workspace render timed out at 30 s while the host's load average was above 8. K2b changes no TypeScript. The timeout was not raised; the whole procedure was re-run on the final head under lighter load, and it passes.
  - Machine paths are sanitized to `<WORKTREE>`, `<VENV>`, `<wt>`, `<home>` and `<tmp>`.
- **The gate:** not run. K2b is kernel only (ROOT's K2b rulings); it runs at F1b.

## Rulings and findings, as recorded in ROOT_RULINGS_V1 [ROOT's decisions at RV11's delta checks were recorded there late, as "K2b: ROOT's decisions at RV11's delta checks" (RV13-S2)]

- **Even b** amends §4.7 step 3's letter, not its intent. The derivation is in RETURN §4, checked by RV11, with its premise corrected by RV11-3.
- **Census scope; residual records published descriptively** (ruling B); **the LEF restatement** (ruling C).
- **Spring-carried stays a named refusal,** limited by M03's audit and routed to W1/K4.
- **The b-rule's window misses the solve's range:** I10's counterexample is pinned as a documented limitation, and the refinement goes to F1b's brief and the T3-close list. ROOT's first framing ("equivalent by construction") was wrong.
- **RV11-1 and RV11D-1:** reactions, member actions and spring actions at scale fail closed, and are never a wrong Normal.
- **For F1b:**
  - wire b through `SparseAssemblyOptions` and the orchestrator;
  - use the checked action helpers, not recipes;
  - loads formed at b = 0 from out-of-range products have already lost bits;
  - consider the b-rule refinement;
  - restate the LEF expectation at product level;
  - RV11's NOTEs N2–N4 and RV11D2-N1 (the double rounding of subnormal published values at b ≠ 0, within the stated precision).
  - RV11D-N2: the reaction check at b = 0 is stricter than flushing requires; F1b's gate measures its availability cost (added after RV13-S2).
