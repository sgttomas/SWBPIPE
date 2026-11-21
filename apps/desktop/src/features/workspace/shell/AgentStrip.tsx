import { Bot } from "lucide-react";
import { AGENT_UNAVAILABLE_REASON } from "../shellLayout";
import { DisabledReason } from "./DisabledReason";

/**
 * The agent strip: 44 px on the right in every stage. The column is a later
 * slice, so the strip's one control is disabled with its reason. The working
 * embedded agent is Review changes' Agent tab, which the toolbar's Agent button opens.
 */
export function AgentStrip() {
  return (
    <aside className="shell-agent-strip" aria-label="Agent" data-testid="agent-strip">
      <span className="shell-reason-anchor">
        <button
          type="button"
          className="shell-rail-item"
          data-testid="agent-strip-open"
          aria-label="Agent"
          aria-disabled="true"
          aria-describedby="agent-strip-reason"
          title={AGENT_UNAVAILABLE_REASON}
        ><Bot size={20} aria-hidden="true" /><span className="shell-rail-label">Agent</span></button>
        <DisabledReason id="agent-strip-reason" text={AGENT_UNAVAILABLE_REASON} />
      </span>
    </aside>
  );
}
