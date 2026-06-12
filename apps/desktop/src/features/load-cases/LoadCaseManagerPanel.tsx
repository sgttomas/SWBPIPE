import { ListPlus, Scale, SlidersHorizontal, Trash2 } from "lucide-react";
import { useEffect, useMemo, useState } from "react";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import {
  acceptedUnits,
  describeUnitBasis,
  loadUnitCatalog,
  unitEntryMatchesDimension,
  type UnitCatalogRoute
} from "../../services/unitCatalogService";

type LoadCase = PreviewModel["load_cases"][number];
type Combination = NonNullable<PreviewModel["combinations"]>[number];
type CombinationTerm = Combination["terms"][number];
type PrimitiveLoad = Record<string, unknown>;

type PrimitiveLoadView = {
  index: number;
  load: PrimitiveLoad;
  loadCase: LoadCase;
};

type CombinationTermView = {
  index: number;
  term: CombinationTerm;
  combination: Combination;
};

type LoadCaseDraft = {
  id: string;
  label: string;
  kind: string;
  status: string;
  provenance: string;
};

type PrimitiveLoadCategory =
  | "concentrated_force"
  | "distributed_force"
  | "concentrated_moment"
  | "pressure"
  | "thermal"
  | "imposed_displacement";

type PrimitiveLoadDraft = {
  loadCaseId: string;
  id: string;
  category: PrimitiveLoadCategory;
  targetNode: string;
  targetPipe: string;
  targetSupport: string;
  direction: string;
  magnitude: string;
  unit: string;
  provenance: string;
};

type CombinationTermDraft = {
  combinationId: string;
  loadCaseId: string;
  factor: string;
  rationale: string;
};

type CombinationDraft = {
  id: string;
  label: string;
  basis: string;
  loadCaseId: string;
  factor: string;
  provenance: string;
  rationale: string;
};

type UnitOption = {
  symbol: string;
  label: string;
};

type LoadMetadataField = "status" | "kind";

