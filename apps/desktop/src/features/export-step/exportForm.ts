/** DEL-07-12: the one place where the form of an export's row is chosen.
 *
 * A format's standing limits are the entries its generator adds to every package of that format whatever the
 * model. The row states them, and they do not stop the press. The record opens unasked, before the file is
 * written, when the export is withheld or refused, when the written file's values are replaced, or when the
 * package holds a loss, an unsupported-behaviour entry, a finding or a failed validation that this model (or its
 * result) produced. Otherwise the row holds the link, and one press writes the file. Every row calls
 * `chooseExportForm` with its package's record and the export control's disposition. */

export type ExportFormat = "result-export" | "stress-neutral" | "pcf" | "review-geometry" | "local-fea";

/** One entry of a package's record, in the generator's own terms. `content` is what the file holds (the solve's
 * own diagnostics, review rows and disclosed rows of the result export): it never opens the record first. */
export type RecordEntry = {
  kind: "loss" | "unsupported" | "finding" | "content";
  /** The generator's identifier: a loss id, a behaviour label or a diagnostic code. */
  term: string;
  /** The generator's category or status, where it gives one (exported, omitted, approximated, TBD, …). */
  category?: string;
  severity?: string;
  /** The generator's own words for the entry. */
  text?: string;
};

export type PackageRecord = {
  format: ExportFormat;
  entries: ReadonlyArray<RecordEntry>;
  /** The package's validation state, where its generator writes one. */
  validation?: { status: string; passed: boolean } | null;
};

/** What the export control will do with the file. Until the control reports its disposition for a row, the row
 * passes `AS_GIVEN`, in one place, so that the control's disposition replaces it there. */
export type ControlDisposition =
  | { kind: "as-given" }
  | { kind: "replaced"; count: number }
  | { kind: "withheld"; reason: string };

export const AS_GIVEN: ControlDisposition = Object.freeze({ kind: "as-given" });

export type ExportForm =
  | { form: "write-on-press"; standing: ReadonlyArray<RecordEntry> }
  | {
      form: "record-first";
      standing: ReadonlyArray<RecordEntry>;
      why: "withheld" | "replaced" | "this-model";
      /** The entries that stand in the way, first in the opened record. */
      causes: ReadonlyArray<RecordEntry>;
      /** The reason in plain words, for a withheld or refused export. */
      reason?: string;
      /** The count of replaced values, for a file written with replacements. */
      replaced?: number;
    };

type StandingEntry = {
  kind: "loss" | "unsupported" | "finding";
  term: string;
  /** The generator line that adds the entry with no condition, or (unplaced) the lines that make it present. */
  source: string;
  /** True where the entry states what the file does carry (an "exported" loss entry). */
  carried?: true;
  /** False where the entry is fixed in wording but its presence depends on the model: it is not a standing limit,
   * and the export keeps the record first while it is present. */
  placed?: false;
};

/** Each format's fixed entries, read from the generators' code (not from any one model's output). The tests in
 * `exportForm.standing.test.tsx` build each package from two different models and check that the entries the two
 * hold in common are exactly these, so that this table fails loudly when a generator changes. */
export const FIXED_ENTRIES: Readonly<Record<ExportFormat, ReadonlyArray<StandingEntry>>> = Object.freeze({
  "result-export": [],
  "stress-neutral": [
    { kind: "loss", term: "loss:desktop-preview:stress-neutral-exported", carried: true, source: "stress-neutral/StressNeutralExportPanel.tsx:311 (reason quotes the result's counts, line 571)" },
    { kind: "loss", term: "loss:desktop-preview:comparison-semantics-tbd", source: "stress-neutral/StressNeutralExportPanel.tsx:314" },
    { kind: "loss", term: "loss:desktop-preview:private-rule-payload-redacted", source: "stress-neutral/StressNeutralExportPanel.tsx:318" },
    { kind: "finding", term: "SN-DESKTOP-PREVIEW-COMPARISON-TBD", source: "stress-neutral/StressNeutralExportPanel.tsx:264, 333" }
  ],
  pcf: [
    { kind: "loss", term: "loss:desktop-preview:pipe-segments-exported", carried: true, source: "pcf-export/PcfExportPanel.tsx:754" },
    { kind: "loss", term: "loss:desktop-preview:supports-omitted", source: "pcf-export/PcfExportPanel.tsx:766" },
    { kind: "loss", term: "loss:desktop-preview:components-approximated", source: "pcf-export/PcfExportPanel.tsx:778" },
    { kind: "loss", term: "loss:desktop-preview:translator-defaults-delegated", source: "pcf-export/PcfExportPanel.tsx:790" },
    { kind: "loss", term: "loss:desktop-preview:boundary-conditions-unsupported", source: "pcf-export/PcfExportPanel.tsx:802" },
    { kind: "loss", term: "loss:desktop-preview:target-profile-and-nominal-size-tbd", source: "pcf-export/PcfExportPanel.tsx:814" },
    { kind: "finding", term: "PCF-TARGET-PROFILE-TBD", source: "pcf-export/PcfExportPanel.tsx:667" },
    { kind: "finding", term: "PCF-NOMINAL-SIZE-TBD", placed: false, source: "pcf-export/PcfExportPanel.tsx:632 (present whenever a segment exists; line 466 writes every segment's size as TBD_SOURCE_REQUIRED)" }
  ],
  "review-geometry": [
    { kind: "loss", term: "loss:pipe-segments-centerline-only", source: "review-geometry/ReviewGeometryPanel.tsx:677" },
    { kind: "loss", term: "loss:supports-point-markers", source: "review-geometry/ReviewGeometryPanel.tsx:684" },
    { kind: "loss", term: "loss:components-point-markers", source: "review-geometry/ReviewGeometryPanel.tsx:691" },
    { kind: "loss", term: "loss:loads-and-combinations-omitted", source: "review-geometry/ReviewGeometryPanel.tsx:698" },
    { kind: "loss", term: "loss:glb-and-viewer-behavior-tbd", source: "review-geometry/ReviewGeometryPanel.tsx:708" },
    { kind: "finding", term: "REVIEW-GEOMETRY-VISUAL-ONLY", source: "review-geometry/ReviewGeometryPanel.tsx:736" }
  ],
  "local-fea": [
    { kind: "unsupported", term: "mesh_generation_not_performed", source: "local-fea-handoff/LocalFeaHandoffPanel.tsx:568" },
    { kind: "unsupported", term: "target_format_not_selected", source: "local-fea-handoff/LocalFeaHandoffPanel.tsx:569" },
    { kind: "unsupported", term: "boundary_transfer_requires_review", source: "local-fea-handoff/LocalFeaHandoffPanel.tsx:570" },
    { kind: "unsupported", term: "local_detail_assumption_unresolved", source: "local-fea-handoff/LocalFeaHandoffPanel.tsx:571" },
    { kind: "unsupported", term: "external_solver_not_invoked", source: "local-fea-handoff/LocalFeaHandoffPanel.tsx:572" },
    { kind: "finding", term: "LOCAL-FEA-MESH-NOT-GENERATED", source: "local-fea-handoff/LocalFeaHandoffPanel.tsx:497" },
    { kind: "finding", term: "LOCAL-FEA-TARGET-FORMAT-TBD", source: "local-fea-handoff/LocalFeaHandoffPanel.tsx:504" },
    { kind: "finding", term: "LOCAL-FEA-BOUNDARY-TRANSFER-REVIEW", source: "local-fea-handoff/LocalFeaHandoffPanel.tsx:511" },
    { kind: "finding", term: "LOCAL-FEA-EXTERNAL-SOLVER-NOT-INVOKED", source: "local-fea-handoff/LocalFeaHandoffPanel.tsx:518" }
  ]
});

