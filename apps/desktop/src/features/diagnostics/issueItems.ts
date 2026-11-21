// The Issues count and the Issues drawer's items, from one function.
//
// The rule: an item is one thing a person must look at, which is one finding
// code on one object, of one class (DEL-07-04 REQ-001 keeps the six classes
// apart, so two classes are never one item). The object is the model entity a
// report's references name, resolved as the model checks resolve them (materials
// and sections included); a reference that names no entity is its own text; a
// report that names no object is its own item. Every report of that code on that
// object is a member of the one item, whoever reported it and however many loads,
// cases or sources repeat it. Whether a report that states no class joins a
// classed item is decided after every report is read: it joins only where its code
// on its object has exactly one stated class; where two or more are stated it is
// its own item. A member keeps its own code, severity and message; nothing is
// reworded or dropped, and the item's severity is its most severe member's. The
// number on the Issues controls is `issueSummary(...).total`, which is exactly the
// number of items the drawer lists. Counts name what they count: "items" are what
// the drawer lists, "reports" the diagnostics and findings before grouping.
import { useMemo } from "react";
import type { SessionResult } from "../../services/nativeResult";
import type { DesignKnowledge, Diagnostic, EntityRef, OperationOutcome, PreviewModel, SolveJobAuditState } from "../../types";
import { isResultTooLargeSolveJob } from "../workspace/solveJobAudit";
import { currentLibraryRefFindings, libraryRefDiagnostics, usePublishedLibraryRefFindings } from "../library/libraryRefFindings";
import { countMissingDataBlockers } from "../missing-data/MissingDataBlockingPanel";
import { modelDocumentDiagnostics } from "../model-data/documentDiagnostics";
import { presentedResultDiagnostics } from "../results/suppliedDiagnostics";
import { entityLabel } from "../model-workspace/modelView";
import { resolveModelCheckEntity, type ModelCheckFinding, type ModelCheckGroupId, type ModelCheckRun } from "./modelChecks";

/** Where a member was reported. */
export type IssueSource = "model-checks" | "operation" | "diagnostics";

/** Who reported a member, as the drawer names it. */
export type IssueOrigin = "model checks" | "operation" | "model" | "model document" | "library reference" | "design knowledge" | "solve";

export const ISSUE_ORIGIN_LABELS: Record<IssueOrigin, string> = {
  "model checks": "the model checks",
  operation: "an operation",
  model: "the model",
  "model document": "the model document check",
  "library reference": "a library reference",
  "design knowledge": "design knowledge",
  solve: "the solve"
};

/** One report that belongs to an item. */
export type IssueMember = Readonly<{
  /** The diagnostic as its producer gave it. */
  diagnostic: Diagnostic;
  /** The text the drawer shows for it (a solve's supplied text keeps its notice). */
  shown: Diagnostic;
  source: IssueSource;
  origin: IssueOrigin;
  /** The model-check finding it came from, for a model-check member. */
  finding?: ModelCheckFinding;
  /** The report's own references beyond its item's object, each in the application's display
   * form where the model names it ("Label (ref)"), else as given. */
  otherRefs: readonly string[];
  /** Whether choosing this report can open its own detail: false for a diagnostic with no
   * identifier whose code another diagnostic shares, since the detail is found by identifier
   * or code and would show the other's. */
  ownDetail: boolean;
}>;

export type IssueItem = Readonly<{
  key: string;
  code: string;
  severity: Diagnostic["severity"];
  /** The object the item is about: its first affected reference that names a model
   * entity, else its first affected reference; null where the finding names none. */
  object: string | null;
  entity: EntityRef | null;
  /** The object in the application's display form for a reference: the entity's name and its
   * reference ("Inboard span (pipe:md-inboard)"), or the reference alone where it names no entity
   * or the entity has no name; null where the finding names none. Every row leads its muted line
   * with it. */
  objectLabel: string | null;
  /** The list that shows the item: the model checks where they reported it, else an
   * operation's, else the diagnostics list. */
  home: IssueSource;
  /** For an item the model checks list, the group of its first model-check member. */
  group: ModelCheckGroupId | null;
  members: readonly IssueMember[];
  /** The member the collapsed item leads with: its most severe, first among equals one that
   * states the class, then in arrival order. */
  lead: IssueMember;
  /** What tells the members' messages apart (for example "installed load
   * (hanger.installed_load)"), one per distinct message, in order, cut from the shown text.
   * Empty where the members share one message, where any member takes the solver report note,
   * or where a distinct message is shown only as supplied, unverified text: an excerpt is then
   * never shown. */
  differences: readonly string[];
}>;