export function LoadCaseManagerPanel({
  model,
  onQueueIntent,
  onSelect,
  selection
}: {
  model: PreviewModel;
  onQueueIntent: (intent: EditorOperationIntent) => void;
  onSelect: (selection: EntityRef) => void;
  selection: EntityRef;
}) {
  const primitiveLoads = useMemo(() => primitiveLoadViews(model), [model]);
  const combinationTerms = useMemo(() => combinationTermViews(model), [model]);
  const [selectedPrimitiveKey, setSelectedPrimitiveKey] = useState(primitiveLoads[0] ? primitiveKey(primitiveLoads[0]) : "");
  const [selectedCombinationTermKey, setSelectedCombinationTermKey] = useState(
    combinationTerms[0] ? combinationTermKey(combinationTerms[0]) : ""
  );
  const selectedPrimitive =
    primitiveLoads.find((primitive) => primitiveKey(primitive) === selectedPrimitiveKey) ?? primitiveLoads[0] ?? null;
  const selectedCombinationTerm =
    combinationTerms.find((term) => combinationTermKey(term) === selectedCombinationTermKey) ?? combinationTerms[0] ?? null;
  const selectedCombination =
    (selection.type === "combination" ? model.combinations?.find((combination) => combination.id === selection.id) : null) ??
    selectedCombinationTerm?.combination ??
    model.combinations?.[0] ??
    null;
  const selectedLoadCase =
    (selection.type === "load" ? model.load_cases.find((loadCase) => loadCase.id === selection.id) : null) ??
    selectedPrimitive?.loadCase ??
    model.load_cases[0] ??
    null;
  const currentMagnitude = selectedPrimitive ? primitiveMagnitudeDisplay(selectedPrimitive.load) : "";
  const currentMagnitudeUnit = selectedPrimitive ? primitiveUnit(selectedPrimitive.load) : "";
  const [proposedMagnitude, setProposedMagnitude] = useState(currentMagnitude);
  const [proposedMagnitudeUnit, setProposedMagnitudeUnit] = useState(currentMagnitudeUnit);
  const [rationale, setRationale] = useState("user_entered_load_case_preview_change");
  const [primitiveDeleteRationale, setPrimitiveDeleteRationale] = useState("user_entered_primitive_load_delete_preview_change");
  const [metadataField, setMetadataField] = useState<LoadMetadataField>("status");
  const currentMetadataValue = selectedLoadCase ? loadCaseMetadataValue(selectedLoadCase, metadataField) : "";
  const [proposedMetadataValue, setProposedMetadataValue] = useState(currentMetadataValue);
  const [metadataRationale, setMetadataRationale] = useState("user_entered_load_case_metadata_preview_change");
  const [loadCaseDeleteRationale, setLoadCaseDeleteRationale] = useState(
    "user_entered_load_case_delete_preview_change"
  );
  const currentCombinationFactor = selectedCombinationTerm ? combinationTermFactorDisplay(selectedCombinationTerm.term) : "";
  const [proposedCombinationFactor, setProposedCombinationFactor] = useState(currentCombinationFactor);
  const [combinationRationale, setCombinationRationale] = useState("user_entered_combination_factor_preview_change");
  const [combinationDeleteRationale, setCombinationDeleteRationale] = useState("user_entered_combination_term_delete_preview_change");
  const currentCombinationBasis = selectedCombination?.basis ?? "";
  const [proposedCombinationBasis, setProposedCombinationBasis] = useState(currentCombinationBasis);
  const [combinationBasisRationale, setCombinationBasisRationale] = useState("user_entered_combination_basis_preview_change");
  const [combinationEntityDeleteRationale, setCombinationEntityDeleteRationale] = useState(
    "user_entered_combination_delete_preview_change"
  );
  const [combinationTermDraft, setCombinationTermDraft] = useState<CombinationTermDraft>(() => defaultCombinationTermDraft(model));
  const nextCombinationId = useMemo(() => nextCombinationIdentifier(model), [model]);
  const [combinationDraft, setCombinationDraft] = useState<CombinationDraft>(() =>
    defaultCombinationDraft(model, nextCombinationId)
  );
  const nextLoadCaseId = useMemo(() => nextLoadCaseIdentifier(model), [model]);
  const [loadCaseDraft, setLoadCaseDraft] = useState<LoadCaseDraft>(() => defaultLoadCaseDraft(nextLoadCaseId));
  const [primitiveLoadDraft, setPrimitiveLoadDraft] = useState<PrimitiveLoadDraft>(() =>
    defaultPrimitiveLoadDraft(model, model.load_cases[0]?.id ?? "")
  );
  const [unitCatalogRoute, setUnitCatalogRoute] = useState<UnitCatalogRoute | null>(null);
  const changed = selectedPrimitive
    ? proposedMagnitude.trim() !== currentMagnitude || proposedMagnitudeUnit.trim() !== currentMagnitudeUnit
    : false;
  const intent =
    selectedPrimitive && changed
      ? buildLoadMagnitudeIntent({
          model,
          primitive: selectedPrimitive,
          proposedMagnitude,
          proposedUnit: proposedMagnitudeUnit,
          rationale
        })
      : null;
  const primitiveDeleteIntent =
    selectedPrimitive && primitiveDeleteRationale.trim()
      ? buildDeletePrimitiveLoadIntent({
          model,
          primitive: selectedPrimitive,
          rationale: primitiveDeleteRationale
        })
      : null;
  const metadataChanged = selectedLoadCase
    ? proposedMetadataValue.trim() !== "" && proposedMetadataValue.trim() !== currentMetadataValue
    : false;
  const metadataIntent =
    selectedLoadCase && metadataChanged
      ? buildLoadMetadataIntent({
          model,
          loadCase: selectedLoadCase,
          field: metadataField,
          proposedValue: proposedMetadataValue,
          rationale: metadataRationale
        })
      : null;
  const loadCaseDeleteIntent =
    selectedLoadCase && loadCaseDeleteRationale.trim()
      ? buildDeleteLoadCaseIntent({
          model,
          loadCase: selectedLoadCase,
          rationale: loadCaseDeleteRationale
        })
      : null;
  const metadataOptions = loadMetadataOptions(model, metadataField, currentMetadataValue);
  const combinationFactorChanged = selectedCombinationTerm
    ? proposedCombinationFactor.trim() !== "" && proposedCombinationFactor.trim() !== currentCombinationFactor
    : false;
  const combinationIntent =
    selectedCombinationTerm && combinationFactorChanged
      ? buildCombinationFactorIntent({
          model,
          termView: selectedCombinationTerm,
          proposedFactor: proposedCombinationFactor,
          rationale: combinationRationale
        })
      : null;
  const combinationDeleteIntent =
    selectedCombinationTerm && combinationDeleteRationale.trim()
      ? buildDeleteCombinationTermIntent({
          model,
          termView: selectedCombinationTerm,
          rationale: combinationDeleteRationale
        })
      : null;
  const combinationBasisChanged = selectedCombination
    ? proposedCombinationBasis.trim() !== "" && proposedCombinationBasis.trim() !== currentCombinationBasis
    : false;
  const combinationBasisIntent =
    selectedCombination && combinationBasisChanged
      ? buildCombinationBasisIntent({
          model,
          combination: selectedCombination,
          proposedBasis: proposedCombinationBasis,
          rationale: combinationBasisRationale
      })
      : null;
  const combinationEntityDeleteIntent =
    selectedCombination && combinationEntityDeleteRationale.trim()
      ? buildDeleteCombinationIntent({
          model,
          combination: selectedCombination,
          rationale: combinationEntityDeleteRationale
        })
      : null;
  const createCombinationTermIntent = isCombinationTermDraftReady(model, combinationTermDraft)
    ? buildCreateCombinationTermIntent({
        model,
        draft: combinationTermDraft
      })
    : null;
  const createCombinationIntent = isCombinationDraftReady(model, combinationDraft)
    ? buildCreateCombinationIntent({
        model,
        draft: combinationDraft
      })
    : null;
  const createLoadCaseIntent = isLoadCaseDraftReady(model, loadCaseDraft)
    ? buildCreateLoadCaseIntent({
        model,
        draft: loadCaseDraft
      })
    : null;
  const createPrimitiveLoadIntent = isPrimitiveLoadDraftReady(model, primitiveLoadDraft)
    ? buildCreatePrimitiveLoadIntent({
        model,
        draft: primitiveLoadDraft
      })
    : null;
  const primitiveDraftDimension = primitiveLoadDraftDimension(primitiveLoadDraft.category, primitiveLoadDraft.direction);
  const primitiveDraftUnit = primitiveLoadDraft.unit;
  const primitiveDraftUnitBasis = describeUnitBasis(unitCatalogRoute, primitiveDraftUnit, primitiveDraftDimension);
  const primitiveDraftUnitOptions = unitOptions(
    unitCatalogRoute,
    primitiveDraftDimension,
    primitiveLoadDefaultUnit(model, primitiveLoadDraft.category, primitiveLoadDraft.direction)
  );
  const selectedPrimitiveUnitOptions = selectedPrimitive
    ? unitOptions(unitCatalogRoute, primitiveDimension(selectedPrimitive.load), currentMagnitudeUnit)
    : [];
  const selectedPrimitiveUnitBasis = selectedPrimitive
    ? describeUnitBasis(unitCatalogRoute, proposedMagnitudeUnit, primitiveDimension(selectedPrimitive.load))
    : null;
  const primitiveDraftTarget = primitiveLoadDraftTargetDisplay(primitiveLoadDraft);

  useEffect(() => {
    if (!primitiveLoads.length) {
      setSelectedPrimitiveKey("");
      return;
    }
    if (!primitiveLoads.some((primitive) => primitiveKey(primitive) === selectedPrimitiveKey)) {
      setSelectedPrimitiveKey(primitiveKey(primitiveLoads[0]));
    }
  }, [primitiveLoads, selectedPrimitiveKey]);

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
          diagnostic: `UNIT-CATALOG-LOAD-FAILED: ${String(error)}`
        });
      });
    return () => {
      active = false;
    };
  }, []);

  useEffect(() => {
    setProposedMagnitude(currentMagnitude);
    setProposedMagnitudeUnit(currentMagnitudeUnit);
    setRationale("user_entered_load_case_preview_change");
    setPrimitiveDeleteRationale("user_entered_primitive_load_delete_preview_change");
  }, [currentMagnitude, currentMagnitudeUnit, selectedPrimitiveKey]);

  useEffect(() => {
    setProposedMetadataValue(currentMetadataValue);
    setMetadataRationale("user_entered_load_case_metadata_preview_change");
    setLoadCaseDeleteRationale("user_entered_load_case_delete_preview_change");
  }, [currentMetadataValue, metadataField, selectedLoadCase?.id]);

  useEffect(() => {
    if (!combinationTerms.length) {
      setSelectedCombinationTermKey("");
      return;
    }
    if (!combinationTerms.some((term) => combinationTermKey(term) === selectedCombinationTermKey)) {
      setSelectedCombinationTermKey(combinationTermKey(combinationTerms[0]));
    }
  }, [combinationTerms, selectedCombinationTermKey]);

  useEffect(() => {
    setProposedCombinationFactor(currentCombinationFactor);
    setCombinationRationale("user_entered_combination_factor_preview_change");
    setCombinationDeleteRationale("user_entered_combination_term_delete_preview_change");
  }, [currentCombinationFactor, selectedCombinationTermKey]);

  useEffect(() => {
    setProposedCombinationBasis(currentCombinationBasis);
    setCombinationBasisRationale("user_entered_combination_basis_preview_change");
    setCombinationEntityDeleteRationale("user_entered_combination_delete_preview_change");
  }, [currentCombinationBasis, selectedCombination?.id]);

  useEffect(() => {
    setCombinationTermDraft((draft) => {
      const combinationId = model.combinations?.some((combination) => combination.id === draft.combinationId)
        ? draft.combinationId
        : selectedCombination?.id ?? model.combinations?.[0]?.id ?? "";
      const loadCaseId = model.load_cases.some((loadCase) => loadCase.id === draft.loadCaseId)
        ? draft.loadCaseId
        : defaultCombinationTermLoadCase(model, combinationId);
      if (combinationId === draft.combinationId && loadCaseId === draft.loadCaseId) return draft;
      return { ...draft, combinationId, loadCaseId };
    });
  }, [model, selectedCombination?.id]);

  useEffect(() => {
    setCombinationDraft((draft) => {
      const id = draft.id.trim() && !model.combinations?.some((combination) => combination.id === draft.id.trim())
        ? draft.id
        : nextCombinationId;
      const loadCaseId = model.load_cases.some((loadCase) => loadCase.id === draft.loadCaseId)
        ? draft.loadCaseId
        : model.load_cases[0]?.id ?? "";
      if (id === draft.id && loadCaseId === draft.loadCaseId) return draft;
      return { ...draft, id, loadCaseId };
    });
  }, [model, nextCombinationId]);

  function updateLoadCaseDraft(field: keyof LoadCaseDraft, value: string) {
    setLoadCaseDraft((draft) => ({ ...draft, [field]: value }));
  }

  function updatePrimitiveLoadDraft(field: keyof PrimitiveLoadDraft, value: string) {
    setPrimitiveLoadDraft((draft) => {
      const nextDraft = { ...draft, [field]: value } as PrimitiveLoadDraft;
      if (field === "category") nextDraft.category = primitiveLoadCategoryFromValue(value);
      if (field === "loadCaseId" || field === "category") {
        nextDraft.direction = defaultPrimitiveLoadDirection(nextDraft.category);
        nextDraft.id = nextPrimitiveLoadIdentifier(model, nextDraft.loadCaseId, nextDraft.category);
      }
      if (field === "category" || field === "direction") {
        nextDraft.unit = primitiveLoadDefaultUnit(model, nextDraft.category, nextDraft.direction);
      }
      return nextDraft;
    });
  }

  function updateCombinationTermDraft(field: keyof CombinationTermDraft, value: string) {
    setCombinationTermDraft((draft) => {
      const nextDraft = { ...draft, [field]: value };
      if (field === "combinationId") {
        nextDraft.loadCaseId = defaultCombinationTermLoadCase(model, value);
      }
      return nextDraft;
    });
  }

  function updateCombinationDraft(field: keyof CombinationDraft, value: string) {
    setCombinationDraft((draft) => ({ ...draft, [field]: value }));
  }

  function handleSelectPrimitive(primitive: PrimitiveLoadView) {
    setSelectedPrimitiveKey(primitiveKey(primitive));
    onSelect({ type: "load", id: primitive.loadCase.id });
  }

  function handleSelectLoadCase(loadCase: LoadCase) {
    onSelect({ type: "load", id: loadCase.id });
    const firstPrimitive = primitiveLoads.find((primitive) => primitive.loadCase.id === loadCase.id);
    if (firstPrimitive) setSelectedPrimitiveKey(primitiveKey(firstPrimitive));
  }

  function handleSelectCombinationTerm(termView: CombinationTermView) {
    setSelectedCombinationTermKey(combinationTermKey(termView));
    onSelect({ type: "combination", id: termView.combination.id });
  }

  function handleSelectCombination(combination: Combination) {
    onSelect({ type: "combination", id: combination.id });
    setCombinationTermDraft((draft) => ({
      ...draft,
      combinationId: combination.id,
      loadCaseId: defaultCombinationTermLoadCase(model, combination.id)
    }));
  }

  return (
    <section className="panel load-case-manager-panel" aria-label="Load case manager" data-testid="load-case-manager">
      <div className="panel-title">
        <Scale size={16} aria-hidden="true" />
        Load Cases
      </div>
      <p className="muted" data-testid="load-case-manager-summary">
        {model.load_cases.length} load cases; {primitiveLoads.length} primitive loads; {model.combinations?.length ?? 0} combinations;
        unit posture=single_unit_system_preview; saved_project_mutated=false
      </p>

      <section className="load-case-create-editor" aria-label="Create load case">
        <div className="load-editor-heading" data-testid="load-manager-create-load-case-heading">
          <ListPlus size={14} aria-hidden="true" />
          <strong>{loadCaseDraft.id}</strong>
          <span>new empty load case; primitives=0; unit=none</span>
        </div>
        <div className="load-case-create-controls">
          <label>
            <span>ID</span>
            <input
              aria-label="New load case id"
              data-testid="load-manager-create-load-id"
              onChange={(event) => updateLoadCaseDraft("id", event.target.value)}
              value={loadCaseDraft.id}
            />
          </label>
          <label>
            <span>Label</span>
            <input
              aria-label="New load case label"
              data-testid="load-manager-create-load-label"
              onChange={(event) => updateLoadCaseDraft("label", event.target.value)}
              value={loadCaseDraft.label}
            />
          </label>
          <label>
            <span>Kind</span>
            <input
              aria-label="New load case kind"
              data-testid="load-manager-create-load-kind"
              onChange={(event) => updateLoadCaseDraft("kind", event.target.value)}
              value={loadCaseDraft.kind}
            />
          </label>
          <label>
            <span>Status</span>
            <input
              aria-label="New load case status"
              data-testid="load-manager-create-load-status"
              onChange={(event) => updateLoadCaseDraft("status", event.target.value)}
              value={loadCaseDraft.status}
            />
          </label>
          <label>
            <span>Provenance</span>
            <input
              aria-label="New load case provenance"
              data-testid="load-manager-create-load-provenance"
              onChange={(event) => updateLoadCaseDraft("provenance", event.target.value)}
              value={loadCaseDraft.provenance}
            />
          </label>
          <button
            data-testid="queue-create-load-case-intent"
            disabled={!createLoadCaseIntent}
            onClick={() => createLoadCaseIntent && onQueueIntent(createLoadCaseIntent)}
            title="Queue load-case creation operation"
            type="button"
          >
            <ListPlus size={14} aria-hidden="true" />
            Queue case
          </button>
        </div>
        <p className="muted load-edit-preview" data-testid="load-manager-create-load-preview">
          {createLoadCaseIntent
            ? `${createLoadCaseIntent.operation_id}; before=${createLoadCaseIntent.change.before}; after=${loadCaseDraft.id}; unit=${createLoadCaseIntent.change.unit}; ${createLoadCaseIntent.change.dimension}; primitive_loads=0; direct_model_mutation_allowed=false; professional_approval=false`
            : loadCaseDraft.id.trim() && model.load_cases.some((loadCase) => loadCase.id === loadCaseDraft.id.trim())
              ? `id=${loadCaseDraft.id.trim()} already exists; no load case queued`
              : "complete id/label/kind/status/provenance to queue an empty load case"}
        </p>
      </section>

      <section className="primitive-load-create-editor" aria-label="Create primitive load">
        <div className="load-editor-heading" data-testid="load-manager-create-primitive-heading">
          <ListPlus size={14} aria-hidden="true" />
          <strong>{primitiveLoadDraft.id}</strong>
          <span>
            {primitiveLoadDraft.category}; target={primitiveDraftTarget}; unit={primitiveDraftUnit}
          </span>
        </div>
        <div className="primitive-load-create-controls">
          <label>
            <span>Type</span>
            <select
              aria-label="Primitive load category"
              data-testid="load-manager-create-primitive-category"
              onChange={(event) => updatePrimitiveLoadDraft("category", event.target.value)}
              value={primitiveLoadDraft.category}
            >
              <option value="concentrated_force">concentrated_force</option>
              <option value="distributed_force">distributed_force</option>
              <option value="concentrated_moment">concentrated_moment</option>
              <option value="pressure">pressure</option>
              <option value="thermal">thermal</option>
              <option value="imposed_displacement">imposed_displacement</option>
            </select>
          </label>
          <label>
            <span>Case</span>
            <select
              aria-label="Primitive load case"
              data-testid="load-manager-create-primitive-load-case"
              onChange={(event) => updatePrimitiveLoadDraft("loadCaseId", event.target.value)}
              value={primitiveLoadDraft.loadCaseId}
            >
              {model.load_cases.map((loadCase) => (
                <option key={loadCase.id} value={loadCase.id}>
                  {loadCase.id}
                </option>
              ))}
            </select>
          </label>
          <label>
            <span>ID</span>
            <input
              aria-label="New primitive load id"
              data-testid="load-manager-create-primitive-id"
              onChange={(event) => updatePrimitiveLoadDraft("id", event.target.value)}
              value={primitiveLoadDraft.id}
            />
          </label>
          {primitiveLoadUsesSupportTarget(primitiveLoadDraft.category) ? (
            <label>
              <span>Support</span>
              <select
                aria-label="Primitive support target"
                data-testid="load-manager-create-primitive-support"
                onChange={(event) => updatePrimitiveLoadDraft("targetSupport", event.target.value)}
                value={primitiveLoadDraft.targetSupport}
              >
                {model.supports.map((support) => (
                  <option key={support.id} value={support.id}>
                    {support.id}
                  </option>
                ))}
              </select>
            </label>
          ) : primitiveLoadUsesPipeTarget(primitiveLoadDraft.category) ? (
            <label>
              <span>Pipe</span>
              <select
                aria-label="Primitive element target pipe"
                data-testid="load-manager-create-primitive-pipe"
                onChange={(event) => updatePrimitiveLoadDraft("targetPipe", event.target.value)}
                value={primitiveLoadDraft.targetPipe}
              >
                {model.pipe_segments.map((pipe) => (
                  <option key={pipe.id} value={pipe.id}>
                    {pipe.id}
                  </option>
                ))}
              </select>
            </label>
          ) : (
            <label>
              <span>Node</span>
              <select
                aria-label="Concentrated force target node"
                data-testid="load-manager-create-primitive-node"
                onChange={(event) => updatePrimitiveLoadDraft("targetNode", event.target.value)}
                value={primitiveLoadDraft.targetNode}
              >
                {model.nodes.map((node) => (
                  <option key={node.id} value={node.id}>
                    {node.id}
                  </option>
                ))}
              </select>
            </label>
          )}
          <label>
            <span>Dir</span>
            <select
              aria-label="Primitive load direction"
              data-testid="load-manager-create-primitive-direction"
              onChange={(event) => updatePrimitiveLoadDraft("direction", event.target.value)}
              value={primitiveLoadDraft.direction}
            >
              {primitiveLoadDirectionOptions(primitiveLoadDraft.category).map((direction) => (
                <option key={direction} value={direction}>
                  {direction}
                </option>
              ))}
            </select>
          </label>
          <label>
            <span>Magnitude unit</span>
            <select
              aria-label="Primitive load unit"
              data-testid="load-manager-create-primitive-unit"
              onChange={(event) => updatePrimitiveLoadDraft("unit", event.target.value)}
              value={primitiveLoadDraft.unit}
            >
              {primitiveDraftUnitOptions.map((option) => (
                <option key={option.symbol} value={option.symbol}>
                  {option.label}
                </option>
              ))}
            </select>
          </label>
          <label>
            <span>Magnitude ({primitiveDraftUnitBasis.label})</span>
            <input
              aria-label="Primitive load magnitude"
              data-testid="load-manager-create-primitive-magnitude"
              onChange={(event) => updatePrimitiveLoadDraft("magnitude", event.target.value)}
              value={primitiveLoadDraft.magnitude}
            />
          </label>
          <label>
            <span>Provenance</span>
            <input
              aria-label="Primitive load provenance"
              data-testid="load-manager-create-primitive-provenance"
              onChange={(event) => updatePrimitiveLoadDraft("provenance", event.target.value)}
              value={primitiveLoadDraft.provenance}
            />
          </label>
          <button
            data-testid="queue-create-primitive-intent"
            disabled={!createPrimitiveLoadIntent}
            onClick={() => createPrimitiveLoadIntent && onQueueIntent(createPrimitiveLoadIntent)}
            title="Queue primitive load operation"
            type="button"
          >
            <ListPlus size={14} aria-hidden="true" />
            Queue load
          </button>
        </div>
        <p className="muted load-edit-preview" data-testid="load-manager-create-primitive-preview">
          {createPrimitiveLoadIntent
            ? `${createPrimitiveLoadIntent.operation_id}; before=${createPrimitiveLoadIntent.change.before}; after=${primitiveLoadDraft.id}; target=${primitiveDraftTarget}; direction=${primitiveLoadDraft.direction}; unit=${createPrimitiveLoadIntent.change.unit}; ${createPrimitiveLoadIntent.change.dimension}; direct_model_mutation_allowed=false; professional_approval=false`
            : primitiveLoadDraft.id.trim() && primitiveLoadExists(model, primitiveLoadDraft.id.trim())
              ? `id=${primitiveLoadDraft.id.trim()} already exists; no primitive load queued`
              : "complete case/id/target/direction/nonzero magnitude/provenance to queue a primitive load"}
          </p>
      </section>

      <section className="combination-create-editor" aria-label="Create combination">
        <div className="load-editor-heading" data-testid="load-manager-create-combination-heading">
          <ListPlus size={14} aria-hidden="true" />
          <strong>{combinationDraft.id}</strong>
          <span>
            basis={combinationDraft.basis}; initial={combinationDraft.loadCaseId} x {combinationDraft.factor}
          </span>
        </div>
        <div className="combination-create-controls">
          <label>
            <span>ID</span>
            <input
              aria-label="New combination id"
              data-testid="load-manager-create-combination-id"
              onChange={(event) => updateCombinationDraft("id", event.target.value)}
              value={combinationDraft.id}
            />
          </label>
          <label>
            <span>Label</span>
            <input
              aria-label="New combination label"
              data-testid="load-manager-create-combination-label"
              onChange={(event) => updateCombinationDraft("label", event.target.value)}
              value={combinationDraft.label}
            />
          </label>
          <label>
            <span>Basis</span>
            <select
              aria-label="New combination basis"
              data-testid="load-manager-create-combination-basis"
              onChange={(event) => updateCombinationDraft("basis", event.target.value)}
              value={combinationDraft.basis}
            >
              <option value="mechanics">mechanics</option>
            </select>
          </label>
          <label>
            <span>Load</span>
            <select
              aria-label="New combination initial load case"
              data-testid="load-manager-create-combination-load-case"
              onChange={(event) => updateCombinationDraft("loadCaseId", event.target.value)}
              value={combinationDraft.loadCaseId}
            >
              {model.load_cases.map((loadCase) => (
                <option key={loadCase.id} value={loadCase.id}>
                  {loadCase.id}
                </option>
              ))}
            </select>
          </label>
          <label>
            <span>Factor</span>
            <input
              aria-label="New combination factor"
              data-testid="load-manager-create-combination-factor"
              onChange={(event) => updateCombinationDraft("factor", event.target.value)}
              value={combinationDraft.factor}
            />
          </label>
          <label>
            <span>Provenance</span>
            <input
              aria-label="New combination provenance"
              data-testid="load-manager-create-combination-provenance"
              onChange={(event) => updateCombinationDraft("provenance", event.target.value)}
              value={combinationDraft.provenance}
            />
          </label>
          <label>
            <span>Rationale</span>
            <input
              aria-label="New combination rationale"
              data-testid="load-manager-create-combination-rationale"
              onChange={(event) => updateCombinationDraft("rationale", event.target.value)}
              value={combinationDraft.rationale}
            />
          </label>
          <button
            data-testid="queue-create-combination-intent"
            disabled={!createCombinationIntent}
            onClick={() => createCombinationIntent && onQueueIntent(createCombinationIntent)}
            title="Queue combination creation operation"
            type="button"
          >
            <ListPlus size={14} aria-hidden="true" />
            Queue combo
          </button>
        </div>
        <p className="muted load-edit-preview" data-testid="load-manager-create-combination-preview">
          {createCombinationIntent
            ? `${createCombinationIntent.operation_id}; before=${createCombinationIntent.change.before}; after=${combinationDraft.id}; term=${combinationDraft.loadCaseId} x ${combinationDraft.factor}; unit=${createCombinationIntent.change.unit}; ${createCombinationIntent.change.dimension}; direct_model_mutation_allowed=false; professional_approval=false`
            : combinationDraft.id.trim() &&
                model.combinations?.some((combination) => combination.id === combinationDraft.id.trim())
              ? `id=${combinationDraft.id.trim()} already exists; no combination queued`
              : "complete id/label/load case/finite factor/provenance/rationale to queue a mechanics combination"}
        </p>
      </section>

      <div className="load-case-list" data-testid="load-case-manager-cases">
        {model.load_cases.map((loadCase) => (
          <button
            className={selection.type === "load" && selection.id === loadCase.id ? "load-case-row active" : "load-case-row"}
            data-testid={`load-manager-case-${loadCase.id}`}
            key={loadCase.id}
            onClick={() => handleSelectLoadCase(loadCase)}
            type="button"
          >
            <strong>{loadCase.label}</strong>
            <small>
              {loadCase.id}; {loadCase.kind}; {loadCase.status}; primitives={loadCase.primitive_loads?.length ?? 0}
            </small>
            <small>{loadCase.provenance}</small>
          </button>
        ))}
      </div>

      {selectedLoadCase ? (
        <section className="load-metadata-editor" aria-label="Load case metadata editor">
          <div className="load-editor-heading" data-testid="load-manager-selected-case">
            <SlidersHorizontal size={14} aria-hidden="true" />
            <strong>{selectedLoadCase.id}</strong>
            <span>
              field={metadataField}; current={currentMetadataValue}; path={metadataField}
            </span>
          </div>
          <div className="load-metadata-controls">
            <label>
              <span>Field</span>
              <select
                aria-label="Load case metadata field"
                data-testid="load-manager-metadata-field"
                onChange={(event) => setMetadataField(event.target.value as LoadMetadataField)}
                value={metadataField}
              >
                <option value="status">Status</option>
                <option value="kind">Kind</option>
              </select>
            </label>
            <label>
              <span>Value</span>
              <select
                aria-label="Load case metadata value"
                data-testid="load-manager-metadata-value"
                onChange={(event) => setProposedMetadataValue(event.target.value)}
                value={proposedMetadataValue}
              >
                {metadataOptions.map((value) => (
                  <option key={value} value={value}>
                    {value}
                  </option>
                ))}
              </select>
            </label>
            <label>
              <span>Rationale</span>
              <input
                aria-label="Load case metadata rationale"
                data-testid="load-manager-metadata-rationale"
                onChange={(event) => setMetadataRationale(event.target.value)}
                value={metadataRationale}
              />
            </label>
            <button
              data-testid="queue-load-metadata-intent"
              disabled={!metadataIntent}
              onClick={() => metadataIntent && onQueueIntent(metadataIntent)}
              title="Queue load-case metadata operation"
              type="button"
            >
              <ListPlus size={14} aria-hidden="true" />
              Queue metadata
            </button>
          </div>
          <p className="muted load-edit-preview" data-testid="load-manager-metadata-preview">
            {metadataIntent
              ? `${metadataIntent.operation_id}; before=${metadataIntent.change.before}; after=${metadataIntent.change.after}; unit=${metadataIntent.change.unit}; ${metadataIntent.change.dimension}; direct_model_mutation_allowed=false; professional_approval=false`
              : `current=${currentMetadataValue}; no changed ${metadataField} queued`}
          </p>
          <div className="load-case-delete-controls">
            <label>
              <span>Delete rationale</span>
              <input
                aria-label="Load case delete rationale"
                data-testid="load-manager-load-case-delete-rationale"
                onChange={(event) => setLoadCaseDeleteRationale(event.target.value)}
                value={loadCaseDeleteRationale}
              />
            </label>
            <button
              data-testid="queue-delete-load-case-intent"
              disabled={!loadCaseDeleteIntent}
              onClick={() => loadCaseDeleteIntent && onQueueIntent(loadCaseDeleteIntent)}
              title="Queue load-case delete operation"
              type="button"
            >
              <Trash2 size={14} aria-hidden="true" />
              Queue delete case
            </button>
          </div>
          <p className="muted load-edit-preview" data-testid="load-manager-load-case-delete-preview">
            {loadCaseDeleteIntent
              ? `${loadCaseDeleteIntent.operation_id}; before=${loadCaseDeleteIntent.change.before}; after=${loadCaseDeleteIntent.change.after}; unit=${loadCaseDeleteIntent.change.unit}; ${loadCaseDeleteIntent.change.dimension}; direct_model_mutation_allowed=false; professional_approval=false`
              : "select a load case and provide rationale to queue deletion"}
          </p>
        </section>
      ) : null}

      <div className="load-primitive-list" data-testid="load-case-manager-primitives">
        {primitiveLoads.map((primitive) => (
          <button
            className={selectedPrimitive && primitiveKey(primitive) === primitiveKey(selectedPrimitive) ? "primitive-load-row active" : "primitive-load-row"}
            data-testid={`load-manager-primitive-${primitiveId(primitive.load)}`}
            key={primitiveKey(primitive)}
            onClick={() => handleSelectPrimitive(primitive)}
            type="button"
          >
            <strong>
              {primitiveCategory(primitive.load)}; {primitiveMagnitudeDisplay(primitive.load)} {primitiveUnit(primitive.load)}
            </strong>
            <small>
              {primitiveId(primitive.load)}; {primitiveTarget(primitive.load)}; {primitiveDirection(primitive.load)};
              dimension={primitiveDimension(primitive.load)}
            </small>
          </button>
        ))}
      </div>

      {selectedPrimitive ? (
        <section className="load-magnitude-editor" aria-label="Primitive load magnitude editor">
          <div className="load-editor-heading" data-testid="load-manager-selected-primitive">
            <SlidersHorizontal size={14} aria-hidden="true" />
            <strong>{primitiveId(selectedPrimitive.load)}</strong>
            <span>
              {selectedPrimitive.loadCase.id}; path=primitive_loads.{selectedPrimitive.index}.magnitude.value
            </span>
          </div>
          <div className="load-editor-controls">
            <label>
              <span>Magnitude unit</span>
              <select
                aria-label="Selected primitive load unit"
                data-testid="load-manager-magnitude-unit"
                onChange={(event) => setProposedMagnitudeUnit(event.target.value)}
                value={proposedMagnitudeUnit}
              >
                {selectedPrimitiveUnitOptions.map((option) => (
                  <option key={option.symbol} value={option.symbol}>
                    {option.label}
                  </option>
                ))}
              </select>
            </label>
            <label>
              <span>Magnitude ({selectedPrimitiveUnitBasis?.label ?? currentMagnitudeUnit})</span>
              <input
                aria-label="Primitive load magnitude"
                data-testid="load-manager-magnitude-value"
                onChange={(event) => setProposedMagnitude(event.target.value)}
                value={proposedMagnitude}
              />
            </label>
            <label>
              <span>Rationale</span>
              <input
                aria-label="Load case edit rationale"
                data-testid="load-manager-rationale"
                onChange={(event) => setRationale(event.target.value)}
                value={rationale}
              />
            </label>
            <button
              data-testid="queue-load-magnitude-intent"
              disabled={!intent}
              onClick={() => intent && onQueueIntent(intent)}
              title="Queue load-magnitude operation"
              type="button"
            >
              <ListPlus size={14} aria-hidden="true" />
              Queue magnitude
            </button>
          </div>
          <p className="muted load-edit-preview" data-testid="load-manager-edit-preview">
            {intent
              ? `${intent.operation_id}; before=${intent.change.before}; after=${intent.change.after}; unit=${intent.change.unit}; ${intent.change.dimension}; direct_model_mutation_allowed=false; professional_approval=false`
              : `current=${currentMagnitude} ${primitiveUnit(selectedPrimitive.load)}; no changed magnitude queued`}
          </p>
          <div className="primitive-load-delete-controls">
            <label>
              <span>Delete rationale</span>
              <input
                aria-label="Primitive load delete rationale"
                data-testid="load-manager-primitive-delete-rationale"
                onChange={(event) => setPrimitiveDeleteRationale(event.target.value)}
                value={primitiveDeleteRationale}
              />
            </label>
            <button
              data-testid="queue-delete-primitive-load-intent"
              disabled={!primitiveDeleteIntent}
              onClick={() => primitiveDeleteIntent && onQueueIntent(primitiveDeleteIntent)}
              title="Queue primitive-load delete operation"
              type="button"
            >
              <Trash2 size={14} aria-hidden="true" />
              Queue delete
            </button>
          </div>
          <p className="muted load-edit-preview" data-testid="load-manager-primitive-delete-preview">
            {primitiveDeleteIntent
              ? `${primitiveDeleteIntent.operation_id}; before=${primitiveDeleteIntent.change.before}; after=${primitiveDeleteIntent.change.after}; unit=${primitiveDeleteIntent.change.unit}; ${primitiveDeleteIntent.change.dimension}; direct_model_mutation_allowed=false; professional_approval=false`
              : "select a primitive load and provide rationale to queue deletion"}
          </p>
        </section>
      ) : (
        <p className="muted" data-testid="load-case-manager-empty">
          No primitive loads are available in the current model.
        </p>
      )}

      {model.combinations?.length ? (
        <div className="load-combination-list" data-testid="load-case-manager-combinations">
          {model.combinations.map((combination) => {
            const terms = combinationTerms.filter((term) => term.combination.id === combination.id);
            return (
              <article className="load-combination-row" data-testid={`load-manager-combination-${combination.id}`} key={combination.id}>
                <button
                  className={
                    selectedCombination?.id === combination.id ? "combination-summary-row active" : "combination-summary-row"
                  }
                  data-testid={`load-manager-combination-select-${combination.id}`}
                  onClick={() => handleSelectCombination(combination)}
                  type="button"
                >
                  <strong>{combination.label}</strong>
                  <small>
                    {combination.id}; basis={combination.basis}; terms=
                    {combination.terms.map((term) => `${term.load_case} x ${term.factor}`).join("; ")}
                  </small>
                </button>
                {terms.length ? (
                  <div className="combination-term-list">
                    {terms.map((term) => (
                      <button
                        className={
                          selectedCombinationTerm && combinationTermKey(term) === combinationTermKey(selectedCombinationTerm)
                            ? "combination-term-row active"
                            : "combination-term-row"
                        }
                        data-testid={`load-manager-combination-term-${safeToken(term.combination.id)}-${term.index}`}
                        key={combinationTermKey(term)}
                        onClick={() => handleSelectCombinationTerm(term)}
                        type="button"
                      >
                        {combinationTermLoadCase(term.term)} x {combinationTermFactorDisplay(term.term)}
                      </button>
                    ))}
                  </div>
                ) : null}
              </article>
            );
          })}
        </div>
      ) : null}

      {model.combinations?.length && model.load_cases.length ? (
        <section className="combination-term-create-editor" aria-label="Create combination term">
          <div className="load-editor-heading" data-testid="load-manager-create-combination-term-heading">
            <ListPlus size={14} aria-hidden="true" />
            <strong>{combinationTermDraft.combinationId}</strong>
            <span>
              new term; load={combinationTermDraft.loadCaseId}; factor={combinationTermDraft.factor}
            </span>
          </div>
          <div className="combination-term-create-controls">
            <label>
              <span>Combination</span>
              <select
                aria-label="Combination term target combination"
                data-testid="load-manager-create-combination-term-combination"
                onChange={(event) => updateCombinationTermDraft("combinationId", event.target.value)}
                value={combinationTermDraft.combinationId}
              >
                {(model.combinations ?? []).map((combination) => (
                  <option key={combination.id} value={combination.id}>
                    {combination.id}
                  </option>
                ))}
              </select>
            </label>
            <label>
              <span>Load</span>
              <select
                aria-label="Combination term load case"
                data-testid="load-manager-create-combination-term-load-case"
                onChange={(event) => updateCombinationTermDraft("loadCaseId", event.target.value)}
                value={combinationTermDraft.loadCaseId}
              >
                {model.load_cases.map((loadCase) => (
                  <option key={loadCase.id} value={loadCase.id}>
                    {loadCase.id}
                  </option>
                ))}
              </select>
            </label>
            <label>
              <span>Factor</span>
              <input
                aria-label="Combination term factor"
                data-testid="load-manager-create-combination-term-factor"
                onChange={(event) => updateCombinationTermDraft("factor", event.target.value)}
                value={combinationTermDraft.factor}
              />
            </label>
            <label>
              <span>Rationale</span>
              <input
                aria-label="Combination term creation rationale"
                data-testid="load-manager-create-combination-term-rationale"
                onChange={(event) => updateCombinationTermDraft("rationale", event.target.value)}
                value={combinationTermDraft.rationale}
              />
            </label>
            <button
              data-testid="queue-create-combination-term-intent"
              disabled={!createCombinationTermIntent}
              onClick={() => createCombinationTermIntent && onQueueIntent(createCombinationTermIntent)}
              title="Queue combination-term creation operation"
              type="button"
            >
              <ListPlus size={14} aria-hidden="true" />
              Queue term
            </button>
          </div>
          <p className="muted load-edit-preview" data-testid="load-manager-create-combination-term-preview">
            {createCombinationTermIntent
              ? `${createCombinationTermIntent.operation_id}; before=${createCombinationTermIntent.change.before}; after=${combinationTermDraft.loadCaseId} x ${combinationTermDraft.factor}; unit=${createCombinationTermIntent.change.unit}; ${createCombinationTermIntent.change.dimension}; direct_model_mutation_allowed=false; professional_approval=false`
              : "select combination/load case and finite factor to queue a combination term"}
          </p>
        </section>
      ) : null}

      {selectedCombination ? (
        <section className="combination-basis-editor" aria-label="Combination basis editor">
          <div className="load-editor-heading" data-testid="load-manager-selected-combination">
            <SlidersHorizontal size={14} aria-hidden="true" />
            <strong>{selectedCombination.id}</strong>
            <span>field=basis; current={currentCombinationBasis}; path=basis</span>
          </div>
          <div className="combination-basis-controls">
            <label>
              <span>Basis</span>
              <input
                aria-label="Combination basis"
                data-testid="load-manager-combination-basis-value"
                onChange={(event) => setProposedCombinationBasis(event.target.value)}
                value={proposedCombinationBasis}
              />
            </label>
            <label>
              <span>Rationale</span>
              <input
                aria-label="Combination basis rationale"
                data-testid="load-manager-combination-basis-rationale"
                onChange={(event) => setCombinationBasisRationale(event.target.value)}
                value={combinationBasisRationale}
              />
            </label>
            <button
              data-testid="queue-combination-basis-intent"
              disabled={!combinationBasisIntent}
              onClick={() => combinationBasisIntent && onQueueIntent(combinationBasisIntent)}
              title="Queue combination-basis operation"
              type="button"
            >
              <ListPlus size={14} aria-hidden="true" />
              Queue basis
            </button>
          </div>
          <p className="muted load-edit-preview" data-testid="load-manager-combination-basis-preview">
            {combinationBasisIntent
              ? `${combinationBasisIntent.operation_id}; before=${combinationBasisIntent.change.before}; after=${combinationBasisIntent.change.after}; unit=${combinationBasisIntent.change.unit}; ${combinationBasisIntent.change.dimension}; direct_model_mutation_allowed=false; professional_approval=false`
              : `current=${currentCombinationBasis}; no changed combination basis queued`}
          </p>
          <div className="combination-delete-controls">
            <label>
              <span>Delete rationale</span>
              <input
                aria-label="Combination delete rationale"
                data-testid="load-manager-combination-entity-delete-rationale"
                onChange={(event) => setCombinationEntityDeleteRationale(event.target.value)}
                value={combinationEntityDeleteRationale}
              />
            </label>
            <button
              data-testid="queue-delete-combination-intent"
              disabled={!combinationEntityDeleteIntent}
              onClick={() => combinationEntityDeleteIntent && onQueueIntent(combinationEntityDeleteIntent)}
              title="Queue combination delete operation"
              type="button"
            >
              <Trash2 size={14} aria-hidden="true" />
              Queue delete combo
            </button>
          </div>
          <p className="muted load-edit-preview" data-testid="load-manager-combination-entity-delete-preview">
            {combinationEntityDeleteIntent
              ? `${combinationEntityDeleteIntent.operation_id}; before=${combinationEntityDeleteIntent.change.before}; after=${combinationEntityDeleteIntent.change.after}; unit=${combinationEntityDeleteIntent.change.unit}; ${combinationEntityDeleteIntent.change.dimension}; direct_model_mutation_allowed=false; professional_approval=false`
              : "select a combination and provide rationale to queue deletion"}
          </p>
        </section>
      ) : null}

      {selectedCombinationTerm ? (
        <section className="combination-factor-editor" aria-label="Combination term factor editor">
          <div className="load-editor-heading" data-testid="load-manager-selected-combination-term">
            <SlidersHorizontal size={14} aria-hidden="true" />
            <strong>{selectedCombinationTerm.combination.id}</strong>
            <span>
              term={combinationTermLoadCase(selectedCombinationTerm.term)}; path=terms.{selectedCombinationTerm.index}.factor
            </span>
          </div>
          <div className="combination-factor-controls">
            <label>
              <span>Factor</span>
              <input
                aria-label="Combination term factor"
                data-testid="load-manager-combination-factor-value"
                onChange={(event) => setProposedCombinationFactor(event.target.value)}
                value={proposedCombinationFactor}
              />
            </label>
            <label>
              <span>Rationale</span>
              <input
                aria-label="Combination term rationale"
                data-testid="load-manager-combination-factor-rationale"
                onChange={(event) => setCombinationRationale(event.target.value)}
                value={combinationRationale}
              />
            </label>
            <button
              data-testid="queue-combination-factor-intent"
              disabled={!combinationIntent}
              onClick={() => combinationIntent && onQueueIntent(combinationIntent)}
              title="Queue combination-factor operation"
              type="button"
            >
              <ListPlus size={14} aria-hidden="true" />
              Queue factor
            </button>
          </div>
          <p className="muted load-edit-preview" data-testid="load-manager-combination-factor-preview">
            {combinationIntent
              ? `${combinationIntent.operation_id}; before=${combinationIntent.change.before}; after=${combinationIntent.change.after}; unit=${combinationIntent.change.unit}; ${combinationIntent.change.dimension}; direct_model_mutation_allowed=false; professional_approval=false`
              : `current=${currentCombinationFactor}; no changed combination factor queued`}
          </p>
          <div className="combination-delete-controls">
            <label>
              <span>Delete rationale</span>
              <input
                aria-label="Combination term delete rationale"
                data-testid="load-manager-combination-delete-rationale"
                onChange={(event) => setCombinationDeleteRationale(event.target.value)}
                value={combinationDeleteRationale}
              />
            </label>
            <button
              data-testid="queue-delete-combination-term-intent"
              disabled={!combinationDeleteIntent}
              onClick={() => combinationDeleteIntent && onQueueIntent(combinationDeleteIntent)}
              title="Queue combination-term delete operation"
              type="button"
            >
              <Trash2 size={14} aria-hidden="true" />
              Queue delete
            </button>
          </div>
          <p className="muted load-edit-preview" data-testid="load-manager-combination-delete-preview">
            {combinationDeleteIntent
              ? `${combinationDeleteIntent.operation_id}; before=${combinationDeleteIntent.change.before}; after=${combinationDeleteIntent.change.after}; unit=${combinationDeleteIntent.change.unit}; ${combinationDeleteIntent.change.dimension}; direct_model_mutation_allowed=false; professional_approval=false`
              : "select a combination term and provide rationale to queue deletion"}
          </p>
        </section>
      ) : null}

      <small className="report-note" data-testid="load-case-manager-boundary">
        Load edits are queued as structured operations, applied only after user action, and remain local-session changes
        until saved. This panel does not add rule-code combinations, protected standards content, or professional
        approval.
      </small>
    </section>
  );
}

