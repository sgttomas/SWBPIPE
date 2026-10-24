// Applying a stored library record to the model (W-17, DEL-03-01 Design
// `apply-library-records.md`). This maps a record AFTER native import
// validation of the stored document; it is not a parallel library validator.
//
// Every result is one reviewed applier batch: insert (create_material,
// create_section) or `apply_library_record` on an existing Material, Section
// or Component. Copying is fixed by table (§2.3); everything else is listed as
// "not copied" with its reason. Units copy as entered (DEC-018); a material
// `unit_ref` must resolve to a catalog unit (§2.4). The model receives the
// copied values, the label, the short `provenance` string and the combined
// `provenance_record` (§2.5); library metadata, other records, allowables,
// non-copied slots and record snapshots never enter it (§2.6).
//
// Material modes (§2.3a as reconciled with the HELP_HUMAN E/nu rule,
// 2026-10-23): the user chooses E/nu or E/G; nothing is inferred.
// - E/nu writes E, nu (unit 1), G and the exact basis. G is the record's
//   stored G when it holds one (copied, never overwritten) or is derived
//   exactly as the exact route derives it (E normalized to Pa, then
//   E / (2(1+nu))) and stored in Pa; `provenance_record` says which, naming
//   the basis "derived from E and ν (isotropic)".
// - E/G writes E and G with no basis. It is refused when the record holds nu,
//   or the target material holds nu, because nu is never discarded.
// Presence is judged apart from copy eligibility: a record that stores G or
// nu in a slot that cannot be copied (temperature-referenced, excluded
// disposition, ineligible status) refuses, naming the slot; G is never
// derived in its place. A mode whose member the record lacks is refused,
// naming the member.
import { canonicalJsonString, canonicalSha256Hex } from "../../services/hashService";
import type { OperationBatch } from "../../services/operationBatchService";
import { loadWasmEngine } from "../../services/wasmEngine/loadWasmEngine";
import type { UnitCatalog } from "../../services/unitCatalogService";
import type { EditorOperationIntent, PreviewModel, ProvenanceRecord } from "../../types";
import { draftIntent } from "../offline-proposal-intake/workflowSupport";

export type ApplyLibraryKind = "material" | "section" | "component" | "hanger";
export type MaterialMode = "e_nu" | "e_g";
/** "group": remove a written group's unsupplied replaceable members (components).
 * "all": remove every governed member the apply does not write. */
export type ReplaceOption = "none" | "group" | "all";

export type LibraryRef = NonNullable<ProvenanceRecord["library_ref"]>;
export type CopiedRow = { member: string; value: string; unit: string; source: string };
export type NotCopiedRow = { source: string; reason: string };
export type LibraryApplyPlan = {
  batch: OperationBatch;
  copied: CopiedRow[];
  notCopied: NotCopiedRow[];
  /** Members the apply removes (shown in the diff). */
  removed: string[];
  /** Pipes bound to an applied section; the applier updates each cache. */
  dependentPipes: string[];
  provenanceRecord: ProvenanceRecord;
};

export class LibraryApplyRefusal extends Error {
  constructor(readonly code: string, message: string) {
    super(`${code}: ${message}`);
  }
}
const refuse = (code: string, message: string): never => {
  throw new LibraryApplyRefusal(code, message);
};

type Json = Record<string, unknown>;
type Level = Json | undefined;
const obj = (value: unknown): Json | undefined =>
  value && typeof value === "object" && !Array.isArray(value) ? (value as Json) : undefined;
const str = (value: unknown): string | undefined => (typeof value === "string" ? value : undefined);

const KEYS: Record<Exclude<ApplyLibraryKind, "hanger">, { library: string; records: string; id: string }> = {
  material: { library: "material_library", records: "material_records", id: "material_id" },
  section: { library: "section_library", records: "section_records", id: "section_id" },
  component: { library: "component_library", records: "component_records", id: "component_id" }
};

/** `library_version`: "sha256:" + the SHA-256 of the record's RFC 8785 form (§2.5). */
export async function libraryRecordVersion(record: unknown): Promise<string> {
  return `sha256:${await canonicalSha256Hex(record)}`;
}

export function libraryProvenanceString(kind: ApplyLibraryKind, libraryId: string, recordId: string): string {
  return `library:${kind}:${libraryId}#${recordId}`;
}

// ---- dispositions (§2.5) --------------------------------------------------

const REDISTRIBUTION_RANK = { public_permissive: 0, unknown: 1, private_only: 2 } as const;
type Redistribution = keyof typeof REDISTRIBUTION_RANK;
const BLOCKED_TOKENS = new Set(["rejected", "protected_suspected", "quarantined"]);

function levelTokens(level: Level): string[] {
  if (!level) return [];
  const provenance = obj(level.provenance);
  return [level.redistribution_status, level.review_status, provenance?.redistribution_status, provenance?.review_status, level.value_status]
    .map(str)
    .filter((token): token is string => Boolean(token));
}

/** The disposition tokens that exclude a slot or value, or refuse a library or record. */
function blockedTokens(level: Level): string[] {
  return levelTokens(level).filter((token) => BLOCKED_TOKENS.has(token));
}

/** Every supported private privacy token: the `private_*` classes, the
 * section/component/hanger `project_private` (the hanger importer reads it as
 * private project data) and the material `quarantine_metadata_only`. */
export function isPrivateClass(privacyClass: string | undefined): boolean {
  return Boolean(privacyClass) && (privacyClass!.startsWith("private") || privacyClass === "project_private" || privacyClass === "quarantine_metadata_only");
}

function redistributionOf(level: Level): Redistribution {
  const provenance = obj(level?.provenance);
  const tokens = [level?.redistribution_status, provenance?.redistribution_status].map(str).filter(Boolean) as string[];
  let rank = tokens.length ? 0 : REDISTRIBUTION_RANK.unknown;
  for (const token of tokens) {
    const normalized = token === "private_only" || token === "public_permissive" ? token : "unknown";
    rank = Math.max(rank, REDISTRIBUTION_RANK[normalized]);
  }
  // A private class keeps the data private whatever the status fields say
  // (the importer's public policy, library_import_document/src/lib.rs:381-392).
  if (isPrivateClass(str(level?.privacy_class))) rank = REDISTRIBUTION_RANK.private_only;
  return (Object.keys(REDISTRIBUTION_RANK) as Redistribution[]).find((key) => REDISTRIBUTION_RANK[key] === rank)!;
}

