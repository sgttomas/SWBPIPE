import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

import { App } from "./App";
import * as hashService from "./services/hashService";
import { loadPreviewModel, runPreviewMechanics } from "./services/previewService";
import { applyModelOperation } from "./services/operationService";
import type {
  EditorOperationIntent,
  LocalProjectEnvelope,
  ModelHashEvidence,
  OperationOutcome,
  ProjectEnvelopeHashEvidence,
  PreviewModel,
} from "./types";

// Slice B2F (project-handler repairs). The setup below is copied from
// src/App.test.tsx, kept to what these cases need: the hoisted invoke mock,
// the Tauri core mock, the afterEach reset, `deferred`, `nativeMenuCommand`,
// an `inventedOpenEnvelope` equivalent, `openWorkspaceSection` and
// `runMechanicsButton`. Nothing is imported from the safety net.

// Slice B3: the status bar carries the chips of specification §5.4. A chip's face is
// "Domain · Label" from statusLabels.ts; the recorded token is its tooltip (and, on a
// click, its popover). These helpers hold both, where a pill's body used to show the token.
function expectStatusChip(testId: string, token: string, face: string) {
  const chip = screen.getByTestId(testId);
  expect(chip).toHaveAttribute("title", token);
  expect(chip).toHaveTextContent(face);
}

