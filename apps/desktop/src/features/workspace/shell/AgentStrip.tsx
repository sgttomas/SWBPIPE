import { Bot } from "lucide-react";
import { useSessionChrome } from "../WorkspaceSessionContext";

/**
 * The agent strip: 44 px on the right in every stage, and the one Agent control.
 * The embedded agent is Review changes' Agent tab; the strip's control opens it
 * and is pressed while that tab is shown.
 */
export function AgentStrip() {
  const { activeSection, operationTab, setActiveSection, setOperationTab } = useSessionChrome();
  const agentShown = activeSection === "operations" && operationTab === "agent";
  return (
    <aside className="shell-agent-strip" aria-label="Agent" data-testid="agent-strip">
      <span className="shell-reason-anchor">
        <button
          type="button"
          className="shell-rail-item"
          data-testid="agent-strip-open"
          aria-label="Agent"
          aria-pressed={agentShown}
          title="Agent"
          onClick={() => { setActiveSection("operations"); setOperationTab("agent"); }}
        ><Bot size={20} aria-hidden="true" /><span className="shell-rail-label">Agent</span></button>
      </span>
    </aside>
  );
}
