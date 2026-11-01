/** C1 interim (DEL-04-05 `c1-native-custody-plan.md` slice 4): the C2 consumers (reports,
 * comparisons, rule checks, stress-neutral and JSON derivative exports, the handoff panels) keep
 * full rows until C2 lands. For a native result they get them only through this gate: lazily, when
 * the panel is shown (or, with `explicit`, when the person asks), copied once per result at or under
 * 64 MiB and read with the legacy readers on the exact copied bytes, with the full record built on
 * that copy and checked against the native preview's checksums. Above the limit the panel shows
 * `RESULT-DETAIL-WEBVIEW-LIMIT` with the sizes, what still works and #51's advice; nothing is
 * withheld silently. A full result in the webview already (historical, browser reference) passes
 * through unchanged. No C1 surface uses this gate. */
import { useEffect, useState, type ReactNode } from "react";
import { describeFullResultRefused, FullResultRefused, isNativeAnalysisRun, isNativeResult, type SessionAnalysisRun, type SessionResult } from "../../services/nativeResult";
import { nativeFailureText } from "../../services/nativeIngress";
import type { AnalysisRunEnvelope, MechanicsResult } from "../../types";

export type C2FullResult = Readonly<{ result: MechanicsResult; analysisRun: AnalysisRunEnvelope | null }>;
/** The session's loader of a native result's C2 full copy and full record. */
export type C2FullResultLoader = (result: SessionResult) => Promise<C2FullResult>;

type GateState =
  | { status: "idle" }
  | { status: "loading" }
  | { status: "ready"; full: C2FullResult; of: SessionResult }
  | { status: "refused"; text: string }
  | { status: "failed"; text: string };

export function C2ResultGate({
  result,
  analysisRun,
  feature,
  load,
  explicit = false,
  children
}: {
  result: SessionResult | null;
  analysisRun: SessionAnalysisRun | null;
  /** The feature's name in the refusal ("the report", "comparisons", …). */
  feature: string;
  load: C2FullResultLoader;
  /** Load only when the person asks (panels shown at first paint). */
  explicit?: boolean;
  children: (full: { result: MechanicsResult | null; analysisRun: AnalysisRunEnvelope | null }) => ReactNode;
}) {
  const native = isNativeResult(result);
  const [state, setState] = useState<GateState>({ status: "idle" });
  const [requested, setRequested] = useState(false);
  const current = state.status === "ready" && state.of === result ? state : null;
  useEffect(() => {
    if (!native || !result || (explicit && !requested)) return;
    let live = true;
    setState({ status: "loading" });
    load(result).then(
      (full) => { if (live) setState({ status: "ready", full, of: result }); },
      (error) => {
        if (!live) return;
        setState(error instanceof FullResultRefused
          ? { status: "refused", text: describeFullResultRefused(error, feature) }
          : { status: "failed", text: `${feature} could not read the full result: ${nativeFailureText(error)}` });
      }
    );
    return () => { live = false; };
  }, [native, result, explicit, requested, load, feature]);
  useEffect(() => { setRequested(false); }, [result]);

  if (!native) return <>{children({ result: result as MechanicsResult | null, analysisRun: isNativeAnalysisRun(analysisRun) ? null : analysisRun })}</>;
  if (current) return <>{children({ result: current.full.result, analysisRun: current.full.analysisRun })}</>;
  if (state.status === "refused") return <p role="status" data-testid="c2-result-refused" className="result-refusal">{state.text}</p>;
  if (state.status === "failed") return <p role="alert" data-testid="c2-result-failed">{state.text}</p>;
  if (explicit && !requested) {
    return (
      <p data-testid="c2-result-explicit">
        {`The full result rows for ${feature} are read on request (they stay native until then). `}
        <button type="button" data-testid="c2-result-load" onClick={() => setRequested(true)}>Load for {feature}</button>
      </p>
    );
  }
  return <p role="status" data-testid="c2-result-loading">{`Reading the full result for ${feature}…`}</p>;
}
