import { useEffect, useState } from "react";
import { Library, ShieldAlert } from "lucide-react";
import type { PreviewModel } from "../../types";
import {
  buildInventedLibraryImportTemplate,
  deleteLocalLibrary,
  listLocalLibraries,
  openLocalLibrary,
  partitionLibraryImportFindings,
  saveLocalLibrary,
  validateLibraryImport,
  type IntendedVisibility,
  type LibraryImportFinding,
  type LibraryImportValidation,
  type LibraryKind,
  type LocalLibraryIndexEntry
} from "../../services/libraryImportService";
import {
  acceptedUnits,
  describeUnitBasis,
  loadUnitCatalog,
  unitEntryMatchesDimension,
  type UnitCatalogRoute
} from "../../services/unitCatalogService";

// Private Library Manager (PRD §13 / §14.6 / §14.1 "Library manager"). Phase C3
// GUI slice (TP-C3-LIBGUI-001), replicating the C2 Rule-Pack Manager pattern:
// import an already-parsed material/section/component library document, validate
// it through the DEL-03-07 provenance seam (the desktop `validate_library_import`
// command), and persist accepted private imports to the local-only store
// (`save_local_library`). The findings display is split along the PRD §13.5
// blocking-vs-advisory axis. The local store is private-by-default and persists
// only an accepted (PRIVATE_LOCAL_ONLY) import — a suspected-protected
// (QUARANTINE) or otherwise-blocked (REJECTED) import is refused, never written
// (DEC-036). Imported private libraries are never transmitted, committed, or
// bundled into public artifacts (OPS-K-PRIV-1, PRD §13.5/§17.3). Every status is
// a software finding only — never a legal, certification, sealing,
// authentication, or code-compliance determination.
//
// The backend runs in the desktop (Tauri) runtime only; browser preview reports
// the explicit unavailable route rather than synthesizing a fallback validator
// or store. Reference wiring between rule packs and imported library allowables
// is a separate Phase C3 slice and is intentionally absent here.

const NO_DRAFT_REASON =
  "No import document; load an invented sample or paste a library document first.";
const NO_PROJECT_REASON =
  "Local libraries are project-scoped; create or open a local project first.";
const BUSY_REASON =
  "A library-import backend request is in progress; wait for it to finish before editing or running another action.";

const LIBRARY_KINDS: ReadonlyArray<{ value: LibraryKind; label: string }> = [
  { value: "material", label: "Material library" },
  { value: "section", label: "Section library" },
  { value: "component", label: "Component library" }
];

const VISIBILITY_OPTIONS: ReadonlyArray<{ value: IntendedVisibility; label: string }> = [
  { value: "private", label: "Private (local-only)" },
  { value: "public", label: "Public (validation preview)" }
];

type DraftState = {
  text: string;
  origin: string;
};

type ComponentFieldDraft = {
  fieldKind: keyof typeof COMPONENT_FIELD_SPECS;
  magnitude: string;
  unit: string;
};

type MaterialPropertyDraft = {
  propertyKind: keyof typeof MATERIAL_PROPERTY_SPECS;
  magnitude: string;
  unit: string;
  unitRefId: string;
};

type SectionQuantityDraft = {
  quantityKind: keyof typeof SECTION_QUANTITY_SPECS;
  magnitude: string;
  unit: string;
};

type LibraryR3JourneyEvent = "library_template_loaded" | "library_validate_requested" | "library_save_requested";

const SECTION_QUANTITY_SPECS = {
  outside_diameter: {
    dimension: "length",
    defaultUnit: "m",
    label: "Outside diameter",
    requiredFor: "section_property_calculation",
    slotType: "dimension"
  },
  wall_thickness: {
    dimension: "length",
    defaultUnit: "m",
    label: "Wall thickness",
    requiredFor: "section_property_calculation",
    slotType: "dimension"
  },
  cross_section_area: {
    dimension: "area",
    defaultUnit: "m^2",
    label: "Cross-section area",
    requiredFor: "mechanics_solve",
    slotType: "property"
  },
  moment_of_inertia: {
    dimension: "second_moment_area",
    defaultUnit: "m^4",
    label: "Moment of inertia",
    requiredFor: "mechanics_solve",
    slotType: "property"
  },
  section_modulus: {
    dimension: "section_modulus",
    defaultUnit: "m^3",
    label: "Section modulus",
    requiredFor: "rule_check",
    slotType: "property"
  },
  mass_per_length: {
    dimension: "mass_per_length",
    defaultUnit: "kg/m",
    label: "Mass per length",
    requiredFor: "mechanics_solve",
    slotType: "property"
  }
} as const;

const MATERIAL_PROPERTY_SPECS = {
  density: {
    dimension: "density",
    defaultUnit: "kg/m^3",
    defaultUnitRefId: "unit:kilogram_per_cubic_meter",
    label: "Density",
    requiredFor: "mechanics_solve"
  },
  elastic_modulus: {
    dimension: "stress",
    defaultUnit: "Pa",
    defaultUnitRefId: "unit:pascal",
    label: "Elastic modulus",
    requiredFor: "mechanics_solve"
  },
  thermal_expansion_coefficient: {
    dimension: "thermal_expansion_coefficient",
    defaultUnit: "1/degC",
    defaultUnitRefId: "unit:per_degree_celsius",
    label: "Thermal expansion coefficient",
    requiredFor: "mechanics_solve"
  }
} as const;

const COMPONENT_FIELD_SPECS = {
  linear_stiffness: {
    dimension: "linear_stiffness",
    defaultUnit: "N/m",
    label: "Linear stiffness"
  },
  bend_radius: {
    dimension: "length",
    defaultUnit: "m",
    label: "Bend radius"
  },
  weight: {
    dimension: "force",
    defaultUnit: "N",
    label: "Weight"
  }
} as const;

