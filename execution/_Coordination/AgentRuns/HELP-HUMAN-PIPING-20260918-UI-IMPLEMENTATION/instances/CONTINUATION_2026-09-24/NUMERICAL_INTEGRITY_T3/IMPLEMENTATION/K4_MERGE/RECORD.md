# K4 merge record

- **PR:** https://github.com/sgttomas/chirality/pull/1054.
  - ROOT (HELP_HUMAN) merged it on 2026-09-29 at 16:25:25Z as `ab02ee3a68a7af8eb120fa7caa4a2d3daba47191`: a merge commit with `--match-head-commit 5a46a6278`, under the owner's standing Git authorization.
  - Main was `7ac7b1c37` (K6), an ancestor of the head, and the merge state was clean.
- **Candidate head:** `5a46a6278af3c857a52ecb9be6880990493190f8`, on branch `codex/piping-k4-20260928`.
- **Where it ran:** the owner's Mac (`aarch64-apple-darwin`, rustc 1.97.1). ROOT dispatched the implementer (I12) and the reviewer (RV19) directly.
- **Scope:** T3 slice K4, W1a's kernel method (retained precision), under `FK/src/structural/retained/`, with its tests in `FK/tests/retained_k4/`.
  - The basis is D1 revision 5a.2 as amended by revision 5a.3 (R7 §5; "D1 revision 5a.3 SELECTED"), with **amendment A1** (ROOT's ruling on RV19-6).
  - **Kernel only.** Every item is `pub(crate)` inside a private `mod retained`, so no product path reaches it. F2a wires W1 into the product.
  - The account is `IMPLEMENTATION/K4/CHANGE_RECORD.md` and `RETURN.md` (with addenda 1 and 2), on the merged branch.

## The chain (branch point: main `e7d930d49`)

| Commit | Content |
|---|---|
| `cef218a10` | Checkpoint A1: the method, tests B to F |
| `3ed6c0e26` | Checkpoint A2: combinations as their own solve (F-1), tests G to L, R1's references lane, the O8 emulation |
| `3668ee8a4` | The Q10 encoding pins; drafts of the records |
| `5ad1b6174` | A combination's prescribed rows published from their exact sum, rounded once (V4's note; K4-M33); the V4-S3 probe |
| `ba88db495` | A merge of main `59cb20073` (F1b) |
| `03e7250b2` | Checkpoint A3-0: the plan for revision 5a.3 |
| `8dfade92e` | Checkpoint A3a: the certified bounds and the formation scale, with no behaviour change |
| `bb7757ac5` | Checkpoint A3b: revision 5a.3 wired into the method, with every control |
| `8a59458a1` | Checkpoint B: ê overflow mapped to `ResolutionScaleUnencodable`; DIRECTIONAL-SPAN's 5a.2 publication shown within b; suites |
| `8f8023a20` | A merge of main `7ac7b1c37` (K6), with no FK overlap |
| `7d8fa9c0e` | Checkpoint D: the honesty predicate tightened to each row's published claim; C's mutation records; RETURN and CHANGE_RECORD |
| `a5fa0eaf7` | RV19's review fixed (RV19-1 to RV19-5), and amendment A1 (RV19-6) |
| `5a46a6278` | RV19-D4 pinned, and PRECISION-RULE checked (DN2): tests, generator output and records only |

Checkpoint C (mutations) changed no code; its records are in D.

## Gates

