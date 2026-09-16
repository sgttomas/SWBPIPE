export const PAGE_CLOCK_SOURCE: Readonly<{ revision: string; timeClamperHeaderSha256: string; timeClamperImplementationSha256: string; performanceImplementationSha256: string; quantumBoundMs: number; isolatedResolutionMs: number; nonIsolatedResolutionMs: number }>;
export const REQUIRED_CHROMIUM_BINDING: Readonly<Record<string, string>>;
export class CausalPresentationExtractionError extends Error {
  code: string;
  context: Record<string, unknown>;
}
export function extractCausalPresentations(traceEvents: any[], sourceBinding: any, markerEvidence: any): any;
export function inventoryCausalPresentationLineages(traceEvents: any[], sourceBinding: any, markerEvidence: any): any;
export function presentationGapsForMeasuredWindow(extraction: any, measuredWindow: any): any;
