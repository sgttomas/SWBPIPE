/** The received-result hash scope of the one current analysis-run version (0.3.0).
 * A retired (0.1.0, 0.2.0) or unknown record has no scope and is refused by its caller. */
export function analysisResultHashScope(version: string | null | undefined): "received_result" | null {
  return version === "0.3.0" ? "received_result" : null;
}
