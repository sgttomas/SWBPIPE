// The Project page's Project units (W-17 slice (b); DEL-02-02 Design
// `project-unit-selection.md` §2.5, §2.7, §3 "A Project units section"). It
// shows the seven keys of `project.units`; choosing a symbol builds one
// `set_project_unit` change, checks it with the operation engine
// (validate-only) against the current model hash and, unless it is refused,
// queues it for Review/Apply, undone like any other model edit. A refusal stays
// here by code and message and nothing is queued. The length unit is fixed
// while nodes exist. A key the document lacks gets a notice naming it: an offer
// to set it, or, for `length` with nodes, a re-author diagnostic with no fix.
// Nothing here converts a stored value.
import { useEffect, useRef, useState } from "react";
import type { EditorOperationIntent, OperationOutcomeDiagnostic, PreviewModel } from "../../types";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import { validateModelOperation } from "../../services/operationService";
import { PROJECT_UNIT_KEYS, type ProjectUnitKey } from "../../services/projectService";
import type { UnitCatalogRoute } from "../../services/unitCatalogService";
import { disabledBy } from "../model-data/modelDataIntents";
import {
  PROJECT_UNIT_DIMENSIONS,
  PROJECT_UNIT_LABELS,
  PROJECT_UNIT_PRESETS,
  canonicalCatalogEntry,
  loadedUnitCatalog,
  projectUnitChoices
} from "./projectUnitSelection";
import { SET_PROJECT_UNIT, projectUnitFieldPath, setProjectUnitIntent, storedProjectUnit, type Canonicalize } from "./projectUnitIntents";

export type ProjectUnitsSectionProps = {
  model: PreviewModel;
  queuedIntents?: EditorOperationIntent[];
  onQueueIntent: (intent: EditorOperationIntent) => void;
  operationBusy?: boolean;
  unitCatalogRoute?: UnitCatalogRoute | null;
  canonicalize?: Canonicalize;
};

export const LENGTH_LOCKED_REASON =
  "The length unit is fixed once nodes exist: node positions are stored in it. Enter positions in any length unit; they are converted on entry.";

/**
 * The symbols offered for a key: the catalog's accepted symbols of its
 * dimension when the catalog is loaded (the creation dialog's Custom list);
 * otherwise the stored symbol and the presets' (the engine still judges).
 */
export function projectUnitSectionChoices(model: PreviewModel, key: ProjectUnitKey, route: UnitCatalogRoute | null | undefined): string[] {
  const catalog = loadedUnitCatalog(route ?? null);
  if (catalog) return projectUnitChoices(catalog, key);
  const stored = storedProjectUnit(model, key);
  const known = [
    ...(typeof stored === "string" && stored !== "C" ? [stored] : []),
    ...PROJECT_UNIT_PRESETS.map((preset) => preset.units[key])
  ];
  return [...new Set(known.filter((symbol) => symbol.trim()))];
}

export type ProjectUnitRow = {
  key: ProjectUnitKey;
  /** The stored string, "" when absent or not a string. */
  stored: string;
  state: "set" | "missing" | "invalid";
  /** Why the selector is disabled, apart from a pending check. */
  lockedReason: string | null;
  /** A notice about the stored value (missing key, legacy alias, not a symbol). */
  notice: { kind: "missing-offer" | "missing-hard-lock" | "legacy" | "invalid"; text: string } | null;
};

/** How each key reads and what the section offers for it (§3). */
export function projectUnitRow(model: PreviewModel, key: ProjectUnitKey, route: UnitCatalogRoute | null | undefined): ProjectUnitRow {
  const stored = storedProjectUnit(model, key);
  const hasNodes = (model.nodes?.length ?? 0) > 0;
  const lengthLocked = key === "length" && hasNodes;
  const dimension = PROJECT_UNIT_DIMENSIONS[key];
  if (stored === undefined) {
    if (lengthLocked) {
      return {
        key, stored: "", state: "missing", lockedReason: LENGTH_LOCKED_REASON,
        notice: {
          kind: "missing-hard-lock",
          text: "Missing project unit: length. The model has nodes whose positions have no stated unit, and assigning one would be an interpretation. Re-author the model with a length unit; no fix is offered."
        }
      };
    }
    return {
      key, stored: "", state: "missing", lockedReason: null,
      notice: {
        kind: "missing-offer",
        text: `Missing project unit: ${key}. Choose a ${dimension} unit to set it; the change goes to Review/Apply, and nothing is assumed meanwhile.`
      }
    };
  }
  if (typeof stored !== "string" || !stored.trim()) {
    return {
      key, stored: "", state: "invalid", lockedReason: lengthLocked ? LENGTH_LOCKED_REASON : null,
      notice: lengthLocked
        ? { kind: "missing-hard-lock", text: "project.units.length is not a unit symbol, and the model has nodes whose positions it governs. Re-author the model with a length unit; no fix is offered." }
        : { kind: "invalid", text: `project.units.${key} is not a unit symbol; choose one.` }
    };
  }
  const catalog = loadedUnitCatalog(route ?? null);
  const legacy = stored === "C"
    ? `Stored as C, a legacy spelling. A new choice must be a catalog symbol (degC).`
    : catalog && !canonicalCatalogEntry(catalog, stored, dimension)
      ? `Stored as ${stored}, which is not a catalog ${dimension} symbol. A new choice must be a catalog symbol.`
      : null;
  return {
    key, stored, state: "set", lockedReason: lengthLocked ? LENGTH_LOCKED_REASON : null,
    notice: legacy ? { kind: "legacy", text: legacy } : null
  };
}

