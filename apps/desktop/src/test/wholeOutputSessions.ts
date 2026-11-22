// Maintained sessions for the whole-output tests of the governed links: the Report Packet's
// "Local JSON" link and the preview handoff summary's link. Invented content only.
import { screen } from "@testing-library/react";
import { expect } from "vitest";
import demoModel from "../../../../fixtures/product_preview/invented_demo_model.json";
import demoResult from "../../../../fixtures/product_preview/invented_demo_result_preview_physics_1_sparse.json";
import { buildAnalysisRunPreview } from "../services/previewService";
import { buildCurrentSessionInputManifest } from "../services/inputManifestService";
import previewText from "../../../../fixtures/results/preview_physics_connected_sparse.json?raw";
import previewModelJson from "../../../../fixtures/model_operations/precision_connected_ui_model.json";
import everySlotJson from "../../../../fixtures/model_document/invented_every_slot_model.json";
import { RULE_PACK_REF_UNRESOLVED } from "../services/rulePackRefDiagnostics";
import type { AnalysisRunEnvelope, MechanicsResult, PreviewModel, RulePackReference } from "../types";

const SOLVER = {
  solver_name: "open_pipe_stress_product_physics",
  solver_version: "0.2.0",
  solver_build_ref: "fixture",
  solver_mode: "sparse_interactive",
  settings: {}
};

/** The maintained demo session: the bundled invented model, its committed solve and the run record. */
export async function demoSession(model: PreviewModel = structuredClone(demoModel) as unknown as PreviewModel) {
  const result = structuredClone(demoResult) as unknown as MechanicsResult;
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: SOLVER, active_rule_packs: [], external_assets: [] });
  const analysisRun: AnalysisRunEnvelope = await buildAnalysisRunPreview(result, { inputManifest });
  return { model, result, analysisRun };
}

export function panelProps(session: Awaited<ReturnType<typeof demoSession>>) {
  return {
    model: session.model,
    knowledge: null,
    result: session.result,
    analysisRun: session.analysisRun,
    comparison: null,
    editorIntents: [],
    projectOperation: "idle",
    projectSummary: null,
    proposal: null,
    selectedReviewTarget: null,
    storageCapability: null
  };
}

/** The bytes a data link holds. */
export function linkText(link: HTMLElement): string {
  const href = link.getAttribute("href") ?? "";
  expect(href.startsWith("data:application/json;charset=utf-8,")).toBe(true);
  return decodeURIComponent(href.slice(href.indexOf(",") + 1));
}

/** The demo model with component C-130's weight replaced by `weight`. */
export function withWeight(weight: (current: Record<string, unknown>) => unknown): PreviewModel {
  const model = structuredClone(demoModel) as unknown as PreviewModel;
  const component = model.components.find((item) => item.id === "component:C-130")!;
  const geometry = component.geometry as unknown as Record<string, unknown>;
  geometry.weight = weight(geometry.weight as Record<string, unknown>);
  return model;
}

/** The maintained session with `refs` as the model's rule-pack references. */
export async function handoffSession(refs: RulePackReference[]) {
  const model = { ...(structuredClone(previewModelJson) as unknown as PreviewModel), rule_pack_refs: refs };
  const result = JSON.parse(previewText) as MechanicsResult;
  const inputManifest = await buildCurrentSessionInputManifest({ model, solver: SOLVER, active_rule_packs: [], external_assets: [] });
  const rulePackExclusions = refs.map((ref) => ({ rule_pack_id: ref.rule_pack_id, rule_pack_version: ref.rule_pack_version, code: RULE_PACK_REF_UNRESOLVED }) as const);
  const analysisRun = await buildAnalysisRunPreview(result, { inputManifest, rulePackExclusions });
  return { model, result, analysisRun, knowledge: null, comparison: null, editorIntents: [], proposal: null, selectedReviewTarget: null };
}

export const everySlotRefs = () => structuredClone((everySlotJson as unknown as PreviewModel).rule_pack_refs!) as RulePackReference[];

export function handoffLinkText(): string {
  const href = screen.getByTestId("handoff-export-link").getAttribute("href") ?? "";
  expect(href.startsWith("data:application/json;charset=utf-8,")).toBe(true);
  return decodeURIComponent(href.slice(href.indexOf(",") + 1));
}

/** Every downloadable data link in the document. */
export const downloadableLinks = () =>
  Array.from(document.querySelectorAll("a[href]")).filter((anchor) => (anchor.getAttribute("href") ?? "").startsWith("data:"));