afterEach(() => {
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

function deferred<T>() {
  let resolve!: (value: T) => void;
  let reject!: (error: unknown) => void;
  const promise = new Promise<T>((onResolve, onReject) => {
    resolve = onResolve;
    reject = onReject;
  });
  return { promise, reject, resolve };
}

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

function setTauriRuntime(present: boolean) {
  if (present) (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  else delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
}

function inventedOpenEnvelope(model: PreviewModel): LocalProjectEnvelope {
  const openedModel = JSON.parse(JSON.stringify(model)) as PreviewModel;
  openedModel.project.id = "project:invented-handler-open";
  openedModel.project.name = "Invented handler open";
  return {
    summary: {
      project_id: openedModel.project.id,
      project_name: openedModel.project.name,
      database_path: "/invented/local/handler-open.sqlite",
      storage_mode: "local_sqlite",
      migration_status: "current",
      migration_framework: "application_service_separate_db_and_product_schema",
      store_schema_version: 1,
      store_schema_target_version: 1,
      migrations_applied_on_open: [],
      fts_indexed: true,
      copied_external_files: false,
      editor_intent_count: 0,
      proposal_count: 0,
      selected_review_target_count: 0,
      selected_review_target_ref: "none",
      persisted_mechanics_result_count: 0,
      persisted_analysis_run_count: 0,
      persisted_analysis_run_ref: "none",
      persisted_model_hash_count: 0,
      persisted_model_hash_ref: "none",
      persisted_project_envelope_hash_count: 0,
      persisted_project_envelope_hash_ref: "none",
      unit_round_trip_status: "not_checked_in_invented_test",
      unit_round_trip_checked_ref_count: 0,
      unit_round_trip_signature: "none",
      message: "Opened invented handler project.",
    },
    model: openedModel,
    editor_intents: [],
    proposal: null,
    selected_review_target: null,
    mechanics_result: null,
    analysis_run: null,
    model_hash: null,
    project_envelope_hash: null,
    model_document_migration: null,
    model_migration_ledger: [],
  };
}

// A stored model hash that cannot match the restored model, so the open
// records `mismatch_review_required` (deriveModelHashIntegrity).
function mismatchedModelHash(model: PreviewModel): ModelHashEvidence {
  return {
    algorithm: "sha256",
    canonicalization: "rfc8785_jcs",
    payload_scope: "model_payload",
    payload_ref: model.project.id,
    value: `sha256:${"f".repeat(64)}`,
    hash_status: "computed_local_preview",
  };
}

type AppWorkspaceSection = "solve" | "project" | "evidence";

function openWorkspaceSection(sectionId: AppWorkspaceSection) {
  let section = screen.queryByTestId(`workspace-section-${sectionId}`);
  if (!section || section.classList.contains("inactive")) {
    if (screen.queryByTestId("menu-view")) {
      fireEvent.click(screen.getByTestId("menu-view"));
      fireEvent.click(screen.getByTestId(`menu-item-view.section.${sectionId}`));
    } else {
      // The in-DOM menu bar is absent under the native runtime flag.
      act(() => nativeMenuCommand(`view.section.${sectionId}`));
    }
    section = screen.getByTestId(`workspace-section-${sectionId}`);
  }
  expect(section).not.toHaveClass("inactive");
  return section;
}

async function runMechanicsButton() {
  await screen.findByTestId("workspace-toolbar");
  return within(openWorkspaceSection("solve")).getByTestId("run-mechanics-preview");
}

const PROJECT_CONTROL_NAMES = [/^Create local$/, /^New blank$/, /^Open local$/, /^List local$/, /^Save local$/];

function projectControls() {
  return PROJECT_CONTROL_NAMES.map((name) => screen.getByRole("button", { name }));
}

function issueCountShown(): number {
  const text = screen.getByTestId("issues-drawer-toggle").textContent ?? "";
  const match = /(\d+) Issues/.exec(text);
  if (!match) throw new Error(`Issue count not shown: ${text}`);
  return Number(match[1]);
}

function projectMessage() {
  return screen.getByTestId("local-project-message");
}

function modelHashLine() {
  return within(openWorkspaceSection("project")).getByTestId("project-validation-model-hash");
}

async function openProjectWithRecordedMismatch(model: PreviewModel) {
  const envelope = inventedOpenEnvelope(model);
  envelope.model_hash = mismatchedModelHash(envelope.model);
  invokeMock.mockImplementation((command: string) =>
    command === "open_local_project" ? Promise.resolve(envelope) : Promise.reject(new Error(`Unexpected command ${command}`)),
  );
  setTauriRuntime(true);
  act(() => nativeMenuCommand("file.open-local"));
  await waitFor(() => expect(projectMessage()).toHaveTextContent(envelope.summary.message));
  await waitFor(() => expect(modelHashLine()).toHaveTextContent("integrity=mismatch_review_required"));
  expect(screen.getByTestId(`tree-row-project-${encodeURIComponent(envelope.model.project.id)}`)).toBeInTheDocument();
  return envelope;
}

// A save response for the session's own model: the returned model is the
// requested one, so the save lands as an unchanged-model save.
function savedEnvelopeFor(model: PreviewModel, saveRequest: Record<string, unknown>, message: string): LocalProjectEnvelope {
  const base = inventedOpenEnvelope(model);
  return {
    ...base,
    ...(saveRequest as Partial<LocalProjectEnvelope>),
    model: saveRequest.model as PreviewModel,
    summary: { ...base.summary, project_id: model.project.id, project_name: model.project.name, message },
  };
}

// The validation packet records the last project operation that landed.
function lastProjectOperation(): string {
  const project = openWorkspaceSection("project");
  const intent = within(project).getByTestId("project-validation-export-link-local-private-intent") as HTMLInputElement;
  if (!intent.checked) fireEvent.click(intent);
  const href = within(project).getByTestId("project-validation-export-link").getAttribute("href") ?? "";
  const packet = JSON.parse(decodeURIComponent(href.split(",", 2)[1])) as { summary: { last_operation: string } };
  return packet.summary.last_operation;
}

async function flushPendingWork() {
  await act(async () => {
    await new Promise((resolve) => setTimeout(resolve, 0));
  });
}

describe("project handlers: open clears the previous project's operation diagnostics (B2F item 1)", () => {
  it("drops the earlier project's operation diagnostics from the issue count and the drawer when a project is opened", async () => {
    const model = await loadPreviewModel();
    const pendingApply = deferred<unknown>();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const baselineIssues = issueCountShown();

    fireEvent.click(screen.getByTestId("layout-mode-grid"));
    fireEvent.change(screen.getByTestId("entity-grid-input-node:N-100-y"), { target: { value: "0.5" } });
    fireEvent.click(screen.getByTestId("queue-entity-grid-intents"));
    invokeMock.mockImplementation((command: string) =>
      command === "apply_model_operation" ? pendingApply.promise : Promise.reject(new Error(`Unexpected command ${command}`)),
    );
    setTauriRuntime(true);
    fireEvent.click(screen.getByTestId("apply-intent-editor-intent-1"));
    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("apply_model_operation", expect.any(Object)));
    const request = invokeMock.mock.calls.find(([command]) => command === "apply_model_operation")![1] as {
      model: PreviewModel; intent: EditorOperationIntent; claimedModelHash: ModelHashEvidence;
    };
    setTauriRuntime(false);
    const outcome: OperationOutcome = await applyModelOperation(request.model, request.intent, request.claimedModelHash);
    outcome.diagnostics = [
      ...outcome.diagnostics,
      {
        id: "diag:invented-open-carryover",
        code: "INVENTED_OPEN_CARRYOVER",
        severity: "info",
        message: "Invented operation diagnostic recorded before another project is opened.",
        remediation: "none",
        affected_refs: [],
        source: "invented_test",
      },
    ];
    setTauriRuntime(true);
    await act(async () => {
      pendingApply.resolve(outcome);
      await pendingApply.promise;
    });
    await waitFor(() => expect(issueCountShown()).toBe(baselineIssues + 1));
    fireEvent.click(screen.getByTestId("issues-drawer-toggle"));
    const drawer = await screen.findByTestId("issues-home");
    expect(within(drawer).getByLabelText("Operation diagnostics")).toHaveTextContent("INVENTED_OPEN_CARRYOVER");

    const envelope = inventedOpenEnvelope(model);
    invokeMock.mockImplementation((command: string) =>
      command === "open_local_project" ? Promise.resolve(envelope) : Promise.reject(new Error(`Unexpected command ${command}`)),
    );
    act(() => nativeMenuCommand("file.open-local"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent(envelope.summary.message));

    expect(within(screen.getByTestId("issues-home")).queryByLabelText("Operation diagnostics")).not.toBeInTheDocument();
    expect(screen.getByTestId("issues-home")).not.toHaveTextContent("INVENTED_OPEN_CARRYOVER");
    expect(issueCountShown()).toBe(baselineIssues);
  });
});

describe("project handlers: listing does not release a busy state it does not own (B2F item 2)", () => {
  it("keeps the project controls disabled through a native list command while a save is pending, then lands the save", async () => {
    const model = await loadPreviewModel();
    const pendingSave = deferred<LocalProjectEnvelope>();
    let saveRequest: Record<string, unknown> | undefined;
    let listCalls = 0;
    invokeMock.mockImplementation((command: string, args?: { request?: Record<string, unknown> }) => {
      if (command === "save_local_project") {
        saveRequest = args?.request;
        return pendingSave.promise;
      }
      if (command === "list_local_projects") {
        listCalls += 1;
        return Promise.resolve([]);
      }
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    setTauriRuntime(true);
    fireEvent.click(screen.getByRole("button", { name: /^Save local$/ }));
    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("save_local_project", expect.any(Object)));
    for (const control of projectControls()) expect(control).toBeDisabled();
    const messageBeforeList = projectMessage().textContent;

    act(() => nativeMenuCommand("file.list-local"));
    await flushPendingWork();
    await flushPendingWork();
    // The list must not have released the save's busy state, and must not
    // have published a result of its own over the pending save.
    for (const control of projectControls()) expect(control).toBeDisabled();
    expect(projectMessage()).not.toHaveTextContent("Listed 0 local project snapshots");
    expect(projectMessage().textContent).toBe(messageBeforeList);
    expect(listCalls).toBe(0);

    const saved = savedEnvelopeFor(model, saveRequest!, "Saved invented project while a list command was refused.");
    await act(async () => {
      pendingSave.resolve(saved);
      await pendingSave.promise;
    });
    await waitFor(() => expect(screen.getByRole("button", { name: /^Save local$/ })).toBeEnabled());
    for (const control of projectControls()) expect(control).toBeEnabled();
    expect(projectMessage()).toHaveTextContent(saved.summary.message);
    expect(lastProjectOperation()).toBe("save");
  });

  it("leaves a save's busy state alone when a listing that started first settles during the save", async () => {
    const model = await loadPreviewModel();
    const pendingList = deferred<unknown[]>();
    const pendingSave = deferred<LocalProjectEnvelope>();
    let saveRequest: Record<string, unknown> | undefined;
    invokeMock.mockImplementation((command: string, args?: { request?: Record<string, unknown> }) => {
      if (command === "list_local_projects") return pendingList.promise;
      if (command === "save_local_project") {
        saveRequest = args?.request;
        return pendingSave.promise;
      }
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    setTauriRuntime(true);
    fireEvent.click(screen.getByRole("button", { name: /^List local$/ }));
    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("list_local_projects"));
    for (const control of projectControls()) expect(control).toBeDisabled();

    // The native menu may start a request-numbered handler during the list.
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("save_local_project", expect.any(Object)));
    await act(async () => {
      pendingList.resolve([]);
      await pendingList.promise;
    });
    await flushPendingWork();
    for (const control of projectControls()) expect(control).toBeDisabled();

    const saved = savedEnvelopeFor(model, saveRequest!, "Saved invented project after an earlier list settled.");
    await act(async () => {
      pendingSave.resolve(saved);
      await pendingSave.promise;
    });
    await waitFor(() => expect(screen.getByRole("button", { name: /^Save local$/ })).toBeEnabled());
    for (const control of projectControls()) expect(control).toBeEnabled();
    expect(projectMessage()).toHaveTextContent(saved.summary.message);
    expect(lastProjectOperation()).toBe("save");
  });
});

describe("project handlers: a failed or empty open leaves the open project as it was (B2F item 3)", () => {
  it("keeps a recorded model-hash mismatch when a later open finds nothing", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const envelope = await openProjectWithRecordedMismatch(model);

    invokeMock.mockImplementation((command: string) =>
      command === "open_local_project" ? Promise.resolve(null) : Promise.reject(new Error(`Unexpected command ${command}`)),
    );
    act(() => nativeMenuCommand("file.open-local"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("No local project snapshot found."));
    expect(modelHashLine()).toHaveTextContent("integrity=mismatch_review_required");
    expect(screen.getByTestId(`tree-row-project-${encodeURIComponent(envelope.model.project.id)}`)).toBeInTheDocument();
  });

  it("keeps a recorded model-hash mismatch when a later open fails", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const envelope = await openProjectWithRecordedMismatch(model);

    invokeMock.mockImplementation((command: string) =>
      command === "open_local_project"
        ? Promise.reject(new Error("Invented open failure"))
        : Promise.reject(new Error(`Unexpected command ${command}`)),
    );
    act(() => nativeMenuCommand("file.open-local"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("Open failed: Error: Invented open failure"));
    expect(modelHashLine()).toHaveTextContent("integrity=mismatch_review_required");
    expect(screen.getByTestId(`tree-row-project-${encodeURIComponent(envelope.model.project.id)}`)).toBeInTheDocument();
  });

  it("keeps a recorded model-hash mismatch when a blank create fails", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const envelope = await openProjectWithRecordedMismatch(model);

    invokeMock.mockImplementation((command: string) =>
      command === "create_local_project"
        ? Promise.reject(new Error("Invented blank create failure"))
        : Promise.reject(new Error(`Unexpected command ${command}`)),
    );
    act(() => nativeMenuCommand("file.new-blank"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("Blank create failed: Error: Invented blank create failure"));
    expect(modelHashLine()).toHaveTextContent("integrity=mismatch_review_required");
    expect(screen.getByTestId(`tree-row-project-${encodeURIComponent(envelope.model.project.id)}`)).toBeInTheDocument();
  });

  it("lets an in-flight rule-check revision land when an open finds nothing", async () => {
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    fireEvent.click(await runMechanicsButton());
    await waitFor(() => expectStatusChip("status-pill-mechanics", "MECHANICS_SOLVED", "Solver · Mechanics solved"));

    const previewService = await import("./services/previewService");
    const realBuild = previewService.buildAnalysisRunPreview.bind(previewService);
    const revisionGate = deferred<void>();
    let heldRevisions = 0;
    vi.spyOn(previewService, "buildAnalysisRunPreview").mockImplementation((result, options) => {
      if (!options?.ruleCheckAggregate) return realBuild(result, options);
      heldRevisions += 1;
      return revisionGate.promise.then(() => realBuild(result, options));
    });
    invokeMock.mockImplementation((command: string) => {
      if (command === "get_unit_catalog") return Promise.reject(new Error("Invented catalog unavailable"));
      if (command === "open_local_project") return Promise.resolve(null);
      if (command === "run_rule_checks") {
        return Promise.resolve({
          document_kind: "openpipestress.rule_check.run",
          rule_pack_id: "invented_demo_rule_pack",
          grammar_version: "1.0.0",
          aggregate_status: "USER_RULE_FAILED",
          checks: [{
            check_id: "invented_in_flight_check",
            status: "USER_RULE_FAILED",
            acceptability_relation: "less_than_or_equal",
            bound_inputs: [],
            completeness_findings: [],
            evaluator_findings: [],
            diagnostic_codes: [],
          }],
          professional_boundary_notice:
            "Rule-check results remain engineering decision-support information requiring responsible-engineer review.",
        });
      }
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    setTauriRuntime(true);
    const solve = openWorkspaceSection("solve");
    fireEvent.click(within(solve).getByTestId("rule-check-load-demo"));
    await within(solve).findByTestId("rule-check-binding-plan");
    fireEvent.click(within(solve).getByTestId("rule-check-run"));
    await waitFor(() => expect(heldRevisions).toBe(1));
    const evidence = openWorkspaceSection("evidence");
    expect(within(evidence).getByTestId("run-audit-status")).not.toHaveTextContent("USER_RULE_FAILED");

    act(() => nativeMenuCommand("file.open-local"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("No local project snapshot found."));
    await act(async () => {
      revisionGate.resolve();
      await revisionGate.promise;
    });
    await waitFor(() => expect(within(evidence).getByTestId("run-audit-status")).toHaveTextContent("USER_RULE_FAILED"));
  });
});

// Slice B2G. A stored envelope hash that cannot match the restored envelope,
// so the open records `mismatch_review_required` on the envelope cell too.
function mismatchedEnvelopeHash(model: PreviewModel): ProjectEnvelopeHashEvidence {
  return {
    algorithm: "sha256",
    canonicalization: "rfc8785_jcs",
    payload_scope: "project_envelope_payload",
    payload_excludes: "storage_summary_and_envelope_hash_carrier_fields",
    payload_ref: model.project.id,
    value: `sha256:${"e".repeat(64)}`,
    hash_status: "computed_local_preview",
  };
}

function envelopeHashLine() {
  return within(openWorkspaceSection("project")).getByTestId("project-validation-envelope-hash");
}

async function openProjectWithBothMismatchesRecorded(model: PreviewModel) {
  const envelope = inventedOpenEnvelope(model);
  envelope.model_hash = mismatchedModelHash(envelope.model);
  envelope.project_envelope_hash = mismatchedEnvelopeHash(envelope.model);
  invokeMock.mockImplementation((command: string) =>
    command === "open_local_project" ? Promise.resolve(envelope) : Promise.reject(new Error(`Unexpected command ${command}`)),
  );
  setTauriRuntime(true);
  act(() => nativeMenuCommand("file.open-local"));
  await waitFor(() => expect(projectMessage()).toHaveTextContent(envelope.summary.message));
  await waitFor(() => expect(modelHashLine()).toHaveTextContent("integrity=mismatch_review_required"));
  await waitFor(() => expect(envelopeHashLine()).toHaveTextContent("integrity=mismatch_review_required"));
  return envelope;
}

describe("project handlers: a failed save or create leaves the open project's integrity record (B2G)", () => {
  it("keeps both recorded mismatches when a save fails", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const envelope = await openProjectWithBothMismatchesRecorded(model);

    invokeMock.mockImplementation((command: string) =>
      command === "save_local_project"
        ? Promise.reject(new Error("Invented save failure"))
        : Promise.reject(new Error(`Unexpected command ${command}`)),
    );
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("Save failed: Error: Invented save failure"));
    expect(modelHashLine()).toHaveTextContent("integrity=mismatch_review_required");
    expect(envelopeHashLine()).toHaveTextContent("integrity=mismatch_review_required");
    expect(screen.getByTestId(`tree-row-project-${encodeURIComponent(envelope.model.project.id)}`)).toBeInTheDocument();
  });

  it("keeps both recorded mismatches when a create fails", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const envelope = await openProjectWithBothMismatchesRecorded(model);

    invokeMock.mockImplementation((command: string) =>
      command === "create_local_project"
        ? Promise.reject(new Error("Invented create failure"))
        : Promise.reject(new Error(`Unexpected command ${command}`)),
    );
    act(() => nativeMenuCommand("file.new-local"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("Create failed: Error: Invented create failure"));
    expect(modelHashLine()).toHaveTextContent("integrity=mismatch_review_required");
    expect(envelopeHashLine()).toHaveTextContent("integrity=mismatch_review_required");
    expect(screen.getByTestId(`tree-row-project-${encodeURIComponent(envelope.model.project.id)}`)).toBeInTheDocument();
  });

  it("keeps both recorded mismatches when a pending save is superseded by an open that finds nothing", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    await openProjectWithBothMismatchesRecorded(model);

    const pendingSave = deferred<LocalProjectEnvelope>();
    invokeMock.mockImplementation((command: string) => {
      if (command === "save_local_project") return pendingSave.promise;
      if (command === "open_local_project") return Promise.resolve(null);
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("save_local_project", expect.any(Object)));
    act(() => nativeMenuCommand("file.open-local"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("No local project snapshot found."));
    await act(async () => {
      pendingSave.reject(new Error("Invented late save failure"));
      await pendingSave.promise.catch(() => undefined);
    });
    await flushPendingWork();
    expect(modelHashLine()).toHaveTextContent("integrity=mismatch_review_required");
    expect(envelopeHashLine()).toHaveTextContent("integrity=mismatch_review_required");
  });

  it("still clears the open-time verification when a save lands, as before", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const envelope = await openProjectWithBothMismatchesRecorded(model);

    invokeMock.mockImplementation((command: string, args?: { request?: Record<string, unknown> }) =>
      command === "save_local_project"
        ? Promise.resolve(savedEnvelopeFor(envelope.model, args!.request!, "Saved invented project over a recorded mismatch."))
        : Promise.reject(new Error(`Unexpected command ${command}`)),
    );
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("Saved invented project over a recorded mismatch."));
    expect(modelHashLine()).toHaveTextContent("integrity=open_verification_not_run_this_session");
    expect(envelopeHashLine()).toHaveTextContent("integrity=open_verification_not_run_this_session");
  });
});

// Slice B2G, correction C1. A model edit advances the request epoch, so a save
// or create that was pending drops its response after its bytes were written.
// The open-time verification then no longer describes the stored bytes.
async function applyGridEditWhilePending() {
  fireEvent.click(screen.getByTestId("layout-mode-grid"));
  fireEvent.change(screen.getByTestId("entity-grid-input-node:N-100-y"), { target: { value: "0.5" } });
  fireEvent.click(screen.getByTestId("queue-entity-grid-intents"));
  // The operation engine runs in the browser route for this one call.
  setTauriRuntime(false);
  fireEvent.click(screen.getByTestId("apply-intent-editor-intent-1"));
  await waitFor(() => expect(screen.getByTestId("operation-apply-summary")).toHaveTextContent("1 applied"), { timeout: 10000 });
  setTauriRuntime(true);
}

describe("project handlers: a landed write clears the open-time record even when a model edit drops its response (B2G-C1)", () => {
  it("clears both lines when a model edit lands while a save is pending and the save then resolves", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const envelope = await openProjectWithBothMismatchesRecorded(model);

    const pendingSave = deferred<LocalProjectEnvelope>();
    let saveRequest: Record<string, unknown> | undefined;
    invokeMock.mockImplementation((command: string, args?: { request?: Record<string, unknown> }) => {
      if (command === "save_local_project") {
        saveRequest = args?.request;
        return pendingSave.promise;
      }
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("save_local_project", expect.any(Object)));
    await applyGridEditWhilePending();
    await act(async () => {
      pendingSave.resolve(savedEnvelopeFor(envelope.model, saveRequest!, "Saved invented project under a model edit."));
      await pendingSave.promise;
    });
    await flushPendingWork();
    expect(modelHashLine()).toHaveTextContent("integrity=open_verification_not_run_this_session");
    expect(envelopeHashLine()).toHaveTextContent("integrity=open_verification_not_run_this_session");
  });

  it("clears both lines when a model edit lands while a create is pending and the create then resolves", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const envelope = await openProjectWithBothMismatchesRecorded(model);

    const pendingCreate = deferred<LocalProjectEnvelope>();
    let createRequest: Record<string, unknown> | undefined;
    // `create_local_project` takes its fields flat, not under `request`.
    invokeMock.mockImplementation((command: string, args?: Record<string, unknown>) => {
      if (command === "create_local_project") {
        createRequest = args;
        return pendingCreate.promise;
      }
      return Promise.reject(new Error(`Unexpected command ${command}`));
    });
    act(() => nativeMenuCommand("file.new-local"));
    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("create_local_project", expect.any(Object)));
    await applyGridEditWhilePending();
    await act(async () => {
      pendingCreate.resolve(savedEnvelopeFor(envelope.model, createRequest!, "Created invented project under a model edit."));
      await pendingCreate.promise;
    });
    await flushPendingWork();
    expect(modelHashLine()).toHaveTextContent("integrity=open_verification_not_run_this_session");
    expect(envelopeHashLine()).toHaveTextContent("integrity=open_verification_not_run_this_session");
  });

  // Correction C2. A blank create writes a new project id, never the open
  // project's stored bytes, so its dropped response leaves the open project's
  // open-time record valid and in place.
  it("keeps both recorded mismatches when a model edit drops a pending blank create's response", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const envelope = await openProjectWithBothMismatchesRecorded(model);

    const pendingCreate = deferred<LocalProjectEnvelope>();
    invokeMock.mockImplementation((command: string) =>
      command === "create_local_project" ? pendingCreate.promise : Promise.reject(new Error(`Unexpected command ${command}`)),
    );
    act(() => nativeMenuCommand("file.new-blank"));
    await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("create_local_project", expect.any(Object)));
    await applyGridEditWhilePending();
    await act(async () => {
      pendingCreate.resolve(inventedOpenEnvelope(envelope.model));
      await pendingCreate.promise;
    });
    await flushPendingWork();
    // The blank create's response was dropped: its message never publishes.
    expect(projectMessage()).toHaveTextContent(envelope.summary.message);
    expect(modelHashLine()).toHaveTextContent("integrity=mismatch_review_required");
    expect(envelopeHashLine()).toHaveTextContent("integrity=mismatch_review_required");
  });
});

