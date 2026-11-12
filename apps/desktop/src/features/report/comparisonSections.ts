// The DEL-08-06 comparison sections, as the reference assembler
// (`core/reporting/state_comparison_handoff_sections/engine.py` `_comparison_sections`
// and the set-level rules a comparison section touches) builds them from comparison
// records. The shared vector `fixtures/reports/invented/state_comparison_handoff_comparison_parity.json`
// is written by that engine (`tools/fixtures/state_comparison_handoff_comparison_parity.py`)
// and holds this code to it (DEL-08-06 AC-002).
//
// The reference orders and de-duplicates by its canonical text,
// `json.dumps(sort_keys=True, separators=(",", ":"), ensure_ascii=True)`; `pythonJson`
// writes that text for every string and every non-integral number, so an order here is
// the reference's order. Known limit of AC-002: two items of one sorted list that differ
// only in an integral number the reference holds as a float (JSON text `2.0` or `1e+16`)
// can order differently, because JavaScript holds only the number while the reference
// orders by how it is written. The product path cannot produce that form: the desktop
// writes every record, and an integral value it writes has no fraction, so the reference
// reads it as an integer.
import { codePointCompare } from "../comparison/stateComparison";

type JsonRecord = Record<string, unknown>;
type Diagnostic = JsonRecord;
type Ref = { object_type: string; ref: string };

export const PROFESSIONAL_BOUNDARY = {
  human_review_required: true,
  software_makes_compliance_claim: false,
  software_makes_certification_claim: false,
  software_makes_sealing_claim: false,
  software_makes_approval_claim: false,
  software_makes_authentication_claim: false,
  software_creates_professional_reliance_record: false,
  software_creates_external_validation_record: false
};

export const ENGINE_PROVENANCE = {
  source_name: "OpenPipeStress DEL-08-06 report-section assembler",
  source_location: "core/reporting/state_comparison_handoff_sections/engine.py",
  source_license: "project-governed",
  contributor: "OpenPipeStress Type 2 worker",
  contributor_attestation: "implementation-only-no-professional-claim",
  redistribution_status: "public_permissive",
  review_classification: "machine_checked",
  privacy_classification: "public_metadata"
};

/** DEL-08-06 AC-005: one named vocabulary per status-bearing member the software writes,
 * the reference's `STATUS_VOCABULARIES`. A value outside its member's vocabulary gives a
 * blocking diagnostic and is not carried; the value is carried nowhere. No member is matched
 * by substring. */
export const STATUS_VOCABULARIES: Readonly<Record<string, ReadonlySet<string>>> = {
  software_analysis_statuses: new Set(["MODEL_INCOMPLETE", "MECHANICS_SOLVED", "RULE_INPUTS_INCOMPLETE", "USER_RULE_CHECKED", "USER_RULE_FAILED", "HUMAN_REVIEW_REQUIRED"]),
  diagnostic_severities: new Set(["info", "warning", "error", "blocking"]),
  state_row_classifications: new Set(["added", "removed", "changed", "unchanged", "mapped_changed", "mapped_unchanged", "unresolved"]),
  match_bases: new Set(["stable_id", "explicit_mapping", "manual_mapping", "unmatched", "stable_result_identity"]),
  field_change_kinds: new Set(["added", "removed", "changed", "unit_changed"]),
  not_compared_reasons: new Set(["semantic_identity_differs", "left_run_needs_recompute", "right_run_needs_recompute", "family_not_compared", "units_differ", "value_not_numeric", "dimension_missing"]),
  unmatched_reasons: new Set(["no_row_with_this_result_id", "identity_differs", "duplicate_result_id", "result_id_missing"]),
  run_sides: new Set(["left", "right"]),
  run_side_kinds: new Set(["recorded_run", "current_session_run"]),
  standing_statuses: new Set(["integrity_checked", "needs_recompute"]),
  delta_classifications: new Set(["not_classified", "within_tolerance_profile", "exceeds_tolerance_profile", "not_tolerance_checked"]),
  delta_classification_bases: new Set([
    "no_tolerance_profile_supplied", "no_matching_tolerance_rule", "tolerance_rule_unit_not_normalized_unit",
    "caller_supplied_relative_absolute_tolerance_rule", "tolerance_value_not_numeric", "caller_supplied_tolerance_rule", "no_tolerance_profile"
  ]),
  mapping_statuses: new Set(["automatic_match", "manual_match", "unresolved_mapping", "unmatched_left", "unmatched_right", "ignored", "TBD"]),
  settings_delta_kinds: new Set(["added", "removed", "changed"]),
  diagnostic_changes: new Set(["added", "removed"]),
  supported_behavior_statuses: new Set(["unsupported", "approximate", "requires_human_review", "not_implemented", "TBD"])
};

/** The reference's `STATUS_MEMBERS` for the record kinds the desktop assembles from records. */
export const STATUS_MEMBERS: Readonly<Record<"model_state_comparison" | "analysis_run_comparison", ReadonlyArray<readonly [string, string]>>> = {
  model_state_comparison: [
    ["entities[].classification", "state_row_classifications"],
    ["entities[].match_basis", "match_bases"],
    ["entities[].changes[].change_kind", "field_change_kinds"],
    ["diagnostics[].severity", "diagnostic_severities"]
  ],
  analysis_run_comparison: [
    ["run_context.left.side_kind", "run_side_kinds"],
    ["run_context.right.side_kind", "run_side_kinds"],
    ["run_context.left.standing.status", "standing_statuses"],
    ["run_context.right.standing.status", "standing_statuses"],
    ["run_context.run_gate[]", "not_compared_reasons"],
    ["run_context.not_compared[].reasons[]", "not_compared_reasons"],
    ["run_context.unmatched[].reason", "unmatched_reasons"],
    ["run_context.unmatched[].side", "run_sides"],
    ["result_deltas[].classification", "delta_classifications"],
    ["result_deltas[].classification_basis", "delta_classification_bases"],
    ["result_deltas[].match_basis", "match_bases"],
    ["result_deltas[].mapping_status", "mapping_statuses"],
    ["settings_deltas[].delta_kind", "settings_delta_kinds"],
    ["diagnostics[].severity", "diagnostic_severities"],
    ["diagnostics[].change", "diagnostic_changes"]
  ]
};

