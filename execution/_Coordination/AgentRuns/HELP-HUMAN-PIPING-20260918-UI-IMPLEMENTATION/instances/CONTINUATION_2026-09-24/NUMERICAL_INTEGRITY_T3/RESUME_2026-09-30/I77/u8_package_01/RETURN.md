# I77: U8's compact PR evidence package (records only)

TASK (Type 2), I77, for ROOT (HELP_HUMAN, Agent 0). 2026-10-06 UTC. No delegation.

**Brief:** ROOT's dispatch message, recorded as RR "I77 dispatched: U8's compact PR evidence package" (committed at NUM `e257461b3c`, after the pin). There is no brief file.

**Placeholders:** `WT`, `NUM`, `P`, `PP`, `RE`, `DT`, `T`, `R` and `RR` as in the dispatch. `PKG` = `T/IMPLEMENTATION/U8`. `SCR` = `WT/scratch/i77_u8_pkg`.

**Read:**
- `NUM/AGENTS.md`, `NUM/agents/AGENT_TASK.md`, `NUM/P/AGENTS.md`;
- #1082's package `T/IMPLEMENTATION/F2A_D1/`: CHANGE_RECORD, PR_BODY, SHA256SUMS, `check_citations.py`, `citations.json` and `source_equality.py`;
- `T/IMPLEMENTATION/F2A_D1_MERGE/RECORD.md` and `ERRATA.md`;
- RR from "I61's U8 plan ruled; …" (11898) to "U8 Pass B returned; …" (12817), and "T3's gate set and Git rules, consolidated…" (12262);
- U8's records: `R/I68/u8_probe_01/` and `R/I68/u8_witnesses_01/`; `R/I69/u8_corpus_07l_01/`, `R/I70/u8_rust_07l_01/` and `R/I71/u8_ts_07l_01/`; `R/REVIEW_RV97/u8_01/` (both rounds); `R/I72/u8_passb_01/`;
- PLAN §1 (`R/I61/u8_plan_01/PLAN.md`);
- the full U8 diff `b1e2d7741e..bd6b4be2c3`, except the bodies of the two L = 0 fixtures. Those are producer output, which the citation scan covers.

**Main's scripts, used unchanged** from `c1bfc460fc:T/IMPLEMENTATION/F2A_D1/`. They are blob-identical to NUM's copies; their sha256s are in `_run_records/main_tools.sha256`. They were extracted to `SCR/main_tools/` with `git show`, not copied into the package.

## 1. What I wrote

**The package, `PKG`:**

| File | sha256 | Bytes |
|---|---|---|
| `CHANGE_RECORD.md` | `fd5e4d4908c9783a4b7725c21c523d1a659dc68dd93654e009a7455f9f558ee7` | 17,720 |
| `PR_BODY.md` | `143ee2fadaa50ee4c8a0860539f3fe5e555084594b943e020d6676815039d5d7` | 4,482 |
| `citations.json` | `ffa4048142ca8d705d51decb25ed99c76c65efc98680a18db94b240e071b3a4d` | 28,804 |
| `SHA256SUMS` (the three above; `shasum -a 256 -c` OK) | `e5e11bc8d34438033cc7ecd86a02ed5335dfabf96f48992434ceaa8d8605af67` | 241 |

- **The change record and the PR body** follow #1082's form, scaled down. The gates not yet run are marked **PENDING**.
- **The PR body** ends with the attribution line.
- **No machine paths** appear in either.

**This folder:**
- `RETURN.md` and `SHA256SUMS`;
- `_run_records/scripts/`:
  - `build_index.py` builds `citations.json`;
  - `verify_named.py` checks `named_references`;
  - `negative_controls.py`;
  - `make_skeleton.py`, the empty index used for the first listing;
  - `find_sites.py`, which locates named-reference sites;
- `_run_records/outputs/`: every check output below, and the `source_equality.py` pre-check.

## 2. How the index was built

1. **The listing first.** I ran main's `check_citations.py --list` with a skeleton index: #1082's documents table, no entries, and the pin `fd3990a710` (`outputs/list_skeleton.txt`). It found:
   - 5 record and RR occurrences: `R/I68/u8_probe_01` ×2, RR "I61's U8 plan ruled" ×2 (PP, Python), and RR "I68's probe verified" ×1;
   - 1 bare code line, `zz_rv93.rs:292–315`;
   - 0 document citations among #1082's 23 names.
