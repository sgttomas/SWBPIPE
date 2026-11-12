// "Include in report" on a comparison view (DEL-08-06 REQ-015, AC-009): includes the
// comparison shown in the next local-private report package saved, or removes it. A
// native button, so it is operable by keyboard; its state is stated in words beside it
// and as its pressed state. The session holds what is included
// (`useModelStateHistory` `includeComparisonInReport`, `removeComparisonFromReport`).

/** What a comparison view needs to offer the control. */
export type ReportInclusion = {
  included: boolean;
  include: () => void;
  remove: () => void;
};

export function ReportInclusionControl({ inclusion, kind, testId }: { inclusion: ReportInclusion; kind: "saved-state" | "run"; testId: string }) {
  const { included } = inclusion;
  return (
    <div className="report-actions" role="group" aria-label={`Report inclusion of this ${kind} comparison`} data-testid={testId}>
      <button
        type="button"
        aria-pressed={included}
        data-testid={`${testId}-toggle`}
        onClick={included ? inclusion.remove : inclusion.include}
      >
        {included ? "Remove from report" : "Include in report"}
      </button>
      <span className="muted" role="status" data-testid={`${testId}-state`}>
        {included
          ? `Included: the next local private report package saved holds this ${kind} comparison.`
          : `Not included in a report package.`}
      </span>
    </div>
  );
}
