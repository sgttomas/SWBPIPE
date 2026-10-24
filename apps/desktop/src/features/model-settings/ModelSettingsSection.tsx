// The Project page's Model settings (DEL-02-01 Design §3.8; DEL-04-06 Design
// model-topology-checks.md, "User control of the node-proximity tolerance"):
// the node-proximity tolerance of the near-duplicate node warning. It shows
// the effective value ("5 mm (default)" while unset), takes a length in a
// catalog length unit (the project's length unit first), and offers Reset to
// default, which removes the setting. Each edit is checked by the operation
// engine (validate-only) and, when it is not refused, queued for Review/Apply
// and undone like any other model edit; a refusal stays here by name.
import { useEffect, useRef, useState } from "react";
import type { EditorOperationIntent, OperationOutcomeDiagnostic, PreviewModel } from "../../types";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { validateModelOperation } from "../../services/operationService";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { disabledBy, stableJson } from "../model-data/modelDataIntents";
import { PROJECT_UNIT_PRESETS, loadedUnitCatalog, projectUnitChoices } from "../project-units/projectUnitSelection";
import {
  DEFAULT_NODE_PROXIMITY_TOLERANCE,
  NODE_PROXIMITY_TOLERANCE_PATH,
  authoredNodeProximityTolerance,
  clearNodeProximityToleranceIntent,
  effectiveNodeProximityTolerance,
  setNodeProximityToleranceIntent,
  type Canonicalize
} from "./modelSettingsIntents";

export type ModelSettingsSectionProps = {
  model: PreviewModel;
  queuedIntents?: EditorOperationIntent[];
  onQueueIntent: (intent: EditorOperationIntent) => void;
  operationBusy?: boolean;
  unitCatalogRoute?: UnitCatalogRoute | null;
  canonicalize?: Canonicalize;
};

/** The length units offered: the catalog's accepted length symbols when it is
 * loaded; otherwise the project's, the stored one, the default's and the
 * presets' (the engine still judges the unit). */
export function toleranceUnitChoices(model: PreviewModel, route: UnitCatalogRoute | null | undefined): string[] {
  const catalog = loadedUnitCatalog(route ?? null);
  const stored = authoredNodeProximityTolerance(model)?.unit;
  const projectLength = typeof model.project.units.length === "string" ? model.project.units.length : "";
  const known = catalog
    ? projectUnitChoices(catalog, "length")
    : [projectLength, DEFAULT_NODE_PROXIMITY_TOLERANCE.unit, ...PROJECT_UNIT_PRESETS.map((preset) => preset.units.length)];
  return [...new Set([...(stored ? [stored] : []), ...known].filter((symbol) => symbol.trim()))];
}

/** The unit a fresh entry starts in: the stored unit, else the project's length unit, else the default's. */
function initialUnit(model: PreviewModel): string {
  const stored = authoredNodeProximityTolerance(model)?.unit;
  const projectLength = model.project.units.length;
  return stored ?? (typeof projectLength === "string" && projectLength.trim() ? projectLength : DEFAULT_NODE_PROXIMITY_TOLERANCE.unit);
}