export function LibraryManagerPanel({
  model,
  onR3JourneyEvent
}: {
  model: PreviewModel | null;
  onR3JourneyEvent?: (event: LibraryR3JourneyEvent) => void;
}) {
  const [draft, setDraft] = useState<DraftState | null>(null);
  const [libraryKind, setLibraryKind] = useState<LibraryKind>("material");
  const [intendedVisibility, setIntendedVisibility] = useState<IntendedVisibility>("private");
  const [actionStatus, setActionStatus] = useState<string>(
    "No library-import action has run in this session."
  );
  const [listStatus, setListStatus] = useState<string>("Local library list not refreshed yet.");
  const [entries, setEntries] = useState<LocalLibraryIndexEntry[]>([]);
  const [validation, setValidation] = useState<LibraryImportValidation | null>(null);
  const [unitCatalogRoute, setUnitCatalogRoute] = useState<UnitCatalogRoute | null>(null);
  const [componentFieldDraft, setComponentFieldDraft] = useState<ComponentFieldDraft>({
    fieldKind: "linear_stiffness",
    magnitude: "0",
    unit: COMPONENT_FIELD_SPECS.linear_stiffness.defaultUnit
  });
  const [materialPropertyDraft, setMaterialPropertyDraft] = useState<MaterialPropertyDraft>({
    propertyKind: "density",
    magnitude: "0",
    unit: MATERIAL_PROPERTY_SPECS.density.defaultUnit,
    unitRefId: MATERIAL_PROPERTY_SPECS.density.defaultUnitRefId
  });
  const [sectionQuantityDraft, setSectionQuantityDraft] = useState<SectionQuantityDraft>({
    quantityKind: "outside_diameter",
    magnitude: "0",
    unit: SECTION_QUANTITY_SPECS.outside_diameter.defaultUnit
  });
  // In-request busy guard: while a backend command is awaiting, the draft
  // textarea, the kind/visibility selectors, and every action are disabled so
  // an async response that calls setDraft (open) cannot clobber a mid-request
  // edit — mirrors the C2 rule-pack manager's slice-1 residual fix.
  const [inFlight, setInFlight] = useState(false);

  const projectId = model?.project.id ?? null;
  const draftReason = !draft ? NO_DRAFT_REASON : inFlight ? BUSY_REASON : undefined;
  const componentSpec = COMPONENT_FIELD_SPECS[componentFieldDraft.fieldKind];
  const componentUnitBasis = describeUnitBasis(
    unitCatalogRoute,
    componentFieldDraft.unit,
    componentSpec.dimension
  );
  const materialSpec = MATERIAL_PROPERTY_SPECS[materialPropertyDraft.propertyKind];
  const materialUnitBasis = describeUnitBasis(
    unitCatalogRoute,
    materialPropertyDraft.unit,
    materialSpec.dimension
  );
  const sectionSpec = SECTION_QUANTITY_SPECS[sectionQuantityDraft.quantityKind];
  const sectionUnitBasis = describeUnitBasis(
    unitCatalogRoute,
    sectionQuantityDraft.unit,
    sectionSpec.dimension
  );

  // The stored-library list is project-scoped. When the active project changes
  // (open/create/switch), the previously-listed libraries belong to the old
  // project and would contradict the scope banner, so clear them; the session
  // draft is intentionally preserved (it is authored, not bound to a project
  // until saved).
  useEffect(() => {
    setEntries([]);
    setListStatus("Local library list not refreshed for this project yet.");
  }, [projectId]);

  useEffect(() => {
    let active = true;
    loadUnitCatalog()
      .then((route) => {
        if (active) setUnitCatalogRoute(route);
      })
      .catch((error) => {
        if (!active) return;
        setUnitCatalogRoute({
          route: "unavailable_browser_preview",
          diagnostic: `UNIT-CATALOG-ERROR: ${String(error)}`
        });
      });
    return () => {
      active = false;
    };
  }, []);

  // Backend (Tauri) commands can reject in desktop mode. Browser preview never
  // reaches them (the services short-circuit to an explicit unavailable route),
  // but a desktop backend error must surface honestly rather than become a
  // silent unhandled rejection.
  function reportBackendError(action: string, error: unknown): void {
    setActionStatus(`LIBRARY-IMPORT-BACKEND-ERROR (${action}): ${String(error)}`);
  }

  function parseDraft(): Record<string, unknown> | null {
    if (!draft) return null;
    try {
      const parsed: unknown = JSON.parse(draft.text);
      if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
        setActionStatus("LIBRARY-IMPORT-DRAFT-JSON-INVALID: the import document must be a JSON object.");
        return null;
      }
      return parsed as Record<string, unknown>;
    } catch (error) {
      setActionStatus(`LIBRARY-IMPORT-DRAFT-JSON-INVALID: ${String(error)}`);
      return null;
    }
  }

  function handleLoadTemplate() {
    const document = buildInventedLibraryImportTemplate(libraryKind);
    setDraft({
      text: JSON.stringify(document, null, 2),
      origin: `invented_${libraryKind}_template_private_by_default`
    });
    setValidation(null);
    onR3JourneyEvent?.("library_template_loaded");
    setActionStatus(
      `Invented private ${libraryKind}-library template loaded in memory ` +
        "(privacy_class=private_user_data, redistribution_status=private_only). " +
        "Nothing is stored until you validate and save."
    );
  }

  function handleDiscardDraft() {
    setDraft(null);
    setValidation(null);
    setActionStatus("Import document discarded from session memory; the local store was not touched.");
  }

  function handleComponentFieldKindChange(fieldKind: ComponentFieldDraft["fieldKind"]) {
    setComponentFieldDraft((current) => ({
      ...current,
      fieldKind,
      unit: COMPONENT_FIELD_SPECS[fieldKind].defaultUnit
    }));
  }

  function handleMaterialPropertyKindChange(propertyKind: MaterialPropertyDraft["propertyKind"]) {
    setMaterialPropertyDraft((current) => ({
      ...current,
      propertyKind,
      unit: MATERIAL_PROPERTY_SPECS[propertyKind].defaultUnit,
      unitRefId: MATERIAL_PROPERTY_SPECS[propertyKind].defaultUnitRefId
    }));
  }

  function handleMaterialUnitChange(unitRefId: string) {
    const option = materialUnitOptions(unitCatalogRoute, materialSpec, materialPropertyDraft).find(
      (item) => item.unitRefId === unitRefId
    );
    setMaterialPropertyDraft((current) => ({
      ...current,
      unitRefId,
      unit: option?.symbol ?? current.unit
    }));
  }

  function handleSectionQuantityKindChange(quantityKind: SectionQuantityDraft["quantityKind"]) {
    setSectionQuantityDraft((current) => ({
      ...current,
      quantityKind,
      unit: SECTION_QUANTITY_SPECS[quantityKind].defaultUnit
    }));
  }

  function handleApplyComponentFieldDraft() {
    const document = parseDraft();
    if (!document) return;
    const magnitude = Number(componentFieldDraft.magnitude);
    if (!Number.isFinite(magnitude)) {
      setActionStatus("COMPONENT-FIELD-DRAFT-INVALID: field magnitude must be finite.");
      return;
    }
    const nextDocument = applyComponentFieldDraft(document, {
      ...componentFieldDraft,
      magnitude: String(magnitude)
    });
    setDraft({
      text: JSON.stringify(nextDocument, null, 2),
      origin: `${draft?.origin ?? "session"}:component_field_unit_helper`
    });
    setValidation(null);
    setActionStatus(
      `Component field draft updated: field_kind=${componentFieldDraft.fieldKind}; ` +
        `dimension=${componentSpec.dimension}; unit=${componentFieldDraft.unit}; ` +
        "private_user_supplied only."
    );
  }

  function handleApplyMaterialPropertyDraft() {
    const document = parseDraft();
    if (!document) return;
    const magnitude = Number(materialPropertyDraft.magnitude);
    if (!Number.isFinite(magnitude)) {
      setActionStatus("MATERIAL-PROPERTY-DRAFT-INVALID: property magnitude must be finite.");
      return;
    }
    const nextDocument = applyMaterialPropertyDraft(document, {
      ...materialPropertyDraft,
      magnitude: String(magnitude)
    });
    setDraft({
      text: JSON.stringify(nextDocument, null, 2),
      origin: `${draft?.origin ?? "session"}:material_property_unit_helper`
    });
    setValidation(null);
    setActionStatus(
      `Material property draft updated: property_kind=${materialPropertyDraft.propertyKind}; ` +
        `dimension=${materialSpec.dimension}; unit=${materialPropertyDraft.unit}; ` +
        "private_user_supplied only."
    );
  }

  function handleApplySectionQuantityDraft() {
    const document = parseDraft();
    if (!document) return;
    const magnitude = Number(sectionQuantityDraft.magnitude);
    if (!Number.isFinite(magnitude)) {
      setActionStatus("SECTION-QUANTITY-DRAFT-INVALID: quantity magnitude must be finite.");
      return;
    }
    const nextDocument = applySectionQuantityDraft(document, {
      ...sectionQuantityDraft,
      magnitude: String(magnitude)
    });
    setDraft({
      text: JSON.stringify(nextDocument, null, 2),
      origin: `${draft?.origin ?? "session"}:section_quantity_unit_helper`
    });
    setValidation(null);
    setActionStatus(
      `Section quantity draft updated: quantity_kind=${sectionQuantityDraft.quantityKind}; ` +
        `dimension=${sectionSpec.dimension}; unit=${sectionQuantityDraft.unit}; ` +
        "private_user_supplied only."
    );
  }

  async function handleRefreshList() {
    setInFlight(true);
    try {
      const route = await listLocalLibraries(projectId);
      if (route.route === "unavailable_browser_preview") {
        setListStatus(route.diagnostic);
        setEntries([]);
        return;
      }
      setEntries(route.entries);
      setListStatus(
        `Local store listed ${route.entries.length} library(ies)` +
          (projectId ? ` for ${projectId}.` : " across all local projects.")
      );
    } catch (error) {
      setEntries([]);
      setListStatus(`LIBRARY-IMPORT-BACKEND-ERROR (list): ${String(error)}`);
    } finally {
      setInFlight(false);
    }
  }

  async function handleValidate() {
    const document = parseDraft();
    if (!document) return;
    onR3JourneyEvent?.("library_validate_requested");
    setInFlight(true);
    try {
      const route = await validateLibraryImport(document, libraryKind, intendedVisibility);
      if (route.route === "unavailable_browser_preview") {
        setActionStatus(route.diagnostic);
        return;
      }
      setValidation(route.validation);
      const { blocking, advisory } = partitionLibraryImportFindings(route.validation);
      setActionStatus(
        `Validation completed: outcome=${route.validation.outcome}; ` +
          `accepted=${String(route.validation.accepted)}; ` +
          `blocking_findings=${blocking.length}; advisory_findings=${advisory.length}.`
      );
    } catch (error) {
      reportBackendError("validate", error);
    } finally {
      setInFlight(false);
    }
  }

  async function handleSave() {
    if (!projectId) {
      setActionStatus(NO_PROJECT_REASON);
      return;
    }
    const document = parseDraft();
    if (!document) return;
    onR3JourneyEvent?.("library_save_requested");
    setInFlight(true);
    try {
      const route = await saveLocalLibrary(projectId, libraryKind, document);
      if (route.route === "unavailable_browser_preview") {
        setActionStatus(route.diagnostic);
        return;
      }
      setValidation(route.result.validation);
      const refusedNote = route.result.stored
        ? ""
        : " Import not stored: validation findings block this library (DEC-036 refuse-to-store).";
      setActionStatus(
        `${route.result.message} (library_id=${route.result.library_id}, ` +
          `stored=${String(route.result.stored)}, storage=${route.result.storage_mode}).${refusedNote}`
      );
      await handleRefreshList();
    } catch (error) {
      reportBackendError("save", error);
    } finally {
      setInFlight(false);
    }
  }

  async function handleOpen(entry: LocalLibraryIndexEntry) {
    setInFlight(true);
    try {
      const route = await openLocalLibrary(entry.project_id, entry.library_kind, entry.library_id);
      if (route.route === "unavailable_browser_preview") {
        setActionStatus(route.diagnostic);
        return;
      }
      if (!route.envelope) {
        setActionStatus(
          `No stored library matched ${entry.library_id}; refresh the list and retry.`
        );
        return;
      }
      setLibraryKind(route.envelope.library_kind);
      setDraft({
        text: JSON.stringify(route.envelope.document, null, 2),
        origin: `opened_from_local_store:${entry.library_id}`
      });
      setValidation(route.envelope.validation);
      setActionStatus(`${route.envelope.message} (library_id=${route.envelope.library_id}).`);
    } catch (error) {
      reportBackendError("open", error);
    } finally {
      setInFlight(false);
    }
  }

  async function handleDelete(entry: LocalLibraryIndexEntry) {
    setInFlight(true);
    try {
      const route = await deleteLocalLibrary(entry.project_id, entry.library_kind, entry.library_id);
      if (route.route === "unavailable_browser_preview") {
        setActionStatus(route.diagnostic);
        return;
      }
      setActionStatus(route.receipt.message);
      await handleRefreshList();
    } catch (error) {
      reportBackendError("delete", error);
    } finally {
      setInFlight(false);
    }
  }

  return (
    <section
      className="panel library-manager-panel"
      aria-label="Library manager"
      data-testid="library-manager-panel"
    >
      <div className="panel-title">
        <Library size={16} aria-hidden="true" />
        Private Library Manager (local-only)
      </div>

      <div className="report-list">
        <small data-testid="library-scope-status">
          {projectId
            ? `Project scope: ${projectId}; storage: local SQLite only.`
            : NO_PROJECT_REASON}
        </small>
      </div>

      <div className="report-list">
        <label htmlFor="library-kind-select">Library kind</label>
        <select
          id="library-kind-select"
          data-testid="library-kind-select"
          value={libraryKind}
          disabled={inFlight}
          title={inFlight ? BUSY_REASON : undefined}
          onChange={(event) => setLibraryKind(event.target.value as LibraryKind)}
        >
          {LIBRARY_KINDS.map((kind) => (
            <option key={kind.value} value={kind.value}>
              {kind.label}
            </option>
          ))}
        </select>
        <label htmlFor="library-visibility-select">Intended visibility (validation preview)</label>
        <select
          id="library-visibility-select"
          data-testid="library-visibility-select"
          value={intendedVisibility}
          disabled={inFlight}
          title={inFlight ? BUSY_REASON : undefined}
          onChange={(event) => setIntendedVisibility(event.target.value as IntendedVisibility)}
        >
          {VISIBILITY_OPTIONS.map((option) => (
            <option key={option.value} value={option.value}>
              {option.label}
            </option>
          ))}
        </select>
        <small data-testid="library-visibility-note">
          Visibility shapes the validation preview only. Save always persists to the private
          local store and stores only an accepted private import (DEC-036).
        </small>
      </div>

      <div className="report-actions">
        <button
          type="button"
          data-testid="library-load-template"
          disabled={inFlight}
          title={inFlight ? BUSY_REASON : undefined}
          onClick={handleLoadTemplate}
        >
          Load invented sample
        </button>
        <button
          type="button"
          data-testid="library-refresh-list"
          disabled={inFlight}
          title={inFlight ? BUSY_REASON : undefined}
          onClick={() => void handleRefreshList()}
        >
          Refresh local list
        </button>
      </div>

      {libraryKind === "material" && draft ? (
        <div className="report-list" data-testid="material-property-unit-helper">
          <strong>Material property unit helper</strong>
          <small data-testid="material-property-unit-boundary">
            Drafts one private material property with explicit unit metadata. This helper writes
            only the import JSON draft; validation and storage still run through the local-only
            import backend.
          </small>
          <label htmlFor="material-property-kind-select">Property kind</label>
          <select
            id="material-property-kind-select"
            data-testid="material-property-kind"
            value={materialPropertyDraft.propertyKind}
            disabled={inFlight}
            onChange={(event) =>
              handleMaterialPropertyKindChange(event.target.value as MaterialPropertyDraft["propertyKind"])
            }
          >
            {Object.entries(MATERIAL_PROPERTY_SPECS).map(([value, spec]) => (
              <option key={value} value={value}>
                {spec.label}
              </option>
            ))}
          </select>
          <label htmlFor="material-property-value-input">Magnitude</label>
          <input
            id="material-property-value-input"
            data-testid="material-property-value"
            type="number"
            value={materialPropertyDraft.magnitude}
            disabled={inFlight}
            onChange={(event) =>
              setMaterialPropertyDraft((current) => ({ ...current, magnitude: event.target.value }))
            }
          />
          <label htmlFor="material-property-unit-select">Unit</label>
          <select
            id="material-property-unit-select"
            data-testid="material-property-unit"
            value={materialPropertyDraft.unitRefId}
            disabled={inFlight}
            onChange={(event) => handleMaterialUnitChange(event.target.value)}
          >
            {materialUnitOptions(unitCatalogRoute, materialSpec, materialPropertyDraft).map((option) => (
              <option key={option.unitRefId} value={option.unitRefId}>
                {option.label}
              </option>
            ))}
          </select>
          <small data-testid="material-property-unit-basis">
            dimension={materialSpec.dimension}; {materialUnitBasis.label}; {materialUnitBasis.detail}
          </small>
          <div className="report-actions">
            <button
              type="button"
              data-testid="material-property-apply-draft"
              disabled={inFlight}
              onClick={handleApplyMaterialPropertyDraft}
            >
              Apply property to draft
            </button>
          </div>
        </div>
      ) : null}

      {libraryKind === "section" && draft ? (
        <div className="report-list" data-testid="section-quantity-unit-helper">
          <strong>Section quantity unit helper</strong>
          <small data-testid="section-quantity-unit-boundary">
            Drafts one private section dimension or property with explicit unit metadata. This
            helper writes only the import JSON draft; validation and storage still run through the
            local-only import backend.
          </small>
          <label htmlFor="section-quantity-kind-select">Quantity kind</label>
          <select
            id="section-quantity-kind-select"
            data-testid="section-quantity-kind"
            value={sectionQuantityDraft.quantityKind}
            disabled={inFlight}
            onChange={(event) =>
              handleSectionQuantityKindChange(event.target.value as SectionQuantityDraft["quantityKind"])
            }
          >
            {Object.entries(SECTION_QUANTITY_SPECS).map(([value, spec]) => (
              <option key={value} value={value}>
                {spec.label}
              </option>
            ))}
          </select>
          <label htmlFor="section-quantity-value-input">Magnitude</label>
          <input
            id="section-quantity-value-input"
            data-testid="section-quantity-value"
            type="number"
            value={sectionQuantityDraft.magnitude}
            disabled={inFlight}
            onChange={(event) =>
              setSectionQuantityDraft((current) => ({ ...current, magnitude: event.target.value }))
            }
          />
          <label htmlFor="section-quantity-unit-select">Unit</label>
          <select
            id="section-quantity-unit-select"
            data-testid="section-quantity-unit"
            value={sectionQuantityDraft.unit}
            disabled={inFlight}
            onChange={(event) =>
              setSectionQuantityDraft((current) => ({ ...current, unit: event.target.value }))
            }
          >
            {sectionUnitOptions(unitCatalogRoute, sectionSpec, sectionQuantityDraft.unit).map((option) => (
              <option key={option.symbol} value={option.symbol}>
                {option.label}
              </option>
            ))}
          </select>
          <small data-testid="section-quantity-unit-basis">
            slot={sectionSpec.slotType}; dimension={sectionSpec.dimension}; {sectionUnitBasis.label};{" "}
            {sectionUnitBasis.detail}
          </small>
          <div className="report-actions">
            <button
              type="button"
              data-testid="section-quantity-apply-draft"
              disabled={inFlight}
              onClick={handleApplySectionQuantityDraft}
            >
              Apply quantity to draft
            </button>
          </div>
        </div>
      ) : null}

      {libraryKind === "component" && draft ? (
        <div className="report-list" data-testid="component-field-unit-helper">
          <strong>Component field unit helper</strong>
          <small data-testid="component-field-unit-boundary">
            Drafts one private component field with explicit unit metadata. This helper writes only
            the import JSON draft; validation and storage still run through the local-only import
            backend.
          </small>
          <label htmlFor="component-field-kind-select">Field kind</label>
          <select
            id="component-field-kind-select"
            data-testid="component-field-kind"
            value={componentFieldDraft.fieldKind}
            disabled={inFlight}
            onChange={(event) => handleComponentFieldKindChange(event.target.value as ComponentFieldDraft["fieldKind"])}
          >
            {Object.entries(COMPONENT_FIELD_SPECS).map(([value, spec]) => (
              <option key={value} value={value}>
                {spec.label}
              </option>
            ))}
          </select>
          <label htmlFor="component-field-value-input">Magnitude</label>
          <input
            id="component-field-value-input"
            data-testid="component-field-value"
            type="number"
            value={componentFieldDraft.magnitude}
            disabled={inFlight}
            onChange={(event) =>
              setComponentFieldDraft((current) => ({ ...current, magnitude: event.target.value }))
            }
          />
          <label htmlFor="component-field-unit-select">Unit</label>
          <select
            id="component-field-unit-select"
            data-testid="component-field-unit"
            value={componentFieldDraft.unit}
            disabled={inFlight}
            onChange={(event) =>
              setComponentFieldDraft((current) => ({ ...current, unit: event.target.value }))
            }
          >
            {componentUnitOptions(
              unitCatalogRoute,
              componentSpec.dimension,
              componentFieldDraft.unit
            ).map((option) => (
              <option key={option.symbol} value={option.symbol}>
                {option.label}
              </option>
            ))}
          </select>
          <small data-testid="component-field-unit-basis">
            dimension={componentSpec.dimension}; {componentUnitBasis.label}; {componentUnitBasis.detail}
          </small>
          <div className="report-actions">
            <button
              type="button"
              data-testid="component-field-apply-draft"
              disabled={inFlight}
              onClick={handleApplyComponentFieldDraft}
            >
              Apply field to draft
            </button>
          </div>
        </div>
      ) : null}

      <div className="report-list" data-testid="library-list">
        <small data-testid="library-list-status">{listStatus}</small>
        {entries.map((entry) => (
          <article
            className="operation-record"
            data-testid={`library-entry-${entry.library_id}`}
            key={`${entry.project_id}:${entry.library_kind}:${entry.library_id}`}
          >
            <strong>
              {entry.library_name} ({entry.library_id})
            </strong>
            <small>
              kind={entry.library_kind}; privacy={entry.privacy_class}; storage={entry.storage_mode}
            </small>
            <div className="report-actions">
              <button
                type="button"
                data-testid={`library-open-${entry.library_id}`}
                disabled={inFlight}
                title={inFlight ? BUSY_REASON : undefined}
                onClick={() => void handleOpen(entry)}
              >
                Open
              </button>
              <button
                type="button"
                data-testid={`library-delete-${entry.library_id}`}
                disabled={inFlight}
                title={inFlight ? BUSY_REASON : undefined}
                onClick={() => void handleDelete(entry)}
              >
                Delete
              </button>
            </div>
          </article>
        ))}
      </div>

      <div className="report-list">
        <label htmlFor="library-draft-json-input">
          Library import document JSON (an already-parsed material/section/component library
          document with provenance — this manager validates and stores it; it does not parse
          external file formats)
        </label>
        <textarea
          id="library-draft-json-input"
          data-testid="library-draft-json"
          rows={14}
          value={draft?.text ?? ""}
          placeholder="No import document. Load an invented sample or paste a library document."
          disabled={!draft || inFlight}
          title={draftReason}
          onChange={(event) =>
            setDraft((current) => (current ? { ...current, text: event.target.value } : current))
          }
        />
        <div className="report-actions">
          <button
            type="button"
            data-testid="library-validate"
            disabled={!draft || inFlight}
            title={draftReason}
            onClick={() => void handleValidate()}
          >
            Validate import
          </button>
          <button
            type="button"
            data-testid="library-save"
            disabled={!draft || inFlight}
            title={draftReason}
            onClick={() => void handleSave()}
          >
            Save to local store
          </button>
          <button
            type="button"
            data-testid="library-discard-draft"
            disabled={!draft || inFlight}
            title={draftReason}
            onClick={handleDiscardDraft}
          >
            Discard import document
          </button>
        </div>
        <small data-testid="library-action-status">{actionStatus}</small>
      </div>

      {validation ? (
        <div className="report-list" data-testid="library-validation">
          <strong>Import validation</strong>
          <small data-testid="library-validation-summary">
            outcome={validation.outcome}; accepted={String(validation.accepted)};
            blocking_findings={String(validation.has_blocking_findings)}; library_kind=
            {validation.library_kind}; intended_visibility={validation.intended_visibility}
          </small>
          <LibraryFindingsList validation={validation} />
          <small>{validation.professional_boundary_notice}</small>
        </div>
      ) : null}

      <small className="report-note" data-testid="library-boundary-note">
        <ShieldAlert size={12} aria-hidden="true" /> Imported private libraries stay in local
        project storage only — never committed to the repository, transmitted, or bundled into
        public artifacts. Only an accepted private import is stored; a suspected-protected or
        otherwise-blocked import is refused, not stored (DEC-036). Import validation reports
        software findings over an already-parsed payload; no protected standards content —
        code-specific data is user-supplied. Acceptance and professional judgment remain with the
        responsible engineer.
      </small>
    </section>
  );
}

