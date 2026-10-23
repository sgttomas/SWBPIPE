/** "Show solver diagnostic" (DEL-07-05 numerical health §3): a one-time request that the
 * Issues drawer's Diagnostics panel open filtered to exactly one diagnostic. It is
 * consumed once: cleared when the drawer closes and when the result or the run changes,
 * so a later ordinary opening of Issues is unfiltered. */
import { useCallback, useEffect, useRef, useState } from "react";

export type DiagnosticFilterRequest = Readonly<{ diagnosticId: string; sequence: number }>;

export function useDiagnosticFilterRequest({ issuesDrawerOpen, result, analysisRun }: {
  issuesDrawerOpen: boolean;
  result: unknown;
  analysisRun: unknown;
}): { filterRequest: DiagnosticFilterRequest | null; requestDiagnosticFilter: (diagnosticId: string) => void } {
  const [filterRequest, setFilterRequest] = useState<DiagnosticFilterRequest | null>(null);
  const sequence = useRef(0);
  useEffect(() => { if (!issuesDrawerOpen) setFilterRequest(null); }, [issuesDrawerOpen]);
  useEffect(() => { setFilterRequest(null); }, [result, analysisRun]);
  const requestDiagnosticFilter = useCallback((diagnosticId: string) => {
    sequence.current += 1;
    setFilterRequest({ diagnosticId, sequence: sequence.current });
  }, []);
  return { filterRequest, requestDiagnosticFilter };
}