function loadCaseMetadataValue(loadCase: LoadCase, field: LoadMetadataField): string {
  return loadCase[field];
}

function loadCaseDisplay(loadCase: LoadCase): string {
  return `${loadCase.id}; ${loadCase.label}; ${loadCase.kind}; ${loadCase.status}; primitives=${
    loadCase.primitive_loads?.length ?? 0
  }`;
}

function loadMetadataOptions(model: PreviewModel, field: LoadMetadataField, currentValue: string): string[] {
  const values = new Set<string>();
  for (const loadCase of model.load_cases) {
    const value = loadCaseMetadataValue(loadCase, field).trim();
    if (value) values.add(value);
  }
  if (currentValue.trim()) values.add(currentValue.trim());
  values.add("TBD");
  if (field === "status") values.add("preview_only");
  if (field === "kind") values.add("primitive_user_load");
  return Array.from(values).sort((left, right) => {
    if (left === currentValue) return -1;
    if (right === currentValue) return 1;
    if (left === "TBD") return 1;
    if (right === "TBD") return -1;
    return left.localeCompare(right);
  });
}

function defaultLoadCaseDraft(id: string): LoadCaseDraft {
  return {
    id,
    label: "User load case",
    kind: "primitive_user_load",
    status: "draft",
    provenance: "user_entered_local_preview"
  };
}

