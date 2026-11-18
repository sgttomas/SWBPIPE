import { useState } from "react";
import type { DraftRecord } from "../rich-authoring/formSupport";
import { VirtualMultiTargetPicker } from "../workspace/VirtualTargetPicker";
import { addEverySource, fillFromTemplate, fillTargets, type FillScope } from "./bulkFill";
import { NumberText, Text } from "./loadStateAuthoring";

type Option = { value: string; label: string };
const SCOPES = ["missing", "range", "chosen"] as const;

/** G7: fills a per-member list from one of its own rows. The person picks the
 * template row and the targets; the draft gains one copy per target and is
 * reviewed and queued as before. */
export function TemplateFill({ name, noun, list, keyName, targets, onChange, omit = [], refuseTemplate, includable }: {
  /** The row name, as in the list's legends ("Element state"). */
  name: string;
  /** The target kind, singular ("pipe"). */
  noun: string;
  list: DraftRecord[];
  keyName: string;
  targets: readonly Option[];
  onChange: (rows: DraftRecord[]) => void;
  /** Members the editor keeps as authored but does not show; never copied. */
  omit?: readonly string[];
  /** Why a row cannot be a template, if it cannot. */
  refuseTemplate?: (row: DraftRecord) => string | undefined;
  /** A member of `omit` the person may include, explicitly and for one fill
   * only, when the chosen template holds it: offered as an unticked choice that
   * shows the text it would carry (review N1). */
  includable?: { key: string; label: string; describe: (value: unknown) => string };
}) {
  // The template is the chosen row itself: if that row is edited, removed or the
  // list is reset, the choice lapses and must be made again (review N3).
  const [chosenTemplate, setChosenTemplate] = useState<{ index: number; row: DraftRecord } | null>(null);
  const templateValid = chosenTemplate !== null && list[chosenTemplate.index] === chosenTemplate.row;
  const template = templateValid ? String(chosenTemplate.index) : "";
  const [scope, setScope] = useState<(typeof SCOPES)[number] | "">("");
  const [from, setFrom] = useState(""); const [to, setTo] = useState("");
  const [chosen, setChosen] = useState<string[]>([]);
  const [status, setStatus] = useState("");
  // Unticked whenever the template is chosen and after every fill: an inclusion
  // holds for the one fill the person ticked it for.
  const [include, setInclude] = useState(false);
  const lower = name.toLowerCase();
  const scopeLabels: Record<(typeof SCOPES)[number], string> = {
    missing: `every ${noun} without a ${lower}`,
    range: `${noun}s from … to … in model order`,
    chosen: `chosen ${noun}s`
  };
  const templateChoices = list.map((row, i) => ({ value: String(i), label: `${name} ${i + 1}${typeof row[keyName] === "string" ? ` (${row[keyName]})` : ""}` }));
  const ids = targets.map(t => t.value);
  const templateRefusal = templateValid ? refuseTemplate?.(chosenTemplate.row) : undefined;
  const offered = templateValid && includable !== undefined && chosenTemplate.row[includable.key] !== undefined && chosenTemplate.row[includable.key] !== null ? includable : undefined;
  const includeLabel = offered && `${name} fill includes the template's ${offered.label}`;
  const reason = list.length === 0 ? `Add and fill one ${lower} first; it is the template.`
    : template === "" ? (chosenTemplate ? "The rows changed; choose the template row again." : "Choose the template row.")
    : templateRefusal ? `${name} ${Number(template) + 1} cannot be a template: ${templateRefusal}.`
    : scope === "" ? `Choose which ${noun}s to fill.`
    : scope === "range" && (!from || !to) ? `Choose the first and last ${noun}.`
    : scope === "chosen" && chosen.length === 0 ? `Choose at least one ${noun}.` : undefined;
  function fill() {
    if (!templateValid || templateRefusal) return;
    const row = chosenTemplate.row;
    const fillScope: FillScope = scope === "range" ? { kind: "range", from, to } : scope === "chosen" ? { kind: "chosen", ids: chosen } : { kind: "missing" };
    const selected = fillTargets(ids, fillScope);
    const result = fillFromTemplate(list, row, keyName, selected, omit, offered && include ? [offered.key] : []);
    onChange(result.rows);
    setInclude(false);
    setStatus(selected.length === 0 ? `No ${noun} is in that range.`
      : `Added ${result.added.length} ${lower}${result.added.length === 1 ? "" : "s"} from ${name} ${Number(template) + 1}.`
        + (result.skipped.length ? ` ${result.skipped.length} ${noun}${result.skipped.length === 1 ? " already has" : "s already have"} one and ${result.skipped.length === 1 ? "was" : "were"} left unchanged.` : "")
        + (result.included.length && result.added.length ? ` Included on every added row, as chosen: ${result.included.join(", ")}.` : "")
        + (result.omitted.length && result.added.length ? ` Not copied (kept only on their own row): ${result.omitted.join(", ")}.` : "")
        + " Review the draft before queueing.");
  }
  return <fieldset><legend>Fill {lower}s from a template</legend>
    <label>{`${name} template`}
      <select aria-label={`${name} template`} value={template} onChange={e => { setInclude(false); setChosenTemplate(e.target.value === "" ? null : { index: Number(e.target.value), row: list[Number(e.target.value)] }); }}>
        <option value="">Choose a row</option>
        {templateChoices.map(c => <option key={c.value} value={c.value}>{c.label}</option>)}
      </select>
    </label>
    <label>{`${name} fill targets`}
      <select aria-label={`${name} fill targets`} value={scope} onChange={e => setScope(e.target.value as (typeof SCOPES)[number] | "")}>
        <option value="">Choose targets</option>
        {SCOPES.map(s => <option key={s} value={s}>{scopeLabels[s]}</option>)}
      </select>
    </label>
    {scope === "range" && <>
      <Text label={`${name} fill from ${noun}`} value={from} choices={ids} onChange={setFrom} />
      <Text label={`${name} fill to ${noun}`} value={to} choices={ids} onChange={setTo} />
    </>}
    {scope === "chosen" && <VirtualMultiTargetPicker label={`${name} fill ${noun}s`} options={[...targets]} values={chosen} onChange={v => setChosen([...v])} />}
    {offered && includeLabel && <>
      <label><input type="checkbox" aria-label={includeLabel} checked={include} onChange={e => setInclude(e.target.checked)} />{includeLabel}</label>
      <p>{`${name} ${Number(template) + 1} ${offered.label}: ${offered.describe(chosenTemplate!.row[offered.key])}. ${include ? "It will be copied to every row this fill adds." : "Not copied unless ticked; it belongs to its own row."}`}</p>
    </>}
    <button type="button" disabled={reason !== undefined} title={reason} onClick={fill}>{`Fill ${lower}s`}</button>
    {reason && <small>{reason}</small>}
    <p role="status">{status}</p>
  </fieldset>;
}

