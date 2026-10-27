/** b7-plan R2: the shared base corpus's statements, as result_export's
 * `tests/base_reader_outcomes.rs` (`corpus()`) builds them, for the desktop's
 * equivalence gate (`baseReaderWasmEquivalence.test.ts`).
 *
 * Every statement is built and written as raw JSON so negative zero and unsafe
 * integers keep their source values. The Rust readers compare numbers by value;
 * value-preserving numeric re-serialization gives the same verdict (A2). The
 * text parses to the `Value` the native test read. An
 * unmutated statement is the corpus file's own text. Test input only; this
 * module reads nothing about a statement.
 *
 * B7b: the corpus is built from the current fixtures only, as the Rust test builds it.
 * The exact-route statements are the v3 twins (0.8.0/3.0.0/exact_pressure_v3) read by the
 * pressure-2 and pressure-source-1 readers, with and without the load/reference-state
 * group; the joined statements take their invocation; the mutation files' `ps` reader
 * is folded into `joined`; the ledger cases apply to the received joined statements
 * directly (the v2 physics-source-1 projection is gone); `reseal` hashes the current
 * case-evidence domain with the record bound by its case id. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { loadWasmEngine } from "../services/wasmEngine/loadWasmEngine";

const PROJECT_ROOT = resolve(__dirname, "../../../../");
const MARKER = "__BASE_READER_TEXT_LITERAL__";

/** A JSON number, kept as its text. */
export class RawNumber { constructor(readonly text: string) {} }
export type Raw = null | boolean | string | RawNumber | Raw[] | RawObject;
export type RawObject = { [key: string]: Raw };

/** Strict JSON text to a raw value (numbers kept as text, object keys in order). */
export function parseRaw(text: string): Raw {
  let i = 0;
  const ws = () => { while (i < text.length && " \t\n\r".includes(text[i])) i++; };
  const fail = (what: string): never => { throw new Error(`raw JSON: ${what} at ${i}`); };
  const value = (): Raw => {
    ws();
    const c = text[i];
    if (c === "{") {
      i++; const object: RawObject = Object.create(null) as RawObject; ws();
      if (text[i] === "}") { i++; return object; }
      for (;;) {
        ws(); if (text[i] !== "\"") fail("key");
        const key = string(); ws(); if (text[i++] !== ":") fail("colon");
        object[key] = value(); ws();
        if (text[i] === ",") { i++; continue; }
        if (text[i++] === "}") return object;
        fail("object");
      }
    }
    if (c === "[") {
      i++; const array: Raw[] = []; ws();
      if (text[i] === "]") { i++; return array; }
      for (;;) {
        array.push(value()); ws();
        if (text[i] === ",") { i++; continue; }
        if (text[i++] === "]") return array;
        fail("array");
      }
    }
    if (c === "\"") return string();
    if (text.startsWith("true", i)) { i += 4; return true; }
    if (text.startsWith("false", i)) { i += 5; return false; }
    if (text.startsWith("null", i)) { i += 4; return null; }
    const number = /-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?/y;
    number.lastIndex = i;
    const match = number.exec(text);
    if (!match) return fail("value");
    i += match[0].length;
    return new RawNumber(match[0]);
  };
  const string = (): string => {
    const start = i; i++;
    while (i < text.length && text[i] !== "\"") i += text[i] === "\\" ? 2 : 1;
    i++;
    return JSON.parse(text.slice(start, i)) as string;
  };
  const result = value(); ws();
  if (i !== text.length) fail("trailing text");
  return result;
}
const isObject = (v: Raw | undefined): v is RawObject => !!v && typeof v === "object" && !Array.isArray(v) && !(v instanceof RawNumber);
/** A raw value's JSON text: strings as `JSON.stringify` writes them, numbers as read. */
export function rawText(v: Raw): string {
  if (v instanceof RawNumber) return v.text;
  if (Array.isArray(v)) return `[${v.map(rawText).join(",")}]`;
  if (isObject(v)) return `{${Object.keys(v).map(k => `${JSON.stringify(k)}:${rawText(v[k])}`).join(",")}}`;
  return JSON.stringify(v);
}
function clone(v: Raw): Raw {
  if (Array.isArray(v)) return v.map(clone);
  if (isObject(v)) { const o: RawObject = Object.create(null) as RawObject; for (const k of Object.keys(v)) o[k] = clone(v[k]); return o; }
  return v;
}
/** serde_json `Value` equality: a number by its class (integer or float) and value. */
function numberKey(n: RawNumber): string {
  if (/[.eE]/.test(n.text)) return `f:${Number(n.text)}`;
  const b = BigInt(n.text);
  if (b === 0n && n.text.startsWith("-")) return "f:0";
  return b >= -(2n ** 63n) && b < 2n ** 64n ? `i:${b}` : `f:${Number(n.text)}`;
}
function rawEqual(a: Raw | undefined, b: Raw | undefined): boolean {
  if (a instanceof RawNumber || b instanceof RawNumber) return a instanceof RawNumber && b instanceof RawNumber && numberKey(a) === numberKey(b);
  if (Array.isArray(a) || Array.isArray(b)) return Array.isArray(a) && Array.isArray(b) && a.length === b.length && a.every((x, k) => rawEqual(x, b[k]));
  if (isObject(a) || isObject(b)) {
    if (!isObject(a) || !isObject(b)) return false;
    const ka = Object.keys(a), kb = Object.keys(b);
    return ka.length === kb.length && ka.every(k => Object.hasOwn(b, k) && rawEqual(a[k], b[k]));
  }
  return a === b;
}
const str = (s: string): Raw => s;
const obj = (entries: [string, Raw][]): RawObject => { const o: RawObject = Object.create(null) as RawObject; for (const [k, v] of entries) o[k] = v; return o; };
/** serde_json `Index`: a member of an object or an element of an array, else null. */
function index(v: Raw | undefined, key: string | number): Raw {
  if (typeof key === "number") return Array.isArray(v) ? v[key] ?? null : null;
  return isObject(v) && Object.hasOwn(v, key) ? v[key] : null;
}

