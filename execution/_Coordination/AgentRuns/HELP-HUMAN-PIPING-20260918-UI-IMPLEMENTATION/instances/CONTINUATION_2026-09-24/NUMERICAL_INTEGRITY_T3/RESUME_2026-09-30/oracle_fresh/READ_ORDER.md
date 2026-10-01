# Actual read and execution ordering

Agent `/root/a1_oracle_fresh` is a fresh Codex native TASK descendant of `/root`.
The parent supplied the role and assignment in the launch message. No child was
created. Host access is unrestricted; the write/read fences are prompt-enforced.
No model-version or model-diversity claim is made.

The actual tool-result chunk identifiers below locate the transcript operations.
Paths use repository-relative names or the parent's declared COORD/WT aliases.
`GIT_OPTIONAL_LOCKS=0` was added to subsequent Git reads when ROOT directed it.
Earlier reads were `git show`, `git ls-tree`, `git rev-parse` and `git grep` only;
no Git/index mutation command was executed. Tools used were exec_command and
apply_patch through functions.exec; communication used collaboration.send_message.
Python was the host's Python 3.9.6 standard library, with no package installation.
The COMMON `<VENV>` alias was not supplied, and no environment was changed.

| Order | Chunk | Actual operation / model-visible material |
|---|---|---|
| 1 | 5a36eb | `pwd`, `git rev-parse --show-toplevel`, `cat AGENTS.md`, `cat agents/AGENT_TASK.md` in the initial Codex worktree. Both instruction files were then read again at the required coordination revision. |
| 2 | a17852 | `git ls-tree -r --name-only 2eb85f3 --` exact instruction paths and R; filenames only. |
| 3 | 461512 | Python subprocess `git show 2eb85f3:PATH` in sequence: Root AGENTS, TASK, Piping AGENTS, A1_ORACLE_FRESH, A1_ORACLE, COMMON. Full text and SHA256. |
| 4 | 1348d9 | `git ls-tree` at response revision over T3 and response run, filtered to handoff/audit Markdown and src/cases.rs, src/main.rs paths; filenames only. |
| 5 | 3e2f1a | `git show 520d7df:T3/AUDIT/AUDIT_RESPONSE_SCOPE_HANDOFF_2026-09-30.md`, full text/hash before response source. |
| 6 | ee8129 | `git show 520d7df:` audit A1_A2_REVIEW.md, then response src/cases.rs and src/main.rs, full text/hashes. Audit exact examples are permitted mathematical claims, not solver outputs. |
| 7 | b71efc | `git ls-tree` at d01ad98 over Piping, filenames filtered to adaptive/source/recover/assembly and named D1/D2/rulings files. |
| 8 | 772f04 | Metadata for DESIGN_STANDING/DESIGN_NUMERICS Markdown; then ROOT_RULINGS_V1 and filtered R7 text. Output was truncated due to the large rulings file; this did not provide a complete model-visible read of its contents. |
| 9 | 3907c6 | R7 lines 285–370; D2 DESIGN lines matching bound/relative/O9 terms and surrounding context; ROOT_RULINGS matching those terms, including A1/O9 adoption. |
| 10 | 1bf9d6 | D1 DESIGN filtered claim/denominator/row text (output partially truncated); adaptive 271–430; recover 1–200; source DOF, component, member, spring, constraint and load definitions. |
| 11 | 9e26ed | D1 DESIGN 315–399; adaptive 1881–1935 and 2571–2630; assemble selected constitutive/axial/torsion lines. |
| 12 | 8f304d | D1 DESIGN 399–487, full selected interval including recovery, guarantee and published-scale definitions. |
| 13 | 6ca611 | `GIT_OPTIONAL_LOCKS=0 git grep -n -m 3 'fn compare_honest' d01ad98 -- P/core P/tools P/validation`; one function location. |
| 14 | 2ddf90 | models.rs lines 489–609 only: compare_honest and beginning of G5a. This is source contract corroboration; no fixture definitions, expected values, or run output were presented. Full blob hash recorded. |
| 15 | 12cec8 | Copy the already-read pinned cases.rs/main.rs to owned inputs; hash and Python-version output. |
| 16 | ecb466 | Read ROOT-released I22/ORACLE_INPUTS.json, verify its advertised hash, display first 5,000 characters. It contains only primitive inputs and metadata; the prohibited source matrix was not opened. |
| 17 | 2c65b5 | Create independent exact_oracle.py and run `python3 exact_oracle.py freeze <oracle_fresh>`: 24 cases/880 rows, TRUTH hash recorded. No external comparison inputs used. |
| 18 | 25bef8 | Standard-library inline Python: bind all I22 primitive inputs independently to cases/main; rederive audit arithmetic; check all exact row values against separate CPython Fraction-to-float rounding. Write INPUT_BINDING, AUDIT_ARITHMETIC, DERIVATION_CHECKS. |
| 19 | 9abbe9 | Import owned oracle with bytecode writing disabled; run self checks, regenerate truth in WT/scratch/a1-oracle-fresh/regenerate, compare byte-for-byte, synthesize TSV from own direct-rounded truth, check comparator on all 24 synthetic cases. No solver outputs read. |
| 20 | d4c65d | Hash-only reread of the consulted blobs and working input handoff; write SOURCE_READS and RAW_CHECK_OUTPUTS. |
| 21 | checkpoint seal | Write READ_ORDER, CHECKPOINT_0 and packet inventory. No preserved solver output read before this seal. |

Numeric rerun: copy `inputs/cases.rs` into an empty owned destination's `inputs/`,
then run `python3 exact_oracle.py freeze <destination>` and compare TRUTH.json
bytes. Freeze refuses to overwrite existing TRUTH.json. Comparison uses
`python3 exact_oracle.py compare <oracle_fresh> <released.tsv> <owned-report.json>`.
The independent exact free-equilibrium and binary64 boundary checks execute in
that script. DERIVATION_CHECKS and RAW_CHECK_OUTPUTS retain the completed checks.

No response matrix.json, response oracle.py, historical COMPARISON/expected-output
record, B01/B02 numerical stdout, or prior oracle-worker output has been opened.
The response matrix filename/hash appearing in the permitted input handoff is
provenance only. No Rust build, solver execution, tool installation, host-guard
change, maintained-source edit, Git/index write, or delegation was performed.
