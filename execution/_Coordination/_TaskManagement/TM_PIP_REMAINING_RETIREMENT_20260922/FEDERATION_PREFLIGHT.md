# Invocation-local D-GOV-33 federation preflight

Scope: Piping legacy-source retirement census only. Basis: `b3e2ce4ec74e01d6f393fc0bc069699bb079df91`.

The deterministic federation helper completed with **COMPLETE** coverage across all 4 canonical tracked registers. It inspected 51 findings and presented 45; register writes: 0. Register positions: ROOT OPEN 10 / DEFERRED 8 / archived 109; APP 8 / 3 / 33; PIP 11 / 23 / 8; PEC 16 / 1 / 7 (plus PEC CLOSED 1).

Relationship findings: foreign-to-local 1; local-to-foreign 23; remote-closed/local-open 22; local-closed/remote-open 1; missing notice 4. These are reported as findings, not resolved errors or retirement dispositions. Federation survey coverage is complete for helper-supported canonical register/archive shapes; it does not assess the semantic grounds of findings.

The invocation-local derived projection is in ignored `.candidates/TM_PIP_REMAINING_RETIREMENT_20260922/federation.json`. `.gitignore:95` was verified with `git check-ignore -v` before generation. The rebuildable census comparison is beside it as `census_comparison.json`; neither JSON is authority. Human-readable source comparison is in `CENSUS_COMPARISON.md`.

Inputs: pinned R5 census SHA-256 `646470f87ea916b939bdbf1d66877ae8181fcd61bd1922b27f63900b30e70551`; D-GOV-33 SHA-256 `813333b2255097d860311279148103526b585cf615f6613a751789e5efcb68b2`. Root `AGENTS.md` SHA-256 `1bb670ca339a990b153cf033dac2d8e29ca71bdea0accee4200e6dd1feed3d57`; Piping `AGENTS.md` SHA-256 `60d9af6492e415f68ba91176530316545ddc25e4c0fddc6fb3192beb10d10100`; Piping `LOOP_INIT.md` SHA-256 `ccdf2daf326a169f4296511b31be4daa5e366d54aca9a5e725b79d9f72794492`; task-management workflow entry SHA-256 `aefe4190ffd1b18cbebd9d7d9c3cfd76b0494bfbb3a89cb1e5b07bf76b6e932f`, contract SHA-256 `775148cc1ae65944e7a26b60b04bdeeb0266444c798b04ab06bed7e154e08290`, and method SHA-256 `7f9e0d5ea7c2b74e7148faf3d2a35efc6ddd0552efd4b577a1c56895f75fe2f8`.
