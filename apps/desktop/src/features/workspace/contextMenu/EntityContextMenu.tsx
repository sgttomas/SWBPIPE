import { useEffect, useState } from "react";
import { canonicalJsonString } from "../../../services/hashService";
import type { EditorOperationIntent, EntityRef, PreviewModel } from "../../../types";
import { buildDeleteNodeIntent, buildDeletePipeIntent, buildDeleteSupportIntent } from "../../model-tree/removalIntents";
import { buildGuardedRemoval, removalEntity } from "../../toolkit/GuardedRemoval";
import { displayedBoundsForEntityKeys, visibilityEligibleSelectionKeys, type ViewportViewCommand } from "../../viewport/viewportSelection";
import type { ModelIndex } from "../modelIndex";
import { entityKey, type EntityKey } from "../selectionState";
import { ContextMenu, type ContextMenuEntry, type ContextMenuRequest } from "./ContextMenu";

// B9: the commands a context menu offers for a model entity, the same on every surface. Each
// runs an existing command through its existing path:
// - Hide, Isolate, Show all and Fit selection dispatch the viewport's own view commands, as its
//   toolbar buttons (and H, I) do. They are session view state and never change the model.
// - Queue removal queues the intent the inspector's Delete panel or Remove panel queues, for the
//   same review and Apply. Opening the menu changes nothing.
// The selection commands act on the selection. A menu opened from the keyboard on an entity
// that is not selected leaves the selection alone, and these commands then say so.

export type EntityCommandEnvironment = Readonly<{
  model: PreviewModel;
  modelIndex: ModelIndex;
  /** The ordered selection the commands act on. */
  selectedKeys: readonly EntityKey[];
  /** The effective hidden mask the viewport draws and fits with. */
  hiddenKeys: ReadonlySet<EntityKey>;
  /** Whether anything is hidden or isolated, so that Show all has something to do. */
  showAllAvailable: boolean;
  /** The viewport's view-command dispatcher; null while no viewport is mounted. */
  viewCommand: ((command: ViewportViewCommand) => void) | null;
  /** True in the viewport, where H and I run Hide and Isolate. */
  viewportShortcuts?: boolean;
  /** Overrides the fit check with the viewport's own (it knows the drawn pipe radii). */
  fitSelectionAvailable?: boolean;
  operationBusy?: boolean;
  onQueueIntent?: (intent: EditorOperationIntent) => void;
}>;

/** The viewport's own reason for Hide and Isolate. */
export const VISIBILITY_SELECTION_REASON = "Select at least one valid drawable node, pipe, support, or component.";
const FIT_SELECTION_REASON = "No visible valid selected geometry can be fitted.";
const SHOW_ALL_REASON = "All eligible entities are already shown.";

export type PreparedRemoval =
  | Readonly<{ state: "ready"; intent: EditorOperationIntent; target: EntityKey; model: PreviewModel }>
  | Readonly<{ state: "preparing" | "failed"; target: EntityKey; model: PreviewModel }>;

function notSelectedReason(target: EntityRef): string {
  return `This acts on the selection, which does not include ${target.id}. Select it first.`;
}

function removalEntry(env: EntityCommandEnvironment, target: EntityRef, prepared: PreparedRemoval | null): ContextMenuEntry {
  const base = { kind: "command" as const, id: "queue-removal", label: "Queue removal for review" };
  const key = entityKey(target);
  const disabled = (reason: string): ContextMenuEntry => ({ ...base, disabled: true, reason, run: () => undefined });
  if (target.type === "load" || target.type === "combination") return disabled("Load cases and combinations are removed in the Load Cases panel.");
  if (target.type !== "node" && target.type !== "pipe" && target.type !== "support" && !removalEntity(env.model, target)) {
    return disabled(target.type === "material" || target.type === "section" || target.type === "component"
      ? `${target.id} is not in the current model.`
      : "This entity has no removal command.");
  }
  if (!env.onQueueIntent) return disabled("The operation queue is unavailable here.");
  if (!env.selectedKeys.includes(key)) return disabled(notSelectedReason(target));
  if (env.selectedKeys.length > 1) return disabled(`Removal queues one entity at a time; select only ${target.id}.`);
  const queue = env.onQueueIntent;
  if (target.type === "node" || target.type === "pipe" || target.type === "support") {
    // The inspector's Delete node, Delete pipe and Delete support panels.
    const intent = target.type === "node"
      ? (() => { const node = env.model.nodes.find((candidate) => candidate.id === target.id); return node ? buildDeleteNodeIntent(node, env.model) : null; })()
      : target.type === "pipe"
        ? (() => { const pipe = env.model.pipe_segments.find((candidate) => candidate.id === target.id); return pipe ? buildDeletePipeIntent(pipe, env.model) : null; })()
        : (() => { const support = env.model.supports.find((candidate) => candidate.id === target.id); return support ? buildDeleteSupportIntent(support, env.model) : null; })();
    if (!intent) return disabled(`${target.id} is not in the current model.`);
    return { ...base, run: () => queue(intent) };
  }
  // The inspector's Remove panel for materials, sections and components (GuardedRemoval).
  if (env.operationBusy) return disabled("Another operation is running.");
  if (!prepared || prepared.target !== key || prepared.model !== env.model || prepared.state === "preparing") return disabled("Preparing the current entity snapshot…");
  if (prepared.state !== "ready") return disabled("The removal preview could not be prepared. Check that the operation engine is available.");
  const intent = prepared.intent;
  return { ...base, run: () => queue(intent) };
}