function defaultPrimitiveLoadDraft(model: PreviewModel, loadCaseId: string): PrimitiveLoadDraft {
  const category = "concentrated_force";
  const direction = defaultPrimitiveLoadDirection(category);
  return {
    loadCaseId,
    id: nextPrimitiveLoadIdentifier(model, loadCaseId, category),
    category,
    targetNode: model.nodes[0]?.id ?? "",
    targetPipe: model.pipe_segments[0]?.id ?? "",
    targetSupport: model.supports[0]?.id ?? "",
    direction,
    magnitude: "250",
    unit: primitiveLoadDefaultUnit(model, category, direction),
    provenance: "user_entered_local_preview"
  };
}

function defaultCombinationTermDraft(model: PreviewModel): CombinationTermDraft {
  const combinationId = model.combinations?.[0]?.id ?? "";
  return {
    combinationId,
    loadCaseId: defaultCombinationTermLoadCase(model, combinationId),
    factor: "1",
    rationale: "user_entered_combination_term_preview_change"
  };
}

function defaultCombinationDraft(model: PreviewModel, id: string): CombinationDraft {
  return {
    id,
    label: "User mechanics combination",
    basis: "mechanics",
    loadCaseId: model.load_cases[0]?.id ?? "",
    factor: "1",
    provenance: "user_entered_local_preview",
    rationale: "user_entered_combination_create_preview_change"
  };
}

