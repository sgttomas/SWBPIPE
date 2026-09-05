import { QuantityReadout } from "../display-units";
import { BookOpen } from "lucide-react";
import type { DesignKnowledge, KnowledgeRecord, MechanicsResult } from "../../types";

const RESULT_UNIT_DISPLAY_ORDER = ["MPa", "N", "N*m", "mm", "rad"];

export function KnowledgePanel({ knowledge, result }: { knowledge: DesignKnowledge | null; result: MechanicsResult | null }) {
  const computedRecords = physicsRecords(result);
  const records = [...(knowledge?.records ?? []), ...computedRecords];
  const unitContext = computedKnowledgeUnitContext(result);
  return (
    <section className="panel knowledge-panel" aria-label="Design knowledge" data-testid="knowledge-panel">
      <div className="panel-title">
        <BookOpen size={16} />
        Design Knowledge
      </div>
      {unitContext ? (
        <div className="report-line" data-testid="knowledge-unit-context">
          <span>Source computed unit context</span>
          <strong>{unitContext}</strong>
        </div>
      ) : null}
      {records.length > 0 ? (
        <div className="record-list">
          {records.map((record) => (
            <article key={record.id} className="record-row" data-testid={`knowledge-record-${record.id}`}>
              <div>
                <strong>{record.title}</strong>
                <small>{record.kind} · {record.status}</small>
              </div>
              <p><KnowledgeSummary record={record} result={result} computed={computedRecords.includes(record)} /></p>
              <span>{record.provenance}</span>
            </article>
          ))}
        </div>
      ) : (
        <p className="muted">Loading local invented knowledge records.</p>
      )}
    </section>
  );
}

export function physicsRecords(result: MechanicsResult | null): KnowledgeRecord[] {
  if (!result) return [];

  const records: KnowledgeRecord[] = result.diagnostics
    .filter((diagnostic) => diagnostic.severity === "warning" || diagnostic.severity === "blocking")
    .map((diagnostic) => ({
      id: `knowledge:${diagnostic.id ?? diagnostic.code}`,
      kind: "computed_mechanics_context",
      title: diagnostic.code.replaceAll("_", " "),
      summary: diagnostic.message,
      affected_refs: diagnostic.affected_refs ?? [],
      provenance: "computed_preview_result",
      status: diagnostic.severity
    }));

  if (result.summary.max_displacement) {
    records.push({
      id: "knowledge:computed-max-displacement",
      kind: "computed_mechanics_context",
      title: "Computed displacement review",
      summary: `${result.summary.max_displacement.result_ref} is ${result.summary.max_displacement.value} ${result.summary.max_displacement.unit} at ${result.summary.max_displacement.location_ref}.`,
      affected_refs: [result.summary.max_displacement.result_ref, result.summary.max_displacement.location_ref],
      provenance: "computed_preview_result",
      status: "human_review_required"
    });
  }

  const axialForce =
    result.results.find((item) => item.id === "result:force:pipe-P-120:axial") ??
    result.results.find((item) => item.kind === "element_local_axial_force");
  if (axialForce) {
    records.push({
      id: "knowledge:computed-axial-force",
      kind: "computed_mechanics_context",
      title: "Computed axial force review",
      summary: `${axialForce.id} is ${axialForce.value} ${axialForce.unit} for ${axialForce.entity_ref}.`,
      affected_refs: [axialForce.id, axialForce.entity_ref],
      provenance: "computed_preview_result",
      status: "human_review_required"
    });
  }

  return records;
}

function computedKnowledgeUnitContext(result: MechanicsResult | null): string | null {
  if (!result) return null;

  const quantities = [
    result.summary.max_displacement
      ? {
          ref: result.summary.max_displacement.result_ref,
          unit: result.summary.max_displacement.unit
        }
      : null,
    computedAxialForce(result)
  ].filter((item): item is { ref: string; unit: string } => Boolean(item));

  if (quantities.length === 0) return null;

  const refs = quantities.map((item) => item.ref).sort();
  const units = Array.from(new Set(quantities.map((item) => item.unit).filter(Boolean))).sort(compareUnitSymbols);
  return [
    `computed_unit_refs=${refs.length}`,
    `units=${units.join(",") || "none"}`,
    "source=computed_preview_result",
    "conversion=false"
  ].join("; ");
}

function computedAxialForce(result: MechanicsResult): { ref: string; unit: string } | null {
  const axialForce =
    result.results.find((item) => item.id === "result:force:pipe-P-120:axial") ??
    result.results.find((item) => item.kind === "element_local_axial_force");
  return axialForce ? { ref: axialForce.id, unit: axialForce.unit } : null;
}

function compareUnitSymbols(left: string, right: string): number {
  const leftIndex = RESULT_UNIT_DISPLAY_ORDER.indexOf(left);
  const rightIndex = RESULT_UNIT_DISPLAY_ORDER.indexOf(right);
  if (leftIndex >= 0 && rightIndex >= 0) return leftIndex - rightIndex;
  if (leftIndex >= 0) return -1;
  if (rightIndex >= 0) return 1;
  return left.localeCompare(right, undefined, { numeric: true, sensitivity: "base" });
}

function KnowledgeSummary({ record, result, computed }: { record: KnowledgeRecord; result: MechanicsResult | null; computed: boolean }) {
  if (!computed) return <>{record.summary}</>;
  const maximum = result?.summary.max_displacement;
  if (record.id === "knowledge:computed-max-displacement" && maximum) {
    return <>{maximum.result_ref} is <QuantityReadout quantity={{ value: maximum.value, unit: maximum.unit, dimension_id: "displacement" }} /> at {maximum.location_ref}.</>;
  }
  const axial = result?.results.find((item) => item.id === "result:force:pipe-P-120:axial") ?? result?.results.find((item) => item.kind === "element_local_axial_force");
  if (record.id === "knowledge:computed-axial-force" && axial) {
    return <>{axial.id} is <QuantityReadout quantity={{ value: axial.value, unit: axial.unit, dimension_id: axial.dimension ?? (axial.kind === "element_local_axial_force" ? "force" : "unknown") }} /> for {axial.entity_ref}.</>;
  }
  return <>{record.summary}</>;
}
