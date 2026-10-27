import { act, cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";

const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("./test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));

import { App } from "./App";

// DEL-07-07 Design `solve-progress-panel.md`: the shell's solve progress panel
// follows a native job (invented replies) from its start, and its own Cancel
// control sends the job's cancellation through the session's handler.
afterEach(() => {
  cleanup();
  vi.restoreAllMocks();
  invokeMock.mockReset();
  delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
});

function nativeMenuCommand(command: string) {
  window.dispatchEvent(new CustomEvent("openpipestress-native-menu-command", { detail: command }));
}

it("opens on a run, shows the job's live state, and cancels it from the panel", async () => {
  let cancelled = false;
  invokeMock.mockImplementation((command: string) => {
    if (command === "sync_native_shell_state") return Promise.resolve(undefined);
    if (command === "start_preview_mechanics_job") {
      return Promise.resolve({ job_id: "backend-solve-job-panel", backend_cancellation_token: "cancel-token-panel", state: "queued", cancellation_scope: "worker_process_terminated" });
    }
    if (command === "poll_preview_mechanics_job") {
      return Promise.resolve({
        job_id: "backend-solve-job-panel", state: cancelled ? "cancelled" : "running", cancellation_requested: cancelled,
        cancellation_status: cancelled ? "worker_terminated" : "not_requested", cancellation_scope: "worker_process_terminated",
        result_available: false, error_message: null
      });
    }
    if (command === "cancel_preview_mechanics_job") {
      cancelled = true;
      return Promise.resolve({ job_id: "backend-solve-job-panel", accepted: true, cancellation_status: "request_recorded", job_state: "running", cancellation_scope: "worker_process_terminated", cancellation_success_claimed: false });
    }
    if (command === "dispose_all_preview_mechanics_results") return Promise.resolve({ released_results: 0, released_bytes: 0, abandoned_jobs: 0 });
    throw new Error(`Unexpected native command ${command}`);
  });

  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  act(() => nativeMenuCommand("analyze.run"));

  const region = await screen.findByRole("region", { name: "Solve progress" });
  await waitFor(() => expect(region).toHaveAttribute("data-state", "running"));
  expect(within(region).getByTestId("solve-progress-job")).toHaveTextContent("backend-solve-job-panel");
  expect(within(region).getByTestId("solve-progress-stage")).toHaveTextContent("Backend solve job backend-solve-job-panel is executing in its worker process");

  const cancel = within(region).getByRole("button", { name: "Cancel solve" });
  cancel.focus();
  fireEvent.click(cancel);
  await waitFor(() => expect(invokeMock).toHaveBeenCalledWith("cancel_preview_mechanics_job", { jobId: "backend-solve-job-panel", cancellationToken: "cancel-token-panel" }));
  await waitFor(() => expect(region).toHaveAttribute("data-state", "cancelled"));
  expect(within(region).getByTestId("solve-progress-cancellation")).toHaveTextContent("Cancellation requested: worker_terminated");
  const events = within(within(region).getByRole("list", { name: "Solve job events" })).getAllByRole("listitem").map((item) => item.getAttribute("data-event-id"));
  expect(events[0]).toBe("solve-preview-queued");
  expect(events).toContain("solve-preview-running");
  expect(events.at(-1)).toBe("solve-preview-cancelled");
  expect(invokeMock.mock.calls.filter(([command]) => command === "cancel_preview_mechanics_job")).toHaveLength(1);

  fireEvent.click(within(region).getByRole("button", { name: "Close" }));
  expect(screen.queryByTestId("solve-progress-panel")).toBeNull();
});

// Codex review r1 of #1665 (its probe, kept): `running` turns on before the
// run's first record replaces the previous one; the panel must not show the
// previous job as the new run's.
it("a rerun after a failed job shows Starting with Cancel, never the previous job, until the new job reports", async () => {
  let starts = 0;
  let releaseSecond: (value: unknown) => void = () => {};
  invokeMock.mockImplementation((command: string) => {
    if (command === "sync_native_shell_state") return Promise.resolve(undefined);
    if (command === "dispose_all_preview_mechanics_results") return Promise.resolve({ released_results: 0, released_bytes: 0, abandoned_jobs: 0 });
    if (command === "start_preview_mechanics_job") {
      starts++;
      if (starts === 2) return new Promise((resolve) => { releaseSecond = resolve; });
      return Promise.resolve({ job_id: "review-job-one", backend_cancellation_token: "review-token", state: "queued", cancellation_scope: "worker_process_terminated" });
    }
    if (command === "poll_preview_mechanics_job") {
      return starts === 1
        ? Promise.resolve({ job_id: "review-job-one", state: "failed", cancellation_requested: false, cancellation_status: "not_requested", cancellation_scope: "worker_process_terminated", result_available: false, error_message: "review first worker failure" })
        : Promise.resolve({ job_id: "review-job-two", state: "running", cancellation_requested: false, cancellation_status: "not_requested", cancellation_scope: "worker_process_terminated", result_available: false, error_message: null });
    }
    throw new Error(`Unexpected native command ${command}`);
  });
  render(<App />);
  await screen.findByTestId("desktop-preview-shell");
  (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
  act(() => nativeMenuCommand("analyze.run"));
  await waitFor(() => expect(screen.getByTestId("solve-progress-panel")).toHaveAttribute("data-state", "failed"));
  fireEvent.click(screen.getByTestId("solve-progress-close"));
  act(() => nativeMenuCommand("analyze.run"));
  await waitFor(() => expect(starts).toBe(2));
  const region = screen.getByRole("region", { name: "Solve progress" });
  expect(region).toHaveAttribute("data-state", "starting");
  expect(screen.queryByTestId("solve-progress-job")).toBeNull();
  expect(screen.queryByTestId("solve-progress-failure")).toBeNull();
  expect(within(region).getByRole("button", { name: "Cancel solve" })).toHaveAttribute("aria-disabled", "false");
  await act(async () => {
    releaseSecond({ job_id: "review-job-two", backend_cancellation_token: "review-token-two", state: "queued", cancellation_scope: "worker_process_terminated" });
    await Promise.resolve();
  });
  await waitFor(() => expect(screen.getByTestId("solve-progress-job")).toHaveTextContent("review-job-two"));
  expect(within(within(region).getByRole("list", { name: "Solve job events" })).getAllByRole("listitem").map((item) => item.getAttribute("data-event-id"))[0]).toBe("solve-preview-queued");
});
