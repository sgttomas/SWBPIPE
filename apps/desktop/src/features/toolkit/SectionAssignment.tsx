import { useEffect, useState } from "react";
import { canonicalJsonString } from "../../services/hashService";
import { makeRichIntent } from "../rich-authoring/formSupport";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../types";

type Props = {
  model: PreviewModel;
  selection: EntityRef;
  onQueueIntent: (intent: EditorOperationIntent) => void;
  operationBusy?: boolean;
};

export function SectionAssignment({ model, selection, onQueueIntent, operationBusy = false }: Props) {
  const pipe = selection.type === "pipe"
    ? model.pipe_segments.find((item) => item.id === selection.id)
    : undefined;
  const [sectionId, setSectionId] = useState("");
  const [basis, setBasis] = useState<{
    model: PreviewModel;
    pipe: typeof pipe;
    before: string;
  } | null>(null);
  const [error, setError] = useState("");
  const [queued, setQueued] = useState("");

  useEffect(() => {
    let active = true;
    setBasis(null);
    setSectionId("");
    setError("");
    setQueued("");
    if (pipe) {
      canonicalJsonString(pipe)
        .then((before) => {
          if (active) setBasis({ model, pipe, before });
        })
        .catch(() => {
          if (active) {
            setError("The current pipe snapshot could not be prepared. Check that the operation engine is available.");
          }
        });
    }
    return () => { active = false; };
  }, [model, pipe]);

  if (!pipe) return null;
  const ready = basis?.model === model && basis?.pipe === pipe;
  const source = model.sections?.find((section) => section.id === sectionId);

  function queue(detach: boolean) {
    if (!ready || !basis || !pipe || operationBusy) return;
    if (detach ? !pipe.section_ref : !source || source.id === pipe.section_ref) return;
    const kind = detach ? "detach_section" : "assign_section";
    const intent = makeRichIntent(
      { object_type: "Element", ref: pipe.id },
      kind,
      "section_ref",
      basis.before,
      detach ? { source_section_ref: pipe.section_ref } : { section_ref: sectionId },
      detach ? "Use local pipe dimensions" : "Assign shared section"
    );
    onQueueIntent(intent);
    setQueued(detach
      ? "Detachment queued for validation and review."
      : `Assignment to ${source?.name} queued for validation and review.`);
  }

  return (
    <section
      id="section-assignment"
      tabIndex={-1}
      className="editor-intent"
      aria-label="Pipe section assignment"
    >
      <h3>Pipe section</h3>
      <p>
        {pipe.section_ref
          ? `Shared section: ${pipe.section_ref}. Diameter and wall follow that source. Edit the section record to update all assigned pipes, or detach to retain local dimensions.`
          : "Local dimensions. Choose a shared section to use its diameter and wall."}
        {" "}Per-pipe supplements remain local.
      </p>
      <label>
        Shared section
        <select
          aria-label="Shared section"
          value={sectionId}
          onChange={(event) => setSectionId(event.target.value)}
        >
          <option value="">Choose a section</option>
          {(model.sections ?? [])
            .filter((section) => section.section_type === "pipe")
            .map((section) => (
              <option key={section.id} value={section.id}>
                {section.name} ({section.id})
              </option>
            ))}
        </select>
      </label>
      {!model.sections?.length ? <p>Create a section first to assign shared dimensions.</p> : null}
      <button
        type="button"
        disabled={!ready || !source || source.id === pipe.section_ref || operationBusy}
        onClick={() => queue(false)}
      >
        Queue section assignment
      </button>
      <button
        type="button"
        disabled={!ready || !pipe.section_ref || operationBusy}
        onClick={() => queue(true)}
      >
        Queue detachment to local dimensions
      </button>
      {error ? <p role="alert">{error}</p> : !ready ? <p role="status">Preparing the current pipe snapshot…</p> : null}
      {queued ? <p role="status">{queued}</p> : null}
      <p>Review the affected pipes and exact dimensions in Pending changes before applying.</p>
    </section>
  );
}
