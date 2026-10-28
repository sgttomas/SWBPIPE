// Run-sheet entry, slice S1 (DEL-07-02 Design `run-sheet-entry.md` §3.3): the
// row-to-operation mapping and the record projection. Its only input is the
// resolver's output (`runSheetModel.ts`). It emits ordinary operations of the
// existing vocabulary and computes no number: coordinates are the resolver's
// normalized values (§6.2), and tangent points are the engine's.
//
// The record projection is the basis model plus the records this batch's own
// earlier steps write, by id and verbatim payload only (O10). It forms the
// `before` texts of `assign_section` and `insert_bend_at_corner`; a wrong
// projection cannot change the model, since the engine compares each `before`
// with its own state and refuses a mismatch.
import { canonicalJsonString } from "../../../services/hashService";
import type { OperationBatch, OperationBatchOutcome } from "../../../services/operationBatchService";
import type { EditorOperationIntent, OperationOutcomeDiagnostic, PreviewModel } from "../../../types";
import { MESSAGES, OPTIONAL_SECTION_COLUMNS, QUANTITY_COLUMNS, normalizeZero, type ResolvedRunSheet, type ResolvedRunSheetRow } from "./runSheetModel";

export const RUN_SHEET_SOURCE = {
  source_ref: "apps/desktop/src/features/model-tree/runSheet/runSheetBatch.ts",
  source_channel: "local_desktop_preview",
  source_role: "gui_editor",
} as const;
/** The form's labels for a bend's records (§3.2, `InsertBendAtCornerForm`). */
export const BEND_LABELS = { start: "Bend start", end: "Bend end", arc: "Bend arc", component: "Bend" } as const;

type NodeRecord = PreviewModel["nodes"][number];
type PipeRecord = PreviewModel["pipe_segments"][number];

export type RunSheetRowBuild = {
  key: string;
  operationIds: string[];
  /** §3.4 ownership: the ids this row's operations leave in the model. Never a node the row did not create. */
  ownedIds: string[];
};
export type RunSheetBuild = { batch: OperationBatch; rows: RunSheetRowBuild[]; projection: PreviewModel };

/** The canonical corner context `insert_bend_at_corner` names (`bend_authoring.rs`
 * `corner_context`; TS `cornerContext`): the corner, its pipes in model order and
 * the bend markers at it. */
export function projectedCornerContext(model: PreviewModel, corner: string) {
  return {
    node: model.nodes.find((node) => node.id === corner) ?? null,
    pipes: model.pipe_segments.filter((pipe) => pipe.from === corner || pipe.to === corner),
    markers: model.components.filter((component) => (component.kind === "bend" || component.kind === "elbow") && component.node === corner),
  };
}

const clone = <T,>(value: T): T => JSON.parse(JSON.stringify(value)) as T;

function intent(input: {
  rowKey: string; index: number; operationKind: EditorOperationIntent["operation_kind"];
  objectType: EditorOperationIntent["target"]["object_type"]; ref: string;
  changeKind: string; fieldLabel: string; fieldPath: string; before: string; after: unknown; unit: string; dimension: string;
}): EditorOperationIntent {
  return {
    operation_id: `op:run-sheet:${input.rowKey}:${input.index}`,
    operation_kind: input.operationKind,
    operation_status: "proposed",
    author_type: "user",
    source: { ...RUN_SHEET_SOURCE },
    target: { object_type: input.objectType, ref: input.ref },
    change: {
      change_id: `change:run-sheet:${input.rowKey}:${input.index}`,
      change_kind: input.changeKind,
      field_label: input.fieldLabel,
      field_path: input.fieldPath,
      before: input.before,
      after: JSON.stringify(input.after),
      unit: input.unit,
      dimension: input.dimension,
      source_note: "Run-sheet row values as entered or shown inherited; no value is suggested or defaulted.",
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation: "not_run",
      diff_preview_status: "not_generated",
      application_status: "not_applied",
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false,
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false,
    },
    rationale: "Run-sheet entry: explicit user rows mapped to structured operations; requires review before apply.",
  } as EditorOperationIntent;
}

/** The node record `create_node` writes, as the applier stores it. The resolver
 * already normalized −0 (§6.2), so the displayed value is the emitted one; the
 * normalization is repeated here so no −0 can reach a payload or the projection. */
export function nodeRecord(row: ResolvedRunSheetRow, source: string): NodeRecord {
  const { x, y, z } = row.end!.position;
  return { id: row.ids.node!, label: row.label!.value, position: { x: normalizeZero(x), y: normalizeZero(y), z: normalizeZero(z) }, provenance: source };
}