const SEVERITY_RANK: Record<Diagnostic["severity"], number> = { blocking: 0, error: 1, warning: 2, info: 3 };
const rank = (severity: string): number => SEVERITY_RANK[severity as Diagnostic["severity"]] ?? SEVERITY_RANK.info;
const HOME_RANK: Record<IssueSource, number> = { "model-checks": 0, operation: 1, diagnostics: 2 };

export type IssueInputs = Readonly<{
  model?: PreviewModel | null;
  modelChecks?: readonly ModelCheckFinding[];
  operations?: readonly Diagnostic[];
  /** The diagnostics list: each as given and as shown (index for index). */
  diagnostics?: readonly Diagnostic[];
  shownDiagnostics?: readonly Diagnostic[];
  /** Who reported each of `diagnostics` (index for index); "model" where not given. */
  diagnosticOrigins?: readonly IssueOrigin[];
}>;

/** The object a report is about: the model entity its first reference that names one resolves to
 * (the model checks' resolver), else its first non-blank reference as text; null where it names
 * none. `key` is what two reports must share to be one item. */
function objectOf(diagnostic: Diagnostic, model: PreviewModel | null | undefined, entity: EntityRef | null): { object: string | null; entity: EntityRef | null; key: string | null } {
  if (entity) return { object: entity.id, entity, key: `${entity.type}:${entity.id}` };
  const refs = (diagnostic.affected_refs ?? []).filter((ref) => typeof ref === "string" && ref.trim() !== "");
  if (model) {
    for (const ref of refs) {
      const resolved = resolveRef(model, ref);
      if (resolved) return { object: ref, entity: resolved, key: `${resolved.type}:${resolved.id}` };
    }
  }
  return refs[0] === undefined ? { object: null, entity: null, key: null } : { object: refs[0], entity: null, key: `text:${refs[0]}` };
}

function resolveRef(model: PreviewModel, ref: string): EntityRef | null {
  return resolveModelCheckEntity({
    ...model,
    nodes: model.nodes ?? [],
    pipe_segments: model.pipe_segments ?? [],
    supports: model.supports ?? [],
    components: model.components ?? [],
    load_cases: model.load_cases ?? []
  }, ref);
}

/** A reference in the application's display form: the entity's name with the reference, or the
 * reference alone. */
function displayRef(ref: string | null, entity: EntityRef | null, model: PreviewModel | null | undefined): string | null {
  if (ref === null) return null;
  const label = entity && model ? entityLabel(model, entity) : null;
  return label && label !== ref ? `${label} (${ref})` : ref;
}

/** A report's references other than its item's object, in display form. */
function otherRefsOf(diagnostic: Diagnostic, model: PreviewModel | null | undefined, objectKey: string | null): string[] {
  const refs = (diagnostic.affected_refs ?? []).filter((ref) => typeof ref === "string" && ref.trim() !== "");
  const shown: string[] = [];
  for (const ref of refs) {
    const resolved = model ? resolveRef(model, ref) : null;
    const key = resolved ? `${resolved.type}:${resolved.id}` : `text:${ref}`;
    if (key === objectKey) continue;
    const text = displayRef(ref, resolved, model) ?? ref;
    if (!shown.includes(text)) shown.push(text);
  }
  return shown;
}

/** A report's class as its producer supplied it (`diagnostic_class`, else `class`); null where it
 * supplied none. Never inferred (DEL-07-04 REQ-004). */
export function classOf(diagnostic: Diagnostic): string | null {
  for (const value of [diagnostic.diagnostic_class, diagnostic.class]) {
    if (typeof value === "string" && value.trim() !== "") return value;
  }
  return null;
}

/** How a supplied class is shown in text; null where none was supplied, which shows nothing at rest. */
export function classText(diagnostic: Diagnostic): string | null {
  const supplied = classOf(diagnostic);
  return supplied === null ? null : `class ${supplied}`;
}

/** An item's class: the one class its members state (the grouping never joins two), shown once
 * on the item; null where no member states one (an item formed because its code's class on its
 * object was ambiguous states none). Nothing is said of a member's missing class at rest. */
