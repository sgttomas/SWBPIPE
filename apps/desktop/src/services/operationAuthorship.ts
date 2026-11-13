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
  // An imported operation carrying a value only the person enters is refused before it is
  // stamped, so no batch that carries one leaves intake; no value from the file is repeated.
  const entered = personEnteredWriteIn(batch.operations);
  if (entered) {
    const which = `Operation ${entered.index + 1}${entered.operationId && entered.operationId.length <= 160 ? ` (${entered.operationId})` : ""} of this file`;
    if (entered.route === "unchecked") throw new Error(`${which} carries ${entered.label}, so nothing was queued. Split it into smaller operations.`);
    if (entered.path) throw new Error(`${which} has a value that names ${entered.label}, which only you enter. ${INDISTINGUISHABLE}, so nothing was queued. Remove it from the file, then enter the value yourself in the component inspector.`);
    throw new Error(`${which} carries ${entered.label}, a value only you enter; an imported operation cannot write it, so nothing was queued. Remove it from the file, then enter the value yourself in the component inspector.`);
  }
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

/** A member of an operation that writes, or would carry, a value only the person enters:
 * which member, how the applier treats it for an operation a person did not author, and a
 * name for it that carries no value from the operation.
 * - `refused`: the applier refuses it by name for a non-user author (its `writes_flexibility`
 *   and `writes_modifier` expressions in core/model_operations/operation_applier/src/lib.rs;
 *   embeddedAgentSession.records.test.ts fails if their literals change).
 * - `dropped`: the applier writes the record without it, for any author: a `modifiers` or
 *   `bend_realization` member at the top level of another operation's after object, and a
 *   curved-bend `mechanics_interface` (a bend realization; refused on rigid components and the
 *   expansion joint, left out of a bend or tee).
 * - `unchecked`: the after text is longer, or its value deeper or wider, than the check reads
 *   (`reason`), so it is refused for that reason, not passed unread.
 * The after value is walked whole, through objects and arrays and a quantity edit's value,
 * because a creator that rebuilds a nested record (a combination's terms, a node's position, a
 * component's geometry) drops a nested member silently while the review row repeats the
 * proposed text. The walk is breadth-first, so the shallowest member is named, with its path
 * (`terms[0].modifiers`); no value is ever part of the result.
 * A field the engine writes as literal text (a label) is parsed like any other after text: the
 * web view has no list of such fields (only the engine's field rules know them), so a text that
 * reads as JSON naming such a member cannot be told apart from the member and is refused in
 * words that claim no structure (`path` is set: "has a value that names …"). */
export type PersonEnteredWrite = Readonly<{ member: string; route: "refused" | "dropped" | "unchecked"; reason?: "size" | "depth" | "count"; label: string; path: string }>;
const named = (path: string) => path.length <= 80 ? `${path} (a component modifier)` : "a component modifier (k, a SIF, a stiffness or their source)";
const INDISTINGUISHABLE = "A value naming it cannot be told apart from it, even as text";
/** The next power of two at or above `n`. */
const nextPowerOfTwo = (n: number) => 2 ** Math.ceil(Math.log2(n));
/** The check's bounds, derived here only. The measurement: the person's self-weight refresh on
 * the 2,000-node rack of release exit criterion 9 (linear-ex, the performance harness's
 * rack_model.py) writes one after text of 5,175,300 characters for its case, over which the walk
 * reads 25,717 entries (the root included). Each bound is the next power of two at or above
 * `factor` times its measurement; change the measurement, not the bounds. The depth bound is
 * not measured: product records nest at most a few levels (3 in that record). `maxAfterChars`
 * counts characters as String.length does (UTF-16 code units). `atRequeue` is the one place that
 * applies the length to stored records at requeue (false: intake and the agent only). */
export const PERSON_ENTERED_CHECK = (() => {
  const measuredEntries = 25_717, measuredChars = 5_175_300, factor = 4;
  return Object.freeze({ measuredEntries, measuredChars, factor, maxDepth: 32,
    maxMembers: nextPowerOfTwo(factor * measuredEntries), maxAfterChars: nextPowerOfTwo(factor * measuredChars), atRequeue: true });
})();
const LABELS = { modifiers: "modifiers (k, SIFs, stiffness and their source)", bend_realization: "bend_realization (a bend realization)",
  mechanics_interface: "a curved-bend mechanics_interface (a bend realization)" } as const;
