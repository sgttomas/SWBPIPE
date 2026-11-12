import { PressureAuthoringPanel } from "../pressure-authoring/PressureAuthoringPanel";
import { LoadReferenceStateInputs } from "./LoadReferenceStateInputs";
import { ConditionUseControl } from "../condition-sets/ConditionUseControl";
import { isPressureFree } from "../condition-sets/conditionSetIntents";
import {
  OPERATING_DIFFERENCE_BASIS,
  OPERATING_DIFFERENCE_EXACT_REFUSAL,
  OPERATING_DIFFERENCE_SOLVE_NOTE,
  OPERATING_DIFFERENCE_TEMPLATE_LABEL,
  combinationDeclaredMembersText
} from "./operatingDifferenceTemplate";
import {
  defaultPrimitiveLoadDirection,
  optionalString,
  primitiveLoadCategoryFromValue,
  primitiveLoadDefaultUnit,
  primitiveLoadDefaultUnitChoice,
  primitiveLoadDirectionOptions,
  primitiveLoadDraftDimension,
  primitiveLoadUsesPipeTarget,
  primitiveLoadUsesSupportTarget,
  type PrimitiveLoadCategory
} from "./primitiveLoadVocabulary";
import { ListPlus, Scale, SlidersHorizontal, Trash2 } from "lucide-react";
import { useEffect, useMemo, useRef, useState } from "react";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import {
  acceptedUnits,
  describeUnitBasis,
  loadUnitCatalog,
  unitEntryMatchesDimension,
  type UnitCatalogRoute
} from "../../services/unitCatalogService";
import { convertDisplayQuantities } from "../../services/displayQuantityService";
import { unitMoveNotice, useClearOnUnitMove } from "../workspace/entryRules";
import { VirtualMultiTargetPicker, VirtualTargetPicker } from "../workspace/VirtualTargetPicker";
import { VirtualList } from "../workspace/VirtualList";
import { combinationTermOperand, combinationTermsText, type Combination, type CombinationTerm } from "./combinationOperands";
import {
  DIFFERENCE_BASIS_OPERAND_CODE,
  NONLINEAR_MECHANICS_TEXT,
  UNSIGNED_GATE_TEXT,
  UNSIGNED_ROW_TEXT,
  combinationOperandRefusal,
  constantEffortSupportIds,
  createCombinationIntent,
  isTermsBasis,
  isUnsignedBasis,
  moveTerm,
  nonlinearSupportIds,
  operationIdToken,
  resolveReference,
  resolvedReferenceText,
  retainedCombinationMembers,
  shadowedCombinationIds,
  termDraftText,
  termPayload,
  termsDraftProblem,
  type OperandKind,
  type TermDraft
} from "./combinationAuthoring";
import { SeismicTemplateSection } from "./SeismicTemplateSection";
import { SEISMIC_TEMPLATE_PROVENANCE } from "./seismicTemplate";
import {
  DELETE_REFERENCED_CODE,
  NUMBER_DISPLAY_ENGINE_ABSENT,
  NumberDisplayUnavailable,
  combinationDeleteBefore,
  combinationReferencePaths,
  combinationTermDeleteBefore,
  historicalCombinationDisplay,
  historicalTermDisplay,
  type Canonicalize
} from "./combinationDelete";
import { EngineFeedback, useEngineCheckedQueue } from "../condition-sets/engineCheckedQueue";
import { canonicalJsonString } from "../../services/hashService";

type LoadCase = PreviewModel["load_cases"][number];
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
  /** "default": the unit follows the current project's default rule, recomputed on
   * every render; "user": an explicit choice, kept as entered. */
  unitSource: "default" | "user";
  provenance: string;
};

type CombinationTermDraft = {
  combinationId: string;
  /** The new term's typed operand: a load case or a combination. */
  operandKind: OperandKind;
  loadCaseId: string;
  combinationRef: string;
  factor: string;
  rationale: string;
};

// Closed basis set mirroring core/loads/load_case_algebra vocabulary; model document 0.9.0
// adds the unsigned bases.
const COMBINATION_BASIS_OPTIONS = ["mechanics", "result_state_subtraction", "range_envelope", "absolute_sum", "srss"] as const;
type CombinationBasis = (typeof COMBINATION_BASIS_OPTIONS)[number];

const COMBINATION_RANGE_MODES = ["min", "max", "min_abs", "max_abs"] as const;
const DIMENSIONLESS_UNIT_VALIDATION_STATUS = "not_required_dimensionless";

