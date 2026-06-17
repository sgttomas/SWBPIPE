# Dependency Semantic Refresh Worker Brief

Role: TASK + dependency-extract package shard worker.

Read these before acting:
- /Users/ryan/ai-env/projects/chirality/agents/AGENT_TASK.md
- /Users/ryan/ai-env/projects/chirality/skills/dependency-extract/SKILL.md
- /Users/ryan/ai-env/projects/chirality/skills/dependency-extract/BRIEF_SCHEMA.md
- /Users/ryan/ai-env/projects/chirality/skills/dependency-extract/QA_CHECKS.md
- /Users/ryan/ai-env/projects/chirality/skills/dependency-extract/TOOL_POLICY.md

Run controls:
- TaskSkill: dependency-extract
- MODE: UPDATE
- STRICTNESS: CONSERVATIVE
- CONSUMER_CONTEXT: RECONCILIATION
- ARCHITECTURE_BASIS_POLICY: PKG00_CONSISTENCY_TRACKERS
- ApplyEdits: true
- RUN_ROOT: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution
- WORKING_ROOT: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping
- DECOMPOSITION_PATH: /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/_Decomposition/SOFTWARE_DECOMP.md

Canonical write-form enums:
- DependencyType: PREREQUISITE, INTERFACE, HANDOVER, CONSTRAINT, ENABLES, OTHER
- TargetType: DELIVERABLE, PACKAGE, WBS_NODE, REQUIREMENT, DOCUMENT, EQUIPMENT, EXTERNAL, UNKNOWN
- Direction: UPSTREAM, DOWNSTREAM
- AnchorType: IMPLEMENTS_NODE, TRACES_TO_REQUIREMENT, NOT_APPLICABLE
- Explicitness: EXPLICIT, IMPLICIT
- SatisfactionStatus: TBD, PENDING, IN_PROGRESS, SATISFIED, WAIVED, NOT_APPLICABLE
- Confidence: HIGH, MEDIUM, LOW
- Origin: DECLARED, EXTRACTED
- Status: ACTIVE, RETIRED

Worker policy:
- You are not alone in the codebase. Do not revert or overwrite unrelated edits.
- Process only the deliverables listed below.
- For each deliverable, read local source docs, existing Dependencies.csv, _DEPENDENCIES.md, and cited upstream/downstream evidence needed to verify support.
- PKG-00 / DEL-00-* rows are valid architecture-consistency dependency trackers. Keep supported rows. Read relevant PKG-00 files when cited. Do not write inside PKG-00.
- Add source-supported missing rows only under CONSERVATIVE evidence.
- Retire unsupported extracted rows rather than deleting them.
- Preserve Origin=DECLARED unless directly contradicted; record contradictions in the run record.
- Never emit legacy core enum values. Preserve useful legacy labels in Notes as legacy_*.
- Never emit Status=CANDIDATE. Candidate/non-gating ideas go to _DEPENDENCIES.md handoff notes and run record.
- Do not edit lifecycle, review, MEMORY, source docs, _STATUS.md, DAG artifacts, coordination pointers, or git state.
- For each deliverable, write/update only Dependencies.csv, _DEPENDENCIES.md, and a new _run_records/TASK_RUN_2026-06-16_*dependency-semantic-refresh*.md.
- Run python3 tools/validation/validate_dependencies_schema.py on each edited Dependencies.csv from /Users/ryan/ai-env/projects/chirality/projects/chirality-piping.
- Final response: summarize per deliverable: rows added, rows retired, rows changed, PKG-00 rows reviewed/changed, warnings, validation result, files changed.

## Package shard
- Package: PKG-17_Export Format Interoperability
- Deliverable count: 9

## Allowed write targets
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-01_CAEPIPE and export-format source basis/Dependencies.csv
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-01_CAEPIPE and export-format source basis/_DEPENDENCIES.md
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-01_CAEPIPE and export-format source basis/_run_records/
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts/Dependencies.csv
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts/_DEPENDENCIES.md
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts/_run_records/
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-03_Native open JSON export package/Dependencies.csv
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-03_Native open JSON export package/_DEPENDENCIES.md
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-03_Native open JSON export package/_run_records/
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/Dependencies.csv
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/_DEPENDENCIES.md
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer/_run_records/
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser/Dependencies.csv
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser/_DEPENDENCIES.md
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser/_run_records/
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/Dependencies.csv
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/_DEPENDENCIES.md
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package/_run_records/
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/Dependencies.csv
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/_DEPENDENCIES.md
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter/_run_records/
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-08_GLB glTF review geometry export/Dependencies.csv
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-08_GLB glTF review geometry export/_DEPENDENCIES.md
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-08_GLB glTF review geometry export/_run_records/
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-09_Export adapter SDK and additional targets/Dependencies.csv
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-09_Export adapter SDK and additional targets/_DEPENDENCIES.md
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-09_Export adapter SDK and additional targets/_run_records/

## Deliverables in scope
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-01_CAEPIPE and export-format source basis
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-02_Export package, profile, and stable ID map contracts
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-03_Native open JSON export package
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-04_CAEPIPE MBF export profile and deterministic writer
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-05_CAEPIPE external run harness and CSV parser
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-06_Stress-neutral CSV JSON package
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-07_Conservative PCF subset exporter
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-08_GLB glTF review geometry export
- /Users/ryan/ai-env/projects/chirality/projects/chirality-piping/execution/PKG-17_Export Format Interoperability/1_Working/DEL-17-09_Export adapter SDK and additional targets