/** The pipe record `connect_pipe_run` writes, as the applier stores it (no `section_ref`). */
export function pipeRecord(row: ResolvedRunSheetRow, source: string): PipeRecord {
  const cells = row.cells;
  const section: PipeRecord["section"] = {
    outside_diameter: { ...cells.od.value! },
    wall_thickness: { ...cells.wall.value! },
  };
  for (const column of OPTIONAL_SECTION_COLUMNS) {
    const value = cells[column].value;
    if (value !== undefined && !cells[column].absent) section[QUANTITY_COLUMNS[column].member!] = { ...value };
  }
  const y = cells.orientation.value!;
  return {
    id: row.ids.pipe!,
    label: row.labels.pipe!,
    from: row.start!,
    to: row.end!.id,
    section,
    material: cells.material.value!,
    y_reference: { x: y.x, y: y.y, z: y.z },
    provenance: source,
  };
}

/** Build the one batch for the resolved rows (§3.3). Refuses a sheet the rules refuse. */
export async function buildRunSheetBatch(resolved: ResolvedRunSheet): Promise<RunSheetBuild> {
  if (!resolved.queueable) throw new Error("The run sheet has refused rows, sheet-level findings or pending conversions; nothing is built.");
  const source = resolved.source;
  const projection = clone(resolved.basis);
  const operations: EditorOperationIntent[] = [];
  const rows: RunSheetRowBuild[] = [];
  const counters = new Map<string, number>();
  const push = (row: ResolvedRunSheetRow, made: Omit<Parameters<typeof intent>[0], "rowKey" | "index">) => {
    const index = counters.get(row.key) ?? 0;
    counters.set(row.key, index + 1);
    const op = intent({ ...made, rowKey: row.key, index });
    operations.push(op);
    rows.find((item) => item.key === row.key)!.operationIds.push(op.operation_id);
  };

  const createNode = (row: ResolvedRunSheetRow) => {
    const record = nodeRecord(row, source);
    push(row, { operationKind: "create", objectType: "Node", ref: record.id, changeKind: "create_node", fieldLabel: "Run-sheet end node", fieldPath: "nodes", before: "not_present", after: record, unit: resolved.storedLengthUnit, dimension: "length" });
    projection.nodes.push(clone(record));
  };
  const createSupport = (row: ResolvedRunSheetRow, node: string) => {
    const support = row.support!;
    const after = { id: row.ids.support!, label: row.labels.support!, node, restraints: [...support.restraints], family: support.family, provenance: source };
    push(row, { operationKind: "create", objectType: "Support", ref: after.id, changeKind: "create_support", fieldLabel: "Run-sheet support", fieldPath: "supports", before: "not_present", after, unit: "none", dimension: "dimensionless" });
  };
  const connect = async (row: ResolvedRunSheetRow) => {
    const record = pipeRecord(row, source);
    push(row, { operationKind: "connect", objectType: "Element", ref: record.id, changeKind: "connect_pipe_run", fieldLabel: "Run-sheet pipe", fieldPath: "pipe_segments", before: "not_present", after: record, unit: record.section.outside_diameter.unit, dimension: "length" });
    projection.pipe_segments.push(clone(record));
    const sectionId = row.cells.section.value;
    if (sectionId !== undefined) {
      const stored = projection.pipe_segments.find((pipe) => pipe.id === record.id)!;
      const before = await canonicalJsonString(stored);
      push(row, { operationKind: "modify", objectType: "Element", ref: record.id, changeKind: "assign_section", fieldLabel: "Run-sheet shared section", fieldPath: "section_ref", before, after: { section_ref: sectionId }, unit: "none", dimension: "dimensionless" });
      // `assign_section` adds `section_ref` and copies the section's OD and wall verbatim.
      const section = (projection.sections ?? []).find((item) => item.id === sectionId)!;
      stored.section_ref = sectionId;
      stored.section.outside_diameter = clone(section.properties.outside_diameter!);
      stored.section.wall_thickness = clone(section.properties.wall_thickness!);
    }
  };
  const insertBend = async (row: ResolvedRunSheetRow) => {
    const corner = row.end!.id;
    const bend = row.bend!;
    const ids = row.ids.bend!;
    const context = projectedCornerContext(projection, corner);
    const before = await canonicalJsonString(context);
    const after = {
      radius: { value: bend.radius.value, unit: bend.radius.unit },
      flexibility_factor: { value: bend.k, unit: "none" },
      flexibility_source_reference: bend.kSource,
      new_nodes: [
        { id: ids.start, label: BEND_LABELS.start, provenance: source },
        { id: ids.end, label: BEND_LABELS.end, provenance: source },
      ],
      new_pipe: { id: ids.arc, label: BEND_LABELS.arc, provenance: source },
      component: { id: ids.component, label: BEND_LABELS.component, provenance: source, geometry_source_reference: source },
    };
    push(row, { operationKind: "modify", objectType: "Node", ref: corner, changeKind: "insert_bend_at_corner", fieldLabel: "Run-sheet bend at corner", fieldPath: "bend_at_corner", before, after, unit: "none", dimension: "dimensionless" });
    // The corner leaves `nodes`; the first pipe in model order now ends at T1 and the second at T2.
    projection.nodes = projection.nodes.filter((node) => node.id !== corner);
    context.pipes.forEach((meeting, order) => {
      const pipe = projection.pipe_segments.find((item) => item.id === meeting.id)!;
      const replacement = order === 0 ? ids.start : ids.end;
      if (pipe.from === corner) pipe.from = replacement;
      if (pipe.to === corner) pipe.to = replacement;
    });
  };

  let pendingBend: ResolvedRunSheetRow | null = null;
  for (const row of resolved.rows) {
    const built: RunSheetRowBuild = { key: row.key, operationIds: [], ownedIds: [] };
    rows.push(built);
    switch (row.kind) {
      case "run":
      case "bend": {
        if (!row.end!.existing) {
          createNode(row);
          if (row.kind === "run") built.ownedIds.push(row.ids.node!);
        }
        await connect(row);
        built.ownedIds.push(row.ids.pipe!);
        if (row.support && row.ids.support) {
          createSupport(row, row.end!.id);
          built.ownedIds.push(row.ids.support!);
        }
        break;
      }
      case "support":
        // The resolver refuses an S row without a support; a support-less row emits nothing.
        if (row.support && row.ids.support && row.start) {
          createSupport(row, row.start);
          built.ownedIds.push(row.ids.support);
        }
        break;
      case "from":
        if (row.support && row.ids.support) {
          createSupport(row, row.end!.id);
          built.ownedIds.push(row.ids.support!);
        }
        break;
      case "point":
        createNode(row);
        built.ownedIds.push(row.ids.node!);
        if (row.support && row.ids.support) {
          createSupport(row, row.end!.id);
          built.ownedIds.push(row.ids.support!);
        }
        break;
      default:
        throw new Error(`Row ${row.number} has no kind.`);
    }
    // A bend run's insertion follows the next run row's operations (§3.3).
    if (pendingBend) {
      await insertBend(pendingBend);
      const ids = pendingBend.ids.bend!;
      rows.find((item) => item.key === pendingBend!.key)!.ownedIds.push(ids.start, ids.end, ids.arc, ids.component);
      pendingBend = null;
    }
    if (row.kind === "bend") pendingBend = row;
  }
  if (pendingBend) throw new Error(MESSAGES.bendNeedsNextRun);
  return {
    batch: { batch_id: `batch:run-sheet-${resolved.sheetKey}-${resolved.batchSeq}`, operations },
    rows,
    projection,
  };
}

