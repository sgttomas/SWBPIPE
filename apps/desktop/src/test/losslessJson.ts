/** Test support: a number-preserving JSON writer for committed fixtures that embed
 * producer output (W-11 slice (b) witness requests). JavaScript numbers lose their
 * lexical form (`0.0` reads and writes as `0`), while the Rust corpora compare
 * `serde_json::Value`s, in which an integer and a float differ. `parseLossless`
 * keeps each number's lexeme; `writeLossless` writes a value as `JSON.stringify` does
 * (compact, same string escaping, same member order) except that, where a parallel
 * `hint` tree holds a lexeme for an equal number at the same place, it writes that
 * lexeme. A hint is only ever the original producer text of a subtree the value
 * copies, so the written subtree is the original's, number for number. Test code
 * only; no product path uses it. */

/** A number with its original lexeme. */
export class Lexeme {
  constructor(readonly text: string, readonly value: number) {}
}

/** The lossless tree of a JSON text: objects and arrays as parsed, numbers as `Lexeme`s. */
export function parseLossless(text: string): unknown {
  let at = 0;
  const space = () => { while (at < text.length && " \t\n\r".includes(text[at])) at += 1; };
  const fail = (what: string): never => { throw new Error(`LOSSLESS-JSON: ${what} at ${at}`); };
  const value = (): unknown => {
    space();
    const char = text[at];
    if (char === "{") {
      at += 1;
      const object: Record<string, unknown> = {};
      space();
      if (text[at] === "}") { at += 1; return object; }
      for (;;) {
        space();
        const key = string();
        space();
        if (text[at] !== ":") fail("expected ':'");
        at += 1;
        Object.defineProperty(object, key, { value: value(), enumerable: true, writable: true, configurable: true });
        space();
        if (text[at] === ",") { at += 1; continue; }
        if (text[at] === "}") { at += 1; return object; }
        fail("expected ',' or '}'");
      }
    }
    if (char === "[") {
      at += 1;
      const array: unknown[] = [];
      space();
      if (text[at] === "]") { at += 1; return array; }
      for (;;) {
        array.push(value());
        space();
        if (text[at] === ",") { at += 1; continue; }
        if (text[at] === "]") { at += 1; return array; }
        fail("expected ',' or ']'");
      }
    }
    if (char === "\"") return string();
    for (const [word, result] of [["true", true], ["false", false], ["null", null]] as const) {
      if (text.startsWith(word, at)) { at += word.length; return result; }
    }
    const match = /^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?/.exec(text.slice(at, at + 64));
    if (!match) fail("unexpected token");
    at += match![0].length;
    return new Lexeme(match![0], Number(match![0]));
  };
  const string = (): string => {
    if (text[at] !== "\"") fail("expected a string");
    let end = at + 1;
    while (end < text.length && text[end] !== "\"") end += text[end] === "\\" ? 2 : 1;
    const token = text.slice(at, end + 1);
    at = end + 1;
    return JSON.parse(token) as string;
  };
  const result = value();
  space();
  if (at !== text.length) fail("trailing text");
  return result;
}

/** The plain value of a lossless tree (numbers as JavaScript numbers). */
export function plainOf(tree: unknown): unknown {
  if (tree instanceof Lexeme) return tree.value;
  if (Array.isArray(tree)) return tree.map(plainOf);
  if (tree && typeof tree === "object") return Object.fromEntries(Object.entries(tree).map(([key, item]) => [key, plainOf(item)]));
  return tree;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value) && !(value instanceof Lexeme);
}

/** `JSON.stringify(value)`, except that a number equal to the hint's lexeme at the
 * same place is written as that lexeme. */
export function writeLossless(value: unknown, hint?: unknown): string {
  if (value === null || typeof value === "boolean" || typeof value === "string") return JSON.stringify(value);
  if (typeof value === "number") {
    return hint instanceof Lexeme && Object.is(hint.value, value) ? hint.text : JSON.stringify(value);
  }
  if (Array.isArray(value)) {
    return `[${value.map((item, index) => (item === undefined ? "null" : writeLossless(item, Array.isArray(hint) ? hint[index] : undefined))).join(",")}]`;
  }
  if (value && typeof value === "object") {
    const record = value as Record<string, unknown>;
    const members = Object.keys(record)
      .filter((key) => record[key] !== undefined && typeof record[key] !== "function")
      .map((key) => `${JSON.stringify(key)}:${writeLossless(record[key], isRecord(hint) && Object.hasOwn(hint, key) ? hint[key] : undefined)}`);
    return `{${members.join(",")}}`;
  }
  throw new Error(`LOSSLESS-JSON: unsupported value ${typeof value}`);
}

/** A hint for an object that copies some members of `original` (a lossless tree):
 * each member whose value equals the original's same-named member, whole. */
export function copiedMembersHint(value: Record<string, unknown>, original: unknown): Record<string, unknown> {
  const hint: Record<string, unknown> = {};
  if (!isRecord(original)) return hint;
  for (const key of Object.keys(value)) {
    if (Object.hasOwn(original, key) && JSON.stringify(plainOf(original[key])) === JSON.stringify(value[key])) hint[key] = original[key];
  }
  return hint;
}
