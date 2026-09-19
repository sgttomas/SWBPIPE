// TP-APP-R2-UXSHELL-001 workspace information architecture.
//
// PRD section 14.1 names the workspace surfaces; the A12 journey (SMOKE.md
// TP-MAC-141) orders them: model entities -> loads -> solve -> results ->
// report. The shell therefore keeps a persistent spatial core (model tree +
// 3D centerline viewport + property inspector, per PRD 14.1/14.3 and
// DEL-07-02) always on screen, and organizes every other panel behind this
// always-visible section navigation, listed in journey order. The rule-pack
// manager landed as Phase C2 slice 1 (TP-C2-EDITOR-001), placed between
// loads and solve because user rule checks consume authored loads and feed
// the solve/check journey (PRD §22.4). The private library manager landed as
// Phase C3 (TP-C3-LIBGUI-001, PRD §13/§14.6), placed immediately before the
// rule-pack manager because both are private local-only asset managers and
// rule packs reference imported library allowables.
export type WorkspaceSectionId =
  | "operations"
  | "loads"
  | "libraries"
  | "rule-packs"
  | "solve"
  | "results"
  | "report"
  | "project"
  | "exports"
  | "evidence";

export const WORKSPACE_SECTIONS: ReadonlyArray<{ id: WorkspaceSectionId; label: string; description: string }> = [
  {
    id: "operations",
    label: "Review changes",
    description: "Queued structured operations, apply/undo/redo, diffs, and the operation review ledger"
  },
  {
    id: "loads",
    label: "Load Cases",
    description: "Load-case manager: create load cases, primitive loads, and combinations"
  },
  {
    id: "libraries",
    label: "Libraries",
    description:
      "Private, local-only library manager: import material/section/component libraries with provenance, validation findings, and the local store"
  },
  {
    id: "rule-packs",
    label: "Rules",
    description:
      "Private, local-only rule-pack manager: drafts, validation findings, checksum generation, and the local store"
  },
  {
    id: "solve",
    label: "Analyze",
    description: "Run the mechanics preview, solve job audit, diagnostics, and missing-data review"
  },
  {
    id: "results",
    label: "Results",
    description: "Results browser, comparison workspace, and design-authoring state"
  },
  {
    id: "report",
    label: "Report",
    description: "Rendered calculation report, report packet, and report content lint"
  },
  {
    id: "project",
    label: "Project",
    description: "Local project storage audit and validation preflight"
  },
  {
    id: "exports",
    label: "Exports",
    description: "Result/geometry exports, exchange adapters, handoff packages, and export review"
  },
  {
    id: "evidence",
    label: "Audit & Boundaries",
    description: "Run audit, validation evidence, telemetry/privacy/security boundary reviews"
  }
];

export const EXPENSIVE_LIFECYCLE_SECTIONS: ReadonlySet<WorkspaceSectionId> = new Set(
  WORKSPACE_SECTIONS.map((section) => section.id)
);
