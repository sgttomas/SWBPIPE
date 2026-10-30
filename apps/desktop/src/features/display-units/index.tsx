import { createContext, useContext, useEffect, useMemo, useState, type ReactNode } from "react";
import { convertDisplayQuantities, type DisplayQuantityRequest, type DisplayQuantityResult } from "../../services/displayQuantityService";
import { DISPLAY_TARGETS, type DisplayUnitPreference } from "./targets";
import { describeResourceRefusal, ResourceUnavailableError } from "../../services/memoryPool";
export { DISPLAY_TARGETS, type DisplayUnitPreference } from "./targets";

export type DisplaySourceQuantity = Readonly<{ value: number | string; unit: string; dimension_id: string }>;
export type DisplayQuantity = { value: number | string; unit: string; status: "entered" | "pending" | "converted" | "unavailable"; notice?: string };
type Converter = typeof convertDisplayQuantities;

// Dedupe only outstanding work, so settled entries do not retain project data.
function makeBatcher(converter: Converter) {
  const outstanding = new Map<string, Promise<DisplayQuantityResult>>();
  let pending: { request: DisplayQuantityRequest; resolve: (result: DisplayQuantityResult) => void }[] = [];
  let counter = 0;
  return (input: Omit<DisplayQuantityRequest, "id">): Promise<DisplayQuantityResult> => {
    const key = JSON.stringify(input);
    const existing = outstanding.get(key);
    if (existing) return existing;
    const promise = new Promise<DisplayQuantityResult>((resolve) => {
      pending.push({ request: { ...input, id: `display-${++counter}` }, resolve });
      if (pending.length !== 1) return;
      queueMicrotask(async () => {
        const batch = pending;
        pending = [];
        let results: DisplayQuantityResult[];
        let unavailable = "Conversion service unavailable or returned an incomplete response.";
        try {
          results = await converter(batch.map((entry) => entry.request));
          if (!Array.isArray(results)) results = [];
        }
        catch (error) {
          results = [];
          if (error instanceof ResourceUnavailableError) unavailable = `conversion refused by the native memory pool: ${describeResourceRefusal(error.refusal)}`;
        }
        for (const { request, resolve: finish } of batch) {
          const matches = results.filter((result) => result && result.id === request.id);
          finish(matches.length === 1 ? matches[0] : { id: request.id, status: "unavailable", message: unavailable });
        }
      });
    });
    outstanding.set(key, promise);
    void promise.then(() => outstanding.delete(key));
    return promise;
  };
}
type DisplayContext = { preference: DisplayUnitPreference; setPreference: (value: DisplayUnitPreference) => void; convert: ReturnType<typeof makeBatcher> };
const Context = createContext<DisplayContext | null>(null);

export function DisplayUnitsProvider({ children, initialPreference = "entered", converter = convertDisplayQuantities }: {
  children: ReactNode; initialPreference?: DisplayUnitPreference; converter?: Converter;
}) {
  const [preference, setPreference] = useState(initialPreference);
  const convert = useMemo(() => makeBatcher(converter), [converter]);
  const context = useMemo(() => ({ preference, setPreference, convert }), [preference, convert]);
  return <Context.Provider value={context}>{children}</Context.Provider>;
}
export function useDisplayUnits() {
  const context = useContext(Context);
  return { preference: context?.preference ?? "entered", setPreference: context?.setPreference ?? (() => {}) };
}
const DISPLAY_UNITS_NOTE = "Readouts use the selected units where conversion is available. Unavailable quantities retain entered values. Editing uses entered units.";
export function DisplayUnitSelector({ id = "display-unit-preference", compact = false }: { id?: string; compact?: boolean }) {
  const { preference, setPreference } = useDisplayUnits();
  // The toolbar's form: the same select and the same behaviour, with the note as its tooltip.
  if (compact) return <select id={id} aria-label="Display units" title={`Display units. ${DISPLAY_UNITS_NOTE}`} value={preference} onChange={(event) => setPreference(event.target.value as DisplayUnitPreference)}>
    <option value="entered">Entered</option><option value="SI">SI</option><option value="US">US</option>
  </select>;
  return <div><label htmlFor={id}>Display units</label>{" "}<select id={id} value={preference} onChange={(event) => setPreference(event.target.value as DisplayUnitPreference)}>
    <option value="entered">Entered</option><option value="SI">SI</option><option value="US">US</option>
  </select><p>Readouts use the selected units where conversion is available. Unavailable quantities retain entered values. Editing uses entered units.</p></div>;
}
/** The catalog target of a quantity under a preference, or undefined (entered, or none). */
export function displayTarget(dimensionId: string, preference: DisplayUnitPreference): string | undefined {
  return preference === "entered" ? undefined : DISPLAY_TARGETS[dimensionId]?.[preference];
}