/** Whose text each text-bearing member of a section is: the reference's `TEXT_ORIGINS`. */
export const TEXT_ORIGINS: Readonly<Record<string, Readonly<Record<string, string>>>> = {
  current_model_state: {
    authored_analysis_status: "document_author", diagnostics: "document_author", warnings: "document_author",
    source_provenance: "software", privacy_classification: "software", review_state: "software"
  },
  model_state_comparison: {
    changed_entities: "model_values", diagnostics: "comparison", settings: "comparison",
    source_provenance: "source_declaration", privacy_classification: "source_declaration", review_state: "source_declaration"
  },
  analysis_run_comparison: {
    largest_differences: "solver_results", settings_deltas: "solver_settings", diagnostics: "solver",
    run_context: "comparison", pairs_not_compared_listed: "comparison", rows_unmatched_listed: "comparison",
    source_provenance: "source_declaration", privacy_classification: "source_declaration", review_state: "source_declaration"
  }
};

const AUTHORITY_FLAG_NAMES = [
  "software_makes_compliance_claim",
  "software_makes_certification_claim",
  "software_makes_sealing_claim",
  "software_makes_approval_claim",
  "software_makes_authentication_claim",
  "software_creates_professional_reliance_record",
  "software_creates_external_validation_record"
];

const PRIVATE_PAYLOAD_KEYS = new Set([
  "payload", "content", "formula", "formulas", "private_payload", "protected_payload",
  "private_rule_payload", "private_library_payload", "secret", "secrets"
]);

const NUMERIC_VALUE_KEYS = [
  "magnitude", "left_magnitude", "right_magnitude", "left_normalized_magnitude",
  "right_normalized_magnitude", "normalized_delta", "absolute_normalized_delta"
];

const UNCHANGED_CLASSIFICATIONS = new Set(["unchanged", "mapped_unchanged"]);
const CHANGE_VALUE_KEYS = new Set(["left", "right", "left_value", "right_value", "value"]);

/* -------------------------------------------------------------------------- *
 * The reference's canonical text
 * -------------------------------------------------------------------------- */

/** Python's `repr` of a float, which `json.dumps` writes: the shortest round-trip digits,
 * fixed notation for decimal exponents -4 to 15, otherwise `d[.ddd]e±XX`. */
function pythonFloat(value: number): string {
  if (Number.isNaN(value)) return "NaN";
  if (!Number.isFinite(value)) return value > 0 ? "Infinity" : "-Infinity";
  const [mantissa, exponentText] = value.toExponential().split("e");
  const exponent = Number(exponentText);
  const negative = mantissa.startsWith("-");
  const digits = mantissa.replace("-", "").replace(".", "");
  const sign = negative ? "-" : "";
  if (exponent < -4 || exponent >= 16) {
    const body = digits.length > 1 ? `${digits[0]}.${digits.slice(1)}` : digits;
    const magnitude = String(Math.abs(exponent)).padStart(2, "0");
    return `${sign}${body}e${exponent < 0 ? "-" : "+"}${magnitude}`;
  }
  if (exponent < 0) return `${sign}0.${"0".repeat(-exponent - 1)}${digits}`;
  const whole = digits.slice(0, exponent + 1).padEnd(exponent + 1, "0");
  const fraction = digits.slice(exponent + 1);
  return `${sign}${whole}.${fraction || "0"}`;
}

/** A JSON number as the reference reads and writes it: JSON text without a fraction or an
 * exponent is an int (JavaScript writes an integral value below 1e21 so), anything else a float. */
function pythonNumber(value: number): string {
  if (Number.isInteger(value) && Math.abs(value) < 1e21) return Object.is(value, -0) ? "0" : String(value);
  return pythonFloat(value);
}

const SHORT_ESCAPES: Record<string, string> = { '"': '\\"', "\\": "\\\\", "\n": "\\n", "\r": "\\r", "\t": "\\t", "\b": "\\b", "\f": "\\f" };

/** A string as `json.dumps(ensure_ascii=True)` writes it: printable ASCII as it is, every other
 * UTF-16 code unit as `\uxxxx`. */
function pythonString(value: string): string {
  let out = '"';
  for (let index = 0; index < value.length; index += 1) {
    const character = value[index];
    const code = value.charCodeAt(index);
    if (SHORT_ESCAPES[character]) out += SHORT_ESCAPES[character];
    else if (code < 0x20 || code > 0x7e) out += `\\u${code.toString(16).padStart(4, "0")}`;
    else out += character;
  }
  return `${out}"`;
}

/** The reference's `canonical_json`. */
export function pythonJson(value: unknown): string {
  if (value === null || value === undefined) return "null";
  if (value === true) return "true";
  if (value === false) return "false";
  if (typeof value === "number") return pythonNumber(value);
  if (typeof value === "string") return pythonString(value);
  if (Array.isArray(value)) return `[${value.map(pythonJson).join(",")}]`;
  const record = value as JsonRecord;
  const keys = Object.keys(record).filter((key) => record[key] !== undefined).sort(codePointCompare);
  return `{${keys.map((key) => `${pythonString(key)}:${pythonJson(record[key])}`).join(",")}}`;
}

