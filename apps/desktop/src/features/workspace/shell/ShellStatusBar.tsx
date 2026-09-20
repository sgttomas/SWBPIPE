import { useId, useState } from "react";
import { Flag, Info } from "lucide-react";
import { useDisplayUnits } from "../../display-units";
import { shellLocation, statusChipInputsFromCells, statusChipText, statusChips } from "../shellLayout";
import type { StatusChip } from "../shellLayout";
import type { StatusAuthorityDomain } from "../statusLabels";
import {
  useSessionChrome,
  useSessionModel,
  useSessionResults,
  useSessionSelection
} from "../WorkspaceSessionContext";

// The chip of each authority domain keeps the test id its pill had.
const CHIP_TEST_IDS: Record<StatusAuthorityDomain, string> = {
  Solver: "status-pill-mechanics",
  "Rule pack": "status-pill-rule-check",
  Human: "status-pill-professional",
  Evidence: "status-pill-evidence"
};

const DISPLAY_UNIT_FACES = { entered: "Entered", SI: "SI", US: "US" } as const;

/**
 * The status bar (specification §5.4, §5.5): the chips of the policy, the
 * issues count, the selection, the display units and the About control.
 * Nothing else lives here. A chip is read, never clicked to change anything:
 * the click opens its popover with the raw token and the domain.
 */
export function ShellStatusBar({ issueCount }: { issueCount: number }) {
  const { model } = useSessionModel();
  const { result, historicalRun, solveJob, ruleCheckAggregate } = useSessionResults();
  const { selection, orderedSelection } = useSessionSelection();
  const { activeSection, stageSurface, auditDrawerOpen, setAuditDrawerOpen, issuesDrawerOpen, setIssuesDrawerOpen } = useSessionChrome();
  const { preference } = useDisplayUnits();
  const [openChip, setOpenChip] = useState<string | null>(null);
  const location = shellLocation(activeSection, stageSurface);
  const chips = statusChips(
    statusChipInputsFromCells({ model, result, historicalRun, solveJob, ruleCheckAggregate }),
    location.stage,
    location.page !== null
  );
  const count = orderedSelection.orderedKeys.length;
  return (
    <footer className="status-bar shell-status-bar" aria-label="Workspace status" data-testid="workspace-status-bar">
      <div className="status-pill-group" aria-label="Analysis statuses" data-testid="status-chips">
        {chips.map((chip) => (
          <StatusChipButton
            key={chip.token}
            chip={chip}
            open={openChip === chip.token}
            onToggle={() => setOpenChip((current) => (current === chip.token ? null : chip.token))}
            onClose={() => setOpenChip(null)}
          />
        ))}
      </div>
      <button
        type="button"
        className="status-drawer-button issue-button"
        data-testid="issues-drawer-toggle"
        aria-expanded={issuesDrawerOpen}
        onClick={() => setIssuesDrawerOpen((open) => !open)}
      >
        <Flag size={12} aria-hidden="true" /> {issueCount} Issues
      </button>
      <span className="shell-status-selection" data-testid="status-selection" title="Selection">
        {selection ? `${selection.type}: ${selection.id}` : ""}{count > 1 ? ` · ${count} selected` : ""}
      </span>
      <span className="shell-status-units" data-testid="status-display-units" title="Display units">{DISPLAY_UNIT_FACES[preference]}</span>
      <button
        type="button"
        className="status-drawer-button shell-about"
        data-testid="audit-drawer-toggle"
        aria-expanded={auditDrawerOpen}
        aria-label="About SWBPIPE…"
        title="About SWBPIPE…"
        onClick={() => setAuditDrawerOpen((open) => !open)}
      >
        <Info size={14} aria-hidden="true" />
      </button>
    </footer>
  );
}

function StatusChipButton({ chip, open, onToggle, onClose }: { chip: StatusChip; open: boolean; onToggle: () => void; onClose: () => void }) {
  const popoverId = useId();
  return (
    <span className="shell-chip-anchor">
      <button
        type="button"
        className="status-pill shell-chip"
        data-testid={CHIP_TEST_IDS[chip.domain]}
        data-status-token={chip.token}
        aria-expanded={open}
        aria-controls={open ? popoverId : undefined}
        title={chip.source ? `${chip.source}: ${chip.token}` : chip.token}
        onClick={onToggle}
        onKeyDown={(event) => { if (event.key === "Escape" && open) { event.stopPropagation(); onClose(); } }}
      >
        {statusChipText(chip)}
      </button>
      {open ? (
        <>
          <span className="shell-chip-backdrop" aria-hidden="true" onClick={onClose} />
          <span className="shell-chip-popover" id={popoverId} role="dialog" aria-label={`${statusChipText(chip)}: ${chip.source ?? "recorded token"}`} data-testid={`${CHIP_TEST_IDS[chip.domain]}-popover`}>
            {chip.source ? <span>{chip.source}</span> : null}
            <code>{chip.token}</code>
            <span>{chip.domain}</span>
          </span>
        </>
      ) : null}
    </span>
  );
}
