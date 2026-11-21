/** Operation authorship on routes that carry agent or imported content
 * (DEL-16-03 offline intake; DEL-16-06 Design `embedded-agent-host.md`,
 * "Author routes"). The applier refuses an agent-authored write of a value the
 * person enters (bend flexibility factor k, SIFs and other component
 * modifiers) by reading `author_type`, which the operation boundary cannot
 * verify. So every route that carries agent or imported content stamps
 * `agent` here, host-side, and never passes a claimed author through. */
import type { EditorOperationIntent, PreviewModel } from "../types";
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
    if (entered.route === "malformed") throw new Error(`${which} has ${entered.label}, so nothing was queued. Write the after value as text in the file.`);
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
export type PersonEnteredWrite = Readonly<{ member: string; route: "refused" | "dropped" | "unchecked" | "malformed"; reason?: "size" | "depth" | "count"; label: string; path: string }>;
/** A refusal carries the operation's identifier, the shape of where a member was found, the
 * member's name and the rule or bound that refused, and nothing else from the operation: no key,
 * field path or value the file or the agent chose. `path` (with the operation's own keys) is
 * returned for tests and for choosing the words, and no caller shows or forwards it. */
const COMPONENT_MODIFIER = "a component modifier (k, a SIF, a stiffness or their source)";
const NUMBER_WORDS = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
/** Where a member was found, by shape only: its depth below the root, and the position in each
 * list it is inside (outermost first); every key the operation chose is left out. */
function located(steps: readonly (string | number)[]): string {
  if (steps.length === 1) return "at the top level of the value";
  const items = steps.filter((step): step is number => typeof step === "number").map(position => String(position + 1));
  const levels = `${NUMBER_WORDS[steps.length] ?? steps.length} levels inside the value`;
  if (items.length === 0) return levels;
  if (items.length === 1) return `${levels}, in item ${items[0]} of a list`;
  return `${levels}, in items ${items.slice(0, -1).join(", ")} and ${items.at(-1)} of nested lists, outermost first`;
}
const MALFORMED = "an after value that is not text, which the operation engine does not read";
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

type Found = { member: keyof typeof LABELS; path: string; steps: (string | number)[] };
/** The shallowest person-entered member anywhere in a parsed after value, or the bound it passes.
 * Each member below the root (the root is level 0) is admitted against the bounds as it is read,
 * so at most one member past the count is read, however wide an array or object is; only
 * containers wait in the queue, each with its parent and key, and a path is built only for the
 * member found. `observe.queued` reports how many containers were queued. A forbidden member is
 * recognised by its name before it is counted, so at the first member past a bound (a count or a
 * depth) the member is named, not the bound. An object's own keys are enumerated by the
 * JavaScript engine (for…in) before the first is yielded: the member count does not bound that
 * work; the after text's length does, and an after given directly as an object has no text, so
 * only the size of what carried it (a file, a stored record) does. */
