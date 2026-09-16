/**
 * Parse bounded Chromium ReturnAsStream JSON without first passing unsafe
 * integer tokens through JavaScript Number. Trace timestamps remain numeric
 * only when their JSON representation is safely representable.
 */
export function parseTraceEventsWithUnsafeIntegersAsDecimalStrings(raw) {
  const outputSpans = [];
  let unchangedStart = 0;
  let convertedCount = 0;
  let index = 0;
  let inString = false;
  let escaped = false;
  const numberPattern = /-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?/y;
  while (index < raw.length) {
    const char = raw[index];
    if (inString) {
      if (escaped) escaped = false;
      else if (char === "\\") escaped = true;
      else if (char === '"') inString = false;
      index += 1;
      continue;
    }
    if (char === '"') {
      inString = true;
      index += 1;
      continue;
    }
    if (char === "-" || (char >= "0" && char <= "9")) {
      numberPattern.lastIndex = index;
      const match = numberPattern.exec(raw);
      if (!match) throw new Error(`invalid JSON number token at raw trace offset ${index}`);
      const token = match[0];
      if (/^-?(?:0|[1-9]\d*)$/.test(token)) {
        if (BigInt(token) > BigInt(Number.MAX_SAFE_INTEGER) || BigInt(token) < BigInt(Number.MIN_SAFE_INTEGER)) {
          outputSpans.push(raw.slice(unchangedStart, index), JSON.stringify(token));
          unchangedStart = index + token.length;
          convertedCount += 1;
        }
      } else {
        const numeric = Number(token);
        if (!Number.isFinite(numeric) || Math.abs(numeric) > Number.MAX_SAFE_INTEGER) {
          throw new Error(`non-integer trace number cannot be represented safely at raw trace offset ${index}: ${token}`);
        }
      }
      index += token.length;
      continue;
    }
    index += 1;
  }
  outputSpans.push(raw.slice(unchangedStart));
  const output = outputSpans.length === 1 ? outputSpans[0] : outputSpans.join("");
  const parsed = JSON.parse(output);
  if (!parsed || !Array.isArray(parsed.traceEvents)) throw new Error("ReturnAsStream trace JSON lacks traceEvents array");
  return { events: parsed.traceEvents, convertedCount };
}
