RequestedBy: W7 Agent1 WORKING_ITEMS under HELP_HUMAN owner implementation direction
RunID: HELP-HUMAN-PIPING-20260905-DESIGN-IMPLEMENTATION
ChildInstanceID: R1
Role: Agent2 TASK, no delegation, fresh context
TaskSkill: software-code-review
ApplyEdits: false
INSTRUCTION_ROOT: {REPO_ROOT}
ScopePath: {WORKING_ROOT}/execution/_Coordination/AgentRuns/HELP-HUMAN-PIPING-20260905-DESIGN-IMPLEMENTATION/instances/W7/children/R1
AllowedWriteTargets: NONE (return structured text to manager; manager persists record)
AllowedTools: read/search, shell read-only, no Git mutations, no model/app operations
DeclaredReads: root/project AGENTS, AGENT_TASK, skill, software-workflow profile, report/map relevant design, W7 PLAN/brief amendments/sourcefreeze and verification, apps/desktop relevant dependencies.
Objective: Review100% frozen UI/config/test diff in W7/snapshots/SOURCE_V1/DIFF.patch with MANIFEST.json. All changed files and tests, not just production. Validate hashes vs current. Trace command routing/disclosures/focus/keyboard, selection and draft retention, exact queue/batch/apply/undo invariants, diagnostics and no silentdefaults, typed field presentation, canvas host ResizeObserver projection/cleanup, boundedrail/drawer CSS, native minWidth1024/minHeight768 only config.
Acceptance criteria: existing07-01 REQ01/03/04/05/07/09 AC001;07-02 RQ001–008 AC001;07-04 AC001;07-05 REQ002–007;07-06 AC001 no independent acceptance;07-08 REQ002–004/008. Full table/list widgets and inline Add/Apply are explicit residual; do not demand unselected implementation.
Evidence: W7/after-result.json + after-geometry.json/screenshots; _run_records/layout-e2e-v2.log2/2PASS after window-only resize reproduction; U2RETURN_V3 focused16PASS; U1RETURN when available focusedroute/copytests; _run_records/integrated-checks-v1.json full desktop tests/build running at dispatch, manager will send outcome. Browser fixture/mechanics and browsermemory are not nativebackend evidence.
Exclusions: no core/Rust/schema review (root owns separately), source repairs, scope/lifecycle acceptance, protected/private model data.
Return: PASS with no actionable findings or FAIL actionable file/line/impact/remedy; list every reviewed file, verification limits, hash drift if any, actualmodelunknown if notexposed. No writes; parent persists return.
