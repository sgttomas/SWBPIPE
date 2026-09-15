import type { EntityRef, PreviewModel, Vec3 } from "../../types";
import { entityKey, type EntityKey } from "./selectionState";

export type Bounds3 = Readonly<{
  min: Readonly<Vec3>;
  max: Readonly<Vec3>;
}>;

export type IndexedEntity = Readonly<{
  key: EntityKey;
  ref: EntityRef;
  label: string;
  record: unknown;
  parentKey: EntityKey | null;
  geometryBounds: Bounds3 | null;
  anchor: Readonly<Vec3> | null;
  geometryIssue: string | null;
}>;

export type SpatialChunk = Readonly<{
  id: number;
  bounds: Bounds3;
  entityKeys: readonly EntityKey[];
}>;

export type SectionBinding = Readonly<{
  source: "shared" | "inline";
  sectionKey: EntityKey | null;
  record: PreviewModel["pipe_segments"][number]["section"] | NonNullable<PreviewModel["sections"]>[number]["properties"];
  issue: string | null;
}>;

export type ModelIndex = Readonly<{
  generation: string;
  sessionGeneration: number;
  modelRevision: number;
  projectKey: EntityKey;
  entities: ReadonlyMap<EntityKey, IndexedEntity>;
  labels: ReadonlyMap<EntityKey, string>;
  treeOrder: readonly EntityKey[];
  treeChildren: ReadonlyMap<EntityKey, readonly EntityKey[]>;
  pipeKeysByNode: ReadonlyMap<EntityKey, readonly EntityKey[]>;
  sectionBindings: ReadonlyMap<EntityKey, SectionBinding>;
  visibilityEligibleKeys: ReadonlySet<EntityKey>;
  spatialChunks: readonly SpatialChunk[];
  geometryBounds: Bounds3 | null;
  invalidGeometry: ReadonlyMap<EntityKey, string>;
  summaries: Readonly<{
    entityCount: number;
    nodeCount: number;
    pipeCount: number;
    supportCount: number;
    componentCount: number;
    invalidGeometryCount: number;
  }>;
}>;

type CachedModelIndex = Readonly<{ generation: string; index: ModelIndex }>;

// Undo/redo may revisit the same model object under many publication generations.
// Keep only the latest generation for each identity so retained history cannot also
// retain one complete 10k index per visit.
const INDEX_CACHE = new WeakMap<PreviewModel, CachedModelIndex>();
const CHUNK_SIZE = 128;

export function modelIndexFor(
  model: PreviewModel,
  sessionGeneration: number,
  modelRevision: number
): ModelIndex {
  const generation = `${sessionGeneration}:${modelRevision}`;
  const existing = INDEX_CACHE.get(model);
  if (existing?.generation === generation) return existing.index;
  const created = buildModelIndex(model, sessionGeneration, modelRevision);
  INDEX_CACHE.set(model, Object.freeze({ generation, index: created }));
  return created;
}

