import { render, screen } from '@testing-library/react';
import { expect, it } from 'vitest';
import { loadPreviewModel, runPreviewMechanics } from '../../services/previewService';
import { failSolveJob, initialSolveJob } from '../workspace/solveJobAudit';
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