// B3A: observable saved canonical model identity, through the shared command route.
describe("B3A saved-model marker", () => {
  it("marks an edit, save clears it, Undo differs from the saved snapshot and Redo returns clean", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument();
    await applyGridEditWhilePending();
    expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited");
    invokeMock.mockImplementation((command: string, args?: { request?: Record<string, unknown> }) => {
      if (command === "sync_native_shell_state") return Promise.resolve({});
      if (command === "save_local_project") return Promise.resolve(savedEnvelopeFor(model, args!.request!, "B3A saved"));
      return Promise.reject(new Error(command));
    });
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument());
    fireEvent.click(screen.getByTestId("workspace-undo"));
    await waitFor(() => expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited"));
    fireEvent.click(screen.getByTestId("workspace-redo"));
    await waitFor(() => expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument());
  });

  it("keeps newer edits after a landed save and makes Undo to that persisted snapshot clean", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    await applyGridEditWhilePending();
    const pending = deferred<LocalProjectEnvelope>();
    let request: Record<string, unknown> | undefined;
    invokeMock.mockImplementation((command: string, args?: { request?: Record<string, unknown> }) => {
      if (command === "sync_native_shell_state") return Promise.resolve({});
      if (command === "save_local_project") { request = args!.request; return pending.promise; }
      return Promise.reject(new Error(command));
    });
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(request).toBeDefined());
    fireEvent.click(screen.getByTestId("workspace-undo"));
    await act(async () => pending.resolve(savedEnvelopeFor(model, request!, "B3A landed")));
    await flushPendingWork();
    expect(screen.getByTestId("project-edited")).toHaveTextContent("Edited");
    fireEvent.click(screen.getByTestId("workspace-redo"));
    await waitFor(() => expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument());
  });
});


