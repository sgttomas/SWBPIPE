// The Issues count and the Issues drawer's items, from one function.
//
// The rule: an item is one thing a person must look at, which is one finding
// code on one object (of one class: DEL-07-04 REQ-001 keeps the six classes
// apart, so two classes are never one item). Every report of that code on that object is a member of
// the one item, whoever reported it (the model checks, an operation, the model
// document, a library reference, design knowledge or the solve) and however
// many loads, cases or sources repeat it. A spring hanger's installed, cold and
// hot load warnings are one item for the hanger, listing the three loads. A
// member keeps its own code, severity and message; nothing is reworded or
// dropped, and the item's severity is its most severe member's. The number on
// the Issues controls is `issueSummary(...).total`, which is exactly the
// number of items the drawer lists.
import { useMemo } from "react";
import type { SessionResult } from "../../services/nativeResult";
import type { DesignKnowledge, Diagnostic, EntityRef, OperationOutcome, PreviewModel, SolveJobAuditState } from "../../types";
import { isResultTooLargeSolveJob } from "../workspace/solveJobAudit";
import { currentLibraryRefFindings, libraryRefDiagnostics, usePublishedLibraryRefFindings } from "../library/libraryRefFindings";
import { countMissingDataBlockers } from "../missing-data/MissingDataBlockingPanel";
import { modelDocumentDiagnostics } from "../model-data/documentDiagnostics";
import { resolveEntitySelection } from "../results/resultInterpretation";
import { presentedResultDiagnostics } from "../results/suppliedDiagnostics";
import type { ModelCheckFinding, ModelCheckGroupId, ModelCheckRun } from "./modelChecks";

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
}>;

export type IssueItem = Readonly<{
  key: string;
  code: string;
  severity: Diagnostic["severity"];
  /** The object the item is about: its first affected reference that names a model
   * entity, else its first affected reference; null where the finding names none. */
  object: string | null;
  entity: EntityRef | null;
  /** The list that shows the item: the model checks where they reported it, else an
   * operation's, else the diagnostics list. */
  home: IssueSource;
  /** For an item the model checks list, the group of its first model-check member. */
  group: ModelCheckGroupId | null;
  members: readonly IssueMember[];
  /** What tells the members' messages apart (for example "installed load
   * (hanger.installed_load)"), one per distinct message, in order; empty where
   * the members share one message. */
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

function objectOf(diagnostic: Diagnostic, model: PreviewModel | null | undefined, entity: EntityRef | null): { object: string | null; entity: EntityRef | null } {
  if (entity) return { object: entity.id, entity };
  const refs = diagnostic.affected_refs ?? [];
  if (model) {
    for (const ref of refs) {
      const resolved = resolveEntitySelection({
        ...model,
        nodes: model.nodes ?? [],
        pipe_segments: model.pipe_segments ?? [],
        supports: model.supports ?? [],
        components: model.components ?? [],
        load_cases: model.load_cases ?? []
      }, ref);
      if (resolved) return { object: ref, entity: resolved };
    }
  }
  return { object: refs[0] ?? null, entity: null };
}

/** A report's class as its producer supplied it (`diagnostic_class`, else `class`); null where it
 * supplied none. Never inferred (DEL-07-04 REQ-004). */
export function classOf(diagnostic: Diagnostic): string | null {
  return diagnostic.diagnostic_class ?? diagnostic.class ?? null;
}

/** How a class is shown in text: the supplied class as given, or that none was supplied. */
export function classText(diagnostic: Diagnostic): string {
  const supplied = classOf(diagnostic);
  return supplied === null ? "class not supplied" : `class ${supplied}`;
}

/** An item's class: the one class its members state (the grouping never joins two), shown once
 * on the item; "class not supplied" where no member states one. */
export function itemClassText(item: IssueItem): string {
  const stated = item.members.map((member) => classOf(member.diagnostic)).find((value) => value !== null) ?? null;
  return stated === null ? "class not supplied" : `class ${stated}`;
}

/** What tells this member's message apart from the item's other messages; null where the item's
 * members share one message. */
export function memberDifference(item: IssueItem, member: IssueMember): string | null {
  if (item.differences.length === 0) return null;
  const distinct = [...new Set(item.members.map((each) => each.diagnostic.message))];
  return item.differences[distinct.indexOf(member.diagnostic.message)] ?? null;
}

/** Groups every report into items (see the rule at the top of this file). */
export function groupIssues(inputs: IssueInputs): IssueItem[] {
  type Group = { code: string; object: string | null; entity: EntityRef | null; members: IssueMember[]; order: number };
  const groups = new Map<string, Group>();
  const add = (member: IssueMember, entity: EntityRef | null) => {
    const where = objectOf(member.diagnostic, inputs.model, entity);
    const base = `${member.diagnostic.code}\u0000${where.object ?? ""}\u0000`;
    // DEL-07-04 REQ-001: reports of two classes are never one entry. A report that
    // states no class joins the first group of its code on its object; class-less
    // reports held so far take the first class stated for them.
    const findingClass = classOf(member.diagnostic);
    let key = findingClass === null ? [...groups.keys()].find((candidate) => candidate.startsWith(base)) ?? base : `${base}${findingClass}`;
    if (findingClass !== null && !groups.has(key) && groups.has(base)) {
      groups.set(key, groups.get(base)!);
      groups.delete(base);
    }
    const group = groups.get(key);
    if (group) {
      group.members.push(member);
      if (!group.entity && where.entity) group.entity = where.entity;
    } else {
      groups.set(key, { code: member.diagnostic.code, object: where.object, entity: where.entity, members: [member], order: groups.size });
    }
  };
  for (const finding of inputs.modelChecks ?? []) add({ diagnostic: finding.diagnostic, shown: finding.diagnostic, source: "model-checks", origin: "model checks", finding }, finding.entity);
  for (const diagnostic of inputs.operations ?? []) add({ diagnostic, shown: diagnostic, source: "operation", origin: "operation" }, null);
  const shown = inputs.shownDiagnostics ?? inputs.diagnostics ?? [];
  const all = inputs.diagnostics ?? [];
  all.forEach((diagnostic, index) => add({ diagnostic, shown: shown[index] ?? diagnostic, source: "diagnostics", origin: inputs.diagnosticOrigins?.[index] ?? "model" }, null));

  return [...groups.entries()].sort((a, b) => a[1].order - b[1].order).map(([key, group]): IssueItem => {
    const home = group.members.reduce<IssueSource>((best, member) => (HOME_RANK[member.source] < HOME_RANK[best] ? member.source : best), "diagnostics");
    const severity = group.members.reduce<Diagnostic["severity"]>(
      (worst, member) => (rank(member.diagnostic.severity) < rank(worst) ? member.diagnostic.severity : worst),
      "info"
    );
    const firstCheck = group.members.find((member) => member.finding)?.finding ?? null;
    return {
      key,
      code: group.code,
      severity,
      object: group.object,
      entity: group.entity,
      home,
      group: home === "model-checks" && firstCheck ? firstCheck.group : null,
      members: group.members,
      differences: messageDifferences(group.members.map((member) => member.diagnostic.message))
    };
  });
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
