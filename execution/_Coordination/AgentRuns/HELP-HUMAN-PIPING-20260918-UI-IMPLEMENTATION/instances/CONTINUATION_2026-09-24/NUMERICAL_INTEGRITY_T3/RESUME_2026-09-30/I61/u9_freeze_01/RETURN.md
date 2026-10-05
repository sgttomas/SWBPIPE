# I61 RETURN: U9 freeze, the evidence package regenerated (RV95 S-2)

**The package is ready to place.**
- **Where it is:** WT/scratch/i61_u9_freeze/F2A_D1/, 10 files, 192,326 B including `SHA256SUMS` (9/9 OK). That `SHA256SUMS` file's sha256 is `c8893d64c8f5a59f284bb435cc0a8be21978331c845bc28f3bd039534378ca0e`.
- **Where it goes:** it replaces `…/NUMERICAL_INTEGRITY_T3/IMPLEMENTATION/F2A_D1/` on the PR as one execution-only commit, which makes F.
- **The checks on the source head** `35d8ae59a7` (integration head NUM `bb3d766379`):
  - `source_equality.py` (check 3 generalized): **5/5 PASS**;
  - `check_citations.py`: **368/0/0 PASS**;
  - **GEN-8: 1 passed**, on a read-only scratch copy of the PR tree with the package placed.
- **No ⟨…⟩ placeholder remains.**

**Who and when:** I61 (TASK, Type 2), dispatched directly by ROOT, 2026-10-04 from 23:00Z to 23:30Z. Records only: no Git writes (reads used `GIT_OPTIONAL_LOCKS=0`), no tree edits, no machine paths.

## 1. What changed in the package (`_run_records/package_vs_cut.diff`)

| File | Change |
|---|---|
| `source_equality.py` (`18786526…`) | **Check 3 generalized:** every S file that main also changed since B must equal the recorded three-way `merge-file` of (INT, B, M), in blob and mode. A clean merge must equal the PR blob. A conflicting merge needs a rule in `MERGE_RULES`: today only `compatibility.py`, with exactly one conflict, kept on the integration side, and main's side (`def _same_canonical(`) occurring byte for byte inside it. Any other conflict fails. Check 2 now covers S minus main's files; check 5 labels the merges |
| `CHANGE_RECORD.md` (`4f73101e…`) | See the list below |
| `PR_BODY.md` (`fe78f534…`) | The same updates, concise: 139 files and both merges; the commits after the cut; RV95's verdict; N-7 ("counts within D1's caps") and the full registered identity; the U1 pin's scope and N-6; the gate results; F's gates pointed to the post-merge record. It still ends with the Claude Code line, and ROOT can paste it as the live PR body |
| `check_citations.py`, `citations.json`, `copies/*` | **Unchanged.** The NUM pin `cfcdb5997a` is on the pushed integration branch, and no cited record moved |

**`CHANGE_RECORD.md` changes:**
- **Status:** regenerated at the freeze.
- **§1:** S at the head is **139 files (59 added, 80 modified), 11,944,314 B, +204,659 / −897** against M. 137 are identical to INT; 2 are recorded merges. The package size is stated.
- **§2:** both merged files. **The corrected PR1080 sentence:** `source_blocks.rs` is in S because of the 32-bit bound, it merges cleanly with main's PR1080, PR1080 cannot run on D1 (I65, RV89), and G8's sweep shows no published byte changed.
- **§3, "After the cut":** the five source commits, NUM to PR:
  - the CI policy, `c7bc3fd54e` → `fd3cbebb42`;
  - the 32-bit bound, `7ff569a55c` → `92a5a9da1c`;
  - the U1 pin scoped to the registered target, `d069ab3ccf` → `6d8f8a82b2`, **with its target scope stated;**
  - RV95 N-3, `c5adc16384` → `35d8ae59a7`;
  - RV95 S-1, `bb3d766379` → `35d8ae59a7`.
  
  Each is given with its reason and its review status.
