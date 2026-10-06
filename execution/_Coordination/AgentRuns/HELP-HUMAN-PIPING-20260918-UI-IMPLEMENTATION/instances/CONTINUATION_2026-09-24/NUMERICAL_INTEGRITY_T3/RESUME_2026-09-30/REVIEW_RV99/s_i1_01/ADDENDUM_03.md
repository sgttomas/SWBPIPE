# RV99 addendum 03: #1100's amended head `ef266247de` and the DEC-025 carry-over premise

**Reviewer:** RV99, TASK (Type 2), the same reviewer, continued by ROOT (HELP_HUMAN, Agent 0). ROOT is the return path. No descendants.

**Placeholders:** as in `REVIEW.md`. `T` is the T3 records folder. No machine paths are recorded here or in `evidence/round3/`.

**Basis read:**
- ROOT's two messages (the request, and the resume after a network interruption);
- RR "RV99 confirms #1100's head; package wording amended; DEC-025 carried by ruling to the new head";
- ROOT's records in NUM `T/IMPLEMENTATION/S_I1_MERGE/_run_records/` (`SHA256SUMS.run_records`, 8 of 8 OK when checked from `_run_records/`).

**Work:** read-only Git (`GIT_OPTIONAL_LOCKS=0`), main's two gate scripts, and one read-only repo-root validator. No cargo and no Git writes. The S-I1 worktree (head `ef266247de`) stayed clean.

**After the interruption:** my partial round-3 scratch (`WT/scratch/rv99_s_i1_01/r3/`) held the exported package and a walker list. I re-checked the package files against the head (4 of 4 identical) and regenerated everything else before relying on it.

## Verdict: **CONFIRMED.** The delta is the package text only, N-8 to N-11 are addressed truthfully, the carry-over premise holds for the changed paths, and both gate tools reproduce ROOT's records exactly.

| | Count |
|---|---|
| BLOCKING | 0 |
| SHOULD-FIX | 0 |
| NOTE | 2 (information; neither affects the carry-over) |

## 1. The delta `20e7e3e5a2..ef266247de`

- **The files:** 3 paths, all under `T/IMPLEMENTATION/S_I1/`: `CHANGE_RECORD.md`, `PR_BODY.md` and `SHA256SUMS` (+10 / −9; `evidence/round3/package_delta.diff`).
- **Outside `projects/chirality-piping/execution/`, the two trees are equal** (0 paths differ). The 8 slice files are untouched.
- **The package's `SHA256SUMS`** verifies 3 of 3 at `ef266247de`.
- **NUM `f9657c51c5`** carries the same 4 package blobs as the PR head.
- **The PR branch head** equals its remote-tracking ref (`ef266247de`).

## 2. N-8 to N-11 in the amended text

| Note | New text | Assessment |
|---|---|---|
| N-8 | PR body: "Gates run before the merge, and recorded on the integration branch: source equality, citations, GEN-8, hosted CI …, and the Mac DEC-025 …" | Truthful. It no longer claims the pending gates are already recorded. |
| N-9 | PR body: "…30,609 evaluator cases (exact-rational and point-path oracles) and 6,615 runner cases (point-path oracle)"; "63,086 runs over the committed rule packs and run fixtures, plus generated packs and variants". Change record: "450,000 exact-rational samples", "6,615 runner cases (against the point path)", and the same differential wording. | Truthful; matches my review. |
| N-10 | Change record: "…reads U, apart from 15 boxes with invalid explicit overlays, which are blocked. None passes."; "a NaN interpolation or step-lookup argument (an exact lookup blocks instead)" | Truthful; matches `ADDENDUM_01.md` §1 and my probes. |
| N-11 | Change record: "I73: … src-tauri 116 … RV99 reproduced the rules crates' and the Python counts"; and in §3, "Soundness is relative to the supplied bound b (D2 §4.11.2). b is operational evidence carried by the receipt, not a forward-error enclosure of the exact solution." | Truthful. I reproduced 49+1, 33, 10 and 193 (`ADDENDUM_01.md` §3), not src-tauri. The b sentence states D2 §4.11.2's limit. |

**No new overclaim.** Nothing else in the two files changed.

## 3. The DEC-025 carry-over premise

**What DEC-025 runs.** I read `run_dec025.sh`, `dec025_mac.sh` and `run_suites_nff.sh` in WT scratch, and `tools/release/run_evidence_sweep.py`. The surfaces are:
- the sweep itself;
- every cargo manifest under `core/` and `validation/benchmarks/` (`discover_cargo_manifests`);
- `pytest -q tests`;
- `npm run build:wasm:desktop`, `test:desktop` and `build:desktop`.