function standingTerms(format: ExportFormat): Set<string> {
  return new Set(FIXED_ENTRIES[format].filter((entry) => entry.placed !== false).map((entry) => entry.term));
}

/** The one rule. Pure: the same record and disposition always give the same form. */
export function chooseExportForm(record: PackageRecord, disposition: ControlDisposition): ExportForm {
  const fixed = standingTerms(record.format);
  const carried = new Set(FIXED_ENTRIES[record.format].filter((entry) => entry.carried).map((entry) => entry.term));
  // The row states the format's standing limits that this package holds: its losses and unsupported-behaviour
  // entries, not an entry that states what the file does carry. Its standing findings are in the record.
  const standing = record.entries.filter((entry) => fixed.has(entry.term) && !carried.has(entry.term) && entry.kind !== "finding");
  if (disposition.kind === "withheld") return { form: "record-first", standing, why: "withheld", causes: [], reason: disposition.reason };
  const causes = record.entries.filter((entry) =>
    entry.kind !== "content" && !fixed.has(entry.term) && !(entry.kind === "finding" && entry.severity === "info"));
  const failed = record.validation && !record.validation.passed
    ? [{ kind: "finding" as const, term: "validation", category: record.validation.status, text: `validation=${record.validation.status}` }]
    : [];
  if (disposition.kind === "replaced") return { form: "record-first", standing, why: "replaced", causes: [...failed, ...causes], replaced: disposition.count };
  if (causes.length > 0 || failed.length > 0) return { form: "record-first", standing, why: "this-model", causes: [...failed, ...causes] };
  return { form: "write-on-press", standing };
}

/** The words of the row and the step, kept together so that they change in one place. */
export const EXPORT_FORM_WORDS = Object.freeze({
  /** Where the standing limits fit on one line: the generator's terms themselves. */
  standingLine: (terms: ReadonlyArray<string>) => `Does not carry: ${terms.join("; ")}`,
  /** Otherwise their count, as a control that opens the record. */
  standingCount: (count: number) => `${count} ${count === 1 ? "thing" : "things"} this format does not carry`,
  /** The row's button, where the record opens first because of this model's entries. */
  reviewThisModel: (count: number) => `Review ${count} ${count === 1 ? "entry" : "entries"} from this model, then export…`,
  /** The row's button, where values would be replaced. */
  reviewReplaced: (count: number) => `Review ${count} replaced ${count === 1 ? "value" : "values"}, then export…`,
  /** The row's button, where the export is withheld or refused. */
  reviewWithheld: "Review…",
  /** The control that opens the whole record from a row that writes on the press. */
  record: "Record",
  /** The heading of the entries that stand in the way, first in the opened record. */
  causesHeading: "First, from this model",
  /** The row line of the result export and the stress-neutral JSON, which write on the press with no tick box. */
  ownLocalCopy: "Saves your own local copy, with the project's values.",
  /** In place of a save control that writes nothing (interim, until the native saves replace it). */
  unavailable: (file: string) => `${file}: not available. This file is withheld.`,
  /** The label of the three exports that are not finished for this release. */
  preview: "Preview",
  /** After what a preview export does not write: what a person uses instead for exchange. */
  exchangeInstead: "For exchange with another pipe stress program use Model batch file (.mbf)."
});

/** How long the joined terms may be and still be stated on one line at the row. */
export const STANDING_LINE_LIMIT = 72;

/** The term as the row states it: the last part of the generator's identifier. */
export function shortTerm(term: string): string {
  return term.slice(term.lastIndexOf(":") + 1);
}
