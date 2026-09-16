import { memo, type ReactNode } from "react";

type DormantSectionProps = Readonly<{
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

export const DormantSection = memo(function DormantSection({ children }: DormantSectionProps) {
  return <>{children}</>;
}, dormantSectionPropsEqual);