/** `sorted(items, key=canonical_json)`. */
export function pythonSorted<T>(items: T[]): T[] {
  return items
    .map((item) => ({ item, key: pythonJson(item) }))
    .sort((left, right) => codePointCompare(left.key, right.key))
    .map(({ item }) => item);
}

/* -------------------------------------------------------------------------- *
 * The reference's helpers
 * -------------------------------------------------------------------------- */

function isMapping(value: unknown): value is JsonRecord {
  return value !== null && typeof value === "object" && !Array.isArray(value);
}

function list(value: unknown): unknown[] {
  return Array.isArray(value) ? value : [];
}

function copy<T>(value: T): T {
  return value === undefined ? (null as T) : structuredClone(value);
}

function truthy(value: unknown): boolean {
  if (Array.isArray(value)) return value.length > 0;
  if (isMapping(value)) return Object.keys(value).length > 0;
  return Boolean(value);
}

/** Python's `str` of a JSON scalar. */
function pythonStr(value: unknown): string {
  if (typeof value === "string") return value;
  if (value === true) return "True";
  if (value === false) return "False";
  if (value === null || value === undefined) return "None";
  if (typeof value === "number") return pythonNumber(value);
  return pythonJson(value);
}

function isMissing(value: unknown): boolean {
  return value === null || value === undefined || value === "" || value === "TBD"
    || (Array.isArray(value) && value.length === 0) || (isMapping(value) && Object.keys(value).length === 0);
}

function strOr(value: unknown, fallback: string): string {
  return isMissing(value) ? fallback : pythonStr(value);
}

function ref(objectType: string, value: string): Ref {
  return { object_type: objectType, ref: value };
}

function affected(objectType: string, value: string): Ref {
  return { object_type: String(objectType), ref: String(value) };
}

function diagnostic(code: string, severity: string, diagnosticClass: string, message: string, remediation: string, affectedRefs: Ref[]): Diagnostic {
  return { code, severity, class: diagnosticClass, message, remediation, affected_refs: affectedRefs, provenance: copy(ENGINE_PROVENANCE) };
}

function* walkDicts(value: unknown, path = "$"): Generator<[string, JsonRecord]> {
  if (Array.isArray(value)) for (let index = 0; index < value.length; index += 1) yield* walkDicts(value[index], `${path}[${index}]`);
  else if (isMapping(value)) {
    yield [path, value];
    for (const [key, item] of Object.entries(value)) yield* walkDicts(item, `${path}.${key}`);
  }
}

function vocabularyDiagnostic(affectedRef: Ref, path: string, vocabulary: string): Diagnostic {
  if (vocabulary === "software_analysis_statuses") {
    return diagnostic(
      "SCH-ANALYSIS-STATUS-BLOCKED", "blocking", "PROFESSIONAL_BOUNDARY",
      "Source analysis status is outside the software's analysis-status vocabulary; it is not carried.",
      "State only the software's analysis statuses in the source record.",
      [affected(affectedRef.object_type, affectedRef.ref), affected("FieldPath", path), affected("Vocabulary", vocabulary)]
    );
  }
  return diagnostic(
    "SCH-STATUS-OUTSIDE-VOCABULARY", "blocking", "STATUS_VOCABULARY",
    "Source record member holds a value outside the software's vocabulary for that member; the value is not carried.",
    "Correct the source record so the member holds a value of its vocabulary, or omit the member.",
    [affected(affectedRef.object_type, affectedRef.ref), affected("FieldPath", path), affected("Vocabulary", vocabulary)]
  );
}

const inVocabulary = (value: unknown, vocabulary: string) => typeof value === "string" && STATUS_VOCABULARIES[vocabulary].has(value);

function checkMember(container: unknown, segments: string[], path: string, vocabulary: string, affectedRef: Ref, diagnostics: Diagnostic[]): void {
  if (!isMapping(container) || segments.length === 0) return;
  const [segment, ...rest] = segments;
  const many = segment.endsWith("[]");
  const key = many ? segment.slice(0, -2) : segment;
  if (!Object.hasOwn(container, key)) return;
  const value = container[key];
  const where = `${path}.${key}`;
  if (many) {
    if (!Array.isArray(value)) return;
    if (rest.length) {
      value.forEach((item, index) => checkMember(item, rest, `${where}[${index}]`, vocabulary, affectedRef, diagnostics));
      return;
    }
    const kept: unknown[] = [];
    value.forEach((item, index) => {
      if (inVocabulary(item, vocabulary)) kept.push(item);
      else diagnostics.push(vocabularyDiagnostic(affectedRef, `${where}[${index}]`, vocabulary));
    });
    container[key] = kept;
    return;
  }
  if (rest.length) { checkMember(value, rest, where, vocabulary, affectedRef, diagnostics); return; }
  if (value === null || value === undefined) return;
  if (!inVocabulary(value, vocabulary)) {
    diagnostics.push(vocabularyDiagnostic(affectedRef, where, vocabulary));
    delete container[key];
  }
}

/** A copy of `record` with each listed status-bearing member checked against its vocabulary:
 * a value outside it is reported and not carried. */
export function checkedMembers(record: unknown, members: ReadonlyArray<readonly [string, string]>, affectedRef: Ref, diagnostics: Diagnostic[]): JsonRecord {
  const checked: JsonRecord = isMapping(record) ? structuredClone(record) : {};
  for (const [path, vocabulary] of members) checkMember(checked, path.split("."), "$", vocabulary, affectedRef, diagnostics);
  return checked;
}

/** The reference's `_checked_record`. */
function checkedRecord(record: unknown, kind: keyof typeof STATUS_MEMBERS, affectedRef: Ref, diagnostics: Diagnostic[]): JsonRecord {
  return checkedMembers(record, STATUS_MEMBERS[kind], affectedRef, diagnostics);
}

