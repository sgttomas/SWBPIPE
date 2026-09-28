# I6 return, addendum 1: RV7's review of PR #1032

This addendum was written by Type 2 TASK I6 for the T3 manager.
- **It answers:** RV7's review of `79c0d320b` (provisional verdict NOT PASS: 1 BLOCKING, 3 SHOULD-FIX, 5 NOTE; the code is correct), as ROOT ruled on it.
- **Scope, per ROOT:** a single addendum. `RETURN.md` and `CHANGE_RECORD.md` stay untouched. The statements named below, by file, section and line (at `80290ce98`), are superseded here.
- **Code:** no product code changes. The FK test file and its generated models gain rows (§9), per ROOT's revised N2 ruling.
- **Record files:** every new record file is a new file, and `SHA256SUMS` gains one entry per new file. No existing entry changes.

Sources:
- RV7's review records, `REVIEW/_run_records/k2a/`:
  - the probe `rv7_rotated_m03.rs`, run with FK's real `transform_roundoff` on an archive of `80290ce98`;
  - `fk_base3.log`;
  - `pp2.log`;
  - `pp_main_skew3.log`, main's product route on an archive of `5ae22926e`.
- My own cross-check, which is not the confirmation: `_run_records/product_reach/skew_replication.py.txt` and its `.stdout.txt`.

## 1. B1 (BLOCKING): M03's floor holds only for axis-aligned members

**Superseded:**
- `RETURN.md` §5.3 line 155: "For a rotated element the same test applies to each transformed entry's magnitude sum, which is of the same order."
- `RETURN.md` §5.4 lines 164–166, right-hand column: the 1/L row "Refused by M03"; the 6EI/L² row "Refused by M03 in every case"; the 12EI/L³ row "Accepted only in the least-subnormal pattern … M03 refuses".
- `RETURN.md` §5.7 line 192, as far as it restates those.
- `CHANGE_RECORD.md` line 37: "Partial underflow with a nonzero subnormal-derived sibling: main's M03 already refuses these".

### 1.1 Mechanism

`transform_roundoff` (FK `structural.rs:1832-1866` at `5ae22926e`) bounds each global entry (i, j) by g·(Σ_k |T_ki|·magnitudes_kj/(1 − g) + Σ_k |T_ki|·|temp_kj|)·(1 + 64ε), where magnitudes_kj = Σ_m |K_km|·|T_mj|, and then applies `checked_value`.
- **Axis-aligned members:** each rotation row has one nonzero entry, of magnitude 1. So each bound is about 2g·|c| for one coefficient c, and the floor argument of §5.3 holds: M03 refuses any nonzero entry below about 2^-974.585.
- **Skew members:** the sums mix coefficients across each 3×3 block.
  - The rotational block mixes GJ/L with 4EI/L and 2EI/L.
  - The translational block mixes EA/L with 12EI/L³.
  - The coupling blocks hold only 6EI/L² terms.

  So a below-floor, subnormal-derived 4EI/L or 2EI/L gets a normal bound from GJ/L, and M03 accepts it. The products |c|·|T| do not underflow, because c itself is normal (at least 2^-1022).

### 1.2 Confirmed kernel figures (RV7, real FK)

**Setup:**
- main's pre-K2a local matrix, with FK's orientation and transform;
- OD 1e-11 m, wall 1e-12 m, so I = 2.899e-46 m⁴ and J = 2I;
- G = 1e-100 Pa, so GJ/L = 3.19e-134;
- L = 2^-39 m; the floor is 2^-974.585.

**Members:**
- axis-aligned x, y reference +y;
- skew (1,1,1), y reference +z;
- skew (1,2,2), y reference +x.

Relative errors are measured against E scaled by 2^600.

