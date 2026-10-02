# RV34 execution record

All Git reads used `GIT_OPTIONAL_LOCKS=0`. The working checkout and existing interpreter are recorded in CONTEXT.json. Shell was zsh. Python JSON parsing, hashing, AST extraction/comparison and bounded integer/float calculations were the only executed computation; no imported author calculator or full runner module was executed.

Initial read-only commands were `pwd`, `git show <revision>:<path>`, and `git ls-tree -r --name-only <revision> <bounded-path>`. Text inspection used `cat` and `rg -n` on the reviewer's frozen snapshots. `SNAPSHOT_READS.json` and `INPUT_READS.jsonl` retain exact origins, revisions, lengths and hashes. The first snapshot loop independently checked the I25 seal and every payload, returning SEAL_CHECK.json.

Canonical final bounded checks ran from this `_run_records` directory using the exact existing Python executable in CONTEXT.json:

```
<VENV> -B independent_audit.py > independent_audit.stdout.json 2> independent_audit.stderr.txt
<VENV> -B provenance_check.py > provenance_check.stdout.json 2> provenance_check.stderr.txt
<VENV> -B boundary_check.py > boundary_check.stdout.json 2> boundary_check.stderr.txt
<VENV> -B raw_summary_check.py > raw_summary_check.stdout.json 2> raw_summary_check.stderr.txt
```

All four commands returned exit 0. `review_io.py` pins the Git basis and supplies read/hash/cache operations only. It can rehydrate the immutable I25 input packet into the reviewer's owned scratch. Override `RV34_CACHE` for another owned scratch location when reproducing. The actual cache location and final relocation are in CACHE_DISPOSITION.json. I moved only my unsealed temporary copies out of the canonical review packet; no original/historical file was changed or deleted. The four checks were rerun successfully after that I/O-only packaging change, and their retained stdout/stderr are the final run.

Additional executed inline metadata checks, retained in SOURCE_WINDOW_AND_PRESERVATION.json, compared nine named historical-versus-frozen H AST function bodies, H raw/committed metadata/record equality, and all 102 preserved-result/changed-field reports. Static reads inspected the exact original parser, observer, summary/prefix edges, original W1-T4 override, VR global summary and finite VR launch gaps. These did not run source algorithms.

One exploratory container-shape print treated CLI24_EXACT_KERNEL.json as a dictionary although it is a list, producing a TypeError after an overlong/truncated print. It performed no numerical check or mutation. Inspection was narrowed to the actual list element keys before the independent checker was written. Some exploratory tool outputs were truncated; missing portions of the relevant methods/review argument were then read explicitly. No failed product check, candidate repair or waived criterion is hidden by the passing checks.

The first helper import created a Python bytecode cache within owned scratch; later commands used `-B`. No author verifier, solver, model/count/graph computation, external-model parser, Rust/Cargo/build/probe or host utility ran. Parent-provided absolute host paths and raw metadata stay in this `_run_records` subtree.