// Findings rendered along the PRD §13.5 blocking-vs-advisory axis. `blocking`
// holds blocking/quarantine findings (the import cannot be accepted as-is);
// `advisory` holds review_required findings (human review before acceptance).
function LibraryFindingsList({ validation }: { validation: LibraryImportValidation }) {
  const { blocking, advisory } = partitionLibraryImportFindings(validation);
  return (
    <>
      <div className="operation-record-list" data-testid="library-findings-blocking">
        <small>
          Blocking / quarantine findings ({blocking.length}) — these block import acceptance.
        </small>
        {blocking.map((finding, index) => (
          <LibraryFinding finding={finding} key={`blocking:${finding.code}:${finding.path}:${index}`} />
        ))}
      </div>
      <div className="operation-record-list" data-testid="library-findings-advisory">
        <small>
          Advisory findings ({advisory.length}) — human review before acceptance, not blocking.
        </small>
        {advisory.map((finding, index) => (
          <LibraryFinding finding={finding} key={`advisory:${finding.code}:${finding.path}:${index}`} />
        ))}
      </div>
    </>
  );
}

function LibraryFinding({ finding }: { finding: LibraryImportFinding }) {
  return (
    <article className="operation-record">
      <strong>{finding.code}</strong>
      <small>
        severity={finding.severity}; path={finding.path}
      </small>
      <p>{finding.message}</p>
      <small>Remediation: {finding.remediation}</small>
    </article>
  );
}

