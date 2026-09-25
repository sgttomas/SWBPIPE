/** Exact analysis-record versions; unknown records never inherit legacy scope. */
export function analysisResultHashScope(version: string | null | undefined): "result_envelope" | "received_result" | null {
  switch (version) {
    case "0.1.0": return "result_envelope";
    case "0.2.0":
    case "0.3.0": return "received_result";
    default: return null;
  }
}