export function findPersonEntered(root: unknown, observe?: { queued: number }): Found | { unchecked: "depth" | "count" } | null {
  const { maxDepth, maxMembers } = PERSON_ENTERED_CHECK;
  const values: unknown[] = [root], depths: number[] = [0], parents: number[] = [-1], keys: (string | number)[] = [""];
  let members = 0;
  const foundAt = (member: keyof typeof LABELS, parent: number): Found => {
    const steps: (string | number)[] = [member];
    for (let at = parent; at > 0; at = parents[at]) steps.push(keys[at]);
    steps.reverse();
    const path = steps.map(part => typeof part === "number" ? `[${part}]` : step(part)).join("");
    return { member, path: path.startsWith(".") ? path.slice(1) : path, steps };
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
        if (key === "modifiers" || key === "bend_realization") return foundAt(key, index);
        if (key === "mechanics_interface" && plain(member) && member.solver_consumption === "curved_bend_macro_element") return foundAt(key, index);
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
  if (object === "Component" && path.startsWith("modifiers.")) return { member: path, route: "refused", label: COMPONENT_MODIFIER, path: "" };
  const unchecked = (reason: keyof typeof UNCHECKED): PersonEnteredWrite =>
    ({ member: "", route: "unchecked", reason, path: "", label: `${UNCHECKED[reason]}, which is not checked for values only the person enters` });
  if (typeof change.after === "string" && change.after.length > maxAfterChars) return unchecked("size");
  // An after that is not text is read directly (an object or an array is walked like parsed
  // text), so nothing passes unread; the engine reads only text (atomic_batch.rs preflight and
  // lib.rs check_intent_structure refuse any other), so without a member it is refused as such.
  // An operation with no after member is left for the engine, which refuses it as incomplete.
  const text = typeof change.after === "string";
  if (!text && !Object.hasOwn(change, "after")) return null;
  const after = text ? parsed(change.after) : change.after;
  if (kind === "apply_library_record" && plain(after)) {
    for (const key in after) if (Object.hasOwn(after, key) && key.startsWith("modifiers.")) return { member: "modifiers.", route: "refused", label: COMPONENT_MODIFIER, path: "" };
  }
  const found = findPersonEntered(after);
  if (found !== null && "unchecked" in found) return unchecked(found.unchecked);
  if (found === null) return text ? null : { member: "", route: "malformed", path: "", label: MALFORMED };
  const top = found.steps.length === 1;
  return { member: found.member, path: found.path, route: kind === "insert_component_symbol" && top && found.member === "modifiers" ? "refused" : "dropped",
    label: `${LABELS[found.member]} ${located(found.steps)}` };
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
  if (entered?.route === "malformed") return { refused: `This stored record has ${entered.label}. ${KEPT} Prepare the change again.` };
  if (entered && entered.path && (agentOrImported || entered.route === "dropped")) {
    return { refused: `This stored record has a value that names ${entered.label}, which only you enter. ${INDISTINGUISHABLE}. ${KEPT} Enter the value yourself in the component inspector, or prepare the change again without it.` };
  }
  if (entered && agentOrImported) {
    return { refused: `This stored record carries ${entered.label}, a value only you enter, which the operation engine would not write for it. ${KEPT} Enter the value yourself in the component inspector, or prepare the change again without it.` };
  }
  if (agentOrImported) {
    // Who proposed it is what an audit reads: an agent's or an import's stored reference and channel are kept as
    // stored. Only an empty one, the desktop's own channel or the commit binding's reference is replaced by the
    // retained mark, so a requeued proposal never reads as a live desktop editor's.
    const storedRef = typeof source?.source_ref === "string" ? source.source_ref : "";
    const kept = storedRef !== "" && channel !== "" && channel !== "local_desktop_preview" && !storedRef.startsWith("desktop-editor:");
    return { batch: { batch_id, operations: [{ ...structuredClone(intent), author_type: "agent",
      source: kept ? { source_ref: storedRef, source_channel: channel, source_role: "agent" } : { source_ref: `retained:${index}`, source_channel: "retained_requeue", source_role: "agent" } } as unknown as EditorOperationIntent] } };
  }
  if (intent.author_type === "user") {
    if (entered) return { refused: "This stored record writes a value only you enter (a flexibility factor k, a SIF or another component modifier), and its author cannot be verified after reopening. Enter the value again in the component inspector." };
    return { batch: { batch_id, operations: [{ ...structuredClone(intent), source: retainedSource(index, "retained_user") } as unknown as EditorOperationIntent] } };
  }
  return { refused: "This stored record does not say who proposed it, so it cannot be requeued. Prepare the change again." };
}
/** The source a requeued record of the person's carries, whatever source it was stored with: it says the operation
 * is a stored record requeued, so once applied from review it never reads as a live desktop editor's commit. The
 * stored record keeps its original source at the same index of the retained context. */
function retainedSource(index: number, role: "retained_user") {
  return { source_ref: `retained:${index}`, source_channel: "retained_requeue", source_role: role };
}

/** The role a desktop editor's binding stamps on what it commits (DEL-16-03, Commit). */
export type EditorCommitRole = "gui_editor" | "viewport_editor";
/** What a desktop editor's own builder returns from the person's typed values: one operation, or one ordered batch
 * applied whole. Built inside the session on the live model; never an argument of any commit entry. */
export type EditorEdit = Readonly<{ intent: EditorOperationIntent }> | Readonly<{ batch: OperationBatch }>;
/** The input guard every bound builder declares: the members its typed input may have, and a check of their types
 * that returns the reason for a refusal, or null. */
export type EditorInputGuard = Readonly<{ members: readonly string[]; check: (input: Readonly<Record<string, unknown>>) => string | null }>;
/** A desktop editor's own builder, bound once at the application root (`editorBinding`): its name, the role its
 * edits are stamped with, its input guard, and the pure function that turns its typed input into its edit on the
 * live model. `id` is minted by the session for the edit's identifiers. */
export type EditorBuilder<I> = Readonly<{
  name: string;
  role: EditorCommitRole;
  input: EditorInputGuard;
  build: (model: PreviewModel, input: I, id: Readonly<{ sequence: number; batchId: string }>) => EditorEdit | Promise<EditorEdit>;
}>;
/** One of the engine's findings, as a detached frozen copy. */
export type EditorFinding = Readonly<{ code: string; severity: string; message: string; affected_refs: readonly string[] }>;
/** Nothing was applied: the reason to show at the control, and the engine's findings when the engine refused. */
export type EditorRefusal = Readonly<{ status: "refused"; reason: string; findings: readonly EditorFinding[] }>;
/** One commit applied: its checkpoint (one Undo reverts it), the name the history shows, whether a held result was
 * cleared, and a notice of a failure after publication (the commit stands). */
export type EditorCommitResult = Readonly<{ status: "committed"; checkpointId: string; historyName: string; resultsCleared: boolean; notice: string | null }> | EditorRefusal;
/** A dialog's single-use handle: opaque, recognised by the session by identity, consumed by its commit. */
export type EditorPreviewHandle = Readonly<{ handle: "editor-preview" }>;
/** What the engine's validate-only preview reports of the batch's extent, as a detached frozen copy. */
export type EditorExtent = Readonly<{
  historyName: string;
  steps: readonly Readonly<{ operationId: string; changeKind: string; targetType: string; targetRef: string; diff: readonly Readonly<Record<string, unknown>>[] }>[];
  findings: readonly EditorFinding[];
}>;
export type EditorPreviewResult = Readonly<{ status: "previewed"; handle: EditorPreviewHandle; extent: EditorExtent }> | EditorRefusal;

/** Members no editor's typed input may carry: the members of an operation, a batch, a stamp or a queue's context at
 * any depth, and the generic `target`, `source` and `change` at the top level only (a typed domain value may rightly
 * hold a member called target). */
const NOT_TYPED_ANYWHERE = ["operations", "intent", "batch", "batch_id", "operation_id", "author_type", "queue_id", "source_ref", "source_channel",
  "source_role", "source_model_hash", "change_kind", "field_path", "precondition", "agentContext", "importContext"];
const NOT_TYPED_AT_TOP = ["target", "source", "change"];
const NOT_TYPED_INPUT = [...NOT_TYPED_ANYWHERE, ...NOT_TYPED_AT_TOP];
/** The bounds of an editor's typed input: its depth, and its total entries (each value counted once). */
export const EDITOR_INPUT_BOUNDS = Object.freeze({ maxDepth: 32, maxEntries: 65_536 });

/** Refuses, at bind time and by name, a builder with no declared input guard, or a guard that names a member of an
 * operation, a batch or a stamp. Returns the reason, or null. */
export function editorBuilderRefusal(builder: unknown): string | null {
  if (!plain(builder)) return "An editor's builder must be declared with a name, a role, an input guard and a build function.";
  const name = typeof builder.name === "string" && builder.name.trim() ? builder.name : null;
  if (!name) return "An editor's builder must have a name.";
  if (builder.role !== "gui_editor" && builder.role !== "viewport_editor") return `The builder ${name} has no editor role.`;
  if (typeof builder.build !== "function") return `The builder ${name} has no build function.`;
  const input = builder.input;
  if (!plain(input) || !Array.isArray(input.members) || input.members.length === 0 || !input.members.every((member) => typeof member === "string") || typeof input.check !== "function") {
    return `The builder ${name} has no declared input guard (its members and a check of their types).`;
  }
  const forbidden = (input.members as string[]).filter((member) => NOT_TYPED_INPUT.includes(member));
  if (forbidden.length) return `The builder ${name} declares ${forbidden.join(", ")} as input; an editor's input is typed values, never an operation, a batch or a stamp.`;
  return null;
}

/** P4: the input a bound callback owns. The caller's value is materialised once, synchronously, into a fresh
 * deep-frozen copy of data only, and the guard's check and the builder get that copy, never the caller's object.
 * Accepted: null, booleans, finite numbers, strings, arrays of accepted values with no other members, and strictly
 * plain objects (prototype Object.prototype or null) whose own keys are all enumerable string data properties.
 * Refused by name: an accessor, a symbol key, a non-enumerable key, a function, any other prototype (a Map, a Set, a
 * class instance, a Proxy failing these checks), a cycle, an input deeper or larger than `EDITOR_INPUT_BOUNDS`, a
 * forbidden member (`NOT_TYPED_ANYWHERE` at any depth, `NOT_TYPED_AT_TOP` at the top), and a top-level member the
 * guard does not name; then the guard's own check runs on the copy. A throw anywhere is a refusal. */
export function ownEditorInput(guard: EditorInputGuard, input: unknown): { input: Readonly<Record<string, unknown>> } | { refused: string } {
  try {
    const copies = new Map<object, unknown>();
    const inProgress = new Set<object>();
    let entries = 0;
    const own = (value: unknown, depth: number, top: boolean): unknown => {
      if (++entries > EDITOR_INPUT_BOUNDS.maxEntries) throw new InputRefusal(`is larger than ${EDITOR_INPUT_BOUNDS.maxEntries} entries`);
      if (value === null || typeof value === "boolean" || typeof value === "string") return value;
      if (typeof value === "number") { if (!Number.isFinite(value)) throw new InputRefusal("carries a number that is not finite"); return value; }
      if (typeof value === "function") throw new InputRefusal("carries a function, which is not a typed value");
      if (typeof value !== "object") throw new InputRefusal(`carries a ${typeof value}, which is not a typed value`);
      if (depth > EDITOR_INPUT_BOUNDS.maxDepth) throw new InputRefusal(`is nested deeper than ${EDITOR_INPUT_BOUNDS.maxDepth} levels`);
      if (inProgress.has(value)) throw new InputRefusal("contains a cycle");
      if (copies.has(value)) return copies.get(value);
      const prototype = Object.getPrototypeOf(value);
      const isArray = Array.isArray(value);
      if (isArray ? prototype !== Array.prototype : prototype !== Object.prototype && prototype !== null) throw new InputRefusal("carries an object that is not plain data (a Map, a Set, a class instance or another prototype)");
      const keys = Reflect.ownKeys(value);
      inProgress.add(value);
      let copy: unknown;
      // Q1: every member of a copy is defined, never assigned, so no inherited setter can fire for any key; an own
      // "__proto__" is refused as not plain data.
      const define = (target: object, key: string, member: unknown) =>
        Object.defineProperty(target, key, { value: member, enumerable: true, writable: true, configurable: true });
      if (keys.includes("__proto__")) throw new InputRefusal("carries a member named __proto__, which is not plain data");
      if (isArray) {
        const lengthDescriptor = Reflect.getOwnPropertyDescriptor(value, "length");
        const length = lengthDescriptor && "value" in lengthDescriptor && typeof lengthDescriptor.value === "number" ? lengthDescriptor.value : -1;
        if (length < 0) throw new InputRefusal("carries an array with no length of its own");
        const out: unknown[] = [];
        for (const key of keys) {
          if (typeof key === "symbol") throw new InputRefusal("carries a symbol key");
          if (key === "length") continue;
          if (!/^(0|[1-9]\d*)$/.test(key) || Number(key) >= length) throw new InputRefusal(`carries an array with the member ${key}`);
        }
        if (keys.length !== length + 1) throw new InputRefusal("carries an array with holes");
        for (let index = 0; index < length; index++) {
          const descriptor = Reflect.getOwnPropertyDescriptor(value, String(index))!;
          if (!("value" in descriptor)) throw new InputRefusal("carries an accessor");
          if (!descriptor.enumerable) throw new InputRefusal("carries a non-enumerable member");
          define(out, String(index), own(descriptor.value, depth + 1, false));
        }
        copy = Object.freeze(out);
      } else {
        const out: Record<string, unknown> = {};
        for (const key of keys) {
          if (typeof key === "symbol") throw new InputRefusal("carries a symbol key");
          if (NOT_TYPED_ANYWHERE.includes(key) || (top && NOT_TYPED_AT_TOP.includes(key))) throw new InputRefusal(`carries ${key}, a member of an operation, a batch or a stamp`);
          const descriptor = Reflect.getOwnPropertyDescriptor(value, key)!;
          if (!("value" in descriptor)) throw new InputRefusal(`carries an accessor at ${key}`);
          if (!descriptor.enumerable) throw new InputRefusal(`carries a non-enumerable member ${key}`);
          define(out, key, own(descriptor.value, depth + 1, false));
        }
        copy = Object.freeze(out);
      }
      inProgress.delete(value);
      copies.set(value, copy);
      return copy;
    };
    if (input === null || typeof input !== "object" || Array.isArray(input)) return { refused: "This edit's input is not the editor's typed values, so nothing was built." };
    const owned = own(input, 0, true) as Readonly<Record<string, unknown>>;
    // Q1: the declaration is checked against the caller's object's own keys, not the copy's.
    const unnamed = Reflect.ownKeys(input).filter((key): key is string => typeof key === "string" && !guard.members.includes(key));
    if (unnamed.length) return { refused: `This edit's input carries ${unnamed.join(", ")}, which the editor does not take, so nothing was built.` };
    const checked = guard.check(owned);
    if (checked !== null) return { refused: typeof checked === "string" ? checked : "The editor's check refused this input, so nothing was built." };
    return { input: owned };
  } catch (error) {
    if (error instanceof InputRefusal) return { refused: `This edit's input ${error.message}, so nothing was built.` };
    return { refused: `This edit's input could not be read as typed values (${error instanceof Error ? error.message : String(error)}), so nothing was built.` };
  }
}
class InputRefusal extends Error {}

/** The prefix of the source reference the binding stamps; a built operation already carrying it is refused. */
const EDITOR_COMMIT_SOURCE_PREFIX = "desktop-editor:";
const EDITOR_SOURCE_MEMBERS = ["source_ref", "source_channel", "source_role"];
const NOT_AN_EDITORS = "so nothing was applied.";

/** The binding's stamp (DEL-16-03, Commit), applied to what an editor's own builder returned. Every operation is
 * stamped as the person's by the binding: `author_type` is `user` and the source is the binding's own
 * (`local_desktop_preview`, its role, a reference derived from the batch), never read from what was built as a
 * grant. What was built is read only against an allowlist: each operation names the person as its author and carries
 * either no source or the desktop editor's own (`local_desktop_preview` with `gui_editor` or `viewport_editor`); a
 * queue id, a retained or already-stamped reference, a requeue batch, a context or a precondition is refused, and
 * nothing is overwritten into acceptance. */
export function stampEditorCommit(edit: unknown, role: EditorCommitRole): { batch: OperationBatch } | { refused: string } {
  if (!plain(edit)) return { refused: `The editor built no change or batch, ${NOT_AN_EDITORS}` };
  const keys = Object.keys(edit);
  if (keys.some((key) => /context|precondition/i.test(key))) return { refused: `The change carries an agent's or an import's context or a precondition, ${NOT_AN_EDITORS}` };
  if (keys.length !== 1 || !(keys[0] === "intent" || keys[0] === "batch")) return { refused: `The editor built something other than one change or one batch, ${NOT_AN_EDITORS}` };
  let batchId: unknown, operations: unknown, sourceModelHash: { source_model_hash: unknown } | null = null;
  if (keys[0] === "intent") {
    operations = [edit.intent];
    batchId = plain(edit.intent) ? edit.intent.operation_id : undefined;
  } else {
    const batch = edit.batch;
    if (!plain(batch)) return { refused: `The editor built something other than one change or one batch, ${NOT_AN_EDITORS}` };
    if (Object.keys(batch).some((key) => /context|precondition/i.test(key))) return { refused: `The change carries an agent's or an import's context or a precondition, ${NOT_AN_EDITORS}` };
    const extra = Object.keys(batch).filter((key) => !["batch_id", "operations", "source_model_hash"].includes(key));
    if (extra.length) return { refused: `The batch carries ${extra.join(", ")}, which no desktop editor writes, ${NOT_AN_EDITORS}` };
    batchId = batch.batch_id;
    operations = batch.operations;
    if (Object.hasOwn(batch, "source_model_hash")) sourceModelHash = { source_model_hash: batch.source_model_hash };
  }
  if (typeof batchId !== "string" || !batchId.trim()) return { refused: `The change has no identifier, ${NOT_AN_EDITORS}` };
  if (batchId.startsWith("requeue-")) return { refused: `The change is a stored record requeued after reopening, ${NOT_AN_EDITORS}` };
  if (!Array.isArray(operations) || operations.length === 0 || !operations.every(plain)) return { refused: `The batch holds no operation a desktop editor builds, ${NOT_AN_EDITORS}` };
  for (const operation of operations as Record<string, unknown>[]) {
    if (Object.hasOwn(operation, "queue_id")) return { refused: `The change carries a queue id: it was queued for review, ${NOT_AN_EDITORS}` };
    if (!Object.hasOwn(operation, "author_type")) return { refused: `The change names no author; a desktop editor's change names the person, ${NOT_AN_EDITORS}` };
    if (operation.author_type !== "user") return { refused: `The change carries another author than the person, ${NOT_AN_EDITORS}` };
    if (!Object.hasOwn(operation, "source")) continue;
    const source = operation.source;
    const ref = plain(source) && typeof source.source_ref === "string" ? source.source_ref : "";
    if (ref.startsWith("retained:") || (plain(source) && source.source_channel === "retained_requeue")) return { refused: `The change is a stored record requeued after reopening, ${NOT_AN_EDITORS}` };
    if (ref.startsWith(EDITOR_COMMIT_SOURCE_PREFIX)) return { refused: `The change is a record already committed once, ${NOT_AN_EDITORS}` };
    if (!plain(source) || Object.keys(source).some((key) => !EDITOR_SOURCE_MEMBERS.includes(key)) ||
      source.source_channel !== "local_desktop_preview" || (source.source_role !== "gui_editor" && source.source_role !== "viewport_editor") ||
      (Object.hasOwn(source, "source_ref") && typeof source.source_ref !== "string")) {
      return { refused: `The change carries a source that is not a desktop editor's own, ${NOT_AN_EDITORS}` };
    }
  }
  const stamped = (operations as Record<string, unknown>[]).map((operation, index) => ({
    ...structuredClone(operation),
    author_type: "user" as const,
    source: { source_ref: `${EDITOR_COMMIT_SOURCE_PREFIX}${role}:${batchId}:${index + 1}`, source_channel: "local_desktop_preview", source_role: role }
  })) as unknown as EditorOperationIntent[];
  // The engine reads and checks a batch's own model hash claim; it is passed as the editor wrote it.
  return { batch: { batch_id: batchId, operations: stamped, ...(sourceModelHash as { source_model_hash?: string } | null) } };
}
