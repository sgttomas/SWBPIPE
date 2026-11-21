// The Issues list a person reads, and its one count.
//
// Everything the session knows to be wrong or worth a look arrives by several routes: the model
// checks (the solve's own checks, the model-data slot checks and the library references), the
// model document's own diagnostics, the document validator, the library-reference findings, the
// design knowledge, the result's diagnostics and the outcomes of applied operations. Some facts
// arrive by two routes (a check finding the solve publishes again in its result; a slot finding
// the checks and the document validator both state; a library reference both list). This module
// makes one list of them: one row per fact, a member group as one row, ordered by what must be
// fixed before Run, then warnings, then notes, and counts that same list. Nothing here changes a
// finding's text in meaning or decides what blocks a solve: "fix before Run" is the Solve
// command's own gate (`readSolveGate`: a blocking or error finding of the solve's checks on the
// current model value, or the solve's refusal to read it).
import type { Diagnostic, EntityRef, OperationOutcomeDiagnostic, PreviewModel } from "../../types";
import type { MenuCommandId } from "../workspace/menuCommands";
import { MODEL_CHECK_GROUP_LABELS, resolveModelCheckEntity, SOLVE_CHECKS_SOURCE, type ModelCheckGroupId, type ModelCheckRun } from "./modelChecks";

/** The route a listed entry arrived by. */
export type IssueRoute = "model-check" | "model" | "document" | "library" | "knowledge" | "result" | "operation";

/** One entry a row stands for: its route and the record as that route gives it. */
export type IssueSource = Readonly<{
  route: IssueRoute;
  /** The model-check group, for a model-check entry. */
  group?: ModelCheckGroupId;
  /** The record as its route lists it (a result diagnostic as the Diagnostics list presents it). */
  diagnostic: Diagnostic;
}>;

/** Where a row stands: what must be fixed before Run, a warning, or a note. */
export type IssueTier = "fix" | "warning" | "note";

/** A command the desktop already has that repairs a finding in one act. */
export type IssueAct = Readonly<{ command: MenuCommandId; label: string }>;

export type IssueRow = Readonly<{
  key: string;
  tier: IssueTier;
  /** The finding's own sentence; for a member group, its members' sentences joined at the member. */
  message: string;
  /** The first model entity the row names, by which it is ordered; null where none. */
  entity: EntityRef | null;
  /** Every model entity the row names, each of which a person can select (DEL-07-04 REQ-014). */
  entities: readonly EntityRef[];
  /** The finding's code; empty for the solve's refusal to read the model, which has none. */
  code: string;
  /** The finding's own severity (the most severe its entries state). */
  severity: Diagnostic["severity"];
  /** The members a group names (`hanger.installed_load`, …), in order; empty for a single fact. */
  members: readonly string[];
  /** Every entry the row stands for, in the order they arrived. */
  sources: readonly IssueSource[];
  /** The one act that repairs it; null where no single act does. */
  act: IssueAct | null;
  /** The row includes one of the solver's own messages (a result diagnostic). */
  solverMessage: boolean;
}>;

export type IssueList = Readonly<{
  rows: readonly IssueRow[];
  /** The solve's checks ran on this model value, so "to fix before Run" is known. */
  checked: boolean;
  /** Blocking or error findings of the solve's checks the bounded report counted but did not list. */
  notListedToFix: number;
}>;

export type IssueCount = Readonly<{
  toFix: number;
  warnings: number;
  notes: number;
  total: number;
  checked: boolean;
  notListedToFix: number;
}>;

/** What the session holds, by route. Each list is as its route gives it; a missing one is empty. */
export type IssueInputs = Readonly<{
  model: PreviewModel;
  /** The session's last model-check run; listed only while it is of this model value. */
  modelCheckRun?: ModelCheckRun | null;
  /** `modelDocumentDiagnostics(model)`. */
  documentDiagnostics?: readonly Diagnostic[];
  /** `libraryRefDiagnostics(currentLibraryRefFindings(model, …))`. */
  libraryRefDiagnostics?: readonly Diagnostic[];
  /** `knowledge.diagnostics`. */
  knowledgeDiagnostics?: readonly Diagnostic[];
  /** The result's own diagnostics (`result.diagnostics`). */
  resultDiagnostics?: readonly Diagnostic[];
  /** The same diagnostics as the Diagnostics list presents them (`presentedResultDiagnostics`),
   * in the same order; where absent, the result's own. */
  presentedResultDiagnostics?: readonly Diagnostic[];
  /** The diagnostics of applied operations' outcomes. */
  operationDiagnostics?: readonly OperationOutcomeDiagnostic[];
}>;