describe("B3A asynchronous canonical comparison", () => {
  it("ignores out-of-order hash completion and stays Edited while Undo hash is pending", async () => {
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    await applyGridEditWhilePending();
    await flushPendingWork();
    const original = hashService.computeModelHash;
    const pending: Array<{ model: PreviewModel; gate: ReturnType<typeof deferred<ModelHashEvidence | null>> }> = [];
    vi.spyOn(hashService, "computeModelHash").mockImplementation((model) => {
      const gate = deferred<ModelHashEvidence | null>(); pending.push({ model, gate }); return gate.promise;
    });
    fireEvent.click(screen.getByTestId("workspace-undo"));
    expect(screen.getByTestId("project-edited")).toBeInTheDocument();
    fireEvent.click(screen.getByTestId("workspace-redo"));
    expect(pending).toHaveLength(2);
    await act(async () => pending[1].gate.resolve(await original(pending[1].model)));
    expect(screen.getByTestId("project-edited")).toBeInTheDocument();
    await act(async () => pending[0].gate.resolve(await original(pending[0].model)));
    expect(screen.getByTestId("project-edited")).toBeInTheDocument();
    fireEvent.click(screen.getByTestId("workspace-undo"));
    expect(screen.getByTestId("project-edited")).toBeInTheDocument();
    await act(async () => pending[2].gate.resolve(await original(pending[2].model)));
    await waitFor(() => expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument());
  });

  it("retains the actually opened baseline through a pending hash, a missing later Open and an edit", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const opened = inventedOpenEnvelope(model);
    const original = hashService.computeModelHash;
    const gate = deferred<void>();
    let delayed = 0;
    vi.spyOn(hashService, "computeModelHash").mockImplementation(async (input) => {
      if (input.project.id === opened.model.project.id && delayed++ < 2) await gate.promise;
      return original(input);
    });
    invokeMock.mockImplementation((command: string) => command === "open_local_project" ? Promise.resolve(opened) : Promise.resolve({}));
    setTauriRuntime(true);
    act(() => nativeMenuCommand("file.open-local"));
    await waitFor(() => expect(screen.getByTestId("toolbar-project-name")).toHaveTextContent(opened.model.project.name));
    invokeMock.mockImplementation((command: string) => command === "open_local_project" ? Promise.resolve(null) : Promise.resolve({}));
    act(() => nativeMenuCommand("file.open-local"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("No local project snapshot"));
    await applyGridEditWhilePending();
    await act(async () => gate.resolve());
    await flushPendingWork();
    expect(screen.getByTestId("project-edited")).toBeInTheDocument();
    fireEvent.click(screen.getByTestId("workspace-undo"));
    await waitFor(() => expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument());
  });

  it.each(["write failure", "wrong model hash", "wrong envelope hash", "wrong project identity"])("does not clear edits after %s", async (failure) => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    await applyGridEditWhilePending();
    invokeMock.mockImplementation((command: string, args?: { request?: Record<string, unknown> }) => {
      if (command !== "save_local_project") return Promise.resolve({});
      if (failure === "write failure") return Promise.reject(new Error("Invented failed write"));
      const saved = savedEnvelopeFor(model, args!.request!, "Invalid response");
      if (failure === "wrong model hash") saved.model_hash = { ...saved.model_hash!, value: "sha256:wrong" };
      if (failure === "wrong envelope hash") saved.project_envelope_hash = { ...saved.project_envelope_hash!, value: "sha256:wrong" };
      if (failure === "wrong project identity") saved.summary.project_id = "project:wrong";
      return Promise.resolve(saved);
    });
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent(failure === "write failure" ? "Save failed:" : "Invalid response"));
    expect(screen.getByTestId("project-edited")).toBeInTheDocument();
    fireEvent.click(screen.getByTestId("workspace-undo"));
    await waitFor(() => expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument());
  });
});