| (12E)·I | E (Pa) | 6EI/L² | 4EI/L (rel. error) | 2EI/L (rel. error) | Axis | (1,1,1) | (1,2,2) |
|---|---|---|---|---|---|---|---|
| 2^-1030 | 2.4992e-266 | 2^-953.00 | 2^-992.58 (5.7e-14) | 2^-993.58 (1.1e-13) | refused | **accepted** | **accepted** |
| 2^-1040 | 2.4407e-269 | 2^-963.00 | 2^-1002.58 (5.8e-11) | 2^-1003.58 (1.16e-10) | refused | **accepted** | **accepted** |
| 2^-1045 | 7.6271e-271 | 2^-968.00 | 2^-1007.58 (1.86e-9) | 2^-1008.58 (3.73e-9) | refused | **accepted** | **accepted** |
| 2^-1050 | 2.3835e-272 | 2^-973.00 | 2^-1012.58 (5.96e-8) | 2^-1013.58 (1.19e-7) | refused | **accepted** | **accepted** |
| 2^-1055 | 7.4483e-274 | 2^-978.00 (below the floor) | 2^-1017.58 (1.9e-6) | 2^-1018.58 (3.8e-6) | refused | refused | refused |
| S6a: exact (12E)·I = 2.5·2^-1075 | 1.7758e-279 | 2^-996.00 (rel. error 0.60) | 0 | 0 | refused | refused | refused |

- **Row details:**
  - In the 2^- rows, all four (kE)·I values are subnormal and nonzero, EA/L is normal (2^-918.2 to 2^-943.2), and 12EI/L³ is normal (2^-913 to 2^-938).
  - In S6a, (12E)·I = (6E)·I = 5e-324, (4E)·I = (2E)·I = 0, 12EI/L³ = 2^-957.00 (20 % wrong) and EA/L = 2^-961.9.