function comparisonId(item: JsonRecord, fallback: string): string {
  for (const key of ["comparison_id", "comparison_ref", "comparison_kind"]) {
    if (truthy(item[key])) {
      const value = item[key];
      if (isMapping(value)) return strOr(truthy(value.ref) ? value.ref : value.ref_id, fallback);
      return pythonStr(value);
    }
  }
  return fallback;
}

function boundaryFlagsFromSource(boundary: unknown, affectedRef: Ref, diagnostics: Diagnostic[]): void {
  if (!isMapping(boundary)) {
    diagnostics.push(diagnostic(
      "SCH-PROFESSIONAL-BOUNDARY-MISSING", "warning", "PROFESSIONAL_BOUNDARY",
      "Source record lacks explicit professional-boundary metadata.",
      "Provide source boundary metadata or keep human-review limitation visible.",
      [affected(affectedRef.object_type, affectedRef.ref)]
    ));
    return;
  }
  for (const flag of AUTHORITY_FLAG_NAMES) {
    if (boundary[flag] === true) {
      diagnostics.push(diagnostic(
        "SCH-SOFTWARE-AUTHORITY-FLAG-BLOCKED", "blocking", "PROFESSIONAL_BOUNDARY",
        "Source boundary metadata attempted to set a software authority flag.",
        "Keep the source as a diagnostic finding and require external human review.",
        [affected(affectedRef.object_type, affectedRef.ref), affected("BoundaryFlag", flag)]
      ));
    }
  }
}

/** The value with private payload members removed (AC-006). Text is carried as it is. */
function safePublicContext(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(safePublicContext);
  if (isMapping(value)) {
    const out: JsonRecord = {};
    for (const [key, item] of Object.entries(value)) if (!PRIVATE_PAYLOAD_KEYS.has(key)) out[key] = safePublicContext(item);
    return out;
  }
  return copy(value);
}

function checkDeltaUnits(delta: unknown, affectedRef: Ref, diagnostics: Diagnostic[]): void {
  const record = isMapping(delta) ? delta : {};
  if (isMissing(record.normalized_unit) || isMissing(record.dimension)) {
    diagnostics.push(diagnostic(
      "SCH-COMPARISON-DELTA-UNIT-MISSING", "blocking", "UNIT_METADATA",
      "Comparison delta lacks normalized unit or dimension metadata.",
      "Provide normalized unit and dimension before reporting numeric deltas.",
      [affected(affectedRef.object_type, affectedRef.ref)]
    ));
  }
}

function manualMappings(items: unknown[]): unknown[] {
  return pythonSorted(items.filter((item) => isMapping(item)
    && pythonStr(truthy(item.match_basis) ? item.match_basis : truthy(item.mapping_status) ? item.mapping_status : "").startsWith("manual")).map(copy));
}

const UNMATCHED_ROW_CLASSIFICATIONS = new Set(["added", "removed", "unresolved"]);
const UNMATCHED_DIAGNOSTIC_CLASSES = new Set(["UNMATCHED_ENTITY", "MAPPING_UNRESOLVED", "MISSING_RESULT_DATA"]);

/** The reference's `_unmatched_classifications`: tokens and references only. */
export function unmatchedClassifications(items: unknown[]): unknown[] {
  const unmatched: JsonRecord[] = [];
  for (const item of items.filter(isMapping)) {
    if (Object.hasOwn(item, "classification")) {
      if (UNMATCHED_ROW_CLASSIFICATIONS.has(pythonStr(item.classification)) || item.match_basis === "unmatched") {
        unmatched.push({ classification: copy(item.classification), match_basis: copy(item.match_basis), left_ref: copy(item.left_ref), right_ref: copy(item.right_ref) });
      }
    } else if (Object.hasOwn(item, "code") && UNMATCHED_DIAGNOSTIC_CLASSES.has(pythonStr(item.class))) {
      unmatched.push({ code: copy(item.code), class: copy(item.class), affected_refs: list(item.affected_refs).map(copy) });
    }
  }
  return pythonSorted(unmatched);
}

/** REQ-009, AC-006: a member holding a library record's own content (its provenance record)
 * keeps only its path and hashes; every other changed field keeps both stored values. */