- **§4:** N-7's "counts within D1's caps"; **N-4 (a 32-bit review) added to the public-activation checklist.**
- **§5:** **N-6 (platform `hypot`) added to the re-qualification obligations;** N-5 routed to T6.
- **§6:** the equality and citation results at the head. The generator still reproduces the GENERATED PROFILE block byte for byte at `35d8ae59a7` (rechecked).
- **§7, the gates done:** RV95 PASS (0/2/7); G4 at the cut; G5 and G6 (transferred to the head by ROOT's ruling); G7; G8; G9a with RV89 PASS; source equality and citations; the three hosted-run defects fixed. **F's gates** (hosted CI with the full-SHA dispatch, GEN-8, the Mac baseline and DEC-025, the native witness, the frozen-head Pass B with RV89, and RV95's confirmation) are stated as **recorded in the post-merge record** `IMPLEMENTATION/F2A_D1_MERGE/` on NUM.
- **The acceptance runs** take RV95's suites (PP 705/1/10, result_export 172, runner 85/2, vitest 3,552, tsc clean).

**The size statement is exact.** It was fixed by iteration: "10 files, 192,326 B including `SHA256SUMS`", in both the record and the PR body.

## 2. Checks (`_run_records/`)

**`source_equality.py --pr 35d8ae59a7 --int bb3d766379 --main 5fdc5ab601`** (`source_equality_head.{txt,json}`): **5/5 PASS.**

| Check | Result |
|---|---|
| 1 | The PR's non-execution paths equal S (139) |
| 2 | 137 paths identical in blob and mode |
| 3 | `compatibility.py`: 1 conflict, the rule holds, expected blob `767da34027c5` = PR blob. `source_blocks.rs`: clean, `e8aadb4189d9` = PR blob |
| 4 | The 10 execution files are exactly the cut package. That is still true until F replaces it with this package |
| 5 | 139 rows: 137 equal, 2 recorded merges, none unexplained |

**Negative controls** (`equality_controls.txt`, `control_se_no_rule.py`). Each fails, as it should:

| Control | Fails at |
|---|---|
| (A) the cut head `6b9bb19a5f` against INT | Checks 1, 2, 3 and 5. The stale S lacks the three post-cut files, PP's comment repairs and the source_blocks merge |
| (B) no recorded rule for `compatibility.py` | Check 3: "1 conflict(s) with no recorded rule: needs a ruling" |
| (C) PR = INT, with no merge applied | Check 3 |

**`check_citations.py --base 5fdc5ab601 --head 35d8ae59a7`** (`citations_head.txt`): **368 resolved, 0 ambiguous, 0 unresolved, PASS.**
- 65 record and RR citations;
- 289 design-document citations;
- 14 pinned code lines;
- 0 verification failures and 0 unused entries.

**GEN-8** (`gen8_scratch.txt`): **1 passed, 10 deselected.**
- It ran on a `git archive` of the head (72,101 tracked paths, no export-ignore) with the regenerated package placed. The copy was read-only to the repository and kept in WT/scratch/i61_u9_freeze/pr_tree.
- Outside a Git worktree GEN-8 walks everything, a superset of what F tracks.
- ROOT still runs GEN-8 on F after placement, as the freeze sequence requires.

## 3. Not done (optional)

- **RV95 N-2** (residues outside the citation patterns: `ruling` strings in the carrier case file, run mentions such as "I66 u6b", and two accurate bare line numbers) is not extended here. It is optional and was noted by ROOT; the check passes without it.

## 4. Files

- `_run_records/`:
  - `source_equality_head.txt` and `source_equality_head.json`;
  - `equality_controls.txt` and `control_se_no_rule.py`;
  - `citations_head.txt` and `gen8_scratch.txt`;
  - `package_SHA256SUMS` (the package's own list) and `package_vs_cut.diff`.
- `SHA256SUMS`.
- **The package itself** is at WT/scratch/i61_u9_freeze/F2A_D1/, for ROOT to place; the cut copy is alongside as `F2A_D1_cut/`.
