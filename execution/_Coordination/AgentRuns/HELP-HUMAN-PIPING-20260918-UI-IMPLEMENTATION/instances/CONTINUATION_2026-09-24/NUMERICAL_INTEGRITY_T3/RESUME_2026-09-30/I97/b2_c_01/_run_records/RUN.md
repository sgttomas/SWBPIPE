# I97 B2-C: execution record

Placeholders as in CONTRACT.md: WT, NUM, P (NUM's `projects/chirality-piping`), R, VENV. `S` = my scratch folder `WT/scratch/i97_b2_c/work` (disposable).

**Environment.** Python 3.13.14 from VENV; `jsonschema` 4.26.0 (VENV's). Every Python run had `PYTHONDONTWRITEBYTECODE=1` and `TMPDIR=WT/scratch/i97_b2_c/tmp`. Git reads used `GIT_OPTIONAL_LOCKS=0`. NUM was at `cebff253d6`; its maintained tree outside `P/execution` equals the dispatch commit `6aa2878872`'s (`git diff --quiet`). Nothing was written outside the scratch folder and this record folder.

| Step | Command (in `S`) | Output | Runs |
|---|---|---|---|
| S1 | `VENV b2c_statics.py NUM/P R/I96/b3_d_01 out_a` | `out_a/`: DEF-C, the revised PTABLE, `SCHEMA_B2.diff`, the merged J1 SCHEMA, `SCHEMA_J1.diff`, `b2c_statics.out.json` | 2 (`out_a`, `out_b`): all six files byte-identical (`diff -r`) |
| S2 | `VENV b2c_checks.py NUM/P NUM/P/core/analysis_runs/retained_precision.py out_a/retained_precision_mp_v2.schema.json checks_a.out.json` | `checks_a.out.json` | 2 (`checks_a`, `checks_b`, the second on `out_b`'s schema): byte-identical (`cmp`) |
| S3 | `sh b2c_collisions.sh NUM` | `collisions_a.out.txt` | 2: identical (`cmp`) |

**What `b2c_statics.py` imports.** It imports I96's `b3d_statics.py` and `b3d_statics_r1.py` from their record folder, unchanged (sha256 in `b2c_statics.out.json` `inputs`), only to apply B3b's own SCHEMA change in the merged J1 text. The file it reads is in `R/I96/b3_d_01/_run_records/` and `revision_01/`.

**What `b2c_checks.py` loads.** It loads NUM's committed PY reader file by path, only to call its G1 walker `_shape`. It replaces the module's `_schema` loader, in memory, with each schema in turn. It writes nothing into the repository.

**Placed in the record** (copies of S1–S3's `_a` outputs, `cmp`-equal):
- `statics/`: `retained_precision_prepared_combination_v1.json`, `semantic_contract_v0_3_preview_physics_retained_1.json`, `SCHEMA_B2.diff`, `retained_precision_mp_v2.schema.json` and `SCHEMA_J1.diff`;
- here: the three scripts, `b2c_statics.out.json` (S1), `b2c_checks.out.json` (S2) and `b2c_collisions.out.txt` (S3).

**Reproduce:** run S1 to S3 on a tree whose `P/fixtures`, `P/schemas` and `P/core/analysis_runs/retained_precision.py` equal NUM `cebff253d6`'s. For those paths that is also the T3 repository's `origin/main` at `54f1ba1f6d`, checked by `git diff --quiet`. The runs are deterministic. S1's `controls` re-derive DEF-O's pinned H, PTABLE's construction from preview-physics-1, and I96's patched SCHEMA `0d5bb812…`.

**A disclosed slip.** Before S2's first run, one text substitution in my scratch copy of `b2c_checks.py` (its argument list) was made with the host's `python3`, not VENV. No check, static or record output came from that call.

## Screen

Run on this folder before returning:
- the brief's strict path pattern (home shorthand, the user and private system folders, the worktree folder and the control-layer worktree name) over every file (no gzipped file here): no hit;
- `find <folder> -type l`: none;
- `git status --ignored` on the folder: untracked only, nothing ignored;
- RV117's host-name screen: this machine's host name, no hit; and the dot-local word pattern.

**The dot-local hits** are only SCHEMA's own `$id` line, in `statics/retained_precision_mp_v2.schema.json` and in the context lines of `statics/SCHEMA_B2.diff` and `statics/SCHEMA_J1.diff`. That `$id` is main's existing schema identifier, not a machine name. These three hits are also listed in the return to ROOT.