const RECORD_CONTENT_MEMBER = /^provenance_record(?:[.[]|$)/;
const ORIGIN_KEYS = ["left_origin_ref", "right_origin_ref"];

function changePath(change: JsonRecord): string {
  return pythonStr(truthy(change.path) ? change.path : truthy(change.field) ? change.field : "");
}

function restatedChange(change: JsonRecord): JsonRecord {
  const contentMember = RECORD_CONTENT_MEMBER.test(changePath(change));
  const kept: JsonRecord = {};
  for (const [key, value] of Object.entries(change)) if (!contentMember || !CHANGE_VALUE_KEYS.has(key)) kept[key] = copy(value);
  return kept;
}

function changedEntities(items: unknown[]): unknown[] {
  const changed = items.filter(isMapping).filter((item) => !UNCHANGED_CLASSIFICATIONS.has(pythonStr(item.classification))).map((item) => {
    const entry: JsonRecord = {
      classification: copy(item.classification),
      left_ref: copy(item.left_ref),
      right_ref: copy(item.right_ref),
      changes: list(item.changes).filter(isMapping).map(restatedChange)
    };
    for (const key of ORIGIN_KEYS) if (Object.hasOwn(item, key)) entry[key] = copy(item[key]);
    return entry;
  });
  return pythonSorted(changed);
}

/* The bound on restated result differences (REQ-011). */
export const LARGEST_DIFFERENCES_PER_FAMILY_AND_UNIT = 10;
const RESULT_FAMILY_NONE = "none";
const RUN_CONTEXT_ROW_LISTS = new Set(["not_compared", "unmatched"]);
export const CSV_STATEMENT = "The comparison table's CSV holds every row of the comparison; an export with a filter holds fewer. The CSV's preamble states the hash of this comparison record.";

function csvStatement(value: unknown): JsonRecord | null {
  if (!isMapping(value)) return null;
  return { table: copy(value.table), row_count: copy(value.row_count), statement: CSV_STATEMENT };
}

function boundedRunContext(value: unknown): unknown {
  if (!isMapping(value)) return copy(value);
  const out: JsonRecord = {};
  for (const [key, item] of Object.entries(value)) if (!RUN_CONTEXT_ROW_LISTS.has(key)) out[key] = copy(item);
  return out;
}

const mapping = (value: unknown): JsonRecord => (isMapping(value) ? value : {});
const isFiniteNumber = (value: unknown): value is number => typeof value === "number" && Number.isFinite(value);
const deltaIdentity = (delta: JsonRecord) => mapping(delta.identity);
function deltaFamily(delta: JsonRecord): string {
  const value = truthy(delta.result_family) ? delta.result_family : deltaIdentity(delta).family;
  return truthy(value) ? pythonStr(value) : RESULT_FAMILY_NONE;
}
const deltaResultId = (delta: JsonRecord) => pythonStr(truthy(delta.left_result_id) ? delta.left_result_id : truthy(delta.result_id) ? delta.result_id : "");
function deltaObject(delta: JsonRecord): unknown {
  const identity = deltaIdentity(delta);
  if (Object.hasOwn(identity, "object_ref")) return identity.object_ref ?? null;
  return mapping(delta.object_refs).left ?? null;
}
function rowFamily(row: JsonRecord): string {
  const value = deltaIdentity(row).family;
  return truthy(value) ? pythonStr(value) : RESULT_FAMILY_NONE;
}

type Counts = { pairs_compared: number; pairs_differing: number; pairs_not_compared: Record<string, number>; rows_unmatched: Record<string, Record<string, number>> };
const emptyCounts = (): Counts => ({ pairs_compared: 0, pairs_differing: 0, pairs_not_compared: {}, rows_unmatched: { left: {}, right: {} } });
const add = (counter: Record<string, number>, key: string) => { counter[key] = (counter[key] ?? 0) + 1; };

/** The reference's `_result_difference_counts`. */
function resultDifferenceCounts(deltas: unknown[], runContext: unknown): JsonRecord {
  const context = mapping(runContext);
  const total = emptyCounts();
  const families = new Map<string, Counts>();
  const buckets = (family: string): Counts[] => {
    if (!families.has(family)) families.set(family, emptyCounts());
    return [total, families.get(family)!];
  };
  for (const delta of deltas.filter(isMapping)) {
    const value = delta.normalized_delta;
    for (const bucket of buckets(deltaFamily(delta))) {
      bucket.pairs_compared += 1;
      if (isFiniteNumber(value) && value !== 0) bucket.pairs_differing += 1;
    }
  }
  for (const pair of list(context.not_compared).filter(isMapping)) {
    for (const bucket of buckets(rowFamily(pair))) for (const reason of list(pair.reasons)) add(bucket.pairs_not_compared, pythonStr(reason));
  }
  for (const row of list(context.unmatched).filter(isMapping)) {
    for (const bucket of buckets(rowFamily(row))) {
      const side = pythonStr(row.side);
      bucket.rows_unmatched[side] ??= {};
      add(bucket.rows_unmatched[side], pythonStr(row.reason));
    }
  }
  return {
    total,
    by_family: [...families.keys()].sort(codePointCompare).map((family) => ({ result_family: family, ...families.get(family)! }))
  };
}

const ROWS_LISTED_PER_FAMILY = 10;
export const SUMMARY_WHERE = "Every pair and row is in the complete comparison record this section names by its hash and in the comparison table's CSV.";
type RowKind = "not_compared" | "unmatched";

function rowEntry(row: JsonRecord, kind: RowKind): JsonRecord {
  const identity = deltaIdentity(row);
  const entry: JsonRecord = {
    result_id: pythonStr(truthy(row.result_id) ? row.result_id : ""),
    object_ref: copy(identity.object_ref),
    basis_ref: copy(identity.basis_ref),
    component: copy(identity.component),
    location: copy(identity.location)
  };
  if (kind === "not_compared") Object.assign(entry, { left_unit: copy(row.left_unit), right_unit: copy(row.right_unit), reasons: list(row.reasons).map(copy) });
  else Object.assign(entry, { side: copy(row.side), unit: copy(row.unit), reason: copy(row.reason), differing_fields: list(row.differing_fields).map(copy) });
  return entry;
}

function rowOrder(left: JsonRecord, right: JsonRecord): number {
  const id = (row: JsonRecord) => pythonStr(truthy(row.result_id) ? row.result_id : "");
  const side = (row: JsonRecord) => pythonStr(truthy(row.side) ? row.side : "");
  return codePointCompare(id(left), id(right)) || codePointCompare(side(left), side(right));
}

function rowsByFamily(runContext: unknown, kind: RowKind): Map<string, JsonRecord[]> {
  const families = new Map<string, JsonRecord[]>();
  for (const row of list(mapping(runContext)[kind]).filter(isMapping)) {
    const family = rowFamily(row);
    if (!families.has(family)) families.set(family, []);
    families.get(family)!.push(row);
  }
  return families;
}

/** The reference's `_listed_rows`. */
function listedRows(runContext: unknown, kind: RowKind): JsonRecord[] {
  const families = rowsByFamily(runContext, kind);
  return [...families.keys()].sort(codePointCompare).map((family) => ({
    result_family: family,
    rows: [...families.get(family)!].sort(rowOrder).slice(0, ROWS_LISTED_PER_FAMILY).map((row) => rowEntry(row, kind))
  }));
}

/** The reference's `_summary_coverage`. */
function summaryCoverage(deltas: unknown[], runContext: unknown): JsonRecord {
  const largest = largestDifferences(deltas);
  const differing = deltas.filter(isMapping).filter((delta) => isFiniteNumber(delta.normalized_delta) && delta.normalized_delta !== 0).length;
  const listed = largest.reduce((count, group) => count + (group.differences as unknown[]).length, 0);
  const notListed = (kind: RowKind) => [...rowsByFamily(runContext, kind).values()].reduce((count, rows) => count + Math.max(0, rows.length - ROWS_LISTED_PER_FAMILY), 0);
  const families = new Set<string>([
    ...deltas.filter(isMapping).map(deltaFamily),
    ...rowsByFamily(runContext, "not_compared").keys(),
    ...rowsByFamily(runContext, "unmatched").keys()
  ]);
  return {
    result_families: [...families].sort(codePointCompare),
    differences_not_listed: differing - listed,
    pairs_not_compared_not_listed: notListed("not_compared"),
    rows_unmatched_not_listed: notListed("unmatched"),
    where: SUMMARY_WHERE
  };
}

function rankCompare(left: JsonRecord, right: JsonRecord): number {
  const a = Math.abs(left.normalized_delta as number);
  const b = Math.abs(right.normalized_delta as number);
  if (a !== b) return b - a > 0 ? 1 : -1;
  return codePointCompare(deltaResultId(left), deltaResultId(right));
}

function differenceEntry(delta: JsonRecord): JsonRecord {
  const identity = deltaIdentity(delta);
  return {
    result_id: deltaResultId(delta),
    object_ref: copy(deltaObject(delta)),
    basis_ref: copy(identity.basis_ref),
    component: copy(identity.component),
    location: copy(identity.location),
    left_magnitude: copy(delta.left_magnitude),
    right_magnitude: copy(delta.right_magnitude),
    normalized_delta: copy(delta.normalized_delta),
    normalized_unit: copy(delta.normalized_unit),
    dimension: copy(delta.dimension)
  };
}

/** The reference's `_largest_differences`. */
function largestDifferences(deltas: unknown[]): JsonRecord[] {
  const groups = new Map<string, { family: string; unit: string; byObject: Map<string, JsonRecord> }>();
  for (const delta of deltas.filter(isMapping)) {
    const value = delta.normalized_delta;
    if (!isFiniteNumber(value) || value === 0) continue;
    const family = deltaFamily(delta);
    const unit = truthy(delta.normalized_unit) ? pythonStr(delta.normalized_unit) : "";
    const key = JSON.stringify([family, unit]);
    if (!groups.has(key)) groups.set(key, { family, unit, byObject: new Map() });
    const byObject = groups.get(key)!.byObject;
    const objectKey = pythonJson(deltaObject(delta));
    const current = byObject.get(objectKey);
    if (!current || rankCompare(delta, current) < 0) byObject.set(objectKey, delta);
  }
  return [...groups.values()]
    .sort((left, right) => codePointCompare(left.family, right.family) || codePointCompare(left.unit, right.unit))
    .map(({ family, unit, byObject }) => ({
      result_family: family,
      normalized_unit: unit,
      differences: [...byObject.values()].sort(rankCompare).slice(0, LARGEST_DIFFERENCES_PER_FAMILY_AND_UNIT).map(differenceEntry)
    }));
}

function hashRefs(value: unknown): unknown[] {
  return pythonSorted(list(value).map(copy));
}

function privacyClassification(record: JsonRecord): string {
  for (const key of ["privacy_classification", "review_classification"]) if (truthy(record[key])) return pythonStr(record[key]);
  const privacy = record.privacy;
  if (isMapping(privacy) && truthy(privacy.classification)) return pythonStr(privacy.classification);
  const provenance = record.provenance;
  if (isMapping(provenance) && truthy(provenance.privacy_classification)) return pythonStr(provenance.privacy_classification);
  return "TBD";
}

function reviewState(record: JsonRecord): string {
  for (const key of ["review_status", "review_classification"]) if (truthy(record[key])) return pythonStr(record[key]);
  const provenance = record.provenance;
  if (isMapping(provenance)) {
    const value = truthy(provenance.review_status) ? provenance.review_status : truthy(provenance.review_classification) ? provenance.review_classification : "TBD";
    return pythonStr(value);
  }
  return "TBD";
}

/* -------------------------------------------------------------------------- *
 * The sections and the set-level rules
 * -------------------------------------------------------------------------- */

/** The comparison records a section set is assembled from, in the reference's inputs. */
export type ComparisonRecords = { model_state_comparisons: unknown[]; analysis_run_comparisons: unknown[] };

/** The reference's `_comparison_sections`: one section per record, in canonical order, with
 * the source diagnostics it raises appended to `diagnostics`. */
export function comparisonSections(records: ComparisonRecords, diagnostics: Diagnostic[]): JsonRecord[] {
  const sections: JsonRecord[] = [];
  for (const raw of records.model_state_comparisons) {
    const item = isMapping(raw) ? raw : {};
    const comparisonRef = ref("ModelStateComparison", comparisonId(item, "model-state-comparison:TBD"));
    boundaryFlagsFromSource(item.professional_boundary, comparisonRef, diagnostics);
    const checked = checkedRecord(item, "model_state_comparison", comparisonRef, diagnostics);
    sections.push(stateComparisonSection(checked, comparisonRef));
  }
  for (const raw of records.analysis_run_comparisons) {
    const item = isMapping(raw) ? raw : {};
    const comparisonRef = ref("AnalysisRunComparison", comparisonId(item, "analysis-run-comparison:TBD"));
    boundaryFlagsFromSource(item.professional_boundary, comparisonRef, diagnostics);
    const checked = checkedRecord(item, "analysis_run_comparison", comparisonRef, diagnostics);
    sections.push(runComparisonSection(checked, comparisonRef, diagnostics));
  }
  return pythonSorted(sections);
}

function stateComparisonSection(item: JsonRecord, comparisonRef: Ref): JsonRecord {
  return {
    section_id: `comparison:${comparisonRef.ref}`,
    section_kind: "model_state_comparison",
    comparison_ref: comparisonRef,
    left_ref: copy(item.left_state_ref),
    right_ref: copy(item.right_state_ref),
    summary: copy(item.summary),
    changed_entities: changedEntities(list(item.entities)),
    hash_refs: hashRefs(item.hashes),
    manual_mappings: manualMappings(list(item.entities)),
    unmatched_classifications: unmatchedClassifications(list(item.entities)),
    tolerance_profile_refs: [],
    unit_normalized_deltas: [],
    settings: copy(item.settings),
    csv: csvStatement(item.csv),
    diagnostics: safePublicContext(list(item.diagnostics)),
    source_provenance: copy(item.provenance),
    privacy_classification: privacyClassification(item),
    review_state: reviewState(item),
    professional_boundary: copy(PROFESSIONAL_BOUNDARY),
    text_origin: copy(TEXT_ORIGINS.model_state_comparison)
  };
}

function runComparisonSection(item: JsonRecord, comparisonRef: Ref, diagnostics: Diagnostic[]): JsonRecord {
  const deltas = list(item.result_deltas).map(copy);
  for (const delta of deltas) checkDeltaUnits(delta, comparisonRef, diagnostics);
  const profiles = new Set<string>();
  for (const delta of deltas) if (isMapping(delta) && truthy(delta.tolerance_profile_ref)) profiles.add(pythonStr(delta.tolerance_profile_ref));
  return {
    section_id: `comparison:${comparisonRef.ref}`,
    section_kind: "analysis_run_comparison",
    comparison_ref: comparisonRef,
    run_context: boundedRunContext(item.run_context),
    hash_refs: hashRefs(item.hashes),
    result_difference_counts: resultDifferenceCounts(deltas, item.run_context),
    largest_differences: largestDifferences(deltas),
    pairs_not_compared_listed: listedRows(item.run_context, "not_compared"),
    rows_unmatched_listed: listedRows(item.run_context, "unmatched"),
    summary_coverage: summaryCoverage(deltas, item.run_context),
    csv: csvStatement(item.csv),
    manual_mappings: manualMappings(deltas),
    unmatched_classifications: unmatchedClassifications(list(item.diagnostics)),
    tolerance_profile_refs: [...profiles].sort(codePointCompare),
    settings_deltas: list(item.settings_deltas).map(copy),
    diagnostics: safePublicContext(list(item.diagnostics)),
    source_provenance: copy(item.provenance),
    privacy_classification: privacyClassification(item),
    review_state: reviewState(item),
    professional_boundary: copy(PROFESSIONAL_BOUNDARY),
    text_origin: copy(TEXT_ORIGINS.analysis_run_comparison)
  };
}

/** The reference's `SOFTWARE_STANDING_NOT_DERIVED`. A later change gives `software_standing` the
 * software's computed reading of the model state; until then it says, in words, that none is derived. */
export const SOFTWARE_STANDING_NOT_DERIVED = "The software does not derive a standing for the current model state when the report package is assembled; the analysis status labels beside it are the document author's.";

/** What the current-state section is formed from: the reference's `build_current_model_state_section` input. */
export type CurrentModelStateRecord = {
  model_ref: string;
  model_hash: string;
  authored_analysis_status: unknown;
  document_diagnostics: unknown[];
  provenance: JsonRecord;
};

/** The reference's `build_current_model_state_section`: the model as it stands, by the model
 * reference and its canonical hash. The document's analysis-status labels and diagnostics are its
 * author's, carried as they are, unfiltered and unjudged; the software's statuses are stated
 * beside them, and the section says that no standing of the state is derived at assembly. */
export function buildCurrentModelStateSection(record: CurrentModelStateRecord): JsonRecord {
  const stateRef = `state:${pythonStr(record.model_ref)}:preview`;
  const reference = ref("ModelState", stateRef);
  const diagnostics = list(record.document_diagnostics).map(copy);
  return {
    section_id: `state-run:${stateRef}`,
    section_kind: "model_state_record",
    state_ref: reference,
    run_ref: null,
    hash_refs: [{ algorithm: "sha256", canonicalization: "rfc8785_jcs", payload_ref: copy(reference), payload_scope: "model_state", value: copy(record.model_hash) }],
    diagnostics,
    warnings: diagnostics.map(copy),
    assumptions: [],
    analysis_status: ["HUMAN_REVIEW_REQUIRED"],
    authored_analysis_status: copy(record.authored_analysis_status),
    software_standing: { derived: false, statement: SOFTWARE_STANDING_NOT_DERIVED },
    unit_context: { ref_type: "unit_system", ref_id: "unit-system:dec-018-si-dual-display" },
    solver_context: null,
    settings_ref: null,
    source_provenance: copy(record.provenance),
    privacy_classification: "private_project_data",
    review_state: "pending",
    limitations: [],
    professional_boundary: copy(PROFESSIONAL_BOUNDARY),
    text_origin: copy(TEXT_ORIGINS.current_model_state)
  };
}

/** Comparison sections already formed from their records, with the source diagnostics their
 * forming raised: what the desktop session holds for an included comparison. */
export type BoundComparisons = { sections: JsonRecord[]; diagnostics: Diagnostic[] };

/** The sections and source diagnostics of comparison records. */
export function boundComparisons(records: ComparisonRecords): BoundComparisons {
  const diagnostics: Diagnostic[] = [];
  return { sections: comparisonSections(records, diagnostics), diagnostics };
}

/** The reference's `_unresolved_tbds` entries of these sections: one per member whose value is null. */
export function sectionTbds(sections: JsonRecord[]): JsonRecord[] {
  const tbds: JsonRecord[] = [];
  for (const section of sections) {
    for (const [key, value] of Object.entries(section)) {
      if (value === null || value === undefined) {
        tbds.push({
          tbd_id: `DEL-08-06-TBD-${pythonStr(section.section_id)}-${key}`,
          description: `Source value for ${key} is unavailable in section ${pythonStr(section.section_id)}.`,
          review_needed: true
        });
      }
    }
  }
  return tbds;
}

function unitDiagnostic(path: string, message: string): Diagnostic {
  return diagnostic(
    "SCH-NUMERIC-UNIT-METADATA-MISSING", "blocking", "UNIT_METADATA", message,
    "Provide unit and dimension metadata or carry the value only as a source diagnostic.",
    [affected("FieldPath", path)]
  );
}

/** The reference's set-level `_numeric_unit_diagnostics` of
 * `value`, a member of the set at `path`. */
export function setLevelDiagnostics(value: unknown, path: string): Diagnostic[] {
  const diagnostics: Diagnostic[] = [];
  for (const [at, container] of walkDicts(value, path)) {
    const numericKeys = NUMERIC_VALUE_KEYS.filter((key) => Object.hasOwn(container, key)).sort(codePointCompare);
    if (!numericKeys.length) continue;
    for (const key of numericKeys) {
      const item = container[key];
      if (!(typeof item === "number" && Number.isFinite(item))) diagnostics.push(unitDiagnostic(`${at}.${key}`, "Numeric value is not finite."));
    }
    const hasUnit = !isMissing(container.unit) || !isMissing(container.normalized_unit);
    if (!hasUnit || isMissing(container.dimension)) diagnostics.push(unitDiagnostic(at, "Numeric value lacks unit or dimension metadata."));
  }
  return diagnostics;
}

/** `sorted(_dedupe_diagnostics(diagnostics), key=canonical_json)`. */
export function settleDiagnostics(diagnostics: Diagnostic[]): Diagnostic[] {
  const seen = new Set<string>();
  const unique = diagnostics.filter((item) => {
    const key = pythonJson(item);
    if (seen.has(key)) return false;
    seen.add(key);
    return true;
  });
  return pythonSorted(unique);
}

/** Whether a section holds warnings or diagnostics, as the coverage statement reads it. */
export function holdsWarnings(section: JsonRecord): boolean {
  return truthy(section.warnings) || truthy(section.diagnostics);
}

/** The reference's whole section set for comparison records only (no state, run or handoff
 * record): the form of the shared parity vector. */
export function buildComparisonSectionSet(inputs: {
  section_set_id: string;
  model_states: unknown[];
  analysis_runs: unknown[];
  model_state_comparisons: unknown[];
  analysis_run_comparisons: unknown[];
  handoff_packages: unknown[];
  export_workflows: unknown[];
  external_prover_metadata: unknown[];
  source_notes: string[];
  provenance: JsonRecord | null;
}): JsonRecord {
  if (inputs.model_states.length || inputs.analysis_runs.length || inputs.handoff_packages.length || inputs.export_workflows.length || inputs.external_prover_metadata.length) {
    throw new Error("DESKTOP-PARITY-FIXTURE-SCOPE: the comparison parity vector holds comparison records only.");
  }
  const diagnostics: Diagnostic[] = [];
  const sections = comparisonSections(inputs, diagnostics);
  const record: JsonRecord = {
    schema_version: "0.1.0",
    deliverable_id: "DEL-08-06",
    package_id: "PKG-08",
    scope_item: "SOW-024",
    objectives: ["OBJ-007", "OBJ-016", "OBJ-017", "OBJ-018"],
    section_set_id: String(inputs.section_set_id),
    section_contract_status: "backend_report_section_records_only",
    source_notes: [...inputs.source_notes.map(String)].sort(codePointCompare),
    sections: { state_run_sections: [], comparison_sections: sections, handoff_sections: [] },
    sow_024_coverage: {
      inputs: "TBD",
      sources: sections.length ? "represented_by_source_provenance_and_source_notes" : "TBD",
      warnings: sections.some(holdsWarnings) ? "represented_by_warning_and_diagnostic_lists" : "TBD",
      assumptions: "TBD",
      results: sections.length ? "represented_by_result_refs_and_unit_normalized_deltas" : "TBD",
      rule_pack_checksums: "TBD",
      limitations: "represented_by_limitations_and_unresolved_tbds"
    },
    limitations: [{
      limitation_id: "DEL-08-06-LIMIT-BACKEND-SECTIONS-ONLY",
      statement: "Report sections are backend records for human review; final layout, transport, solver execution, and external-tool execution are out of scope.",
      human_review_required: true
    }],
    unresolved_tbds: pythonSorted([
      {
        tbd_id: "DEL-08-06-TBD-FINAL-LAYOUT",
        description: "Final report styling, layout, transport, and release thresholds remain outside this deliverable.",
        review_needed: true
      },
      ...sectionTbds(sections)
    ]),
    diagnostics: [],
    professional_boundary: copy(PROFESSIONAL_BOUNDARY),
    provenance: copy(truthy(inputs.provenance) ? inputs.provenance : ENGINE_PROVENANCE)
  };
  record.diagnostics = settleDiagnostics([...diagnostics, ...setLevelDiagnostics(record, "$")]);
  return record;
}