const UNCHECKED = {
  size: `an after value longer than ${PERSON_ENTERED_CHECK.maxAfterChars} characters`,
  depth: `an after value nested deeper than ${PERSON_ENTERED_CHECK.maxDepth} levels`,
  count: `an after value with more than ${PERSON_ENTERED_CHECK.maxMembers} members`,
} as const;
const step = (key: string) => /^[A-Za-z_][A-Za-z0-9_]*$/.test(key) ? `.${key}` : `[${JSON.stringify(key)}]`;
const shown = (path: string) => path.length <= 120 ? path : `${path.slice(0, 119)}…`;

type Found = { member: keyof typeof LABELS; path: string };
/** The shallowest person-entered member anywhere in a parsed after value, or the bound it passes.
 * Each member below the root (the root is level 0) is admitted against the bounds as it is read,
 * so at most one member past the count is read, however wide an array or object is; only
 * containers wait in the queue, each with its parent and key, and a path is built only for the
 * member found. `observe.queued` reports how many containers were queued. An object's own keys
 * are enumerated by the JavaScript engine (for…in), whose work is bounded by the text's length. */
export function findPersonEntered(root: unknown, observe?: { queued: number }): Found | { unchecked: "depth" | "count" } | null {
  const { maxDepth, maxMembers } = PERSON_ENTERED_CHECK;
  const values: unknown[] = [root], depths: number[] = [0], parents: number[] = [-1], keys: (string | number)[] = [""];
  let members = 0;
  const pathOf = (parent: number, key: string | number) => {
    const parts: string[] = [typeof key === "number" ? `[${key}]` : step(key)];
    for (let at = parent; at > 0; at = parents[at]) parts.push(typeof keys[at] === "number" ? `[${keys[at]}]` : step(keys[at] as string));
    const path = parts.reverse().join("");
    return path.startsWith(".") ? path.slice(1) : path;
  };
  /** Admit one member: refuse past a bound, queue it if it is a container. */
  const admit = (value: unknown, depth: number, parent: number, key: string | number): "depth" | "count" | null => {
    if (++members > maxMembers) return "count";
    if (depth > maxDepth) return "depth";
    if (value !== null && typeof value === "object") { values.push(value); depths.push(depth); parents.push(parent); keys.push(key); }
    return null;
  };
  try {
    for (let index = 0; index < values.length; index++) {
      const value = values[index], depth = depths[index] + 1;
      if (Array.isArray(value)) {
        for (let position = 0; position < value.length; position++) {
          const bound = admit(value[position], depth, index, position);
          if (bound) return { unchecked: bound };
        }
        continue;
      }
      if (!plain(value)) continue;
      for (const key in value) {
        if (!Object.hasOwn(value, key)) continue;
        const member = value[key];
        if (key === "modifiers" || key === "bend_realization") return { member: key, path: pathOf(index, key) };
        if (key === "mechanics_interface" && plain(member) && member.solver_consumption === "curved_bend_macro_element") return { member: key, path: pathOf(index, key) };
        const bound = admit(member, depth, index, key);
        if (bound) return { unchecked: bound };
      }
    }
    return null;
  } finally {
    if (observe) observe.queued = values.length;
  }
}

/** The one list of person-entered members, used by every entry that queues an operation not
 * authored by the person: the agent's executor (embeddedAgentSession.ts), offline intake
 * (stampImportedBatch) and requeue (requeueRetainedRecord). The member it carries, or null.
 * The after text's length is read before it is parsed; `maxAfterChars` is the requeue switch's
 * way in (PERSON_ENTERED_CHECK.atRequeue).
 *
 * A producer of the schema's other non-user authors, `import_adapter` and `project_template`
 * (none writes an operation yet), must call this the day it writes one: the applier writes no
 * such member for an author that is not the user, and until it refuses one (after PR #217) this is the gate. */
export function writesUserEnteredModifier(intent: unknown, maxAfterChars: number = PERSON_ENTERED_CHECK.maxAfterChars): PersonEnteredWrite | null {
  if (!plain(intent)) return null;
  const change = plain(intent.change) ? intent.change : {};
  const target = plain(intent.target) ? intent.target : {};
  const kind = String(change.change_kind ?? ""), path = String(change.field_path ?? ""), object = String(target.object_type ?? "");
  if (kind === "insert_bend_at_corner") return { member: kind, route: "refused", label: "a bend insertion (insert_bend_at_corner)", path: "" };
  if (object === "Component" && path === "bend_realization") return { member: path, route: "refused", label: LABELS.bend_realization, path: "" };
  if (object === "Component" && path.startsWith("modifiers.")) return { member: path, route: "refused", label: named(path), path: "" };
  const unchecked = (reason: keyof typeof UNCHECKED): PersonEnteredWrite =>
    ({ member: "", route: "unchecked", reason, path: "", label: `${UNCHECKED[reason]}, which is not checked for values only the person enters` });
  if (typeof change.after === "string" && change.after.length > maxAfterChars) return unchecked("size");
  const after = parsed(change.after);
  if (kind === "apply_library_record" && plain(after)) {
    for (const key in after) if (Object.hasOwn(after, key) && key.startsWith("modifiers.")) return { member: key, route: "refused", label: named(key), path: "" };
  }
  const found = findPersonEntered(after);
  if (found === null) return null;
  if ("unchecked" in found) return unchecked(found.unchecked);
  const top = found.path === found.member;
  return { member: found.member, path: found.path, route: kind === "insert_component_symbol" && top && found.member === "modifiers" ? "refused" : "dropped",
    label: top ? LABELS[found.member] : `${LABELS[found.member]} at ${shown(found.path)}` };
}

