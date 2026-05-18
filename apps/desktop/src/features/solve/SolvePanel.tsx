import { Play } from "lucide-react";
import type { MechanicsResult, PreviewModel } from "../../types";

export function SolvePanel({
  model,
  result,
  running,
  onRun
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  running: boolean;
  onRun: () => void;
}) {
  return (
    <section className="panel solve-panel" aria-label="Solve execution" data-testid="solve-panel">
      <div className="panel-title">Execution</div>
      <div className="status-grid">
        <Status label="Mechanics" value={result?.status.mechanics ?? model.analysis_status.mechanics} />
        <Status label="Rule check" value={result?.status.rule_check ?? model.analysis_status.rule_check} />
        <Status label="Professional acceptance" value={result?.status.professional_acceptance ?? model.analysis_status.professional_acceptance} />
      </div>
      <button className="primary-action" data-testid="run-mechanics-preview" onClick={onRun} disabled={running} type="button">
        <Play size={16} />
        {running ? "Running preview" : "Run mechanics preview"}
      </button>
    </section>
  );
}

function Status({ label, value }: { label: string; value: string }) {
  return (
    <div className="status-pill">
      <span>{label}</span>
      <strong data-testid={`status-${label.toLowerCase().replaceAll(" ", "-")}`}>{value.replaceAll("_", " ")}</strong>
    </div>
  );
}
