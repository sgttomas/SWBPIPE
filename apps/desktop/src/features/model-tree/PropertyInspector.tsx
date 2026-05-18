import type { EntityRef, PreviewModel } from "../../types";
import { entityLabel, selectedProperties } from "../model-workspace/modelView";

export function PropertyInspector({ model, selection }: { model: PreviewModel; selection: EntityRef }) {
  const fields = selectedProperties(model, selection);
  return (
    <div className="panel inspector" aria-label="Property inspector">
      <div className="panel-title">Properties</div>
      <h2>{entityLabel(model, selection.id)}</h2>
      <dl>
        {fields.map(([label, value]) => (
          <div key={label}>
            <dt>{label}</dt>
            <dd>{value}</dd>
          </div>
        ))}
      </dl>
    </div>
  );
}