export function itemClassText(item: IssueItem): string | null {
  const stated = item.members.map((member) => classOf(member.diagnostic)).find((value) => value !== null) ?? null;
  return stated === null ? null : `class ${stated}`;
}

/** The fields a report's producer did not supply, for the one short line an opened row may show. */
export function notSuppliedFields(diagnostic: Diagnostic): string[] {
  const absent: string[] = [];
  if (classOf(diagnostic) === null) absent.push("class");
  if (!diagnostic.source) absent.push("source");
  if (!diagnostic.remediation) absent.push("remediation");
  if (diagnostic.provenance === undefined || diagnostic.provenance === null || diagnostic.provenance === "") absent.push("provenance");
  return absent;
}

/** What tells this member's message apart from the item's other messages; null where the item
 * shows no excerpt. */
export function memberDifference(item: IssueItem, member: IssueMember): string | null {
  if (item.differences.length === 0) return null;
  const distinct = [...new Set(item.members.map((each) => each.diagnostic.message))];
  return item.differences[distinct.indexOf(member.diagnostic.message)] ?? null;
}

type ReadMember = Omit<IssueMember, "otherRefs" | "ownDetail">;

/** Groups every report into items (see the rule at the top of this file). */
export function groupIssues(inputs: IssueInputs): IssueItem[] {
  type Pending = { member: ReadMember; where: ReturnType<typeof objectOf>; cls: string | null; order: number };
  const pending: Pending[] = [];
  const read = (member: ReadMember, entity: EntityRef | null) => {
    pending.push({ member, where: objectOf(member.diagnostic, inputs.model, entity), cls: classOf(member.diagnostic), order: pending.length });
  };
  for (const finding of inputs.modelChecks ?? []) read({ diagnostic: finding.diagnostic, shown: finding.diagnostic, source: "model-checks", origin: "model checks", finding }, finding.entity);
  for (const diagnostic of inputs.operations ?? []) read({ diagnostic, shown: diagnostic, source: "operation", origin: "operation" }, null);
  const shown = inputs.shownDiagnostics ?? inputs.diagnostics ?? [];
  const all = inputs.diagnostics ?? [];
  all.forEach((diagnostic, index) => read({ diagnostic, shown: shown[index] ?? diagnostic, source: "diagnostics", origin: inputs.diagnosticOrigins?.[index] ?? "model" }, null));

  // A diagnostic with no identifier opens its own detail only where its code is its own.
  const codeCounts = new Map<string, number>();
  for (const diagnostic of all) codeCounts.set(diagnostic.code, (codeCounts.get(diagnostic.code) ?? 0) + 1);
  const ownDetail = (member: ReadMember) => member.source !== "diagnostics" || member.diagnostic.id !== undefined || (codeCounts.get(member.diagnostic.code) ?? 0) <= 1;

  // The classes stated for each code on each object, after every report is read.
  const stated = new Map<string, Set<string>>();
  for (const entry of pending) {
    if (entry.where.key === null || entry.cls === null) continue;
    const base = `${entry.member.diagnostic.code}\u0000${entry.where.key}`;
    const set = stated.get(base) ?? new Set<string>();
    set.add(entry.cls);
    stated.set(base, set);
  }
  type Group = { code: string; object: string | null; entity: EntityRef | null; objectKey: string | null; entries: Pending[]; order: number };
  const groups = new Map<string, Group>();
  for (const entry of pending) {
    const base = `${entry.member.diagnostic.code}\u0000${entry.where.key ?? ""}`;
    const classes = stated.get(base);
    // No object: never grouped. A classless report joins its code on its object only where exactly one class is stated.
    const key = entry.where.key === null ? `single\u0000${entry.order}`
      : entry.cls !== null ? `${base}\u0000${entry.cls}`
      : !classes || classes.size === 0 ? `${base}\u0000`
      : classes.size === 1 ? `${base}\u0000${[...classes][0]}`
      : `single\u0000${entry.order}`;
    const group = groups.get(key);
    if (group) {
      group.entries.push(entry);
      if (!group.entity && entry.where.entity) group.entity = entry.where.entity;
    } else {
      groups.set(key, { code: entry.member.diagnostic.code, object: entry.where.object, entity: entry.where.entity, objectKey: entry.where.key, entries: [entry], order: entry.order });
    }
  }

  return [...groups.entries()].sort((a, b) => a[1].order - b[1].order).map(([key, group]): IssueItem => {
    const members: IssueMember[] = group.entries.map((entry) => ({
      ...entry.member,
      otherRefs: otherRefsOf(entry.member.diagnostic, inputs.model, group.objectKey),
      ownDetail: ownDetail(entry.member)
    }));
    const home = members.reduce<IssueSource>((best, member) => (HOME_RANK[member.source] < HOME_RANK[best] ? member.source : best), "diagnostics");
    const severity = members.reduce<Diagnostic["severity"]>(
      (worst, member) => (rank(member.diagnostic.severity) < rank(worst) ? member.diagnostic.severity : worst),
      "info"
    );
    // The most severe report leads; among equals one that states the class, then arrival order.
    const lead = members.reduce((best, member) => {
      const byRank = rank(member.diagnostic.severity) - rank(best.diagnostic.severity);
      if (byRank !== 0) return byRank < 0 ? member : best;
      return classOf(best.diagnostic) === null && classOf(member.diagnostic) !== null ? member : best;
    });
    const firstCheck = members.find((member) => member.finding)?.finding ?? null;
    return {
      key,
      code: group.code,
      severity,
      object: group.object,
      entity: group.entity,
      objectLabel: displayRef(group.object, group.entity, inputs.model),
      home,
      group: home === "model-checks" && firstCheck ? firstCheck.group : null,
      members,
      lead,
      differences: excerptsAllowed(members) ? messageDifferences(members.map((member) => member.shown.message)) : []
    };
  });
}