- **Independent review, RV19** (`REVIEW/K4_REVIEW.md`, with `REVIEW/_run_records/k4_review/`):
  - **FAIL** at `7d8fa9c0e`: 1 BLOCKING, 5 SHOULD-FIX and 7 NOTEs.
    - **RV19-1 (BLOCKING):** a selected row outside its claim. The stop rule's S\* (`scales_at`) included rows the candidate cannot publish, which the classification's S\* excludes (O9). OVF-ROT-928 published Rz = −2^901 as `relative_verified` against an exact 0.
    - **RV19-6:** a design-level gap. Under D1's rule for S\* < 2^-988, the published bound b omits the publication rounding; TINY-S-995 missed its claim on seven rows. ROOT ruled amendment A1.
    - RV19-2 to RV19-5: the honesty test's silent skips and uncovered controls; combinations with differing stations or support groups; and two surviving mutants.
    - RV19 confirmed I12's §6 item 4 (θ ≤ 1/2 with the certified B makes a data-carrying block's K\* nonsingular). ROOT adopted it as a ruling.
  - **PASS** on the delta at `a5fa0eaf7`: 0 BLOCKING, 1 SHOULD-FIX (RV19-D4, a test gap on the RV19-1 fix) and 4 NOTEs.
  - **PASS** at the final head `5a46a6278`, with no findings. RV19-D4 and its underflow-only variant are killed from clean archives.
  - **RV19's independent oracle** (a 1,600-digit decimal solve written from D1 §4.1, not from GEN) finds every selected publication within its claim: 120 publications (8,272 rows) and the six 100-member frames (15,378 rows). GEN's 128-bit expectations agree with it on all 28,735 keys.
  - ROOT's rulings: `ROOT_RULINGS_V1.md`, "K4: rulings on RV19's review" and "K4: rulings on RV19's delta check at a5fa0eaf7".
- **Tests at the final head:** FK's full suite 394 passed, of which K4's are 127. `gen_k4_vectors.py --check` gives 23 of 23. The controls test checks 117 selected controls and combinations (9,412 checks; the worst is 0.949 of its allowance).
- **Mutants:**
  - at C: every killable mutant, including R7 §7's list and K4-M34 to M40;
  - at D: the evidence pass under the tightened predicate;
  - on RV19's fixes: the reverted RV19-1 fix, the reverted amendment A1, RV19-M2, RV19-M6 and RV19-D4, all killed.
  - Under the final checks, R7-M1's false claims are caught on ASSEMBLY-SAT, F-2, F-2-CEIL, F-2-SPOS, PRESCRIBED-TAIL and PRESCRIBED-TAIL-FREE.
- **Hosted CI on `5a46a6278`:**
  - pull_request runs: Piping Desktop E2E **36593109243**, Harness Pre-merge **36593109220**, governance-harness **36593109362** and pec-tests **36593109318**, all successful. 12 checks passed and 4 were skipped as selected;
  - the full-SHA dispatch **36593106169** (target_base `7ac7b1c377b614e2276d0b828205ac288ced5829`): success;
  - **the numerical cargo job's time (K4's brief, Q12):** 20.1 min on the pull_request run and 18.3 min on the dispatch, against 14.1 min on K6's dispatch (36548351414). The budget is 45 min;
  - the earlier heads were also green: `7d8fa9c0e` (dispatch 36570042397) and `a5fa0eaf7` (dispatch 36584771469, with its four pull_request runs).