/** G7: one load source per primitive of this case, with the factor the person enters. */
export function EverySourceFill({ list, primitives, onChange }: { list: DraftRecord[]; primitives: readonly string[]; onChange: (rows: DraftRecord[]) => void }) {
  const [factor, setFactor] = useState("");
  const [status, setStatus] = useState("");
  const reason = primitives.length === 0 ? "This case has no stored primitives." : factor.trim() === "" ? "Enter the factor first." : undefined;
  function add() {
    try {
      const result = addEverySource(list, primitives, factor);
      onChange(result.rows);
      setStatus(`Added ${result.added.length} load source${result.added.length === 1 ? "" : "s"} with factor ${factor.trim()}; ${primitives.length - result.added.length} already listed. Review the draft before queueing.`);
    } catch (e) { setStatus(e instanceof Error ? e.message : String(e)); }
  }
  return <fieldset><legend>Every primitive of this case</legend>
    <NumberText label="Factor for every primitive (dimensionless)" value={factor} onChange={setFactor} />
    <button type="button" disabled={reason !== undefined} title={reason} onClick={add}>Add every primitive of this case</button>
    {reason && <small>{reason}</small>}
    <p role="status">{status}</p>
  </fieldset>;
}

/** G7/G8: replaces a case's draft with a copy from another case, through `onCopy`,
 * which returns the outcome or throws, leaving the draft unchanged. Only a case
 * still offered can be the source (review N2). */
export function CopyFromCase({ what, cases, onCopy }: { what: string; cases: readonly Option[]; onCopy: (caseId: string) => string }) {
  const [chosen, setSource] = useState("");
  const [status, setStatus] = useState("");
  const source = cases.some(c => c.value === chosen) ? chosen : "";
  const reason = cases.length === 0 ? `No other case has ${what}.` : source === "" ? "Choose the case to copy from." : undefined;
  function copy() {
    if (!source) return;
    try { setStatus(onCopy(source)); } catch (e) { setStatus(e instanceof Error ? e.message : String(e)); }
  }
  return <fieldset><legend>Copy {what} from another case</legend>
    <label>{`Copy ${what} from case`}
      <select aria-label={`Copy ${what} from case`} value={source} onChange={e => setSource(e.target.value)}>
        <option value="">Choose a case</option>
        {cases.map(c => <option key={c.value} value={c.value}>{c.label}</option>)}
      </select>
    </label>
    <button type="button" disabled={reason !== undefined} title={reason} onClick={copy}>{`Replace this draft's ${what} with the copy`}</button>
    {reason && <small>{reason}</small>}
    <p role="status">{status}</p>
  </fieldset>;
}