function defaultCombinationTermLoadCase(model: PreviewModel, combinationId: string): string {
  const combination = model.combinations?.find((item) => item.id === combinationId);
  const existing = new Set((combination?.terms ?? []).map((term) => term.load_case));
  return model.load_cases.find((loadCase) => !existing.has(loadCase.id))?.id ?? model.load_cases[0]?.id ?? "";
}

function nextLoadCaseIdentifier(model: PreviewModel): string {
  const used = new Set(model.load_cases.map((loadCase) => loadCase.id));
  for (let index = 300; index < 1000; index += 1) {
    const candidate = `load:L-${index}`;
    if (!used.has(candidate)) return candidate;
  }
  return `load:L-${model.load_cases.length + 1}`;
}

function nextCombinationIdentifier(model: PreviewModel): string {
  const used = new Set((model.combinations ?? []).map((combination) => combination.id));
  for (let index = 300; index < 1000; index += 1) {
    const candidate = `combination:C-${index}`;
    if (!used.has(candidate)) return candidate;
  }
  return `combination:C-${(model.combinations?.length ?? 0) + 1}`;
}

function isLoadCaseDraftReady(model: PreviewModel, draft: LoadCaseDraft): boolean {
  const id = draft.id.trim();
  return Boolean(
    id &&
      draft.label.trim() &&
      draft.kind.trim() &&
      draft.status.trim() &&
      draft.provenance.trim() &&
      !model.load_cases.some((loadCase) => loadCase.id === id)
  );
}