function reviewOf(level: Level): "pending" | "accepted" {
  const provenance = obj(level?.provenance);
  const tokens = [level?.review_status, provenance?.review_status].map(str).filter(Boolean) as string[];
  return tokens.length > 0 && tokens.every((token) => token === "accepted") ? "accepted" : "pending";
}

/** The most restrictive redistribution and review across the levels. No
 * level promotes another; TBD or missing statuses become unknown and pending. */
export function combineDispositions(levels: Level[]): { redistribution_status: Redistribution; review_status: "pending" | "accepted" } {
  let rank = 0;
  let review: "pending" | "accepted" = "accepted";
  for (const level of levels) {
    rank = Math.max(rank, REDISTRIBUTION_RANK[redistributionOf(level)]);
    if (reviewOf(level) === "pending") review = "pending";
  }
  const redistribution = (Object.keys(REDISTRIBUTION_RANK) as Redistribution[]).find((key) => REDISTRIBUTION_RANK[key] === rank)!;
  return { redistribution_status: redistribution, review_status: review };
}

const SOURCE_FIELDS = ["source_name", "source_location", "source_license", "contributor", "contributor_certification"] as const;

function sourceDiffers(record: Json, level: Level): boolean {
  const recordProvenance = obj(record.provenance) ?? {};
  const levelProvenance = obj(level?.provenance);
  return Boolean(levelProvenance) && SOURCE_FIELDS.some((field) => levelProvenance![field] !== recordProvenance[field]);
}

type Copy = { member: string; value: unknown; unit: string; source: string; levels: Level[]; display: string };

type Context = {
  kind: ApplyLibraryKind;
  libraryId: string;
  recordId: string;
  library: Json;
  record: Json;
  version: string;
};

async function context(kind: Exclude<ApplyLibraryKind, "hanger">, document: Json, recordId: string): Promise<Context> {
  const keys = KEYS[kind];
  const library = obj(document[keys.library]);
  const records = Array.isArray(document[keys.records]) ? (document[keys.records] as unknown[]) : [];
  const record = records.map(obj).find((item) => item?.[keys.id] === recordId);
  if (!library || !record) refuse("LIBRARY-APPLY-RECORD-MISSING", "The selected record is not in the stored library. Refresh the library list, open the library again and choose a record it holds.");
  for (const [label, level] of [["library", library], ["record", record]] as const) {
    const blocked = blockedTokens(level);
    if (blocked.length) refuse("LIBRARY-APPLY-SOURCE-REJECTED", `The ${label} is marked ${blocked.join(", ")}, so its data is not applied. Choose a different record, or re-import the library with reviewed source dispositions.`);
  }
  return {
    kind,
    libraryId: str(library!.library_id) ?? "",
    recordId,
    library: library!,
    record: record!,
    version: await libraryRecordVersion(record)
  };
}

/** `replaceAllClears`: whether "replace all values" would remove every
 * retained value (false when a retained value is required or not replaceable). */
type Previous = { record?: ProvenanceRecord; retained: string[]; replaceAllClears?: boolean };

function provenanceRecord(ctx: Context, copies: Copy[], previous: Previous | null, notes: string[]): ProvenanceRecord {
  const levels: Level[] = [ctx.library, ctx.record, ...copies.flatMap((copy) => copy.levels)];
  const retained = previous?.retained ?? [];
  if (retained.length) {
    const prev = previous?.record;
    const blocked = [prev?.redistribution_status, prev?.review_status].filter((token) => token && BLOCKED_TOKENS.has(token));
    if (blocked.length) {
      refuse(
        "LIBRARY-APPLY-RETAINED-QUARANTINED",
        `The entity keeps ${retained.join(", ")} from a previous source marked ${blocked.join(", ")}, so they cannot be combined with this record. ` +
          (previous?.replaceAllClears
            ? `Choose "replace all values" to remove them, or have the previous source reviewed and update the entity's Source review, then apply.`
            : `Have the previous source reviewed and update the entity's Source review in its Provenance record section, then apply.`)
      );
    }
    // A missing previous record counts as unknown and pending.
    levels.push(prev ? { redistribution_status: prev.redistribution_status ?? "unknown", review_status: prev.review_status ?? "pending" } : { redistribution_status: "unknown", review_status: "pending" });
  }
  const combined = combineDispositions(levels);
  const source = obj(ctx.record.provenance) ?? {};
  const result: ProvenanceRecord = { source_name: str(source.source_name) ?? `${ctx.kind} library ${ctx.libraryId}` };
  for (const field of SOURCE_FIELDS.slice(1)) {
    const value = str(source[field]);
    if (value?.trim()) result[field] = value;
  }
  result.redistribution_status = combined.redistribution_status;
  result.review_status = combined.review_status;
  const differing = copies.filter((copy) => copy.levels.some((level) => sourceDiffers(ctx.record, level))).map((copy) => copy.source);
  const allNotes = [...notes];
  if (differing.length) allNotes.push(`Source fields differ from the record's for slots ${[...new Set(differing)].join(", ")}.`);
  if (retained.length) allNotes.push(`The entity also keeps values from its previous source: ${retained.join(", ")}.`);
  const previousRef = previous?.record?.library_ref;
  if (previousRef) {
    allNotes.push(`Applied over the previous library reference library:${previousRef.library_kind}:${previousRef.library_id}#${previousRef.record_id ?? ""}` +
      `${previousRef.library_version ? ` (${previousRef.library_version})` : ""}.`);
  }
  if (allNotes.length) result.verification_note = allNotes.join(" ");
  result.library_ref = { library_kind: ctx.kind, library_id: ctx.libraryId, library_version: ctx.version, record_id: ctx.recordId };
  return result;
}

// ---- values and units (§2.4) ----------------------------------------------

