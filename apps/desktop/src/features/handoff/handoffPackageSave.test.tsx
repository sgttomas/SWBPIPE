import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { createHash } from "node:crypto";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import demoModel from "../../../../../fixtures/product_preview/invented_demo_model.json";
import everySlotModel from "../../../../../fixtures/model_document/invented_every_slot_model.json";
import { App } from "../../App";
import { canonicalJsonString, computeModelHash } from "../../services/hashService";
import type { ModelStateSummary, StateLabels } from "../../services/modelStateService";
import type { LocalProjectEnvelope, ModelHashEvidence, PreviewModel } from "../../types";
import { isMenuCommandId } from "../workspace/menuCommands";
import { MODEL_DOC_SAVE_INTENT_REQUIRED, prepareModelDocumentSave } from "../model-file/modelDocumentSave";
import {
  HANDOFF_NEW_STATE_LABEL,
  HANDOFF_PACKAGE_ROUTE_ID,
  HANDOFF_SAVE_DESKTOP_ONLY,
  HANDOFF_SAVE_INTENT_LABEL,
  HANDOFF_SAVE_RECEIPT_INVALID,
  HANDOFF_SAVE_ROUTE,
  defaultHandoffStateName,
  findBasisState,
  handoffBasis,
  handoffSaveMessage,
  readHandoffOutcome,
  saveHandoffPackage,
  type HandoffSaveArgs
} from "./handoffPackageSave";

// DEL-15-01 W-6b: File > Save Handoff…. The native command's own checks
// (admission, basis state, manifest, publication, verifier) are the Rust tests
// in src-tauri/src/handoff_package.rs; these are the webview's adapters, the
// intent and basis controls and the menu wiring.

afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

const DEMO = demoModel as unknown as PreviewModel;
const BASE = "sha256:invented-stored-base";

function setTauriRuntime() {
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
}

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

async function hashOf(model: unknown): Promise<ModelHashEvidence> {
  return (await computeModelHash(model as PreviewModel))!;
}

function state(overrides: Partial<ModelStateSummary>): ModelStateSummary {
  return {
    state_name: "Invented state", tags: [], notes: [], assumptions: [],
    project_id: DEMO.project.id, state_id: "state:00000000000000000000000000000001", state_kind: "design_snapshot",
    created_at: "2026-10-24T10:00:00.000Z", parent_state_id: null, model_hash: "sha256:" + "0".repeat(64),
    record_hash: "record", labels_updated_at: "2026-10-24T10:00:00.000Z", child_state_ids: [], analysis_status: ["HUMAN_REVIEW_REQUIRED"],
    ...overrides
  };
}