function componentUnitOptions(
  route: UnitCatalogRoute | null,
  dimensionId: string,
  fallbackSymbol: string
): { symbol: string; label: string }[] {
  const fallbackBasis = describeUnitBasis(route, fallbackSymbol, dimensionId);
  const fallback = { symbol: fallbackBasis.symbol, label: fallbackBasis.label };
  if (route?.route !== "tauri_unit_catalog") return [fallback];

  const options = acceptedUnits(route.catalog)
    .filter((entry) => unitEntryMatchesDimension(entry, dimensionId))
    .sort((left, right) => Number(right.canonical) - Number(left.canonical) || left.symbol.localeCompare(right.symbol))
    .map((entry) => {
      const basis = describeUnitBasis(route, entry.symbol, dimensionId);
      return { symbol: entry.symbol, label: basis.label };
    });

  if (!options.some((option) => option.symbol === fallback.symbol)) options.unshift(fallback);
  return options.length ? options : [fallback];
}

function materialUnitOptions(
  route: UnitCatalogRoute | null,
  spec: (typeof MATERIAL_PROPERTY_SPECS)[keyof typeof MATERIAL_PROPERTY_SPECS],
  draft: MaterialPropertyDraft
): { unitRefId: string; symbol: string; label: string }[] {
  const fallbackBasis = describeUnitBasis(route, spec.defaultUnit, spec.dimension);
  const fallback = {
    unitRefId: spec.defaultUnitRefId,
    symbol: fallbackBasis.symbol,
    label: fallbackBasis.label
  };
  if (route?.route !== "tauri_unit_catalog") return [fallback];

  const options = acceptedUnits(route.catalog)
    .filter((entry) => unitEntryMatchesDimension(entry, spec.dimension))
    .sort((left, right) => Number(right.canonical) - Number(left.canonical) || left.symbol.localeCompare(right.symbol))
    .map((entry) => {
      const basis = describeUnitBasis(route, entry.symbol, spec.dimension);
      return { unitRefId: entry.unit_id, symbol: entry.symbol, label: basis.label };
    });

  if (!options.some((option) => option.unitRefId === draft.unitRefId)) {
    options.unshift({ unitRefId: draft.unitRefId, symbol: draft.unit, label: `${draft.unit}, stored metadata` });
  }
  if (!options.some((option) => option.unitRefId === fallback.unitRefId)) options.unshift(fallback);
  return options.length ? options : [fallback];
}

