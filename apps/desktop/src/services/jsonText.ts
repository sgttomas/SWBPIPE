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

/** `jsonTextKeepingNegativeZero`'s text, handed to `put` in pieces in order instead of being
 * built, so a caller can digest it (`equalityDigest.ts`) without holding it whole. Joined,
 * the pieces are exactly that function's text, including its forms for values JSON has no
 * text for: an array element with none (`undefined`, a function, a symbol, a hole) writes
 * nothing, an object member whose value has none other than `undefined` writes `undefined`.
 * Returns false, having put nothing, where that function returns `undefined`; it throws
 * where that function throws. */
export function writeJsonTextKeepingNegativeZero(value: unknown, put: (piece: string) => void): boolean {
  if (typeof value === "number") { put(Object.is(value, -0) ? "-0" : JSON.stringify(value)); return true; }
  if (Array.isArray(value)) {
    put("[");
    const length = value.length;
    for (let index = 0; index < length; index += 1) {
      if (index > 0) put(",");
      // `map` skips a hole and `join` writes it, like an element with no text, as nothing.
      if (index in value) writeJsonTextKeepingNegativeZero(value[index], put);
    }
    put("]");
    return true;
  }
  if (value !== null && typeof value === "object") {
    put("{");
    let first = true;
    for (const [key, item] of Object.entries(value)) {
      if (item === undefined) continue;
      if (!first) put(",");
      first = false;
      put(JSON.stringify(key));
      put(":");
      if (!writeJsonTextKeepingNegativeZero(item, put)) put("undefined");
    }
    put("}");
    return true;
  }
  const text = JSON.stringify(value) as string | undefined;
  if (text === undefined) return false;
  put(text);
  return true;
}
