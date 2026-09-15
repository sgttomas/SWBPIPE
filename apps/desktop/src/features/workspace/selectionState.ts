import type { EntityRef } from "../../types";

export type EntityKey = string & { readonly __entityKey: unique symbol };

export type SelectionModifiers = {
  additive?: boolean;
  toggle?: boolean;
};

export type BoxSelectionModifiers = SelectionModifiers;

export type OrderedSelectionState = Readonly<{
  orderedKeys: readonly EntityKey[];
  primaryKey: EntityKey | null;
  rangeAnchorKey: EntityKey | null;
  focusKey: EntityKey | null;
  preparationEpoch: number;
}>;

export function entityKey(ref: EntityRef): EntityKey {
  return JSON.stringify([ref.type, ref.id]) as EntityKey;
}

export function entityRefFromKey(key: EntityKey): EntityRef | null {
  try {
    const decoded: unknown = JSON.parse(key);
    if (!Array.isArray(decoded) || decoded.length !== 2) return null;
    const [type, id] = decoded;
    if (typeof type !== "string" || typeof id !== "string" || !ENTITY_TYPES.has(type as EntityRef["type"])) {
      return null;
    }
    return { type: type as EntityRef["type"], id };
  } catch {
    return null;
  }
}

export function emptySelection(preparationEpoch = 0): OrderedSelectionState {
  return freezeSelection({
    orderedKeys: [],
    primaryKey: null,
    rangeAnchorKey: null,
    focusKey: null,
    preparationEpoch
  });
}

export function singletonSelection(
  ref: EntityRef,
  previous: OrderedSelectionState = emptySelection()
): OrderedSelectionState {
  return replaceSelection(previous, [entityKey(ref)], entityKey(ref), entityKey(ref));
}

export function projectSelection(
  project: EntityRef,
  previous: OrderedSelectionState = emptySelection()
): OrderedSelectionState {
  if (project.type !== "project") throw new Error("Project selection requires a project entity.");
  return singletonSelection(project, previous);
}

export function applySelection(
  current: OrderedSelectionState,
  ref: EntityRef,
  modifiers: SelectionModifiers = {}
): OrderedSelectionState {
  const key = entityKey(ref);
  if (ref.type === "project") return projectSelection(ref, current);
  if (!modifiers.toggle && !modifiers.additive && current.primaryKey === key &&
      current.orderedKeys.length === 1 && current.orderedKeys[0] === key) {
    return current;
  }

  const withoutProject = current.orderedKeys.filter((candidate) => entityRefFromKey(candidate)?.type !== "project");
  let orderedKeys: EntityKey[];
  let primaryKey: EntityKey | null;

  if (modifiers.toggle) {
    if (withoutProject.includes(key)) {
      orderedKeys = withoutProject.filter((candidate) => candidate !== key);
      primaryKey = current.primaryKey === key
        ? orderedKeys.at(-1) ?? null
        : current.primaryKey && orderedKeys.includes(current.primaryKey)
          ? current.primaryKey
          : orderedKeys.at(-1) ?? null;
    } else {
      orderedKeys = [...withoutProject, key];
      primaryKey = key;
    }
  } else if (modifiers.additive) {
    orderedKeys = withoutProject.includes(key) ? withoutProject : [...withoutProject, key];
    primaryKey = key;
  } else {
    orderedKeys = [key];
    primaryKey = key;
  }

  return replaceSelection(current, orderedKeys, primaryKey, key);
}

export function applyDisplayedRange(
  current: OrderedSelectionState,
  hit: EntityRef,
  displayedOrder: readonly EntityKey[]
): OrderedSelectionState {
  const hitKey = entityKey(hit);
  if (hit.type === "project") return projectSelection(hit, current);
  const anchor = current.rangeAnchorKey;
  const anchorIndex = anchor ? displayedOrder.indexOf(anchor) : -1;
  const hitIndex = displayedOrder.indexOf(hitKey);
  if (anchorIndex < 0 || hitIndex < 0) return applySelection(current, hit, { additive: true });

  const start = Math.min(anchorIndex, hitIndex);
  const end = Math.max(anchorIndex, hitIndex);
  const orderedKeys = current.orderedKeys.filter((key) => entityRefFromKey(key)?.type !== "project");
  const membership = new Set(orderedKeys);
  for (const key of displayedOrder.slice(start, end + 1)) {
    if (membership.has(key)) continue;
    membership.add(key);
    orderedKeys.push(key);
  }
  return replaceSelection(current, orderedKeys, hitKey, anchor);
}