2. **Then I read the diff myself** for what the tool's classes miss (`outputs/named_sites.txt`, 63 site lines).
3. **The index** (`citations.json`, schema `u9-citation-index-v1`):
   - **`num_commit`** `fd3990a7100b3c5f191deaa647ea0855d2cc2fa3`, `source_basis` `bd6b4be2c3…` and `source_base` `b1e2d7741e…`. `records_root`, `rr_path` and `github` are #1082's.
   - **`citations`:** the record path, with `num_paths` I68/u8_probe_01 and its PROBE.md; the two RR titles, with heading lines 11898 and 12344.
   - **`documents`:** #1082's 23 names unchanged, plus two new entries:
     - **SNAPSHOT_05_PLAN,** one path;
     - **PLAN,** which has 16 candidates: T3's PLAN.md files, excluding sealed copies and run inputs. A context rule for PP's `retained_facade_tests.rs` resolves it to `R/I61/u8_plan_01/PLAN.md`.
     
     Both documents are blob-identical on main, so the tool reports "main: …".
   - **`code_anchors`:** one entry, of kind `record`. It pins `R/REVIEW_RV93/u3_grant2_01/evidence/probe/zz_rv93.rs` at the NUM pin, recording the text of lines 292, 296, 298, 299 and 315. Lines 292–315 are `fn zz_rv93_input_fallbacks`.
     - PLAN §1.2 and I68's brief cite this exact path.
     - `evidence/addendum_01/zz_rv93.rs` holds the same text at 292–315.
   - **`named_references`** (15 entries; the tool ignores this key) holds the citations the tool has no class for:
     - review findings: RV93 N-5, RV94 N-3, RV90 S1/N1/N2/N4, C04;
     - decisions: D-U6-5, decision 6, W-C1/W-C2, D-U7-2, D11/D37;
     - attributions: U8-0/I68, I69;
     - the `§3` anchor of `R/I68/u8_probe_01`;
     - the U5 criterion (`u5_compare.py:104`);
     - the `u8_head` commit;
     - the wrapped code-line citation.
     
     Each entry gives its sites (relative to P at the U8 head) and its targets (a record line, an RR line under its heading, a repository line, or a commit). Entries carried from main's text in modified lines are marked `carried`.
   - **Not indexed:** unit and snapshot names (U1, U5, U7, U8-1…U8-3, 07h–07l) and terms (B′, G-C, N1). They are T3 vocabulary, not record citations.

## 3. The citations check, verbatim

`python3 SCR/main_tools/check_citations.py --repo WT/f2a-u8 --base b1e2d7741e --head bd6b4be2c3 --index PKG/citations.json --package PKG --out …/resolved.md` (rc 0; `outputs/check_pass.out`):
```
COUNTS resolved 10; ambiguous 0; unresolved 0
  records/RR: occurrences 5, distinct 3, unresolved 0; documents: 4 citations (4 resolved, 0 ambiguous, 0 unresolved); verification failures 0; unused index entries 0; code lines: 1 (1 pinned in code_anchors, 0 unresolved)
RESULT PASS
```
- **The same result** with `--base c1bfc460fc` (main) and with `--head WORKTREE` (WT/f2a-u8, clean at `bd6b4be2c3`). See `check_base_main.out` and `check_worktree_mode.out`.
- **The resolved table** is `outputs/resolved.md`.

**`verify_named.py`** (rc 0; `outputs/verify_named.out`): 15 entries, 63 checks, 0 failed.
- Every site holds its token at the U8 head.
- Every target holds its text at the pin, and every RR line lies under its stated heading.
- `retained_memory_witness_tests.rs` has the same blob at the U8 head and on main (`9745e3fe38ef`).
- `d449097085` is reachable, among remote branches, only from `origin/codex/piping-f2a-u8-20261005`. It is not an ancestor of NUM `fd3990a710` or of main `c1bfc460fc`.

**Negative controls** (`outputs/negative_controls.out`), all as expected:

| Control | Result |
|---|---|
| PLAN's rule removed | rc 0, the 3 PLAN citations listed as AMBIGUOUS |
| The code anchor removed | UNRESOLVED, rc 1 |
| A wrong pin (`b1e2d7741e`, without U8's RR rulings) | the tool stops with an `IndexError`, rc 1 |
| One anchored line's text altered | UNRESOLVED, rc 1 |
| An RR heading line moved | FAILED, rc 1 |
| An RR entry removed | UNRESOLVED, rc 1 |

**`source_equality.py` shape pre-check** (`outputs/se_precheck.*`; not the gate). Run as `--pr bd6b4be2c3 --int bd6b4be2c3 --main c1bfc460fc --work SCR/se_work`:
- B = `c1bfc460fc`; |S| = 7;
- checks 1, 2, 3 and 5 PASS. Check 3 reports #1082's `compatibility.py` rule as not needed;
- check 4 FAILs only because no package exists at that head (rc 1, as expected).

## 4. For ROOT to rule on

1. **The record-file code anchor** (`zz_rv93.rs:292–315`). #1082's rule forbids bare code-line citations, except its generated and corpus-data anchors.
   - This one cites a record at a pinned commit, which cannot move, so I pinned it rather than leave the check failing.
   - **Alternatives:** reword the PP comment to name `zz_rv93_input_fallbacks` at the file's next touch (B1), or rule the anchor kind in.
2. **PLAN's context rule.** Like #1082's COMP, a bare document name with several candidate files needs ROOT's ruling. I propose the U8 plan for PP's U8 section; the evidence is in the entry's `why`.
3. **`named_references`.** This key is outside the tool's classes, checked by my `verify_named.py`, which lives in these records rather than in the package.
   - **Keep it** as a reader's index;
   - **or drop it** from the package and keep only the records;
   - **or later teach `check_citations.py`** the classes it lacks.
   
   It does not change the tool's verdict.
4. **The wrapped code-line citation** `retained_memory_witness_tests.rs` / `:181–199` (PP `retained_facade_tests.rs:851–852`).
   - It is a bare citation of maintained code, the form #1082's rule forbids.
   - The tool's per-line regex cannot see it, because the line number wraps onto the next comment line.
   - It is accurate at the U8 head and on main (`fn w6_input()`, lines 181–199).
   - **The choice:** accept it as is, or name `w6_input()` at the next touch (with RV97 R2-N-2 in B1). Teaching the tool to join wrapped comment lines would be a tool change.
5. **The `u8_head` commit** (`d449097085`) is cited in the corpus provenance and asserted by Python's test.
   - Today it is reachable only from the pushed U8 branch.
   - After ROOT merges U8 into NUM, it is on NUM. It is never on main, because the PR carries the 7 files, not the branch history.
   - Keep the U8 branch, or NUM, pushed.
6. **Main has moved.** The local `origin/main` is `c1571f7feb`: #1098 and #1099, 209 files, all under `projects/chirality-app-v4/`. No piping path and none of the 7 files change. NUM absorbed #1098 at `10df4a37ea`.
   - **So:**
     - the PR is cut from the newer main;
     - DEC-025's fresh baseline is that main;
     - `source_equality.py`'s B will be NUM's merge base with main (`0329f8fe6b` today).
   - **S stays exactly the 7 files** if NUM then carries no other maintained change beyond main. That holds now: NUM `fd3990a710` against `0329f8fe6b`, outside execution, shows 0 files. It would fail if T6S or S-I1 were merged into NUM first.
   - I did not fetch, so a later main is unknown to me.
7. **For the run on the PR head:**
   - pass `--index …/U8/citations.json` explicitly, since the tool's default is #1082's index;
   - after filling the **PENDING** rows, regenerate `SHA256SUMS`, which `source_equality.py` check 4 verifies against the PR's blobs;
   - RV97's confirmation should read the final package.
8. **Minor, tool robustness (no action needed for U8).** A pin that lacks the cited RR lines makes `check_citations.py` raise `IndexError`. That is fail-closed (rc 1), but it gives no FAILED line.
9. **Noted, not a ruling.** On PP `:831`, the second title ("… and "I68's probe verified…"") has no `RR` prefix, so the tool does not detect it there. The same heading is detected and verified at `:870`.

## 5. Host, fence and cleanup

- **Writes:** `PKG` (4 files), this folder and `SCR` only. The host's write guard accepted every write into NUM.
- **Git:** reads only, with `GIT_OPTIONAL_LOCKS=0`: `show`, `diff`, `log`, `rev-parse`, `ls-tree`, `cat-file`, `grep`, `branch --contains`, `merge-base`, `status`.
  - **Disclosed:** one `git fetch --dry-run`, run while checking `origin/main`. It updates no ref, and FETCH_HEAD's mtime is unchanged (2026-10-05 22:11 local).
  - No Git writes.
- **System temp, disclosed:** two transient files went to `/tmp` during the site scan and an RR read. One was moved into `SCR` at once and the other deleted; neither remains.
- **Not run:** no cargo, native, solver or DEC-025 job; no installs; Python 3 with the standard library only.
- **Left in `SCR`, about 1.3 MB, for ROOT's verification:**
  - main's three scripts;
  - the skeleton index;
  - the build outputs and negative-control outputs;
  - a copy of RR at the pin;
  - an empty `se_work/`.
  
  It is reproducible from these records, and ROOT's cleanup may remove it.