export function ModelSettingsSection(props: ModelSettingsSectionProps) {
  const { model } = props;
  const authored = authoredNodeProximityTolerance(model);
  const [valueText, setValueText] = useState(authored ? String(authored.value) : "");
  const [unit, setUnit] = useState(() => initialUnit(model));
  const [busy, setBusy] = useState(false);
  const [error, setError] = useState("");
  const [notice, setNotice] = useState("");
  const [diagnostics, setDiagnostics] = useState<OperationOutcomeDiagnostic[]>([]);
  const latest = useRef(props);
  latest.current = props;
  const live = useRef(false);
  useEffect(() => {
    live.current = true;
    return () => {
      live.current = false;
    };
  }, []);
  // Re-seed the draft whenever the stored setting changes (apply, undo, redo, another document).
  const storedKey = stableJson(authored ?? null);
  useEffect(() => {
    setValueText(authored ? String(authored.value) : "");
    setUnit(initialUnit(model));
  }, [storedKey, model.project.id, model.project.units.length]);

  const choices = toleranceUnitChoices(model, props.unitCatalogRoute);
  const unitOptions = choices.includes(unit) ? choices : [unit, ...choices];
  const working = busy || Boolean(props.operationBusy);
  const unchanged = authored !== null && valueText.trim() === String(authored.value) && unit === authored.unit;
  const canonicalize = props.canonicalize ?? canonicalJsonString;

  async function submit(build: (canonicalize: Canonicalize) => Promise<EditorOperationIntent>) {
    if (working) return;
    const captured = props;
    setBusy(true);
    setError("");
    setNotice("");
    setDiagnostics([]);
    try {
      if (captured.queuedIntents?.some((queued) =>
        queued.target.object_type === "Project" && queued.change.field_path === NODE_PROXIMITY_TOLERANCE_PATH)) {
        throw new Error("Review or remove the pending node-proximity tolerance change before queuing another.");
      }
      const intent = await build(canonicalize);
      const outcome = await validateModelOperation(captured.model, intent, await computeModelHash(captured.model));
      const current = latest.current;
      if (!live.current) return;
      if (current.model !== captured.model || current.queuedIntents !== captured.queuedIntents || current.operationBusy) {
        throw new Error("The model or pending changes changed. Review this draft and queue it again.");
      }
      const blocking = outcome.diagnostics.filter((item) => item.severity === "blocking");
      setDiagnostics(blocking);
      if (blocking.length) {
        setError(`${intent.change.field_label} was refused by the operation engine; nothing was queued.`);
        return;
      }
      current.onQueueIntent(intent);
      setNotice(`${intent.change.field_label} checked by the operation engine and queued for Review/Apply.`);
    } catch (caught) {
      if (live.current) setError(caught instanceof Error ? caught.message : String(caught));
    } finally {
      if (live.current) setBusy(false);
    }
  }

  return (
    <section className="model-settings" aria-label="Model settings" data-testid="model-settings">
      <fieldset className="model-data-fieldset" data-testid="model-settings-node-proximity">
        <legend>Node-proximity tolerance</legend>
        <p data-testid="model-settings-node-proximity-effective">
          Effective: <strong>{effectiveNodeProximityTolerance(model)}</strong>
        </p>
        <p className="model-data-note">
          Distinct nodes that no element joins and that lie no farther apart than this are reported before
          solving as near duplicates (NODE_NEAR_DUPLICATE). The warning changes no solved value.
        </p>
        <label>Tolerance
          <input
            aria-label="Node-proximity tolerance value"
            data-testid="model-settings-node-proximity-value"
            inputMode="decimal"
            placeholder={`${DEFAULT_NODE_PROXIMITY_TOLERANCE.value} ${DEFAULT_NODE_PROXIMITY_TOLERANCE.unit} (default)`}
            value={valueText}
            onChange={(event) => setValueText(event.target.value)}
          />
        </label>
        <label>Unit
          <select
            aria-label="Node-proximity tolerance unit"
            data-testid="model-settings-node-proximity-unit"
            value={unit}
            onChange={(event) => setUnit(event.target.value)}
          >
            {unitOptions.map((symbol) => <option key={symbol} value={symbol}>{symbol}</option>)}
          </select>
        </label>
        <div className="model-data-actions">
          <button
            type="button"
            data-testid="model-settings-node-proximity-queue"
            {...disabledBy(working, [!valueText.trim(), "Enter a tolerance."], [unchanged, "The tolerance is unchanged."])}
            onClick={() => void submit((canon) => setNodeProximityToleranceIntent(canon, model, valueText, unit))}
          >
            Queue tolerance
          </button>
          <button
            type="button"
            data-testid="model-settings-node-proximity-reset"
            {...disabledBy(working, [authored === null, "The 5 mm default already applies."])}
            onClick={() => void submit((canon) => clearNodeProximityToleranceIntent(canon, model))}
          >
            Reset to default
          </button>
        </div>
        {error ? <p role="alert" data-testid="model-settings-node-proximity-error">{error}</p> : null}
        {diagnostics.length ? (
          <ul aria-label="Operation engine diagnostics" data-testid="model-settings-node-proximity-diagnostics">
            {diagnostics.map((item) => (
              <li key={item.id} data-testid={`model-settings-node-proximity-diagnostic-${item.code}`}>
                {item.code}: {item.message}
              </li>
            ))}
          </ul>
        ) : null}
        {notice ? <p role="status" data-testid="model-settings-node-proximity-notice">{notice}</p> : null}
      </fieldset>
    </section>
  );
}
