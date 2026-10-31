/** Operation authorship on routes that carry agent or imported content
 * (DEL-16-03 offline intake; DEL-16-06 Design `embedded-agent-host.md`,
 * "Author routes"). The applier refuses an agent-authored write of a value the
 * person enters (bend flexibility factor k, SIFs and other component
 * modifiers) by reading `author_type`, which the operation boundary cannot
 * verify. So every route that carries agent or imported content stamps
 * `agent` here, host-side, and never passes a claimed author through. */
import type { EditorOperationIntent } from "../types";
import type { OperationBatch } from "./operationBatchService";

/** Display-only record of what an imported file claimed; never part of an intent. */
export type ImportContext = Readonly<{
  kind: "offline_intake";
  fileBatchId: string;
  claimed: readonly Readonly<{ operationId: string; authorType: unknown; source: unknown }>[];
}>;

const plain = (value: unknown): value is Record<string, unknown> => !!value && typeof value === "object" && !Array.isArray(value);
const parsed = (text: unknown): unknown => {
  if (typeof text !== "string") return undefined;
  try { return JSON.parse(text); } catch { return undefined; }
};

/** Offline intake: every imported operation becomes agent-authored with a host
 * source; the file's claimed author and source are kept beside the batch for
 * display only. A non-object entry is left for the engine to refuse. */
export function stampImportedBatch(batch: OperationBatch): { batch: OperationBatch; context: ImportContext } {
  const claimed = batch.operations.map((operation, index) => {
    const record = plain(operation) ? operation as Record<string, unknown> : {};
    return Object.freeze({
      operationId: typeof record.operation_id === "string" ? record.operation_id : `#${index + 1}`,
      authorType: structuredClone(record.author_type ?? null),
      source: structuredClone(record.source ?? null)
    });
  });
  const operations = batch.operations.map((operation, index) => plain(operation)
    ? { ...structuredClone(operation), author_type: "agent" as const,
        source: { source_ref: `offline-import:${batch.batch_id}:${index + 1}`, source_channel: "offline_intake", source_role: "imported" } }
    : operation) as EditorOperationIntent[];
  return { batch: { ...structuredClone(batch), operations }, context: Object.freeze({ kind: "offline_intake", fileBatchId: batch.batch_id, claimed: Object.freeze(claimed) }) };
}

/** True when the operation writes a value only the person enters, by the same
 * routes the applier refuses for a non-user author. */
export function writesUserEnteredModifier(intent: unknown): boolean {
  if (!plain(intent)) return false;
  const change = plain(intent.change) ? intent.change : {};
  const target = plain(intent.target) ? intent.target : {};
  const kind = String(change.change_kind ?? ""), path = String(change.field_path ?? ""), object = String(target.object_type ?? "");
  if (kind === "insert_bend_at_corner") return true;
  if (object === "Component" && (path === "bend_realization" || path.startsWith("modifiers."))) return true;
  const after = parsed(change.after);
  if (kind === "insert_component_symbol" && plain(after) && "modifiers" in after) return true;
  if (kind === "apply_library_record" && plain(after) && Object.keys(after).some(key => key.startsWith("modifiers."))) return true;
  return false;
}

const AGENT_OR_IMPORTED_CHANNELS = ["embedded_agent", "offline_intake", "offline_user_supplied", "offline"];
export type RequeueResult = { batch: OperationBatch } | { refused: string };

/** Requeue a stored record from a reopened project. Saved records keep no
 * batch context, so the author is read from the record itself: agent or
 * imported content is re-stamped `agent`; a `user` record that writes a value
 * only the person enters is refused, because its author cannot be verified
 * after reopening; a record that names no recognizable author is refused. */
export function requeueRetainedRecord(intent: unknown, index: number): RequeueResult {
  if (!plain(intent)) return { refused: "This stored record is not an operation and cannot be requeued." };
  const source = plain(intent.source) ? intent.source : null;
  const channel = typeof source?.source_channel === "string" ? source.source_channel : "";
  const agentOrImported = intent.author_type === "agent" || AGENT_OR_IMPORTED_CHANNELS.includes(channel)
    || (typeof source?.source_role === "string" && ["agent", "imported"].includes(source.source_role));
  const batch_id = `requeue-${index}`;
  if (agentOrImported) {
    return { batch: { batch_id, operations: [{ ...structuredClone(intent), author_type: "agent",
      source: { source_ref: typeof source?.source_ref === "string" && source.source_ref ? source.source_ref : `retained:${index}`, source_channel: channel || "retained_requeue", source_role: "agent" } } as unknown as EditorOperationIntent] } };
  }
  if (intent.author_type === "user") {
    if (writesUserEnteredModifier(intent)) return { refused: "This stored record writes a value only you enter (a flexibility factor k, a SIF or another component modifier), and its author cannot be verified after reopening. Enter the value again in the component inspector." };
    return { batch: { batch_id, operations: [structuredClone(intent) as unknown as EditorOperationIntent] } };
  }
  return { refused: "This stored record does not say who proposed it, so it cannot be requeued. Prepare the change again." };
}
