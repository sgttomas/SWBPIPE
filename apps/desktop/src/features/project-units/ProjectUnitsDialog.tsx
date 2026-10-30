import { useEffect, useLayoutEffect, useRef, useState } from "react";
import { createPortal } from "react-dom";
import {
  PROJECT_UNIT_KEYS,
  SI_PROJECT_UNITS,
  type ProjectUnitKey,
  type ProjectUnitMap
} from "../../services/projectService";
import { loadUnitCatalog, type UnitCatalogRoute } from "../../services/unitCatalogService";
import {
  PROJECT_UNIT_LABELS,
  PROJECT_UNIT_PRESETS,
  composedDefaultsFor,
  loadedUnitCatalog,
  projectUnitChoices,
  projectUnitPresetFor,
  projectUnitSelectionProblems,
  type ProjectUnitPresetId
} from "./projectUnitSelection";

export type BlankProjectKind = "local" | "load_state";

/** An open "Units at creation" request (W-17 §3). Create builds the blank with the map; Cancel creates nothing. */
export type BlankProjectUnitsRequest = {
  kind: BlankProjectKind;
  opener: HTMLElement | null;
  create: (units: ProjectUnitMap) => void;
  cancel: () => void;
};

const KIND_TITLE: Record<BlankProjectKind, string> = {
  local: "New blank project",
  load_state: "New blank load/reference model"
};

/**
 * The units a new blank project starts with. Modal like the replace prompt: the
 * workspace is inert while it is open (App.tsx), Tab stays inside, window
 * shortcuts are held back, and Escape cancels. SI is selected. A Custom choice
 * per key lists the accepted catalog symbols for that key's dimension, from the
 * catalog the desktop loads; where the catalog is not exposed only the presets
 * are offered.
 */
export function ProjectUnitsDialog({ request }: { request: BlankProjectUnitsRequest | null }) {
  if (!request) return null;
  return <OpenProjectUnitsDialog request={request} />;
}

