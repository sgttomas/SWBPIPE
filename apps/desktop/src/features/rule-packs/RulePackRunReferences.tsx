// The rule-pack references a run records (FR-CMP-002): the packs its input
// manifest's `active_rule_packs` holds (the model's references that resolved to a
// stored pack with the same checksum at solve time), and each excluded reference's
// diagnostic, read from the run record's diagnostics. Shown with the solve, the
// Record Run form and a recorded run's check. Identity only: the solve neither
// reads nor applies a pack, and a reference never makes a check pass (F-PIP-2).
import type { SessionAnalysisRun } from "../../services/nativeResult";
import type { AnalysisRunEnvelope } from "../../types";
import type { CurrentSessionInputManifest } from "../../services/inputManifestService";
import { runRulePackLines } from "../../services/modelRulePackResolution";

export const RULE_PACK_RUN_REFERENCES_NOTE =
  "Identity only: the solve does not read, apply or check a rule pack, and a reference never makes a check pass.";

/** Nothing when the solved model referenced no rule pack. */
export function RulePackRunReferences({ manifest, record, testId }: {
  manifest: CurrentSessionInputManifest | null | undefined;
  record: SessionAnalysisRun | null | undefined;
  testId: string;
}) {
  const lines = runRulePackLines(manifest, record);
  if (!lines) return null;
  return (
    <div className="report-list" data-testid={testId}>
      <strong>Rule-pack references recorded with this run</strong>
      <small>{RULE_PACK_RUN_REFERENCES_NOTE}</small>
      {lines.recorded.length
        ? <ul data-testid={`${testId}-recorded`}>{lines.recorded.map((line) => <li key={line}>{line}</li>)}</ul>
        : <small data-testid={`${testId}-none`}>None of the model's rule-pack references resolved to a stored pack with the same checksum.</small>}
      {lines.diagnostics.map((line) => <p key={line} role="note" data-testid={`${testId}-diagnostic`}>{line}</p>)}
    </div>
  );
}