/** Whether an item may show excerpts of its messages: every distinct message has a member shown
 * exactly as its producer gave it, so an excerpt is cut from the producer's own text, never from
 * supplied, unverified text. An opened row marks its excerpt within its message. */
function excerptsAllowed(members: readonly IssueMember[]): boolean {
  const distinct = [...new Set(members.map((member) => member.diagnostic.message))];
  return distinct.every((message) => members.some((member) => member.diagnostic.message === message && member.shown.message === message));
}

/** The words that tell distinct messages apart: what is left of each once the
 * words all of them begin and end with are taken away. One per distinct message,
 * in order; empty where there is only one distinct message. */
export function messageDifferences(messages: readonly string[]): string[] {
  const distinct = [...new Set(messages)];
  if (distinct.length < 2) return [];
  const words = distinct.map((message) => message.split(/\s+/));
  const shortest = Math.min(...words.map((list) => list.length));
  let prefix = 0;
  while (prefix < shortest && words.every((list) => list[prefix] === words[0][prefix])) prefix += 1;
  let suffix = 0;
  while (suffix < shortest - prefix && words.every((list) => list[list.length - 1 - suffix] === words[0][words[0].length - 1 - suffix])) suffix += 1;
  return words.map((list) => list.slice(prefix, list.length - suffix).join(" ").replace(/[;:,]$/, "") || "(no other words)");
}

/** The items that hold a solve report and are blocking or errors: what a solve that did not solve
 * leaves a person to do, in the order the drawer lists them. */
export function solveBlockingItems(items: readonly IssueItem[]): IssueItem[] {
  return items.filter((item) => (item.severity === "blocking" || item.severity === "error") && item.members.some((member) => member.origin === "solve"));
}

/** Orders items as a person meets them: most severe first, then as reported. */
export function bySeverity(items: readonly IssueItem[]): IssueItem[] {
  return items.map((item, index) => ({ item, index }))
    .sort((a, b) => rank(a.item.severity) - rank(b.item.severity) || a.index - b.index)
    .map(({ item }) => item);
}

export function countItemsBySeverity(items: readonly IssueItem[]): Record<Diagnostic["severity"], number> {
  const counts = { blocking: 0, error: 0, warning: 0, info: 0 };
  for (const item of items) counts[item.severity in counts ? item.severity : "info"] += 1;
  return counts;
}

/** Whether an item has anything a person may want to compare: more than one report. */
export function isGrouped(item: IssueItem): boolean {
  return item.members.length > 1;
}

/** The last solve, where it ended without a result for a reason the job gives: one
 * item, which the solve status line's "Show in Issues" opens. A stopped solve is the
 * person's own act, not an item. */