// RFC 6901, as the Rust test's helpers.
const tokens = (path: string) => path.slice(1).split("/").map(t => t.replaceAll("~1", "/").replaceAll("~0", "~"));
function pointer(doc: Raw, path: string): Raw {
  if (path === "") return doc;
  let v: Raw = doc;
  for (const t of tokens(path)) {
    if (Array.isArray(v)) { if (!/^(0|[1-9]\d*)$/.test(t) || Number(t) >= v.length) throw new Error(`missing ${path}`); v = v[Number(t)]; }
    else if (isObject(v) && Object.hasOwn(v, t)) v = v[t];
    else throw new Error(`missing ${path}`);
  }
  return v;
}
function parentOf(doc: Raw, path: string): [Raw, string] {
  const t = tokens(path);
  const parent = "/" + t.slice(0, -1).map(x => x.replaceAll("~", "~0").replaceAll("/", "~1")).join("/");
  return [pointer(doc, parent === "/" ? "" : parent), t[t.length - 1]];
}
function set(doc: Raw, path: string, value: Raw): void {
  const [parent, last] = parentOf(doc, path);
  if (Array.isArray(parent)) { const k = Number(last); if (!(k < parent.length)) throw new Error(`index ${path}`); parent[k] = value; }
  else if (isObject(parent)) parent[last] = value;
  else throw new Error(`scalar parent of ${path}`);
}
const matches = (item: Raw, pattern: Raw) => isObject(pattern) && Object.keys(pattern).every(k => isObject(item) && Object.hasOwn(item, k) && rawEqual(item[k], pattern[k]));