export function buildModelIndex(
  model: PreviewModel,
  sessionGeneration: number,
  modelRevision: number
): ModelIndex {
  const entities = new Map<EntityKey, IndexedEntity>();
  const labels = new Map<EntityKey, string>();
  const treeChildren = new Map<EntityKey, EntityKey[]>();
  const pipeKeysByNode = new Map<EntityKey, EntityKey[]>();
  const sectionBindings = new Map<EntityKey, SectionBinding>();
  const visibilityEligibleKeys = new Set<EntityKey>();
  const invalidGeometry = new Map<EntityKey, string>();
  const projectRef: EntityRef = { type: "project", id: model.project.id };
  const projectKey = entityKey(projectRef);
  const treeOrder: EntityKey[] = [projectKey];
  const spatialEntries: Array<{ key: EntityKey; bounds: Bounds3 }> = [];

  const add = (
    ref: EntityRef,
    label: string,
    record: unknown,
    geometryBounds: Bounds3 | null = null,
    anchor: Vec3 | null = null,
    geometryIssue: string | null = null
  ) => {
    const key = entityKey(ref);
    const frozenAnchor = anchor ? freezeVec(anchor) : null;
    const entry: IndexedEntity = Object.freeze({
      key,
      ref: Object.freeze({ ...ref }),
      label,
      record,
      parentKey: ref.type === "project" ? null : projectKey,
      geometryBounds,
      anchor: frozenAnchor,
      geometryIssue
    });
    if (!entities.has(key)) {
      entities.set(key, entry);
      labels.set(key, label);
      if (ref.type !== "project") {
        treeOrder.push(key);
        const children = treeChildren.get(projectKey) ?? [];
        children.push(key);
        treeChildren.set(projectKey, children);
      }
    }
    if (geometryIssue) invalidGeometry.set(key, geometryIssue);
    if (geometryBounds && !geometryIssue) {
      visibilityEligibleKeys.add(key);
      spatialEntries.push({ key, bounds: geometryBounds });
    }
    return key;
  };

  add(projectRef, model.project.name || model.project.id, model);

  for (const material of model.materials ?? []) {
    add({ type: "material", id: material.id }, material.label || material.id, material);
  }
  const duplicateSectionIds = duplicateIds(model.sections ?? []);
  const uniqueSections = new Map(
    (model.sections ?? []).filter((section) => !duplicateSectionIds.has(section.id)).map((section) => [section.id, section] as const)
  );
  for (const section of model.sections ?? []) {
    add({ type: "section", id: section.id }, section.name || section.id, section);
  }

  const duplicateNodeIds = duplicateIds(model.nodes);
  const duplicatePipeIds = duplicateIds(model.pipe_segments);
  const duplicateSupportIds = duplicateIds(model.supports);
  const duplicateComponentIds = duplicateIds(model.components);
  const nodesById = new Map(
    model.nodes.filter((node) => !duplicateNodeIds.has(node.id)).map((node) => [node.id, node] as const)
  );
  for (const node of model.nodes) {
    const issue = duplicateNodeIds.has(node.id)
      ? `Node ${node.id} has a duplicate same-type identifier; its authored geometry is ambiguous.`
      : finiteVec(node.position)
        ? null
        : `Node ${node.id} has a non-finite authored coordinate.`;
    const bounds = issue ? null : pointBounds(node.position);
    add({ type: "node", id: node.id }, node.label || node.id, node, bounds, issue ? null : node.position, issue);
  }

  for (const pipe of model.pipe_segments) {
    const from = nodesById.get(pipe.from)?.position;
    const to = nodesById.get(pipe.to)?.position;
    let issue: string | null = null;
    if (duplicatePipeIds.has(pipe.id)) {
      issue = `Pipe ${pipe.id} has a duplicate same-type identifier; its authored geometry is ambiguous.`;
    } else if (duplicateNodeIds.has(pipe.from) || duplicateNodeIds.has(pipe.to)) {
      issue = `Pipe ${pipe.id} references an ambiguous duplicate node endpoint.`;
    } else if (!from || !to) issue = `Pipe ${pipe.id} references a missing endpoint.`;
    else if (!finiteVec(from) || !finiteVec(to)) issue = `Pipe ${pipe.id} has a non-finite endpoint.`;
    else if (!finiteNumber(distanceSquared(from, to)) || distanceSquared(from, to) <= 0) {
      issue = `Pipe ${pipe.id} has an unrepresentable or zero authored span.`;
    }
    const bounds = issue || !from || !to ? null : boundsFromPoints([from, to]);
    const anchor = issue || !from || !to ? null : midpoint(from, to);
    const pipeKey = add({ type: "pipe", id: pipe.id }, pipe.label || pipe.id, pipe, bounds, anchor, issue);
    for (const endpointId of [pipe.from, pipe.to]) {
      const nodeKey = entityKey({ type: "node", id: endpointId });
      const adjacent = pipeKeysByNode.get(nodeKey) ?? [];
      adjacent.push(pipeKey);
      pipeKeysByNode.set(nodeKey, adjacent);
    }
    const hasSectionRef = pipe.section_ref !== undefined;
    const sectionRef = pipe.section_ref?.trim() ?? "";
    const sectionKey = sectionRef ? entityKey({ type: "section", id: sectionRef }) : null;
    const sharedSection = sectionRef ? uniqueSections.get(sectionRef) : null;
    const shared = sharedSection && sectionKey ? entities.get(sectionKey) : null;
    const sectionIssue = !hasSectionRef
      ? null
      : !sectionRef
        ? `Pipe ${pipe.id} has a blank shared section reference.`
        : duplicateSectionIds.has(sectionRef)
          ? `Pipe ${pipe.id} references duplicate shared section ${sectionRef}.`
          : !sharedSection || !shared
            ? `Pipe ${pipe.id} references missing shared section ${sectionRef}.`
            : sharedSection.section_type !== "pipe"
              ? `Pipe ${pipe.id} references shared section ${sectionRef} with unsupported type ${sharedSection.section_type}.`
              : sharedSectionBindingIssue(pipe, sharedSection);
    sectionBindings.set(pipeKey, Object.freeze({
      source: hasSectionRef ? "shared" : "inline",
      sectionKey: shared ? sectionKey : null,
      record: shared
        ? (shared.record as NonNullable<PreviewModel["sections"]>[number]).properties
        : pipe.section,
      issue: sectionIssue
    }));
  }

  for (const support of model.supports) {
    const position = nodesById.get(support.node)?.position;
    const issue = duplicateSupportIds.has(support.id)
      ? `Support ${support.id} has a duplicate same-type identifier; its authored geometry is ambiguous.`
      : duplicateNodeIds.has(support.node)
      ? `Support ${support.id} references an ambiguous duplicate node.`
      : !position
      ? `Support ${support.id} references a missing node.`
      : !finiteVec(position)
        ? `Support ${support.id} is anchored to a non-finite node.`
        : null;
    add(
      { type: "support", id: support.id },
      support.label || support.id,
      support,
      issue || !position ? null : pointBounds(position),
      issue ? null : position ?? null,
      issue
    );
  }

  for (const component of model.components) {
    const position = nodesById.get(component.node)?.position;
    const issue = duplicateComponentIds.has(component.id)
      ? `Component ${component.id} has a duplicate same-type identifier; its authored geometry is ambiguous.`
      : duplicateNodeIds.has(component.node)
      ? `Component ${component.id} references an ambiguous duplicate node.`
      : !position
      ? `Component ${component.id} references a missing node.`
      : !finiteVec(position)
        ? `Component ${component.id} is anchored to a non-finite node.`
        : null;
    add(
      { type: "component", id: component.id },
      component.label || component.id,
      component,
      issue || !position ? null : pointBounds(position),
      issue ? null : position ?? null,
      issue
    );
  }

  for (const load of model.load_cases) {
    add({ type: "load", id: load.id }, load.label || load.id, load);
  }
  for (const combination of model.combinations ?? []) {
    add({ type: "combination", id: combination.id }, combination.label || combination.id, combination);
  }
  model.diagnostics.forEach((diagnostic, index) => {
    const id = diagnostic.id ?? `${diagnostic.code}:${index}`;
    add({ type: "diagnostic", id }, diagnostic.code, diagnostic);
  });

  const spatialChunks = chunkSpatialEntries(spatialEntries);
  const geometryBounds = unionBounds(spatialEntries.map((entry) => entry.bounds));
  for (const [key, children] of treeChildren) treeChildren.set(key, Object.freeze([...children]) as EntityKey[]);
  for (const [key, adjacent] of pipeKeysByNode) pipeKeysByNode.set(key, Object.freeze([...adjacent]) as EntityKey[]);

  return Object.freeze({
    generation: `${sessionGeneration}:${modelRevision}`,
    sessionGeneration,
    modelRevision,
    projectKey,
    entities,
    labels,
    treeOrder: Object.freeze(treeOrder),
    treeChildren,
    pipeKeysByNode,
    sectionBindings,
    visibilityEligibleKeys,
    spatialChunks,
    geometryBounds,
    invalidGeometry,
    summaries: Object.freeze({
      entityCount: entities.size,
      nodeCount: model.nodes.length,
      pipeCount: model.pipe_segments.length,
      supportCount: model.supports.length,
      componentCount: model.components.length,
      invalidGeometryCount: invalidGeometry.size
    })
  });
}

