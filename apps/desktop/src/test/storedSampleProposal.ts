// A review proposal as an earlier version of the desktop made and saved it with a project: the bundled
// invented proposal, retargeted at a row of the given result. The desktop no longer makes one (its two
// sample buttons were removed); it still opens a project that carries one and shows it. Tests of that
// use this, so the stored shape stays the one those projects hold.
import type { AgentProposal, MechanicsResult, SelectedReviewTarget } from "../types";

export async function storedSampleProposal(result: MechanicsResult, selectedTarget?: SelectedReviewTarget | null): Promise<AgentProposal> {
  const fixture = (await import("../../../../fixtures/product_preview/invented_agent_proposal.json")).default as AgentProposal;
  return build(result, fixture, selectedTarget);
}

function build(
  result: MechanicsResult,
  agentProposalFixture: AgentProposal,
  selectedTarget?: SelectedReviewTarget | null,
): AgentProposal {
  const forceResult =
    result.results.find(
      (item) => item.id === "result:force:pipe-P-120:axial",
    ) ??
    result.results.find((item) => item.kind === "element_local_axial_force");
  const primaryDiagnostic =
    result.diagnostics.find(
      (item) => item.severity === "warning" || item.severity === "blocking",
    ) ?? result.diagnostics[0];
  const targetRef =
    selectedTarget?.id ??
    forceResult?.id ??
    primaryDiagnostic?.id ??
    primaryDiagnostic?.affected_refs?.[0] ??
    result.summary.max_displacement?.result_ref ??
    result.results[0]?.id ??
    "diagnostic:physics:context-unavailable";
  const targetKind =
    selectedTarget?.target_type.replaceAll("_", " ") ?? "computed mechanics";

  return {
    ...agentProposalFixture,
    proposal_id: "proposal:physics-diagnostic-review",
    prompt:
      "Review current computed mechanics diagnostics and suggest a non-mutating follow-up.",
    operation: {
      ...agentProposalFixture.operation,
      operation_id: "op:review-computed-diagnostic",
      operation_kind: "attach_design_knowledge",
      affected_entity_ids: [targetRef],
      changes: [
        {
          change_id: "change:add-review-note",
          change_kind: "attach_design_knowledge",
          target_ref: targetRef,
          before: `No review note attached for the selected ${targetKind} context.`,
          after: `Attach review note referencing the current computed preview ${targetKind} context.`,
        },
      ],
    },
    rationale: `Generated from current preview mechanics context; selected review reference is ${targetRef}. This narrative is review-only and does not mutate accepted model state.`,
    validation: {
      ...agentProposalFixture.validation,
      unit_validation: "not_required_metadata_review_only",
      constraint_validation: "warning_computed_context_requires_human_review",
      diff_preview_status: "generated_from_computed_context",
    },
  };
}