describe("Save Handoff…: adapters", () => {
  it("names a new basis state \"Handoff <local date>\"", () => {
    expect(defaultHandoffStateName(new Date(2026, 9, 24, 23, 30))).toBe("Handoff 2026-10-24");
    expect(defaultHandoffStateName(new Date(2027, 0, 5))).toBe("Handoff 2027-01-05");
  });

  it("finds the project's saved state of exactly this model hash, else asks for a new one", async () => {
    const hash = (await hashOf(DEMO)).value;
    const states = [state({ model_hash: "sha256:" + "1".repeat(64) }), state({ state_id: "state:match", model_hash: hash, state_kind: "comparison_basis" }),
      state({ state_id: "state:other-project", project_id: "project:other", model_hash: hash })];
    expect(findBasisState(states, DEMO.project.id, hash)?.state_id).toBe("state:match");
    expect(handoffBasis(states, DEMO.project.id, hash)).toEqual({ kind: "existing", state: states[1] });
    expect(handoffBasis(states.slice(0, 1), DEMO.project.id, hash, new Date(2026, 9, 24))).toEqual({ kind: "new", defaultName: "Handoff 2026-10-24" });
    expect(findBasisState([states[2]], DEMO.project.id, hash)).toBeNull();
  });

  it("admits the model bytes as W-6a does, under the governed handoff route", async () => {
    const preparation = await prepareModelDocumentSave({ model: DEMO, modelHash: await hashOf(DEMO), explicitLocalPrivateIntent: true, route: HANDOFF_SAVE_ROUTE });
    if (preparation.status !== "ready") throw new Error("demo refused");
    expect(preparation.args.canonicalText).toBe(await canonicalJsonString(DEMO));
    expect(preparation.args.screening).toMatchObject({
      route_id: HANDOFF_PACKAGE_ROUTE_ID, export_context: "local_private", explicit_local_private_intent: true, lossless_required: true, exact_payload_match: true,
      payload_sha256: `sha256:${createHash("sha256").update(preparation.args.canonicalText).digest("hex")}`
    });
    expect(preparation.args.localFirstEvidence).toMatchObject({ route_id: HANDOFF_PACKAGE_ROUTE_ID, blocked: false, explicit_local_private_intent: true });
    const refused = await prepareModelDocumentSave({ model: DEMO, modelHash: await hashOf(DEMO), explicitLocalPrivateIntent: false, route: HANDOFF_SAVE_ROUTE });
    expect(refused.status === "refused" && refused.diagnostics.map((item) => [item.code, item.message])).toEqual([
      [MODEL_DOC_SAVE_INTENT_REQUIRED, `Saving the model document needs the explicit choice "${HANDOFF_SAVE_INTENT_LABEL}".`]
    ]);
    // A quarantined record refuses under this route too (W-6a's screening).
    const quarantined = structuredClone(everySlotModel) as unknown as Record<string, any>;
    quarantined.materials[0].provenance_record.review_status = "quarantined";
    const blocked = await prepareModelDocumentSave({ model: quarantined as unknown as PreviewModel, modelHash: await hashOf(quarantined), explicitLocalPrivateIntent: true, route: HANDOFF_SAVE_ROUTE });
    expect(blocked.status === "refused" && blocked.diagnostics.map((item) => [item.code, item.path])).toEqual([["PROTECTED_CONTENT_BLOCKED", "materials[0].provenance_record"]]);
  });

  it("reads the native outcome strictly and checks the receipt against the request", async () => {
    const hash = (await hashOf(DEMO)).value;
    const saved = { outcome: "saved", package_name: "invented.swbhandoff", model_hash: hash, export_hash: "sha256:" + "a".repeat(64),
      basis_state_id: "state:x", replaced_existing: false, durability: "durable", warnings: [] };
    expect(readHandoffOutcome(saved)).toEqual(saved);
    expect(readHandoffOutcome({ outcome: "cancelled" })).toEqual({ outcome: "cancelled" });
    for (const garbage of [null, { outcome: "saved" }, { ...saved, export_hash: "sha256:short" }, { ...saved, warnings: [{ code: 1 }] }]) {
      expect(readHandoffOutcome(garbage)).toMatchObject({ outcome: "refused", diagnostics: [{ code: HANDOFF_SAVE_RECEIPT_INVALID }] });
    }
    const args = { canonicalText: "{}", claimedModelHash: hash, basisStateId: "state:x", packageLabel: "Invented" } as unknown as HandoffSaveArgs;
    expect(await saveHandoffPackage(args)).toMatchObject({ outcome: "refused", diagnostics: [{ code: HANDOFF_SAVE_DESKTOP_ONLY }] });
    setTauriRuntime();
    invokeMock.mockResolvedValueOnce(saved).mockResolvedValueOnce({ ...saved, basis_state_id: "state:other" });
    expect(await saveHandoffPackage(args)).toEqual(saved);
    expect(await saveHandoffPackage(args)).toMatchObject({ outcome: "refused", diagnostics: [{ code: HANDOFF_SAVE_RECEIPT_INVALID }] });
  });

  it("states every outcome on one status line", () => {
    const saved = { outcome: "saved" as const, package_name: "p.swbhandoff", model_hash: "sha256:m", export_hash: "sha256:e", basis_state_id: "state:s",
      replaced_existing: true, durability: "durable", warnings: [{ code: "MODEL-LIBRARY-REF-UNRESOLVED", path: "materials[0].provenance_record.library_ref", message: "m" }] };
    expect(handoffSaveMessage(saved)).toBe("Saved handoff package p.swbhandoff (model hash sha256:m; export hash sha256:e; basis state state:s). Warnings: MODEL-LIBRARY-REF-UNRESOLVED at materials[0].provenance_record.library_ref.");
    expect(handoffSaveMessage({ ...saved, warnings: [], durability: "saved_durability_uncertain" })).toContain("durability is uncertain");
    expect(handoffSaveMessage({ outcome: "cancelled" }, "Saved model state \"H\" (state:s) as the handoff basis; it stays saved."))
      .toBe("Save handoff cancelled; nothing was written. Saved model state \"H\" (state:s) as the handoff basis; it stays saved.");
    expect(handoffSaveMessage({ outcome: "refused", diagnostics: [{ code: "HANDOFF-UNIT-UNDERIVABLE", path: "project.units.stress", message: "m" }] }))
      .toBe("Save handoff refused: HANDOFF-UNIT-UNDERIVABLE at project.units.stress: m");
  });
});

type SaveStateRequest = { project_id: string; expected_base: string; model_hash: string; state_kind: string; parent_state_id: string | null; labels: StateLabels };

/** A fake of the native commands the flow uses: the store's states, Save Local
 * Project, Save Model State and the handoff save. */
