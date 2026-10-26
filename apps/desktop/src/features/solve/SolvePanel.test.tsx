import { render, screen } from '@testing-library/react';
import { expect, it } from 'vitest';
import { loadPreviewModel, runPreviewMechanics } from '../../services/previewService';
import { ResourceUnavailableError } from '../../services/memoryPool';
import { failSolveJob, initialSolveJob, refusedAtTakeSolveJob, refusedSolveJob, startSolveJob } from '../workspace/solveJobAudit';
import { SolvePanel } from './SolvePanel';

it('exposes the actual browser backend refusal without fabricating result diagnostics or model status', async () => {
  const model = await loadPreviewModel(), before = JSON.stringify(model);
  let failure: unknown;
  try { await runPreviewMechanics(model); } catch (error) { failure = error; }
  expect(failure).toBeInstanceOf(Error);
  const job = failSolveJob(initialSolveJob(), failure);
  const props = { model, result: null, analysisRun: null, running: false, solveJob: job,
    solverMode: 'sparse_interactive' as const, onRun() {}, onCancel() {}, onSolverModeChange() {} };
  const view = render(<SolvePanel {...props} />);
  expect(screen.getByTestId('solve-job-error')).toHaveAttribute('role', 'status');
  expect(screen.getByTestId('solve-job-error')).toHaveTextContent(String(failure));
  expect(screen.getByTestId('solve-job-summary')).toHaveTextContent('state=failed');
  expect(screen.getByTestId('solve-job-summary')).toHaveTextContent('result_rows=0');
  expect(JSON.stringify(model)).toBe(before);
  view.rerender(<SolvePanel {...props} solveJob={initialSolveJob()} />);
  expect(screen.queryByTestId('solve-job-error')).toBeNull();
});

it.each([
  ['the result take', (job: ReturnType<typeof startSolveJob>) => refusedAtTakeSolveJob(job, new ResourceUnavailableError({ kind: 'headroom_insufficient', required_bytes: 68165632, available_bytes: 52641792 }, null)),
    'Refused when the finished result was handed to the app; the session does not retry a refused take, so it released the result. Available memory was re-sampled for up to 2 s before this refusal stood. You can run the solve again when more memory is available.'],
  ['the result hand-off', (job: ReturnType<typeof startSolveJob>) => refusedSolveJob(job, { kind: 'headroom_insufficient', required_bytes: 68165632, available_bytes: 52641792 }),
    'Refused at the result hand-off after the solve finished; the finished result was dropped. Available memory was re-sampled for up to 2 s before this refusal stood. You can run the solve again when more memory is available.']
])('shows a solve refused at %s as refused, by name, with its exact bytes and retry path, and no rows', async (_point, refuse, retry) => {
  const model = await loadPreviewModel();
  const started = startSolveJob(model, { mode: 'backend_job', job_id: 'backend-solve-job-9', backend_cancellation_token: 'token-9', state: 'queued', cancellation_scope: 'worker_process_terminated',
    resource: { invocation_budget_bytes: 12 * 1024 ** 3, pool_bytes: 96 * 1024 ** 3, route_pricing: 'ordinary_worker_capped', ingress_reserved_bytes: 8192, worker_grant_bytes: null, worker_peak_grant_bytes: 65 * 1024 ** 2, worker_peak_footprint_bytes: 3649044 } });
  render(<SolvePanel model={model} result={null} analysisRun={null} running={false} solveJob={refuse(started)} solverMode="sparse_interactive" onRun={() => {}} onCancel={() => {}} onSolverModeChange={() => {}} />);
  expect(screen.getByTestId('solve-job-summary')).toHaveTextContent('state=refused; events=2; result_rows=0');
  expect(screen.getByTestId('solve-job-resource-refusal')).toHaveTextContent("Refused by the native memory pool: Not enough available memory on this Mac right now (free plus reclaimable file cache and purgeable memory, less the kernel's reserve): needs 65.01 MiB, 50.2 MiB available. No result was published.");
  expect(screen.getByTestId('solve-job-refusal-retry')).toHaveTextContent(`${retry} Refusal code: headroom_insufficient (required_bytes=68165632, available_bytes=52641792)`);
  expect(screen.getByTestId('solve-job-resource')).toHaveTextContent('refusal=headroom_insufficient');
  expect(screen.getByTestId('solve-job-binding')).toHaveTextContent('not generated; result rows=0; hashes=0');
  expect(screen.queryByTestId('solve-job-error')).toBeNull();
});