function isPrimitiveLoadDraftReady(model: PreviewModel, draft: PrimitiveLoadDraft): boolean {
  const id = draft.id.trim();
  const magnitude = parseFiniteNumber(draft.magnitude);
  const targetExists =
    primitiveLoadUsesSupportTarget(draft.category)
      ? model.supports.some((support) => support.id === draft.targetSupport)
      : primitiveLoadUsesPipeTarget(draft.category)
      ? model.pipe_segments.some((pipe) => pipe.id === draft.targetPipe)
      : model.nodes.some((node) => node.id === draft.targetNode);
  return Boolean(
    draft.loadCaseId &&
      model.load_cases.some((loadCase) => loadCase.id === draft.loadCaseId) &&
      id &&
      !primitiveLoadExists(model, id) &&
      targetExists &&
      primitiveLoadDirectionOptions(draft.category).includes(draft.direction) &&
      magnitude !== null &&
      magnitude !== 0 &&
      draft.provenance.trim() &&
      draft.unit.trim() &&
      draft.unit.trim() !== "TBD"
  );
}

function isCombinationDraftReady(model: PreviewModel, draft: CombinationDraft): boolean {
  const id = draft.id.trim();
  return Boolean(
    id &&
      draft.label.trim() &&
      draft.basis === "mechanics" &&
      draft.loadCaseId &&
      model.load_cases.some((loadCase) => loadCase.id === draft.loadCaseId) &&
      parseFiniteNumber(draft.factor) !== null &&
      draft.provenance.trim() &&
      draft.rationale.trim() &&
      !(model.combinations ?? []).some((combination) => combination.id === id)
  );
}

function isCombinationTermDraftReady(model: PreviewModel, draft: CombinationTermDraft): boolean {
  return Boolean(
    draft.combinationId &&
      model.combinations?.some((combination) => combination.id === draft.combinationId) &&
      draft.loadCaseId &&
      model.load_cases.some((loadCase) => loadCase.id === draft.loadCaseId) &&
      parseFiniteNumber(draft.factor) !== null &&
      draft.rationale.trim()
  );
}

function buildCreateLoadCaseIntent({
  model,
  draft
}: {
  model: PreviewModel;
  draft: LoadCaseDraft;
}): EditorOperationIntent {
  const payload = {
    id: draft.id.trim(),
    label: draft.label.trim(),
    kind: draft.kind.trim(),
    status: draft.status.trim(),
    provenance: draft.provenance.trim(),
    primitive_loads: []
  };
  const operationToken = `create-${safeToken(payload.id)}`;
  return {
    operation_id: `op:load-manager-${operationToken}`,
    operation_kind: "create",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "load_case_manager",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Load",
      ref: payload.id
    },
    change: {
      change_id: `change:load-manager-${operationToken}`,
      change_kind: "create_load_case",
      field_label: "Load case",
      field_path: "load_cases",
      before: "not_present",
      after: JSON.stringify(payload),
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered empty load-case shell; primitive loads are not inferred"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: `load-case creation intent for ${model.project.id}`
  };
}

function buildCreateCombinationIntent({
  model,
  draft
}: {
  model: PreviewModel;
  draft: CombinationDraft;
}): EditorOperationIntent {
  const factor = parseFiniteNumber(draft.factor) ?? 0;
  const payload = {
    id: draft.id.trim(),
    label: draft.label.trim(),
    basis: "mechanics",
    terms: [{ load_case: draft.loadCaseId, factor }],
    provenance: draft.provenance.trim()
  };
  const operationToken = `create-${safeToken(payload.id)}`;
  return {
    operation_id: `op:load-manager-${operationToken}`,
    operation_kind: "create",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "load_case_manager",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Combination",
      ref: payload.id
    },
    change: {
      change_id: `change:load-manager-${operationToken}`,
      change_kind: "create_combination",
      field_label: "Combination",
      field_path: "combinations",
      before: "not_present",
      after: JSON.stringify(payload),
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered mechanics combination; no code/rule combination default inferred"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: draft.rationale.trim() || `combination creation intent for ${model.project.id}`
  };
}

function buildCreatePrimitiveLoadIntent({
  model,
  draft
}: {
  model: PreviewModel;
  draft: PrimitiveLoadDraft;
}): EditorOperationIntent {
  const unit = draft.unit.trim() || "TBD";
  const dimension = primitiveLoadDraftDimension(draft.category, draft.direction);
  const magnitude = parseFiniteNumber(draft.magnitude) ?? 0;
  const payload = {
    id: draft.id.trim(),
    category: draft.category,
    target:
      primitiveLoadUsesSupportTarget(draft.category)
        ? { type: "support", support: draft.targetSupport, dof: draft.direction }
        : primitiveLoadUsesPipeTarget(draft.category)
        ? { type: "element", pipe: draft.targetPipe }
        : { type: "node", node: draft.targetNode },
    direction: draft.direction,
    magnitude: { value: magnitude, unit },
    dimension,
    provenance: draft.provenance.trim()
  };
  const operationToken = `${safeToken(draft.loadCaseId)}-${safeToken(payload.id)}-primitive`;
  return {
    operation_id: `op:load-manager-${operationToken}`,
    operation_kind: "create",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "load_case_manager",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Load",
      ref: draft.loadCaseId
    },
    change: {
      change_id: `change:load-manager-${operationToken}`,
      change_kind: "create_primitive_load",
      field_label: primitiveLoadFieldLabel(draft.category),
      field_path: "primitive_loads",
      before: "not_present",
      after: JSON.stringify(payload),
      unit,
      dimension,
      source_note: primitiveLoadSourceNote(draft.category)
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: `${draft.category} primitive-load creation intent for ${model.project.id}`
  };
}

