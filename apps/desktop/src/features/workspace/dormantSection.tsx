import { memo, type ReactNode } from "react";

type DormantSectionProps = Readonly<{
  active: boolean;
  sessionGeneration: number;
  children: ReactNode;
}>;

export function dormantSectionPropsEqual(
  previous: DormantSectionProps,
  next: DormantSectionProps
): boolean {
  // A replacement session must reach retained children so their preparation
  // guards and async cleanup advance even while this section is dormant.
  if (previous.sessionGeneration !== next.sessionGeneration) return false;
  // While inactive, retain the mounted subtree and its drafts without sending
  // ordinary model publications through expensive report/export computations.
  return !next.active;
}

export const DormantSection = memo(function DormantSection({ children }: DormantSectionProps) {
  return <>{children}</>;
}, dormantSectionPropsEqual);
