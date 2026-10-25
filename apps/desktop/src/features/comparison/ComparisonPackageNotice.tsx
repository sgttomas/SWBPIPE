import { packageNotice, type WithinRunComparisonBuild } from "./withinRunComparison";

/** W-12 S3c: which within-run comparison the report, handoff and export packages
 * carry, or that they omit it and why. Shown in the Comparison panel and beside the
 * package actions, so the user need not remember an earlier choice. */
export function ComparisonPackageNotice({ packaged, testId }: { packaged: WithinRunComparisonBuild | null; testId: string }) {
  const text = packageNotice(packaged);
  if (!text) return null;
  return <p className="muted" role="note" data-testid={testId}>{text}</p>;
}