type Hash = (domain: string, payload: Raw) => string;
/** The mutation files' `reseal` op (B7b: the current case-evidence domain, the record
 * bound by its case id, as the reader binds it). */
function reseal(doc: RawObject, op: RawObject, hash: Hash): void {
  const evidence = clone(doc.contract_evidence);
  const body = index(doc.source_block_recovery, "body");
  if (op.physical !== null && op.physical !== undefined) {
    const exact = index(evidence, "exact_cases") as Raw[];
    (index(body, "cases") as Raw[]).forEach((c, k) => {
      const caseId = index(exact[k] ?? null, "load_case_id");
      const pressure = (index(evidence, "pressure") as Raw[]).filter(p => rawEqual(index(p, "load_case_id"), caseId));
      const records = index(evidence, "load_reference_states");
      const record = Array.isArray(records) ? records.find(r => rawEqual(index(r, "load_case_id"), caseId)) ?? null : null;
      const [domain, payload] = op.physical === "joined"
        ? ["pressure_source_case_evidence_v1", obj([["exact_case", exact[k] ?? null], ["pressure", pressure], ["load_reference_state", record]])]
        : op.physical === "joined_without_record"
          ? ["pressure_source_case_evidence_v1", obj([["exact_case", exact[k] ?? null], ["pressure", pressure]])]
          : op.physical === "physics"
            ? ["physics_source_case_evidence_v1", obj([["exact_case", exact[k] ?? null], ["pressure", pressure]])]
            : (() => { throw new Error(`unknown reseal ${String(op.physical)}`); })();
      (c as RawObject).physical_evidence_sha256 = hash(domain, payload);
    });
  }
  if (op.publication === true) {
    const publication = clone(doc) as RawObject; delete publication.source_block_recovery;
    (body as RawObject).publication_sha256 = hash("source_blocks_publication_v1", publication);
  }
  if (op.receipt === true) (doc.source_block_recovery as RawObject).receipt_sha256 = hash("source_blocks_receipt_v1", index(doc.source_block_recovery, "body"));
}
/** The Rust test's `apply`: the mutated document, and for a `nonfinite` or
 * `json_text` op the literal that replaces the marker in its text. */
function apply(base: Raw, ops: Raw[], hash: Hash, sources: (name: string) => Raw): [Raw, string | null] {
  const doc = clone(base);
  let literal: string | null = null;
  for (const op of ops as RawObject[]) {
    const path = typeof op.path === "string" ? op.path : "";
    const value = () => clone(op.value ?? null);
    switch (op.op) {
      case "set": case "replace": set(doc, path, value()); break;
      case "remove": {
        const [parent, last] = parentOf(doc, path);
        if (Array.isArray(parent)) parent.splice(Number(last), 1);
        else if (isObject(parent)) { if (!Object.hasOwn(parent, last)) throw new Error(`remove ${path}`); delete parent[last]; }
        else throw new Error("scalar parent");
        break;
      }
      case "append": (pointer(doc, path) as Raw[]).push(value()); break;
      case "add": {
        const [parent, last] = parentOf(doc, path);
        if (isObject(parent)) parent[last] = value();
        else if (Array.isArray(parent) && last === "-") parent.push(value());
        else if (Array.isArray(parent)) parent.splice(Number(last), 0, value());
        else throw new Error("scalar parent");
        break;
      }
      case "reverse": (pointer(doc, path) as Raw[]).reverse(); break;
      case "copy_append": (pointer(doc, path) as Raw[]).push(clone(pointer(doc, op.from as string))); break;
      case "swap": {
        const other = op.with as string, a = clone(pointer(doc, path)), b = clone(pointer(doc, other));
        set(doc, path, b); set(doc, other, a); break;
      }
      case "graft": set(doc, path, clone(pointer(sources(op.source as string), op.from as string))); break;
      case "remove_where": {
        const array = pointer(doc, path) as Raw[];
        array.splice(0, array.length, ...array.filter(item => !matches(item, op.match)));
        break;
      }
      case "set_where":
        for (const item of pointer(doc, path) as Raw[]) if (matches(item, op.match)) (item as RawObject)[op.key as string] = value();
        break;
      case "append_copy_where": {
        const array = pointer(doc, path) as Raw[];
        const found = array.find(item => matches(item, op.match));
        if (found === undefined) throw new Error(`append_copy_where ${path}`);
        const copy = clone(found) as RawObject;
        for (const [k, v] of Object.entries(op.set as RawObject)) copy[k] = clone(v);
        array.push(copy);
        break;
      }
      case "nonfinite":
        set(doc, path, str(MARKER));
        literal = ({ Infinity: "1e400", "-Infinity": "-1e400", NaN: "NaN" } as Record<string, string>)[op.value as string];
        if (literal === undefined) throw new Error(`unknown non-finite ${String(op.value)}`);
        break;
      case "json_text": set(doc, path, str(MARKER)); literal = op.value as string; break;
      case "reseal": reseal(doc as RawObject, op, hash); break;
      default: throw new Error(`unknown op ${String(op.op)}`);
    }
  }
  return [doc, literal];
}