function sectionUnitOptions(
  route: UnitCatalogRoute | null,
  spec: (typeof SECTION_QUANTITY_SPECS)[keyof typeof SECTION_QUANTITY_SPECS],
  fallbackSymbol: string
): { symbol: string; label: string }[] {
  const fallbackBasis = describeUnitBasis(route, fallbackSymbol, spec.dimension);
  const fallback = { symbol: fallbackBasis.symbol, label: fallbackBasis.label };
  if (route?.route !== "tauri_unit_catalog") return [fallback];

  const options = acceptedUnits(route.catalog)
    .filter((entry) => unitEntryMatchesDimension(entry, spec.dimension))
    .sort((left, right) => Number(right.canonical) - Number(left.canonical) || left.symbol.localeCompare(right.symbol))
    .map((entry) => {
      const basis = describeUnitBasis(route, entry.symbol, spec.dimension);
      return { symbol: entry.symbol, label: basis.label };
    });

  if (!options.some((option) => option.symbol === fallback.symbol)) options.unshift(fallback);
  return options.length ? options : [fallback];
}

function applySectionQuantityDraft(
  document: Record<string, unknown>,
  draft: SectionQuantityDraft
): Record<string, unknown> {
  const spec = SECTION_QUANTITY_SPECS[draft.quantityKind];
  const provenance = sectionDraftProvenance(document);
  const sectionRecords = Array.isArray(document.section_records) ? [...document.section_records] : [];
  const firstRecord =
    typeof sectionRecords[0] === "object" && sectionRecords[0] !== null && !Array.isArray(sectionRecords[0])
      ? { ...(sectionRecords[0] as Record<string, unknown>) }
      : defaultSectionRecord(provenance);
  const value = {
    magnitude: Number(draft.magnitude),
    unit: draft.unit,
    dimension: spec.dimension,
    value_status: "private_user_supplied",
    provenance
  };

  if (spec.slotType === "dimension") {
    const dimensionId = `dimension:${draft.quantityKind}:user_draft`;
    const dimensions = Array.isArray(firstRecord.dimensions) ? [...firstRecord.dimensions] : [];
    const dimensionSlot = {
      dimension_id: dimensionId,
      dimension_kind: draft.quantityKind,
      value,
      value_status: "private_user_supplied",
      required_for: spec.requiredFor,
      provenance,
      review_status: "pending"
    };
    const existingIndex = dimensions.findIndex(
      (dimension) =>
        typeof dimension === "object" &&
        dimension !== null &&
        !Array.isArray(dimension) &&
        (dimension as Record<string, unknown>).dimension_id === dimensionId
    );
    if (existingIndex >= 0) dimensions[existingIndex] = dimensionSlot;
    else dimensions.push(dimensionSlot);
    firstRecord.dimensions = dimensions;
  } else {
    const propertyId = `property:${draft.quantityKind}:user_draft`;
    const properties = Array.isArray(firstRecord.properties) ? [...firstRecord.properties] : [];
    const propertySlot = {
      property_id: propertyId,
      property_kind: draft.quantityKind,
      value,
      value_status: "private_user_supplied",
      calculation_status: "not_calculated",
      required_for: spec.requiredFor,
      provenance,
      review_status: "pending"
    };
    const existingIndex = properties.findIndex(
      (property) =>
        typeof property === "object" &&
        property !== null &&
        !Array.isArray(property) &&
        (property as Record<string, unknown>).property_id === propertyId
    );
    if (existingIndex >= 0) properties[existingIndex] = propertySlot;
    else properties.push(propertySlot);
    firstRecord.properties = properties;
  }

  sectionRecords[0] = firstRecord;
  return {
    ...document,
    section_records: sectionRecords
  };
}

