import { knownSemanticNotices } from "./knownSemanticLimitations";
import type { MechanicsResult } from "../../types";

/** Text-only frozen T0R notices for a received result (S1 §10). Placement is the
 * caller's; the notices never change standing, rows or exported bytes (N-A). */
export function KnownSemanticNotices({ result, testIdPrefix }: { result: MechanicsResult | null | undefined; testIdPrefix: string }) {
  const notices = knownSemanticNotices(result);
  if (!notices.length) return null;
  return (
    <>
      {notices.map((notice) => (
        <p className="muted" key={notice.id} data-testid={`${testIdPrefix}-notice-${notice.id}`}>
          {notice.text}
        </p>
      ))}
    </>
  );
}