**Explicit references, at `ef266247de`** (code, config and data under `projects/chirality-piping/`, outside `execution/` and `validation/evidence/`; `evidence/round3/t3_and_package_references.txt`):
- **No file names `IMPLEMENTATION/S_I1` or `S_I1/`.**
- **The only code naming `NUMERICAL_INTEGRITY_T3`** is the three known readers: `gen_k4_vectors.py`, `gen_vk_cases.py` and `run_harness_mutants.py`. Each reads only `REFERENCES/references.{json,py}` and `DESIGN_NUMERICS/_run_records/floor_kinds.json`. `run_harness_mutants.py` also runs a Git path query over `P` with `execution/` excluded.
- **The other mentions** are comments in Rust tests, or data: `validation/portability_policy.json`'s historical-record ledger and fixture provenance strings.

**Recursive readers** (`walkers_recursive_readers.txt`, plus every Python `.glob`):
- every walker reachable from a DEC-025 surface is rooted at `apps/`, `core/`, `tools/`, `apps/desktop/src`, a crate's own folders, a fixture folder, or `core/` + `validation/benchmarks/` (manifest discovery);
- the two that walk wider (`retained_precision_carriers.rs` and `feature_guard.rs`) explicitly skip `execution`;
- none is rooted at the project root or at `execution/`.

**So no DEC-025 suite reads the 3 changed files.** Outputs bound to the commit hash (the sweep summary's name and `git` block, and tests that read `HEAD`) differ by the hash only.

**N-12 (NOTE): the "execution/ tree generally" wording is too narrow; the carry-over stands.** Other piping code reads `execution/` paths outside T3:
- `tests/security/test_redaction_export_controls.py` and `test_secret_private_library_handling.py`: PKG-12 `MEMORY.md` files;
- `tests/test_handoff_export_workflow.py` and `test_handoff_package_schema.py`: PKG-15 fixtures;
- `tests/test_architecture_basis_validation.py`, via `tools/validation/validate_architecture_basis.py`: PKG-00, `_Decomposition/SOFTWARE_DECOMP.md` and `_DECISIONS/D-43…`;
- release-readiness and coordination tools: `_DAG/` paths.

`nativeMechanicsReplay.ts` names an `ENGINE_INTEGRATION` capture only as a label string; it does not read the file. None of these paths differs between the two heads, so the premise as it bears on the changed paths holds. For future carry-overs, the precise premise is "no DEC-025 suite reads `T/`, beyond the three readers of `REFERENCES/` and `DESIGN_NUMERICS/`".

**N-13 (NOTE, outside piping and outside DEC-025; information for ROOT).** Two repo-root tools do read AgentRuns records, the package included:
- **GEN-8** (`tools/practitioner_harness`): ROOT's `gen8_2.txt` shows it passing at `ef266247de`.
- **`tools/validation/validate_path_anchors.py`**, run by the governance-harness workflow. My read-only run at `ef266247de` (`path_anchors_readonly.txt`) finds nothing in piping or the package (which contains no home paths). It exits 1 on a single finding: a literal home path in `projects/chirality-app-v4/execution/_Coordination/_COORDINATION.md:18`. The merged tree equals main outside the slice, so that finding is main's App v4 content, not S-I1's.

## 4. `source_equality.py` and `check_citations.py` on `ef266247de`

**The tools** are main's copies from `T/IMPLEMENTATION/F2A_D1/` (blobs `fb96decf…` and `50c39a8a…`, equal at main and at the head), run with ROOT's arguments:
- **source_equality:** PR `ef266247de`, INT `f9657c51c5`, MAIN `c1571f7feb`, `--package T/IMPLEMENTATION/S_I1`;
- **check_citations:** base main, head `ef266247de`, and the head's `citations.json`.

| Tool | My result | ROOT's record |
|---|---|---|
| `source_equality.py` | exit 0, **RESULT PASS**, all 5 checks pass (\|S\| = 8; 8 identical; no merge rule needed; 4 execution files, all inside the package, sums verified; 8 rows equal) | `se2.txt` **identical**; `se2.json` **byte-identical** |
| `check_citations.py` | exit 0, **RESULT PASS**: 34 resolved, 0 ambiguous, 0 unresolved, 0 verification failures | `citations2.txt` **identical** |

## Records

- `evidence/round3/`:
  - `package_delta.diff`, `t3_and_package_references.txt`, `walkers_recursive_readers.txt`;
  - `se2.txt`, `se2.json`, `citations2.txt`, `resolved2.md`;
  - `path_anchors_readonly.txt` (the home path elided), `commands.txt` and `inputs_sha256.txt`.
- `SHA256SUMS.addendum_03` covers this addendum and `evidence/round3/`.
- `REVIEW.md`, `ADDENDUM_01.md`, `ADDENDUM_02.md` and their sums are unaltered.