/** Applies one completed box gesture as one immutable selection publication. */
export function applyBoxSelection(
  current: OrderedSelectionState,
  hitKeys: readonly EntityKey[],
  modifiers: BoxSelectionModifiers = {}
): OrderedSelectionState {
  const hits = [...new Set(hitKeys)].filter((key) => entityRefFromKey(key)?.type !== "project");
  const existing = current.orderedKeys.filter((key) => entityRefFromKey(key)?.type !== "project");
  if ((modifiers.toggle || modifiers.additive) && hits.length === 0) return current;

  if (!modifiers.toggle && !modifiers.additive) {
    return replaceSelection(current, hits, hits.at(-1) ?? null, hits.at(-1) ?? null);
  }

  const orderedKeys = [...existing];
  const membership = new Set(orderedKeys);
  let primaryKey = current.primaryKey && membership.has(current.primaryKey)
    ? current.primaryKey
    : orderedKeys.at(-1) ?? null;

  for (const key of hits) {
    if (modifiers.toggle) {
      if (membership.delete(key)) {
        const index = orderedKeys.indexOf(key);
        if (index >= 0) orderedKeys.splice(index, 1);
        if (primaryKey === key) primaryKey = orderedKeys.at(-1) ?? null;
      } else {
        membership.add(key);
        orderedKeys.push(key);
        primaryKey = key;
      }
      continue;
    }
    if (!membership.has(key)) {
      membership.add(key);
      orderedKeys.push(key);
    }
    primaryKey = key;
  }
  return replaceSelection(current, orderedKeys, primaryKey, hits.at(-1) ?? current.rangeAnchorKey);
}

export function setSelectionFocus(
  current: OrderedSelectionState,
  focusKey: EntityKey | null
): OrderedSelectionState {
  if (current.focusKey === focusKey) return current;
  return freezeSelection({ ...current, focusKey });
}

export function pruneSelection(
  current: OrderedSelectionState,
  validKeys: ReadonlySet<EntityKey>,
  _project: EntityRef
): OrderedSelectionState {
  const kept = current.orderedKeys.filter((key) => validKeys.has(key));
  if (kept.length === 0) {
    const empty = replaceSelection(current, [], null, null);
    return empty.focusKey && !validKeys.has(empty.focusKey) ? setSelectionFocus(empty, null) : empty;
  }
  const primaryKey = current.primaryKey && kept.includes(current.primaryKey)
    ? current.primaryKey
    : kept.at(-1) ?? null;
  const rangeAnchorKey = current.rangeAnchorKey && validKeys.has(current.rangeAnchorKey)
    ? current.rangeAnchorKey
    : null;
  const next = replaceSelection(current, kept, primaryKey, rangeAnchorKey);
  return next.focusKey && !validKeys.has(next.focusKey) ? setSelectionFocus(next, null) : next;
}

export function selectionContains(current: OrderedSelectionState, ref: EntityRef): boolean {
  return current.orderedKeys.includes(entityKey(ref));
}

export function primarySelection(current: OrderedSelectionState, fallback: EntityRef): EntityRef {
  return (current.primaryKey && entityRefFromKey(current.primaryKey)) ?? fallback;
}

export function sameSelection(a: OrderedSelectionState, b: OrderedSelectionState): boolean {
  return a.primaryKey === b.primaryKey &&
    a.orderedKeys.length === b.orderedKeys.length &&
    a.orderedKeys.every((key, index) => key === b.orderedKeys[index]);
}

function replaceSelection(
  current: OrderedSelectionState,
  orderedKeys: readonly EntityKey[],
  primaryKey: EntityKey | null,
  rangeAnchorKey: EntityKey | null
): OrderedSelectionState {
  const uniqueKeys = [...new Set(orderedKeys)];
  const normalizedPrimary = primaryKey && uniqueKeys.includes(primaryKey) ? primaryKey : uniqueKeys.at(-1) ?? null;
  const changed = current.primaryKey !== normalizedPrimary ||
    current.orderedKeys.length !== uniqueKeys.length ||
    current.orderedKeys.some((key, index) => key !== uniqueKeys[index]);
  if (!changed && current.rangeAnchorKey === rangeAnchorKey) return current;
  return freezeSelection({
    orderedKeys: uniqueKeys,
    primaryKey: normalizedPrimary,
    rangeAnchorKey,
    focusKey: current.focusKey,
    preparationEpoch: changed ? current.preparationEpoch + 1 : current.preparationEpoch
  });
}

function freezeSelection(state: {
  orderedKeys: readonly EntityKey[];
  primaryKey: EntityKey | null;
  rangeAnchorKey: EntityKey | null;
  focusKey: EntityKey | null;
  preparationEpoch: number;
}): OrderedSelectionState {
  return Object.freeze({ ...state, orderedKeys: Object.freeze([...state.orderedKeys]) });
}

const ENTITY_TYPES: ReadonlySet<EntityRef["type"]> = new Set([
  "project",
  "material",
  "section",
  "node",
  "pipe",
  "support",
  "component",
  "load",
  "combination",
  "diagnostic"
]);