export function boundsIntersect(a: Bounds3, b: Bounds3): boolean {
  return a.min.x <= b.max.x && a.max.x >= b.min.x &&
    a.min.y <= b.max.y && a.max.y >= b.min.y &&
    a.min.z <= b.max.z && a.max.z >= b.min.z;
}

export function boundsFromPoints(points: readonly Readonly<Vec3>[]): Bounds3 | null {
  if (points.length === 0 || points.some((point) => !finiteVec(point))) return null;
  const min = { ...points[0] };
  const max = { ...points[0] };
  for (const point of points.slice(1)) {
    min.x = Math.min(min.x, point.x);
    min.y = Math.min(min.y, point.y);
    min.z = Math.min(min.z, point.z);
    max.x = Math.max(max.x, point.x);
    max.y = Math.max(max.y, point.y);
    max.z = Math.max(max.z, point.z);
  }
  return freezeBounds({ min, max });
}

function pointBounds(point: Vec3): Bounds3 {
  return freezeBounds({ min: point, max: point });
}

function chunkSpatialEntries(entries: readonly { key: EntityKey; bounds: Bounds3 }[]): readonly SpatialChunk[] {
  // Sort by authored-space cell then typed key so spatial locality is stable and
  // independent of declaration order. Exact per-entry bounds remain authoritative.
  const overall = unionBounds(entries.map((entry) => entry.bounds));
  const span = overall ? Math.max(
    overall.max.x - overall.min.x,
    overall.max.y - overall.min.y,
    overall.max.z - overall.min.z,
    1
  ) : 1;
  const cell = span / Math.max(1, Math.ceil(Math.cbrt(entries.length / CHUNK_SIZE)));
  const spatial = [...entries].sort((a, b) => {
    const ac = boundsCenter(a.bounds);
    const bc = boundsCenter(b.bounds);
    const ax = Math.floor((ac.x - (overall?.min.x ?? 0)) / cell);
    const bx = Math.floor((bc.x - (overall?.min.x ?? 0)) / cell);
    if (ax !== bx) return ax - bx;
    const ay = Math.floor((ac.y - (overall?.min.y ?? 0)) / cell);
    const by = Math.floor((bc.y - (overall?.min.y ?? 0)) / cell);
    if (ay !== by) return ay - by;
    const az = Math.floor((ac.z - (overall?.min.z ?? 0)) / cell);
    const bz = Math.floor((bc.z - (overall?.min.z ?? 0)) / cell);
    return az !== bz ? az - bz : String(a.key).localeCompare(String(b.key));
  });
  const chunks: SpatialChunk[] = [];
  for (let offset = 0; offset < spatial.length; offset += CHUNK_SIZE) {
    const slice = spatial.slice(offset, offset + CHUNK_SIZE);
    const bounds = unionBounds(slice.map((entry) => entry.bounds));
    if (!bounds) continue;
    chunks.push(Object.freeze({
      id: chunks.length,
      bounds,
      entityKeys: Object.freeze(slice.map((entry) => entry.key))
    }));
  }
  return Object.freeze(chunks);
}