function magnitude(raw: unknown, source: string): number {
  const value = typeof raw === "number" ? raw : typeof raw === "string" && /^[-+]?(\d+\.?\d*|\.\d+)([eE][-+]?\d+)?$/.test(raw.trim()) ? Number(raw.trim()) : NaN;
  if (!Number.isFinite(value)) refuse("LIBRARY-APPLY-MAGNITUDE-INVALID", `Slot ${source} has the magnitude ${JSON.stringify(raw)}, which is not a finite number. Correct the value in the library and re-import it, or choose a different record.`);
  return value;
}

/** A material `unit_ref.ref_id` resolves to its symbol through the catalog;
 * an id the catalog does not hold refuses the apply. */
const UNIT_EXAMPLE: Record<string, string> = {
  poisson_ratio: "unit:one",
  thermal_expansion_coefficient: "unit:per_degree_celsius"
};

export function resolveCatalogUnit(catalog: UnitCatalog | null, unitId: unknown, source: string, kind = "elastic_modulus"): string {
  const entry = typeof unitId === "string" ? catalog?.entries.find((item) => item.unit_id === unitId) : undefined;
  if (!entry) {
    refuse("LIBRARY-APPLY-UNIT-UNRESOLVED", `Slot ${source} names unit ${JSON.stringify(unitId)}, which is not a unit of the DEC-018 catalog${catalog ? "" : " (the catalog is loaded only in the desktop application)"}. Use a catalog unit id (for example ${UNIT_EXAMPLE[kind] ?? "unit:pascal"}) in the library and re-import it${catalog ? "" : ", or apply from the desktop application"}.`);
  }
  return entry!.symbol;
}

const MATERIAL_ELIGIBLE = new Set(["provided", "private_user_supplied"]);
const SLOT_ELIGIBLE = new Set(["user_supplied", "private_user_supplied", "public_permissive_reviewed", "invented_non_engineering_example"]);

function shown(value: unknown): string {
  return typeof value === "object" ? JSON.stringify(value) : String(value);
}

// ---- material (§2.3, §2.3a) -----------------------------------------------

const MATERIAL_NOT_COPIED: Record<string, string> = {
  density: "the model holds density per pipe (section.material_density)",
  yield_strength: "rule checks read strengths from the library at run time",
  tensile_strength: "rule checks read strengths from the library at run time",
  allowable_stress: "rule checks read allowables from the library at run time",
  specific_heat: "no model slot",
  thermal_conductivity: "no model slot",
  user_defined: "no model slot",
  TBD: "no model slot"
};

/** `stored`: G or ν slots the record holds that cannot be copied (presence
 * is judged apart from copy eligibility). */
type MaterialValues = { copies: Map<string, Copy>; notCopied: NotCopiedRow[]; stored: Map<string, NotCopiedRow[]> };

function materialValues(ctx: Context, catalog: UnitCatalog | null): MaterialValues {
  const copies = new Map<string, Copy>();
  const notCopied: NotCopiedRow[] = [];
  const stored = new Map<string, NotCopiedRow[]>();
  const properties = Array.isArray(ctx.record.properties) ? (ctx.record.properties as unknown[]).map(obj) : [];
  for (const property of properties) {
    if (!property) continue;
    const kind = str(property.property_kind) ?? "TBD";
    const source = str(property.property_id) ?? kind;
    if (MATERIAL_NOT_COPIED[kind]) { notCopied.push({ source, reason: MATERIAL_NOT_COPIED[kind] }); continue; }
    const value = obj(property.value);
    const skip = (reason: string) => {
      notCopied.push({ source, reason });
      // The record stores this value even though it cannot be copied.
      if ((kind === "shear_modulus" || kind === "poisson_ratio") && (value || property.value_status === "protected_suspected")) {
        stored.set(kind, [...(stored.get(kind) ?? []), { source, reason }]);
      }
    };
    if (property.temperature_ref) { skip("temperature-referenced; library temperature points carry no temperature value"); continue; }
    const blocked = [...blockedTokens(property), ...blockedTokens(value)];
    if (blocked.length) { skip(`excluded: ${[...new Set(blocked)].join(", ")}`); continue; }
    if (!MATERIAL_ELIGIBLE.has(str(property.value_status) ?? "") || !value) {
      skip(`value status ${str(property.value_status) ?? "missing"} is not copied`);
      continue;
    }
    if (copies.has(kind)) refuse("LIBRARY-APPLY-AMBIGUOUS-SOURCE", `The record holds more than one copyable ${kind}, so the source is ambiguous. Keep one ${kind} in the record and re-import the library, or choose a different record.`);
    const unitRef = obj(value.unit_ref);
    const unit = resolveCatalogUnit(catalog, unitRef?.ref_id, source, kind);
    const number = magnitude(value.magnitude, source);
    copies.set(kind, { member: kind, value: { value: number, unit }, unit, source, levels: [property, value], display: String(number) });
  }
  for (const allowable of Array.isArray(ctx.record.allowables) ? (ctx.record.allowables as unknown[]).map(obj) : []) {
    if (allowable) notCopied.push({ source: str(allowable.allowable_id) ?? "allowable", reason: "rule checks read allowables from the library at run time" });
  }
  return { copies, notCopied, stored };
}

const describeStored = (rows: NotCopiedRow[]) => rows.map((row) => `${row.source} (${row.reason})`).join(", ");

type Material = NonNullable<PreviewModel["materials"]>[number];

type MaterialResult = { members: Record<string, unknown>; copies: Copy[]; notCopied: NotCopiedRow[]; notes: string[] };

type DeriveG = (elasticModulus: number, unit: string, poissonRatio: number) => string;

