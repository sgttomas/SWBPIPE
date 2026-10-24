import type { PreviewModel } from "../../types";
import type { OperationBatch } from "../../services/operationBatchService";
import { validateOperationBatch } from "../../services/operationBatchService";
import { computeModelHash } from "../../services/hashService";
import { buildReconnectBatch, emptyGeometryDraft, RECONNECT_CHANGE_KIND } from "./geometryDraft";

/** W-10 capability reference (DEL-16-06 Design `pipe-reconnect.md` §1): the
 * schema's `OperationChange` is not the intake shape, so the reference carries
 * the wire constants and one complete batch for the open model, built by the
 * Geometry tools builder and checked by the shared engine's batch validation.
 * Bounded and on demand: at most 32 candidates, cached against the model hash. */
export const RECONNECT_EXAMPLE_CANDIDATE_CAP = 32;
export const RECONNECT_WIRE = {
  operation_kind: "modify",
  target: { object_type: "Element", ref: "<pipe id>" },
  change_kind: RECONNECT_CHANGE_KIND,
  field_path: "pipe_segments",
  unit: "none",
  dimension: "dimensionless",
  before: "canonical JSON (RFC 8785) of the pipe's current record",
  after: "JSON text {\"pipe_ref\": target.ref, \"end\": \"from\" | \"to\", \"target_node_ref\": \"<existing node id>\", \"y_reference\"?: {\"x\", \"y\", \"z\"}}; unknown fields are refused",
  new_point: "queue [create_node, reconnect_pipe_run] in one batch",
  preview: "a batch containing reconnect_pipe_run is applied only after a clean Validate"
} as const;
export type ReconnectOperationExample =
  | { wire: typeof RECONNECT_WIRE; model_hash: string; batch: OperationBatch; candidates_checked: number }
  | { wire: typeof RECONNECT_WIRE; model_hash: string; omitted_reason: string; candidates_checked: number };

const cache = new Map<string, Promise<ReconnectOperationExample>>();
export function clearReconnectExampleCache() {
  cache.clear();
}

/** Deterministic candidates: pipes by id, `to` then `from`, targets by node id. */
export function reconnectCandidates(model: PreviewModel): Array<{ pipe: string; end: "from" | "to"; target: string }> {
  const pipes = [...model.pipe_segments].sort((a, b) => (a.id < b.id ? -1 : a.id > b.id ? 1 : 0));
  const nodes = model.nodes.map((n) => n.id).sort();
  const out: Array<{ pipe: string; end: "from" | "to"; target: string }> = [];
  for (const pipe of pipes) {
    for (const end of ["to", "from"] as const) {
      for (const target of nodes) {
        if (target === pipe.from || target === pipe.to) continue;
        out.push({ pipe: pipe.id, end, target });
        if (out.length === RECONNECT_EXAMPLE_CANDIDATE_CAP) return out;
      }
    }
  }
  return out;
}

async function search(model: PreviewModel, hashValue: string): Promise<ReconnectOperationExample> {
  const hash = await computeModelHash(model);
  if (!hash) return { wire: RECONNECT_WIRE, model_hash: hashValue, omitted_reason: "the model hash is unavailable", candidates_checked: 0 };
  const candidates = reconnectCandidates(model);
  let checked = 0;
  for (const candidate of candidates) {
    checked += 1;
    const draft = { ...emptyGeometryDraft(), action: "reconnect" as const, pipeRefs: [candidate.pipe], reconnectEnd: candidate.end, targetNode: candidate.target };
    let batch: OperationBatch;
    try {
      batch = await buildReconnectBatch(model, draft, "batch:capability-example:reconnect");
    } catch {
      continue;
    }
    const outcome = await validateOperationBatch(model, batch, hash);
    const blocked = outcome.validation.batch_validation_status !== "passed"
      || outcome.diagnostics.some((d) => d.severity === "blocking")
      || outcome.operation_outcomes.some((step) => step.diagnostics.some((d) => d.severity === "blocking"));
    if (!blocked) return { wire: RECONNECT_WIRE, model_hash: hash.value, batch, candidates_checked: checked };
  }
  const omitted_reason = candidates.length < RECONNECT_EXAMPLE_CANDIDATE_CAP
    ? `no valid reconnect found among the ${candidates.length} candidates of this model`
    : `no valid reconnect found in the first ${RECONNECT_EXAMPLE_CANDIDATE_CAP} candidates`;
  return { wire: RECONNECT_WIRE, model_hash: hash.value, omitted_reason, candidates_checked: checked };
}

/** On demand only (never on render); cached by the model hash. */
export async function reconnectOperationExample(model: PreviewModel): Promise<ReconnectOperationExample> {
  const hash = await computeModelHash(model);
  const key = hash?.value ?? "unavailable";
  const cached = cache.get(key);
  if (cached) return cached;
  const pending = search(model, key);
  cache.set(key, pending);
  pending.catch(() => cache.delete(key));
  return pending;
}

/** The capability reference's `operation_examples` member. */
export async function operationExamples(model: PreviewModel): Promise<Record<string, unknown>> {
  return { reconnect_pipe_run: await reconnectOperationExample(model) };
}