function applyMaterialPropertyDraft(
  document: Record<string, unknown>,
  draft: MaterialPropertyDraft
): Record<string, unknown> {
  const spec = MATERIAL_PROPERTY_SPECS[draft.propertyKind];
  const provenance = materialDraftProvenance(document);
  const materialRecords = Array.isArray(document.material_records) ? [...document.material_records] : [];
  const firstRecord =
    typeof materialRecords[0] === "object" && materialRecords[0] !== null && !Array.isArray(materialRecords[0])
      ? { ...(materialRecords[0] as Record<string, unknown>) }
      : defaultMaterialRecord(provenance);

  const propertyId = `property:${draft.propertyKind}:user_draft`;
  const properties = Array.isArray(firstRecord.properties) ? [...firstRecord.properties] : [];
  const propertySlot = {
    property_id: propertyId,
    property_kind: draft.propertyKind,
    value_status: "private_user_supplied",
    required_for: spec.requiredFor,
    value: {
      magnitude: Number(draft.magnitude),
      unit_ref: {
        ref_type: "Unit",
        ref_id: draft.unitRefId
      },
      dimension_id: spec.dimension,
      quantity_kind: "unit_bearing",
      unit_required: true,
      missing_unit_behavior: "diagnostic_blocking",
      provenance
    },
    provenance,
    review_status: "pending"
  };
  const existingIndex = properties.findIndex(
    (property) =>
      typeof property === "object" &&
      property !== null &&
      !Array.isArray(property) &&
      (property as Record<string, unknown>).property_id === propertyId
  );
  if (existingIndex >= 0) properties[existingIndex] = propertySlot;
  else properties.push(propertySlot);

  firstRecord.properties = properties;
  materialRecords[0] = firstRecord;
  return {
    ...document,
    material_records: materialRecords
  };
}