/** The findings one existing command repairs: the solve's empty-collection checks
 * (core/product_physics/src/validation.rs `validate_required_collections`) and its no-restraint
 * pre-check (validation/model_checks.rs), each answered by the Insert menu's own command. */
export const ISSUE_ACTS: Readonly<Record<string, IssueAct>> = {
  NODE_INPUT_MISSING: { command: "insert.node", label: "Add a node" },
  PIPE_INPUT_MISSING: { command: "insert.pipe", label: "Add a pipe" },
  LOAD_INPUT_MISSING: { command: "insert.load", label: "Add a load" },
  SUPPORT_INPUT_MISSING: { command: "insert.support", label: "Add a support" }
};

/** The words for each route, as the opened row names where an entry came from. */
export const ISSUE_ROUTE_LABELS: Readonly<Record<IssueRoute, string>> = Object.freeze({
  "model-check": "Model checks",
  model: "The model document",
  document: "Model-data slot checks",
  library: "Library references",
  knowledge: "Design knowledge",
  result: "The solver's result",
  operation: "An applied operation"
});

/** A source's route as words, with the check group for a model-check entry. */
export function issueSourceLabel(source: IssueSource): string {
  return source.route === "model-check" && source.group ? `${ISSUE_ROUTE_LABELS["model-check"]}: ${MODEL_CHECK_GROUP_LABELS[source.group]}` : ISSUE_ROUTE_LABELS[source.route];
}

const SEVERITY_RANK: Readonly<Record<Diagnostic["severity"], number>> = { blocking: 3, error: 2, warning: 1, info: 0 };
const TIER_RANK: Readonly<Record<IssueTier, number>> = { fix: 0, warning: 1, note: 2 };
/** The model's order: as the model tree lists it. */
const ENTITY_TYPE_ORDER: readonly EntityRef["type"][] = ["material", "section", "node", "pipe", "support", "component", "load"];

function severityOf(value: string): Diagnostic["severity"] {
  return value === "blocking" || value === "error" || value === "warning" ? value : "info";
}

/** One entry before it is a row: its source, the text compared for "the same fact" (a result
 * diagnostic's own message, not its presented one) and the text shown. */
type Entry = Readonly<{ source: IssueSource; raw: Diagnostic; entity: EntityRef | null; withholds: boolean }>;

function refsKey(diagnostic: Diagnostic): string {
  return JSON.stringify([...(diagnostic.affected_refs ?? [])].sort());
}

/** Two entries are the same fact when they have the same code and the same affected references,
 * and the same id or the same message. */
export function sameFact(a: Diagnostic, b: Diagnostic): boolean {
  if (a.code !== b.code || refsKey(a) !== refsKey(b)) return false;
  return (a.id !== undefined && a.id === b.id) || a.message === b.message;
}

function entityOf(model: PreviewModel, diagnostic: Diagnostic): EntityRef | null {
  for (const ref of diagnostic.affected_refs ?? []) {
    const entity = resolveModelCheckEntity(model, ref);
    if (entity) return entity;
  }
  return null;
}

function entries(inputs: IssueInputs): { entries: Entry[]; checked: boolean; notListedToFix: number } {
  const { model } = inputs;
  const out: Entry[] = [];
  const add = (route: IssueRoute, diagnostic: Diagnostic, extra: { group?: ModelCheckGroupId; raw?: Diagnostic; entity?: EntityRef | null; withholds?: boolean } = {}) => {
    out.push({
      source: extra.group ? { route, group: extra.group, diagnostic } : { route, diagnostic },
      raw: extra.raw ?? diagnostic,
      entity: extra.entity !== undefined ? extra.entity : entityOf(model, diagnostic),
      withholds: extra.withholds ?? false
    });
  };
  const run = inputs.modelCheckRun ?? null;
  // A run of another model value is not true of this one; a run that did not complete knows nothing.
  const current = run !== null && run.model === model && run.solve.kind !== "unavailable";
  if (current) {
    if (run.solve.kind === "refused") {
      add("model-check", { code: "", severity: "blocking", message: run.solve.refusal, source: SOLVE_CHECKS_SOURCE }, { group: "solve", entity: null, withholds: true });
    }
    for (const finding of run.findings) {
      const blocking = finding.diagnostic.severity === "blocking" || finding.diagnostic.severity === "error";
      add("model-check", finding.diagnostic, { group: finding.group, entity: finding.entity, withholds: finding.group === "solve" && blocking });
    }
  }
  for (const diagnostic of model.diagnostics ?? []) add("model", diagnostic);
  for (const diagnostic of inputs.documentDiagnostics ?? []) add("document", diagnostic);
  for (const diagnostic of inputs.libraryRefDiagnostics ?? []) add("library", diagnostic);
  for (const diagnostic of inputs.knowledgeDiagnostics ?? []) add("knowledge", diagnostic);
  const result = inputs.resultDiagnostics ?? [];
  const presented = inputs.presentedResultDiagnostics;
  result.forEach((diagnostic, index) => add("result", presented?.[index] ?? diagnostic, { raw: diagnostic }));
  for (const item of inputs.operationDiagnostics ?? []) {
    add("operation", { id: item.id, code: item.code, severity: severityOf(item.severity), message: item.message, remediation: item.remediation, affected_refs: item.affected_refs, source: item.source });
  }
  return { entries: out, checked: current, notListedToFix: current ? run.notShownBlockingByGroup.solve : 0 };
}

