import { knownSemanticNotices } from "./knownSemanticLimitations";
import type { ContractBinding } from "./resultReader";
import type { MechanicsResult } from "../../types";

/** Text-only frozen T0R notices for a received result. Placement is the
 * caller's; the notices never change standing, rows or exported bytes (N-A). */
export function KnownSemanticNotices({ result, testIdPrefix, recordedTableRead = null }: {
  result: MechanicsResult | null | undefined;
  testIdPrefix: string;
  /** a reopened run recorded before RT-A4's table; current callers omit it. */
  recordedTableRead?: ContractBinding | null;
}) {
  const notices = knownSemanticNotices(result, recordedTableRead);
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