function materialMembers(ctx: Context, catalog: UnitCatalog | null, mode: MaterialMode, derivedG: DeriveG): MaterialResult {
  const { copies, notCopied, stored } = materialValues(ctx, catalog);
  const e = copies.get("elastic_modulus");
  const g = copies.get("shear_modulus");
  const nu = copies.get("poisson_ratio");
  const alpha = copies.get("thermal_expansion_coefficient");
  if (!e) refuse("LIBRARY-APPLY-MEMBER-MISSING", "The record has no copyable elastic_modulus (E). Choose a record that provides E, or add E to this record and re-import the library.");
  const members: Record<string, unknown> = { label: str(ctx.record.name), elastic_modulus: e!.value };
  const used: Copy[] = [e!];
  const notes: string[] = [];
  const storedNu = stored.get("poisson_ratio");
  const storedG = stored.get("shear_modulus");
  if (mode === "e_nu") {
    if (!nu && storedNu) refuse("LIBRARY-APPLY-MEMBER-BLOCKED", `The record stores ν but it cannot be copied (${describeStored(storedNu)}). Fix or re-import the record's ν, or choose a different record.`);
    if (!nu) refuse("LIBRARY-APPLY-MEMBER-MISSING", "E/ν mode needs poisson_ratio (ν), and the record has none. Choose E/G mode if the record provides G, or choose a record that provides ν.");
    if (nu!.unit !== "1") refuse("LIBRARY-APPLY-UNIT-UNRESOLVED", `Slot ${nu!.source} gives ν in unit ${nu!.unit}; ν must carry unit 1 (unit:one). Correct the unit in the library and re-import it, or choose a different record.`);
    members.constitutive_basis = "homogeneous_isotropic_E_nu_v1";
    members.poisson_ratio = nu!.value;
    used.push(nu!);
    if (g) {
      // A stored G is used as stored and never overwritten.
      members.shear_modulus = g.value;
      used.push(g);
      notes.push(`shear_modulus: stored in the record (${g.source}), copied as stored; not derived.`);
    } else {
      // The record stores a G it cannot copy: G is never derived in its place.
      if (storedG) refuse("LIBRARY-APPLY-MEMBER-BLOCKED", `The record stores G but it cannot be copied (${describeStored(storedG)}), and G is never derived when the record stores one. Fix or re-import the record's G, or choose a different record.`);
      const eValue = e!.value as { value: number; unit: string };
      const nuValue = (nu!.value as { value: number }).value;
      // The applier's derivation, bit for bit the exact route's: E in Pa
      // through the unit catalog, then E / (2 × (1 + ν)); stored in Pa.
      const derived = JSON.parse(derivedG(eValue.value, eValue.unit, nuValue)) as { value?: number; unit?: string; error?: string };
      if (derived.error || derived.value === undefined) refuse("LIBRARY-APPLY-MEMBER-MISSING", `G cannot be derived from E and ν (${derived.error ?? "no value"}). Check the record's E and ν values, or choose a record that provides G.`);
      members.shear_modulus = { value: derived.value, unit: "Pa" };
      notes.push(
        `shear_modulus: derived from E and ν (isotropic), G = E/(2(1+ν)) with E in Pa, from elastic_modulus ${e!.source} ` +
          `(${eValue.value} ${eValue.unit}) and poisson_ratio ${nu!.source} (${nuValue} 1).`
      );
    }
  } else {
    if (nu) refuse("LIBRARY-APPLY-NU-NOT-DISCARDED", `The record provides ν (slot ${nu.source}), and E/G mode would discard it. Choose E/ν mode.`);
    if (storedNu) refuse("LIBRARY-APPLY-NU-NOT-DISCARDED", `The record stores ν but it cannot be copied (${describeStored(storedNu)}), and ν is never discarded. Fix or re-import the record's ν, or choose a different record.`);
    if (!g && storedG) refuse("LIBRARY-APPLY-MEMBER-BLOCKED", `The record stores G but it cannot be copied (${describeStored(storedG)}). Fix or re-import the record's G, or choose a different record.`);
    if (!g) refuse("LIBRARY-APPLY-MEMBER-MISSING", "E/G mode needs shear_modulus (G), and the record has none. Choose E/ν mode if the record provides ν, or choose a record that provides G.");
    members.shear_modulus = g!.value;
    used.push(g!);
  }
  if (alpha) { members.thermal_expansion_coefficient = alpha.value; used.push(alpha); }
  if (!members.label) delete members.label;
  return { members, copies: used, notCopied, notes };
}

const MATERIAL_GOVERNED = ["elastic_modulus", "shear_modulus", "poisson_ratio", "constitutive_basis", "thermal_expansion_coefficient", "temperature_points", "expansion_laws"];

function copiedRows(copies: Copy[], members: Record<string, unknown>): CopiedRow[] {
  const rows: CopiedRow[] = copies.map((copy) => ({ member: copy.member, value: copy.display, unit: copy.unit, source: copy.source }));
  const g = obj(members.shear_modulus) as { value: number; unit: string } | undefined;
  if (g && !copies.some((copy) => copy.member === "shear_modulus")) {
    rows.push({ member: "shear_modulus", value: String(g.value), unit: g.unit, source: "derived from E and ν (isotropic)" });
  }
  return rows;
}

function batch(kind: string, operations: EditorOperationIntent[]): OperationBatch {
  return { batch_id: `batch:library-${kind}:${crypto.randomUUID()}`, operations };
}

function changeBase(field_label: string, source_note: string) {
  return { change_id: `change:${crypto.randomUUID()}`, field_label, source_note };
}

const RATIONALE = "User-initiated application of a stored private library record; copied values, units and combined provenance only.";

export type InsertMaterialInput = { model: PreviewModel; document: Json; recordId: string; newId: string; mode: MaterialMode; catalog: UnitCatalog | null };

export async function buildMaterialInsert(input: InsertMaterialInput): Promise<LibraryApplyPlan> {
  const ctx = await context("material", input.document, input.recordId);
  checkNewId(input.model, input.newId);
  const result = materialMembers(ctx, input.catalog, input.mode, (await loadWasmEngine()).derivedShearModulusJson);
  const provenance = libraryProvenanceString("material", ctx.libraryId, ctx.recordId);
  const record = provenanceRecord(ctx, result.copies, null, result.notes);
  const payload = { id: input.newId, ...result.members, label: result.members.label ?? input.newId, provenance, provenance_record: record };
  const e = result.members.elastic_modulus as { unit: string };
  const operation = draftIntent({
    operation_kind: "create",
    target: { object_type: "Material", ref: input.newId },
    change: { ...changeBase("Insert library material", provenance), change_kind: "create_material", field_path: "materials", before: "not_present", after: await canonicalJsonString(payload), unit: e.unit, dimension: "stress" },
    rationale: RATIONALE
  });
  return { batch: batch("material", [operation]), copied: copiedRows(result.copies, result.members), notCopied: result.notCopied, removed: [], dependentPipes: [], provenanceRecord: record };
}