describe("B3A baseline ownership after asynchronous persistence", () => {
  it("does not let a delayed Open hash overwrite a newer verified same-generation save", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const opened = inventedOpenEnvelope(model);
    const original = hashService.computeModelHash;
    const gate = deferred<void>();
    let delayed = 0;
    vi.spyOn(hashService, "computeModelHash").mockImplementation(async (input) => {
      if (input.project.id === opened.model.project.id && delayed++ < 2) await gate.promise;
      return original(input);
    });
    invokeMock.mockImplementation((command: string, args?: { request?: Record<string, unknown> }) => {
      if (command === "open_local_project") return Promise.resolve(opened);
      if (command === "save_local_project") return Promise.resolve(savedEnvelopeFor(opened.model, args!.request!, "Newer verified save"));
      return Promise.resolve({});
    });
    setTauriRuntime(true);
    act(() => nativeMenuCommand("file.open-local"));
    await waitFor(() => expect(screen.getByTestId("toolbar-project-name")).toHaveTextContent(opened.model.project.name));
    await applyGridEditWhilePending();
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("Newer verified save"));
    expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument();
    await act(async () => gate.resolve());
    await flushPendingWork();
    expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument();
    fireEvent.click(screen.getByTestId("workspace-undo"));
    await waitFor(() => expect(screen.getByTestId("project-edited")).toBeInTheDocument());
  });

  it("drops a landed old save after same-ID project replacement and preserves the reopened baseline", async () => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    await applyGridEditWhilePending();
    const pending = deferred<LocalProjectEnvelope>();
    let request: Record<string, unknown> | undefined;
    const reopened = savedEnvelopeFor(model, { model }, "Same-ID reopened source");
    invokeMock.mockImplementation((command: string, args?: { request?: Record<string, unknown> }) => {
      if (command === "save_local_project") { request = args!.request; return pending.promise; }
      if (command === "open_local_project") return Promise.resolve(reopened);
      return Promise.resolve({});
    });
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(request).toBeDefined());
    act(() => nativeMenuCommand("file.open-local"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("Same-ID reopened source"));
    await waitFor(() => expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument());
    await act(async () => pending.resolve(savedEnvelopeFor(model, request!, "Obsolete landed save")));
    await flushPendingWork();
    expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument();
    expect(screen.getByTestId("workspace-undo")).toBeDisabled();
    expect(projectMessage()).toHaveTextContent("Same-ID reopened source");
  });
});

