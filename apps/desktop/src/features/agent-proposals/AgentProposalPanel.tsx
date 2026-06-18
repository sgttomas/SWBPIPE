import { CheckSquare, ClipboardList, ShieldCheck, Wand2 } from "lucide-react";
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
  const unitPolicy = proposal ? proposalUnitPolicy(proposal) : null;

  return (
    <section className="panel agent-panel" aria-label="Agentic proposal" data-testid="agent-proposal-panel">
      <div className="panel-title">Proposal</div>
      <button
        className="secondary-action"
        data-testid="generate-review-proposal"
        onClick={onLoad}
        disabled={!mechanicsReady}
        title={
          mechanicsReady
            ? undefined
            : "Disabled: review proposals are generated against solve results. Open the Solve section and run the mechanics preview first."
        }
        type="button"
      >
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
          <section className="proposal-section" aria-label="Operation summary" data-testid="proposal-operation-summary">
            <h3>
              <ClipboardList size={14} aria-hidden="true" />
              Operation
            </h3>
            <div className="proposal-meta-grid">
              <ProposalFact label="Operation" value={proposal.operation.operation_id} />
              <ProposalFact label="Kind" value={proposal.operation.operation_kind} />
              <ProposalFact label="Status" value={proposal.operation.operation_status} />
              <ProposalFact
                label="Affected entities"
                value={proposal.operation.affected_entity_ids.join(", ")}
                testId="proposal-affected-entities"
              />
            </div>
          </section>
          <div className="status-pill">
            <span>Validation</span>
            <strong>{proposal.validation.diff_preview_status}</strong>
          </div>
          <section className="proposal-section" aria-label="Validation status" data-testid="proposal-validation-status">
            <h3>Validation Status</h3>
            <div className="proposal-meta-grid">
              {Object.entries(proposal.validation).map(([key, value]) => (
                <ProposalFact key={key} label={formatKey(key)} value={value} />
              ))}
            </div>
          </section>
          {unitPolicy ? (
            <section className="proposal-section" aria-label="Proposal unit policy" data-testid="proposal-unit-policy">
              <h3>Unit Policy</h3>
              <div className="proposal-meta-grid">
                <ProposalFact label="Status" value={unitPolicy.status} />
                <ProposalFact label="Source" value={unitPolicy.source} />
                <ProposalFact label="Conversion" value={unitPolicy.conversion} />
              </div>
            </section>
          ) : null}
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
          <section className="proposal-section" aria-label="Proposal audit boundary" data-testid="proposal-audit-boundary">
            <h3>
              <ShieldCheck size={14} aria-hidden="true" />
              Audit Boundary
            </h3>
            <div className="proposal-flag-grid">
              {Object.entries(proposal.audit_boundary).map(([key, value]) => (
                <ProposalFlag key={key} label={formatKey(key)} value={value} />
              ))}
            </div>
          </section>
          <section
            className="proposal-section"
            aria-label="Proposal professional boundary"
            data-testid="proposal-professional-boundary"
          >
            <h3>Professional Boundary</h3>
            <div className="proposal-flag-grid">
              {Object.entries(proposal.professional_boundary).map(([key, value]) => (
                <ProposalFlag key={key} label={formatKey(key)} value={value} />
              ))}
            </div>
          </section>
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

function ProposalFact({ label, value, testId }: { label: string; value: string; testId?: string }) {
  return (
    <div className="proposal-fact" data-testid={testId}>
      <span>{label}</span>
      <strong>{value}</strong>
    </div>
  );
}

function ProposalFlag({ label, value }: { label: string; value: boolean }) {
  return (
    <div className={`proposal-flag ${value ? "true" : "false"}`}>
      <span>{label}</span>
      <strong>{String(value)}</strong>
    </div>
  );
}

function formatKey(key: string): string {
  return key.replaceAll("_", " ");
}

function proposalUnitPolicy(proposal: AgentProposal) {
  return {
    status: proposal.validation.unit_validation ?? "not_declared",
    source: "proposal.validation.unit_validation",
    conversion: "false"
  };
}
