/** A case in a derived table (support loads, stress and ratio, displacement extremes): its id as the
 * result states it, as the result rows' Case column shows it, with the model's full label as its
 * hover text, so the value columns beside it keep their width at the default view's width. A note
 * the label carries after the case's own label (a range envelope, a sum) stays visible. */
import type { ResultBasisRef } from "../../types";

/** The id of a case key ("load_case:load:L-100" is "load:L-100"); the key itself where it has no type. */
export function caseIdOfKey(key: string): string {
  const at = key.indexOf(":");
  return at > 0 ? key.slice(at + 1) : key;
}

export function CaseRef({ basis, caseKey, label, ownLabel, testId }: {
  /** The case's basis, or its key where only the key is held. */
  basis?: ResultBasisRef | null;
  caseKey?: string;
  /** The label the table had shown: the hover text, whole. */
  label: string;
  /** The case's own label, when `label` adds a note after it; the note stays visible. */
  ownLabel?: string;
  testId?: string;
}) {
  const id = basis?.ref_id ?? (caseKey ? caseIdOfKey(caseKey) : null);
  if (!id) return <span data-testid={testId}>{label}</span>;
  const note = ownLabel && label.length > ownLabel.length && label.startsWith(ownLabel) ? label.slice(ownLabel.length).trim() : "";
  return (
    <span className="results-case-ref" title={label} data-testid={testId}>
      {id}{note ? <small> {note}</small> : null}
    </span>
  );
}
