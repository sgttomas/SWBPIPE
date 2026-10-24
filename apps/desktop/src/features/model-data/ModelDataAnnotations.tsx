// Property inspector: Tags and Notes for the selected entity and for the
// project, and a Provenance record section for materials, sections,
// components and supports (W-13 §3.1, §3.5, §6). Each edit is queued as a
// structured operation for validation, diff review and apply; clearing
// removes the member.
import { useEffect, useState } from "react";
import { currentLibraryRefFindings, usePublishedLibraryRefFindings } from "../library/libraryRefFindings";
import { MODEL_LIBRARY_REF_VERSION_CHANGED } from "../../services/libraryImportService";
import type { EditorOperationObjectType, EntityRef, ProvenanceRecord } from "../../types";
import {
  ANNOTATED_OBJECT_TYPES,
  PROVENANCE_RECORD_TYPES,
  REDISTRIBUTION_STATUSES,
  SOURCE_REVIEW_STATUSES,
  annotatedRecord,
  annotationIntent,
  disabledBy,
  parseTags,
  stableJson
} from "./modelDataIntents";
import { useModelDataQueue, type ModelDataQueueProps } from "./useModelDataQueue";

type Props = ModelDataQueueProps & { selection: EntityRef };

export function ModelDataAnnotations(props: Props) {
  const { model, selection } = props;
  const projectRef: EntityRef = { type: "project", id: model.project.id };
  const showsEntity = selection.type !== "project" && ANNOTATED_OBJECT_TYPES[selection.type] !== undefined && annotatedRecord(model, selection) !== null;
  return (
    <section className="model-data-annotations" aria-label="Tags, notes and provenance" data-testid="model-data-annotations">
      {showsEntity ? <AnnotationEditor {...props} target={selection} title={`Tags and notes: ${selection.id}`} /> : null}
      {showsEntity && PROVENANCE_RECORD_TYPES.includes(selection.type) ? <ProvenanceRecordEditor {...props} target={selection} /> : null}
      <AnnotationEditor {...props} target={projectRef} title="Project tags and notes" />
    </section>
  );
}

function AnnotationEditor(props: Props & { target: EntityRef; title: string }) {
  const { model, target } = props;
  const objectType = ANNOTATED_OBJECT_TYPES[target.type] as EditorOperationObjectType;
  const current = annotatedRecord(model, target) ?? {};
  const storedTags = Array.isArray(current.tags) ? (current.tags as string[]) : [];
  const storedNotes = typeof current.notes === "string" ? current.notes : "";
  const [tagsText, setTagsText] = useState(storedTags.join(", "));
  const [notesText, setNotesText] = useState(storedNotes);
  const { busy, error, notice, queue } = useModelDataQueue(props);
  useEffect(() => {
    setTagsText(storedTags.join(", "));
    setNotesText(storedNotes);
    // Re-seed when the target or its stored values change.
  }, [target.type, target.id, storedTags.join("\u0000"), storedNotes]);
  const testId = `annotations-${target.type}`;
  const entered = parseTags(tagsText);
  const tagsChanged = JSON.stringify(entered) !== JSON.stringify(storedTags);
  const notesChanged = notesText.trim() !== storedNotes;
  return (
    <fieldset className="model-data-fieldset" data-testid={testId}>
      <legend>{props.title}</legend>
      <label>Tags
        <input
          aria-label={`${props.title}: tags`}
          data-testid={`${testId}-tags`}
          placeholder="Comma-separated, e.g. tie-in, rev-B"
          value={tagsText}
          onChange={(event) => setTagsText(event.target.value)}
        />
      </label>
      <div className="model-data-actions">
        <button type="button" data-testid={`${testId}-tags-queue`} {...disabledBy(busy, [entered.length === 0, "Enter at least one tag; Clear tags removes them."], [!tagsChanged, "The tags are unchanged."])}
          onClick={() => void queue((canonicalize) => annotationIntent(canonicalize, objectType, target.id, current, "tags", entered, "Tags"))}>
          Queue tags
        </button>
        <button type="button" data-testid={`${testId}-tags-clear`} {...disabledBy(busy, [current.tags === undefined, "No tags are recorded."])}
          onClick={() => void queue((canonicalize) => annotationIntent(canonicalize, objectType, target.id, current, "tags", null, "Clear tags"))}>
          Clear tags
        </button>
      </div>
      <label>Notes
        <textarea
          aria-label={`${props.title}: notes`}
          data-testid={`${testId}-notes`}
          rows={3}
          value={notesText}
          onChange={(event) => setNotesText(event.target.value)}
        />
      </label>
      <div className="model-data-actions">
        <button type="button" data-testid={`${testId}-notes-queue`} {...disabledBy(busy, [!notesText.trim(), "Enter a note; Clear notes removes it."], [!notesChanged, "The note is unchanged."])}
          onClick={() => void queue((canonicalize) => annotationIntent(canonicalize, objectType, target.id, current, "notes", notesText.trim(), "Notes"))}>
          Queue notes
        </button>
        <button type="button" data-testid={`${testId}-notes-clear`} {...disabledBy(busy, [current.notes === undefined, "No note is recorded."])}
          onClick={() => void queue((canonicalize) => annotationIntent(canonicalize, objectType, target.id, current, "notes", null, "Clear notes"))}>
          Clear notes
        </button>
      </div>
      {error ? <p role="alert">{error}</p> : null}
      {notice ? <p role="status">{notice}</p> : null}
    </fieldset>
  );
}

