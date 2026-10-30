import type { ReactElement } from "react";
import { CENTER_OF_GRAVITY_CONVENTION, RIGID_SOLVER_CONSUMPTIONS, type ComponentDraft } from "./componentIntent";

type TextInput = (props: { label: string; testId: string; value: string; onChange: (value: string) => void }) => ReactElement;
type RigidKey = "rigidStiffnessScale" | "rigidScaleSourceReference" | "rigidSolverConsumption";

/** W-15 (DEL-03-05): a valve's, flange's or reducer's optional DEC-045
 * stiffness scale with its source reference and its solver-consumption mode,
 * shared by the inspector and viewport creation forms, with the owner's R10
 * centre-of-gravity convention. Nothing is preselected or defaulted. */
export function RigidDraftFields({ draft, prefix, TextInput, update }: {
  draft: ComponentDraft;
  prefix: string;
  TextInput: TextInput;
  update: (key: RigidKey, value: string) => void;
}) {
  return (
    <>
      <p role="note" data-testid={`${prefix}-component-cog-convention`}>{CENTER_OF_GRAVITY_CONVENTION}</p>
      <TextInput label="Stiffness scale" testId={`${prefix}-component-stiffness-scale`} value={draft.rigidStiffnessScale} onChange={(value) => update("rigidStiffnessScale", value)} />
      <TextInput label="Stiffness scale source reference" testId={`${prefix}-component-scale-source`} value={draft.rigidScaleSourceReference} onChange={(value) => update("rigidScaleSourceReference", value)} />
      <label>
        <span>Solver consumption</span>
        <select aria-label="New component solver consumption" data-testid={`${prefix}-component-solver-consumption`} value={draft.rigidSolverConsumption} onChange={(event) => update("rigidSolverConsumption", event.target.value)}>
          <option value="">Not declared</option>
          {RIGID_SOLVER_CONSUMPTIONS.map((mode) => <option key={mode} value={mode}>{mode.replaceAll("_", " ")}</option>)}
        </select>
      </label>
      <p>A stiffness scale needs its source reference and applies, with the component weight, under mechanics geometry only.</p>
    </>
  );
}
