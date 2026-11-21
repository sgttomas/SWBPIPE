// The Loads page's layout parts (DEL-07-03 on the DEL-07-06 presentation): a creation form
// shown only after its command is chosen, a closed "More" disclosure for the fields a person
// does not choose for ordinary work, and the selected case's list of sections. Display only:
// nothing here builds or changes an operation.
import { forwardRef, type ReactNode } from "react";
import { X } from "lucide-react";

/** A creation form at the top of the editor column. It is a disclosure whose summary is not
 * shown: the page's commands open it, its Cancel closes it, and the workspace's command search
 * may open it directly (it opens a field's enclosing disclosures before focusing the field), so
 * `onOpenChange` follows the element's own state. */
export const CreateForm = forwardRef<HTMLDetailsElement, {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  testId: string;
  cancelTestId: string;
  children: ReactNode;
}>(function CreateForm({ open, onOpenChange, title, testId, cancelTestId, children }, ref) {
  return (
    <details
      ref={ref}
      className="loads-form"
      data-testid={testId}
      open={open}
      onToggle={(event) => {
        const next = event.currentTarget.open;
        if (next !== open) onOpenChange(next);
      }}
    >
      <summary>{title}</summary>
      <div className="loads-form-head">
        <h3>{title}</h3>
        {open ? (
          <button type="button" className="loads-quiet-button" data-testid={cancelTestId} onClick={() => onOpenChange(false)}>
            <X size={14} aria-hidden="true" />
            Cancel
          </button>
        ) : null}
      </div>
      {children}
    </details>
  );
});

/** The closed "More" disclosure inside a form or editor. */
export function More({ children, testId }: { children: ReactNode; testId?: string }) {
  return (
    <details className="loads-more" data-testid={testId}>
      <summary>More</summary>
      <div className="loads-fields">{children}</div>
    </details>
  );
}

/** One section of the selected case's editor. A disclosure section states in its summary what
 * the case has, and is open when the case has content. Another section is one more entry. */
export type CaseSection = {
  id: string;
  title: string;
  /** Present for a disclosure: what the case has, such as "none" or "temperature T1". */
  summary?: string;
  /** For a disclosure: open when the case has content. */
  hasContent?: boolean;
  /** Commands shown beside the section's heading (not for a disclosure). */
  actions?: ReactNode;
  render: () => ReactNode;
};

export function CaseSectionList({ caseId, sections }: { caseId: string; sections: readonly CaseSection[] }) {
  return (
    <>
      {sections.map((section) =>
        section.summary === undefined ? (
          <section key={section.id} className="loads-case-section" aria-label={section.title} data-testid={`load-case-section-${section.id}`}>
            <div className="loads-section-head">
              <h4>{section.title}</h4>
              {section.actions}
            </div>
            {section.render()}
          </section>
        ) : (
          // Keyed by case: choosing another case opens or closes each section by that case's content.
          <details
            key={`${section.id}:${caseId}`}
            className="loads-case-section loads-disclosure"
            data-testid={`load-case-section-${section.id}`}
            open={section.hasContent}
          >
            <summary>
              <span className="loads-disclosure-title">{section.title}:</span> <span className="loads-disclosure-summary">{section.summary}</span>
            </summary>
            <div className="loads-disclosure-body">{section.render()}</div>
          </details>
        )
      )}
    </>
  );
}

/** A display form of a reference: its type prefix ("load:", "combination:", "node:") is left out. */
export function shortRef(id: string): string {
  return id.replace(/^[A-Za-z_]+:/, "") || id;
}

const BASIS_LABEL: Record<string, string> = {
  mechanics: "Algebraic sum",
  result_state_subtraction: "Difference",
  range_envelope: "Range envelope",
  absolute_sum: "Absolute sum",
  srss: "SRSS"
};

/** A combination basis as a person reads it; an unknown stored basis is shown as stored. */
export function basisLabel(basis: string): string {
  return BASIS_LABEL[basis] ?? basis;
}

const CATEGORY_LABEL: Record<string, string> = {
  concentrated_force: "Force",
  distributed_force: "Distributed force",
  concentrated_moment: "Moment",
  thermal: "Thermal",
  imposed_displacement: "Imposed displacement",
  pressure: "Pressure (legacy)"
};

/** A primitive load's category as a person reads it; an unknown category is shown as stored. */
export function categoryLabel(category: string): string {
  return CATEGORY_LABEL[category] ?? category;
}

/** A direction as a person reads it: global_y is "Global Y", rotation_z (a moment's axis) is
 * "About Z"; a DOF (UX, RZ) is shown as is. */
export function directionLabel(direction: string): string {
  const axis = /^global_([xyz])$/.exec(direction);
  if (axis) return `Global ${axis[1].toUpperCase()}`;
  const about = /^rotation_([xyz])$/.exec(direction);
  return about ? `About ${about[1].toUpperCase()}` : direction;
}

export function plural(count: number, one: string, many: string): string {
  return `${count} ${count === 1 ? one : many}`;
}
