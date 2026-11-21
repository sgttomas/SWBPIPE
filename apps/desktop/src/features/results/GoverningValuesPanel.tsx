/** The governing values at the head of the Results stage (DEL-07-05 OUT-001): each names its entity,
 * its case, its component and its value with its unit, and opens the table that holds its row with
 * that row in view and selected. A quantity the result does not have says so in a few words. */
import { useMemo, type ReactNode } from "react";
import type { SessionResult } from "../../services/nativeResult";
import type { MechanicsResult, PreviewModel } from "../../types";
import type { SessionRuleCheckRun } from "../report/renderableReportInput";
import type { ResultOverlayStanding } from "./numericalHealth";
import { EngineeringQuantity } from "./EngineeringQuantity";
import { governingValues, isAbsent, type GoverningDisplacement, type GoverningReaction, type GoverningStress, type GoverningValue, type GoverningValues } from "./governingValues";

/** What a governing value opened asks of its table, once per serial: the row to show as current. */
export type GoverningRequestInput = Readonly<
  | { kind: "displacement" }
  | { kind: "support"; supportId: string; caseKey: string | null }
  | { kind: "stress"; groupId: string; mode: "stress" | "ratio" }
>;
export type GoverningRequest = GoverningRequestInput & Readonly<{ serial: number }>;

export const GOVERNING_VALUES_LABEL = "Governing values";

/** A case by its short reference (the key's id, e.g. "load:L-100"); its full name is the hover text. */
export function governingCaseText(item: Pick<GoverningValue, "caseKey" | "caseLabel">): string {
  const key = item.caseKey ?? "";
  const at = key.indexOf(":");
  return at > 0 ? key.slice(at + 1) : item.caseLabel;
}
/** The entity of a located value: its first part ("pipe:P-100 · station · method" reads "pipe:P-100"). */
export function governingEntityText(label: string): string {
  return label.split(" · ")[0];
}

function ValueButton({ item, testId, onOpen, prefix }: { item: GoverningValue; testId: string; onOpen: () => void; prefix?: string }) {
  return (
    <button type="button" className="governing-value" data-testid={testId} onClick={onOpen}
      title={`${item.entityLabel}, ${item.caseLabel}. Opens its row in its table.`}>
      <span className="governing-component">{prefix ?? item.component}</span>
      <EngineeringQuantity quantity={{ value: item.value, unit: item.unit, dimension_id: item.dimension }} />
      <span className="governing-entity">{governingEntityText(item.entityLabel)}</span>
      <span className="governing-case">{governingCaseText(item)}</span>
    </button>
  );
}

function Row({ label, testId, children }: { label: string; testId: string; children: ReactNode }) {
  return (
    <div className="governing-row" data-testid={testId}>
      <span className="governing-label">{label}</span>
      <div className="governing-items">{children}</div>
    </div>
  );
}

function Absence({ text, testId }: { text: string; testId: string }) {
  return <span className="governing-absent" data-testid={testId}>{text}</span>;
}

export function GoverningValuesPanel({ values, onOpenDisplacement, onOpenReaction, onOpenStress, onOpenWeight }: {
  values: GoverningValues;
  onOpenDisplacement: (item: GoverningDisplacement) => void;
  onOpenReaction: (item: GoverningReaction) => void;
  onOpenStress: (item: GoverningStress, kind: "stress" | "ratio") => void;
  onOpenWeight: () => void;
}) {
  const { displacement, reactions, stresses, ratios } = values;
  return (
    <section className="governing-values" aria-label={GOVERNING_VALUES_LABEL} data-testid="governing-values">
      <Row label="Largest displacement" testId="governing-displacement">
        {isAbsent(displacement) ? <Absence text={displacement.absent} testId="governing-displacement-absent" />
          : <ValueButton item={displacement} testId="governing-displacement-value" onOpen={() => onOpenDisplacement(displacement)} />}
      </Row>
      <Row label="Largest support reactions" testId="governing-reactions">
        {reactions.map((item) => isAbsent(item)
          ? <span key={item.column} className="governing-value governing-value-absent" data-testid={`governing-reaction-${item.column}`}><span className="governing-component">{item.column}</span> <Absence text={item.absent} testId={`governing-reaction-${item.column}-absent`} /></span>
          : <ValueButton key={item.column} item={item} testId={`governing-reaction-${item.column}`} onOpen={() => onOpenReaction(item)} />)}
      </Row>
      <Row label="Largest stress" testId="governing-stress">
        {isAbsent(stresses) ? <Absence text={stresses.absent} testId="governing-stress-absent" />
          : stresses.map((item) => <ValueButton key={item.groupId} item={item} prefix={stresses.length > 1 ? item.groupLabel : undefined} testId={`governing-stress-${item.groupId}`} onOpen={() => onOpenStress(item, "stress")} />)}
      </Row>
      <Row label="Largest ratio" testId="governing-ratio">
        {isAbsent(ratios) ? <Absence text={ratios.absent} testId="governing-ratio-absent" />
          : ratios.map((item) => <ValueButton key={item.groupId} item={item} prefix={item.groupLabel} testId={`governing-ratio-${item.groupId}`} onOpen={() => onOpenStress(item, "ratio")} />)}
      </Row>
      <Row label="Weight" testId="governing-weight">
        <button type="button" className="governing-value" data-testid="governing-weight-open" onClick={onOpenWeight}>
          <span className="governing-component">Weight and centre of gravity</span>
          <span className="governing-case">no solve needed; enter gravity</span>
        </button>
      </Row>
    </section>
  );
}

/** The governing values of `result` for the chosen case, read once per input. */
export function GoverningValuesFor(props: {
  model: PreviewModel;
  result: SessionResult | null;
  fullResult?: MechanicsResult | null;
  standing: ResultOverlayStanding | null | undefined;
  ruleCheckRun?: SessionRuleCheckRun | null;
  caseKey: string;
} & Omit<Parameters<typeof GoverningValuesPanel>[0], "values">) {
  const { model, result, fullResult = null, standing, ruleCheckRun = null, caseKey } = props;
  const values = useMemo(() => governingValues({ model, result, fullResult, standing, ruleCheckRun, caseKey }), [model, result, fullResult, standing, ruleCheckRun, caseKey]);
  return <GoverningValuesPanel values={values} onOpenDisplacement={props.onOpenDisplacement} onOpenReaction={props.onOpenReaction} onOpenStress={props.onOpenStress} onOpenWeight={props.onOpenWeight} />;
}