function buildCreateCombinationTermIntent({
  model,
  draft
}: {
  model: PreviewModel;
  draft: CombinationTermDraft;
}): EditorOperationIntent {
  const combination = model.combinations?.find((item) => item.id === draft.combinationId);
  const termIndex = combination?.terms.length ?? 0;
  const factor = parseFiniteNumber(draft.factor) ?? 0;
  const payload = {
    load_case: draft.loadCaseId,
    factor
  };
  const operationToken = `${safeToken(draft.combinationId)}-term-${termIndex}-create`;
  return {
    operation_id: `op:load-manager-${operationToken}`,
    operation_kind: "create",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "load_case_manager",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Combination",
      ref: draft.combinationId
    },
    change: {
      change_id: `change:load-manager-${operationToken}`,
      change_kind: "create_combination_term",
      field_label: "Combination term",
      field_path: "terms",
      before: "not_present",
      after: JSON.stringify(payload),
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered combination term; no code/rule combination default inferred"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: draft.rationale.trim() || `combination term creation intent for ${model.project.id}`
  };
}

function buildLoadMetadataIntent({
  model,
  loadCase,
  field,
  proposedValue,
  rationale
}: {
  model: PreviewModel;
  loadCase: LoadCase;
  field: LoadMetadataField;
  proposedValue: string;
  rationale: string;
}): EditorOperationIntent {
  const operationToken = `${safeToken(loadCase.id)}-${field}`;
  return {
    operation_id: `op:load-manager-${operationToken}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "load_case_manager",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Load",
      ref: loadCase.id
    },
    change: {
      change_id: `change:load-manager-${operationToken}`,
      change_kind: "update_load",
      field_label: `Load case ${field}`,
      field_path: field,
      before: loadCaseMetadataValue(loadCase, field),
      after: proposedValue.trim() || "TBD",
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered load-case metadata"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: rationale.trim() || `load case metadata edit intent for ${model.project.id}`
  };
}

function buildDeleteLoadCaseIntent({
  model,
  loadCase,
  rationale
}: {
  model: PreviewModel;
  loadCase: LoadCase;
  rationale: string;
}): EditorOperationIntent {
  const operationToken = `${safeToken(loadCase.id)}-delete`;
  return {
    operation_id: `op:load-manager-${operationToken}`,
    operation_kind: "delete",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "load_case_manager",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Load",
      ref: loadCase.id
    },
    change: {
      change_id: `change:load-manager-${operationToken}`,
      change_kind: "delete_load_case",
      field_label: `${loadCase.label} load case`,
      field_path: "load_cases",
      before: loadCaseDisplay(loadCase),
      after: "not_present",
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered load-case deletion; whole load-case entity only"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: rationale.trim() || `load-case delete intent for ${model.project.id}`
  };
}

function combinationTermViews(model: PreviewModel): CombinationTermView[] {
  return (model.combinations ?? []).flatMap((combination) =>
    combination.terms.map((term, index) => ({
      index,
      term,
      combination
    }))
  );
}

function buildCombinationBasisIntent({
  model,
  combination,
  proposedBasis,
  rationale
}: {
  model: PreviewModel;
  combination: Combination;
  proposedBasis: string;
  rationale: string;
}): EditorOperationIntent {
  const operationToken = `${safeToken(combination.id)}-basis`;
  return {
    operation_id: `op:load-manager-${operationToken}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "load_case_manager",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Combination",
      ref: combination.id
    },
    change: {
      change_id: `change:load-manager-${operationToken}`,
      change_kind: "update_load",
      field_label: `${combination.label} combination basis`,
      field_path: "basis",
      before: combination.basis,
      after: proposedBasis.trim() || "TBD",
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered combination basis label; no code/rule combination default inferred"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: rationale.trim() || `combination basis edit intent for ${model.project.id}`
  };
}

function buildDeleteCombinationIntent({
  model,
  combination,
  rationale
}: {
  model: PreviewModel;
  combination: Combination;
  rationale: string;
}): EditorOperationIntent {
  const operationToken = `${safeToken(combination.id)}-delete`;
  return {
    operation_id: `op:load-manager-${operationToken}`,
    operation_kind: "delete",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "load_case_manager",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Combination",
      ref: combination.id
    },
    change: {
      change_id: `change:load-manager-${operationToken}`,
      change_kind: "delete_combination",
      field_label: `${combination.label} combination`,
      field_path: "combinations",
      before: combinationDisplay(combination),
      after: "not_present",
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered combination deletion; whole combination entity only"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: rationale.trim() || `combination delete intent for ${model.project.id}`
  };
}

function buildCombinationFactorIntent({
  model,
  termView,
  proposedFactor,
  rationale
}: {
  model: PreviewModel;
  termView: CombinationTermView;
  proposedFactor: string;
  rationale: string;
}): EditorOperationIntent {
  const fieldPath = `terms.${termView.index}.factor`;
  const operationToken = `${safeToken(termView.combination.id)}-term-${termView.index}-factor`;
  return {
    operation_id: `op:load-manager-${operationToken}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "load_case_manager",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Combination",
      ref: termView.combination.id
    },
    change: {
      change_id: `change:load-manager-${operationToken}`,
      change_kind: "update_load",
      field_label: `${combinationTermLoadCase(termView.term)} combination factor`,
      field_path: fieldPath,
      before: combinationTermFactorDisplay(termView.term),
      after: proposedFactor.trim() || combinationTermFactorDisplay(termView.term),
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered combination term factor; existing term only"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: rationale.trim() || `combination factor edit intent for ${model.project.id}`
  };
}

function buildDeleteCombinationTermIntent({
  model,
  termView,
  rationale
}: {
  model: PreviewModel;
  termView: CombinationTermView;
  rationale: string;
}): EditorOperationIntent {
  const fieldPath = `terms.${termView.index}`;
  const operationToken = `${safeToken(termView.combination.id)}-term-${termView.index}-delete`;
  return {
    operation_id: `op:load-manager-${operationToken}`,
    operation_kind: "delete",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "load_case_manager",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Combination",
      ref: termView.combination.id
    },
    change: {
      change_id: `change:load-manager-${operationToken}`,
      change_kind: "delete_combination_term",
      field_label: `${combinationTermLoadCase(termView.term)} combination term`,
      field_path: fieldPath,
      before: combinationTermDisplay(termView.term),
      after: "not_present",
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered combination term deletion; existing indexed term only"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: rationale.trim() || `combination term delete intent for ${model.project.id}`
  };
}

function primitiveLoadViews(model: PreviewModel): PrimitiveLoadView[] {
  return model.load_cases.flatMap((loadCase) =>
    (loadCase.primitive_loads ?? []).map((load, index) => ({
      index,
      load,
      loadCase
    }))
  );
}

function nextPrimitiveLoadIdentifier(
  model: PreviewModel,
  loadCaseId: string,
  category: PrimitiveLoadCategory = "concentrated_force"
): string {
  const suffix =
    category === "distributed_force"
      ? "D"
      : category === "concentrated_moment"
        ? "M"
        : category === "pressure"
          ? "P"
          : category === "thermal"
            ? "T"
            : category === "imposed_displacement"
              ? "I"
              : "F";
  for (let index = 300; index < 1000; index += 1) {
    const candidate = loadCaseId ? `${loadCaseId}-${suffix}${index}` : `load:${suffix}-${index}`;
    if (!primitiveLoadExists(model, candidate)) return candidate;
  }
  return `${loadCaseId || `load:${suffix}`}-${primitiveLoadViews(model).length + 1}`;
}

function primitiveLoadExists(model: PreviewModel, id: string): boolean {
  return primitiveLoadViews(model).some((primitive) => primitiveId(primitive.load) === id);
}

function buildLoadMagnitudeIntent({
  model,
  primitive,
  proposedMagnitude,
  proposedUnit,
  rationale
}: {
  model: PreviewModel;
  primitive: PrimitiveLoadView;
  proposedMagnitude: string;
  proposedUnit: string;
  rationale: string;
}): EditorOperationIntent {
  const fieldPath = `primitive_loads.${primitive.index}.magnitude.value`;
  const operationToken = `${safeToken(primitive.loadCase.id)}-${safeToken(primitiveId(primitive.load))}-magnitude`;
  const unit = proposedUnit.trim() || primitiveUnit(primitive.load);
  const parsedMagnitude = parseFiniteNumber(proposedMagnitude);
  const after = JSON.stringify({
    value: parsedMagnitude ?? (proposedMagnitude.trim() || "TBD"),
    unit
  });
  return {
    operation_id: `op:load-manager-${operationToken}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "load_case_manager",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Load",
      ref: primitive.loadCase.id
    },
    change: {
      change_id: `change:load-manager-${operationToken}`,
      change_kind: "update_load",
      field_label: `${primitiveCategory(primitive.load)} primitive magnitude`,
      field_path: fieldPath,
      before: primitiveMagnitudeDisplay(primitive.load),
      after,
      unit,
      dimension: primitiveDimension(primitive.load),
      source_note: "unit metadata required"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: rationale.trim() || `load magnitude edit intent for ${model.project.id}`
  };
}

function buildDeletePrimitiveLoadIntent({
  model,
  primitive,
  rationale
}: {
  model: PreviewModel;
  primitive: PrimitiveLoadView;
  rationale: string;
}): EditorOperationIntent {
  const fieldPath = `primitive_loads.${primitive.index}`;
  const operationToken = `${safeToken(primitive.loadCase.id)}-${safeToken(primitiveId(primitive.load))}-delete`;
  return {
    operation_id: `op:load-manager-${operationToken}`,
    operation_kind: "delete",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: "load_case_manager",
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: "Load",
      ref: primitive.loadCase.id
    },
    change: {
      change_id: `change:load-manager-${operationToken}`,
      change_kind: "delete_primitive_load",
      field_label: `${primitiveCategory(primitive.load)} primitive load`,
      field_path: fieldPath,
      before: primitiveLoadDisplay(primitive.load),
      after: "not_present",
      unit: primitiveUnit(primitive.load),
      dimension: primitiveDimension(primitive.load),
      source_note: "explicit user-entered primitive load deletion; existing indexed primitive only"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: rationale.trim() || `primitive-load delete intent for ${model.project.id}`
  };
}