type CombinationDraft = {
  id: string;
  label: string;
  basis: CombinationBasis;
  /** The terms of a mechanics, absolute_sum or srss combination, in authored order. */
  terms: TermDraft[];
  minuendId: string;
  subtrahendId: string;
  operandIds: string[];
  mode: string;
  provenance: string;
  rationale: string;
  /** The "Operating minus sustained difference" template: writes
   * `difference_basis` on a result-state subtraction, only when the person chooses it. */
  operatingDifference: boolean;
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
  selection,
  queuedIntents, operationBusy, preparationEpoch, getPreparationEpoch
}: {
  model: PreviewModel;
  onQueueIntent: (intent: EditorOperationIntent) => void;
  onSelect: (selection: EntityRef) => void;
  selection: EntityRef;
  queuedIntents?: EditorOperationIntent[];
  operationBusy?: boolean;
  preparationEpoch?: number;
  getPreparationEpoch?: () => number;
}) {
  const primitiveLoads = useMemo(() => primitiveLoadViews(model), [model]);
  const combinationTerms = useMemo(() => combinationTermViews(model), [model]);
  const loadCaseOptions = useMemo(() => model.load_cases.map((loadCase) => ({ value: loadCase.id, label: loadCase.label || loadCase.id })), [model.load_cases]);
  const supportOptions = useMemo(() => model.supports.map((support) => ({ value: support.id, label: support.label || support.id, keywords: [support.node] })), [model.supports]);
  const pipeOptions = useMemo(() => model.pipe_segments.map((pipe) => ({ value: pipe.id, label: pipe.label || pipe.id, keywords: [pipe.from, pipe.to] })), [model.pipe_segments]);
  const nodeOptions = useMemo(() => model.nodes.map((node) => ({ value: node.id, label: node.label || node.id })), [model.nodes]);
  const combinationOptions = useMemo(() => (model.combinations ?? []).map((combination) => ({ value: combination.id, label: combination.label || combination.id })), [model.combinations]);
  // A bare reference (minuend, subtrahend, envelope operand) names a load case, or a
  // combination when no load case has the id; a combination whose id
  // a load case also has cannot be named this way and is listed as such, never offered.
  const shadowedCombinations = useMemo(() => shadowedCombinationIds(model), [model]);
  const referenceOptions = useMemo(() => [
    ...model.load_cases.map((loadCase) => ({ value: loadCase.id, label: `Load case: ${loadCase.label || loadCase.id}` })),
    ...(model.combinations ?? [])
      .filter((combination) => !shadowedCombinations.includes(combination.id))
      .map((combination) => ({ value: combination.id, label: `Combination: ${combination.label || combination.id}` }))
  ], [model.load_cases, model.combinations, shadowedCombinations]);
  const constantEffortSupports = useMemo(() => constantEffortSupportIds(model), [model]);
  const nonlinearSupports = useMemo(() => nonlinearSupportIds(model), [model]);
  const createLoadCaseIdRef = useRef<HTMLInputElement>(null);
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
  const [primitiveLoadDraftState, setPrimitiveLoadDraft] = useState<PrimitiveLoadDraft>(() =>
    defaultPrimitiveLoadDraft(model, model.load_cases[0]?.id ?? "")
  );
  const [unitCatalogRoute, setUnitCatalogRoute] = useState<UnitCatalogRoute | null>(null);
  // A default-sourced unit is never retained across a model, project-unit or catalog
  // change: it is the current rule's default, so a replaced project or a catalog that
  // arrives later can only clear it, never leave an inherited unit queueable.
  const primitiveLoadDraft: PrimitiveLoadDraft = primitiveLoadDraftState.unitSource === "default"
    ? { ...primitiveLoadDraftState, unit: primitiveLoadDefaultUnit(model, primitiveLoadDraftState.category, primitiveLoadDraftState.direction, unitCatalogRoute) }
    : primitiveLoadDraftState;
  // A magnitude typed beside a unit the user did not change is cleared when that unit
  // moves with the project units; the form says which unit changed (DEL-02-02 AC-011).
  // Choosing another category or direction is the user's own act and starts afresh.
  const [primitiveUnitClearNotice, setPrimitiveUnitClearNotice] = useState<string | null>(null);
  useClearOnUnitMove({
    magnitude: { label: "magnitude", unit: primitiveLoadDraft.unit, follows: primitiveLoadDraftState.unitSource === "default", values: [primitiveLoadDraft.magnitude] }
  }, `${primitiveLoadDraft.category}:${primitiveLoadDraft.direction}`, (moves) => {
    setPrimitiveLoadDraft((current) => ({ ...current, magnitude: "" }));
    setPrimitiveUnitClearNotice(unitMoveNotice(moves));
  });
  const primitiveUnitSymbol = primitiveLoadDraft.unit.trim();
  const primitiveDimensionId = primitiveLoadDraftDimension(primitiveLoadDraft.category, primitiveLoadDraft.direction);
  // Request identity also changes for A → B → A, so an earlier success for A
  // cannot enable queuing while its new validation request is pending.
  const primitiveUnitKey = useMemo(() => ({ unit: primitiveUnitSymbol, dimension: primitiveDimensionId }), [primitiveUnitSymbol, primitiveDimensionId]);
  const [primitiveUnitCheck, setPrimitiveUnitCheck] = useState<{ key: typeof primitiveUnitKey; valid: boolean; message: string } | null>(null);
  useEffect(() => {
    let active = true;
    convertDisplayQuantities([{ id: "primitive-unit", value: 1, from_unit: primitiveUnitSymbol, to_unit: primitiveUnitSymbol, dimension_id: primitiveDimensionId }])
      .then((results) => {
        if (!active) return;
        const result = results.find((item) => item.id === "primitive-unit");
        setPrimitiveUnitCheck({ key: primitiveUnitKey, valid: result?.status === "converted",
          message: result?.status === "unavailable" ? result.message : result?.status === "converted" ? "Unit dimension validated" : "Unit validation unavailable" });
      })
      .catch((error: unknown) => {
        if (active) setPrimitiveUnitCheck({ key: primitiveUnitKey, valid: false, message: `Unit validation unavailable: ${String(error)}` });
      });
    return () => { active = false; };
  }, [primitiveUnitSymbol, primitiveDimensionId, primitiveUnitKey]);
  const primitiveUnitReady = primitiveUnitCheck?.key === primitiveUnitKey && primitiveUnitCheck.valid;
  const primitiveUnitDiagnostic = primitiveUnitCheck?.key === primitiveUnitKey ? primitiveUnitCheck.message : "Validating unit dimension…";
  const changed = selectedPrimitive
    ? proposedMagnitude.trim() !== currentMagnitude || proposedMagnitudeUnit.trim() !== currentMagnitudeUnit
    : false;
  // The create form's rule: a finite, nonzero decimal magnitude.
  const parsedProposedMagnitude = parseFiniteNumber(proposedMagnitude);
  const proposedMagnitudeValid = parsedProposedMagnitude !== null && parsedProposedMagnitude !== 0;
  const intent =
    selectedPrimitive && changed && proposedMagnitudeValid
      ? buildLoadMagnitudeIntent({
          model,
          primitive: selectedPrimitive,
          proposedMagnitude: parsedProposedMagnitude,
          proposedUnit: proposedMagnitudeUnit,
          unitCatalogRoute,
          rationale
        })
      : null;
  const primitiveDeleteIntent =
    selectedPrimitive && primitiveDeleteRationale.trim()
      ? buildDeletePrimitiveLoadIntent({
          model,
          primitive: selectedPrimitive,
          unitCatalogRoute,
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
  // The create forms' rule: a finite decimal factor; zero is a valid explicit term.
  const proposedCombinationFactorValid = parseFiniteNumber(proposedCombinationFactor) !== null;
  const combinationIntent =
    selectedCombinationTerm && combinationFactorChanged && proposedCombinationFactorValid
      ? buildCombinationFactorIntent({
          model,
          termView: selectedCombinationTerm,
          proposedFactor: proposedCombinationFactor,
          rationale: combinationRationale
        })
      : null;
  // Delete before-values come only from the stored record or term: the historical display
  // where the engine keeps it, otherwise the stored JSON's canonical form (computed by the
  // wasm engine, so shown once ready).
  const { before: termDeleteBefore, unavailable: termDeleteUnavailable } = useStoredBefore(selectedCombinationTerm?.term ?? null, historicalTermDisplay);
  const combinationDeleteIntent =
    selectedCombinationTerm && combinationDeleteRationale.trim() && termDeleteBefore !== null
      ? buildDeleteCombinationTermIntent({
          model,
          termView: selectedCombinationTerm,
          rationale: combinationDeleteRationale,
          before: termDeleteBefore
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
  const { before: entityDeleteBefore, unavailable: entityDeleteUnavailable } = useStoredBefore(selectedCombination, historicalCombinationDisplay);
  const combinationEntityDeleteIntent =
    selectedCombination && combinationEntityDeleteRationale.trim() && entityDeleteBefore !== null
      ? buildDeleteCombinationIntent({
          model,
          combination: selectedCombination,
          rationale: combinationEntityDeleteRationale,
          before: entityDeleteBefore
        })
      : null;
  const queueProps = { model, queuedIntents, onQueueIntent, operationBusy, preparationEpoch, getPreparationEpoch };
  const entityDeleteQueue = useEngineCheckedQueue(queueProps);
  const termDeleteQueue = useEngineCheckedQueue(queueProps);
  const entityDeleteReferences = selectedCombination ? combinationReferencePaths(model, selectedCombination.id) : [];
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
  const createPrimitiveLoadIntent = primitiveUnitReady && isPrimitiveLoadDraftReady(model, primitiveLoadDraft)
    ? buildCreatePrimitiveLoadIntent({
        model,
        draft: primitiveLoadDraft,
        unitCatalogRoute
      })
    : null;
  const primitiveDraftDimension = primitiveLoadDraftDimension(primitiveLoadDraft.category, primitiveLoadDraft.direction);
  const primitiveDraftUnit = primitiveLoadDraft.unit;
  const primitiveDraftUnitBasis = describeUnitBasis(unitCatalogRoute, primitiveDraftUnit, primitiveDraftDimension);
  const primitiveDefaultUnitChoice = primitiveLoadDefaultUnitChoice(model, primitiveLoadDraft.category, primitiveLoadDraft.direction, unitCatalogRoute);
  const primitiveDraftUnitOptions = unitOptions(unitCatalogRoute, primitiveDraftDimension, primitiveDefaultUnitChoice.symbol);
  // a composed default the catalog lacks leaves the unit unselected, and says why.
  const primitiveUnitNotice = primitiveLoadDraft.unit === "" ? primitiveDefaultUnitChoice.notice : null;
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
      // Nothing is proposed: a case that is no longer present is cleared.
      const loadCaseId = model.load_cases.some((loadCase) => loadCase.id === draft.loadCaseId) ? draft.loadCaseId : "";
      const combinationRef = model.combinations?.some((combination) => combination.id === draft.combinationRef) ? draft.combinationRef : "";
      if (combinationId === draft.combinationId && loadCaseId === draft.loadCaseId && combinationRef === draft.combinationRef) return draft;
      return { ...draft, combinationId, loadCaseId, combinationRef };
    });
  }, [model, selectedCombination?.id]);

  useEffect(() => {
    setCombinationDraft((draft) => {
      const loadCaseExists = (ref: string) => model.load_cases.some((loadCase) => loadCase.id === ref);
      const id = draft.id.trim() && !model.combinations?.some((combination) => combination.id === draft.id.trim())
        ? draft.id
        : nextCombinationId;
      const combinationExists = (ref: string) => (model.combinations ?? []).some((combination) => combination.id === ref);
      // A term whose operand is no longer present is cleared, never retargeted; it keeps its kind.
      const referenceExists = (ref: string) => loadCaseExists(ref) || combinationExists(ref);
      const terms = draft.terms.map((term) =>
        term.ref === "" || (term.kind === "combination" ? combinationExists(term.ref) : loadCaseExists(term.ref)) ? term : { ...term, ref: "" }
      );
      const minuendId = referenceExists(draft.minuendId) ? draft.minuendId : "";
      const subtrahendId = referenceExists(draft.subtrahendId) ? draft.subtrahendId : "";
      const operandIds = draft.operandIds.filter(referenceExists);
      if (
        id === draft.id &&
        terms.every((term, index) => term === draft.terms[index]) &&
        minuendId === draft.minuendId &&
        subtrahendId === draft.subtrahendId &&
        operandIds.length === draft.operandIds.length
      ) {
        return draft;
      }
      return { ...draft, id, terms, minuendId, subtrahendId, operandIds };
    });
  }, [model, nextCombinationId]);

  useEffect(() => {
    // As the combination draft does: a model update that takes the drafted id re-proposes it.
    setLoadCaseDraft((draft) => {
      const id = draft.id.trim() && !model.load_cases.some((loadCase) => loadCase.id === draft.id.trim())
        ? draft.id
        : nextLoadCaseId;
      return id === draft.id ? draft : { ...draft, id };
    });
  }, [model, nextLoadCaseId]);

  useEffect(() => {
    // As the load-case draft does: a model update that takes the drafted id re-proposes it.
    setPrimitiveLoadDraft((draft) => {
      const id = draft.id.trim() && !primitiveLoadExists(model, draft.id.trim())
        ? draft.id
        : nextPrimitiveLoadIdentifier(model, draft.loadCaseId, draft.category);
      return id === draft.id ? draft : { ...draft, id };
    });
  }, [model]);

  function updateLoadCaseDraft(field: keyof LoadCaseDraft, value: string) {
    setLoadCaseDraft((draft) => ({ ...draft, [field]: value }));
  }

  function updatePrimitiveLoadDraft(field: keyof PrimitiveLoadDraft, value: string) {
    setPrimitiveUnitClearNotice(null);
    setPrimitiveLoadDraft((draft) => {
      const nextDraft = { ...draft, [field]: value } as PrimitiveLoadDraft;
      if (field === "unit") nextDraft.unitSource = "user";
      if (field === "category") nextDraft.category = primitiveLoadCategoryFromValue(value);
      if (field === "loadCaseId" || field === "category") {
        nextDraft.direction = defaultPrimitiveLoadDirection(nextDraft.category);
        nextDraft.id = nextPrimitiveLoadIdentifier(model, nextDraft.loadCaseId, nextDraft.category);
      }
      if (field === "category" || field === "direction") {
        nextDraft.unit = primitiveLoadDefaultUnit(model, nextDraft.category, nextDraft.direction, unitCatalogRoute);
        nextDraft.unitSource = "default";
      }
      return nextDraft;
    });
  }

  function updateCombinationTermDraft(field: keyof CombinationTermDraft, value: string) {
    setCombinationTermDraft((draft) => {
      const nextDraft = { ...draft, [field]: value } as CombinationTermDraft;
      if (field === "operandKind") nextDraft.operandKind = value === "combination" ? "combination" : "load_case";
      // A new target combination clears the chosen operand; nothing is proposed.
      if (field === "combinationId") {
        nextDraft.loadCaseId = "";
        nextDraft.combinationRef = "";
      }
      return nextDraft;
    });
  }

  function updateCombinationDraft(field: keyof CombinationDraft, value: string) {
    setCombinationDraft((draft) => {
      const nextDraft = { ...draft, [field]: value } as CombinationDraft;
      if (field === "basis") nextDraft.basis = combinationBasisFromValue(value);
      return nextDraft;
    });
  }

  /** Choosing the template clears both cases, so the person chooses which is which. */
  function updateCombinationDraftTemplate(operatingDifference: boolean) {
    setCombinationDraft((draft) => ({ ...draft, operatingDifference, ...(operatingDifference ? { minuendId: "", subtrahendId: "" } : {}) }));
  }

  function updateCombinationDraftTerm(index: number, patch: Partial<TermDraft>) {
    setCombinationDraft((draft) => ({
      ...draft,
      terms: draft.terms.map((term, position) => {
        if (position !== index) return term;
        const next = { ...term, ...patch };
        // Changing a term's kind clears its operand: the person chooses one of that kind.
        if (patch.kind !== undefined && patch.kind !== term.kind) next.ref = "";
        return next;
      })
    }));
  }

  function addCombinationDraftTerm() {
    setCombinationDraft((draft) => ({ ...draft, terms: [...draft.terms, { kind: "load_case", ref: "", factor: "" }] }));
  }

  function removeCombinationDraftTerm(index: number) {
    setCombinationDraft((draft) => ({ ...draft, terms: draft.terms.filter((_, position) => position !== index) }));
  }

  function moveCombinationDraftTerm(index: number, offset: -1 | 1) {
    setCombinationDraft((draft) => ({ ...draft, terms: moveTerm(draft.terms, index, offset) }));
  }

  function updateCombinationDraftOperands(operandIds: string[]) {
    setCombinationDraft((draft) => ({ ...draft, operandIds }));
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
    setCombinationTermDraft((draft) => (draft.combinationId === combination.id ? draft : {
      ...draft,
      combinationId: combination.id,
      loadCaseId: "",
      combinationRef: ""
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

      <PressureAuthoringPanel model={model} selection={selection} queuedIntents={queuedIntents} operationBusy={operationBusy} preparationEpoch={preparationEpoch} getPreparationEpoch={getPreparationEpoch} onQueueIntent={onQueueIntent} loadCaseId={selectedLoadCase?.id} />
      <LoadReferenceStateInputs model={model} selection={selection} queuedIntents={queuedIntents} operationBusy={operationBusy} preparationEpoch={preparationEpoch} getPreparationEpoch={getPreparationEpoch} onQueueIntent={onQueueIntent} loadCase={selectedLoadCase} />
      <ConditionUseControl key={selectedLoadCase?.id ?? "no-case"} model={model} queuedIntents={queuedIntents} operationBusy={operationBusy} preparationEpoch={preparationEpoch} getPreparationEpoch={getPreparationEpoch} onQueueIntent={onQueueIntent} loadCase={selectedLoadCase} />
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
              ref={createLoadCaseIdRef}
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
              list="load-manager-kind-suggestions"
              onChange={(event) => updateLoadCaseDraft("kind", event.target.value)}
              value={loadCaseDraft.kind}
            />
            <datalist id="load-manager-kind-suggestions" data-testid="load-manager-kind-suggestions">
              {loadMetadataOptions(model, "kind", "").filter((value) => value !== "TBD").map((value) => <option key={value} value={value} />)}
            </datalist>
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
            ? `${createLoadCaseIntent.operation_id}; before=${createLoadCaseIntent.change.before}; after=${loadCaseDraft.id}; unit=${createLoadCaseIntent.change.unit}; ${createLoadCaseIntent.change.dimension}; primitive_loads=0; unit_validation=${createLoadCaseIntent.validation.unit_validation}; direct_model_mutation_allowed=false; professional_approval=false`
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
              <option value="thermal">thermal</option>
              <option value="imposed_displacement">imposed_displacement</option>
            </select>
          </label>
          <p className="muted" data-testid="load-manager-pressure-primitive-retired">
            Legacy pressure primitives are retired. Author pressure as exact pressure regions in the pressure mechanics profile.
          </p>
          <VirtualTargetPicker label="Primitive load case" testId="load-manager-create-primitive-load-case" options={loadCaseOptions} value={primitiveLoadDraft.loadCaseId} onChange={(value) => updatePrimitiveLoadDraft("loadCaseId", value)} />
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
            <VirtualTargetPicker label="Primitive support target" testId="load-manager-create-primitive-support" options={supportOptions} value={primitiveLoadDraft.targetSupport} onChange={(value) => updatePrimitiveLoadDraft("targetSupport", value)} />
          ) : primitiveLoadUsesPipeTarget(primitiveLoadDraft.category) ? (
            <VirtualTargetPicker label="Primitive element target pipe" testId="load-manager-create-primitive-pipe" options={pipeOptions} value={primitiveLoadDraft.targetPipe} onChange={(value) => updatePrimitiveLoadDraft("targetPipe", value)} />
          ) : (
            <VirtualTargetPicker label="Concentrated force target node" testId="load-manager-create-primitive-node" options={nodeOptions} value={primitiveLoadDraft.targetNode} onChange={(value) => updatePrimitiveLoadDraft("targetNode", value)} />
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
            {primitiveUnitNotice ? <span className="muted" data-testid="load-manager-create-primitive-unit-notice">{primitiveUnitNotice}</span> : null}
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
          {primitiveUnitClearNotice ? <p role="status" data-testid="load-manager-create-primitive-unit-cleared">{primitiveUnitClearNotice}</p> : null}
          <button
            data-testid="queue-create-primitive-intent"
            disabled={!createPrimitiveLoadIntent}
            onClick={() => { if (!createPrimitiveLoadIntent) return; onQueueIntent(createPrimitiveLoadIntent); setPrimitiveUnitClearNotice(null); }}
            title="Queue primitive load operation"
            type="button"
          >
            <ListPlus size={14} aria-hidden="true" />
            Queue load
          </button>
        </div>
        <p className="muted load-edit-preview" data-testid="load-manager-create-primitive-preview">
          {createPrimitiveLoadIntent
            ? `${createPrimitiveLoadIntent.operation_id}; before=${createPrimitiveLoadIntent.change.before}; after=${primitiveLoadDraft.id}; target=${primitiveDraftTarget}; direction=${primitiveLoadDraft.direction}; unit=${createPrimitiveLoadIntent.change.unit}; ${createPrimitiveLoadIntent.change.dimension}; unit_validation=${createPrimitiveLoadIntent.validation.unit_validation}; direct_model_mutation_allowed=false; professional_approval=false`
            : primitiveLoadDraft.id.trim() && primitiveLoadExists(model, primitiveLoadDraft.id.trim())
              ? `id=${primitiveLoadDraft.id.trim()} already exists; no primitive load queued`
              : primitiveUnitNotice ?? (!primitiveUnitReady ? primitiveUnitDiagnostic
              : "complete case/id/target/direction/nonzero magnitude/provenance to queue a primitive load")}
          </p>
      </section>

      <section className="combination-create-editor" aria-label="Create combination">
        <div className="load-editor-heading" data-testid="load-manager-create-combination-heading">
          <ListPlus size={14} aria-hidden="true" />
          <strong>{combinationDraft.id}</strong>
          <span>{combinationDraftSummary(combinationDraft)}</span>
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
              {COMBINATION_BASIS_OPTIONS.map((basis) => (
                <option key={basis} value={basis}>
                  {basis}
                </option>
              ))}
            </select>
          </label>
          {combinationDraft.basis === "result_state_subtraction" ? (
            <>
              <fieldset className="combination-template" data-testid="load-manager-operating-difference">
                <legend>Template</legend>
                <label>
                  <input
                    type="checkbox"
                    aria-label={OPERATING_DIFFERENCE_TEMPLATE_LABEL}
                    data-testid="load-manager-operating-difference-toggle"
                    checked={combinationDraft.operatingDifference}
                    disabled={!isPressureFree(model)}
                    aria-describedby={!isPressureFree(model) ? "load-manager-operating-difference-refused" : undefined}
                    onChange={(event) => updateCombinationDraftTemplate(event.target.checked)}
                  />
                  {OPERATING_DIFFERENCE_TEMPLATE_LABEL} (difference_basis {OPERATING_DIFFERENCE_BASIS})
                </label>
                {!isPressureFree(model) ? (
                  <p role="note" id="load-manager-operating-difference-refused" data-testid="load-manager-operating-difference-refused">
                    <strong>Refused: {OPERATING_DIFFERENCE_EXACT_REFUSAL.code}</strong> — {OPERATING_DIFFERENCE_EXACT_REFUSAL.text}
                  </p>
                ) : combinationDraft.operatingDifference ? (
                  <p className="muted" data-testid="load-manager-operating-difference-note">
                    Choose the operating case as the minuend and the sustained case as the subtrahend; neither is chosen for you. {OPERATING_DIFFERENCE_SOLVE_NOTE}
                  </p>
                ) : null}
              </fieldset>
              <VirtualTargetPicker label={combinationDraft.operatingDifference ? "Operating case (minuend)" : "Subtraction minuend (load case or combination)"} testId="load-manager-create-combination-minuend" options={combinationDraft.operatingDifference ? loadCaseOptions : referenceOptions} value={combinationDraft.minuendId} onChange={(value) => updateCombinationDraft("minuendId", value)} />
              <VirtualTargetPicker label={combinationDraft.operatingDifference ? "Sustained case (subtrahend)" : "Subtraction subtrahend (load case or combination)"} testId="load-manager-create-combination-subtrahend" options={combinationDraft.operatingDifference ? loadCaseOptions : referenceOptions} value={combinationDraft.subtrahendId} onChange={(value) => updateCombinationDraft("subtrahendId", value)} />
              <p className="muted" data-testid="load-manager-create-combination-resolved">
                Minuend: {resolvedReferenceText(model, combinationDraft.minuendId)}. Subtrahend: {resolvedReferenceText(model, combinationDraft.subtrahendId)}.
                {combinationDraft.operatingDifference ? ` A declared operating comparison names load cases only (${DIFFERENCE_BASIS_OPERAND_CODE}), so only load cases are offered.` : ""}
              </p>
            </>
          ) : combinationDraft.basis === "range_envelope" ? (
            <>
              <VirtualMultiTargetPicker label="Range envelope operands (load cases or combinations)" testId="load-manager-create-combination-operands" options={referenceOptions} values={combinationDraft.operandIds} onChange={(values) => updateCombinationDraftOperands([...values])} />
              <p className="muted" data-testid="load-manager-create-combination-resolved">
                Operands: {combinationDraft.operandIds.length ? combinationDraft.operandIds.map((id) => resolvedReferenceText(model, id)).join("; ") : "none chosen"}.
              </p>
              <label>
                <span>Mode</span>
                <select
                  aria-label="Range envelope mode"
                  data-testid="load-manager-create-combination-mode"
                  onChange={(event) => updateCombinationDraft("mode", event.target.value)}
                  value={combinationDraft.mode}
                >
                  <option value="">Choose…</option>
                  {COMBINATION_RANGE_MODES.map((mode) => (
                    <option key={mode} value={mode}>
                      {mode}
                    </option>
                  ))}
                </select>
              </label>
            </>
          ) : (
            <fieldset className="combination-terms" data-testid="load-manager-create-combination-terms">
              <legend>Terms, in authored order</legend>
              {combinationDraft.terms.map((term, index) => {
                const position = index + 1;
                const kindOptions = term.kind === "combination"
                  ? combinationOptions.map((option) => {
                      const refusal = combinationOperandRefusal(model, combinationDraft.id.trim(), option.value);
                      return refusal ? { ...option, label: `${option.label} (refused: ${refusal})`, disabled: true } : option;
                    })
                  : loadCaseOptions;
                return (
                  <div className="combination-term-draft" data-testid={`load-manager-create-combination-term-${index}`} key={index} role="group" aria-label={`Term ${position}`}>
                    <label>
                      <span>Operand kind</span>
                      <select
                        aria-label={`Term ${position} operand kind`}
                        data-testid={`load-manager-create-combination-term-${index}-kind`}
                        onChange={(event) => updateCombinationDraftTerm(index, { kind: event.target.value === "combination" ? "combination" : "load_case" })}
                        value={term.kind}
                      >
                        <option value="load_case">Load case</option>
                        <option value="combination" disabled={!combinationOptions.length}>Combination</option>
                      </select>
                    </label>
                    <VirtualTargetPicker
                      label={index === 0 && term.kind === "load_case" ? "New combination initial load case" : `Term ${position} ${term.kind === "combination" ? "combination" : "load case"}`}
                      testId={index === 0 && term.kind === "load_case" ? "load-manager-create-combination-load-case" : `load-manager-create-combination-term-${index}-${term.kind === "combination" ? "combination" : "load-case"}`}
                      options={kindOptions}
                      value={term.ref}
                      onChange={(value) => updateCombinationDraftTerm(index, { ref: value })}
                    />
                    <label>
                      <span>Factor</span>
                      <input
                        aria-label={index === 0 ? "New combination factor" : `Term ${position} factor`}
                        data-testid={index === 0 ? "load-manager-create-combination-factor" : `load-manager-create-combination-term-${index}-factor`}
                        onChange={(event) => updateCombinationDraftTerm(index, { factor: event.target.value })}
                        value={term.factor}
                      />
                    </label>
                    <button type="button" aria-label={`Move term ${position} up`} data-testid={`load-manager-create-combination-term-${index}-up`} disabled={index === 0} onClick={() => moveCombinationDraftTerm(index, -1)}>Up</button>
                    <button type="button" aria-label={`Move term ${position} down`} data-testid={`load-manager-create-combination-term-${index}-down`} disabled={index === combinationDraft.terms.length - 1} onClick={() => moveCombinationDraftTerm(index, 1)}>Down</button>
                    <button type="button" aria-label={`Remove term ${position}`} data-testid={`load-manager-create-combination-term-${index}-remove`} disabled={combinationDraft.terms.length === 1} title={combinationDraft.terms.length === 1 ? "A terms combination keeps at least one term." : undefined} onClick={() => removeCombinationDraftTerm(index)}>Remove</button>
                  </div>
                );
              })}
              <button type="button" data-testid="load-manager-create-combination-add-term" onClick={addCombinationDraftTerm}>
                <ListPlus size={14} aria-hidden="true" />
                Add term
              </button>
              {combinationTermsGateNote(combinationDraft, constantEffortSupports, nonlinearSupports) ? (
                <p className="muted" role="note" data-testid="load-manager-create-combination-gates">
                  {combinationTermsGateNote(combinationDraft, constantEffortSupports, nonlinearSupports)}
                </p>
              ) : null}
            </fieldset>
          )}
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
            ? `${createCombinationIntent.operation_id}; before=${createCombinationIntent.change.before}; after=${combinationDraft.id}; ${combinationDraftExpression(combinationDraft)}; unit=${createCombinationIntent.change.unit}; ${createCombinationIntent.change.dimension}; unit_validation=${createCombinationIntent.validation.unit_validation}; direct_model_mutation_allowed=false; professional_approval=false`
            : combinationDraft.id.trim() &&
                model.combinations?.some((combination) => combination.id === combinationDraft.id.trim())
              ? `id=${combinationDraft.id.trim()} already exists; no combination queued`
              : combinationDraftHelpText(model, combinationDraft)}
        </p>
      </section>

      <SeismicTemplateSection
        model={model}
        onQueueIntent={onQueueIntent}
        queuedIntents={queuedIntents}
        operationBusy={operationBusy}
        preparationEpoch={preparationEpoch}
        getPreparationEpoch={getPreparationEpoch}
        avoidIds={[combinationDraft.id.trim()]}
        onCreateLoadCase={() => {
          const input = createLoadCaseIdRef.current;
          input?.closest("section")?.scrollIntoView?.({ block: "nearest" });
          input?.focus();
        }}
      />

      <VirtualList
        ariaLabel="Load cases"
        className="load-case-list"
        height={Math.min(360, Math.max(56, model.load_cases.length * 56))}
        itemKey={(loadCase) => loadCase.id}
        items={model.load_cases}
        renderItem={(loadCase) => (
          <button
            className={selection.type === "load" && selection.id === loadCase.id ? "load-case-row active" : "load-case-row"}
            data-testid={`load-manager-case-${loadCase.id}`}
            key={loadCase.id}
            onClick={() => handleSelectLoadCase(loadCase)}
            type="button"
          >
            <strong>{loadCaseLabelDisplay(loadCase)}</strong>
            <small>
              {loadCase.id}; {metadataDisplay(loadCase.kind)}; {metadataDisplay(loadCase.status)}; primitives={loadCase.primitive_loads?.length ?? 0}
            </small>
            <small>{loadCase.provenance}</small>
          </button>
        )}
        role="list"
        rowHeight={56}
        testId="load-case-manager-cases"
      />

      {selectedLoadCase ? (
        <section className="load-metadata-editor" aria-label="Load case metadata editor">
          <div className="load-editor-heading" data-testid="load-manager-selected-case">
            <SlidersHorizontal size={14} aria-hidden="true" />
            <strong>{selectedLoadCase.id}</strong>
            <span>
              field={metadataField}; current={metadataDisplay(currentMetadataValue)}; path={metadataField}
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
              ? `${metadataIntent.operation_id}; before=${metadataIntent.change.before}; after=${metadataIntent.change.after}; unit=${metadataIntent.change.unit}; ${metadataIntent.change.dimension}; unit_validation=${metadataIntent.validation.unit_validation}; direct_model_mutation_allowed=false; professional_approval=false`
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
              ? `${loadCaseDeleteIntent.operation_id}; before=${loadCaseDeleteIntent.change.before}; after=${loadCaseDeleteIntent.change.after}; unit=${loadCaseDeleteIntent.change.unit}; ${loadCaseDeleteIntent.change.dimension}; unit_validation=${loadCaseDeleteIntent.validation.unit_validation}; direct_model_mutation_allowed=false; professional_approval=false`
              : "select a load case and provide rationale to queue deletion"}
          </p>
        </section>
      ) : null}

      <div className="load-primitive-list" data-testid="load-case-manager-primitives">
        {primitiveLoads.map((primitive) => (
          <button
            aria-pressed={Boolean(selectedPrimitive && primitiveKey(primitive) === primitiveKey(selectedPrimitive))}
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
              ? `${intent.operation_id}; before=${intent.change.before}; after=${intent.change.after}; unit=${intent.change.unit}; ${intent.change.dimension}; unit_validation=${intent.validation.unit_validation}; direct_model_mutation_allowed=false; professional_approval=false`
              : changed && !proposedMagnitudeValid
              ? `current=${currentMagnitude} ${primitiveUnit(selectedPrimitive.load)}; enter a finite nonzero magnitude; no changed magnitude queued`
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
              ? `${primitiveDeleteIntent.operation_id}; before=${primitiveDeleteIntent.change.before}; after=${primitiveDeleteIntent.change.after}; unit=${primitiveDeleteIntent.change.unit}; ${primitiveDeleteIntent.change.dimension}; unit_validation=${primitiveDeleteIntent.validation.unit_validation}; direct_model_mutation_allowed=false; professional_approval=false`
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
                  aria-pressed={selectedCombination?.id === combination.id}
                  className={
                    selectedCombination?.id === combination.id ? "combination-summary-row active" : "combination-summary-row"
                  }
                  data-testid={`load-manager-combination-select-${combination.id}`}
                  onClick={() => handleSelectCombination(combination)}
                  type="button"
                >
                  <strong>{combination.label}</strong>
                  <small>
                    {combination.id}; basis={combination.basis}; {combinationRowExpression(combination)}{combinationDeclaredMembersText(combination)}
                  </small>
                </button>
                {storedReferenceIds(combination).length ? (
                  <small className="muted" data-testid={`load-manager-combination-resolved-${combination.id}`}>
                    References: {storedReferenceIds(combination).map((id) => resolvedReferenceText(model, id)).join("; ")}
                  </small>
                ) : null}
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
              new term; {combinationTermDraft.operandKind === "combination" ? `combination=${combinationTermDraft.combinationRef || "TBD"}` : `load=${combinationTermDraft.loadCaseId || "TBD"}`}; factor={combinationTermDraft.factor.trim() || "TBD"}
            </span>
          </div>
          <div className="combination-term-create-controls">
            <VirtualTargetPicker label="Combination term target combination" testId="load-manager-create-combination-term-combination" options={combinationOptions} value={combinationTermDraft.combinationId} onChange={(value) => updateCombinationTermDraft("combinationId", value)} />
            <label>
              <span>Operand kind</span>
              <select
                aria-label="Combination term operand kind"
                data-testid="load-manager-create-combination-term-kind"
                onChange={(event) => updateCombinationTermDraft("operandKind", event.target.value)}
                value={combinationTermDraft.operandKind}
              >
                <option value="load_case">Load case</option>
                <option value="combination">Combination</option>
              </select>
            </label>
            {combinationTermDraft.operandKind === "combination" ? (
              <VirtualTargetPicker
                label="Combination term operand combination"
                testId="load-manager-create-combination-term-operand-combination"
                options={combinationOptions.map((option) => {
                  const refusal = combinationTermOperandRefusal(model, combinationTermDraft.combinationId, option.value);
                  return refusal ? { ...option, label: `${option.label} (refused: ${refusal})`, disabled: true } : option;
                })}
                value={combinationTermDraft.combinationRef}
                onChange={(value) => updateCombinationTermDraft("combinationRef", value)}
              />
            ) : (
              <VirtualTargetPicker label="Combination term load case" testId="load-manager-create-combination-term-load-case" options={loadCaseOptions} value={combinationTermDraft.loadCaseId} onChange={(value) => updateCombinationTermDraft("loadCaseId", value)} />
            )}
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
              ? `${createCombinationTermIntent.operation_id}; before=${createCombinationTermIntent.change.before}; after=${combinationTermDraft.operandKind === "combination" ? `combination ${combinationTermDraft.combinationRef}` : combinationTermDraft.loadCaseId} x ${combinationTermDraft.factor}; unit=${createCombinationTermIntent.change.unit}; ${createCombinationTermIntent.change.dimension}; unit_validation=${createCombinationTermIntent.validation.unit_validation}; direct_model_mutation_allowed=false; professional_approval=false`
              : `select combination/load case and finite factor to queue a combination term${combinationTermDraftProblem(model, combinationTermDraft) ? `; ${combinationTermDraftProblem(model, combinationTermDraft)}` : ""}`}
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
          {retainedCombinationMembers(selectedCombination).length ? (
            <p className="muted" data-testid="load-manager-combination-retained">
              Retained as stored (this editor does not edit them; a basis, factor or term edit writes only its own member):{" "}
              {retainedCombinationMembers(selectedCombination).join(", ")}.
            </p>
          ) : null}
          <div className="combination-basis-controls">
            <label>
              <span>Basis</span>
              <select
                aria-label="Combination basis"
                data-testid="load-manager-combination-basis-value"
                onChange={(event) => setProposedCombinationBasis(event.target.value)}
                value={proposedCombinationBasis}
              >
                {!COMBINATION_BASIS_OPTIONS.some((basis) => basis === currentCombinationBasis) && (
                  <option value={currentCombinationBasis}>{currentCombinationBasis} (stored)</option>
                )}
                {COMBINATION_BASIS_OPTIONS.map((basis) => (
                  <option key={basis} value={basis}>
                    {basis}
                  </option>
                ))}
              </select>
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
              ? `${combinationBasisIntent.operation_id}; before=${combinationBasisIntent.change.before}; after=${combinationBasisIntent.change.after}; unit=${combinationBasisIntent.change.unit}; ${combinationBasisIntent.change.dimension}; unit_validation=${combinationBasisIntent.validation.unit_validation}; direct_model_mutation_allowed=false; professional_approval=false`
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
              disabled={!combinationEntityDeleteIntent || entityDeleteQueue.busy}
              aria-describedby={entityDeleteReferences.length ? "load-manager-combination-entity-delete-references" : undefined}
              onClick={() => {
                const combination = selectedCombination;
                const rationale = combinationEntityDeleteRationale;
                void entityDeleteQueue.submit(async (canonicalize: Canonicalize) =>
                  buildDeleteCombinationIntent({ model, combination, rationale, before: await combinationDeleteBefore(combination, canonicalize) }));
              }}
              title="Queue combination delete operation"
              type="button"
            >
              <Trash2 size={14} aria-hidden="true" />
              Queue delete combo
            </button>
          </div>
          {entityDeleteReferences.length ? (
            <p role="note" id="load-manager-combination-entity-delete-references" data-testid="load-manager-combination-entity-delete-references">
              {combinationDeleteReferenceText(model, selectedCombination.id, entityDeleteReferences)}
            </p>
          ) : null}
          <p className="muted load-edit-preview" data-testid="load-manager-combination-entity-delete-preview">
            {combinationEntityDeleteIntent
              ? `${combinationEntityDeleteIntent.operation_id}; before=${combinationEntityDeleteIntent.change.before}; after=${combinationEntityDeleteIntent.change.after}; unit=${combinationEntityDeleteIntent.change.unit}; ${combinationEntityDeleteIntent.change.dimension}; unit_validation=${combinationEntityDeleteIntent.validation.unit_validation}; direct_model_mutation_allowed=false; professional_approval=false`
              : entityDeleteUnavailable
                ? NUMBER_DISPLAY_ENGINE_ABSENT
                : selectedCombination && combinationEntityDeleteRationale.trim()
                ? "computing the stored record's canonical JSON for the delete's before value"
                : "select a combination and provide rationale to queue deletion"}
          </p>
          <EngineFeedback error={entityDeleteQueue.error} notice={entityDeleteQueue.notice} diagnostics={entityDeleteQueue.diagnostics} testId="load-manager-combination-entity-delete-engine" />
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
              ? `${combinationIntent.operation_id}; before=${combinationIntent.change.before}; after=${combinationIntent.change.after}; unit=${combinationIntent.change.unit}; ${combinationIntent.change.dimension}; unit_validation=${combinationIntent.validation.unit_validation}; direct_model_mutation_allowed=false; professional_approval=false`
              : combinationFactorChanged && !proposedCombinationFactorValid
              ? `current=${currentCombinationFactor}; enter a finite factor; no changed combination factor queued`
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
              disabled={!combinationDeleteIntent || termDeleteQueue.busy}
              onClick={() => {
                const termView = selectedCombinationTerm;
                const rationale = combinationDeleteRationale;
                void termDeleteQueue.submit(async (canonicalize: Canonicalize) =>
                  buildDeleteCombinationTermIntent({ model, termView, rationale, before: await combinationTermDeleteBefore(termView.term, canonicalize) }));
              }}
              title="Queue combination-term delete operation"
              type="button"
            >
              <Trash2 size={14} aria-hidden="true" />
              Queue delete
            </button>
          </div>
          <p className="muted load-edit-preview" data-testid="load-manager-combination-delete-preview">
            {combinationDeleteIntent
              ? `${combinationDeleteIntent.operation_id}; before=${combinationDeleteIntent.change.before}; after=${combinationDeleteIntent.change.after}; unit=${combinationDeleteIntent.change.unit}; ${combinationDeleteIntent.change.dimension}; unit_validation=${combinationDeleteIntent.validation.unit_validation}; direct_model_mutation_allowed=false; professional_approval=false`
              : termDeleteUnavailable
                ? NUMBER_DISPLAY_ENGINE_ABSENT
                : selectedCombinationTerm && combinationDeleteRationale.trim()
                ? "computing the stored term's canonical JSON for the delete's before value"
                : "select a combination term and provide rationale to queue deletion"}
          </p>
          <EngineFeedback error={termDeleteQueue.error} notice={termDeleteQueue.notice} diagnostics={termDeleteQueue.diagnostics} testId="load-manager-combination-delete-engine" />
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

/** An absent (or non-text) label, kind or status reads as empty: the panel
 * shows it as not set and never writes a value for it (T1 WP3 addendum). */
function loadCaseMetadataValue(loadCase: LoadCase, field: LoadMetadataField): string {
  const value: unknown = loadCase[field];
  return typeof value === "string" ? value : "";
}

function metadataDisplay(value: unknown): string {
  return typeof value === "string" && value !== "" ? value : "not set";
}

/** Display only: an absent or empty label is shown as the case id. */
function loadCaseLabelDisplay(loadCase: LoadCase): string {
  const label: unknown = loadCase.label;
  return typeof label === "string" && label !== "" ? label : loadCase.id;
}

/** Mirrors the operation engine's delete before-display
 * (`load_case_delete_display`), which reads an absent text field as TBD. The
 * string is a comparison value only; nothing is written to the model. */
function engineMetadataText(value: unknown): string {
  return typeof value === "string" ? value : "TBD";
}

function loadCaseDisplay(loadCase: LoadCase): string {
  return `${loadCase.id}; ${engineMetadataText(loadCase.label)}; ${engineMetadataText(loadCase.kind)}; ${engineMetadataText(loadCase.status)}; primitives=${
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
  if (field === "kind") {
    values.add("primitive_user_load");
    // Model document 0.9.0 §2.3: a suggestion only; `kind` stays a free string the solver
    // ignores, and `operating` implies no composition or other meaning.
    values.add("operating");
  }
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
    // The source is the user's own statement: it starts empty (PRD §5.5; DEL-07-03 AC-001).
    provenance: ""
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
    unitSource: "default",
    // The source is the user's own statement: it starts empty (PRD §5.5; DEL-07-03 AC-001).
    provenance: ""
  };
}

function defaultCombinationTermDraft(model: PreviewModel): CombinationTermDraft {
  const combinationId = model.combinations?.[0]?.id ?? "";
  return {
    combinationId,
    operandKind: "load_case",
    // Every operand and factor is entered by the person: blank means not chosen.
    loadCaseId: "",
    combinationRef: "",
    factor: "",
    rationale: "user_entered_combination_term_preview_change"
  };
}

function defaultCombinationDraft(model: PreviewModel, id: string): CombinationDraft {
  return {
    id,
    label: "User mechanics combination",
    basis: "mechanics",
    // No operand, factor or mode is proposed (no first case, no factor 1): blank means not chosen.
    terms: [{ kind: "load_case", ref: "", factor: "" }],
    minuendId: "",
    subtrahendId: "",
    operandIds: [],
    mode: "",
    // The source is the user's own statement: it starts empty (PRD §5.5; DEL-07-03 AC-001).
    provenance: "",
    rationale: "user_entered_combination_create_preview_change",
    operatingDifference: false
  };
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
  const common = Boolean(
    id &&
      draft.label.trim() &&
      draft.provenance.trim() &&
      draft.rationale.trim() &&
      !(model.combinations ?? []).some((combination) => combination.id === id)
  );
  if (!common) return false;
  // A bare reference resolves to a load case, else a combination; a
  // declared operating comparison names load cases only.
  const resolvesTo = (ref: string) => resolveReference(model, ref).kind;
  if (draft.basis === "result_state_subtraction") {
    const admitted = (ref: string) => (draft.operatingDifference ? resolvesTo(ref) === "load_case" : resolvesTo(ref) !== "unknown");
    return Boolean(
      draft.minuendId &&
        draft.subtrahendId &&
        draft.minuendId !== draft.subtrahendId &&
        admitted(draft.minuendId) &&
        admitted(draft.subtrahendId)
    );
  }
  if (draft.basis === "range_envelope") {
    return Boolean(
      draft.operandIds.length > 0 &&
        new Set(draft.operandIds).size === draft.operandIds.length &&
        draft.operandIds.every((ref) => resolvesTo(ref) !== "unknown") &&
        COMBINATION_RANGE_MODES.includes(draft.mode as (typeof COMBINATION_RANGE_MODES)[number])
    );
  }
  return isTermsBasis(draft.basis) && termsDraftProblem(model, draft.terms, id) === null;
}

function isCombinationTermDraftReady(model: PreviewModel, draft: CombinationTermDraft): boolean {
  if (draft.operandKind === "combination") {
    return Boolean(
      draft.combinationId &&
        model.combinations?.some((combination) => combination.id === draft.combinationId) &&
        draft.combinationRef &&
        model.combinations?.some((combination) => combination.id === draft.combinationRef) &&
        combinationTermOperandRefusal(model, draft.combinationId, draft.combinationRef) === null &&
        parseFiniteNumber(draft.factor) !== null &&
        draft.rationale.trim()
    );
  }
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
  const operationToken = `create-${operationIdToken(payload.id)}`;
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
      unit_validation: DIMENSIONLESS_UNIT_VALIDATION_STATUS,
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

function combinationCreatePayload(draft: CombinationDraft): Record<string, unknown> {
  const common = {
    id: draft.id.trim(),
    label: draft.label.trim(),
    basis: draft.basis,
    provenance: draft.provenance.trim()
  };
  if (draft.basis === "result_state_subtraction") {
    return {
      ...common,
      minuend_id: draft.minuendId,
      subtrahend_id: draft.subtrahendId,
      terms: [],
      // Written only when the person chose the template; never subtrahend_support_state.
      ...(draft.operatingDifference ? { difference_basis: OPERATING_DIFFERENCE_BASIS } : {})
    };
  }
  if (draft.basis === "range_envelope") {
    return {
      ...common,
      operand_ids: draft.operandIds,
      mode: draft.mode,
      terms: []
    };
  }
  // mechanics, absolute_sum, srss: the typed terms in authored order, nothing else.
  return {
    ...common,
    terms: draft.terms.map(termPayload)
  };
}

function buildCreateCombinationIntent({
  model,
  draft
}: {
  model: PreviewModel;
  draft: CombinationDraft;
}): EditorOperationIntent {
  return createCombinationIntent({
    model,
    payload: combinationCreatePayload(draft) as Record<string, unknown> & { id: string },
    rationale: draft.rationale,
    sourceNote: `explicit user-entered ${draft.basis} combination; no code/rule combination default inferred`
  });
}

function buildCreatePrimitiveLoadIntent({
  model,
  draft,
  unitCatalogRoute
}: {
  model: PreviewModel;
  draft: PrimitiveLoadDraft;
  unitCatalogRoute: UnitCatalogRoute | null;
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
  const operationToken = `${operationIdToken(draft.loadCaseId)}-${operationIdToken(payload.id)}-primitive`;
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
      unit_validation: primitiveUnitValidationStatus(unitCatalogRoute, unit, dimension),
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
  const termIndex = combination?.terms?.length ?? 0;
  const factor = parseFiniteNumber(draft.factor) ?? 0;
  // A load-case term writes exactly {"load_case", "factor"}; a combination term exactly {"combination", "factor"}.
  const payload = draft.operandKind === "combination" ? { combination: draft.combinationRef, factor } : { load_case: draft.loadCaseId, factor };
  const operationToken = `${operationIdToken(draft.combinationId)}-term-${termIndex}-create`;
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
      unit_validation: DIMENSIONLESS_UNIT_VALIDATION_STATUS,
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
  const operationToken = `${operationIdToken(loadCase.id)}-${field}`;
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
      unit_validation: DIMENSIONLESS_UNIT_VALIDATION_STATUS,
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
  const operationToken = `${operationIdToken(loadCase.id)}-delete`;
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
      field_label: `${loadCaseLabelDisplay(loadCase)} load case`,
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
      unit_validation: DIMENSIONLESS_UNIT_VALIDATION_STATUS,
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
    (combination.terms ?? []).map((term, index) => ({
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
  const operationToken = `${operationIdToken(combination.id)}-basis`;
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
      unit_validation: DIMENSIONLESS_UNIT_VALIDATION_STATUS,
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
  rationale,
  before
}: {
  model: PreviewModel;
  combination: Combination;
  rationale: string;
  /** From the stored record only: `combinationDeleteBefore`. */
  before: string;
}): EditorOperationIntent {
  const operationToken = `${operationIdToken(combination.id)}-delete`;
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
      before,
      after: "not_present",
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered combination deletion; whole combination entity only"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: DIMENSIONLESS_UNIT_VALIDATION_STATUS,
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
  const operationToken = `${operationIdToken(termView.combination.id)}-term-${termView.index}-factor`;
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
      unit_validation: DIMENSIONLESS_UNIT_VALIDATION_STATUS,
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
  rationale,
  before
}: {
  model: PreviewModel;
  termView: CombinationTermView;
  rationale: string;
  /** From the stored term only: `combinationTermDeleteBefore`. */
  before: string;
}): EditorOperationIntent {
  const fieldPath = `terms.${termView.index}`;
  const operationToken = `${operationIdToken(termView.combination.id)}-term-${termView.index}-delete`;
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
      before,
      after: "not_present",
      unit: "none",
      dimension: "dimensionless",
      source_note: "explicit user-entered combination term deletion; existing indexed term only"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: DIMENSIONLESS_UNIT_VALIDATION_STATUS,
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
  unitCatalogRoute,
  rationale
}: {
  model: PreviewModel;
  primitive: PrimitiveLoadView;
  proposedMagnitude: number;
  proposedUnit: string;
  unitCatalogRoute: UnitCatalogRoute | null;
  rationale: string;
}): EditorOperationIntent {
  const fieldPath = `primitive_loads.${primitive.index}.magnitude.value`;
  const operationToken = `${operationIdToken(primitive.loadCase.id)}-${operationIdToken(primitiveId(primitive.load))}-magnitude`;
  const unit = proposedUnit.trim() || primitiveUnit(primitive.load);
  const dimension = primitiveDimension(primitive.load);
  const after = JSON.stringify({ value: proposedMagnitude, unit });
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
      dimension,
      source_note: "unit metadata required"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: primitiveUnitValidationStatus(unitCatalogRoute, unit, dimension),
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

function primitiveUnitValidationStatus(
  route: UnitCatalogRoute | null,
  unit: string,
  dimension: string
): string {
  const normalizedUnit = unit.trim();
  const normalizedDimension = dimension.trim();
  if (!normalizedUnit || normalizedUnit === "TBD" || !normalizedDimension || normalizedDimension === "TBD") {
    return "missing_unit_or_dimension";
  }
  const basis = describeUnitBasis(route, normalizedUnit, normalizedDimension);
  switch (basis.source) {
    case "dec018_catalog_accepted":
      return "dec018_catalog_dimension_match";
    case "dec018_catalog_unreviewed":
      return `dec018_catalog_${basis.review_status ?? "unreviewed"}_dimension_match`;
    case "dec018_catalog_miss":
      return "dec018_catalog_dimension_mismatch";
    case "browser_preview_model_metadata":
      return "model_metadata_unit_dimension_declared_catalog_unavailable_browser_preview";
    case "catalog_loading":
      return "catalog_loading_unit_dimension_declared";
    default:
      return "unit_dimension_status_unknown";
  }
}

function buildDeletePrimitiveLoadIntent({
  model,
  primitive,
  unitCatalogRoute,
  rationale
}: {
  model: PreviewModel;
  primitive: PrimitiveLoadView;
  unitCatalogRoute: UnitCatalogRoute | null;
  rationale: string;
}): EditorOperationIntent {
  const fieldPath = `primitive_loads.${primitive.index}`;
  const operationToken = `${operationIdToken(primitive.loadCase.id)}-${operationIdToken(primitiveId(primitive.load))}-delete`;
  const unit = primitiveUnit(primitive.load);
  const dimension = primitiveDimension(primitive.load);
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
      unit,
      dimension,
      source_note: "explicit user-entered primitive load deletion; existing indexed primitive only"
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: primitiveUnitValidationStatus(unitCatalogRoute, unit, dimension),
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

// A load-case term shows its id as the engine's displays do; a combination term
// names itself as a combination (model document 0.9.0).
function combinationTermLoadCase(term: CombinationTerm): string {
  return combinationTermOperand(term);
}

function combinationTermFactorDisplay(term: CombinationTerm): string {
  return String(term.factor);
}


function combinationBasisFromValue(value: string): CombinationBasis {
  return COMBINATION_BASIS_OPTIONS.find((basis) => basis === value) ?? "mechanics";
}

function combinationDraftSummary(draft: CombinationDraft): string {
  return `basis=${draft.basis}; ${combinationDraftExpression(draft)}`;
}

function combinationDraftExpression(draft: CombinationDraft): string {
  if (draft.basis === "result_state_subtraction") {
    return `minuend=${draft.minuendId || "TBD"}; subtrahend=${draft.subtrahendId || "TBD"}${draft.operatingDifference ? `; difference_basis=${OPERATING_DIFFERENCE_BASIS}` : ""}`;
  }
  if (draft.basis === "range_envelope") {
    return `mode=${draft.mode || "TBD"}; operands=${draft.operandIds.length ? draft.operandIds.join(", ") : "TBD"}`;
  }
  return draft.terms.length === 1 ? `term=${termDraftText(draft.terms[0])}` : `terms=${draft.terms.map(termDraftText).join("; ") || "none"}`;
}

function combinationDraftHelpText(model: PreviewModel, draft: CombinationDraft): string {
  const basis = draft.basis;
  const missing = combinationDraftMissing(model, draft);
  if (basis === "result_state_subtraction") {
    const base = "select distinct minuend/subtrahend load cases and complete id/label/provenance/rationale to queue a subtraction combination";
    return missing ? `${base}; ${missing}` : base;
  }
  if (basis === "range_envelope") {
    const base = "select at least one operand load case and a mode and complete id/label/provenance/rationale to queue a range envelope combination";
    return missing ? `${base}; ${missing}` : base;
  }
  const base = basis === "mechanics"
    ? "complete id/label/load case/finite factor/provenance/rationale to queue a mechanics combination"
    : `complete id/label, at least one term with a finite factor and no repeated operand, and provenance/rationale to queue an ${basis} combination`;
  const problem = termsDraftProblem(model, draft.terms, draft.id.trim());
  return problem ? `${base}; ${problem}` : base;
}

/** What a subtraction or envelope draft still needs the person to choose, or null. */
function combinationDraftMissing(model: PreviewModel, draft: CombinationDraft): string | null {
  if (draft.basis === "result_state_subtraction") {
    if (!draft.minuendId) return "not chosen: the minuend";
    if (!draft.subtrahendId) return "not chosen: the subtrahend";
    if (draft.minuendId === draft.subtrahendId) return "the minuend and subtrahend are the same";
    return null;
  }
  if (draft.basis === "range_envelope") {
    if (!draft.operandIds.length) return "not chosen: any operand";
    if (!draft.mode) return "not chosen: the mode";
    return null;
  }
  return termsDraftProblem(model, draft.terms, draft.id.trim());
}

/** The solve's known gates for a terms combination being drafted, or null. */
function combinationTermsGateNote(draft: CombinationDraft, constantEffort: readonly string[], nonlinear: readonly string[]): string | null {
  if (isUnsignedBasis(draft.basis)) {
    const loadCaseTerms = draft.terms.filter((term) => term.kind === "load_case" && term.ref).map((term) => term.ref);
    return [
      UNSIGNED_ROW_TEXT,
      UNSIGNED_GATE_TEXT,
      loadCaseTerms.length ? `This draft names load case${loadCaseTerms.length > 1 ? "s" : ""} ${loadCaseTerms.join(", ")} directly (κ = 1): where a fixed-load source applies, the solve withholds it.` : "",
      constantEffort.length ? `This model carries constant-effort support${constantEffort.length > 1 ? "s" : ""} ${constantEffort.join(", ")}; the solve decides whether it consumes ${constantEffort.length > 1 ? "them" : "it"}.` : ""
    ].filter(Boolean).join(" ");
  }
  if (draft.basis === "mechanics" && nonlinear.length) return `This model carries nonlinear support${nonlinear.length > 1 ? "s" : ""} ${nonlinear.join(", ")}. ${NONLINEAR_MECHANICS_TEXT}`;
  return null;
}

/** A stored combination's bare references, as authored. */
function storedReferenceIds(combination: Combination): string[] {
  return [combination.minuend_id, combination.subtrahend_id, ...(combination.operand_ids ?? [])].filter((ref): ref is string => typeof ref === "string");
}

/** Why combination `targetId` cannot take a term naming combination `operandId`, or null. */
function combinationTermOperandRefusal(model: PreviewModel, targetId: string, operandId: string): string | null {
  const refusal = combinationOperandRefusal(model, targetId, operandId);
  if (refusal) return refusal;
  const target = model.combinations?.find((combination) => combination.id === targetId);
  if (target?.terms?.some((term) => term.combination === operandId)) return `${operandId} is already a term of ${targetId} (one factor per operand)`;
  return null;
}

function combinationTermDraftProblem(model: PreviewModel, draft: CombinationTermDraft): string | null {
  if (draft.operandKind === "combination") {
    if (!draft.combinationRef) return "not chosen: the combination the new term names";
    const refusal = combinationTermOperandRefusal(model, draft.combinationId, draft.combinationRef);
    if (refusal) return refusal;
  } else if (!draft.loadCaseId) {
    return "not chosen: the load case the new term names";
  }
  if (!draft.factor.trim()) return "not entered: the factor";
  return null;
}

/** The paths that still name a combination, shown before a delete is attempted. */
function combinationDeleteReferenceText(model: PreviewModel, combinationId: string, paths: readonly string[]): string {
  const referencers = [...new Set(paths.map((path) => path.slice(0, path.lastIndexOf(path.includes(".operand_ids.") ? ".operand_ids." : path.includes(".terms.") ? ".terms." : "."))))];
  const fromTemplate = referencers.some((id) => model.combinations?.find((combination) => combination.id === id)?.provenance === SEISMIC_TEMPLATE_PROVENANCE);
  return `Named by ${paths.join(", ")}. The operation engine refuses deleting it while another combination names it (${DELETE_REFERENCED_CODE}); nothing is deleted with it. ` +
    (fromTemplate
      ? "This is part of a three-axis static seismic template set: delete top-down: the unsigned combination first."
      : `Delete top-down: delete or change ${referencers.join(", ")} first.`);
}

/** A before value built only from the stored value: `historical` when it applies, else the
 * stored value's canonical JSON from the wasm engine (null until it is ready). Null as well
 * when the historical display needs the engine's number text and the engine is not loaded:
 * then no delete is built (`unavailable`). */
function useStoredBefore<T>(stored: T | null, historical: (value: T) => string | null): { before: string | null; unavailable: boolean } {
  const [canonical, setCanonical] = useState<{ stored: T; value: string } | null>(null);
  let sync: string | null = null;
  let unavailable = false;
  if (stored !== null) {
    try {
      sync = historical(stored);
    } catch (error) {
      if (!(error instanceof NumberDisplayUnavailable)) throw error;
      unavailable = true;
    }
  }
  const needsCanonical = stored !== null && sync === null && !unavailable;
  useEffect(() => {
    if (!needsCanonical || stored === null) return;
    let live = true;
    void canonicalJsonString(stored).then((value) => { if (live) setCanonical({ stored, value }); }).catch(() => undefined);
    return () => { live = false; };
  }, [stored, needsCanonical]);
  if (stored === null || unavailable) return { before: null, unavailable };
  return { before: sync ?? (canonical?.stored === stored ? canonical.value : null), unavailable };
}

function combinationRowExpression(combination: Combination): string {
  if (combination.basis === "result_state_subtraction") {
    return `minuend=${combination.minuend_id ?? "TBD"}; subtrahend=${combination.subtrahend_id ?? "TBD"}`;
  }
  if (combination.basis === "range_envelope") {
    return `mode=${combination.mode ?? "TBD"}; operands=${
      combination.operand_ids?.length ? combination.operand_ids.join(", ") : "TBD"
    }`;
  }
  return `terms=${combinationTermsText(combination) ?? "absent"}`;
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
  // "" is an unselected default: offer a placeholder and the catalog, nothing in its place.
  const fallback = fallbackSymbol === ""
    ? { symbol: "", label: "Choose a unit" }
    : { symbol: fallbackSymbol, label: `${fallbackSymbol} (model metadata)` };
  // An undeclared project unit (TBD) has no default; with the catalog loaded the
  // field still offers its catalog units to choose from (W-17 §2.4: the moment
  // default is project.units.moment, never composed).
  if (route?.route !== "tauri_unit_catalog" || !Array.isArray(route.catalog?.entries)) return [fallback];
  const options = acceptedUnits(route.catalog)
    .filter((entry) => unitEntryMatchesDimension(entry, dimensionId))
    .map((entry) => ({
      symbol: entry.symbol,
      label: `${entry.symbol}${entry.canonical ? " (DEC-018 canonical)" : " (DEC-018 display)"}`
    }));
  if (!options.some((option) => option.symbol === fallbackSymbol)) options.unshift(fallback);
  return options;
}

function primitiveLoadDraftTargetDisplay(draft: PrimitiveLoadDraft): string {
  if (primitiveLoadUsesSupportTarget(draft.category)) return draft.targetSupport || "support:TBD";
  return primitiveLoadUsesPipeTarget(draft.category) ? draft.targetPipe || "pipe:TBD" : draft.targetNode || "node:TBD";
}

function primitiveLoadFieldLabel(category: PrimitiveLoadCategory): string {
  if (category === "distributed_force") return "Distributed force primitive load";
  if (category === "concentrated_moment") return "Concentrated moment primitive load";
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
  if (category === "thermal") {
    return "explicit user-entered temperature-interval primitive; no pressure or imposed displacements inferred";
  }
  if (category === "imposed_displacement") {
    return "explicit user-entered support-target imposed displacement; no support coordinate policy or hidden restraint defaults inferred";
  }
  return "explicit user-entered concentrated node force; no distributed loads, moments, or imposed displacements inferred";
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

/** A test-id token only (display); operation ids use `operationIdToken`. */
function safeToken(value: string): string {
  return value.replace(/[^A-Za-z0-9_.:-]+/g, "-");
}

/** The Load Case Manager's create paths (default drafts and intent builders),
 * for the desktop emission golden that drives each authoring path. */
export const loadCaseAuthoringPaths = {
  defaultLoadCaseDraft,
  buildCreateLoadCaseIntent,
  defaultPrimitiveLoadDraft,
  buildCreatePrimitiveLoadIntent,
  defaultCombinationDraft,
  buildCreateCombinationIntent
};
