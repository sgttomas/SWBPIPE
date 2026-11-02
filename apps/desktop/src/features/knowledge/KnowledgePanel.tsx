import { useEffect, useState } from "react";
import { isNativeResult, lookupNativeRows, type SessionResult } from "../../services/nativeResult";
import { nativeFailureText } from "../../services/nativeIngress";
import { QuantityReadout } from "../display-units";
import { BookOpen } from "lucide-react";
import type { DesignKnowledge, KnowledgeRecord, MechanicsResult } from "../../types";

const RESULT_UNIT_DISPLAY_ORDER = ["MPa", "N", "N*m", "mm", "rad"];

type Row = MechanicsResult["results"][number];
const AXIAL_ROW_ID = "result:force:pipe-P-120:axial";
const AXIAL_ROW_KIND = "element_local_axial_force";

/** The axial-force row the knowledge records review: the row with the fixed id, else the first row
 * of its kind (`results.find`). A full result's from its rows; a native result's by one bounded
 * native lookup (`useKnowledgeAxialRow`). */
function axialRowOf(result: MechanicsResult): Row | null {
  return result.results.find((item) => item.id === AXIAL_ROW_ID) ?? result.results.find((item) => item.kind === AXIAL_ROW_KIND) ?? null;
}
export function useKnowledgeAxialRow(result: SessionResult | null): { row: Row | null; reading: boolean; refused: string | null } {
  const [found, setFound] = useState<{ of: SessionResult; row: Row | null; refused: string | null } | null>(null);
  useEffect(() => {
    if (!result || !isNativeResult(result)) return;
    let live = true;
    lookupNativeRows(result, { ids: [AXIAL_ROW_ID], kinds: [AXIAL_ROW_KIND], match: "first" }).then(
      (lookup) => { if (live) setFound({ of: result, row: (lookup.ids[0] ?? lookup.kinds[0])?.row ?? null, refused: null }); },
      // A refused lookup is said as such, never shown as a result without the row.
      (error) => { if (live) setFound({ of: result, row: null, refused: nativeFailureText(error) }); }
    );
    return () => { live = false; };
  }, [result]);
  if (!result) return { row: null, reading: false, refused: null };
  if (!isNativeResult(result)) return { row: axialRowOf(result), reading: false, refused: null };
  return found && found.of === result ? { row: found.row, reading: false, refused: found.refused } : { row: null, reading: true, refused: null };
}

export function KnowledgePanel({ knowledge, result }: { knowledge: DesignKnowledge | null; result: SessionResult | null }) {
  const axial = useKnowledgeAxialRow(result);
  const computedRecords = physicsRecords(result, axial.row);
  const records = [...(knowledge?.records ?? []), ...computedRecords];
  const unitContext = computedKnowledgeUnitContext(result, axial.row);
  return (
    <section className="panel knowledge-panel" aria-label="Design knowledge" data-testid="knowledge-panel">
      <div className="panel-title">
        <BookOpen size={16} />
        Design Knowledge
      </div>
      {axial.refused ? (
        <p role="status" data-testid="knowledge-axial-force-refused">{`The computed axial-force record could not be read from the result: ${axial.refused}`}</p>
      ) : null}
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
              <p><KnowledgeSummary record={record} result={result} computed={computedRecords.includes(record)} axialRow={axial.row} /></p>
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

export function physicsRecords(result: SessionResult | null, axialRow?: Row | null): KnowledgeRecord[] {
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

  const axialForce = axialRow !== undefined ? axialRow : isNativeResult(result) ? null : axialRowOf(result);
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

function computedKnowledgeUnitContext(result: SessionResult | null, axialRow: Row | null): string | null {
  if (!result) return null;

  const quantities = [
    result.summary.max_displacement
      ? {
          ref: result.summary.max_displacement.result_ref,
          unit: result.summary.max_displacement.unit
        }
      : null,
    axialRow ? { ref: axialRow.id, unit: axialRow.unit } : null
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


function compareUnitSymbols(left: string, right: string): number {
  const leftIndex = RESULT_UNIT_DISPLAY_ORDER.indexOf(left);
  const rightIndex = RESULT_UNIT_DISPLAY_ORDER.indexOf(right);
  if (leftIndex >= 0 && rightIndex >= 0) return leftIndex - rightIndex;
  if (leftIndex >= 0) return -1;
  if (rightIndex >= 0) return 1;
  return left.localeCompare(right, undefined, { numeric: true, sensitivity: "base" });
}

function KnowledgeSummary({ record, result, computed, axialRow }: { record: KnowledgeRecord; result: SessionResult | null; computed: boolean; axialRow: Row | null }) {
  if (!computed) return <>{record.summary}</>;
  const maximum = result?.summary.max_displacement;
  if (record.id === "knowledge:computed-max-displacement" && maximum) {
    return <>{maximum.result_ref} is <QuantityReadout quantity={{ value: maximum.value, unit: maximum.unit, dimension_id: "displacement" }} /> at {maximum.location_ref}.</>;
  }
  const axial = axialRow;
  if (record.id === "knowledge:computed-axial-force" && axial) {
    return <>{axial.id} is <QuantityReadout quantity={{ value: axial.value, unit: axial.unit, dimension_id: axial.dimension ?? (axial.kind === "element_local_axial_force" ? "force" : "unknown") }} /> for {axial.entity_ref}.</>;
  }
  return <>{record.summary}</>;
}