function primitiveKey(primitive: PrimitiveLoadView): string {
  return `${primitive.loadCase.id}::${primitive.index}::${primitiveId(primitive.load)}`;
}

function combinationTermKey(termView: CombinationTermView): string {
  return `${termView.combination.id}::${termView.index}::${combinationTermLoadCase(termView.term)}`;
}

function combinationTermLoadCase(term: CombinationTerm): string {
  return term.load_case;
}

function combinationTermFactorDisplay(term: CombinationTerm): string {
  return String(term.factor);
}

function combinationTermDisplay(term: CombinationTerm): string {
  return `${combinationTermLoadCase(term)} x ${combinationTermFactorDisplay(term)}`;
}

function combinationDisplay(combination: Combination): string {
  const terms = combination.terms.length ? combination.terms.map(combinationTermDisplay).join("; ") : "none";
  return `${combination.id}; ${combination.label}; basis=${combination.basis}; terms=${terms}`;
}

function primitiveId(load: PrimitiveLoad): string {
  return stringField(load, "id");
}

function primitiveCategory(load: PrimitiveLoad): string {
  return stringField(load, "category");
}

function primitiveDirection(load: PrimitiveLoad): string {
  return stringField(load, "direction");
}

function primitiveDimension(load: PrimitiveLoad): string {
  return stringField(load, "dimension");
}

function primitiveTarget(load: PrimitiveLoad): string {
  const target = load.target;
  if (!target || typeof target !== "object" || Array.isArray(target)) return "target:TBD";
  const record = target as Record<string, unknown>;
  const type = stringField(record, "type");
  const pipe = optionalString(record.pipe);
  const node = optionalString(record.node);
  const support = optionalString(record.support);
  if (pipe) return `${type}:${pipe}`;
  if (node) return `${type}:${node}`;
  if (support) return `${type}:${support}`;
  return `${type}:TBD`;
}

function primitiveLoadDisplay(load: PrimitiveLoad): string {
  return `${primitiveId(load)}; ${primitiveCategory(load)}; ${primitiveTarget(load)}; ${primitiveDirection(load)}; ${primitiveMagnitudeDisplay(load)} ${primitiveUnit(load)}; ${primitiveDimension(load)}`;
}

function primitiveMagnitudeDisplay(load: PrimitiveLoad): string {
  const magnitude = load.magnitude;
  if (!magnitude || typeof magnitude !== "object" || Array.isArray(magnitude)) return "TBD";
  const value = (magnitude as Record<string, unknown>).value;
  return typeof value === "number" || typeof value === "string" ? String(value) : "TBD";
}

function primitiveUnit(load: PrimitiveLoad): string {
  const magnitude = load.magnitude;
  if (!magnitude || typeof magnitude !== "object" || Array.isArray(magnitude)) return "TBD";
  return optionalString((magnitude as Record<string, unknown>).unit) ?? "TBD";
}

function unitOptions(route: UnitCatalogRoute | null, dimensionId: string, fallbackSymbol: string): UnitOption[] {
  const fallback = { symbol: fallbackSymbol, label: `${fallbackSymbol} (model metadata)` };
  if (!fallbackSymbol || fallbackSymbol === "TBD") return [fallback];
  if (route?.route !== "tauri_unit_catalog") return [fallback];
  const options = acceptedUnits(route.catalog)
    .filter((entry) => unitEntryMatchesDimension(entry, dimensionId))
    .map((entry) => ({
      symbol: entry.symbol,
      label: `${entry.symbol}${entry.canonical ? " (DEC-018 canonical)" : " (DEC-018 display)"}`
    }));
  if (!options.some((option) => option.symbol === fallbackSymbol)) options.unshift(fallback);
  return options;
}

function projectForceUnit(model: PreviewModel): string {
  return optionalString(model.project.units.force) ?? "TBD";
}

function projectLengthUnit(model: PreviewModel): string {
  return optionalString(model.project.units.length) ?? "TBD";
}

function projectDistributedForceUnit(model: PreviewModel): string {
  const force = projectForceUnit(model);
  const length = projectLengthUnit(model);
  return force === "TBD" || length === "TBD" ? "TBD" : `${force}/${length}`;
}

function projectMomentUnit(model: PreviewModel): string {
  const force = projectForceUnit(model);
  const length = projectLengthUnit(model);
  return force === "TBD" || length === "TBD" ? "TBD" : `${force}*${length}`;
}

function projectPressureUnit(model: PreviewModel): string {
  return optionalString(model.project.units.pressure) ?? "TBD";
}

function projectTemperatureUnit(model: PreviewModel): string {
  return optionalString(model.project.units.temperature) ?? "TBD";
}

function projectRotationUnit(model: PreviewModel): string {
  return optionalString(model.project.units.angle) ?? "TBD";
}

function primitiveLoadDefaultUnit(model: PreviewModel, category: PrimitiveLoadCategory, direction: string): string {
  if (category === "distributed_force") return projectDistributedForceUnit(model);
  if (category === "concentrated_moment") return projectMomentUnit(model);
  if (category === "pressure") return projectPressureUnit(model);
  if (category === "thermal") return projectTemperatureUnit(model);
  if (category === "imposed_displacement") return primitiveLoadDofIsRotational(direction) ? projectRotationUnit(model) : projectLengthUnit(model);
  return projectForceUnit(model);
}

function primitiveLoadDraftDimension(category: PrimitiveLoadCategory, direction: string): string {
  if (category === "distributed_force") return "force_per_length";
  if (category === "concentrated_moment") return "moment";
  if (category === "pressure") return "pressure";
  if (category === "thermal") return "temperature_interval";
  if (category === "imposed_displacement") return primitiveLoadDofIsRotational(direction) ? "rotation" : "displacement";
  return "force";
}

function primitiveLoadDraftTargetDisplay(draft: PrimitiveLoadDraft): string {
  if (primitiveLoadUsesSupportTarget(draft.category)) return draft.targetSupport || "support:TBD";
  return primitiveLoadUsesPipeTarget(draft.category) ? draft.targetPipe || "pipe:TBD" : draft.targetNode || "node:TBD";
}

function primitiveLoadCategoryFromValue(value: string): PrimitiveLoadCategory {
  if (value === "concentrated_moment") return "concentrated_moment";
  if (value === "pressure") return "pressure";
  if (value === "thermal") return "thermal";
  if (value === "imposed_displacement") return "imposed_displacement";
  return value === "distributed_force" ? "distributed_force" : "concentrated_force";
}

function primitiveLoadUsesPipeTarget(category: PrimitiveLoadCategory): boolean {
  return category === "distributed_force" || category === "pressure" || category === "thermal";
}

function primitiveLoadUsesSupportTarget(category: PrimitiveLoadCategory): boolean {
  return category === "imposed_displacement";
}

function primitiveLoadDirectionOptions(category: PrimitiveLoadCategory): string[] {
  if (category === "imposed_displacement") return ["UX", "UY", "UZ", "RX", "RY", "RZ"];
  return category === "concentrated_moment" ? ["rotation_x", "rotation_y", "rotation_z"] : ["global_x", "global_y", "global_z"];
}

function defaultPrimitiveLoadDirection(category: PrimitiveLoadCategory): string {
  if (category === "concentrated_moment") return "rotation_z";
  if (category === "pressure") return "global_x";
  if (category === "thermal") return "global_z";
  if (category === "imposed_displacement") return "UZ";
  return "global_y";
}

function primitiveLoadFieldLabel(category: PrimitiveLoadCategory): string {
  if (category === "distributed_force") return "Distributed force primitive load";
  if (category === "concentrated_moment") return "Concentrated moment primitive load";
  if (category === "pressure") return "Pressure primitive load";
  if (category === "thermal") return "Thermal primitive load";
  if (category === "imposed_displacement") return "Imposed displacement primitive load";
  return "Concentrated force primitive load";
}

function primitiveLoadSourceNote(category: PrimitiveLoadCategory): string {
  if (category === "distributed_force") {
    return "explicit user-entered distributed element force; no concentrated moments, pressure, temperature, or imposed displacements inferred";
  }
  if (category === "concentrated_moment") {
    return "explicit user-entered concentrated nodal moment; no pressure, temperature, or imposed displacements inferred";
  }
  if (category === "pressure") {
    return "explicit user-entered pressure primitive; no temperature or imposed displacements inferred";
  }
  if (category === "thermal") {
    return "explicit user-entered temperature-interval primitive; no pressure or imposed displacements inferred";
  }
  if (category === "imposed_displacement") {
    return "explicit user-entered support-target imposed displacement; no support coordinate policy or hidden restraint defaults inferred";
  }
  return "explicit user-entered concentrated node force; no distributed loads, moments, or imposed displacements inferred";
}

function primitiveLoadDofIsRotational(direction: string): boolean {
  return direction === "RX" || direction === "RY" || direction === "RZ";
}

function parseFiniteNumber(raw: string): number | null {
  const trimmed = raw.trim();
  if (!/^[+-]?(\d+\.?\d*|\.\d+)([eE][+-]?\d+)?$/.test(trimmed)) return null;
  const parsed = Number.parseFloat(trimmed);
  return Number.isFinite(parsed) ? parsed : null;
}

function stringField(record: Record<string, unknown>, key: string): string {
  return optionalString(record[key]) ?? "TBD";
}

function optionalString(value: unknown): string | null {
  return typeof value === "string" && value.trim() ? value : null;
}

function safeToken(value: string): string {
  return value.replace(/[^A-Za-z0-9_.:-]+/g, "-");
}
