import { act, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", () => ({ invoke: invokeMock }));

import { App } from "./App";
import { loadPreviewModel } from "./services/previewService";
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

  it("clears both lines when a model edit lands while a blank create is pending and the create then resolves", async () => {
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
    expect(modelHashLine()).toHaveTextContent("integrity=open_verification_not_run_this_session");
    expect(envelopeHashLine()).toHaveTextContent("integrity=open_verification_not_run_this_session");
  });
});
