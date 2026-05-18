import { CheckSquare, Wand2 } from "lucide-react";
import type { AgentProposal, SelectedReviewTarget } from "../../types";

export function AgentProposalPanel({
  proposal,
  mechanicsReady,
  selectedReviewTarget,
  onLoad
}: {
  proposal: AgentProposal | null;
  mechanicsReady: boolean;
  selectedReviewTarget: SelectedReviewTarget | null;
  onLoad: () => void;
}) {
  return (
    <section className="panel agent-panel" aria-label="Agentic proposal" data-testid="agent-proposal-panel">
      <div className="panel-title">Proposal</div>
      <button className="secondary-action" data-testid="generate-review-proposal" onClick={onLoad} disabled={!mechanicsReady} type="button">
        <Wand2 size={16} />
        Generate review proposal
      </button>
      {selectedReviewTarget ? (
        <div className="status-pill" data-testid="selected-review-target">
          <span>Review target</span>
          <strong>{`${selectedReviewTarget.target_type}: ${selectedReviewTarget.id}`}</strong>
        </div>
      ) : null}
      {proposal ? (
        <div className="proposal-body" data-testid="proposal-body">
          <h2>{proposal.proposal_id}</h2>
          <p>{proposal.rationale}</p>
          <div className="status-pill">
            <span>Validation</span>
            <strong>{proposal.validation.diff_preview_status}</strong>
          </div>
          <div className="diff-list">
            {proposal.operation.changes.map((change) => (
              <article key={change.change_id}>
                <strong>{change.target_ref}</strong>
                <small>{change.change_kind}</small>
                <p>{change.before}</p>
                <p>{change.after}</p>
              </article>
            ))}
          </div>
          <button className="review-action" data-testid="accept-proposal-disabled" disabled type="button" title="Review-only until accepted mutation is implemented">
            <CheckSquare size={16} />
            Accept disabled
          </button>
        </div>
      ) : (
        <p className="muted">Local proposal examples are deterministic and review-only.</p>
      )}
    </section>
  );
}
