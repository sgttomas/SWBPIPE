# TASK pressure return

RUN_STATUS: SUCCESS (bounded preparation; manager review pending)
ControlSurface: MERGED — parent dispatch plus sealed INIT-TASK.md V1
TaskProfile: NONE
TaskSkill: NONE
ScopePath: {WORKING_ROOT}/execution/_Evaluation/PHYSICS_UI_PREPARATION_20260907/E1/children/pressure
ToolsUsed: functions.exec / tools.exec_command (zsh; cat, rg, sed, git read-only, Python standard-library reads/arithmetic/scoped writes); functions.exec / tools.web__run (primary engineering search/read); collaboration.send_message to parent only. Exact interpreter/tool-path spec format is unavailable for connector tools; names are recorded faithfully.
ToolPolicyCompliance: PASS. No builds, source changes, staging, external messages, sibling messages or delegation.
WriteAuthorization: ALLOWED_WRITE_TARGETS — own subtree only.
Outputs: DECISION_BRIEF.md; SOURCES.json; this RETURN.md; _run_records/TASK_RUN_2026-09-07_0001.md; MANIFEST.json. Manifest hashes every output except itself and also seals unchanged INIT-TASK.md.

Findings: current straight correction computes effective S=Nwall−pAi, whereas curved pressure recovery yields wall force. This static inference is independently supported by official effective-force definitions. Historical free pressure witness gives zero axial stress at .052609 mm extension. Current mean-radius separate longitudinal formula is 5.75 MPa versus pAi/As=5.260869565 MPa, so simply unsuppressing it is insufficient. The brief specifies signs, support reaction meaning and free/fixed expectations.

NEEDS_HUMAN_RULING: select limited cap-thrust wall-force recovery A (nonbinding recommendation), effective-reference B, or full pressure constitutive investigation C; explicitly accept/decline omission of hoop Poisson coupling, public output reference/migration and future independently verified repair scope. Audit D03 is not project register D-03.
MISSING: new numerical replay (not authorized); owner-vetted full constitutive/boundary reference; full closure/EJ/Poisson coverage; public compatibility adoption. No new tolerance or physical acceptance supplied.
DEPENDENCY_NOTES: accepted source HEAD 35249accf139f52478d029458946e50ed25ee5dc; decomposition .12/SCA-009/DAG-010; prior Receipt 134 validated by root/CHANGE. Parent control freeze applies. Local context .7 history is not current authority. Package routing remains PKG05 plus PKG04 interface and independent PKG09 validation under parent control; no dependency satisfaction is inferred.
AppliedChanges: candidate evidence files in own subtree only.
Attribution: /root/physics_preparation/pressure; TASK Agent2; delegated-harness-native; instruction+config asserted role/nondelegation; exact model unexposed/unknown. No descendants.

Closure: bounded preparation complete, derivative CANDIDATE, no adoption. Accepted upstream S1/S2 evidence unchanged; no authoritative pointer update. Rerun source trace on changed basis and new independent tests only after Owner-selected repair; parent fan-in and fresh independent review still required. Next owner EVALUATION → HELP_HUMAN → Owner. Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
