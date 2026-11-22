/** Issue 588: the text and digests a results view's identity is read from. View state only: never a
 * provenance, model, result or envelope hash (those keep their checked canonical forms), and never
 * evidence of anything.
 *
 * The text distinguishes what a reader of the result distinguishes and nothing an ordering choice
 * makes: object members in sorted key order (an insertion-order difference is the same result), a
 * negative zero written `-0` (the retained-precision reader tells -0 from 0), non-finite numbers by
 * name, and members whose value is `undefined` left out. */
export function viewIdentityText(value: unknown): string {
  if (typeof value === "number") return Object.is(value, -0) ? "-0" : Number.isFinite(value) ? JSON.stringify(value) : String(value);
  if (typeof value === "bigint") return `${value}n`;
  if (Array.isArray(value)) return `[${value.map((item) => (item === undefined ? "null" : viewIdentityText(item))).join(",")}]`;
  if (value !== null && typeof value === "object") {
    const members = Object.keys(value).sort().filter((key) => (value as Record<string, unknown>)[key] !== undefined);
    return `{${members.map((key) => `${JSON.stringify(key)}:${viewIdentityText((value as Record<string, unknown>)[key])}`).join(",")}}`;
  }
  return value === undefined ? "null" : JSON.stringify(value);
}

/** A synchronous content fingerprint of a view's text: its length and two 53-bit string hashes
 * (cyrb53 with two seeds). Transient view state only. */
export function viewFingerprint(text: string): string {
  const hash = (seed: number) => {
    let h1 = 0xdeadbeef ^ seed, h2 = 0x41c6ce57 ^ seed;
    for (let i = 0; i < text.length; i += 1) {
      const ch = text.charCodeAt(i);
      h1 = Math.imul(h1 ^ ch, 2654435761);
      h2 = Math.imul(h2 ^ ch, 1597334677);
    }
    h1 = Math.imul(h1 ^ (h1 >>> 16), 2246822507) ^ Math.imul(h2 ^ (h2 >>> 13), 3266489909);
    h2 = Math.imul(h2 ^ (h2 >>> 16), 2246822507) ^ Math.imul(h1 ^ (h1 >>> 13), 3266489909);
    return (4294967296 * (2097151 & h2) + (h1 >>> 0)).toString(36);
  };
  return `${text.length.toString(36)}.${hash(0)}.${hash(0x9e3779b9)}`;
}

/** The SHA-256 (hex) of a value's view text, or null when it cannot be read or digested. */
export async function viewSha256Hex(value: unknown): Promise<string | null> {
  try {
    const digest = await globalThis.crypto.subtle.digest("SHA-256", new TextEncoder().encode(viewIdentityText(value)));
    return [...new Uint8Array(digest)].map((byte) => byte.toString(16).padStart(2, "0")).join("");
  } catch {
    return null;
  }
}
