import { memo, type ReactNode } from "react";
import { PartErrorBoundary } from "./shell/PartErrorBoundary";
import { WORKSPACE_SECTIONS, type WorkspaceSectionId } from "./workspaceSections";

type DormantSectionProps = Readonly<{
  /** The page this section is: its name is the one a failure of the page shows. */
  page: WorkspaceSectionId;
  active: boolean;
  sessionGeneration: number;
  guardGeneration?: number;
  children: ReactNode;
}>;

export function dormantSectionPropsEqual(
  previous: DormantSectionProps,
  next: DormantSectionProps
): boolean {
  // A replacement session must reach retained children so their preparation
  // guards and async cleanup advance even while this section is dormant.
  if (previous.sessionGeneration !== next.sessionGeneration) return false;
  // Only preparation subtrees opt into a guard generation. Read-only report,
  // export and evidence views stay asleep across ordinary model publications.
  if (previous.guardGeneration !== next.guardGeneration) return false;
  // While inactive, retain the mounted subtree and its drafts without sending
  // ordinary model publications through expensive report/export computations.
  return !next.active;
}

// A render error in the page shows its message in the page's place and leaves the
// rest of the window working; a new project session tries the page again. The way
// back is the Model stage, except from Review changes, which is that stage's own tab.
export const DormantSection = memo(function DormantSection({ page, sessionGeneration, children }: DormantSectionProps) {
  const label = WORKSPACE_SECTIONS.find((section) => section.id === page)?.label ?? page;
  return (
    <PartErrorBoundary subject={`The ${label} page`} resetKey={sessionGeneration} back={page === "operations" ? undefined : "model-stage"}>
      {children}
    </PartErrorBoundary>
  );
}, dormantSectionPropsEqual);
