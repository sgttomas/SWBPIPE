import { AlertTriangle, Play, ShieldCheck } from "lucide-react";
import type { Diagnostic, MechanicsResult, PreviewModel } from "../../types";

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
  const diagnostics = [...model.diagnostics, ...(result?.diagnostics ?? [])];
  const readinessItems = readinessSummary({ model, result, diagnostics });
  return (
    <section className="panel solve-panel" aria-label="Solve execution" data-testid="solve-panel">
      <div className="panel-title">Execution</div>
      <div className="status-grid">
        <Status label="Mechanics" value={result?.status.mechanics ?? model.analysis_status.mechanics} />
        <Status label="Rule check" value={result?.status.rule_check ?? model.analysis_status.rule_check} />
        <Status label="Professional acceptance" value={result?.status.professional_acceptance ?? model.analysis_status.professional_acceptance} />
      </div>
      <section className="readiness-summary" aria-label="Solve readiness summary" data-testid="solve-readiness-summary">
        {readinessItems.map((item) => (
          <ReadinessRow key={item.id} item={item} />
        ))}
      </section>
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

type ReadinessItem = {
  id: "mechanics" | "rule" | "diagnostics" | "professional";
  label: string;
  value: string;
  tone: "ok" | "info" | "warning" | "blocking";
};

function readinessSummary({
  model,
  result,
  diagnostics
}: {
  model: PreviewModel;
  result: MechanicsResult | null;
  diagnostics: Diagnostic[];
}): ReadinessItem[] {
  const mechanicsStatus = result?.status.mechanics ?? model.analysis_status.mechanics;
  const ruleStatus = result?.status.rule_check ?? model.analysis_status.rule_check;
  const professionalStatus = result?.status.professional_acceptance ?? model.analysis_status.professional_acceptance;
  const warningCount = diagnostics.filter((item) => item.severity === "warning").length;
  const blockingCount = diagnostics.filter((item) => item.severity === "blocking" || item.severity === "error").length;

  return [
    {
      id: "mechanics",
      label: "Mechanics readiness",
      value: result
        ? `${result.results.length} computed result rows; ${formatStatus(mechanicsStatus)}`
        : `preview run not started; ${formatStatus(mechanicsStatus)}`,
      tone: result ? "ok" : "info"
    },
    {
      id: "rule",
      label: "Rule-check readiness",
      value: ruleReadiness(ruleStatus),
      tone: ruleStatus.toLowerCase().includes("incomplete") || ruleStatus.toLowerCase().includes("missing")
        ? "warning"
        : "info"
    },
    {
      id: "diagnostics",
      label: "Diagnostics",
      value: `${diagnostics.length} ${plural("diagnostic", diagnostics.length)}; ${warningCount} ${plural(
        "warning",
        warningCount
      )}; ${blockingCount} blocking/error`,
      tone: blockingCount > 0 ? "blocking" : warningCount > 0 ? "warning" : "ok"
    },
    {
      id: "professional",
      label: "Professional boundary",
      value: professionalReadiness(professionalStatus),
      tone: "warning"
    }
  ];
}

function ReadinessRow({ item }: { item: ReadinessItem }) {
  const Icon = item.tone === "ok" ? ShieldCheck : AlertTriangle;
  return (
    <div className={`readiness-row ${item.tone}`} data-testid={`readiness-${item.id}`}>
      <Icon size={14} aria-hidden="true" />
      <span>{item.label}</span>
      <strong>{item.value}</strong>
    </div>
  );
}

function ruleReadiness(status: string): string {
  const normalized = status.toLowerCase();
  if (normalized.includes("incomplete") || normalized.includes("missing") || normalized.includes("not_performed")) {
    return `rule inputs incomplete; mechanics results remain reviewable only; ${formatStatus(status)}`;
  }
  return formatStatus(status);
}

function professionalReadiness(status: string): string {
  const normalized = status.toLowerCase();
  if (normalized.includes("not_provided") || normalized.includes("not provided")) {
    return "human review required; no professional acceptance record";
  }
  return formatStatus(status);
}

function formatStatus(value: string): string {
  return value.replaceAll("_", " ").toLowerCase();
}

function plural(label: string, count: number): string {
  return count === 1 ? label : `${label}s`;
}
