# I89 B1-SA redaction 01: placeholder paths in the records, redacted in place

TASK (Type 2), I89, role I-A, for ROOT (HELP_HUMAN, Agent 0). I made no delegation. 2026-10-07 UTC.

**Why.** ROOT asked for this redaction. RETURN's committed `_run_records/` (NUM `bc37d43a0a`) and FOLLOWUP_01.md §3 contained a home-relative form of WT's and VENV's paths, which fails the publication screen. The cause is in FOLLOWUP_01.md §3: the first sanitizer took `WT` one level too shallow (`WT/scratch`).

**This is records-only work in NUM, with no Git write.** ROOT commits it.
- **The 46 originals** remain only in NUM's history at `bc37d43a0a`, which is never merged into main.
- **FOLLOWUP_01.md's first version and `followup_01/sanitize.py`** were never committed. Their old bytes survive only as the old sha256 values below.
- **RETURN.md is byte-identical:** sha256 `05c2687d3a9cfe89997926a67ba8207194cbbb8255806c7a7d422626b3ab6184`.

## 1. Method and checks (`_run_records/redaction_01/`)

`redact_01.py` handles each committed file under `_run_records/` (outside `followup_01/`) that matched any of the five screened forms (a home-relative path; the home and temporary-directory roots; the worktrees folder fragment; the worktree's name). **46 of the 75 recorded files matched.** For each one it:
1. **Finds its raw source in `S`**: the kept log, the script or `tmp` file that `write_records.sh` read, with the same `--filter` flag.
2. **Proves provenance.** The first sanitizer, as run (`sanitize_v1.py`, from the same folder), applied to that source reproduces the committed bytes exactly. This held for **46 of 46**.
3. **Re-sanitizes** that source with the corrected sanitizer (`followup_01/sanitize.py`).
4. **Checks that only path forms changed:**
   - the line count is unchanged in all 46;
   - every changed line equals the old line with its path forms mapped by the rules in §2;
   - no other change: 0 lines;
   - no cut line's digest changed: 0.
5. **Checks that the new file is clean** of all five patterns.

**Reproduction.** The script was then run again on the committed originals, extracted with `git archive bc37d43a0a`, whose `SHA256SUMS` verifies 75 of 75. Its 46 outputs are byte-identical to the files now in place. `report.json` is that run's record.

**One edit to the recorded script.** Before it was recorded, the script was changed so that it derives its two old-form constants and its pattern list instead of spelling them. That is why its copy here can pass the screen.

## 2. Substitution rules

Applied by the corrected sanitizer to the raw sources:
1. **WT's absolute path → `WT`.** WT is the T3 worktree root: four levels above the sanitizer, and asserted by its `tools/` and `guard/` folders.
2. **VENV's absolute path → `VENV`.**
3. **The home directory → `~`.** Any home-relative path left over, or any worktrees fragment or worktree name, is now refused by assertion. None remained.
4. **Unchanged from the first version:** lines over 4,000 bytes are cut to 1,000, followed by their length and sha256 (no such line changed here); `--filter` keeps cargo's Running, test, panic, failure and result lines.

Mapped onto the old bytes, the effect on each changed line is:
- the home-relative form of WT's path → `WT`;
- the home-relative form of VENV's path → `VENV`;
- the old `WT/<scratch folder>` (which meant `WT/scratch/<scratch folder>`) → `WT/scratch/<scratch folder>`.

Nothing else in any file changed.

## 3. Files rewritten (old → new sha256)

| File | Old sha256 | New sha256 | Change |
|---|---|---|---|
| `_run_records/cargo_jobs_i89.log` | `8c9031bdbcd0d14ee2332cb9985d603d2c28588dba2e7ebc040b1c440bd08176` | `9ef82fece1bd5777e6c20ed972ec33ab5676d855ba5d718689bbeddf709f1db2` | 120 of 121 lines; path forms only |
| `_run_records/early/early_retained_memory.log` | `fb92a74617d037050ca7311ec60dcdf39191b5f0e5fc4bea605e5f32313b2035` | `3e66d5621318f9abe5f7816343f6c88a3d22c5f95a0682650a2648e2f34459dc` | 2 of 496 lines; path forms only |
| `_run_records/early/early_retained_memory_2.log` | `aa2a99a5eb3c3c354528e6131893dd1c7e5bd2c8d130010bf8957c72e4a3b230` | `d65bac3f99a1f9dae631a547991f542dfd4881d5f3d5ce9038cf54d809b77f04` | 3 of 469 lines; path forms only |
| `_run_records/guards/cand_guards_pp.log` | `d96fe0409903365db8eb0a86890aa68a86da98d5b745a6db40f596def27b881f` | `685ae35303d318405b78f4c1fd59a5cfa56fed29f4eba273eb0f8357dcca290d` | 3 of 164 lines; path forms only |
| `_run_records/guards/cand_guards_re.log` | `0d2940857199944c4b6c6dedda1dd5912dc7928dddf9bd5fd969ac55e049ba18` | `c40ef36c267e423e392a75dd9b700ac87cd96f6b1748a6d8bc3dc98c57e95750` | 2 of 35 lines; path forms only |
| `_run_records/i2_preview/i2_preview_lib.log` | `ad23350fa90bde1e0e66975e4b9df4c31f69bad2f6878f9ab2c2370be37319a5` | `2fe5d2adc3e6632ac13e699c70a90577088800a6ef0a42e201cfe3d9954b4fe3` | 17 of 749 lines; path forms only |
| `_run_records/i2_preview/i2_unpatched_parked_test.log` | `482a266c4eba4a50f8131d0789293264b712e9f7d838a032481153858b126db0` | `734d083aadc97a73fe0efa9b51f44a521344e5b244beeae0df4ab8cbba114667` | 3 of 143 lines; path forms only |
| `_run_records/identity/base_identity.log` | `71328e4d7654e182043673df9c528c18e2dd54b4f67b23ea9e4d8615f0e95c8e` | `044c0ee943f626d4cc0dfce5e79627ed02e00e82d44e2786617c07cc61efb79d` | 17 of 163 lines; path forms only |
| `_run_records/identity/build_norun.log` | `1923e07e0fea335dedd9f860d6094d76a45ae156e711fb085f3a174cacdfdb2f` | `7193251a0cde3027a3530fc3e09091601c2b51a768cbe9dd7c8a1c439c6d4cf4` | 39 of 206 lines; path forms only |
| `_run_records/identity/cand_identity.log` | `7cc6bd624c3546bfa06dba8b3cb70694183bdd2075dc8a285e7cf9fb0834e820` | `0aaefc3c4fdc67c481ac6b03e4e67dc877172b71fc6672f5e5a35c5bec14c12c` | 2 of 126 lines; path forms only |
| `_run_records/mutants/mutant_MA1_d1_4_lt_c.filtered.log` | `2b6a8631ac474c6e630468301615d8208bb17b806e2f77b8b296eff60c8758ae` | `a766acd4e577eb85d5096ed998c11f79e103e2b5dc8227f0aaeff0d5622f1001` | 1 of 592 lines; path forms only |
| `_run_records/mutants/mutant_MA2_d1_4_le_c_plus_1.filtered.log` | `a7f16ac6c19e37749e3a798ba2e55467e00421b9b5ffe0ed0f41ae3dcd203b48` | `8e8d258260531d290a394a5cb3b0019f56b2280c645ef1f40cea5850b3924179` | 1 of 590 lines; path forms only |
| `_run_records/mutants/mutant_MA3_load_rows_case_0_only.filtered.log` | `3e4d426be2d34d59c7b345f914f79cfe5faad4f3c840c105f01739f3e7f5089c` | `0aab1a06ea7e2691788682f7dbdc6e6ed0c2f228f6db464d69533d9d69d2eb78` | 1 of 584 lines; path forms only |
| `_run_records/mutants/mutant_MA4_g_b_total_dropped.filtered.log` | `aee78488879bf7ff63ad99e17e4affcfc5960357958405c72fa1f730218e3143` | `3832658f19fe333c3a4e94d54e54b135b10c93ac7812a4006e200f381219bcfb` | 1 of 580 lines; path forms only |
| `_run_records/mutants/mutant_MA5_g_b_total_current_case_only.filtered.log` | `7199e26f7fdf4361bb6fb25eb6f7b61e6cc4924a01152a01574a90bc054391d0` | `f7c2e4bbbc0e528717374a5bd3c26f9144f4defa02fd472cf148d19962d7b0cf` | 1 of 582 lines; path forms only |
| `_run_records/mutants/mutant_MA6_t3e_seeds_only.filtered.log` | `2144eba94ca497d49b6e5232133df3a54ceb3628749edb3ec2a86c1ce277ef0e` | `cad0ae9d622c13e5b5bc25656637ddf41ceb5d938db91e6fb478145c808c16c8` | 1 of 580 lines; path forms only |
| `_run_records/mutants/mutant_MA7a_t3e_requested_ignored_at_g_c.filtered.log` | `49cf22659d23a4247b5fbef30bd4c04b14ce536aad94b5462acc994afc6a5ea7` | `d85cafad6d9ed270c2b362a17b128590b211b7f46141d82d698b6f5a137e24d4` | 1 of 580 lines; path forms only |
| `_run_records/mutants/mutant_MA7b_t3e_requested_ignored_in_count.filtered.log` | `0f670ac908c85700204d63686e54a0211f49bdb0a907eddd1980aefed122b26b` | `ecb6a1d074fafa01f7cfb7b3fd418706ebd68ef3fc6802427c1ff972da495543` | 1 of 586 lines; path forms only |
| `_run_records/mutants/mutant_MA8_t3e_ge_not_eq.filtered.log` | `17d4b05218afe1fcbffa88a66e2090f40f7a31af26a34508af7da5d3b24465f0` | `fee4f807290523329d57474d5307e198646120bfd053fc50aa6ca4220d8d2625` | 1 of 580 lines; path forms only |
| `_run_records/mutants/mutant_MA9_envelope_results_p_final.filtered.log` | `85c0e284798e4d86d725687c3def8a23bb9cda7d1315ce2064454f49defe5306` | `d6c3674763575a907289bf3071595a7cba3d5904db228f65290a5a9128fa80ce` | 1 of 584 lines; path forms only |
| `_run_records/mutants/mutant_MA9b_envelope_capacity_p_final.filtered.log` | `faba5b9d87c9151ef104bd94502389732a7794a8bd4cf08629d32b25cd1f3169` | `000c97e2df7c66a560a55aea49a1f42d57cd573bfa8b6725c2f19e16957f932c` | 1 of 584 lines; path forms only |
| `_run_records/mutants/mutant_MA9c_envelope_text_p_final.filtered.log` | `9eeca2aa7a69b40c2c9cef825fbcfcff86f50a5212141fbf3dfaab8df3af53d1` | `5de329c0647280be7cbaf34c1523a13f9cbe4b44a82d0a61057b00f048a25ecd` | 1 of 584 lines; path forms only |
| `_run_records/mutants/mutant_MX1_d1_5_case_0_only.filtered.log` | `76993a44a556783998d7e17a319a214257c28bf04f6d78d6d3b7193226a0624c` | `06cf5257be824a4c063e23ebe6a314420eb07cc2b95eefc6e85247ed0cfb615b` | 1 of 580 lines; path forms only |
| `_run_records/mutants/mutant_MX2_d1_7_case_0_only.filtered.log` | `19872a1aefcd00aee76c3528a93ba52739057127cf3e5ce64466b57bd3d7d691` | `43a4be57c7301fdb8e7e69fe74ae00f7513084fda99838826ef44d2dbafe38f1` | 1 of 580 lines; path forms only |
| `_run_records/mutants/mutant_MX3_total_loads_row_dropped.filtered.log` | `3f927fbd7d8d96789a35ef0c90d1b3b6a0e88edd15b8b3e5f824466b4b6b5b0c` | `b4f0385dee0a7fabf8ded3aacdbeea20ffca9587c5cc130c07e368f27836b382` | 1 of 582 lines; path forms only |
| `_run_records/mutants/mutant_MX4_census_total_last_case_only.filtered.log` | `6446462b06519260cc8a228ef2fa4b99e0d2e1d265797c5afc1995f033bd58a9` | `c61603fa6b6a87f884e01d01fc0faf755e3dadfe92be5e4d1cb6096e360f23f7` | 1 of 586 lines; path forms only |
| `_run_records/mutants/mutant_MX5_seam_add_dropped.filtered.log` | `26dad8d2566ff89a73c56d43c95f5cb2b432d1ec266bf45879df8d49d0a90b3d` | `0bc568b6db65f22e45d7f0dbc1b09281888f7c200e68eefad108857d709c331d` | 1 of 580 lines; path forms only |
| `_run_records/mutants/mutant_MX6_b6_one_notice.filtered.log` | `f7c075056a84e96468d25ddf3cb6ec2305231902eb3bac5a81c1f86d6f7ce33f` | `97b2ed55d8c844e2c877f5b2c7948094cce10a2da4126cc2956a3a0f3815e42f` | 1 of 582 lines; path forms only |
| `_run_records/mutants/mutant_MX7_contract_evidence_one_case.filtered.log` | `bdf0e0e86c527e835a08cb99a719a8c49e9b9820e3727340c9652d018c9d0312` | `615374d28a92e7a7aa90215dee21c4c4fcf0c54353ec0cf9403bb3166226feb3` | 1 of 580 lines; path forms only |
| `_run_records/mutants/mutant_MX8_retained_error_one_case.filtered.log` | `f9fe44d2a9a1779f61cbe3cf04065b1fde767ca8b42ddaaf4e284ace05810f6b` | `c23d75dffb39273fe97499ad557cf7bded8888effa2375981a25ed01c3eb1eb5` | 1 of 582 lines; path forms only |
| `_run_records/mutants/mutant_MX9_load_cases_capacity_one.filtered.log` | `5f3432dba300bb2b2a46f447b6b4fe54a91e282810f24a5930c206ca87481771` | `8abfcb083f03f178c630f2e2f0f6867b8e1c86ee3e805628a589aca810863e64` | 1 of 592 lines; path forms only |
| `_run_records/pins/pins_base.log` | `42874682d2d415b8d01d8e248093495920fcb96381ac87c4021824d47bb8e392` | `2ffab978f6d97c78c93db3bbda61a91d52972efa6a775469ef93fd98107044f5` | 2 of 127 lines; path forms only |
| `_run_records/pins/pins_cand.log` | `5fa1c477ca8b94f29fb4df1383b73550f99f149b64f1eebd7bc2161f61c81f15` | `2eed98d3c1cd963b9f087a8a78ef04b6dc799e23ec0aea534545e1959ab5281d` | 2 of 127 lines; path forms only |
| `_run_records/scripts/cargo_cand.sh` | `761376edd7d95ee0ba25a3e5c4d33be59dbdbed7dd7bf2fc7cc24e247e0d983a` | `4ef09902bbf5164093ca020eed6fcca67f927234c34ec8ae20896920c058e425` | 1 of 9 lines; path forms only |
| `_run_records/scripts/i2_preview.sh` | `b883057e23787b8ca766079d39b4ab84687f9aece0a8cb29097f99b6aace6525` | `a578a7686c45945b86cde46b27a1547ec2537a175f3bacddc56d00881817e8b8` | 1 of 23 lines; path forms only |
| `_run_records/scripts/run_pins.sh` | `9ae60f0a167ffcf3d94e707c1d6bde976a46d93fae2dd85f341f2a657e06ad37` | `92fb63bc76cfb8c8a61dea0bdbc79a1955b3909a567bfb7621f740b70926f3ba` | 1 of 26 lines; path forms only |
| `_run_records/scripts/run_suites.sh` | `6ac10b5f9f5c64e94c25ad4deda4fc4041df6bba37ca9b944f9a8cbc9e9811af` | `d9d483d246d380e517824475142ab622d5c8495d85c2f957003debfb6db7acff` | 1 of 37 lines; path forms only |
| `_run_records/scripts/write_records.sh` | `5de8a3d7ef974d5cac12c5b9ece3b7f74b4d2f602774b532e4f227cc4667ba02` | `8340f424bd7c6a4e842451200092309a946bd0269bf3e06878a38101411a93dd` | 2 of 43 lines; path forms only |
| `_run_records/suites/base_reg_pp.log` | `155f68fa3c1387907329b0982953715e8353ea3b61ca0d52c07cdb447f800c50` | `1da039b102ffbc44a29d61563c7da92f2d74bb9cf86872e1716576098a77c3e6` | 25 of 1055 lines; path forms only |
| `_run_records/suites/base_reg_runner.filtered.log` | `011ae7927eee63eeb84d23f32545f71bf3f76a5875c118734200b937f9616610` | `1b356eba6369d4a72a76be80e8a2d5efe9295faa3f42b4888871d5e11c8bf9db` | 8 of 119 lines; path forms only |
| `_run_records/suites/base_reg_witness.log` | `bcaa407e95e0d74dc626a4022f533f73c13f7efb9f2eb67b1a8759a4d2dca54a` | `32e1e5c270326b23aa87a1af259bcb69bc671c1d37626fda44f71a843ee42d90` | 2 of 186 lines; path forms only |
| `_run_records/suites/cand_reg_pp.log` | `4bdf8a5ea6e2af8e60b3653847747b3e52ee7ed7f4637ba1eb8561363c02be7a` | `c2dc960a0a9a6286b42e6b8163ff90c116dbfa8e0a5cdc30cde5c94c63f2cb93` | 25 of 1062 lines; path forms only |
| `_run_records/suites/cand_reg_runner.filtered.log` | `eec79eb436e6f05cb7a46a53e72c13fe623dafc0de8fad3dc79b3c9806057422` | `f1e778abf19293536b84929d9d219ee6ef677ec7ebef44e452e80163d8a62010` | 8 of 119 lines; path forms only |
| `_run_records/suites/cand_reg_witness.log` | `70290ff6429910501a2963d872e30d340e28be3cf987acdecd65d96c81d5dec6` | `150f6312f44b85b8e5a7d18ab0873a4a64b1bab25ba09cf3ac03050b51b957e0` | 2 of 186 lines; path forms only |
| `_run_records/suites/cand_sa_nocapture.log` | `cdb23fa57305b126571d09e1c73c68be929bd629de133ac462d9114e626443ec` | `81c0e6158a92c1a8d940a3ff0482e7ad619ce8b2844d7b1f36bd24ee78f856d2` | 2 of 133 lines; path forms only |
| `_run_records/suites/cand_stale_retained_memory.log` | `8fa911328331ef74b471c93d889251d5cf1e9031defd171334ec2ab13e3892e9` | `cfd7992ed8b3893bd78fb620c1c7a1e61fdcaa9e659e84ed26b9e2ae75c28013` | 17 of 221 lines; path forms only |
| `FOLLOWUP_01.md` | `401d40b498d4752655cbbc45660a9aa3c054886e0e785ffacb16e9e652640d07` | `88eaca2b195a99db3b09fffb05501a3b4c6150f1c8026bb5da158ae251f3466a` | §3 reworded: the bad form described, not reproduced; a closing note points here |
| `_run_records/followup_01/sanitize.py` | `3bc9589882d849796f4775643d82458e30526324642476d9f010bc445e507066` | `48f6107788bb22de1f019e4b5b16fc23bc943e0b27bf3579f61ac246931deb6f` | the corrected sanitizer re-copied: its comment and assertions no longer spell the forms they refuse; one assertion added |

**Regenerated sum files:**

| File | Old sha256 | New sha256 | Change |
|---|---|---|---|
| `SHA256SUMS` | `edb6421433f996f6eb113fbd991d3da6ab8afa603f83175679b6fc7e0e6848b0` | `fbc315f4815e6388e11fc6d3fd5eb0668fa6f7e489b30c3b03aba9edf432161e` | regenerated over the same 75 files, in the same order |
| `SHA256SUMS.followup_01` | `9d91706ad04d2f57a73fa8aa1a6f46217ae5dfee8ac1c45668ff90e1a42a0263` | `3dfaf5665269a0e4a38e8d84a7ae63dac2d534611c4bd0167a426354814ce24c` | regenerated over the same 17 files, in the same order |

**In all:** 48 files rewritten — 46 under `_run_records/`, plus FOLLOWUP_01.md and `followup_01/sanitize.py` — and the 2 sum files regenerated. Nothing else changed.

## 4. The scan, and the sums

**The scan.** `grep -rE` over `R/I89/` for the five screened forms (a home-relative path; the home and temporary-directory roots; the worktrees folder fragment; the worktree's name) finds nothing, this file included. The patterns are as ROOT gave them.

**The sums verify on disk:**
- `SHA256SUMS`: 75 of 75 OK;
- `SHA256SUMS.followup_01`: 17 of 17 OK;
- `SHA256SUMS.redaction_01`: OK. It seals this file and `_run_records/redaction_01/` (`redact_01.py`, `sanitize_v1.py` and `report.json`).

**Kept:** the raw sources stay in `S` for RV112 until I2.
