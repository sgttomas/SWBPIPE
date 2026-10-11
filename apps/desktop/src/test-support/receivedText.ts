/** Test input: a result as the desktop receives it, read from its JSON text with the
 * text kept beside it (`rememberReceivedText`), as previewService keeps a native solve's
 * bytes. The Rust readers then read the text, not a re-serialization (b7-plan R2). */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { rememberReceivedText } from "../features/results/resultReader";

const PROJECT_ROOT = resolve(__dirname, "../../../../");
type Json = any; // eslint-disable-line @typescript-eslint/no-explicit-any
/** A value read from this text, its text kept as received. */
export function receivedFromText<T = Json>(text: string): T {
  const value = JSON.parse(text);
  if (value && typeof value === "object") rememberReceivedText(value, text);
  return value as T;
}
/** A fresh value read from the project file at `path` (from the project root), its text kept as received. */
export function receivedFixture<T = Json>(path: string): T {
  return receivedFromText<T>(readFileSync(resolve(PROJECT_ROOT, path), "utf8"));
}