function applyComponentFieldDraft(
  document: Record<string, unknown>,
  draft: ComponentFieldDraft
): Record<string, unknown> {
  const spec = COMPONENT_FIELD_SPECS[draft.fieldKind];
  const provenance = componentDraftProvenance(document);
  const componentRecords = Array.isArray(document.component_records)
    ? [...document.component_records]
    : [];
  const firstRecord =
    typeof componentRecords[0] === "object" && componentRecords[0] !== null && !Array.isArray(componentRecords[0])
      ? { ...(componentRecords[0] as Record<string, unknown>) }
      : defaultComponentRecord(provenance);

  const fieldId = `field:${draft.fieldKind}:user_draft`;
  const fields = Array.isArray(firstRecord.fields) ? [...firstRecord.fields] : [];
  const fieldSlot = {
    field_id: fieldId,
    field_kind: draft.fieldKind,
    value_status: "private_user_supplied",
    public_repository_value_policy: "private_user_supplied_only",
    required_for: "library_import",
    value: {
      magnitude: Number(draft.magnitude),
      unit: draft.unit,
      dimension: spec.dimension,
      value_status: "private_user_supplied",
      provenance
    },
    provenance,
    review_status: "pending"
  };
  const existingIndex = fields.findIndex(
    (field) =>
      typeof field === "object" &&
      field !== null &&
      !Array.isArray(field) &&
      (field as Record<string, unknown>).field_id === fieldId
  );
  if (existingIndex >= 0) fields[existingIndex] = fieldSlot;
  else fields.push(fieldSlot);

  firstRecord.fields = fields;
  componentRecords[0] = firstRecord;
  return {
    ...document,
    component_records: componentRecords
  };
}