function duplicateIds<T extends { id: string }>(items: readonly T[]): ReadonlySet<string> {
  const counts = new Map<string, number>();
  for (const item of items) counts.set(item.id, (counts.get(item.id) ?? 0) + 1);
  return new Set([...counts].filter(([, count]) => count > 1).map(([id]) => id));
}

function boundsCenter(bounds: Bounds3): Vec3 {
  return {
    x: (bounds.min.x + bounds.max.x) / 2,
    y: (bounds.min.y + bounds.max.y) / 2,
    z: (bounds.min.z + bounds.max.z) / 2
  };
}

function sharedSectionBindingIssue(
  pipe: PreviewModel["pipe_segments"][number],
  section: NonNullable<PreviewModel["sections"]>[number]
): string | null {
  const definedKeys = Object.keys(section.properties).filter((key) => section.properties[key] !== undefined);
  if (definedKeys.some((key) => key !== "outside_diameter" && key !== "wall_thickness")) {
    return `Shared section ${section.id} contains unsupported properties for Actual OD resolution.`;
  }
  const outside = section.properties.outside_diameter;
  const wall = section.properties.wall_thickness;
  if (!validPositiveQuantity(outside) || !validPositiveQuantity(wall)) {
    return `Shared section ${section.id} has an invalid OD or wall-thickness envelope.`;
  }
  const inlineOutside = pipe.section.outside_diameter;
  const inlineWall = pipe.section.wall_thickness;
  if (!sameQuantity(inlineOutside, outside) || !sameQuantity(inlineWall, wall)) {
    return `Pipe ${pipe.id} inline OD/wall values are inconsistent with shared section ${section.id}.`;
  }
  return null;
}

function validPositiveQuantity(value: { value: number; unit: string } | undefined): value is { value: number; unit: string } {
  return Boolean(value && Number.isFinite(value.value) && value.value > 0 && value.unit.trim());
}

function sameQuantity(
  left: { value: number; unit: string } | undefined,
  right: { value: number; unit: string } | undefined
): boolean {
  return Boolean(left && right && left.value === right.value && left.unit === right.unit);
}

function unionBounds(bounds: readonly Bounds3[]): Bounds3 | null {
  if (bounds.length === 0) return null;
  return boundsFromPoints(bounds.flatMap((item) => [item.min, item.max]));
}

function freezeBounds(bounds: { min: Vec3; max: Vec3 }): Bounds3 {
  return Object.freeze({ min: freezeVec(bounds.min), max: freezeVec(bounds.max) });
}

function freezeVec(value: Vec3): Readonly<Vec3> {
  return Object.freeze({ x: value.x, y: value.y, z: value.z });
}

function finiteVec(value: Vec3): boolean {
  return finiteNumber(value.x) && finiteNumber(value.y) && finiteNumber(value.z);
}

function finiteNumber(value: number): boolean {
  return Number.isFinite(value);
}

function distanceSquared(a: Vec3, b: Vec3): number {
  const x = b.x - a.x;
  const y = b.y - a.y;
  const z = b.z - a.z;
  return x * x + y * y + z * z;
}

function midpoint(a: Vec3, b: Vec3): Vec3 {
  return { x: (a.x + b.x) / 2, y: (a.y + b.y) / 2, z: (a.z + b.z) / 2 };
}
