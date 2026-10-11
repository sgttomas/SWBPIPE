/** JSON text of a parsed value with every negative zero written as `-0`. `JSON.stringify`
 * writes -0 as 0, which reads as another statement where the sign is checked (b7-plan R1:
 * the Rust reader refuses a -0 in a receipt, and a native solve hands over the producer's
 * exact bytes). Otherwise it is `JSON.stringify`'s text: object members whose value is
 * `undefined` are left out. It checks nothing; callers check the value first when they
 * must. */
export function jsonTextKeepingNegativeZero(value: unknown): string {
  if (typeof value === "number") return Object.is(value, -0) ? "-0" : JSON.stringify(value);
  if (Array.isArray(value)) return `[${value.map(jsonTextKeepingNegativeZero).join(",")}]`;
  if (value !== null && typeof value === "object") {
    return `{${Object.entries(value).filter(([, item]) => item !== undefined).map(([key, item]) => `${JSON.stringify(key)}:${jsonTextKeepingNegativeZero(item)}`).join(",")}}`;
  }
  return JSON.stringify(value);
}