export function ProjectUnitsSection(props: ProjectUnitsSectionProps) {
  const { model } = props;
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
  const working = busy || Boolean(props.operationBusy);
  const canonicalize = props.canonicalize ?? canonicalJsonString;

  async function choose(key: ProjectUnitKey, symbol: string) {
    if (working || !symbol) return;
    const captured = props;
    setBusy(true);
    setError("");
    setNotice("");
    setDiagnostics([]);
    try {
      const fieldPath = projectUnitFieldPath(key);
      if (captured.queuedIntents?.some((queued) =>
        queued.change.change_kind === SET_PROJECT_UNIT && queued.change.field_path === fieldPath)) {
        throw new Error(`Review or remove the pending project ${key} unit change before queuing another.`);
      }
      const intent = await setProjectUnitIntent(canonicalize, captured.model, key, symbol);
      const outcome = await validateModelOperation(captured.model, intent, await computeModelHash(captured.model));
      const current = latest.current;
      if (!live.current) return;
      if (current.model !== captured.model || current.queuedIntents !== captured.queuedIntents || current.operationBusy) {
        throw new Error("The model or pending changes changed. Choose the unit again.");
      }
      const blocking = outcome.diagnostics.filter((item) => item.severity === "blocking");
      setDiagnostics(blocking);
      if (blocking.length) {
        setError(`${intent.change.field_label} to ${symbol} was refused by the operation engine; nothing was queued.`);
        return;
      }
      current.onQueueIntent(intent);
      setNotice(`${intent.change.field_label} to ${symbol} checked by the operation engine and queued for Review/Apply.`);
    } catch (caught) {
      if (live.current) setError(caught instanceof Error ? caught.message : String(caught));
    } finally {
      if (live.current) setBusy(false);
    }
  }

  return (
    <section className="project-units" aria-label="Project units" data-testid="project-units-section">
      <p className="model-data-note" data-testid="project-units-section-note">
        Values are stored in the units you enter. Changing a project unit converts no stored value; it sets the
        default unit of new fields. The length unit is fixed once the project has nodes.
      </p>
      {PROJECT_UNIT_KEYS.map((key) => {
        const row = projectUnitRow(model, key, props.unitCatalogRoute);
        const choices = projectUnitSectionChoices(model, key, props.unitCatalogRoute);
        const storedOffered = row.stored !== "" && choices.includes(row.stored);
        return (
          <div className="project-units-row" key={key} data-testid={`project-units-section-row-${key}`}>
            <label>
              {PROJECT_UNIT_LABELS[key]}
              <select
                aria-label={`Project ${PROJECT_UNIT_LABELS[key].toLowerCase()} unit`}
                data-testid={`project-units-section-${key}`}
                value={row.stored}
                {...disabledBy(working, [row.lockedReason !== null, row.lockedReason ?? ""])}
                onChange={(event) => void choose(key, event.target.value)}
              >
                {row.stored === "" ? <option value="" disabled>Choose a unit</option> : null}
                {row.stored !== "" && !storedOffered ? <option value={row.stored} disabled>{`${row.stored} (stored)`}</option> : null}
                {choices.map((symbol) => <option key={symbol} value={symbol}>{symbol}</option>)}
              </select>
            </label>
            {row.lockedReason && row.state === "set" ? (
              <p className="model-data-note" data-testid={`project-units-section-${key}-locked`}>{row.lockedReason}</p>
            ) : null}
            {row.notice ? (
              <p
                className="model-data-note"
                role={row.notice.kind === "missing-hard-lock" ? "alert" : undefined}
                data-testid={`project-units-section-${key}-notice`}
                data-notice={row.notice.kind}
              >
                {row.notice.text}
              </p>
            ) : null}
          </div>
        );
      })}
      {error ? <p role="alert" data-testid="project-units-section-error">{error}</p> : null}
      {diagnostics.length ? (
        <ul aria-label="Operation engine diagnostics" data-testid="project-units-section-diagnostics">
          {diagnostics.map((item) => (
            <li key={item.id} data-testid={`project-units-section-diagnostic-${item.code}`}>
              {item.code}: {item.message}
            </li>
          ))}
        </ul>
      ) : null}
      {notice ? <p role="status" data-testid="project-units-section-notice">{notice}</p> : null}
    </section>
  );
}