export type ApplyMaterialInput = { model: PreviewModel; document: Json; recordId: string; targetId: string; mode: MaterialMode; replace: ReplaceOption; catalog: UnitCatalog | null };

export function defaultMaterialMode(material: Pick<Material, "constitutive_basis" | "poisson_ratio"> | undefined): MaterialMode {
  return material?.constitutive_basis || material?.poisson_ratio ? "e_nu" : "e_g";
}

export async function buildMaterialApply(input: ApplyMaterialInput): Promise<LibraryApplyPlan> {
  const ctx = await context("material", input.document, input.recordId);
  const target = input.model.materials?.find((material) => material.id === input.targetId);
  if (!target) refuse("LIBRARY-APPLY-TARGET-MISSING", `Material ${input.targetId} is not in the model. Select a material in the model and prepare the apply again.`);
  const current = target as unknown as Json;
  // ν is never discarded: E/G does not apply onto a material that holds ν (a
  // basis change stays possible through ordinary material editing).
  if (input.mode === "e_g" && current.poisson_ratio !== undefined) {
    refuse("LIBRARY-APPLY-NU-NOT-DISCARDED", `This material already has poisson_ratio (ν = ${shown((current.poisson_ratio as { value?: unknown } | undefined)?.value)}). Apply a record that provides ν in E/ν mode, or insert this record as a new material in E/G mode.`);
  }
  const result = materialMembers(ctx, input.catalog, input.mode, (await loadWasmEngine()).derivedShearModulusJson);
  const after: Record<string, unknown> = { ...result.members };
  // E/G on a material carrying the exact basis without ν removes the basis.
  if (input.mode === "e_g" && current.constitutive_basis !== undefined) after.constitutive_basis = null;
  const unwritten = MATERIAL_GOVERNED.filter((member) => current[member] !== undefined && !(member in after));
  const removed = Object.keys(after).filter((member) => after[member] === null);
  if (input.replace === "all") for (const member of unwritten) { after[member] = null; removed.push(member); }
  const retained = input.replace === "all" ? [] : unwritten;
  return applyPlan(ctx, input.model, "Material", input.targetId, current, after, result, retained, removed, []);
}

async function applyPlan(
  ctx: Context, model: PreviewModel, objectType: "Material" | "Section" | "Component", targetId: string, current: Json,
  after: Record<string, unknown>, result: { copies: Copy[]; notCopied: NotCopiedRow[]; notes: string[]; members: Record<string, unknown> },
  retained: string[], removed: string[], dependentPipes: string[], replaceAllClears = true
): Promise<LibraryApplyPlan> {
  const record = provenanceRecord(ctx, result.copies, { record: current.provenance_record as ProvenanceRecord | undefined, retained, replaceAllClears }, result.notes);
  const provenance = libraryProvenanceString(ctx.kind, ctx.libraryId, ctx.recordId);
  after.provenance = provenance;
  after.provenance_record = record;
  const before: Record<string, unknown> = {};
  for (const path of Object.keys(after)) {
    const value = path.split(".").reduce<unknown>((value, segment) => obj(value)?.[segment], current);
    if (value !== undefined) before[path] = value;
  }
  const operation = draftIntent({
    operation_kind: "modify",
    target: { object_type: objectType, ref: targetId },
    change: { ...changeBase(`Apply library ${ctx.kind} record`, provenance), change_kind: "apply_library_record", field_path: "library_record", before: await canonicalJsonString(before), after: await canonicalJsonString(after), unit: "none", dimension: "dimensionless" },
    rationale: RATIONALE
  });
  void model;
  return { batch: batch(ctx.kind, [operation]), copied: copiedRows(result.copies, result.members), notCopied: result.notCopied, removed, dependentPipes, provenanceRecord: record };
}

function checkNewId(model: PreviewModel, id: string) {
  if (!/^[A-Za-z][A-Za-z0-9_.:-]*$/.test(id)) refuse("LIBRARY-APPLY-ID-INVALID", `${JSON.stringify(id)} is not a valid model id. Enter an id that starts with a letter and uses only letters, digits, _ . : or -.`);
  const taken = [...(model.materials ?? []), ...(model.sections ?? []), ...model.nodes, ...model.pipe_segments, ...model.supports, ...model.components]
    .some((entity) => entity.id === id);
  if (taken) refuse("LIBRARY-APPLY-ID-INVALID", `${id} is already used in the model. Enter an id the model does not use yet.`);
}

// ---- section (§2.3) --------------------------------------------------------

function sectionValues(ctx: Context): { copies: Map<string, Copy>; notCopied: NotCopiedRow[] } {
  if (ctx.record.section_type !== "pipe") refuse("LIBRARY-APPLY-SECTION-TYPE", `Only pipe sections apply, and this record is ${shown(ctx.record.section_type)}. Choose a record whose section_type is pipe.`);
  const copies = new Map<string, Copy>();
  const notCopied: NotCopiedRow[] = [];
  for (const slot of (Array.isArray(ctx.record.dimensions) ? (ctx.record.dimensions as unknown[]) : []).map(obj)) {
    if (!slot) continue;
    const kind = str(slot.dimension_kind) ?? "TBD";
    const source = str(slot.dimension_id) ?? kind;
    if (kind !== "outside_diameter" && kind !== "wall_thickness") {
      notCopied.push({ source, reason: "allowances, tolerances and insulation are pipe fields; other dimensions have no section slot" });
      continue;
    }
    const value = obj(slot.value);
    const blocked = [...blockedTokens(slot), ...blockedTokens(value)];
    if (blocked.length) { notCopied.push({ source, reason: `excluded: ${[...new Set(blocked)].join(", ")}` }); continue; }
    const status = str(value?.value_status) ?? str(slot.value_status) ?? "missing";
    if (!value || !SLOT_ELIGIBLE.has(str(slot.value_status) ?? "") || !SLOT_ELIGIBLE.has(status)) {
      notCopied.push({ source, reason: `value status ${status} is not copied` });
      continue;
    }
    if (copies.has(kind)) refuse("LIBRARY-APPLY-AMBIGUOUS-SOURCE", `The record holds more than one copyable ${kind}, so the source is ambiguous. Keep one ${kind} in the record and re-import the library, or choose a different record.`);
    const number = magnitude(value.magnitude, source);
    const unit = str(value.unit) ?? "";
    copies.set(kind, { member: `properties.${kind}`, value: { value: number, unit }, unit, source, levels: [slot, value], display: String(number) });
  }
  for (const slot of (Array.isArray(ctx.record.properties) ? (ctx.record.properties as unknown[]) : []).map(obj)) {
    if (slot) notCopied.push({ source: str(slot.property_id) ?? "property", reason: "a bound section holds only outside_diameter and wall_thickness" });
  }
  return { copies, notCopied };
}

