// The Loads page's Wind and Seismic sections for the selected case (DEC-068 item 2, on the
// DEL-07-06 presentation): the case's `equivalent_static` record in an engineer's terms, each
// quantity with its unit beside it. Each changed field is one `update_load` operation, the one
// the inspector queues (`equivalentStaticIntents.ts`); the section's Queue has the engine check
// the changes as one sequence and queues them for Review/Apply. Nothing is defaulted.
import { useMemo, useState } from "react";
import { ListPlus } from "lucide-react";
import type { PreviewModel } from "../../types";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { acceptedUnits, unitEntryMatchesDimension } from "../../services/unitCatalogService";
import type { EngineCheckedQueueProps } from "../condition-sets/engineCheckedQueue";
import { disabledBy, useReseed } from "../model-data/modelDataIntents";
import { VirtualMultiTargetPicker } from "../workspace/VirtualTargetPicker";
import { SectionFindings, useCheckedSectionQueue } from "./caseSectionQueue";
import {
  SEISMIC_FIELDS,
  WIND_DIRECTIONS,
  WIND_DIRECTION_LABEL,
  WIND_FIELDS,
  entryChanged,
  entryOf,
  finiteNumber,
  staticFieldIntent,
  type StaticField
} from "./equivalentStaticIntents";
import { shortRef } from "./LoadsPageParts";

type LoadCase = PreviewModel["load_cases"][number];
type Entry = { value: string; unit: string };
type SectionProps = EngineCheckedQueueProps & { loadCase: LoadCase; unitCatalogRoute: UnitCatalogRoute | null };

/** The drafts of a section's fields, re-seeded whenever the stored record changes. */
function useFieldDrafts<K extends string>(loadCase: LoadCase, fields: Record<K, StaticField>, initial: (key: K, entry: Entry) => Entry) {
  const seed = () => Object.fromEntries(Object.entries(fields).map(([key, field]) => [key, initial(key as K, entryOf(loadCase, field as StaticField))])) as Record<K, Entry>;
  const [drafts, setDrafts] = useState(seed);
  useReseed([loadCase.id, loadCase.equivalent_static ?? null], () => setDrafts(seed()));
  const set = (key: K, patch: Partial<Entry>) => setDrafts((current) => ({ ...current, [key]: { ...current[key], ...patch } }));
  return [drafts, set] as const;
}

/** The changed fields' intents in order, or the reason none can be queued. */
function pending<K extends string>(loadCase: LoadCase, fields: Record<K, StaticField>, drafts: Record<K, Entry>) {
  const changed = (Object.keys(fields) as K[]).filter((key) => entryChanged(loadCase, fields[key], drafts[key]));
  for (const key of changed) {
    const field = fields[key];
    const entry = drafts[key];
    if ((field.kind === "quantity" || field.kind === "factor") && finiteNumber(entry.value) === null) return { problem: `Enter a number for ${field.label.toLowerCase()}.` };
    if (field.kind === "quantity" && !entry.unit.trim()) return { problem: `Enter the unit of ${field.label.toLowerCase()}.` };
  }
  if (!changed.length) return { problem: "Nothing to queue: the case already holds these values." };
  return { problem: null, intents: changed.map((key) => staticFieldIntent(loadCase, fields[key], drafts[key])) };
}

function catalogUnits(route: UnitCatalogRoute | null, dimension: string): string[] {
  if (route?.route !== "tauri_unit_catalog" || !Array.isArray(route.catalog?.entries)) return [];
  return acceptedUnits(route.catalog).filter((entry) => unitEntryMatchesDimension(entry, dimension)).map((entry) => entry.symbol);
}

/** A value with its unit beside it: the unit is entered (suggested from the unit catalog where
 * the desktop has it) for a quantity, and shown for a factor. */
function ValueField({ label, entry, onChange, testId, unit, units, unitPlaceholder }: {
  label: string;
  entry: Entry;
  onChange: (patch: Partial<Entry>) => void;
  testId: string;
  /** A factor's fixed unit text; a quantity has none and takes an entered unit. */
  unit?: string;
  units?: readonly string[];
  unitPlaceholder?: string;
}) {
  const listId = `${testId}-units`;
  return (
    <label className="loads-value-field">
      <span>{label}</span>
      <span className="loads-value-entry">
        <input aria-label={label} data-testid={`${testId}-value`} inputMode="decimal" value={entry.value} onChange={(event) => onChange({ value: event.target.value })} />
        {unit !== undefined ? (
          <span className="loads-unit" data-testid={`${testId}-unit`}>{unit}</span>
        ) : (
          <>
            <input aria-label={`${label} unit`} className="loads-unit-input" data-testid={`${testId}-unit`} value={entry.unit} placeholder={unitPlaceholder ?? "unit"}
              list={units?.length ? listId : undefined} onChange={(event) => onChange({ unit: event.target.value })} />
            {units?.length ? <datalist id={listId}>{units.map((symbol) => <option key={symbol} value={symbol} />)}</datalist> : null}
          </>
        )}
      </span>
    </label>
  );
}