/** The readout of a settled conversion: the converter's value, or the entered value with the
 * reason (the hook's rule, shared with the CSV export so both read the same binary64). */
export function settledDisplayQuantity(quantity: DisplaySourceQuantity, preference: DisplayUnitPreference, result: DisplayQuantityResult | null): DisplayQuantity {
  const { value, unit, dimension_id } = quantity;
  if (preference === "entered") return { value, unit, status: "entered" };
  const target = displayTarget(dimension_id, preference);
  const fallback = (notice: string): DisplayQuantity => ({ value, unit, status: "unavailable", notice: `Entered value shown: ${notice}` });
  const valid = typeof value === "number" && Number.isFinite(value) && !!target;
  if (!valid) return fallback(!target ? `no ${preference} catalog target for ${dimension_id || "unknown dimension"}.` : "quantity is not a finite numeric value.");
  if (!result) return { value, unit, status: "pending", notice: "Conversion pending; entered value shown." };
  if (result.status !== "converted") return fallback(result.message ?? "conversion unavailable.");
  if (!Number.isFinite(result.value) || result.unit !== target) return fallback("invalid conversion response.");
  return { value: result.value, unit: result.unit, status: "converted" };
}

/** RT-R6: the session's display conversion as a function, for an export that must write the
 * same value a readout shows: the same preference, the same converter and the same request. */
export function useDisplayQuantityResolver(): {
  preference: DisplayUnitPreference;
  resolve: (quantity: DisplaySourceQuantity) => Promise<DisplayQuantity>;
  /** The converter's output for one stored unit and for zero, as it converts them. */
  convertRaw: (request: Omit<DisplayQuantityRequest, "id">) => Promise<DisplayQuantityResult | null>;
} {
  const context = useContext(Context);
  const preference = context?.preference ?? "entered";
  const convert = context?.convert;
  return useMemo(() => ({
    preference,
    convertRaw: async (request) => (convert ? convert(request) : null),
    resolve: async (quantity) => {
      const target = displayTarget(quantity.dimension_id, preference);
      const valid = typeof quantity.value === "number" && Number.isFinite(quantity.value) && !!target;
      if (preference === "entered" || !valid || !convert || !target) return settledDisplayQuantity(quantity, preference, null);
      const result = await convert({ value: quantity.value as number, from_unit: quantity.unit, to_unit: target, dimension_id: quantity.dimension_id });
      return settledDisplayQuantity(quantity, preference, result);
    }
  }), [preference, convert]);
}

export function useDisplayQuantity(quantity: DisplaySourceQuantity): DisplayQuantity {
  const context = useContext(Context);
  const preference = context?.preference ?? "entered";
  const { value, unit, dimension_id } = quantity;
  const target = preference === "entered" ? undefined : DISPLAY_TARGETS[dimension_id]?.[preference];
  const valid = typeof value === "number" && Number.isFinite(value) && !!target;
  const key = JSON.stringify([preference, value, unit, dimension_id, target]);
  const [resolved, setResolved] = useState<{ key: string; convert: DisplayContext["convert"]; result: DisplayQuantityResult }>();
  const convert = context?.convert;
  useEffect(() => {
    let active = true;
    if (preference !== "entered" && valid && convert && target && typeof value === "number") {
      void convert({ value, from_unit: unit, to_unit: target, dimension_id }).then((result) => {
        if (active) setResolved({ key, convert, result });
      });
    }
    return () => { active = false; };
  }, [key, preference, valid, convert, target, value, unit, dimension_id]);
  const settled = resolved && resolved.key === key && resolved.convert === convert ? resolved.result : null;
  return settledDisplayQuantity(quantity, preference, settled);
}
export function QuantityReadout({ quantity, className, label }: { quantity: DisplaySourceQuantity; className?: string; label?: string }) {
  const display = useDisplayQuantity(quantity);
  return <span className={className} aria-label={label} data-display-status={display.status}>
    {display.value} {display.unit}{display.notice && <small role="status"> ({display.notice})</small>}
  </span>;
}
