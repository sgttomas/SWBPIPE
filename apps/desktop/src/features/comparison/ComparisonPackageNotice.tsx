import { droppedNotice, packageNotice, type WithinRunComparisonBuild } from "./withinRunComparison";

/** W-12 S3c: which within-run comparison the report, handoff and export packages
 * carry, or that they omit it and why, and, when the user's choice was dropped, which
 * bases and why. Shown in the Comparison panel and beside the package actions in
 * Report and Exports, so the user need not remember an earlier choice or visit
 * Results to learn it was dropped. */
export function ComparisonPackageNotice({ packaged, dropped = null, testId }: { packaged: WithinRunComparisonBuild | null; dropped?: string[] | null; testId: string }) {
  const text = packageNotice(packaged);
  if (!text) return null;
  return (
    <div role="note" data-testid={testId}>
      {dropped && dropped.length > 0 ? <p role="status" data-testid={`${testId}-dropped`}>{droppedNotice(dropped)}</p> : null}
      <p className="muted" data-testid={`${testId}-text`}>{text}</p>
    </div>
  );
}