export type InsertSectionInput = { model: PreviewModel; document: Json; recordId: string; newId: string };

export async function buildSectionInsert(input: InsertSectionInput): Promise<LibraryApplyPlan> {
  const ctx = await context("section", input.document, input.recordId);
  checkNewId(input.model, input.newId);
  const { copies, notCopied } = sectionValues(ctx);
  for (const kind of ["outside_diameter", "wall_thickness"]) if (!copies.has(kind)) refuse("LIBRARY-APPLY-MEMBER-MISSING", `A new section needs ${kind}, and the record has no copyable value for it. Choose a record that provides it, or apply this record to an existing section instead.`);
  const used = [...copies.values()];
  const provenance = libraryProvenanceString("section", ctx.libraryId, ctx.recordId);
  const record = provenanceRecord(ctx, used, null, []);
  const od = copies.get("outside_diameter")!.value as { unit: string };
  const payload = {
    id: input.newId, name: str(ctx.record.name) ?? input.newId, section_type: "pipe",
    properties: { outside_diameter: copies.get("outside_diameter")!.value, wall_thickness: copies.get("wall_thickness")!.value },
    provenance, provenance_record: record
  };
  const operation = draftIntent({
    operation_kind: "create",
    target: { object_type: "Section", ref: input.newId },
    change: { ...changeBase("Insert library section", provenance), change_kind: "create_section", field_path: "sections", before: "not_present", after: await canonicalJsonString(payload), unit: od.unit, dimension: "length" },
    rationale: RATIONALE
  });
  return { batch: batch("section", [operation]), copied: copiedRows(used, {}), notCopied, removed: [], dependentPipes: [], provenanceRecord: record };
}

export type ApplySectionInput = { model: PreviewModel; document: Json; recordId: string; targetId: string; replace: ReplaceOption };

export async function buildSectionApply(input: ApplySectionInput): Promise<LibraryApplyPlan> {
  const ctx = await context("section", input.document, input.recordId);
  const target = input.model.sections?.find((section) => section.id === input.targetId);
  if (!target) refuse("LIBRARY-APPLY-TARGET-MISSING", `Section ${input.targetId} is not in the model. Select a section in the model and prepare the apply again.`);
  const { copies, notCopied } = sectionValues(ctx);
  const used = [...copies.values()];
  if (!used.length) refuse("LIBRARY-APPLY-NOTHING-COPIED", "The record has no copyable outside_diameter or wall_thickness, so nothing would change. Choose a record that provides them, or add them to this record and re-import the library.");
  const after: Record<string, unknown> = { name: str(ctx.record.name) ?? target!.name };
  for (const copy of used) after[copy.member] = copy.value;
  const retained = ["outside_diameter", "wall_thickness"].filter((kind) => !copies.has(kind) && target!.properties[kind] !== undefined).map((kind) => `properties.${kind}`);
  if (input.replace === "all" && retained.length) refuse("LIBRARY-APPLY-REQUIRED-MEMBER", `A pipe section requires ${retained.join(", ")}, and the record does not supply it, so "replace all values" cannot remove it. Choose "keep them", or a record that supplies both dimensions.`);
  const dependents = input.model.pipe_segments.filter((pipe) => pipe.section_ref === input.targetId).map((pipe) => pipe.id).sort();
  return applyPlan(ctx, input.model, "Section", input.targetId, target as unknown as Json, after, { copies: used, notCopied, notes: [], members: {} }, retained, [], dependents, false);
}

// ---- component (§2.3b) ----------------------------------------------------

