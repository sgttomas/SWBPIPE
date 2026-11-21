/** What the Results stage says when it shows no result. */

/** Before any run: there is none, and Run makes one. */
export const RESULTS_NO_RUN_TEXT = "No run yet. Run solves the model and shows its results here.";
/** The lead of a stored run the native host refused on open; the host's own sentence follows it. */
export const STORED_RUN_REFUSED_LEAD = "The stored run was not opened, so its results are not shown. The model is open: solve it again to see results.";

/** A project opened with its stored run refused by the native host: the plain statement, then the
 * host's sentence (`stored_run.message`) as the host returns it. Shown until a new solve replaces
 * the run. */
export function StoredRunRefusedNotice({ message }: { message: string }) {
  return (
    <div className="results-stored-run-refused" role="status" data-testid="results-stored-run-refused">
      <p className="results-stored-run-refused-lead">{STORED_RUN_REFUSED_LEAD}</p>
      <p data-testid="results-stored-run-refused-message">{message}</p>
    </div>
  );
}
