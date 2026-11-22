import { droppedNotice, packageNotice, type WithinRunComparisonBuild } from "./withinRunComparison";

/** W-12 S3c: which within-run comparison the local-private report package carries, or
 * that it carries none and why, and, when the user's choice was dropped, which bases and
 * why. Shown in the Comparison panel and beside the package actions in Report (the
 * Exports page carries no comparison), so the user need not remember an earlier choice
 * or visit Results to learn it was dropped. Without a package sentence (no Current run, or no
 * fixed pair in the run without a choice), a dropped choice is still named. */
export function ComparisonPackageNotice({ packaged, dropped = null, testId }: { packaged: WithinRunComparisonBuild | null; dropped?: string[] | null; testId: string }) {
  const text = packageNotice(packaged);
  const named = dropped && dropped.length > 0 ? dropped : null;
  if (!text && !named) return null;
  return (
    <div role="note" data-testid={testId}>
      {named ? <p role="status" data-testid={`${testId}-dropped`}>{droppedNotice(named)}</p> : null}
      {text ? <p className="muted" data-testid={`${testId}-text`}>{text}</p> : null}
    </div>
  );
}