describe("B3A persisted normalization comparison", () => {
  it.each([true, false])("clears only a verified supported normalization (ledger=%s)", async (supported) => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    await applyGridEditWhilePending();
    invokeMock.mockImplementation(async (command: string, args?: { request?: Record<string, unknown> }) => {
      if (command !== "save_local_project") return {};
      const returned = savedEnvelopeFor(model, args!.request!, "Normalized saved model");
      const beforeHash = await hashService.computeModelHash(returned.model);
      returned.model = { ...returned.model, schema_version: "0.2.0" };
      returned.model_hash = await hashService.computeModelHash(returned.model);
      returned.project_envelope_hash = await hashService.computeProjectEnvelopeHash({
        model: returned.model, editor_intents: returned.editor_intents, proposal: returned.proposal,
        selected_review_target: returned.selected_review_target, mechanics_result: returned.mechanics_result,
        analysis_run: returned.analysis_run, model_hash: returned.model_hash
      });
      if (supported) {
        const ids = ["model-doc-0.1.0-to-0.2.0-additive-combination-shape-noop"];
        returned.model_document_migration = {
          status: "migrated", source_schema_version: "0.1.0", target_schema_version: "0.2.0",
          migration_framework: "application_service_separate_db_and_product_schema",
          db_migration_status: "store_user_version_ledger_separate_ddl_only", product_schema_migration_status: "migrated",
          applied_migration_ids: ids, persistence_state: "persisted_with_ledger_record", detail: "Invented supported normalization"
        };
        returned.model_migration_ledger = [{
          record_kind: "model_document_migration_ledger_record", recorded_at_unix: 1,
          source_schema_version: "0.1.0", target_schema_version: "0.2.0", applied_migration_ids: ids,
          migration_framework: "application_service_separate_db_and_product_schema",
          pre_migration_model_hash: beforeHash!.value, post_migration_model_hash: returned.model_hash!.value,
          trigger: "migrate_in_memory_on_open_persisted_on_save", destructive_rewrite: false,
          professional_boundary: { human_review_required: true, software_makes_compliance_claim: false },
          hash_evidence: {
            schema: "model_migration_hash_evidence_v1", source_payload_basis: "incoming_pre_migration_model",
            received: { model_hash: beforeHash, project_envelope_hash: null }, prior_stored: null,
            computed: { pre_migration_model_hash: beforeHash!.value, post_migration_model_hash: returned.model_hash!.value,
              post_migration_project_envelope_hash: returned.project_envelope_hash!.value }, received_claim_verification: "not_asserted"
          }
        }];
      }
      return returned;
    });
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent(supported ? "Normalized saved model" : "Save failed:"));
    if (supported) {
      expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument();
      expect(screen.getByTestId("workspace-undo")).toBeDisabled();
    } else expect(screen.getByTestId("project-edited")).toBeInTheDocument();
  });
});