type Draft = { entries: Entry[]; members: string[]; message: string };

/** The row's sentence: the first entry that is not the solver's presented copy, else that copy. */
function representative(draft: Draft): Entry {
  return draft.entries.find((entry) => entry.source.route !== "result") ?? draft.entries[0]!;
}

function entitiesOf(model: PreviewModel, draft: Draft): EntityRef[] {
  const found = new Map<string, EntityRef>();
  for (const entry of draft.entries) {
    const named = [entry.entity, ...(entry.raw.affected_refs ?? []).map((ref) => resolveModelCheckEntity(model, ref))];
    for (const entity of named) if (entity && !found.has(`${entity.type}:${entity.id}`)) found.set(`${entity.type}:${entity.id}`, entity);
  }
  return [...found.values()];
}

function toRow(model: PreviewModel, draft: Draft, index: number): IssueRow {
  const first = representative(draft);
  const severity = draft.entries.reduce<Diagnostic["severity"]>((worst, entry) => SEVERITY_RANK[entry.raw.severity] > SEVERITY_RANK[worst] ? entry.raw.severity : worst, "info");
  const tier: IssueTier = draft.entries.some((entry) => entry.withholds) ? "fix" : severity === "info" ? "note" : "warning";
  const code = first.raw.code;
  return {
    key: `${index}:${first.source.route}:${first.raw.id ?? code}`,
    tier,
    message: draft.message,
    entity: draft.entries.find((entry) => entry.entity !== null)?.entity ?? null,
    entities: entitiesOf(model, draft),
    code,
    severity,
    members: draft.members,
    sources: draft.entries.map((entry) => entry.source),
    act: Object.hasOwn(ISSUE_ACTS, code) ? ISSUE_ACTS[code]! : null,
    solverMessage: draft.entries.some((entry) => entry.source.route === "result")
  };
}

/** The member a single fact names beside its entity: its only affected reference that is not a
 * model entity, where its other references are entities. Null where there is not exactly one. */
function memberOf(model: PreviewModel, entry: Entry): string | null {
  const others = (entry.raw.affected_refs ?? []).filter((ref) => resolveModelCheckEntity(model, ref) === null);
  return others.length === 1 ? others[0]! : null;
}

/** The sentences of a member group joined at the member: the words they share before and after,
 * with each one's own words between them. Null unless each one's own words name its member. */
function joinedAtMembers(messages: readonly string[], members: readonly string[]): string | null {
  const words = messages.map((message) => message.split(" "));
  const shortest = Math.min(...words.map((item) => item.length));
  let prefix = 0;
  while (prefix < shortest && words.every((item) => item[prefix] === words[0]![prefix])) prefix += 1;
  let suffix = 0;
  while (suffix < shortest - prefix && words.every((item) => item[item.length - 1 - suffix] === words[0]![words[0]!.length - 1 - suffix])) suffix += 1;
  const middles = words.map((item) => item.slice(prefix, item.length - suffix).join(" "));
  if (middles.some((middle, index) => middle === "" || !middle.includes(members[index]!))) return null;
  const joined = middles.length === 2 ? `${middles[0]} and ${middles[1]}` : `${middles.slice(0, -1).join(", ")} and ${middles[middles.length - 1]}`;
  return [...words[0]!.slice(0, prefix), joined, ...words[0]!.slice(words[0]!.length - suffix)].join(" ");
}

/** Facts about one entity that differ only in a member become one row naming the members: the same
 * code, severity, routes and entity references, each with a member of its own that its text names.
 * Where their sentences cannot be joined at the members, each stays a row of its own. */