/** The row an operation acts for: `op:run-sheet:⟨rowKey⟩:⟨n⟩`. */
export function rowKeyOfOperation(operationId: string): string | null {
  const match = /^op:run-sheet:(.+):(\d+)$/.exec(operationId);
  return match ? match[1] : null;
}

export type RunSheetRowCheck =
  | { status: "passed"; diagnostics: OperationOutcomeDiagnostic[] }
  | { status: "refused"; diagnostics: OperationOutcomeDiagnostic[] }
  | { status: "not_checked"; message: string };

/** §4.2 Map an engine outcome to rows through the `rowKey` in each `operation_id`.
 * The batch stops at its first blocked step, so a row with an operation the
 * engine did not reach, or a row after the refused one, is "not checked". */
export function mapOutcomeToRows(outcome: Pick<OperationBatchOutcome, "operation_outcomes">, build: RunSheetBuild): Map<string, RunSheetRowCheck> {
  const checks = new Map<string, RunSheetRowCheck>();
  const reached = new Map(outcome.operation_outcomes.map((step) => [step.operation_id, step]));
  const blocked = outcome.operation_outcomes.find((step) => step.simulation_status === "blocked");
  const blockedRow = blocked ? rowKeyOfOperation(blocked.operation_id) : null;
  const blockedIndex = blockedRow ? build.rows.findIndex((row) => row.key === blockedRow) : -1;
  build.rows.forEach((row, index) => {
    const steps = row.operationIds.map((id) => reached.get(id));
    const diagnostics = steps.flatMap((step) => step?.diagnostics ?? []);
    if (row.key === blockedRow) checks.set(row.key, { status: "refused", diagnostics });
    else if (steps.some((step) => step === undefined) || (blockedIndex >= 0 && index > blockedIndex && steps.length === 0) || (outcome.operation_outcomes.length === 0 && build.batch.operations.length > 0)) {
      checks.set(row.key, { status: "not_checked", message: MESSAGES.notChecked });
    } else checks.set(row.key, { status: "passed", diagnostics });
  });
  return checks;
}
