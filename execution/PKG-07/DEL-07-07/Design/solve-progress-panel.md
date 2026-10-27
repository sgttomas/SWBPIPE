# Solve progress panel

The desktop shows a solve job's live state in a panel over the workspace while
the job runs, and keeps its final state there until the user closes it. Paths
are relative to `projects/chirality-piping/apps/desktop/`. Standard claim fence
applies (F-PIP-2; claims taxonomy per DEC-081).

The panel serves REQ-07-07-002 to -004 and FR-SOL-015's run-time reporting. It
adds no job field, state or event. Everything it shows is already in the
session's solve-job record (`SolveJobAuditState`, kept by
`src/features/workspace/solveJobAudit.ts` from the native start receipt, each
status poll, the cancellation receipt and the terminal transition).

## What it shows

`src/features/solve/SolveProgressPanel.tsx`, mounted once in the shell
(`src/App.tsx`), beside the status bar:

| Line | Source in the record |
|---|---|
| State (`role="status"`, polite live region) | `state`: queued, running, cancelling, cancelled, completed, failed or refused |
| Stage | the latest event's `message`: the job's own words for where it stands |
| Job | `job_id` |
| Progress basis | `progress_basis`, `percentages_synthesized`, `backend_percent_stream_available` |
| Queue reason (queued or cancelling) | `resource.queue_reason`, read with `describeResourceRefusal` and shown with its exact code (`resourceRefusalCode`) |
| Worker memory (while active, on the worker-capped route) | `resource.worker_grant_bytes`, `worker_peak_grant_bytes`, `worker_peak_footprint_bytes`, `invocation_budget_bytes` |
| Cancellation | `cancellation_status`, once `cancellation_requested` |
| Completion | the completed event's `result_row_count` and `diagnostic_count` |
| Refusal | `resource.refusal`: its reading, the refusal point's retry path (`describeSolveRefusalRetry`) and its exact code with its byte fields |
| Failure | `error_message`, or the worker-failure reading (`describeSolveWorkerFailure`) |
| Events | every entry of `events`, in order, with its state and message |

**No percentage, estimate or clock.** The job reports states and events only:
`progress_basis` is `preview_service_event_state_only_no_percent_stream` and
`percentages_synthesized` is false. The events carry no timestamps, so the panel
shows no elapsed time either. A progress bar, a percentage, an estimate of the
time remaining or an elapsed time would need a job field that does not exist
today. Adding one is a change to the job contract (DEL-00-03), not to this
panel.

## When it shows

- A run opens it: the panel appears when the session's `running` flag turns
  true, and follows the record from then on.
- It stays open after the terminal state, so a completion, cancellation,
  failure or refusal remains readable. **Close** dismisses it until the next
  run.
- A record that describes no job (`not_started`: a model edit, undo, a new
  project) closes it. A completed record the session did not run, such as a
  run restored from the project store, never opens it.

## Cancel

While the job is queued or running, the panel offers **Cancel solve**. It calls
the session's `handleCancelRun`, the handler that the Solve panel's Cancel
preview control and the Analyze menu's Cancel Run use, so the request goes
through the application-service cancellation contract (REQ-07-07-004).

The control is a native button in the tab order, so Enter and Space activate
it. After a request it stays in place, reads **Cancelling** and is
`aria-disabled`, so a second press sends nothing. Keyboard focus moves to the
state line, which announces the job's next state. The panel claims no
cancellation success. It shows the cancellation status and terminal state the
job reports.

## Evidence

- `src/features/solve/SolveProgressPanel.test.tsx` uses invented native job
  reports. It covers these behaviours:
  - the panel stays hidden until a run and opens nothing for a restored record;
  - the live state, the stage, the queue reason with its code, worker memory
    and the event list in order, with no percentage, estimate, clock or
    progress bar;
  - Cancel as a focusable native button: focus moves to the state, and after
    the request the control is `aria-disabled` and sends nothing;
  - a refusal's reading, retry path and exact code;
  - a worker failure;
  - completion counts, Close until the next run, and closing on a model change.
- `src/App.solveProgress.test.tsx` runs a native job in the full shell. The
  panel opens on Analyze → Run, shows the running stage, and its Cancel sends
  exactly one `cancel_preview_mechanics_job` with the job's token. It then
  shows the cancelled state, and Close dismisses it.
