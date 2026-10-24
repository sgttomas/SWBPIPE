// The rule-pack references a run's input manifest records (FR-CMP-002): the
// model's references that resolved to a stored pack with the same checksum at
// solve time, and each excluded reference with its diagnostic. Shown with the
// solve, the Record Run form and a recorded run's check. Identity only: the
// solve neither reads nor applies a pack, and a reference never makes a check
// pass (F-PIP-2).
import type { CurrentSessionInputManifest } from "../../services/inputManifestService";
import { manifestRulePackLines } from "../../services/modelRulePackResolution";

export const RULE_PACK_RUN_REFERENCES_NOTE =
  "Identity only: the solve does not read, apply or check a rule pack, and a reference never makes a check pass.";

/** Nothing when the solved model referenced no rule pack. */
export function RulePackRunReferences({ manifest, testId }: { manifest: CurrentSessionInputManifest | null | undefined; testId: string }) {
  const referenced = manifest?.model_basis?.model_payload?.rule_pack_refs?.length ?? 0;
  if (!manifest || referenced === 0) return null;
  const { recorded, diagnostics } = manifestRulePackLines(manifest);
  return (
    <div className="report-list" data-testid={testId}>
      <strong>Rule-pack references recorded with this run</strong>
      <small>{RULE_PACK_RUN_REFERENCES_NOTE}</small>
      {recorded.length
        ? <ul data-testid={`${testId}-recorded`}>{recorded.map((line) => <li key={line}>{line}</li>)}</ul>
        : <small data-testid={`${testId}-none`}>None of the model's rule-pack references resolved to a stored pack with the same checksum.</small>}
      {diagnostics.map((line) => <p key={line} role="note" data-testid={`${testId}-diagnostic`}>{line}</p>)}
    </div>
  );
}