function nativeHost(options: { states?: ModelStateSummary[]; handoff?: (args: HandoffSaveArgs) => unknown } = {}) {
  const store = { states: [...(options.states ?? [])], projectStored: false, sequence: 0 };
  invokeMock.mockImplementation((command: string, raw?: Record<string, unknown>) => {
    const args = raw ?? {};
    switch (command) {
      case "sync_native_shell_state":
        return Promise.resolve(null);
      case "check_model_library_refs":
        return Promise.resolve([]);
      case "list_model_states":
        return Promise.resolve(store.states.filter((item) => item.project_id === args.projectId));
      case "find_local_project":
        return Promise.resolve(null);
      case "save_local_project": {
        const request = args.request as { model: PreviewModel; model_hash: unknown };
        store.projectStored = true;
        return Promise.resolve({
          summary: { project_id: request.model.project.id, project_name: request.model.project.name, message: "Saved current project model snapshot without external file copies." },
          model: request.model, editor_intents: [], proposal: null, selected_review_target: null, mechanics_result: null,
          analysis_run: null, model_hash: request.model_hash, project_envelope_hash: { value: BASE }, model_document_version: null
        } as unknown as LocalProjectEnvelope);
      }
      case "save_model_state": {
        const request = args.request as SaveStateRequest;
        if (!store.projectStored || request.expected_base !== BASE) return Promise.reject(`MODEL-STATE-PROJECT-MISSING: project \`${request.project_id}\` has no stored local project.`);
        const created = state({ ...request.labels, state_id: `state:${String(++store.sequence).padStart(32, "0")}`, state_kind: request.state_kind as ModelStateSummary["state_kind"], model_hash: request.model_hash, parent_state_id: request.parent_state_id });
        store.states.unshift(created);
        return Promise.resolve({ outcome: "saved", state: created, message: "Saved." });
      }
      case "save_handoff_package": {
        const handoff = args as unknown as HandoffSaveArgs;
        return Promise.resolve(options.handoff ? options.handoff(handoff) : {
          outcome: "saved", package_name: "invented-demo-loop-01.swbhandoff", model_hash: handoff.claimedModelHash, export_hash: "sha256:" + "e".repeat(64),
          basis_state_id: handoff.basisStateId, replaced_existing: false, durability: "durable", warnings: []
        });
      }
      default:
        return Promise.reject(new Error(`Unexpected command ${command}`));
    }
  });
  return store;
}

const calls = (command: string) => invokeMock.mock.calls.filter(([name]) => name === command).map(([, args]) => args as Record<string, unknown>);
const writes = () => invokeMock.mock.calls.map(([name]) => name as string).filter((name) => ["save_local_project", "save_model_state", "save_handoff_package"].includes(name));
const projectMessage = () => screen.getByTestId("local-project-message");

async function renderApp() {
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  await screen.findByTestId(`tree-row-project-${encodeURIComponent(DEMO.project.id)}`);
}