export type SolveIssue = Readonly<{ state: "failed" | "refused"; jobId: string }>;

export function solveIssueOf(solveJob: SolveJobAuditState | null | undefined): SolveIssue | null {
  if (!solveJob) return null;
  if (solveJob.state === "failed" || solveJob.state === "refused" || isResultTooLargeSolveJob(solveJob)) {
    return { state: solveJob.state === "failed" ? "failed" : "refused", jobId: solveJob.job_id };
  }
  return null;
}

/** Everything the Issues drawer lists, and the count the Issues controls show. */
export type IssueSummary = Readonly<{
  items: readonly IssueItem[];
  /** The missing-input blockers the drawer's Missing Data panel lists. */
  missingInputs: number;
  solve: SolveIssue | null;
  /** items + missing inputs + the last solve's failure or refusal. */
  total: number;
  /** How many reports the items gather (each diagnostic and finding once). */
  reports: number;
}>;

export function issueSummary(inputs: IssueInputs & { missingInputs: number; solveJob?: SolveJobAuditState | null }): IssueSummary {
  const items = groupIssues(inputs);
  const solve = solveIssueOf(inputs.solveJob);
  return {
    items,
    missingInputs: inputs.missingInputs,
    solve,
    total: items.length + inputs.missingInputs + (solve ? 1 : 0),
    reports: items.reduce((count, item) => count + item.members.length, 0)
  };
}

/** Items the drawer shows under one list. */
export function itemsAt(summary: IssueSummary, home: IssueSource): IssueItem[] {
  return summary.items.filter((item) => item.home === home);
}

/**
 * The session's issues: the inputs the drawer reads, gathered once, so the
 * toolbar, the rail, the status bar and the drawer all show the same items.
 */
export function useIssueSummary({
  model,
  knowledge,
  result,
  operationOutcomes,
  modelCheckRun,
  solveJob = null
}: {
  model: PreviewModel | null;
  knowledge: DesignKnowledge | null;
  result: SessionResult | null;
  operationOutcomes: Record<string, OperationOutcome>;
  modelCheckRun: ModelCheckRun | null;
  solveJob?: SolveJobAuditState | null;
}): IssueSummary {
  const publishedLibraryRefs = usePublishedLibraryRefFindings();
  return useMemo(() => {
    if (!model) return issueSummary({ missingInputs: 0, solveJob });
    const { diagnostics, shownDiagnostics, diagnosticOrigins } = diagnosticsListOf(model, knowledge, result, publishedLibraryRefs);
    return issueSummary({
      model,
      diagnosticOrigins,
      modelChecks: modelCheckRun?.findings ?? [],
      operations: Object.values(operationOutcomes).flatMap((outcome) => outcome.diagnostics) as Diagnostic[],
      diagnostics,
      shownDiagnostics,
      missingInputs: countMissingDataBlockers({ model, result }),
      solveJob
    });
  }, [model, knowledge, result, operationOutcomes, modelCheckRun, solveJob, publishedLibraryRefs]);
}

/** The diagnostics list the drawer's Diagnostics panel reads: the model's own, the
 * document validator's, the library references', design knowledge's and the
 * result's, as given and as shown. */
export function diagnosticsListOf(
  model: PreviewModel,
  knowledge: DesignKnowledge | null,
  result: SessionResult | null,
  publishedLibraryRefs: Parameters<typeof currentLibraryRefFindings>[1]
): { diagnostics: Diagnostic[]; shownDiagnostics: Diagnostic[]; diagnosticOrigins: IssueOrigin[] } {
  const parts: Array<[IssueOrigin, readonly Diagnostic[]]> = [
    ["model", model.diagnostics ?? []],
    ["model document", modelDocumentDiagnostics(model)],
    ["library reference", libraryRefDiagnostics(currentLibraryRefFindings(model, publishedLibraryRefs))],
    ["design knowledge", knowledge?.diagnostics ?? []]
  ];
  const before = parts.flatMap(([, list]) => list);
  const origins = parts.flatMap(([origin, list]) => list.map(() => origin));
  const raw = result?.diagnostics ?? [];
  const shown = presentedResultDiagnostics(result);
  return {
    diagnostics: [...before, ...raw],
    shownDiagnostics: [...before, ...shown],
    diagnosticOrigins: [...origins, ...raw.map((): IssueOrigin => "solve")]
  };
}