export function WindSection(props: SectionProps) {
  const { model, loadCase, unitCatalogRoute } = props;
  const [drafts, set] = useFieldDrafts(loadCase, WIND_FIELDS, (key, entry) =>
    key === "pressure" && !entry.unit ? { ...entry, unit: model.project.units.pressure ?? "" } : entry);
  const queue = useCheckedSectionQueue(props);
  const pipeOptions = useMemo(() => model.pipe_segments.map((pipe) => ({ value: pipe.id, label: pipe.label || shortRef(pipe.id), keywords: [pipe.from, pipe.to] })), [model.pipe_segments]);
  const pipes = drafts.exposedPipes.value.split(",").map((ref) => ref.trim()).filter(Boolean);
  const spans = loadCase.equivalent_static?.wind?.exposed_spans ?? [];
  const next = pending(loadCase, WIND_FIELDS, drafts);
  return (
    <div className="loads-static" data-testid="load-case-wind">
      <div className="loads-fields">
        <ValueField label="Wind pressure" entry={drafts.pressure} onChange={(patch) => set("pressure", patch)} testId="load-case-wind-pressure" units={catalogUnits(unitCatalogRoute, "pressure")} />
        <ValueField label="Shape factor" entry={drafts.shapeFactor} onChange={(patch) => set("shapeFactor", patch)} testId="load-case-wind-shape-factor" unit="–" />
        <label>
          <span>Direction</span>
          <select aria-label="Wind direction" data-testid="load-case-wind-direction" value={drafts.direction.value} onChange={(event) => set("direction", { value: event.target.value })}>
            <option value="">Choose…</option>
            {drafts.direction.value && !(WIND_DIRECTIONS as readonly string[]).includes(drafts.direction.value) ? <option value={drafts.direction.value}>{drafts.direction.value} (as stored)</option> : null}
            {WIND_DIRECTIONS.map((token) => <option key={token} value={token}>{WIND_DIRECTION_LABEL[token]}</option>)}
          </select>
        </label>
      </div>
      <div className="loads-picker">
        <VirtualMultiTargetPicker label="Exposed pipes" testId="load-case-wind-pipes" options={pipeOptions} values={pipes} onChange={(values) => set("exposedPipes", { value: values.join(", ") })} />
      </div>
      {spans.length ? (
        <dl className="loads-values">
          <dt>Exposed spans</dt>
          <dd data-testid="load-case-wind-spans">
            {spans.map((span) => `${shortRef(span.pipe_ref)} from ${span.start_fraction?.value} to ${span.end_fraction?.value}`).join("; ")}
          </dd>
        </dl>
      ) : null}
      <div className="loads-form-actions">
        <button type="button" className="loads-primary-button" data-testid="load-case-wind-queue" {...disabledBy(queue.busy, [next.problem !== null, next.problem ?? ""])}
          onClick={() => void queue.submit("Wind", async () => next.intents ?? [])}>
          <ListPlus size={14} aria-hidden="true" />
          Queue wind
        </button>
      </div>
      <SectionFindings error={queue.error} notice={queue.notice} diagnostics={queue.diagnostics} testId="load-case-wind" />
    </div>
  );
}

export function SeismicSection(props: SectionProps) {
  const { model, loadCase, unitCatalogRoute } = props;
  const [drafts, set] = useFieldDrafts(loadCase, SEISMIC_FIELDS, (_key, entry) => entry);
  const queue = useCheckedSectionQueue(props);
  const next = pending(loadCase, SEISMIC_FIELDS, drafts);
  return (
    <div className="loads-static" data-testid="load-case-seismic">
      <div className="loads-fields">
        <ValueField label="Gravity acceleration" entry={drafts.gravity} onChange={(patch) => set("gravity", patch)} testId="load-case-seismic-gravity" units={catalogUnits(unitCatalogRoute, "acceleration")} unitPlaceholder="unit, e.g. m/s^2" />
        <ValueField label="X factor" entry={drafts.x} onChange={(patch) => set("x", patch)} testId="load-case-seismic-x" unit="g" />
        <ValueField label="Y factor" entry={drafts.y} onChange={(patch) => set("y", patch)} testId="load-case-seismic-y" unit="g" />
        <ValueField label="Z factor" entry={drafts.z} onChange={(patch) => set("z", patch)} testId="load-case-seismic-z" unit="g" />
      </div>
      <div className="loads-form-actions">
        <button type="button" className="loads-primary-button" data-testid="load-case-seismic-queue" {...disabledBy(queue.busy, [next.problem !== null, next.problem ?? ""])}
          onClick={() => void queue.submit("Seismic", async () => next.intents ?? [])}>
          <ListPlus size={14} aria-hidden="true" />
          Queue seismic
        </button>
      </div>
      <SectionFindings error={queue.error} notice={queue.notice} diagnostics={queue.diagnostics} testId="load-case-seismic" />
      <CaseModelFindings model={model} caseId={loadCase.id} testId="load-case-seismic-model-findings" matches={(code) => /SEISMIC|EQUIVALENT_STATIC|G_FACTOR|GRAVITY/.test(code)} />
    </div>
  );
}

/** The model check's findings for this case, as the document carries them: the place a
 * section's model-check warnings render. Empty when there are none. */
export function CaseModelFindings({ model, caseId, testId, matches }: { model: PreviewModel; caseId: string; testId: string; matches: (code: string) => boolean }) {
  const findings = (model.diagnostics ?? []).filter((item) => (item.affected_refs ?? []).includes(caseId) && matches(item.code));
  return (
    <div className="loads-model-findings" data-testid={testId} aria-label="Model check" hidden={findings.length === 0}>
      {findings.length ? (
        <ul className="loads-findings">
          {findings.map((item, index) => (
            <li key={`${item.id}:${index}`} data-testid={`${testId}-${item.code}`} data-severity={item.severity}>
              <strong>Model check: {item.severity}</strong> <code>{item.code}</code>
              <span className="loads-finding-message">{item.message}</span>
            </li>
          ))}
        </ul>
      ) : null}
    </div>
  );
}
