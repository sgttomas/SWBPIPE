# RP R-01 control-repair return

Status: **COMPLETE**. Writer state closes through `WRITER_FREEZE_R01.json`.

The active `LAUNCH_BRIEF.md` now uses only `{REPO_ROOT}`, `{WORKING_ROOT}`, and `{RUN_ROOT}` anchors. Its SHA-256 changed from `526aa77f53b4d949bc0a0b5f217dfe0e3c7c43eefbb27016d0d49d2cb346cb47` (3332 bytes) to `a9e274403872cc9ad61500956002d50e2007459c614678040d49429956098bc0` (3417 bytes).

The original exact bytes are preserved at `_run_records/R-01_PORTABILITY_REPAIR_20260908/ORIGINAL_LAUNCH_BRIEF.base64.json`, container SHA-256 `42102c7d78502552dc19acd0ae764114b68e7478fdfa1e822749dd0a2b9d2655`. Decoding its `content_base64` field yields exactly 3332 bytes with SHA-256 `526aa77f53b4d949bc0a0b5f217dfe0e3c7c43eefbb27016d0d49d2cb346cb47`; byte comparison to the pre-correction brief passed.

`CORRECTION_MAP_R01.json` SHA-256 is `40052c37d82c9633d096a31fa4390af8c87e5d5e82f0420dd634d230daf3bfc9` (2688 bytes). It directs consumers of preserved `MANIFEST.json` SHA-256 `89d474de34d00c82676675865b746945866f346efdda04592b056260892deb7c` to resolve its historical `LAUNCH_BRIEF.md` member to the decoded archive. `MANIFEST_V2.json` is the successor resolver for the active brief and repair artifacts.

The retrospective timing and wrong-lane-attempt disclosure remains byte-for-byte unchanged. The accepted physics `INVESTIGATION_REPORT.md` remains SHA-256 `746b0def10e7ac42dd6598cdeb9e39f188bb91e474f3e615450301bd0c58fa4b`; the accepted physics `RETURN.md` remains SHA-256 `939c28a8657cf3fa84a138be592366a6e6c617a2ca62f3a70644dc91b19f1d57`. No mathematics was rerun and no subject source, test, harness, Cargo, native-build, lifecycle, DAG, or Git action occurred.

R-01 is closed within RP's own control scope. This repair changes no physics verdict or adoption state. Next owner: `/root`.