type Group = "bend" | "branch" | "rigid" | "expansion" | "modifiers";
const FAMILY: Record<string, Exclude<Group, "modifiers">> = {
  bend: "bend", elbow: "bend", branch: "branch", tee: "branch", branch_connection: "branch",
  reducer: "rigid", valve: "rigid", flange: "rigid", rigid: "rigid", specialty: "rigid", expansion_joint: "expansion"
};
/** Source field_kind → destination member, its group, and the families it belongs to. */
const COMPONENT_TABLE: Record<string, { member: string; group: Group; families: Array<Exclude<Group, "modifiers">> }> = {
  bend_radius: { member: "geometry.bend_radius", group: "bend", families: ["bend"] },
  bend_angle: { member: "geometry.bend_angle", group: "bend", families: ["bend"] },
  branch_run_size: { member: "geometry.branch_run_size", group: "branch", families: ["branch"] },
  branch_header_size: { member: "geometry.branch_header_size", group: "branch", families: ["branch"] },
  branch_connection_angle: { member: "geometry.branch_connection_angle", group: "branch", families: ["branch"] },
  branch_reinforcement_area: { member: "geometry.branch_reinforcement_area", group: "branch", families: ["branch"] },
  rigid_body_length: { member: "geometry.rigid_body_length", group: "rigid", families: ["rigid"] },
  end_a_size: { member: "geometry.end_a_size", group: "rigid", families: ["rigid"] },
  end_b_size: { member: "geometry.end_b_size", group: "rigid", families: ["rigid"] },
  weight: { member: "geometry.weight", group: "rigid", families: ["rigid"] },
  effective_area: { member: "geometry.effective_area", group: "expansion", families: ["expansion"] },
  movement_limit: { member: "geometry.movement_limit", group: "expansion", families: ["expansion"] },
  sif_user_value: { member: "modifiers.sif_user_value", group: "modifiers", families: ["bend"] },
  flexibility_factor_user_value: { member: "modifiers.flexibility_factor_user_value", group: "modifiers", families: ["bend", "branch"] },
  sif_in_plane_user_value: { member: "modifiers.sif_in_plane_user_value", group: "modifiers", families: ["bend"] },
  sif_out_of_plane_user_value: { member: "modifiers.sif_out_of_plane_user_value", group: "modifiers", families: ["bend"] },
  linear_stiffness: { member: "modifiers.linear_stiffness_user_value", group: "modifiers", families: ["rigid"] },
  rotational_stiffness: { member: "modifiers.rotational_stiffness_user_value", group: "modifiers", families: ["rigid"] }
};
const COMPONENT_EXCLUDED: Record<string, string> = {
  center_of_gravity: "the library holds a scalar; the model holds a vector {x, y, z, unit}",
  outside_diameter: "a pipe-section field",
  wall_thickness: "a pipe-section field"
};
const STRING_SLOT_REASON = "the library's form is a typed Reference or a scalar, while the model's is free text of another meaning";
const NO_SLOT_REASON = "no model slot";
const SOURCE_STRING: Record<Group, string> = {
  bend: "geometry.bend_geometry_source_reference",
  branch: "geometry.branch_geometry_source_reference",
  rigid: "geometry.rigid_component_source_reference",
  expansion: "geometry.expansion_joint_source_reference",
  modifiers: "modifiers.source_reference"
};
/** Replaceable members: exactly the table's destinations in each group. */
const REPLACEABLE: Record<Group, string[]> = {
  bend: ["geometry.bend_radius", "geometry.bend_angle"],
  branch: ["geometry.branch_run_size", "geometry.branch_header_size", "geometry.branch_connection_angle", "geometry.branch_reinforcement_area"],
  rigid: ["geometry.rigid_body_length", "geometry.end_a_size", "geometry.end_b_size", "geometry.weight"],
  expansion: ["geometry.effective_area", "geometry.movement_limit"],
  modifiers: ["modifiers.sif_user_value", "modifiers.flexibility_factor_user_value", "modifiers.sif_in_plane_user_value", "modifiers.sif_out_of_plane_user_value", "modifiers.linear_stiffness_user_value", "modifiers.rotational_stiffness_user_value"]
};
/** Values of each group, including non-replaceable ones such as center_of_gravity. */
const GROUP_VALUES: Record<Exclude<Group, "modifiers">, string[]> = {
  bend: REPLACEABLE.bend,
  branch: REPLACEABLE.branch,
  rigid: [...REPLACEABLE.rigid, "geometry.center_of_gravity"],
  expansion: REPLACEABLE.expansion
};

function requiredMembers(family: Exclude<Group, "modifiers">, realized: boolean): string[] {
  switch (family) {
    case "bend": return ["geometry.bend_radius", ...(realized ? [] : ["geometry.bend_angle"]), "modifiers.sif_user_value", "modifiers.flexibility_factor_user_value"];
    case "branch": return ["geometry.branch_run_size", "geometry.branch_header_size", "geometry.branch_connection_angle", "modifiers.flexibility_factor_user_value"];
    case "rigid": return ["geometry.rigid_body_length", "geometry.end_a_size", "geometry.end_b_size", "geometry.weight", "modifiers.linear_stiffness_user_value", "modifiers.rotational_stiffness_user_value"];
    default: return [];
  }
}

function recordMatchesKind(componentType: string, kind: string): boolean {
  if (componentType === "branch") return ["branch", "tee", "branch_connection"].includes(kind);
  return componentType === kind;
}

function at(entity: Json, path: string): unknown {
  return path.split(".").reduce<unknown>((value, segment) => obj(value)?.[segment], entity);
}

function inGroup(group: Group, member: string): boolean {
  return group === "modifiers" ? member.startsWith("modifiers.") && member !== "modifiers.source_reference" : GROUP_VALUES[group].includes(member);
}

/** The values a group of the entity holds now. */
function groupValues(entity: Json, group: Group): string[] {
  const paths = group === "modifiers"
    ? Object.keys(obj(entity.modifiers) ?? {}).map((key) => `modifiers.${key}`)
    : GROUP_VALUES[group];
  return paths.filter((path) => inGroup(group, path) && at(entity, path) !== undefined);
}

export type ApplyComponentInput = { model: PreviewModel; document: Json; recordId: string; targetId: string; replace: ReplaceOption };

