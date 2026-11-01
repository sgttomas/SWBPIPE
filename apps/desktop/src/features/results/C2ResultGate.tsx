/** C1 interim (DEL-04-05 `c1-native-custody-plan.md` slice 4): the C2 consumers (reports,
 * comparisons, rule checks, stress-neutral and JSON derivative exports, the handoff panels) keep
 * full rows until C2 lands. For a native result they get them only through this gate: lazily, when
 * the panel is shown (or, with `explicit`, when the person asks), copied once per result at or under
 * 64 MiB and read with the legacy readers on the exact copied bytes, with the full record built on
 * that copy and checked against the native preview's checksums. Above the limit the panel shows
 * `RESULT-DETAIL-WEBVIEW-LIMIT` with the sizes, what still works and #51's advice; nothing is
 * withheld silently. A full result in the webview already (historical, browser reference) passes
 * through unchanged. No C1 surface uses this gate. */
import { useEffect, useRef, useState, type ReactNode } from "react";
import { describeFullResultRefused, FullResultRefused, isNativeAnalysisRun, isNativeResult, type SessionAnalysisRun, type SessionResult } from "../../services/nativeResult";
import { nativeFailureText } from "../../services/nativeIngress";
import { useOptionalSessionResults } from "../workspace/WorkspaceSessionContext";
import type { AnalysisRunEnvelope, MechanicsResult } from "../../types";

export type C2FullResult = Readonly<{ result: MechanicsResult; analysisRun: AnalysisRunEnvelope | null }>;
/** The session's loader of a native result's C2 full copy and full record. */
export type C2FullResultLoader = (result: SessionResult) => Promise<C2FullResult>;

type GateState =
  | { status: "idle" }
  | { status: "loading"; of: SessionResult; run: SessionAnalysisRun | null }
  | { status: "ready"; full: C2FullResult; of: SessionResult; run: SessionAnalysisRun | null }
  | { status: "refused"; text: string; of: SessionResult; run: SessionAnalysisRun | null }
  | { status: "failed"; text: string; of: SessionResult; run: SessionAnalysisRun | null };

export function C2ResultGate({
  result,
  analysisRun,
  feature,
  load: givenLoad,
  explicit = false,
  children
}: {
  result: SessionResult | null;
  analysisRun: SessionAnalysisRun | null;
  /** The feature's name in the refusal ("the report", "comparisons", …). */
  feature: string;
  /** The loader; the session's when omitted. */
  load?: C2FullResultLoader;
  /** Load only when the person asks (panels shown at first paint). */
  explicit?: boolean;
  children: (full: { result: MechanicsResult | null; analysisRun: AnalysisRunEnvelope | null }) => ReactNode;
}) {
  const sessionResults = useOptionalSessionResults();
  // The loader is read when a load starts, never as a dependency: the session's loader is a new
  // function on each session render, and a load must not restart (and copy again) for that.
  const loadRef = useRef<C2FullResultLoader | null>(null);
  loadRef.current = givenLoad ?? sessionResults?.loadC2Full ?? null;
  const native = isNativeResult(result);
  const [state, setState] = useState<GateState>({ status: "idle" });
  // A request is for the result it was made on; a new result needs its own.
  const [requestedFor, setRequestedFor] = useState<SessionResult | null>(null);
  const requested = requestedFor !== null && requestedFor === result;
  // Each state is for the result and record it was reached on (a revised record, such as a rule-check
  // aggregate's, gets its own full record built on the same copy); another's outcome is never shown.
  const shown = state.status !== "idle" && state.of === result && state.run === analysisRun ? state : null;
  useEffect(() => {
    if (!native || !result || (explicit && !requested)) return;
    const load = loadRef.current;
    if (!load) {
      setState({ status: "failed", text: `${feature} has no route to the full result here.`, of: result, run: analysisRun });
      return;
    }
    let live = true;
    setState({ status: "loading", of: result, run: analysisRun });
    load(result).then(
      (full) => { if (live) setState({ status: "ready", full, of: result, run: analysisRun }); },
      (error) => {
        if (!live) return;
        setState(error instanceof FullResultRefused
          ? { status: "refused", text: describeFullResultRefused(error, feature), of: result, run: analysisRun }
          : { status: "failed", text: `${feature} could not read the full result: ${nativeFailureText(error)}`, of: result, run: analysisRun });
      }
    );
    return () => { live = false; };
  }, [native, result, analysisRun, explicit, requested, feature]);

  if (!native) return <>{children({ result: result as MechanicsResult | null, analysisRun: isNativeAnalysisRun(analysisRun) ? null : analysisRun })}</>;
  if (shown?.status === "ready") return <>{children({ result: shown.full.result, analysisRun: shown.full.analysisRun })}</>;
  if (shown?.status === "refused") return <p role="status" data-testid="c2-result-refused" className="result-refusal">{shown.text}</p>;
  if (shown?.status === "failed") return <p role="alert" data-testid="c2-result-failed">{shown.text}</p>;
  if (explicit && !requested) {
    return (
      <p data-testid="c2-result-explicit">
        {`The full result rows for ${feature} are read on request (they stay native until then). `}
        <button type="button" data-testid="c2-result-load" onClick={() => setRequestedFor(result)}>Load for {feature}</button>
      </p>
    );
  }
  return <p role="status" data-testid="c2-result-loading">{`Reading the full result for ${feature}…`}</p>;
}