export function entityCommandEntries(env: EntityCommandEnvironment, target: EntityRef, prepared: PreparedRemoval | null = null): ContextMenuEntry[] {
  const key = entityKey(target);
  const selected = env.selectedKeys.includes(key);
  const eligible = visibilityEligibleSelectionKeys(env.modelIndex, env.selectedKeys).length > 0;
  const fittable = env.fitSelectionAvailable ?? Boolean(displayedBoundsForEntityKeys(env.modelIndex, env.selectedKeys, env.hiddenKeys, new Map()));
  const view = env.viewCommand;
  const viewReason = (available: boolean, reason: string) => !view ? "The viewport is unavailable." : !selected ? notSelectedReason(target) : available ? null : reason;
  const viewEntry = (id: string, label: string, command: ViewportViewCommand, reason: string | null, shortcut?: string): ContextMenuEntry => ({
    kind: "command", id, label, shortcut, disabled: Boolean(reason), reason, run: () => view?.(command)
  });
  return [
    viewEntry("hide-selection", "Hide selected", { type: "hide-selection" }, viewReason(eligible, VISIBILITY_SELECTION_REASON), env.viewportShortcuts ? "H" : undefined),
    viewEntry("isolate-selection", "Isolate selected", { type: "isolate-selection" }, viewReason(eligible, VISIBILITY_SELECTION_REASON), env.viewportShortcuts ? "I" : undefined),
    viewEntry("fit-selection", "Fit selection", { type: "fit-selection" }, viewReason(fittable, FIT_SELECTION_REASON)),
    viewEntry("show-all", "Show all", { type: "show-all" }, !view ? "The viewport is unavailable." : env.showAllAvailable ? null : SHOW_ALL_REASON),
    { kind: "separator", id: "separator-removal" },
    removalEntry(env, target, prepared)
  ];
}

/** Prepares the guarded removal snapshot for a material, section or component, as the Remove panel does. */
function usePreparedRemoval(model: PreviewModel, target: EntityRef): PreparedRemoval | null {
  const entity = removalEntity(model, target);
  const key = entityKey(target);
  const [prepared, setPrepared] = useState<PreparedRemoval | null>(null);
  useEffect(() => {
    if (!entity) { setPrepared(null); return; }
    let active = true;
    setPrepared({ state: "preparing", target: key, model });
    canonicalJsonString(entity)
      .then((before) => { if (active) setPrepared({ state: "ready", target: key, model, intent: buildGuardedRemoval(target, before) }); })
      .catch(() => { if (active) setPrepared({ state: "failed", target: key, model }); });
    return () => { active = false; };
  }, [model, entity, key]);
  return prepared;
}

export function entityMenuLabel(target: EntityRef): string {
  return `Commands for ${target.type} ${target.id}`;
}

/** The entity menu, with a surface's own commands (a grid cell's) above the entity commands. */
export function EntityContextMenu({ env, target, leading = [], request, onClose, testId = "entity-context-menu" }: {
  env: EntityCommandEnvironment;
  target: EntityRef;
  leading?: readonly ContextMenuEntry[];
  request: ContextMenuRequest;
  onClose: () => void;
  testId?: string;
}) {
  const prepared = usePreparedRemoval(env.model, target);
  const entries = [
    ...leading,
    ...(leading.length > 0 ? [{ kind: "separator", id: "separator-entity" } as const] : []),
    ...entityCommandEntries(env, target, prepared)
  ];
  return <ContextMenu label={entityMenuLabel(target)} entries={entries} request={request} onClose={onClose} testId={testId} />;
}