export async function buildComponentApply(input: ApplyComponentInput): Promise<LibraryApplyPlan> {
  const ctx = await context("component", input.document, input.recordId);
  const target = input.model.components.find((component) => component.id === input.targetId);
  if (!target) refuse("LIBRARY-APPLY-TARGET-MISSING", `Component ${input.targetId} is not in the model. Select a component in the model and prepare the apply again.`);
  const entity = target as unknown as Json;
  const componentType = str(ctx.record.component_type) ?? "TBD";
  if (!recordMatchesKind(componentType, target!.kind)) {
    refuse("LIBRARY-APPLY-KIND-MISMATCH", `A ${componentType} record does not apply to a ${target!.kind} component. Choose a ${target!.kind} record, or select a ${componentType} component.`);
  }
  const family = FAMILY[target!.kind];
  if (!family) refuse("LIBRARY-APPLY-KIND-MISMATCH", `A ${target!.kind} component takes no library values. Select a component of kind bend, elbow, branch, tee, branch_connection, reducer, valve, flange, rigid, specialty or expansion_joint whose kind matches the record's component_type (${componentType}), or choose a record of one of those kinds.`);
  const realized = ["bend", "elbow"].includes(target!.kind) && target!.mechanics_interface?.solver_consumption === "curved_bend_macro_element";

  const copies = new Map<string, Copy>();
  const notCopied: NotCopiedRow[] = [];
  for (const slot of (Array.isArray(ctx.record.fields) ? (ctx.record.fields as unknown[]) : []).map(obj)) {
    if (!slot) continue;
    const kind = str(slot.field_kind) ?? "TBD";
    const source = str(slot.field_id) ?? kind;
    const row = COMPONENT_TABLE[kind];
    if (!row) {
      const reason = COMPONENT_EXCLUDED[kind] ?? (kind.endsWith("_reference") || ["bend_plane_orientation", "branch_connection_type"].includes(kind) ? STRING_SLOT_REASON : NO_SLOT_REASON);
      notCopied.push({ source, reason });
      continue;
    }
    if (!row.families.includes(family!)) { notCopied.push({ source, reason: `not a member of a ${target!.kind} component` }); continue; }
    const value = obj(slot.value);
    const blocked = [...blockedTokens(slot), ...blockedTokens(value)];
    if (blocked.length) { notCopied.push({ source, reason: `excluded: ${[...new Set(blocked)].join(", ")}` }); continue; }
    const status = str(value?.value_status) ?? "missing";
    if (!value || !SLOT_ELIGIBLE.has(str(slot.value_status) ?? "") || !SLOT_ELIGIBLE.has(status)) {
      notCopied.push({ source, reason: `value status ${!value ? str(slot.value_status) ?? "missing" : status} is not copied` });
      continue;
    }
    if (copies.has(kind)) refuse("LIBRARY-APPLY-AMBIGUOUS-SOURCE", `The record holds more than one copyable ${kind}, so the source is ambiguous. Keep one ${kind} in the record and re-import the library, or choose a different record.`);
    const number = magnitude(value.magnitude, source);
    const unit = str(value.unit) ?? "";
    copies.set(kind, { member: row.member, value: { value: number, unit }, unit, source, levels: [slot, value], display: String(number) });
  }
  // The directional SIF pair: both or neither, on a realized curved bend only.
  const pair = ["sif_in_plane_user_value", "sif_out_of_plane_user_value"];
  const pairPresent = pair.filter((kind) => copies.has(kind));
  if (pairPresent.length && (!realized || pairPresent.length !== 2)) {
    const reason = !realized
      ? "the directional SIF pair applies only to a bend realized as the curved-bend macro element (BEND_DIRECTIONAL_SIF_NOT_REALIZED_ARC)"
      : "the directional SIF pair copies only together";
    for (const kind of pairPresent) { notCopied.push({ source: copies.get(kind)!.source, reason }); copies.delete(kind); }
  }

  const used = [...copies.values()];
  if (!used.length) refuse("LIBRARY-APPLY-NOTHING-COPIED", "The record supplies no value this component can take, so nothing would change. Choose a record that provides a value from the copy table for this component, or add one to this record and re-import the library.");
  const written = new Set(used.map((copy) => copy.member));
  const after: Record<string, unknown> = {};
  for (const copy of used) after[copy.member] = copy.value;
  const required = new Set(requiredMembers(family!, realized));
  const groupsWritten = [...new Set(used.map((copy) => Object.values(COMPONENT_TABLE).find((row) => row.member === copy.member)!.group))];
  const removed: string[] = [];
  const removalGroups: Group[] = input.replace === "group" ? groupsWritten : input.replace === "all" ? [family!, "modifiers"] : [];
  for (const group of removalGroups) {
    for (const member of REPLACEABLE[group]) {
      if (written.has(member) || at(entity, member) === undefined) continue;
      if (required.has(member)) refuse("LIBRARY-APPLY-REQUIRED-MEMBER", `${member} is required for a ${target!.kind}, and the record does not supply it, so it is not removed. Choose "keep them", or a record that supplies ${member}.`);
      after[member] = null;
      removed.push(member);
    }
  }
  // Source-reference strings: rewritten for every group the apply writes.
  const libraryString = libraryProvenanceString("component", ctx.libraryId, ctx.recordId);
  for (const group of groupsWritten) {
    const retainedInGroup = groupValues(entity, group).filter((member) => !written.has(member) && !removed.includes(member));
    const writtenInGroup = used.filter((copy) => inGroup(group, copy.member)).map((copy) => copy.member.split(".")[1]).sort();
    const previous = str(at(entity, SOURCE_STRING[group]))?.trim();
    if (!retainedInGroup.length) after[SOURCE_STRING[group]] = libraryString;
    else if (previous) after[SOURCE_STRING[group]] = `${previous}; ${libraryString}: ${writtenInGroup.join(", ")}`;
    else refuse("LIBRARY-APPLY-RETAINED-UNSOURCED", `${retainedInGroup.join(", ")} would keep values that have no source reference. Enter the group's source reference on the component first, then apply again.`);
  }
  // Governed values the entity keeps (every unwritten quantity of its groups).
  const retained = [...new Set([...groupValues(entity, family!), ...groupValues(entity, "modifiers")])]
    .filter((member) => !written.has(member) && !removed.includes(member));
  const replaceable = new Set([...REPLACEABLE[family!], ...REPLACEABLE.modifiers]);
  const replaceAllClears = retained.every((member) => replaceable.has(member) && !required.has(member));
  return applyPlan(ctx, input.model, "Component", input.targetId, entity, after, { copies: used, notCopied, notes: [], members: {} }, retained, removed, [], replaceAllClears);
}

// ---- hanger (§2.6: the hanger path moves to the same rule) ----------------

/** The short provenance string and combined `provenance_record` of a hanger
 * selection: library metadata, record and every copied quantity's own
 * provenance; no library payload. */
export async function hangerLibraryProvenance(library: Json, record: Json, copiedQuantities: Json[]): Promise<{ provenance: string; provenanceRecord: ProvenanceRecord }> {
  const ctx: Context = {
    kind: "hanger",
    libraryId: str(library.library_id) ?? "",
    recordId: str(record.hanger_id) ?? "",
    library,
    record,
    version: await libraryRecordVersion(record)
  };
  for (const [label, level] of [["library", library], ["record", record]] as const) {
    const blocked = blockedTokens(level);
    if (blocked.length) refuse("LIBRARY-APPLY-SOURCE-REJECTED", `The hanger ${label} is marked ${blocked.join(", ")}, so its data is not applied. Choose a different record, or re-import the library with reviewed source dispositions.`);
  }
  for (const quantity of copiedQuantities) {
    const blocked = blockedTokens(quantity);
    if (blocked.length) refuse("LIBRARY-APPLY-SOURCE-REJECTED", `A hanger quantity in this record is marked ${[...new Set(blocked)].join(", ")}, so the record is not applied. Choose a different record, or re-import the library with reviewed source dispositions.`);
  }
  const copies: Copy[] = copiedQuantities.map((quantity, index) => ({ member: `hanger.${index}`, value: quantity, unit: str(quantity.unit) ?? "", source: ctx.recordId, levels: [quantity], display: "" }));
  return { provenance: libraryProvenanceString("hanger", ctx.libraryId, ctx.recordId), provenanceRecord: provenanceRecord(ctx, copies, null, []) };
}