const PROVENANCE_TEXT_FIELDS: Array<[keyof ProvenanceRecord, string]> = [
  ["source_name", "Source name (required)"],
  ["source_location", "Source location"],
  ["source_license", "Source license"],
  ["contributor", "Contributor"],
  ["contributor_certification", "Contributor statement"],
  ["reviewed_by", "Reviewed by"],
  ["reviewed_on", "Reviewed on (YYYY-MM-DD)"],
  ["verification_note", "Verification note"]
];

type ProvenanceDraft = Record<string, string>;

function draftFrom(record: unknown): ProvenanceDraft {
  const source = record && typeof record === "object" ? (record as Record<string, unknown>) : {};
  return Object.fromEntries(Object.entries(source).filter(([, value]) => typeof value === "string")) as ProvenanceDraft;
}

function ProvenanceRecordEditor(props: Props & { target: EntityRef }) {
  const { model, target } = props;
  const objectType = ANNOTATED_OBJECT_TYPES[target.type] as EditorOperationObjectType;
  const current = annotatedRecord(model, target) ?? {};
  const stored = current.provenance_record as Record<string, unknown> | undefined;
  const [draft, setDraft] = useState<ProvenanceDraft>(() => draftFrom(stored));
  const { busy, error, notice, queue } = useModelDataQueue(props);
  const publishedLibraryRefs = usePublishedLibraryRefFindings();
  const libraryRefFindings = currentLibraryRefFindings(model, publishedLibraryRefs).filter((finding) => finding.entity_ref === target.id);
  const storedKey = stableJson(stored ?? null);
  useEffect(() => {
    setDraft(draftFrom(stored));
  }, [target.type, target.id, storedKey]);
  const next = (): Record<string, unknown> => {
    const record: Record<string, unknown> = {};
    for (const [key, value] of Object.entries(draft)) if (value.trim()) record[key] = value.trim();
    // A library reference, when present, is kept as recorded (S2b sets it on library insert).
    if (stored && stored.library_ref !== undefined) record.library_ref = stored.library_ref;
    return record;
  };
  const set = (key: string, value: string) => setDraft((previous) => ({ ...previous, [key]: value }));
  return (
    <fieldset className="model-data-fieldset" data-testid="provenance-record-editor">
      <legend>Provenance record: {target.id}</legend>
      <p className="model-data-note">Source review records the review of the source data only.</p>
      {PROVENANCE_TEXT_FIELDS.map(([key, label]) => (
        <label key={key}>{label}
          <input aria-label={label} data-testid={`provenance-record-${key}`} value={draft[key] ?? ""} onChange={(event) => set(key, event.target.value)} />
        </label>
      ))}
      <label>Redistribution
        <select aria-label="Redistribution" data-testid="provenance-record-redistribution_status" value={draft.redistribution_status ?? ""}
          onChange={(event) => set("redistribution_status", event.target.value)}>
          <option value="">Not recorded</option>
          {REDISTRIBUTION_STATUSES.map((status) => <option key={status} value={status}>{status.replaceAll("_", " ")}</option>)}
        </select>
      </label>
      <label>Source review
        <select aria-label="Source review" data-testid="provenance-record-review_status" value={draft.review_status ?? ""}
          onChange={(event) => set("review_status", event.target.value)}>
          <option value="">Not recorded</option>
          {SOURCE_REVIEW_STATUSES.map((status) => <option key={status} value={status}>{status}</option>)}
        </select>
      </label>
      {stored?.library_ref !== undefined ? (
        <p className="model-data-note" data-testid="provenance-record-library-ref">Library record: {JSON.stringify(stored.library_ref)}</p>
      ) : null}
      {libraryRefFindings.map((finding) => (
        <p className="model-data-note" role="note" key={finding.code} data-testid={`provenance-record-library-warning-${finding.code}`}>
          {finding.code} (warning): {finding.message}{finding.code === MODEL_LIBRARY_REF_VERSION_CHANGED ? " Re-apply from library in the Library Manager to adopt the stored record." : ""}
        </p>
      ))}
      <div className="model-data-actions">
        <button type="button" data-testid="provenance-record-queue" {...disabledBy(busy, [!draft.source_name?.trim(), "Enter a source name."], [stableJson(next()) === storedKey, "The provenance record is unchanged."])}
          onClick={() => void queue((canonicalize) => annotationIntent(canonicalize, objectType, target.id, current, "provenance_record", next(), "Provenance record"))}>
          Queue provenance record
        </button>
        <button type="button" data-testid="provenance-record-clear" {...disabledBy(busy, [stored === undefined, "No provenance record is recorded."])}
          onClick={() => void queue((canonicalize) => annotationIntent(canonicalize, objectType, target.id, current, "provenance_record", null, "Clear provenance record"))}>
          Clear provenance record
        </button>
      </div>
      {error ? <p role="alert">{error}</p> : null}
      {notice ? <p role="status">{notice}</p> : null}
    </fieldset>
  );
}
