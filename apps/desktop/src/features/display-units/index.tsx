import { createContext, useContext, useEffect, useMemo, useState, type ReactNode } from "react";
import { convertDisplayQuantities, type DisplayQuantityRequest, type DisplayQuantityResult } from "../../services/displayQuantityService";
import { DISPLAY_TARGETS, type DisplayUnitPreference } from "./targets";
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
        try {
          results = await converter(batch.map((entry) => entry.request));
          if (!Array.isArray(results)) results = [];
        }
        catch { results = []; }
        for (const { request, resolve: finish } of batch) {
          const matches = results.filter((result) => result && result.id === request.id);
          finish(matches.length === 1 ? matches[0] : { id: request.id, status: "unavailable", message: "Conversion service unavailable or returned an incomplete response." });
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
export function DisplayUnitSelector({ id = "display-unit-preference" }: { id?: string }) {
  const { preference, setPreference } = useDisplayUnits();
  return <div><label htmlFor={id}>Display units</label>{" "}<select id={id} value={preference} onChange={(event) => setPreference(event.target.value as DisplayUnitPreference)}>
    <option value="entered">Entered</option><option value="SI">SI</option><option value="US">US</option>
  </select><p>Readouts use the selected units where conversion is available. Unavailable quantities retain entered values. Editing uses entered units.</p></div>;
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
  if (preference === "entered") return { value, unit, status: "entered" };
  const fallback = (notice: string): DisplayQuantity => ({ value, unit, status: "unavailable", notice: `Entered value shown: ${notice}` });
  if (!valid) return fallback(!target ? `no ${preference} catalog target for ${dimension_id || "unknown dimension"}.` : "quantity is not a finite numeric value.");
  if (!resolved || resolved.key !== key || resolved.convert !== convert) return { value, unit, status: "pending", notice: "Conversion pending; entered value shown." };
  const result = resolved.result;
  if (result.status !== "converted") return fallback(result.message ?? "conversion unavailable.");
  if (!Number.isFinite(result.value) || result.unit !== target) return fallback("invalid conversion response.");
  return { value: result.value, unit: result.unit, status: "converted" };
}
export function QuantityReadout({ quantity, className, label }: { quantity: DisplaySourceQuantity; className?: string; label?: string }) {
  const display = useDisplayQuantity(quantity);
  return <span className={className} aria-label={label} data-display-status={display.status}>
    {display.value} {display.unit}{display.notice && <small role="status"> ({display.notice})</small>}
  </span>;
}
