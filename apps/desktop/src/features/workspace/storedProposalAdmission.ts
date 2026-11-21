/** Issue 519: the admission of a saved project's stored review proposal at open.
 *
 * A stored document is untrusted input: a store written by an older build, by another tool or
 * damaged can carry a proposal without the members the desktop's readers use, and one unguarded
 * read empties the window. A stored proposal enters the session only if it has every member a
 * reader reads without a guard, with the type that reader assumes (`ADMITTED_SHAPE`, established
 * from the readers: the Project page's storage and validation panels, the agent pane, the
 * proposal, diff preview, operation ledger and design workspace panels, the report and the
 * export, handoff and package panels). This is a check of presence and type, not a re-validation
 * of the proposal's operation against the model.
 *
 * A proposal that is not admitted is kept out of the session's proposal state and named in the
 * open's message (`storedProposalSentence`); the stored project is not changed by the refusal.
 * Only the proposal's identifier (when it is a short plain string) and member names from the fixed
 * shape below reach the message; no other stored text does. */
import type { AgentProposal } from "../../types";

export const STORED_PROPOSAL_NOT_ADMITTED = "STORED-PROPOSAL-NOT-ADMITTED";

/** The most member names a sentence lists before it gives the count of the rest. */
export const MAX_NAMED_MEMBERS = 6;
/** The longest identifier a sentence shows; a longer one is not shown. */
export const MAX_SHOWN_IDENTIFIER = 64;

type Rule = "text" | "flag" | "texts" | "object" | "list";
type Member = { path: string; rule: Rule; members?: Member[]; each?: Member[]; values?: "text" | "flag" };

/** The members the desktop's readers read without a guard, each with the type the reader assumes. */
export const ADMITTED_SHAPE: readonly Member[] = [
  { path: "proposal_id", rule: "text" },
  { path: "prompt", rule: "text" },
  { path: "rationale", rule: "text" },
  { path: "assumptions", rule: "texts" },
  {
    path: "operation", rule: "object", members: [
      { path: "operation_id", rule: "text" },
      { path: "operation_kind", rule: "text" },
      { path: "operation_status", rule: "text" },
      { path: "affected_entity_ids", rule: "texts" },
      {
        path: "changes", rule: "list", each: [
          { path: "change_id", rule: "text" },
          { path: "change_kind", rule: "text" },
          { path: "target_ref", rule: "text" },
          { path: "before", rule: "text" },
          { path: "after", rule: "text" }
        ]
      }
    ]
  },
  {
    path: "validation", rule: "object", values: "text", members: [
      { path: "application_status", rule: "text" },
      { path: "diff_preview_status", rule: "text" }
    ]
  },
  {
    path: "audit_boundary", rule: "object", values: "flag", members: [
      { path: "requires_user_acceptance", rule: "flag" },
      { path: "mutates_accepted_model_state", rule: "flag" },
      { path: "acceptance_recorded_as_review_only", rule: "flag" }
    ]
  },
  {
    path: "professional_boundary", rule: "object", values: "flag", members: [
      { path: "human_review_required", rule: "flag" },
      { path: "software_makes_compliance_claim", rule: "flag" },
      { path: "software_makes_approval_claim", rule: "flag" }
    ]
  }
];

export type StoredProposalAdmission =
  | { admitted: true; proposal: AgentProposal }
  | { admitted: false; notice: StoredProposalNotice };

/** What a sentence may say of a proposal that was not admitted: nothing but these. */
export type StoredProposalNotice = Readonly<{
  /** The identifier, only when it is a short plain string; otherwise null. */
  identifier: string | null;
  /** Member paths from `ADMITTED_SHAPE` that are absent. */
  missing: readonly string[];
  /** Member paths from `ADMITTED_SHAPE` present with another type (or "the proposal itself"). */
  wrongType: readonly string[];
}>;

function isPlainObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function fits(rule: Rule | "text" | "flag", value: unknown): boolean {
  switch (rule) {
    case "text": return typeof value === "string";
    case "flag": return typeof value === "boolean";
    case "texts": return Array.isArray(value) && value.every((item) => typeof item === "string");
    case "object": return isPlainObject(value);
    case "list": return Array.isArray(value);
  }
}

function check(holder: Record<string, unknown>, members: readonly Member[], prefix: string, missing: string[], wrongType: string[]) {
  for (const member of members) {
    const path = `${prefix}${member.path}`;
    if (!Object.prototype.hasOwnProperty.call(holder, member.path)) { missing.push(path); continue; }
    const value = holder[member.path];
    if (!fits(member.rule, value)) { wrongType.push(path); continue; }
    if (member.rule === "object") {
      const object = value as Record<string, unknown>;
      // Every value is read (shown entry by entry): a value of another type is named by its holder,
      // never by its own key, which is stored text.
      if (member.values && !Object.values(object).every((item) => fits(member.values!, item))) wrongType.push(`${path} (an entry)`);
      check(object, member.members ?? [], `${path}.`, missing, wrongType);
    }
    if (member.rule === "list") {
      const items = value as unknown[];
      if (!items.every(isPlainObject)) { wrongType.push(`${path}[]`); continue; }
      const itemMissing: string[] = [], itemWrong: string[] = [];
      for (const item of items) check(item as Record<string, unknown>, member.each ?? [], `${path}[].`, itemMissing, itemWrong);
      missing.push(...new Set(itemMissing));
      wrongType.push(...new Set(itemWrong));
    }
  }
}

