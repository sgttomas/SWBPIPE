/** A value in a results table cell or summary (DEL-07-05 REQ-003): converted to the display unit the
 * user chose by the session's display converter, then printed by the report's display rule's
 * rounding step (six significant digits, half away from zero; `reportQuantityDisplay.ts`), so the
 * same row reads the same in a table and in the report. The value in its displayed unit at full
 * precision is the readout's hover text and stays in the row's detail; the stored value is never
 * changed and every CSV writes it as before. */
import { useDisplayQuantity, type DisplaySourceQuantity } from "../display-units";
import { displayNumber } from "../report/reportQuantityDisplay";

/** The display rule's rounding step on a number in its displayed unit. */
export function engineeringNumberText(value: number): string {
  if (!Number.isFinite(value)) return String(value);
  return displayNumber(Object.is(value, -0) ? "-0" : String(value), 0);
}

export function EngineeringQuantity({ quantity, testId, showUnit = true }: { quantity: DisplaySourceQuantity; testId?: string; showUnit?: boolean }) {
  const display = useDisplayQuantity(quantity);
  const number = typeof display.value === "number" ? engineeringNumberText(display.value) : String(display.value);
  return (
    <span className="engineering-quantity" data-display-status={display.status} data-testid={testId} title={`${display.value} ${display.unit}`}>
      {number}{showUnit ? ` ${display.unit}` : null}{display.notice ? <small role="status"> ({display.notice})</small> : null}
    </span>
  );
}