function sectionDraftProvenance(document: Record<string, unknown>): Record<string, unknown> {
  const library =
    typeof document.section_library === "object" &&
    document.section_library !== null &&
    !Array.isArray(document.section_library)
      ? (document.section_library as Record<string, unknown>)
      : {};
  const provenance =
    typeof library.provenance === "object" &&
    library.provenance !== null &&
    !Array.isArray(library.provenance)
      ? (library.provenance as Record<string, unknown>)
      : {};
  return {
    source_name: String(provenance.source_name ?? "Invented local section quantity draft"),
    source_location: String(provenance.source_location ?? "user-authored private draft"),
    source_license: String(provenance.source_license ?? "private user basis"),
    contributor: String(provenance.contributor ?? "OpenPipeStress user"),
    contributor_certification: String(
      provenance.contributor_certification ?? "invented non-engineering draft; not for project reliance"
    ),
    redistribution_status: "private_only",
    review_status: "pending"
  };
}

function materialDraftProvenance(document: Record<string, unknown>): Record<string, unknown> {
  const library =
    typeof document.material_library === "object" &&
    document.material_library !== null &&
    !Array.isArray(document.material_library)
      ? (document.material_library as Record<string, unknown>)
      : {};
  const provenance =
    typeof library.provenance === "object" &&
    library.provenance !== null &&
    !Array.isArray(library.provenance)
      ? (library.provenance as Record<string, unknown>)
      : {};
  return {
    source_name: String(provenance.source_name ?? "Invented local material property draft"),
    source_location: String(provenance.source_location ?? "user-authored private draft"),
    source_license: String(provenance.source_license ?? "private user basis"),
    contributor: String(provenance.contributor ?? "OpenPipeStress user"),
    contributor_certification: String(
      provenance.contributor_certification ?? "invented non-engineering draft; not for project reliance"
    ),
    redistribution_status: "private_only",
    review_status: "pending"
  };
}

function componentDraftProvenance(document: Record<string, unknown>): Record<string, unknown> {
  const library =
    typeof document.component_library === "object" &&
    document.component_library !== null &&
    !Array.isArray(document.component_library)
      ? (document.component_library as Record<string, unknown>)
      : {};
  const provenance =
    typeof library.provenance === "object" &&
    library.provenance !== null &&
    !Array.isArray(library.provenance)
      ? (library.provenance as Record<string, unknown>)
      : {};
  return {
    source_name: String(provenance.source_name ?? "Invented local component field draft"),
    source_location: String(provenance.source_location ?? "user-authored private draft"),
    source_license: String(provenance.source_license ?? "private user basis"),
    contributor: String(provenance.contributor ?? "OpenPipeStress user"),
    contributor_certification: String(
      provenance.contributor_certification ?? "invented non-engineering draft; not for project reliance"
    ),
    redistribution_status: "private_only",
    review_status: "pending"
  };
}

function defaultSectionRecord(provenance: Record<string, unknown>): Record<string, unknown> {
  return {
    section_id: "section:invented.local_draft",
    name: "Invented private section draft",
    section_type: "pipe",
    privacy_class: "private_user_supplied",
    redistribution_status: "private_only",
    dimensions: [],
    properties: [],
    completeness: [],
    provenance,
    review_status: "pending"
  };
}

function defaultMaterialRecord(provenance: Record<string, unknown>): Record<string, unknown> {
  return {
    material_id: "material:invented.local_draft",
    name: "Invented private material draft",
    material_family: "generic_user_defined",
    privacy_class: "private_user_supplied",
    redistribution_status: "private_only",
    properties: [],
    allowables: [],
    completeness: [],
    provenance,
    review_status: "pending"
  };
}

function defaultComponentRecord(provenance: Record<string, unknown>): Record<string, unknown> {
  return {
    component_id: "component:invented.local_draft",
    name: "Invented private component draft",
    component_type: "other",
    privacy_class: "private_user_supplied",
    redistribution_status: "private_only",
    fields: [],
    completeness: [],
    provenance,
    review_status: "pending"
  };
}
