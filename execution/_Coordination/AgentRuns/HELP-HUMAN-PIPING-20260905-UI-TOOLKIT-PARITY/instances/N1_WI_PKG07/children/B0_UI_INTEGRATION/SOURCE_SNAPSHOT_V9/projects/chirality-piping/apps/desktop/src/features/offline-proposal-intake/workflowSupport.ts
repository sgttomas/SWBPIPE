import { useEffect, useRef, useState } from "react";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";
import type { OperationBatch } from "../../services/operationBatchService";
export type WorkflowProps = {
  model: PreviewModel;
  selection: EntityRef;
  onQueueBatch: (batch: OperationBatch) => void;
  busy?: boolean;
  requestEpoch?: number;
};
export function draftIntent(input: Pick<EditorOperationIntent, "operation_kind" | "target" | "change" | "rationale">): EditorOperationIntent {
  return {
    ...input,
    operation_id: `operation:${crypto.randomUUID()}`,
    operation_status: "proposed",
    author_type: "user",
    validation: {
      schema_validation: "not_run", constraint_validation: "not_run", unit_validation: "not_run",
      diff_preview_status: "not_generated", application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only", direct_model_mutation_allowed: false,
      requires_user_acceptance: true, mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true, software_makes_compliance_claim: false,
      software_makes_certification_claim: false, software_makes_sealing_claim: false,
      software_makes_approval_claim: false, software_makes_authentication_claim: false
    }
  };
}
// Every asynchronous preparation is invalidated by inputs, model, selection or external busy changes.
export function usePreparation(props: WorkflowProps, inputs: unknown) {
  const basis = [props.model, props.selection.id, props.selection.type, props.busy, props.requestEpoch, inputs];
  const current = useRef(basis);
  const epoch = useRef(0);
  const revision = useRef(0);
  const inFlight = useRef(false);
  if (basis.some((value, index) => value !== current.current[index])) {
    epoch.current += 1;
    revision.current += 1;
    current.current = basis;
  }
  const [pending, setPending] = useState(false);
  const [error, setError] = useState("");
  useEffect(() => () => { epoch.current += 1; }, []);
  return {
    pending, error, setError, revision: revision.current,
    async run<T>(prepare: () => Promise<T>, consume: (result: T) => void) {
      if (props.busy || inFlight.current) return;
      inFlight.current = true;
      const token = ++epoch.current;
      setPending(true); setError("");
      try {
        const result = await prepare();
        if (epoch.current === token) consume(result);
      } catch (cause) {
        if (epoch.current === token) setError(cause instanceof Error ? cause.message : String(cause));
      } finally { inFlight.current = false; setPending(false); }
    }
  };
}