describe("B3A initial source ownership", () => {
  it("keeps untouched loading source unmarked and does not overwrite a newer save when its hash resolves", async () => {
    const model = await loadPreviewModel();
    const original = hashService.computeModelHash;
    const gate = deferred<void>();
    let calls = 0;
    vi.spyOn(hashService, "computeModelHash").mockImplementation(async (input) => {
      if (calls++ < 2) await gate.promise;
      return original(input);
    });
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument();
    await applyGridEditWhilePending();
    expect(screen.getByTestId("project-edited")).toBeInTheDocument();
    invokeMock.mockImplementation((command: string, args?: { request?: Record<string, unknown> }) => command === "save_local_project"
      ? Promise.resolve(savedEnvelopeFor(model, args!.request!, "Saved ahead of initial hash")) : Promise.resolve({}));
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(projectMessage()).toHaveTextContent("Saved ahead of initial hash"));
    expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument();
    await act(async () => gate.resolve());
    await flushPendingWork();
    expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument();
    fireEvent.click(screen.getByTestId("workspace-undo"));
    await waitFor(() => expect(screen.getByTestId("project-edited")).toBeInTheDocument());
    await waitFor(() => expect(invokeMock).toHaveBeenLastCalledWith("sync_native_shell_state", { state: expect.objectContaining({ modelEdited: true }) }));
  });
});


it("B3A keeps the model available when the initial canonical hash fails, without a clean saved claim", async () => {
  const original = hashService.computeModelHash;
  let calls = 0;
  vi.spyOn(hashService, "computeModelHash").mockImplementation((input) => calls++ < 2
    ? Promise.reject(new Error("Invented hash unavailable")) : original(input));
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  await flushPendingWork();
  expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument();
  expect(screen.getByTestId("toolbar-project-name")).not.toBeEmptyDOMElement();
  await applyGridEditWhilePending();
  expect(screen.getByTestId("project-edited")).toBeInTheDocument();
});


it("B3A verifies unchanged Historical save against its exact retained hash carrier, preserving Historical standing", async () => {
  const model = await loadPreviewModel();
  const opened = inventedOpenEnvelope(model);
  opened.mechanics_result = await runPreviewMechanics(opened.model);
  opened.model_hash = mismatchedModelHash(opened.model);
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  const original = hashService.computeModelHash;
  const gate = deferred<void>();
  let openedHashes = 0;
  vi.spyOn(hashService, "computeModelHash").mockImplementation(async (input) => {
    if (input.project.id === opened.model.project.id) {
      openedHashes += 1;
      // Historical verification is first; delay the committed Open baseline
      // and live hash, so only the later verified save can clear the marker.
      if (openedHashes === 2 || openedHashes === 3) await gate.promise;
    }
    return original(input);
  });
  let saveRequest: Record<string, unknown> | undefined;
  invokeMock.mockImplementation((command: string, args?: { request?: Record<string, unknown> }) => {
    if (command === "open_local_project") return Promise.resolve(opened);
    if (command === "save_local_project") {
      saveRequest = args!.request;
      return Promise.resolve(savedEnvelopeFor(opened.model, saveRequest!, "Historical carrier retained"));
    }
    return Promise.resolve({});
  });
  setTauriRuntime(true);
  act(() => nativeMenuCommand("file.open-local"));
  await screen.findByTestId("historical-run-context");
  expect(screen.getByTestId("project-edited")).toBeInTheDocument();
  act(() => nativeMenuCommand("file.save-local"));
  await waitFor(() => expect(projectMessage()).toHaveTextContent("Historical carrier retained"));
  expect(saveRequest!.model_hash).toEqual(opened.model_hash);
  expect(saveRequest!.model).toEqual(opened.model);
  expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument();
  expect(screen.getByTestId("historical-run-context")).toHaveTextContent("HISTORICAL_MODEL_HASH_MISMATCH");
  expect(screen.queryByTestId("status-pill-mechanics")).not.toBeInTheDocument();
  expect(screen.getByTestId("workspace-undo")).toBeDisabled();
  await act(async () => gate.resolve());
  await flushPendingWork();
  expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument();
});