function OpenProjectUnitsDialog({ request }: { request: BlankProjectUnitsRequest }) {
  const dialog = useRef<HTMLElement>(null);
  const presetSelect = useRef<HTMLSelectElement>(null);
  const [route, setRoute] = useState<UnitCatalogRoute | null>(null);
  const [units, setUnits] = useState<ProjectUnitMap>({ ...SI_PROJECT_UNITS });
  const [preset, setPreset] = useState<ProjectUnitPresetId>("si");

  useEffect(() => {
    let active = true;
    loadUnitCatalog()
      .then((loaded) => { if (active) setRoute(loaded); })
      .catch((error: unknown) => {
        if (active) setRoute({ route: "unavailable_browser_preview", diagnostic: `UNIT-CATALOG-LOAD-FAILED: ${String(error)}` });
      });
    return () => { active = false; };
  }, []);

  // Focus and the key hold are set inside the commit that shows the dialog,
  // as in ReplaceProjectPrompt: a passive effect could let an early key through.
  useLayoutEffect(() => {
    const opener = request.opener;
    presetSelect.current?.focus();
    const hold = (event: KeyboardEvent) => {
      if (event.key === "Escape") {
        event.preventDefault();
        event.stopImmediatePropagation();
        request.cancel();
        return;
      }
      if (event.key === "Tab") {
        const controls = [...(dialog.current?.querySelectorAll<HTMLElement>("select:not(:disabled), button:not(:disabled)") ?? [])];
        if (controls.length === 0) return;
        const at = controls.indexOf(document.activeElement as HTMLElement);
        const next = event.shiftKey ? (at <= 0 ? controls.length - 1 : at - 1) : (at + 1) % controls.length;
        event.preventDefault();
        event.stopImmediatePropagation();
        controls[next].focus();
        return;
      }
      if (event.metaKey || event.ctrlKey || event.altKey) {
        event.preventDefault();
        event.stopImmediatePropagation();
      }
    };
    window.addEventListener("keydown", hold, true);
    return () => {
      window.removeEventListener("keydown", hold, true);
      if (opener?.isConnected) requestAnimationFrame(() => opener.focus());
    };
  }, [request]);

  const catalog = loadedUnitCatalog(route);
  const problems = projectUnitSelectionProblems(route, units);
  const composed = preset === "custom" ? composedDefaultsFor(route, units) : [];

  function choosePreset(value: string) {
    const chosen = PROJECT_UNIT_PRESETS.find((candidate) => candidate.id === value);
    if (chosen) {
      setUnits({ ...chosen.units });
      setPreset(chosen.id);
    } else if (value === "custom") {
      setPreset("custom");
    }
  }

  function chooseUnit(key: ProjectUnitKey, symbol: string) {
    const next = { ...units, [key]: symbol };
    setUnits(next);
    setPreset(projectUnitPresetFor(next));
  }

  return createPortal(
    <div className="replace-project-backdrop" data-testid="project-units-backdrop">
      <section
        ref={dialog}
        role="dialog"
        aria-modal="true"
        aria-labelledby="project-units-title"
        aria-describedby="project-units-detail"
        className="replace-project-prompt project-units-dialog panel"
        data-testid="project-units-dialog"
        data-kind={request.kind}
      >
        <h2 id="project-units-title">{KIND_TITLE[request.kind]}: units</h2>
        <p id="project-units-detail" data-testid="project-units-detail">
          Values are stored in the units you enter, and the solver converts them at its boundary.
          The length unit is fixed once the project has nodes.
        </p>
        <label className="project-units-preset">
          <span>Preset</span>
          <select
            ref={presetSelect}
            aria-label="Unit preset"
            data-testid="project-units-preset"
            value={preset}
            onChange={(event) => choosePreset(event.target.value)}
          >
            {PROJECT_UNIT_PRESETS.map((candidate) => (
              <option key={candidate.id} value={candidate.id}>{candidate.label}</option>
            ))}
            <option value="custom" disabled={!catalog}>Custom</option>
          </select>
        </label>
        <table className="project-units-table" data-testid="project-units-table">
          <tbody>
            {PROJECT_UNIT_KEYS.map((key) => {
              const choices = catalog ? projectUnitChoices(catalog, key) : [];
              const options = choices.includes(units[key]) ? choices : [units[key], ...choices];
              return (
                <tr key={key}>
                  <th scope="row">{PROJECT_UNIT_LABELS[key]}</th>
                  <td>
                    <select
                      aria-label={`${PROJECT_UNIT_LABELS[key]} unit`}
                      data-testid={`project-units-${key}`}
                      disabled={!catalog}
                      value={units[key]}
                      onChange={(event) => chooseUnit(key, event.target.value)}
                    >
                      {options.map((symbol) => <option key={symbol} value={symbol}>{symbol}</option>)}
                    </select>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
        {!catalog ? (
          <p className="muted" data-testid="project-units-catalog-status">
            {route === null
              ? "Loading the unit catalog. Presets are available meanwhile."
              : `Custom units need the desktop unit catalog; presets are available.${route.route === "unavailable_browser_preview" ? ` ${route.diagnostic}` : ""}`}
          </p>
        ) : null}
        {composed.length ? (
          <div data-testid="project-units-composed">
            <p>Composed defaults for new fields:</p>
            <ul>
              {composed.map((item) => (
                <li key={item.kind} data-testid={`project-units-composed-${item.kind}`} data-in-catalog={item.inCatalog === null ? "unknown" : String(item.inCatalog)}>
                  {item.label}: {item.symbol}
                  {item.inCatalog === false ? ` (not a catalog unit; that field will start unselected)` : ""}
                </li>
              ))}
            </ul>
          </div>
        ) : null}
        {problems.length ? (
          <ul role="alert" data-testid="project-units-problems">
            {problems.map((problem) => <li key={problem.key}>{PROJECT_UNIT_LABELS[problem.key]}: {problem.message}</li>)}
          </ul>
        ) : null}
        <div className="report-actions">
          <button type="button" data-testid="project-units-cancel" onClick={() => request.cancel()}>
            Cancel
          </button>
          <button
            type="button"
            data-testid="project-units-create"
            disabled={problems.length > 0}
            onClick={() => { if (problems.length === 0) request.create({ ...units }); }}
          >
            Create
          </button>
        </div>
      </section>
    </div>,
    document.body
  );
}