- **DEC-025,** under the owner's Mac decision (`OWNER_DIRECTION.md`, 2026-09-28), on the final head `5a46a6278`. The evidence is in `dec025/`, and the driver is `M03_SKEW_PIN_MERGE/dec025/dec025_mac.sh.txt`, unchanged. **The sweep's shared target was fresh:** ROOT deleted it before the sweep and after (K6's procedure note).
  1. **The sweep invocation** failed at the cargo surface on product_physics's platform test `t13` (fail-fast), as expected on the Mac. The summary JSON is sanitized; its original sha256 is in `sweep_json_original_sha256.txt`.
  2. **All 39 manifests** were run with `--no-fail-fast`, against K6's Mac run at `cd325c1fe`, whose piping source equals main `7ac7b1c37`'s (`suites_vs_baseline.txt`).
     - frame_kernel grows 267 → 394 tests, K4's own.
     - operation_applier reads 0 → 194. The 0 is the baseline's shared-target build failure (`K6_MERGE/RECORD.md`), and 194 equals K6's own fresh-target re-run. K4 changes no file in operation_applier or its dependency closure.
     - The failing tests are exactly the baseline's three Mac platform tests.
  3. **Surfaces 2, 3 and 5:**
     - pytest: 3062 passed, 32 skipped;
     - the wasm build: exit 0;
     - vitest: 134 files, 2822 of 2822;
     - the production build: exit 0.
  - **An earlier DEC-025 at `a5fa0eaf7`** (`dec025/earlier_a5fa0eaf7/`) gave the same result, with frame_kernel at 392.
    - Its first sweep invocation found the tree dirty: K6's sweep summary had been left untracked in the sweep worktree, so the summary went to a temporary path. That log is in `first_invocation_dirty_tree/`.
    - ROOT checked the leftover was byte-identical to K6's recorded copy, removed it, and re-ran the invocation on a clean tree.
    - **Procedure note:** the driver copies the canonical summary but leaves it in the sweep worktree. Remove it after each sweep, as ROOT did after this one.
  - Machine paths are sanitized to `<WORKTREE>`, `<VENV>`, `<wt>`, `<home>` and `<tmp>`, and ANSI colour codes are stripped.
- **GEN-8** on `5a46a6278`: 1 passed (ROOT, with `set -o pipefail`). It also passed at `7d8fa9c0e` and `a5fa0eaf7`.
- **T9 and the both-entry gate:** not run. K4 is kernel only (K4's brief), and RV19 re-verified the scan: no product path reaches `retained`.

## What K4 established

- **W1a's kernel method runs, and is honest on every control checked:**
  - the retained-precision schedule 128 → 256 → 512, with 1024 for verification only;
  - revision 5a.3's acceptance rule, with no uncertified step in the honesty guarantee;
  - combinations solved as their own case;
  - the classification with its bounds.
- **Amendment A1 (D1 revision 5a.3):** where 0 < S\* < 2^-988, each `absolute_verified` row carries b_row = fl↑(fl↑(2^-64·S\*) + fl↑(2^-53·|q_pub|) + 2^-1074). b is unchanged where S\* ≥ 2^-988. `DESIGN.md` stays hash-pinned; the amendment is ROOT's ruling.
- **The stop rule's S\* and the publication's S\* come from the same rows** (O9, applied to both).
- **Deterministic work counts** per stage and precision (RETURN §14), for ROOT's W1 limits with K6b and V-K.

## Routed and open

- **Next:**
  - **K6b (I16):** its checkpoint A0 adds FK's `retained` export, exactly RETURN §16's list;
  - **V-K (I17):** it reuses the same export commit;
  - **then ROOT's W1 limits** from K6, K6b, V-K and K4's work counts, before F2a merges.
- **For F2a:**
  - the limits (Q5, as amended);
  - Q9 (a body with no data block has no B_b entry, and θ = 0);
  - unifying `Binary64Outcome` with K2b's `Representability` (Q11);
  - the zero conventions.
- **For D2:** G5 checks amendment A1's b_row, and Q9.
- **Deferred by ruling:**
  - a full geometric treatment of partial directional grounds (O1, W4/K5);
  - W1b, until F3 meets R7 §6.5;
  - the open or argued items of RETURN §19, none of which is a step of the honesty guarantee.
- **Recorded NOTEs:**
  - RV19-N3, N5 and N7;
  - RV19-DN1: GEN marks three RF-LARGE-TREE-n00010-ROT keys `underflow` where the truth is 0. No check reads the marker there.

## Scratch to prune

`<wt>/k4-target`, `<wt>/k4-b` and `<wt>/k4-b-target`, `<wt>/k4-mut`, `<wt>/k4-rv19`, `<wt>/k4-rv19d`, `<wt>/scratch/i12`, and `<wt>/scratch/sweep_k4` and `sweep_k4_a5fa0eaf7`. The records above keep every hash.