describe("File > Save Handoff…", () => {
  it("is a File menu command whenever a model is open; the browser preview refuses it by name", async () => {
    expect(isMenuCommandId("file.save-handoff")).toBe(true);
    await renderApp();
    fireEvent.click(screen.getByTestId("menu-file"));
    const item = await screen.findByTestId("menu-item-file.save-handoff");
    expect(item).toHaveTextContent("Save Handoff…");
    expect(item).toBeEnabled();
    fireEvent.click(item);
    await waitFor(() => expect(projectMessage()).toHaveTextContent(`Save handoff refused: ${HANDOFF_SAVE_DESKTOP_ONLY}`));
    expect(screen.queryByTestId("handoff-save-dialog")).not.toBeInTheDocument();
  });

  it("with a saved state of this model: names it, asks for the explicit intent, then saves through the native command", async () => {
    await renderApp();
    setTauriRuntime();
    const hash = (await hashOf(DEMO)).value;
    const basis = state({ state_id: "state:invented-basis", model_hash: hash, state_name: "Invented basis" });
    nativeHost({ states: [basis] });
    act(() => nativeMenuCommand("file.save-handoff"));
    const dialog = await screen.findByTestId("handoff-save-dialog");
    expect(within(dialog).getByTestId("handoff-save-basis")).toHaveTextContent(`Basis state: "Invented basis" (state:invented-basis)`);
    expect(within(dialog).queryByTestId("handoff-save-state-confirm")).not.toBeInTheDocument();
    expect(within(dialog).getByText(HANDOFF_SAVE_INTENT_LABEL)).toBeInTheDocument();
    expect(within(dialog).getByTestId("handoff-save-intent")).not.toBeChecked();
    expect(within(dialog).getByTestId("handoff-save-label")).toHaveValue(DEMO.project.name);
    expect(within(dialog).getByTestId("handoff-save-confirm")).toBeDisabled();
    expect(document.activeElement).toBe(within(dialog).getByTestId("handoff-save-cancel"));
    expect(screen.getByTestId("desktop-preview-shell")).toHaveAttribute("inert");
    fireEvent.click(within(dialog).getByTestId("handoff-save-intent"));
    // An empty label holds the save back.
    fireEvent.change(within(dialog).getByTestId("handoff-save-label"), { target: { value: "  " } });
    expect(within(dialog).getByTestId("handoff-save-confirm")).toBeDisabled();
    fireEvent.change(within(dialog).getByTestId("handoff-save-label"), { target: { value: "Invented handoff label" } });
    fireEvent.click(within(dialog).getByTestId("handoff-save-confirm"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent(
      `Saved handoff package invented-demo-loop-01.swbhandoff (model hash ${hash}; export hash sha256:${"e".repeat(64)}; basis state state:invented-basis).`
    ));
    expect(writes()).toEqual(["save_handoff_package"]);
    const sent = calls("save_handoff_package")[0] as unknown as HandoffSaveArgs;
    expect(sent.canonicalText).toBe(await canonicalJsonString(DEMO));
    expect(sent.claimedModelHash).toBe(hash);
    expect(sent.basisStateId).toBe("state:invented-basis");
    expect(sent.packageLabel).toBe("Invented handoff label");
    expect(sent.screening.route_id).toBe(HANDOFF_PACKAGE_ROUTE_ID);
    expect(sent.localFirstEvidence.route_id).toBe(HANDOFF_PACKAGE_ROUTE_ID);
  });

  it("without one: the new handoff_basis state is confirmed in the dialog, saved through S1's flow first, then the package", async () => {
    await renderApp();
    setTauriRuntime();
    const store = nativeHost({ handoff: () => ({ outcome: "cancelled" }) });
    act(() => nativeMenuCommand("file.save-handoff"));
    const dialog = await screen.findByTestId("handoff-save-dialog");
    expect(within(dialog).getByTestId("handoff-save-basis")).toHaveTextContent("This model is not a saved model state yet.");
    expect(within(dialog).getByTestId("handoff-save-state-name")).toHaveValue(defaultHandoffStateName());
    expect(within(dialog).getByText(HANDOFF_NEW_STATE_LABEL)).toBeInTheDocument();
    fireEvent.click(within(dialog).getByTestId("handoff-save-intent"));
    // The intent alone is not enough: the new state is confirmed by its own control.
    expect(within(dialog).getByTestId("handoff-save-confirm")).toBeDisabled();
    fireEvent.click(within(dialog).getByTestId("handoff-save-state-confirm"));
    fireEvent.change(within(dialog).getByTestId("handoff-save-state-name"), { target: { value: "" } });
    expect(within(dialog).getByTestId("handoff-save-confirm")).toBeDisabled();
    fireEvent.change(within(dialog).getByTestId("handoff-save-state-name"), { target: { value: "Handoff invented" } });
    fireEvent.click(within(dialog).getByTestId("handoff-save-confirm"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent(
      `Save handoff cancelled; nothing was written. Saved model state "Handoff invented" (state:${"1".padStart(32, "0")}) as the handoff basis; it stays saved.`
    ));
    // The project row first (S1's flow), then the state, then the package.
    expect(writes()).toEqual(["save_local_project", "save_model_state", "save_handoff_package"]);
    const request = calls("save_model_state")[0].request as SaveStateRequest;
    expect(request).toMatchObject({ project_id: DEMO.project.id, expected_base: BASE, state_kind: "handoff_basis", model_hash: (await hashOf(DEMO)).value });
    expect(request.labels).toEqual({ state_name: "Handoff invented", tags: [], notes: [], assumptions: [] });
    expect((calls("save_handoff_package")[0] as unknown as HandoffSaveArgs).basisStateId).toBe(store.states[0].state_id);
    // The state left by the cancelled publication is the basis of the next save.
    act(() => nativeMenuCommand("file.save-handoff"));
    const again = await screen.findByTestId("handoff-save-dialog");
    expect(within(again).getByTestId("handoff-save-basis")).toHaveTextContent(`Basis state: "Handoff invented"`);
  });

  it("Cancel and Escape send nothing", async () => {
    await renderApp();
    setTauriRuntime();
    nativeHost();
    act(() => nativeMenuCommand("file.save-handoff"));
    fireEvent.click(within(await screen.findByTestId("handoff-save-dialog")).getByTestId("handoff-save-cancel"));
    expect(screen.queryByTestId("handoff-save-dialog")).not.toBeInTheDocument();
    act(() => nativeMenuCommand("file.save-handoff"));
    await screen.findByTestId("handoff-save-dialog");
    fireEvent.keyDown(window, { key: "Escape" });
    await waitFor(() => expect(screen.queryByTestId("handoff-save-dialog")).not.toBeInTheDocument());
    expect(writes()).toEqual([]);
  });
});