// Independent-review P2: unsuccessful request initiation cannot retire a
// same-session write that subsequently lands. No B3B menu/integrity changes.
describe("B3A P2 landed-write observation", () => {
  it.each(["missing", "failed"])("retains landed B after a later %s Open, with truthful Undo/Redo and unchanged integrity guards", async (openOutcome) => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    const opened = await openProjectWithBothMismatchesRecorded(model);
    await applyGridEditWhilePending();
    const pending = deferred<LocalProjectEnvelope>();
    let request: Record<string, unknown> | undefined;
    invokeMock.mockImplementation((command: string, args?: { request?: Record<string, unknown> }) => {
      if (command === "save_local_project") { request = args!.request; return pending.promise; }
      if (command === "open_local_project") return openOutcome === "missing" ? Promise.resolve(null) : Promise.reject(new Error("Invented later Open failure"));
      return Promise.resolve({});
    });
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(request).toBeDefined());
    expect((request!.model as PreviewModel).nodes[0].position.y).toBe(0.5);
    act(() => nativeMenuCommand("file.open-local"));
    const openMessage = openOutcome === "missing" ? "No local project snapshot found." : "Open failed: Error: Invented later Open failure";
    await waitFor(() => expect(projectMessage()).toHaveTextContent(openMessage));
    await act(async () => pending.resolve(savedEnvelopeFor(opened.model, request!, "Obsolete UI response, real landed B")));
    await waitFor(() => expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument());
    expect(projectMessage()).toHaveTextContent(openMessage);
    expect(modelHashLine()).toHaveTextContent("integrity=mismatch_review_required");
    expect(envelopeHashLine()).toHaveTextContent("integrity=mismatch_review_required");
    expect(screen.getByTestId("entity-grid-input-node:N-100-y")).toHaveValue("0.5");
    fireEvent.click(screen.getByTestId("workspace-undo"));
    await waitFor(() => expect(screen.getByTestId("project-edited")).toBeInTheDocument());
    expect(screen.getByTestId("entity-grid-input-node:N-100-y")).toHaveValue("0");
    fireEvent.click(screen.getByTestId("workspace-redo"));
    await waitFor(() => expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument());
    expect(screen.getByTestId("entity-grid-input-node:N-100-y")).toHaveValue("0.5");
  });

  it.each([true, false])("orders verified responses independently of hash completion (later response valid=%s)", async (laterValid) => {
    const model = await loadPreviewModel();
    render(<App />);
    await screen.findByTestId("desktop-preview-shell");
    await applyGridEditWhilePending();
    const first = deferred<LocalProjectEnvelope>();
    const second = deferred<LocalProjectEnvelope>();
    const requests: Record<string, unknown>[] = [];
    invokeMock.mockImplementation((command: string, args?: { request?: Record<string, unknown> }) => {
      if (command === "save_local_project") {
        requests.push(args!.request!);
        return requests.length === 1 ? first.promise : second.promise;
      }
      return Promise.resolve({});
    });
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(requests).toHaveLength(1));
    fireEvent.click(screen.getByTestId("workspace-undo"));
    act(() => nativeMenuCommand("file.save-local"));
    await waitFor(() => expect(requests).toHaveLength(2));
    const firstResponse = savedEnvelopeFor(model, requests[0], "First response B");
    const original = hashService.computeModelHash;
    const gate = deferred<void>();
    let firstVerifying = false;
    vi.spyOn(hashService, "computeModelHash").mockImplementation(async (input) => {
      if (input === firstResponse.model) { firstVerifying = true; await gate.promise; }
      return original(input);
    });
    try {
      await act(async () => first.resolve(firstResponse));
      await waitFor(() => expect(firstVerifying).toBe(true));
      const secondResponse = savedEnvelopeFor(model, requests[1], "Second response A");
      if (!laterValid) secondResponse.model_hash = { ...secondResponse.model_hash!, value: "sha256:unverified-later-claim" };
      await act(async () => second.resolve(secondResponse));
      await waitFor(() => expect(projectMessage()).toHaveTextContent("Second response A"));
      expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument();
      await act(async () => gate.resolve());
      await flushPendingWork();
      if (laterValid) expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument();
      else expect(screen.getByTestId("project-edited")).toBeInTheDocument();
      expect(screen.getByTestId("entity-grid-input-node:N-100-y")).toHaveValue("0");
      fireEvent.click(screen.getByTestId("workspace-redo"));
      await waitFor(() => laterValid
        ? expect(screen.getByTestId("project-edited")).toBeInTheDocument()
        : expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument());
      expect(screen.getByTestId("entity-grid-input-node:N-100-y")).toHaveValue("0.5");
      fireEvent.click(screen.getByTestId("workspace-undo"));
      await waitFor(() => laterValid
        ? expect(screen.queryByTestId("project-edited")).not.toBeInTheDocument()
        : expect(screen.getByTestId("project-edited")).toBeInTheDocument());
      expect(projectMessage()).toHaveTextContent("Second response A");
    } finally { await act(async () => gate.resolve()); }
  });
});
