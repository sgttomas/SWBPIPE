// Identity normalization that agrees with the native side. Rust's `str::trim`
// strips exactly the Unicode White_Space characters at both ends; JavaScript's
// `String.prototype.trim` also strips U+FEFF, which is not White_Space. A
// webview adapter that must resolve the same identity as native code (e.g.
// `open_local_library`, which trims with Rust's rule) uses this, never `trim()`.
export function trimLikeRust(value: string): string {
  return value.replace(/^\p{White_Space}+|\p{White_Space}+$/gu, "");
}