/** A short plain identifier, or null: what a sentence may show of the stored proposal's own text. */
export function shownIdentifier(value: unknown): string | null {
  if (!isPlainObject(value)) return null;
  const id = value.proposal_id;
  return typeof id === "string" && id.length > 0 && id.length <= MAX_SHOWN_IDENTIFIER && /^[A-Za-z0-9._:-]+$/.test(id) ? id : null;
}

/** Admits a stored proposal only if it has every member of `ADMITTED_SHAPE` with its type. */
export function admitStoredProposal(value: unknown): StoredProposalAdmission {
  const missing: string[] = [], wrongType: string[] = [];
  if (!isPlainObject(value)) wrongType.push("the proposal itself");
  else check(value, ADMITTED_SHAPE, "", missing, wrongType);
  if (missing.length === 0 && wrongType.length === 0) return { admitted: true, proposal: value as AgentProposal };
  return { admitted: false, notice: Object.freeze({ identifier: shownIdentifier(value), missing: Object.freeze(missing), wrongType: Object.freeze(wrongType) }) };
}

/** What became of the stored proposal in the act the sentence reports. The sentence is one text for
 * the open and every save; only this clause changes, as grammar requires. */
export type StoredProposalOutcome = "open" | "kept" | "not-carried" | "replaced";

const OUTCOME_CLAUSE: Record<StoredProposalOutcome, string> = {
  open: "It is kept in the saved project as stored and is not shown or applied.",
  kept: "It was kept in the saved project as stored and is not shown or applied.",
  "not-carried": "This save made a new project, which does not carry it; this save did not read or change the saved project it came from.",
  replaced: "It was not carried into this save, and the save replaced the saved project, so it is no longer stored."
};

/** The Replace prompt's added sentence while the session holds the open project's stored proposal that
 * was not admitted (issue 519): a Replace writes the session's own proposal, so it does not keep it. */
export const REPLACE_REMOVES_STORED_PROPOSAL = "If the stored project still holds the review proposal this version could not open, replacing it removes that proposal; a new project would not carry it.";

export const STORED_EDITOR_INTENTS_NOT_READ = "STORED-EDITOR-INTENTS-NOT-READ";

/** The stored editor intents as the open adopts them for retained review context: a list as it is, none
 * as none; anything else is a damaged store, not adopted (`readable: false`). */
export function readStoredEditorIntents<T>(value: T[] | null | undefined): { intents: T[]; readable: boolean } {
  if (value == null) return { intents: [], readable: true };
  return Array.isArray(value) ? { intents: value, readable: true } : { intents: [], readable: false };
}

/** The open's sentence for stored editor intents that are not a list, in the same form as the proposal's:
 * plain statement first, the code last. No stored content is quoted. */
export function storedEditorIntentsSentence(): string {
  return `The review records stored with this project could not be read, because they are not a list. They are not shown, and a save of this project does not keep them. (${STORED_EDITOR_INTENTS_NOT_READ}: the stored editor intents; wrong type the editor intents themselves)`;
}

/** The session's private record of the open project's unadmitted stored proposal. */
export type UnadmittedStoredProposal = Readonly<{ projectId: string; generation: number; notice: StoredProposalNotice }>;

/** The sentence a project write adds for the open project's unadmitted stored proposal, or "": `kept`
 * when the store kept it (only a save in place against the revision the session opened keeps it);
 * written under another project id (a copy), not carried, with nothing said of the project it came
 * from, which the write did not read; written over the same project without keeping it, by a Replace
 * (`replaced`), or by a save carrying the session's own proposal, which says nothing more than any save
 * does. */
export function writtenStoredProposalSentence(
  unadmitted: UnadmittedStoredProposal | null,
  write: Readonly<{ projectId: string; generation: number; kept: boolean; replacedWhole: boolean }>
): string {
  if (!unadmitted || unadmitted.generation !== write.generation) return "";
  if (write.kept) return ` ${storedProposalSentence(unadmitted.notice, "kept")}`;
  if (unadmitted.projectId !== write.projectId) return ` ${storedProposalSentence(unadmitted.notice, "not-carried")}`;
  return write.replacedWhole ? ` ${storedProposalSentence(unadmitted.notice, "replaced")}` : "";
}

function listed(names: readonly string[], budget: number): { text: string; used: number } {
  const shown = names.slice(0, Math.max(0, budget));
  const rest = names.length - shown.length;
  return { text: `${shown.join(", ")}${rest > 0 ? `${shown.length ? " " : ""}and ${rest} more` : ""}`, used: shown.length };
}

/** The sentence a person reads: the plain statement first, then the code, the identifier and the
 * member names in parentheses. At most `MAX_NAMED_MEMBERS` names, then a count. */
export function storedProposalSentence(notice: StoredProposalNotice, outcome: StoredProposalOutcome): string {
  const which = notice.identifier ? `proposal \`${notice.identifier}\`` : "the stored review proposal";
  const parts: string[] = [];
  const missing = listed(notice.missing, MAX_NAMED_MEMBERS);
  if (notice.missing.length) parts.push(`missing ${missing.text}`);
  const wrong = listed(notice.wrongType, MAX_NAMED_MEMBERS - missing.used);
  if (notice.wrongType.length) parts.push(`wrong type ${wrong.text}`);
  return `A review proposal stored in this project was not opened, because this version cannot read it. ${OUTCOME_CLAUSE[outcome]} (${STORED_PROPOSAL_NOT_ADMITTED}: ${which}; ${parts.join("; ")})`;
}