/** The first operation of a batch that carries a person-entered member, with its position. */
export function personEnteredWriteIn(operations: readonly unknown[]): (PersonEnteredWrite & { index: number; operationId: string | null }) | null {
  for (const [index, operation] of operations.entries()) {
    const found = writesUserEnteredModifier(operation);
    if (found) return { ...found, index, operationId: plain(operation) && typeof operation.operation_id === "string" ? operation.operation_id : null };
  }
  return null;
}

const AGENT_OR_IMPORTED_CHANNELS = ["embedded_agent", "offline_intake", "offline_user_supplied", "offline"];
export type RequeueResult = { batch: OperationBatch } | { refused: string };
const KEPT = "It was not queued for review; the record stays stored and nothing of yours is lost.";

/** Requeue a stored record from a reopened project. Saved records keep no
 * batch context, so the author is read from the record itself: agent or
 * imported content is re-stamped `agent`; a `user` record that writes a value
 * only the person enters is refused, because its author cannot be verified
 * after reopening; a record that names no recognizable author is refused.
 * The check reads every stored record, whatever author it claims. */
export function requeueRetainedRecord(intent: unknown, index: number): RequeueResult {
  if (!plain(intent)) return { refused: "This stored record is not an operation and cannot be requeued." };
  const source = plain(intent.source) ? intent.source : null;
  const channel = typeof source?.source_channel === "string" ? source.source_channel : "";
  const agentOrImported = intent.author_type === "agent" || AGENT_OR_IMPORTED_CHANNELS.includes(channel)
    || (typeof source?.source_role === "string" && ["agent", "imported"].includes(source.source_role));
  const batch_id = `requeue-${index}`;
  const entered = writesUserEnteredModifier(intent, PERSON_ENTERED_CHECK.atRequeue ? PERSON_ENTERED_CHECK.maxAfterChars : Number.POSITIVE_INFINITY);
  if (entered?.route === "unchecked") {
    const change = plain(intent.change) ? intent.change : {};
    if (change.change_kind === "update_load" && change.field_path === "generated_self_weight") {
      return { refused: `This stored record is a self-weight refresh that carries ${entered.label}. ${KEPT} Refresh the self-weight from the model instead: in Plan self-weight loads, choose "Refresh an existing case", choose the case, then "Generate self-weight refresh plan".` };
    }
    return { refused: `This stored record carries ${entered.label}. ${KEPT} Prepare the change again in smaller operations.` };
  }
  if (entered && entered.path && (agentOrImported || entered.route === "dropped")) {
    return { refused: `This stored record has a value that names ${entered.label}, which only you enter. ${INDISTINGUISHABLE}. ${KEPT} Enter the value yourself in the component inspector, or prepare the change again without it.` };
  }
  if (entered && agentOrImported) {
    return { refused: `This stored record carries ${entered.label}, a value only you enter, which the operation engine would not write for it. ${KEPT} Enter the value yourself in the component inspector, or prepare the change again without it.` };
  }
  if (agentOrImported) {
    return { batch: { batch_id, operations: [{ ...structuredClone(intent), author_type: "agent",
      source: { source_ref: typeof source?.source_ref === "string" && source.source_ref ? source.source_ref : `retained:${index}`, source_channel: channel || "retained_requeue", source_role: "agent" } } as unknown as EditorOperationIntent] } };
  }
  if (intent.author_type === "user") {
    if (entered) return { refused: "This stored record writes a value only you enter (a flexibility factor k, a SIF or another component modifier), and its author cannot be verified after reopening. Enter the value again in the component inspector." };
    return { batch: { batch_id, operations: [structuredClone(intent) as unknown as EditorOperationIntent] } };
  }
  return { refused: "This stored record does not say who proposed it, so it cannot be requeued. Prepare the change again." };
}