/** One corpus statement as the Rust test reads it: its texts, and what is read. */
export type BaseReaderInput = {
  id: string;
  /** The statement's text; when `parsed` is false, a text no serde Value holds. */
  sourceJson: string;
  parsed: boolean;
  modelJson?: string;
  invocationJson?: string;
  readers: string[];
  transports: string[];
  /** Each row's text, for the unmutated statements whose rows are read. */
  rowsJson: string[] | null;
};

const ROUTE_READER: Record<string, string> = {
  preview_physics: "preview-physics", pressure: "pressure", pressure_source: "pressure-source",
};

/** The corpus inputs, in the native outcomes' order. A base statement's reader
 * comes from its route, which the Rust test takes from `reader_verdict::route`:
 * here the wasm dispatch of the same text, whose agreement with the native
 * outcome the gate checks separately. */
export async function baseReaderInputs(): Promise<BaseReaderInput[]> {
  const engine = await loadWasmEngine();
  const hash: Hash = (domain, payload) => engine.canonicalSha256HexCheckedV1(`{"domain":${JSON.stringify(domain)},"payload":${rawText(payload)}}`);
  const fileText = (path: string) => readFileSync(resolve(PROJECT_ROOT, path), "utf8");
  const read = (path: string) => parseRaw(fileText(path));
  const modelAt = (path: string) => { const v = read(path); return isObject(v) && Object.hasOwn(v, "model") ? v.model : v; };
  const inputs: BaseReaderInput[] = [];
  const base = (id: string, sourceJson: string, model?: Raw, invocation?: Raw) => {
    const source = parseRaw(sourceJson);
    const route = (JSON.parse(engine.readerDispatchJson(sourceJson)) as { ok: boolean; route: string });
    const reader = route.ok ? ROUTE_READER[route.route] : undefined;
    const rows = index(source, "results");
    inputs.push({
      id, sourceJson, parsed: true,
      modelJson: model === undefined ? undefined : rawText(model),
      invocationJson: invocation === undefined ? undefined : rawText(invocation),
      readers: reader ? [reader] : [], transports: reader ? [reader] : [],
      rowsJson: Array.isArray(rows) ? rows.map(rawText) : [],
    });
  };
  const results: [string, string | null][] = [
    ["preview_physics_connected_sparse", "fixtures/model_operations/precision_connected_ui_model.json"],
    ["preview_physics_connected_dense", "fixtures/model_operations/precision_connected_ui_model.json"],
    ["preview_physics_invented_sparse", "core/product_physics/tests/fixtures/preview_physics_invented_model.json"],
    ["preview_physics_invented_dense", "core/product_physics/tests/fixtures/preview_physics_invented_model.json"],
    ["preview_physics_unicode_ids_sparse", "core/product_physics/tests/fixtures/preview_physics_unicode_ids_model.json"],
    ["pressure_connected_mechanics_sparse", "core/product_physics/tests/fixtures/exact_pressure_connected_request.json"],
    ["pressure_connected_mechanics_dense", "core/product_physics/tests/fixtures/exact_pressure_connected_request.json"],
    ["pressure_connected_ui_mechanics_sparse", "fixtures/model_operations/exact_pressure_authoring_model.json"],
    ["pressure_connected_ui_mechanics_dense", "fixtures/model_operations/exact_pressure_authoring_model.json"],
    ["pressure_thermal_ui_mechanics_sparse", "fixtures/model_operations/physics_thermal_ui_model.json"],
    ["pressure_thermal_ui_mechanics_dense", "fixtures/model_operations/physics_thermal_ui_model.json"],
    // T4-U4b review item 5: a directional pressure-2 arc and a preview arc envelope.
    ["pressure_u4b_arc_sparse", "core/product_physics/tests/fixtures/u4b_l90_p_model.json"],
    ["preview_physics_u4b_arc_sparse", "core/product_physics/tests/fixtures/u4b_z_preview_rev_model.json"],
    ["../product_preview/invented_mechanics_result_preview_physics_1_sparse", null],
    ["../product_preview/invented_mechanics_result_preview_physics_1_dense", null],
    ["../product_preview/invented_demo_result_preview_physics_1_sparse", null],
    ["../product_preview/invented_demo_result_preview_physics_1_dense", null],
  ];
  for (const [name, model] of results) {
    base(`results/${name.replace(/^(\.\.\/)+/, "")}`, fileText(`fixtures/results/${name}.json`), model === null ? undefined : modelAt(model));
  }
  const modes = ["sparse_interactive", "dense_scrutiny"];
  const invocationOf = (request: Raw, mode: string) => obj([["request", request], ["solver_mode", mode]]);
  for (const [dir, names, invoked] of [
    ["pressure_source/physics_source", ["n05", "n06", "fields", "mixed", "mixed_units", "n05_units", "n05_unicode"], true],
    ["pressure/load_reference", ["connected", "pressure"], false],
    // The joined load/reference-state statements are read by the one source reader,
    // which takes their invocation (never numerically eligible).
    ["pressure_source/load_reference_source", ["n05", "n06", "fields", "mixed", "eigen_motion"], true],
  ] as const) {
    for (const name of names) {
      const request = read(`fixtures/product_preview/${dir}/${name}.request.json`);
      for (const mode of modes) {
        base(`${dir}/${name}-${mode}`, fileText(`fixtures/product_preview/${dir}/${name}-${mode}.raw.json`), index(request, "model"),
          invoked ? invocationOf(request, mode) : undefined);
      }
    }
  }
  // B4/D1 (b): the preview route's retired source-blocks-1 statements left the corpus.
  const fallback = modelAt("core/reporting/result_export/tests/fixtures/load_reference_fallback_uz.request.json");
  for (const mode of modes) {
    base(`load_reference_fallback_uz-${mode}`, fileText(`core/reporting/result_export/tests/fixtures/load_reference_fallback_uz-${mode}.raw.json`), fallback);
  }
  // W-16 slice b: the shared pressure-2 combination corpus, each envelope with its model.
  for (const c of index(read("fixtures/results/pressure_v3_combination_reader_corpus.json"), "cases") as RawObject[]) {
    base(`combination/${c.label as string}`, rawText(c.envelope), c.model);
  }
  const statement = (id: string, doc: Raw, literal: string | null, readers: string[], transports: string[] = []): BaseReaderInput => {
    if (literal === null) return { id, sourceJson: rawText(doc), parsed: true, readers, transports, rowsJson: null };
    // A text serde_json refuses (the gate checks the engine refuses it).
    const text = rawText(doc).replace(`"${MARKER}"`, literal);
    return { id, sourceJson: text, parsed: false, readers, transports, rowsJson: null };
  };
  // The shared tamper vector (preview-physics).
  const vector = read("fixtures/results/preview_physics_tamper_vector.json") as RawObject;
  for (const variant of vector.variants as RawObject[]) {
    const [doc, literal] = apply(read(index(vector.bases, variant.base as string) as string), variant.ops as Raw[], hash, () => { throw new Error("graft"); });
    if (literal !== null) throw new Error("tamper literal");
    // Codex round 2: a variant with a transport expectation is also read by transport.
    inputs.push(statement(`tamper/${variant.id as string}`, doc, null, ["preview-physics"], "transport" in variant ? ["preview-physics"] : []));
  }
  // load_reference*_mutations.json.
  for (const [file, reader, joined] of [
    ["core/reporting/result_export/tests/fixtures/load_reference_mutations.json", "pressure", false],
    ["core/reporting/result_export/tests/fixtures/load_reference_source_mutations.json", "pressure-source", true],
  ] as const) {
    const cases = read(file) as RawObject;
    const sources = new Map(Object.entries(cases.sources as RawObject).map(([k, v]) => [k, read(v as string)]));
    const lookup = (name: string) => sources.get(name)!;
    const prefix = joined ? "lrs" : "lr";
    for (const c of cases.cases as RawObject[]) {
      const [doc, literal] = apply(lookup(c.source as string), c.ops as Raw[], hash, lookup);
      const readers: string[] = [reader];
      if (joined && Object.hasOwn(c, "lr")) readers.push("pressure");
      if (Object.hasOwn(c, "ps")) throw new Error("B7b folded 'ps' into 'joined'");
      inputs.push(statement(`${prefix}/${c.id as string}`, doc, literal, readers));
    }
    for (const c of cases.transport_cases as RawObject[]) {
      const [doc, literal] = apply(lookup(c.source as string), c.ops as Raw[], hash, lookup);
      if (literal !== null) throw new Error("transport literal");
      const transported: RawObject = obj([["schema_version", index(doc, "schema_version")]]);
      for (const key of ["producer", "numerical_quality", "formulation_basis", "contract_evidence", "source_block_recovery", "carrier_evidence"]) {
        if (isObject(doc) && Object.hasOwn(doc, key)) transported[key] = doc[key];
      }
      inputs.push(statement(`${prefix}-transport/${c.id as string}`, transported, null, [], [reader]));
    }
  }
  // The two case files the TypeScript tests authored, now shared.
  for (const c of read("fixtures/results/loadReferenceReaderCases.cases.json") as RawObject[]) {
    const [doc, literal] = apply(read(c.source as string), c.ops as Raw[], hash, () => { throw new Error("graft"); });
    if (literal !== null) throw new Error("reader-case literal");
    inputs.push(statement(`lr-reader/${c.id as string}`, doc, null, ["pressure"]));
  }
  // The ledger cases: the source reader's ledger checks on the received joined
  // statement (B7b: no projection; the ops read the same members).
  const received = new Map<string, Raw>();
  for (const c of read("fixtures/results/loadReferenceSourceLedger.cases.json") as RawObject[]) {
    const name = c.source as string;
    if (!received.has(name)) received.set(name, read(`fixtures/product_preview/pressure_source/load_reference_source/${name}.raw.json`));
    const [doc, literal] = apply(received.get(name)!, c.ops as Raw[], hash, () => { throw new Error("graft"); });
    if (literal !== null) throw new Error("ledger literal");
    if (typeof c.reseal_publication === "boolean") reseal(doc as RawObject, obj([["physical", null], ["publication", c.reseal_publication], ["receipt", true]]), hash);
    inputs.push(statement(`ledger/${c.id as string}`, doc, null, ["pressure-source"]));
  }
  return inputs;
}