- Every refusal is `Range("arithmetic outside normal range")`.
- The least rotational-block bound is 2^-492.47 on (1,1,1) and 2^-494.05 on (1,2,2), because GJ/L mixes in. On the accepted members, the least bound is in the coupling block, about 2^-47.4 below 6EI/L².
- **An exact cantilever solve** with the 2^-1050 coefficients is off by 2.4e-7 relative (RV7's Python).
- **Cross-check** (my replication, `skew_replication.stdout.txt`): for E from 1e-270 to 1e-267 Pa, giving 2EI/L from 2^-1008.2 to 2^-998.2, (1,0,0) is refused and (1,1,1) and (1,2,2) are accepted at every E. This agrees in direction.

### 1.3 Restated (replacing §5.3 line 155 and the right-hand column of §5.4)

- **Axis-aligned members:** §5.3 and §5.4 hold as written. Every nonzero element entry below about 2^-974.585 is refused. S1 (§2) corrects the 12EI/L³ figure.
- **Skew members:** acceptance is decided by the coupling block, that is, by **6EI/L² against the floor.** [Imprecise in general: the skew threshold lies up to about 1.6 binades above the floor. See `REVIEW/M03_SKEW_PIN_REVIEW.md` S2(a) and N7, and the scoped `IMPLEMENTATION/M03_SKEW_PIN/RETURN.md` §3.4 (RV13-N5).]
  - Where 6EI/L² is above the floor, M03 accepts subnormal-derived 4EI/L and 2EI/L below the floor, with errors up to 1.2e-7 in RV7's rows. It does so even when 6EI/L² itself comes from a subnormal (6E)·I (the 2^-1030 to 2^-1050 rows).
  - Where 6EI/L² is below the floor, M03 refuses (the 2^-1055 row, and S6a).
  - **1/L row:** "Refused by M03" holds on axis-aligned members only. On skew members, accepted as above.
  - **6EI/L² row:** a sub-floor 6EI/L² is refused on both orientations; this is confirmed for S6a and the 2^-1055 row. "In every case" is withdrawn. Its (2·E)·I-nonzero branch relied on 2EI/L being refused, which fails on skew members.
  - **12EI/L³ row:** a subnormal-derived 12EI/L³ below the floor, with EA/L dominating the translational block on a skew member, is plausible but **not probed**, so it is **not established**.
- **Main's product route on skew members** (RV7 step B, `pp_main_skew3.log`, archive of `5ae22926e`):
  - **Model:** one member with N1 free in all 6 DOFs and a moment RZ = 0.1·EI/L. (12E)·I = 2^-1045 and 2^-1050, on axis, (1,1,1) and (1,2,2) members, both modes and both entries: 24 runs.
  - **Main publishes nothing in any of the 24 runs.** All are `NUMERICAL_INTEGRITY_UNRESOLVED`, with no N1 results.
    - Axis-aligned: `Range("arithmetic outside normal range")`, the M03 floor.
    - Skew: M03 passes, then `NumericallyUnresolved { "nonpositive or cancellation-unresolved structural pivot", global_dof: Some(11) }` in sparse mode and `Some(10)` in dense mode. GJ/L exceeds the bending terms by about 10^170, which likely drives the cancellation.
    - The captured entry also carries `SOURCE_BLOCK_RECOVERY_UNAVAILABLE`.
  - **Not established:** that every skew case is refused downstream. A better-conditioned skew model (G ≈ E) was not run.
- **Candidate, same probe** (`80290ce98`): all 24 runs give `SOLVER_SYSTEM_BLOCKED` with `range: … at 12EIy/L^3: (12*E)*Iy`.

### 1.4 Benefit, restated (supersedes `RETURN.md` §5.7 line 192 and `CHANGE_RECORD.md` lines 37 and 41 as far as they restate §5.4)

K2a refuses every zero or subnormal-derived coefficient at formation, by name, **on every orientation.**
- On axis-aligned members, M03 already refuses the nonzero subnormal-derived cases, and K2a refuses them earlier.
- **On skew members, M03 can accept them** (§1.2), and K2a is then the formation-layer refusal. In RV7's product probe, main's later pivot screen refused them; that is not established in general.

K2a's benefit is unchanged in kind and somewhat larger. `CHANGE_RECORD.md` line 37 is restated as: main's M03 refuses these **on axis-aligned members**; on skew members M03 can accept them, and main's downstream standing is as in §1.3.

**Follow-up (not in this PR):** a skew kernel pin (main's element accepted by `transform_roundoff`, K2a refusing by name). It goes to the T3-close list with K1's pattern-path M03 tests.

## 2. S1: the 12EI/L³ subnormal column depends on L

**Superseded:** `RETURN.md` §5.4 line 166, "Here 12EI/L³ ≤ 2^-954.4 is normal, above the floor".

**Derivation.** In the least-subnormal pattern, main forms 12EI/L³ = 2^-1074/L³, with L > 2^-39.863 (the axis tolerance, §5.1).
- It is **normal** iff 2^-1074/L³ ≥ 2^-1022 ⟺ L³ ≤ 2^-52 ⟺ L ≤ 2^-17.33 m.
- It is **above the floor** iff 2^-1074/L³ ≥ 2^-974.585 ⟺ L³ ≤ 2^-99.415 ⟺ L ≤ 2^-33.14 m (about 1.1e-10 m).

**Restated:** the value is at most 2^-954.4. It is above the floor, and so accepted on axis-aligned members, only for L below about 2^-33.1 m.
- For L in (2^-33.14, 2^-17.33) m it is normal but below the floor, and M03 refuses it on axis-aligned members.
- For L ≥ 2^-17.33 m it is subnormal, and refused at `checked_product`.
- reach_lef (L = 2^-39) lies in the accepted range.

## 3. S2: main's product standing is evidenced on the captured entry only

**Superseded:** the entry scope of main's product standing in `RETURN.md` §5.5 (reach_zero, reach_lef, reach_six2 and reach_gj) and §5.8 item 3.

`main_*_probe.jsonl` and the §5.5 standings come from S11-K's harness, **which runs the captured entry only** (`NOTES.txt` lines 1–3; `RETURN.md` §5 introduction).

**Restated:** main's linear and gap-route product standing for these probes is evidenced on the **captured entry, in both modes.** On the typed entry, main's standing was not run and is not established.

**The PP test comment at `core/product_physics/tests/k2a_formation_range_runtime.rs:421-423`** says main's standing holds on "both entries and both modes" from those records. **That comment is overstated.** It is left unedited, per ROOT's ruling, and this addendum records it. The test's assertions are unaffected: its K2a refusals do run on both entries and in both modes, and main's linear standing is pinned at kernel level.

## 4. S3: `pp_k2a_4.log` was cited but never existed

**Superseded:** `RETURN.md` §3 line 66, "(`_run_records/phase1/pp_k2a_4.log`)".

**The cited file `_run_records/phase1/pp_k2a_4.log` never existed** in the committed records, and it has no `SHA256SUMS` entry. At `80290ce98`, `phase1/` holds only:
- `pp_k2a.log`: 2 tests, from before the file's final shape;
- `pp_k2a_2.log`: 3 tests, from before the rounding asserts.

**What evidences the claim instead** ("the final PP runtime test file passes 3/3"):
- RV7's slot run of the PP runtime test on the `80290ce98` archive, which passes 3/3 (`REVIEW/_run_records/k2a/pp2.log`);
- hosted CI on `79c0d320b`.

**Added in this addendum, under a new name** (ROOT: a file created after the fact must not carry the cited name):
- `_run_records/addendum1/pp_k2a_rerun_20260927.log` is the raw output of my post-assert rerun, the run RETURN meant to cite. Its run time is 2026-09-27 23:50 UTC, on the working tree whose test file sha256 is `18ec1d5e…`, the committed file. It passes 3/3.
- Only machine paths are replaced by placeholders. The provenance is in `pp_k2a_rerun_20260927.provenance.txt`: the command, the tree, the run time, and the sanitization.

## 5. N1: derived section values are not the user's exact values

**Superseded:** `CHANGE_RECORD.md` Limits, line 69: "A subnormal **input** (for example a modulus below 2^-1022) is the user's exact value and is not itself refused."

- **Restated:** a subnormal E or G *as entered* is the user's exact value. **A, I and J are not.** PP `derive_pipe_section` (`lib.rs:8471-8475`) derives them from OD and wall, and can form a subnormal value with a large error.
- **Example:**
  - OD 5e-81 m, wall 5e-82 m;
  - OD⁴ = 6.27e-322 and ID⁴ = 2.57e-322 are subnormal, so I = 1.98e-323, which is **9.1 % off** the exact π(OD⁴ − ID⁴)/64 of the same binary64 OD and ID;
  - with E = 9e24 Pa, every kernel intermediate is normal ((12E)·I = 2.1e-297), so **K2a passes it.**
- This is pre-kernel, and outside K2a by design. The manager adds it to the routed input-validation finding.

## 6. N3: the typed-entry LEF-large message in the gate records

`RETURN.md` §5.1 line 122 and §9 line 307 cite the gate runs for the typed-entry LEF-large message. The committed gate records carry no messages, and `runs.jsonl` (246 MB) is not committed.
- **Added:** `_run_records/gate/runs_extract.py.txt` and `runs_extract.jsonl` (888 lines, 0.5 MB), a sanitized extract of `runs.jsonl`. The source's sha256 is in `runs_jsonl_sha256.txt`: `fd0cdbb9…`.
- The extract carries, per run: the case, mode, entry, outcome, quality, wall time, and each blocking or integrity diagnostic's code and message (or the probe's error text).
- **It shows** LEF-large on the typed entry as `SOLVER_SYSTEM_BLOCKED` with `range: stiffness formation outside the binary64 normal range at GJ/L: G*J …`, on all 3 bases and in both modes.
- The same refusal is also pinned by the kernel tests: LEF-large through `FrameElement` and assembly.

## 7. N4: arithmetic and wording

- **§5.2 line 147:** "Its relative error is up to 2^-1075/|p|, which is 100 % at the least subnormal." **Restated:** the absolute rounding error of a subnormal intermediate is at most 2^-1075.
  - Relative to the **formed** p, it is at most 2^-1075/|p|: 50 % at the least subnormal.
  - Relative to the **exact** value, it can reach 100%: an exact value just above 2^-1075 rounds to 2^-1074.
- **§5.2 line 148:** "(< 2^-1078 when zeroed)". **Restated:** for L ≥ 2 m, a zeroed 6EI/L² is below 2^-1075/L² ≤ 2^-1077, and a zeroed 12EI/L³ is below 2^-1075/L³ ≤ 2^-1078.
- **The cost sentences** (`RETURN.md` §5.7 line 199; `CHANGE_RECORD.md` line 42) state "where main's value was accurate" beside correction 3's wording. **Correction 3's wording governs:** "K2a refuses 1/L-lifted zeros where main's published value was within its K-D5-limited criterion." "Accurate" is true of the spring-carried example only.

## 8. N5: incomplete "killed by" lists (`RETURN.md` §6 lines 233 and 236)

The lists are completed from `MUTANTS.txt`, which is unchanged.
- **18b:** add `k2a_every_other_rf_range_vector_keeps_every_coefficient_normal_and_byte_identical`. The full list is:
  - the per-site table;
  - the every-other-vector test;
  - the partial-underflow control;
  - refused-before-K-D5;
  - LEF-large;
  - LEF-small;
  - the subnormal-intermediate control.
- **Reassociation:** add `k2a_refused_formation_is_refused_before_the_formation_check_runs`. The full list is:
  - the per-site table;
  - the every-other-vector test;
  - the 1,683-case bit-identity sweep;
  - the partial-underflow control;
  - refused-before-K-D5.

## 9. N2: surviving site-specific mutants, and their closure in this PR (ROOT's revised ruling)

### 9.1 What survived (RV7, on `80290ce98`)

**Superseded:** `RETURN.md` §6, "No mutant survives." That statement held only for my own mutant set.

Ten per-site rows of `FORMATION_SITES` form a zero or an infinity, not a subnormal. Those rows are L³, E·A, (E·A)/L, G·J, (G·J)/L, 12·E, (12E)·Iy, 12EIy/L³, (12E)·Iz and 12EIz/L³. So a check that accepted a subnormal **at those sites only** was not caught.

**Survived all 10 kernel tests:**

| Mutant | What it does |
|---|---|
| R1 | accepts a subnormal only at the 12EI/L³ quotients |
| R2 | only at E·A, (E·A)/L, G·J and (G·J)/L |
| R3 | only at L³ |
| R4 | only at (12E)·Iz |
| R5 | only at 12·E |
| R5b | checks numerator·(1/L) but returns the quotient |
| R10 | also refuses exactly 2^-1022 |

**Killed:**

| Mutant | What it does | Killed by |
|---|---|---|
| R9 | a subnormal accepted at all products | the per-site table and both controls |
| R6 | reorders 6EIz | the RF-RANGE byte identity and the bit-identity sweep |
| R7 | forms 12EIz as (12E)·(Iz/L³) | the per-site table and both bit-identity tests |

### 9.2 Rows added (FK `tests/k2a_checked_formation.rs` and the generated `tests/k2a/rf_range_models.rs`; test files only)

The operand sets are generated and verified by `_run_records/k2a_models_v2.py.txt`, a new file. The committed `k2a_models.py.txt` is kept unchanged. Its output is `k2a_models_v2.json`. A new test helper, `unchecked_intermediates`, lists all 26 intermediates of the unchecked formula in the checked order. The test asserts that its names equal `FORMATION_SITES`'s names.

| Test | Rows | What each row asserts |
|---|---|---|
| `k2a_each_zero_or_infinite_site_also_refuses_a_subnormal_by_its_own_name` | 10 sites, below | Paths differ: unchecked, the site forms a subnormal (nonzero, finite) and every earlier intermediate is normal. Then `NumericalRange` with that site's exact name |
| `k2a_the_smallest_normal_intermediate_is_accepted_bit_identically` (R10) | E = 2^-600 Pa, A = 2^-422 m², others 1 | E·A = (E·A)/L = 2^-1022 exactly (`f64::MIN_POSITIVE`), with every intermediate normal. Accepted, with the unchecked bits |
| `k2a_the_quotient_itself_is_checked_not_the_product_with_the_reciprocal` (R5b) | E = 0x1.9ffffffffffffp-1019 Pa, A = 1, Iy = Iz = 2^200, L = 13 m | x/13 = 2^-1022 − 2^-1074 (subnormal) while x·fl(1/13) = 2^-1022 (normal), and every other intermediate is normal. Refused at `EA/L: (E*A)/L` |

The ten subnormal rows (inputs in SI; unlisted inputs are 1):

| Site | Operands | Subnormal formed |
|---|---|---|
| L³ | L = 2^-345 | 2.716e-312 |
| E·A | E = 2^-600, A = 2^-430 | 8.692e-311 |
| (E·A)/L | E = A = 2^-500, L = 2^30 | 8.692e-311 |
| G·J | G = 2^-600, J = 2^-430 | 8.692e-311 |
| (G·J)/L | G = J = 2^-500, L = 2^30 | 8.692e-311 |
| 12·E | E = 2^-1030 (a subnormal input), A = 2^100 | 1.043e-309 |
| (12E)·Iy | E = 2^-600, Iy = 2^-430 | 1.043e-309 |
| 12EIy/L³ | E = 2^-400, Iy = 2^-403, L = 2^76 | 5.215e-310 |
| (12E)·Iz | E = 2^-600, Iz = 2^-430 | 1.043e-309 |
| 12EIz/L³ | E = 2^-400, Iz = 2^-403, L = 2^76 | 5.215e-310 |

- **Run** (FK k2a target only, in a small target, then removed): **13/13 pass** (`_run_records/review/fk_k2a_rows.log`).
- Machine paths in the log are replaced by placeholders.
- No product code changed.

### 9.3 Kills shown (RV7's re-run on the new tree)

RV7 re-ran each former survivor on the local commit `a1029d7da` (test file `0fa172eb…`), with a clean target per mutant. The patches applied unchanged, because they anchor on `src/lib.rs`, which is unchanged (`7622e7cc…`). The no-mutation control passes 13/13. Logs: `REVIEW/_run_records/k2a/rerun_a1029d7da/`, in numerics.

**All 7 former survivors are killed.**

| Mutant | Killed by (test, line, assertion) | Behaviour under the mutant |
|---|---|---|
| R1 (subnormal accepted at the 12EI/L³ quotients only) | `k2a_each_zero_or_infinite_site_also_refuses_a_subnormal_by_its_own_name`, :501, `assert_eq!(local_stiffness(p), range(name))` | row `12EIy/L^3: (12*E*Iy)/L^3` returns `Ok(matrix)` instead of the refusal |
| R2 (at E·A, (E·A)/L, G·J, (G·J)/L only) | the same test, :501; and `k2a_the_quotient_itself_is_checked_not_the_product_with_the_reciprocal`, :541 | row `EA/L: E*A` returns `Ok`; the quotient test returns `Ok` instead of `Err(NumericalRange "EA/L: (E*A)/L")` |
| R3 (at L³ only) | the subnormal-site test, :501 | row `L^3 (for 12EI/L^3): L^2*L` names `12EIy/L^3: (12*E*Iy)/L^3` instead |
| R4 (at (12E)·Iz only) | the subnormal-site test, :501 | row `12EIz/L^3: (12*E)*Iz` names `12EIz/L^3: (12*E*Iz)/L^3` instead |
| R5 (at 12·E only) | the subnormal-site test, :501 | row `12EIy/L^3 and 12EIz/L^3: 12*E` names `12EIy/L^3: (12*E)*Iy` instead |
| R5b (checks numerator·(1/L), returns the quotient) | the quotient test, :541 | `Ok` instead of `Err(NumericalRange "EA/L: (E*A)/L")` |
| R10 (also refuses exactly 2^-1022) | `k2a_the_smallest_normal_intermediate_is_accepted_bit_identically`, :519, `expect("2^-1022 is normal")` | panics with `NumericalRange "EA/L: E*A"` |

- **Kill status:** every mutant in my own set (`RETURN.md` §6) and in RV7's set (R1–R10, R5b) is now killed.
- **RV7's delta review of this addendum's content:** §1–§8 match its findings, and B1, S1–S3 and N1–N5 are closed.

## 10. Test-file pins

| File | At `80290ce98` | After this addendum |
|---|---|---|
| FK `tests/k2a_checked_formation.rs` | `54383afee975d4969b9723bed057b7a1a06b6708f7e1e7c6bbaa6e56c8b78396` | `0fa172eba552b24093034c59731fe50911e4e6eb411abc6b6f490028f7c3f4d9` |
| FK `tests/k2a/rf_range_models.rs` | `f4a6f6aa67516f7c59dfe9473074ce2ae5b650efaea2d24f4d8e09817fbffc6a` | `3463d22784a252d18f6bc232f4c8616eefc712679c1c7f5bd8137e7413b0c7c5` |
| PP `tests/k2a_formation_range_runtime.rs` | `18ec1d5e0e817af2bdafd7788258f2a24b165d4e86c9add1d501b9e1a7bb2a33` | unchanged |

## 11. New record files (each a new `SHA256SUMS` entry; no existing entry changes)

- `RETURN_ADDENDUM_1.md`
- `_run_records/product_reach/skew_replication.py.txt`, `skew_replication.stdout.txt` (the cross-check, §1.2)
- `_run_records/addendum1/pp_k2a_rerun_20260927.log` and `.provenance.txt` (§4)
- `_run_records/gate/runs_extract.py.txt`, `runs_extract.jsonl` (§6)
- `_run_records/k2a_models_v2.py.txt`, `k2a_models_v2.json` (§9.2)
- `_run_records/review/fk_k2a_rows.log` (§9.2)