function groupMembers(model: PreviewModel, drafts: readonly Draft[]): Draft[] {
  const signature = (draft: Draft): string | null => {
    const first = representative(draft);
    const entity = draft.entries.find((entry) => entry.entity !== null)?.entity;
    if (!entity || memberOf(model, first) === null) return null;
    const entityRefs = (first.raw.affected_refs ?? []).filter((ref) => resolveModelCheckEntity(model, ref) !== null).sort();
    const routes = draft.entries.map((entry) => `${entry.source.route}/${entry.source.group ?? ""}/${entry.raw.severity}/${entry.withholds}`).join(",");
    return JSON.stringify([first.raw.code, entity.type, entity.id, entityRefs, routes]);
  };
  const groups: Draft[][] = [];
  const byKey = new Map<string, Draft[]>();
  for (const draft of drafts) {
    const key = signature(draft);
    const existing = key === null ? undefined : byKey.get(key);
    if (existing) { existing.push(draft); continue; }
    const group = [draft];
    if (key !== null) byKey.set(key, group);
    groups.push(group);
  }
  return groups.flatMap((group) => {
    if (group.length < 2) return group;
    const members = group.map((draft) => memberOf(model, representative(draft))!);
    if (new Set(members).size !== members.length) return group;
    const message = joinedAtMembers(group.map((draft) => draft.message), members);
    return message === null ? group : [{ entries: group.flatMap((draft) => draft.entries), members, message }];
  });
}

function entityRank(model: PreviewModel, entity: EntityRef | null): [number, number] {
  if (!entity) return [-1, -1];
  const type = ENTITY_TYPE_ORDER.indexOf(entity.type);
  const list: readonly { id: string }[] =
    entity.type === "material" ? model.materials ?? []
      : entity.type === "section" ? model.sections ?? []
      : entity.type === "node" ? model.nodes
      : entity.type === "pipe" ? model.pipe_segments
      : entity.type === "support" ? model.supports
      : entity.type === "component" ? model.components
      : entity.type === "load" ? model.load_cases
      : [];
  return [type === -1 ? ENTITY_TYPE_ORDER.length : type, list.findIndex((item) => item.id === entity.id)];
}

/** The list a person reads: one row per fact, a member group as one row, ordered by what must be
 * fixed before Run, then warnings, then notes, and within each by the entity in the model's order
 * (rows that name no entity first, in the order they arrived). */
export function buildIssueList(inputs: IssueInputs): IssueList {
  const { entries: all, checked, notListedToFix } = entries(inputs);
  const drafts: Draft[] = [];
  for (const entry of all) {
    const same = drafts.find((draft) => draft.entries.some((other) => sameFact(other.raw, entry.raw)));
    if (same) same.entries.push(entry);
    else drafts.push({ entries: [entry], members: [], message: "" });
  }
  // A row's sentence is its first entry's that is not the solver's presented copy, else that copy.
  for (const draft of drafts) draft.message = representative(draft).source.diagnostic.message;
  const rows = groupMembers(inputs.model, drafts).map((draft, index) => toRow(inputs.model, draft, index));
  const ranked = rows.map((row, index) => ({ row, index, entity: entityRank(inputs.model, row.entity) }));
  ranked.sort((a, b) =>
    TIER_RANK[a.row.tier] - TIER_RANK[b.row.tier]
    || a.entity[0] - b.entity[0]
    || a.entity[1] - b.entity[1]
    || a.index - b.index);
  return { rows: ranked.map((item) => item.row), checked, notListedToFix };
}

/** The one count of the Issues list: everything that shows a count of issues reads it. */
export function countIssues(list: IssueList): IssueCount {
  const tier = (value: IssueTier) => list.rows.filter((row) => row.tier === value).length;
  return { toFix: tier("fix"), warnings: tier("warning"), notes: tier("note"), total: list.rows.length, checked: list.checked, notListedToFix: list.notListedToFix };
}

const plural = (count: number, word: string) => `${count} ${word}${count === 1 ? "" : "s"}`;

/** The count as one line: "3 to fix before Run", or "Nothing to fix. 2 warnings, 5 notes."; where
 * the solve's checks have not run on this model value, "Model not checked." in place of "Nothing
 * to fix.". */
export function issueCountLine(count: IssueCount): string {
  if (count.toFix > 0 || count.notListedToFix > 0) {
    const listed = `${count.toFix} to fix before Run`;
    return count.notListedToFix > 0 ? `${listed}, and ${count.notListedToFix} more not listed` : listed;
  }
  const rest = [count.warnings > 0 ? plural(count.warnings, "warning") : null, count.notes > 0 ? plural(count.notes, "note") : null].filter(Boolean).join(", ");
  const lead = count.checked ? "Nothing to fix." : "Model not checked.";
  return rest ? `${lead} ${rest}.` : lead;
}
