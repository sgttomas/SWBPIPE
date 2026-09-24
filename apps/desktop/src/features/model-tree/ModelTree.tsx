import { QuantityReadout } from "../display-units";
import { Anchor, Box, CircleDot, GitBranch, ListTree, Search, SquareStack, Table2, Waypoints, X, Zap } from "lucide-react";
import type React from "react";
import { createPortal } from "react-dom";
import { useEffect, useId, useMemo, useRef, useState, type KeyboardEvent } from "react";
import type { EditorOperationIntent, EditorOperationObjectType, EntityRef, PreviewModel } from "../../types";
import { entityKey, type EntityKey, type OrderedSelectionState, type SelectionModifiers } from "../workspace/selectionState";
import { modelIndexFor, type ModelIndex } from "../workspace/modelIndex";
import { VirtualList } from "../workspace/VirtualList";
import { EngineeringTable, useTableBodyHeight, type TableApplyResult } from "../workspace/table/EngineeringTable";
import { buildGridOperationIntent, materialTableColumns, sectionTableColumns, nodeTableColumns, type GridColumn, type GridRow } from "../workspace/table/modelTableAdapter";
import { quantityCellKey, useQuantitySortProjection } from "../workspace/table/quantitySortProjection";
import { capturedCellIsCurrent, type CapturedCell, type TableRow } from "../workspace/table/tableState";

type Props = {
  model: PreviewModel;
  boundedGrid?: boolean;
  compactGrid?: boolean;
  selection: EntityRef;
  selectionState?: OrderedSelectionState;
  density?: "comfortable" | "compact";
  hiddenKeys?: ReadonlySet<EntityKey>;
  modelIndex?: ModelIndex;
  projectSessionGeneration?: number;
  onSelect: (
    selection: EntityRef,
    modifiers?: SelectionModifiers & { range?: boolean },
    displayedOrder?: readonly EntityKey[]
  ) => void;
  onFocusChange?: (key: EntityKey | null) => void;
  onFilterPublication?: (publication: { actionSequence: number; publicationSequence: number; query: string; visibleCount: number; inputAt: number | null; inputEventTimeStamp: number | null; publishedAt: number }) => void;
  onQueueIntent?: (intent: EditorOperationIntent) => void;
  onApplyCellIntent?: (intent: EditorOperationIntent) => Promise<TableApplyResult>;
  operationBusy?: boolean;
};

type LayoutMode = "tree" | "grid";
type GridEntityType =
  | "materials"
  | "sections"
  | "nodes"
  | "pipes"
  | "supports"
  | "components"
  | "load_cases"
  | "combinations";

const GRID_ENTITY_TYPES: ReadonlyArray<{ id: GridEntityType; label: string }> = [
  { id: "nodes", label: "Nodes" },
  { id: "pipes", label: "Pipes" },
  { id: "supports", label: "Supports" },
  { id: "materials", label: "Materials" },
  { id: "sections", label: "Sections" },
  { id: "components", label: "Components" },
  { id: "load_cases", label: "Load Cases" },
  { id: "combinations", label: "Combinations" }
];

export function ModelTree({ boundedGrid = false, compactGrid = false, model, modelIndex, selection, selectionState, density = "comfortable", hiddenKeys = new Set(), projectSessionGeneration = 0, onSelect, onFocusChange = () => {}, onFilterPublication = () => {}, onQueueIntent, onApplyCellIntent, operationBusy }: Props) {
  const [toolbarTarget, setToolbarTarget] = useState<HTMLDivElement | null>(null);
  const [filterText, setFilterText] = useState("");
  const [layoutMode, setLayoutMode] = useState<LayoutMode>("tree");
  const [gridOpened, setGridOpened] = useState(false);
  const [gridEntityType, setGridEntityType] = useState<GridEntityType>(() => gridEntityTypeFromSelection(selection));
  const [gridDrafts, setGridDrafts] = useState<Record<string, string>>({});
  const [focusedGridKey, setFocusedGridKey] = useState<EntityKey | null>(null);
  const activeModelIndex = useMemo(() => modelIndex ?? modelIndexFor(model, 0, 0), [model, modelIndex]);
  const tree = useMemo(() => buildTree(activeModelIndex), [activeModelIndex]);
  const filteredTree = useMemo(() => filterTree(tree, filterText), [tree, filterText]);
  const [expandedGroups, setExpandedGroups] = useState<ReadonlySet<string>>(() => new Set(
    tree.groups.map((group) => group.title)
  ));
  const [filteredCollapsedGroups, setFilteredCollapsedGroups] = useState<ReadonlySet<string>>(() => new Set());
  const [focusedTreeKey, setFocusedTreeKey] = useState<string | null>(() => selectionState?.focusKey ?? null);
  const filterActionSequence = useRef(0);
  const filterPublicationSequence = useRef(0);
  const filterInputTiming = useRef<{ inputAt: number | null; inputEventTimeStamp: number | null }>({ inputAt: null, inputEventTimeStamp: null });
  const treeRows = useMemo(
    () => flattenTreeRows(filteredTree, expandedGroups, Boolean(filterText.trim()), filteredCollapsedGroups),
    [expandedGroups, filterText, filteredTree, filteredCollapsedGroups]
  );
  const effectiveFocusedTreeKey = focusedTreeKey ?? treeRows[0]?.key ?? null;
  const focusedTreeIndex = Math.max(0, treeRows.findIndex((row) => row.key === effectiveFocusedTreeKey));
  const displayedEntityOrder = useMemo(
    () => treeRows.flatMap((row) => row.kind === "entity" ? [entityKey(row.item)] : []),
    [treeRows]
  );
  const selectedKeys = useMemo(
    () => new Set(selectionState?.orderedKeys ?? [entityKey(selection)]),
    [selection.id, selection.type, selectionState]
  );

  useEffect(() => {
    const externallyFocused = selectionState?.focusKey ?? null;
    if (externallyFocused && treeRows.some((row) => row.key === externallyFocused)) {
      setFocusedTreeKey(externallyFocused);
      return;
    }
    if (focusedTreeKey && treeRows.some((row) => row.key === focusedTreeKey)) return;
    const fallback = treeRows.find((row) => row.kind === "entity") ?? treeRows[0] ?? null;
    const fallbackKey = fallback?.key ?? null;
    setFocusedTreeKey(fallbackKey);
    onFocusChange(fallback?.kind === "entity" ? entityKey(fallback.item) : null);
  }, [focusedTreeKey, onFocusChange, selectionState?.focusKey, treeRows]);

  useEffect(() => {
    if (layoutMode === "grid") {
      setGridEntityType(gridEntityTypeFromSelection(selection));
    }
  }, [layoutMode, selection.id, selection.type]);

  useEffect(() => {
    setGridDrafts({});
    setFocusedGridKey(null);
  }, [projectSessionGeneration]);

  useEffect(() => {
    onFilterPublication({
      actionSequence: filterActionSequence.current,
      publicationSequence: ++filterPublicationSequence.current,
      query: filterText,
      visibleCount: filteredTree.count,
      inputAt: filterInputTiming.current.inputAt,
      inputEventTimeStamp: filterInputTiming.current.inputEventTimeStamp,
      publishedAt: performance.now()
    });
  }, [filterText, filteredTree.count, onFilterPublication]);

  // Details is nonmodal: Tab can leave the dialog while staying in the table.
  // Own Escape after the editor/enum handlers but before the shell drawer.
  // Leaving this surface dismisses Details without taking focus back.
  function closeCompactDetails(restoreFocus = false) {
    if (!compactGrid || layoutMode !== "grid") return false;
    const details = toolbarTarget?.querySelector<HTMLElement>(".compact-table-details");
    if (!details?.matches(":popover-open")) return false;
    details.hidePopover();
    if (restoreFocus) toolbarTarget?.querySelector<HTMLButtonElement>(".compact-table-info")?.focus();
    return true;
  }

  return (
    <div className="panel model-tree" data-bounded-grid={boundedGrid && layoutMode === "grid"} data-compact-grid={compactGrid && layoutMode === "grid"} aria-label="Model tree"
      onKeyDown={(event) => {
        if (event.key === "Escape" && !event.defaultPrevented && closeCompactDetails(true)) { event.preventDefault(); event.stopPropagation(); }
      }}
      onBlur={(event) => { if (!(event.relatedTarget instanceof Node) || !event.currentTarget.contains(event.relatedTarget)) closeCompactDetails(); }}>
      <div className="panel-title">Model</div>
      <div className="model-grid-toolbar">
      <section className="layout-mode-toggle" aria-label="Layout grid mode" data-testid="layout-grid-mode-toggle">
        <button
          aria-pressed={layoutMode === "tree"}
          data-testid="layout-mode-tree"
          onClick={() => setLayoutMode("tree")}
          type="button"
        >
          <ListTree size={14} aria-hidden="true" />
          Tree
        </button>
        <button
          aria-pressed={layoutMode === "grid"}
          data-testid="layout-mode-grid"
          onClick={() => { setGridOpened(true); setLayoutMode("grid"); }}
          type="button"
        >
          <Table2 size={14} aria-hidden="true" />
          Grid
        </button>
      </section>
      <TreeControls
        filterText={filterText}
        filteredCount={filteredTree.count}
        totalCount={tree.count}
        onFilterChange={(value, inputAt, inputEventTimeStamp) => {
          filterInputTiming.current = { inputAt, inputEventTimeStamp };
          filterActionSequence.current += 1;
          if (value.trim().toLowerCase() !== filterText.trim().toLowerCase()) setFilteredCollapsedGroups(new Set());
          setFilterText(value);
        }}
      />
      <div className="model-grid-toolbar-target" ref={setToolbarTarget} />
      </div>
      {gridOpened ? <div className="model-grid-frame" hidden={layoutMode !== "grid"} inert={layoutMode !== "grid"}>
        <EntityGrid
          bounded={boundedGrid}
          compact={compactGrid && layoutMode === "grid"}
          toolbarTarget={toolbarTarget}
          density={density}
          drafts={gridDrafts}
          entityType={gridEntityType}
          filterText={filterText}
          focusedGridKey={focusedGridKey}
          model={model}
          onEntityTypeChange={setGridEntityType}
          onQueueIntent={onQueueIntent}
          onApplyCellIntent={onApplyCellIntent}
          operationBusy={operationBusy}
          onSelect={onSelect}
          projectSessionGeneration={projectSessionGeneration}
          selection={selection}
          setDrafts={setGridDrafts}
          setFocusedGridKey={setFocusedGridKey}
        />
      </div> : null}
      {layoutMode !== "grid" ? (
        treeRows.length > 0 ? (
          <VirtualList
            activeIndex={focusedTreeIndex}
            ariaActiveDescendant={effectiveFocusedTreeKey ? treeRowDomId(treeRows[focusedTreeIndex]) : undefined}
            ariaLabel="Model"
            ariaMultiselectable
            className="model-tree-virtual"
            height={420}
            itemKey={(row) => row.key}
            items={treeRows}
            onKeyDown={(event) => handleTreeKeyDown(event, treeRows, focusedTreeIndex, (index) => {
              const row = treeRows[index];
              setFocusedTreeKey(row.key);
              onFocusChange(row.kind === "entity" ? entityKey(row.item) : null);
              if (row.kind === "group") {
                if (
                  event.key === "Enter" ||
                  event.key === " " ||
                  (event.key === "ArrowRight" && !row.expanded) ||
                  (event.key === "ArrowLeft" && row.expanded && index === focusedTreeIndex)
                ) toggleExpandedGroup(row.groupKey);
                return;
              }
              if (event.key === "Enter") onSelect(row.item, {}, displayedEntityOrder);
              if (event.key === " ") onSelect(row.item, {
                additive: event.shiftKey,
                range: event.shiftKey,
                toggle: event.ctrlKey || event.metaKey
              }, displayedEntityOrder);
            })}
            renderItem={(row) => row.kind === "group" ? (
              <button
                aria-expanded={row.expanded}
                aria-level={1}
                aria-posinset={row.position}
                aria-setsize={row.setSize}
                className="tree-row tree-group-row"
                data-testid={`tree-group-${encodeURIComponent(row.groupKey)}`}
                id={treeRowDomId(row)}
                onClick={(event) => {
                  setFocusedTreeKey(row.key);
                  onFocusChange(null);
                  toggleExpandedGroup(row.groupKey);
                  event.currentTarget.closest<HTMLElement>('[role="tree"]')?.focus();
                }}
                role="treeitem"
                tabIndex={-1}
                type="button"
              >
                <span aria-hidden="true">{row.expanded ? "▾" : "▸"}</span><strong>{row.title}</strong>
              </button>
            ) : (
              <TreeButton
                active={selectedKeys.has(entityKey(row.item))}
                focused={effectiveFocusedTreeKey === row.key}
                hidden={hiddenKeys.has(entityKey(row.item))}
                item={row.item}
                level={row.level}
                onClick={(event) => {
                  setFocusedTreeKey(row.key);
                  onFocusChange(entityKey(row.item));
                  onSelect(row.item, {
                    additive: event.shiftKey,
                    range: event.shiftKey,
                    toggle: event.ctrlKey || event.metaKey
                  }, displayedEntityOrder);
                  event.currentTarget.closest<HTMLElement>('[role="tree"]')?.focus();
                }}
                position={row.position}
                setSize={row.setSize}
              />
            )}
            role="tree"
            rowHeight={density === "compact" ? 28 : 34}
            pinIndex={focusedTreeIndex}
            tabIndex={0}
            testId="model-tree-virtual"
          />
        ) : (
            <p className="muted" data-testid="model-tree-filter-empty">
              No model entities match this filter.
            </p>
        )
      ) : null}
    </div>
  );

  function toggleExpandedGroup(groupKey: string) {
    if (filterText.trim()) {
      setFilteredCollapsedGroups((current) => {
        const next = new Set(current);
        if (next.has(groupKey)) next.delete(groupKey);
        else next.add(groupKey);
        return next;
      });
      return;
    }
    setExpandedGroups((current) => {
      const next = new Set(current);
      if (next.has(groupKey)) next.delete(groupKey);
      else next.add(groupKey);
      return next;
    });
  }
}

type TreeItem = {
  id: string;
  type: EntityRef["type"];
  label: string;
  detail?: string;
  icon: React.ReactNode;
  keywords: string[];
};

type TreeGroupModel = {
  title: string;
  items: TreeItem[];
};

type TreeModel = {
  project: TreeItem;
  groups: TreeGroupModel[];
  count: number;
};

type FilteredTreeModel = {
  project: TreeItem | null;
  groups: TreeGroupModel[];
  count: number;
};

function TreeControls({
  filterText,
  filteredCount,
  totalCount,
  onFilterChange
}: {
  filterText: string;
  filteredCount: number;
  totalCount: number;
  onFilterChange: (value: string, inputAt: number, inputEventTimeStamp: number) => void;
}) {
  return (
    <section className="model-tree-controls" aria-label="Model tree filtering" data-testid="model-tree-controls">
      <label>
        <Search size={14} aria-hidden="true" />
        <span>Filter model</span>
        <input
          aria-label="Filter model tree"
          data-testid="model-tree-filter-input"
          onChange={(event) => {
            const inputAt = performance.now();
            onFilterChange(event.target.value, inputAt, event.nativeEvent.timeStamp);
          }}
          placeholder="Name, ID, type, or group"
          type="search"
          value={filterText}
        />
      </label>
      <span data-testid="model-tree-filter-summary">
        {filteredCount} of {totalCount} model entities visible
      </span>
      <button
        aria-label="Clear model tree filter"
        data-testid="clear-model-tree-filter"
        disabled={!filterText}
        onClick={(event) => onFilterChange("", performance.now(), event.nativeEvent.timeStamp)}
        type="button"
      >
        <X size={14} aria-hidden="true" />
      </button>
    </section>
  );
}

function buildTree(index: ModelIndex): TreeModel {
  const projectEntity = index.entities.get(index.projectKey)!;
  const project = indexedTreeItem(projectEntity.ref, projectEntity.label, projectEntity.record, projectEntity.geometryIssue);
  const groupOrder: ReadonlyArray<{ type: EntityRef["type"]; title: string }> = [
    { type: "material", title: "Materials" },
    { type: "section", title: "Sections" },
    { type: "node", title: "Nodes" },
    { type: "pipe", title: "Pipes" },
    { type: "support", title: "Supports" },
    { type: "component", title: "Components" },
    { type: "load", title: "Load Cases" },
    { type: "combination", title: "Combinations" },
    { type: "diagnostic", title: "Diagnostics" }
  ];
  const itemsByType = new Map<EntityRef["type"], TreeItem[]>();
  for (const key of index.treeOrder) {
    if (key === index.projectKey) continue;
    const entity = index.entities.get(key);
    if (!entity) continue;
    const items = itemsByType.get(entity.ref.type) ?? [];
    items.push(indexedTreeItem(entity.ref, entity.label, entity.record, entity.geometryIssue));
    itemsByType.set(entity.ref.type, items);
  }
  const groups = groupOrder.flatMap(({ type, title }) => {
    const items = itemsByType.get(type) ?? [];
    return items.length ? [{ title, items }] : [];
  });
  return { project, groups, count: index.treeOrder.length };
}

function indexedTreeItem(ref: EntityRef, label: string, record: unknown, geometryIssue: string | null): TreeItem {
  const values = record && typeof record === "object" ? record as Record<string, unknown> : {};
  const simpleTerms = ["id", "label", "name", "from", "to", "node", "kind", "status", "family", "basis", "code", "message"]
    .flatMap((key) => typeof values[key] === "string" ? [values[key] as string] : []);
  return {
    id: ref.id,
    type: ref.type,
    label,
    detail: geometryIssue ? `${ref.id} · invalid geometry` : ref.id,
    icon: treeItemIcon(ref.type),
    keywords: [ref.id, label, ref.type, ...simpleTerms, ...legacyTreeKeywords(ref.type, record), geometryIssue ?? ""]
  };
}

function legacyTreeKeywords(type: EntityRef["type"], record: unknown): string[] {
  if (type === "project") {
    const value = (record as PreviewModel).project;
    return [value.description, "project", "model"];
  }
  if (type === "material") {
    const value = record as NonNullable<PreviewModel["materials"]>[number];
    return [value.provenance, "material", "materials"];
  }
  if (type === "section") {
    const value = record as NonNullable<PreviewModel["sections"]>[number];
    return [value.section_type, provenanceKeyword(value.provenance), "section", "sections", "pipe section"];
  }
  if (type === "node") {
    const value = record as PreviewModel["nodes"][number];
    return [value.provenance, "node", "nodes"];
  }
  if (type === "pipe") {
    const value = record as PreviewModel["pipe_segments"][number];
    return [value.material, value.provenance, "pipe", "pipe segment"];
  }
  if (type === "support") {
    const value = record as PreviewModel["supports"][number];
    return [value.hanger?.hanger_type ?? "", value.hanger?.source_reference ?? "",
      value.hanger?.mechanics_consumption ?? "", value.restraints.join(" "), value.provenance, "support", "supports"];
  }
  if (type === "component") {
    const value = record as PreviewModel["components"][number];
    return [value.provenance, legacyComponentKeyword(value), "component", "components"];
  }
  if (type === "load") {
    const value = record as PreviewModel["load_cases"][number];
    return [value.provenance, "load", "load case"];
  }
  if (type === "combination") {
    const value = record as NonNullable<PreviewModel["combinations"]>[number];
    return [value.provenance, "combination", "combinations"];
  }
  return [];
}

function legacyComponentKeyword(component: PreviewModel["components"][number]): string {
  return [
    component.geometry?.bend_radius
      ? `${component.geometry.bend_radius.value} ${component.geometry.bend_radius.unit}`
      : "",
    component.geometry?.bend_angle
      ? `${component.geometry.bend_angle.value} ${component.geometry.bend_angle.unit}`
      : "",
    component.geometry?.bend_plane_orientation ?? "",
    component.geometry?.bend_pipe_ref ?? "",
    component.geometry?.bend_geometry_source_reference ?? "",
    component.geometry?.branch_header_pipe_ref ?? "",
    component.geometry?.branch_branch_pipe_ref ?? "",
    component.geometry?.branch_run_size
      ? `${component.geometry.branch_run_size.value} ${component.geometry.branch_run_size.unit}`
      : "",
    component.geometry?.branch_header_size
      ? `${component.geometry.branch_header_size.value} ${component.geometry.branch_header_size.unit}`
      : "",
    component.geometry?.branch_connection_angle
      ? `${component.geometry.branch_connection_angle.value} ${component.geometry.branch_connection_angle.unit}`
      : "",
    component.geometry?.branch_connection_type ?? "",
    component.geometry?.branch_reinforcement_reference ?? "",
    component.geometry?.branch_geometry_source_reference ?? "",
    component.geometry?.rigid_pipe_ref ?? "",
    component.geometry?.rigid_body_length
      ? `${component.geometry.rigid_body_length.value} ${component.geometry.rigid_body_length.unit}`
      : "",
    component.geometry?.end_a_size
      ? `${component.geometry.end_a_size.value} ${component.geometry.end_a_size.unit}`
      : "",
    component.geometry?.end_b_size
      ? `${component.geometry.end_b_size.value} ${component.geometry.end_b_size.unit}`
      : "",
    component.geometry?.weight ? `${component.geometry.weight.value} ${component.geometry.weight.unit}` : "",
    component.geometry?.center_of_gravity
      ? `${component.geometry.center_of_gravity.x} ${component.geometry.center_of_gravity.y} ${component.geometry.center_of_gravity.z} ${component.geometry.center_of_gravity.unit}`
      : "",
    component.geometry?.connection_end_a_reference ?? "",
    component.geometry?.connection_end_b_reference ?? "",
    component.geometry?.stiffness_behavior_reference ?? "",
    component.geometry?.rigid_component_source_reference ?? "",
    component.geometry?.expansion_joint_pipe_ref ?? "",
    component.geometry?.effective_area
      ? `${component.geometry.effective_area.value} ${component.geometry.effective_area.unit}`
      : "",
    component.geometry?.movement_limit
      ? `${component.geometry.movement_limit.value} ${component.geometry.movement_limit.unit}`
      : "",
    component.geometry?.hardware_reference ?? "",
    component.geometry?.manufacturer_reference ?? "",
    component.geometry?.pressure_thrust_reference ?? "",
    component.geometry?.expansion_joint_source_reference ?? "",
    component.modifiers?.sif_user_value
      ? `${component.modifiers.sif_user_value.value} ${component.modifiers.sif_user_value.unit}`
      : "",
    component.modifiers?.branch_header_sif_user_value
      ? `${component.modifiers.branch_header_sif_user_value.value} ${component.modifiers.branch_header_sif_user_value.unit}`
      : "",
    component.modifiers?.branch_branch_sif_user_value
      ? `${component.modifiers.branch_branch_sif_user_value.value} ${component.modifiers.branch_branch_sif_user_value.unit}`
      : "",
    component.modifiers?.flexibility_factor_user_value
      ? `${component.modifiers.flexibility_factor_user_value.value} ${component.modifiers.flexibility_factor_user_value.unit}`
      : "",
    component.modifiers?.stiffness_scaling_user_value
      ? `${component.modifiers.stiffness_scaling_user_value.value} ${component.modifiers.stiffness_scaling_user_value.unit}`
      : "",
    component.modifiers?.linear_stiffness_user_value
      ? `${component.modifiers.linear_stiffness_user_value.value} ${component.modifiers.linear_stiffness_user_value.unit}`
      : "",
    component.modifiers?.rotational_stiffness_user_value
      ? `${component.modifiers.rotational_stiffness_user_value.value} ${component.modifiers.rotational_stiffness_user_value.unit}`
      : "",
    component.modifiers?.axial_stiffness_user_value
      ? `${component.modifiers.axial_stiffness_user_value.value} ${component.modifiers.axial_stiffness_user_value.unit}`
      : "",
    component.modifiers?.lateral_stiffness_user_value
      ? `${component.modifiers.lateral_stiffness_user_value.value} ${component.modifiers.lateral_stiffness_user_value.unit}`
      : "",
    component.modifiers?.angular_stiffness_user_value
      ? `${component.modifiers.angular_stiffness_user_value.value} ${component.modifiers.angular_stiffness_user_value.unit}`
      : "",
    component.modifiers?.torsional_stiffness_user_value
      ? `${component.modifiers.torsional_stiffness_user_value.value} ${component.modifiers.torsional_stiffness_user_value.unit}`
      : "",
    component.modifiers?.source_reference ?? "",
    component.mechanics_interface?.solver_consumption ?? "",
    component.completeness?.map((finding) => `${finding.diagnostic_code} ${finding.status}`).join(" ") ?? ""
  ].join(" ");
}

function treeItemIcon(type: EntityRef["type"]): React.ReactNode {
  if (type === "project") return <SquareStack size={16} aria-hidden="true" />;
  if (type === "node") return <CircleDot size={14} aria-hidden="true" />;
  if (type === "pipe" || type === "combination") return <GitBranch size={14} aria-hidden="true" />;
  if (type === "support") return <Anchor size={14} aria-hidden="true" />;
  if (type === "component") return <Waypoints size={14} aria-hidden="true" />;
  if (type === "load") return <Zap size={14} aria-hidden="true" />;
  return <Box size={14} aria-hidden="true" />;
}

function filterTree(tree: TreeModel, filterText: string): FilteredTreeModel {
  const query = filterText.trim().toLowerCase();
  if (!query) {
    return { project: tree.project, groups: tree.groups, count: tree.count };
  }

  const project = matchesTreeItem(tree.project, "Project", query) ? tree.project : null;
  const groups = tree.groups
    .map((group) => ({
      title: group.title,
      items: group.items.filter((item) => matchesTreeItem(item, group.title, query))
    }))
    .filter((group) => group.items.length > 0);

  return {
    project,
    groups,
    count: (project ? 1 : 0) + groups.reduce((count, group) => count + group.items.length, 0)
  };
}

function matchesTreeItem(item: TreeItem, groupTitle: string, query: string): boolean {
  return [...item.keywords, item.type, item.label, item.detail, groupTitle]
    .filter((value): value is string => Boolean(value))
    .join(" ")
    .toLowerCase()
    .includes(query);
}

function TreeButton({ active, focused, hidden, item, level, onClick, position, setSize }: {
  active: boolean;
  focused: boolean;
  hidden: boolean;
  item: TreeItem;
  level: number;
  onClick: (event: React.MouseEvent<HTMLButtonElement>) => void;
  position: number;
  setSize: number;
}) {
  return (
    <button
      className={`tree-row ${active ? "active" : ""} ${focused ? "focused" : ""} ${hidden ? "hidden-entity" : ""}`}
      data-testid={`tree-row-${encodeURIComponent(item.type)}-${encodeURIComponent(item.id)}`}
      id={`tree-row-${encodeURIComponent(item.type)}-${encodeURIComponent(item.id)}`}
      aria-level={level}
      aria-posinset={position}
      aria-selected={active}
      aria-setsize={setSize}
      title={`${item.label}${item.detail ? ` (${item.detail})` : ""}`}
      onClick={onClick}
      role="treeitem"
      tabIndex={-1}
      type="button"
    >
      {item.icon}
      <span>
        <strong>{item.label}</strong>
        {item.detail ? <small>{item.detail}</small> : null}
      </span>
      {hidden ? <span className="tree-visibility-state">Hidden</span> : null}
    </button>
  );
}

type FlatTreeRow =
  | Readonly<{ kind: "group"; key: string; groupKey: string; title: string; expanded: boolean; position: number; setSize: number }>
  | Readonly<{ kind: "entity"; key: string; item: TreeItem; level: number; position: number; setSize: number }>;

function flattenTreeRows(tree: FilteredTreeModel, expandedGroups: ReadonlySet<string>, filtering: boolean, filteredCollapsedGroups: ReadonlySet<string>): FlatTreeRow[] {
  const rows: FlatTreeRow[] = [];
  const topLevelSize = tree.groups.length + (tree.project ? 1 : 0);
  if (tree.project) rows.push({
    kind: "entity", key: entityKey(tree.project), item: tree.project, level: 1, position: 1, setSize: topLevelSize
  });
  tree.groups.forEach((group, groupIndex) => {
    const groupKey = group.title;
    const expanded = filtering ? !filteredCollapsedGroups.has(group.title) : expandedGroups.has(group.title);
    rows.push({
      kind: "group", key: `group:${groupKey}`, groupKey, title: group.title, expanded,
      position: groupIndex + (tree.project ? 2 : 1), setSize: topLevelSize
    });
    if (!expanded) return;
    group.items.forEach((item, itemIndex) => rows.push({
      kind: "entity", key: entityKey(item), item, level: 2, position: itemIndex + 1, setSize: group.items.length
    }));
  });
  return rows;
}

function handleTreeKeyDown(
  event: KeyboardEvent<HTMLDivElement>, rows: readonly FlatTreeRow[], currentIndex: number,
  activate: (index: number) => void
): void {
  if (rows.length === 0) return;
  let next = currentIndex;
  const current = rows[currentIndex];
  if (event.key === "ArrowRight") {
    if (current?.kind !== "group") return;
    // WAI-ARIA tree behavior: expand a closed branch, then move to its first
    // child on a subsequent Right press while it is open.
    next = current.expanded && rows[currentIndex + 1]?.kind === "entity"
      ? currentIndex + 1
      : currentIndex;
  } else if (event.key === "ArrowLeft") {
    if (current?.kind === "group") {
      if (!current.expanded) return;
    } else if (current?.kind === "entity" && current.level > 1) {
      let parentIndex = currentIndex - 1;
      while (parentIndex >= 0 && rows[parentIndex].kind !== "group") parentIndex -= 1;
      if (parentIndex < 0) return;
      next = parentIndex;
    } else return;
  } else if (event.key === "ArrowDown") next = Math.min(rows.length - 1, currentIndex + 1);
  else if (event.key === "ArrowUp") next = Math.max(0, currentIndex - 1);
  else if (event.key === "Home") next = 0;
  else if (event.key === "End") next = rows.length - 1;
  else if (event.key !== "Enter" && event.key !== " ") return;
  activate(next);
  event.preventDefault();
}

function treeRowDomId(row: FlatTreeRow): string {
  return row.kind === "group"
    ? `tree-group-${encodeURIComponent(row.groupKey)}`
    : `tree-row-${encodeURIComponent(row.item.type)}-${encodeURIComponent(row.item.id)}`;
}

function EntityGrid({
  compact, toolbarTarget,
  bounded,
  density,
  drafts,
  entityType,
  filterText,
  focusedGridKey,
  model,
  onEntityTypeChange,
  onQueueIntent,
  onApplyCellIntent,
  operationBusy,
  onSelect,
  selection,
  setDrafts,
  setFocusedGridKey,
  projectSessionGeneration
}: {
  bounded: boolean;
  compact: boolean;
  toolbarTarget: HTMLDivElement | null;
  density: "comfortable" | "compact";
  drafts: Record<string, string>;
  entityType: GridEntityType;
  filterText: string;
  focusedGridKey: EntityKey | null;
  model: PreviewModel;
  onEntityTypeChange: (entityType: GridEntityType) => void;
  onQueueIntent?: (intent: EditorOperationIntent) => void;
  onApplyCellIntent?: (intent: EditorOperationIntent) => Promise<TableApplyResult>;
  operationBusy?: boolean;
  onSelect: (selection: EntityRef) => void;
  selection: EntityRef;
  setDrafts: React.Dispatch<React.SetStateAction<Record<string, string>>>;
  setFocusedGridKey: React.Dispatch<React.SetStateAction<EntityKey | null>>;
  projectSessionGeneration: number;
}) {
  const [queuedMessage, setQueuedMessage] = useState("");
  const [reviewOpen, setReviewOpen] = useState(false);
  const [materialReviewOpen, setMaterialReviewOpen] = useState(false);
  const [sectionReviewOpen, setSectionReviewOpen] = useState(false);
  const [sectionDraftRetained, setSectionDraftRetained] = useState(false);
  const [sectionReviewReset, setSectionReviewReset] = useState(0);
  const [materialDraftRetained, setMaterialDraftRetained] = useState(false);
  const [directDraftRetained, setDirectDraftRetained] = useState(false);
  const [reviewReset, setReviewReset] = useState(0);
  const [materialReviewReset, setMaterialReviewReset] = useState(0);
  const commonFamily = entityType === "nodes" || entityType === "materials" || entityType === "sections";
  const currentReviewOpen = entityType === "sections" ? sectionReviewOpen : entityType === "materials" ? materialReviewOpen : reviewOpen;
  const reviewVisible = !commonFamily || currentReviewOpen;
  const directVisible = entityType === "nodes" && !reviewOpen;
  const reviewRegionId = useId();
  const bulkBody = useTableBodyHeight(bounded, reviewVisible);
  const nodeRows = useMemo(() => gridRows(model, "nodes"), [model]);
  const nodeGridColumns = useMemo(() => gridColumns(model, "nodes"), [model]);
  const coordinateColumns = useMemo(() => nodeTableColumns(model.project.units.length ?? ""), [model.project.units.length]);
  const reviewColumns = useMemo(() => nodeTableColumns(model.project.units.length ?? "", true), [model.project.units.length]);
  const tableRows: TableRow[] = useMemo(() => nodeRows.map((row) => ({ key: entityKey(row), label: row.id, searchText: row.searchText,
    cells: Object.fromEntries(nodeGridColumns.map((column) => [column.key, { value: column.value(row), unit: column.quantity ? model.project.units.length ?? "" : column.unit(row), readonly: gridCellReadonly(column, row) }]))
  })), [nodeRows, nodeGridColumns, model.project.units.length]);
  const reviewRows: TableRow[] = useMemo(() => tableRows.map((tableRow, index) => ({ ...tableRow,
    cells: Object.fromEntries(nodeGridColumns.map((column) => [column.key, { ...tableRow.cells[column.key],
      value: drafts[draftKey(nodeRows[index], column, projectSessionGeneration)] ?? tableRow.cells[column.key].value,
      readout: column.quantity ? <QuantityReadout quantity={{ value: numericGridValue(column.value(nodeRows[index])), unit: column.unit(nodeRows[index]), dimension_id: column.dimension }} /> : undefined }]))
  })), [tableRows, nodeRows, nodeGridColumns, drafts, projectSessionGeneration]);
  const tableGeneration = JSON.stringify([model.project.id, projectSessionGeneration]);
  const operationSequence = useRef(0);
  const retainedMaterialDrafts = changedGridCells({ columns: gridColumns(model, "materials"), drafts, rows: gridRows(model, "materials"), projectSessionGeneration }).length;
  // Retained raw enum drafts may be ineligible for Queue, but remain visible here.
  const retainedSectionDrafts = gridRows(model, "sections").reduce((count, row) => count + gridColumns(model, "sections").filter((column) => {
    const value = drafts[draftKey(row, column, projectSessionGeneration)];
    return value !== undefined && value !== column.value(row);
  }).length, 0);
  const retainedNodeDrafts = changedGridCells({ columns: gridColumns(model, "nodes"), drafts, rows: nodeRows, projectSessionGeneration }).length;
  async function applyCoordinate(captured: CapturedCell, value: string): Promise<TableApplyResult> {
    if (!onApplyCellIntent) return { applied: false, messages: ["The operation route is unavailable."] };
    if (!capturedCellIsCurrent(captured, tableRows, tableGeneration)) return { applied: false, messages: ["The cell or project changed. Cancel and edit the current value."] };
    const row = nodeRows.find((candidate) => entityKey(candidate) === captured.rowKey);
    const column = nodeGridColumns.find((candidate) => candidate.key === captured.columnKey);
    if (!row || !column) return { applied: false, messages: ["The target cell is unavailable."] };
    if (column.quantity && !captured.unit.trim()) return { applied: false, messages: ["The model has no declared length unit. Direct coordinate Apply is unavailable."] };
    const capturedColumn = { ...column, unit: () => captured.unit, value: () => captured.before };
    const intent = buildGridOperationIntent({ row, column: capturedColumn, model, sequence: ++operationSequence.current, value, interaction: "cell" });
    // Direct edits need unique outcome identity even after a view/session remount.
    const identity = crypto.randomUUID();
    intent.operation_id = `op:table-cell-${identity}`;
    intent.change.change_id = `change:table-cell-${identity}`;
    return onApplyCellIntent(intent);
  }
  const rows = useMemo(() => gridRows(model, entityType), [model, entityType]);
  const columns = useMemo(() => gridColumns(model, entityType), [model, entityType]);
  const visibleRows = useMemo(() => {
    const query = filterText.trim().toLowerCase();
    return query ? rows.filter((row) => row.searchText.includes(query)) : rows;
  }, [filterText, rows]);
  const changedCells = useMemo(
    () => changedGridCells({ columns, drafts, rows: visibleRows, projectSessionGeneration }),
    [columns, drafts, projectSessionGeneration, visibleRows]
  );
  const focusedGridIndex = focusedGridKey === null ? null : visibleRows.findIndex(
    (row) => entityKey({ type: row.type, id: row.id }) === focusedGridKey
  );
  const queueDisabled = !onQueueIntent || changedCells.length === 0;
  const virtualGrid = visibleRows.length >= 100;

  useEffect(() => {
    setQueuedMessage("");
  }, [projectSessionGeneration]);

  function updateCell(row: GridRow, column: GridColumn, value: string) {
    setDrafts((current) => {
      const key = draftKey(row, column, projectSessionGeneration);
      const baseValue = column.value(row);
      const next = { ...current };
      if (value === baseValue) {
        delete next[key];
      } else {
        next[key] = value;
      }
      return next;
    });
  }

  function handleQueueChangedCells() {
    if (!onQueueIntent || changedCells.length === 0) return;
    changedCells.forEach(({ column, row, value }, index) => {
      onQueueIntent(
        buildGridOperationIntent({
          column,
          model,
          row,
          sequence: index + 1,
          value
        })
      );
    });
    if (entityType === "sections") setSectionReviewReset((value) => value + 1);
    else if (entityType === "materials") setMaterialReviewReset((value) => value + 1);
    else setReviewReset((value) => value + 1);
    setQueuedMessage(
      `Queued ${changedCells.length} review intent${changedCells.length === 1 ? "" : "s"} from Grid mode.`
    );
    setDrafts((current) => {
      const next = { ...current };
      for (const { column, row } of changedCells) delete next[draftKey(row, column, projectSessionGeneration)];
      return next;
    });
  }

  const compactInfoId = useId();
  const gridActions = <div className="entity-grid-actions">
        <button
          data-testid="queue-entity-grid-intents"
          disabled={queueDisabled}
          onClick={handleQueueChangedCells}
          type="button"
        >
          <Table2 size={14} aria-hidden="true" />
          {compact ? "Queue" : "Queue changed cells"}
        </button>
        <button
          data-testid="clear-entity-grid-drafts"
          disabled={entityType === "materials" || entityType === "sections" ? Object.keys(drafts).length === 0 : changedCells.length === 0}
          onClick={() => { setReviewReset((value) => value + 1); setMaterialReviewReset((value) => value + 1); setSectionReviewReset((value) => value + 1); setDrafts({}); }}
          type="button"
        >
          <X size={14} aria-hidden="true" />
          {compact ? "Clear" : "Clear grid edits"}
        </button>
      </div>;
  const reviewToggle = <button className="entity-grid-review-toggle" hidden={!commonFamily} data-testid={`${entityType === "sections" ? "section" : entityType === "materials" ? "material" : "node"}-grid-review-disclosure`}
        type="button" aria-expanded={reviewVisible} aria-controls={reviewRegionId} onClick={() => entityType === "sections" ? setSectionReviewOpen((value) => !value) : entityType === "materials" ? setMaterialReviewOpen((value) => !value) : setReviewOpen((value) => !value)}>
        {currentReviewOpen ? `Return to ${entityType === "sections" ? "section" : entityType === "materials" ? "material" : "node"} fields` : (compact ? "Review changes" : "Review multiple changes")}{!currentReviewOpen && (entityType === "sections" ? retainedSectionDrafts : entityType === "materials" ? retainedMaterialDrafts : retainedNodeDrafts) > 0 ? ` · ${entityType === "sections" ? retainedSectionDrafts : entityType === "materials" ? retainedMaterialDrafts : retainedNodeDrafts} retained draft${(entityType === "sections" ? retainedSectionDrafts : entityType === "materials" ? retainedMaterialDrafts : retainedNodeDrafts) === 1 ? "" : "s"}` : ""}
      </button>;
  const compactControls = <>
    <label className="compact-family-label">Family
      <select aria-label="Grid family" data-testid="entity-grid-family" value={entityType} onChange={(event) => onEntityTypeChange(event.target.value as GridEntityType)}>
        {GRID_ENTITY_TYPES.map((item) => <option key={item.id} value={item.id}>{item.label}{item.id === entityType ? ` (${rows.length})` : ""}</option>)}
      </select>
    </label>
    {reviewToggle}
    {reviewVisible ? gridActions : null}
    {queuedMessage ? <span role="status" className="compact-queued-message">{queuedMessage}</span> : null}
    <button type="button" popoverTarget={compactInfoId} aria-label="Table details" className="compact-table-info">Details</button>
    <div id={compactInfoId} popover="auto" className="compact-table-details" role="dialog" aria-label="Table details" tabIndex={-1}
      onToggle={(event) => { if (event.currentTarget.matches(":popover-open")) event.currentTarget.focus(); }}>
      <p>{visibleRows.length} of {rows.length} {GRID_ENTITY_TYPES.find((item) => item.id === entityType)?.label}; {changedCells.length} changed cells.</p>
      <p>Grid mode fans each changed cell into a structured review intent; storage remains local. {commonFamily ? "Blank or whitespace text becomes TBD when queued; keeping a draft does not change the model." : ""}</p>
      {directDraftRetained ? <p>Direct node edit retained. Return to node fields to correct or cancel it.</p> : null}
      {materialDraftRetained ? <p>Direct material edit retained. Return to material fields to correct or cancel it.</p> : null}
      {sectionDraftRetained ? <p>Direct section edit retained. Return to section fields to correct or cancel it.</p> : null}
    </div>
  </>;
  return (
    <section className={`entity-grid${bounded ? " bounded" : ""}`} aria-label="Bulk entity grid" data-testid="entity-grid">
      {compact && toolbarTarget ? createPortal(compactControls, toolbarTarget) : null}
      <div hidden={compact} className="entity-grid-tabs" aria-label="Grid entity type">
        {GRID_ENTITY_TYPES.map((item) => (
          <button
            aria-pressed={item.id === entityType}
            data-testid={`entity-grid-type-${item.id}`}
            key={item.id}
            onClick={() => onEntityTypeChange(item.id)}
            type="button"
          >
            {item.label}
          </button>
        ))}
      </div>
      {!directVisible && directDraftRetained ? <p className="retained-direct-draft" role="status" data-testid="retained-direct-draft">Direct node edit retained. Return to node fields to correct or cancel it.</p> : null}
      <div className="direct-coordinate-workarea" hidden={!directVisible} inert={!directVisible}>
        <EngineeringTable label="Node fields" bounded={bounded} compact={compact} active={directVisible} onDraftStateChange={setDirectDraftRetained} rows={tableRows} columns={coordinateColumns} generation={tableGeneration}
          filter={filterText} density={density} selectedKey={entityKey(selection)} busy={operationBusy}
          onSelect={(key) => { const row = nodeRows.find((candidate) => entityKey(candidate) === key); if (row) onSelect({ type: row.type, id: row.id }); }}
          onApply={applyCoordinate} />
      </div>
      {! (entityType === "materials" && !materialReviewOpen) && materialDraftRetained ? <p className="retained-direct-draft" role="status">Direct material edit retained. Return to material fields to correct or cancel it.</p> : null}
      <div className="direct-coordinate-workarea" hidden={entityType !== "materials" || materialReviewOpen} inert={entityType !== "materials" || materialReviewOpen}>
        <LibraryTable family="materials" model={model} drafts={drafts} generation={tableGeneration} projectSessionGeneration={projectSessionGeneration}
          active={entityType === "materials" && !materialReviewOpen} bounded={bounded} compact={compact} density={density} filter={filterText} selection={selection}
          onSelect={onSelect} onApplyCellIntent={onApplyCellIntent} operationBusy={operationBusy} onDraftStateChange={setMaterialDraftRetained}
          updateCell={updateCell} resetEditsKey={materialReviewReset} />
      </div>
      {!(entityType === "sections" && !sectionReviewOpen) && sectionDraftRetained ? <p className="retained-direct-draft" role="status">Direct section edit retained. Return to section fields to correct or cancel it.</p> : null}
      <div className="direct-coordinate-workarea" hidden={entityType !== "sections" || sectionReviewOpen} inert={entityType !== "sections" || sectionReviewOpen}>
        <LibraryTable family="sections" model={model} drafts={drafts} generation={tableGeneration} projectSessionGeneration={projectSessionGeneration}
          active={entityType === "sections" && !sectionReviewOpen} bounded={bounded} compact={compact} density={density} filter={filterText} selection={selection}
          onSelect={onSelect} onApplyCellIntent={onApplyCellIntent} operationBusy={operationBusy} onDraftStateChange={setSectionDraftRetained}
          updateCell={updateCell} resetEditsKey={sectionReviewReset} />
      </div>
      <div className={`entity-grid-review${!commonFamily ? " other-family" : ""}`} id={reviewRegionId} hidden={!reviewVisible} inert={!reviewVisible}>
        <div className="entity-grid-review-content">
      <div className="entity-grid-summary">
        <span data-testid="entity-grid-summary">
          {visibleRows.length} of {rows.length}{" "}
          {GRID_ENTITY_TYPES.find((item) => item.id === entityType)?.label ?? "rows"}
        </span>
        <span data-testid="entity-grid-change-count">{changedCells.length} changed cells</span>
      </div>
      <div className="node-review-workarea" hidden={entityType !== "nodes"} inert={entityType !== "nodes"}>
        <EngineeringTable label="Node review drafts" persistentEditor policy="review" resetEditsKey={reviewReset} bounded={bounded} compact={compact} active={reviewVisible && entityType === "nodes"}
          rows={reviewRows} columns={reviewColumns} generation={tableGeneration} filter={filterText} density={density} selectedKey={entityKey(selection)}
          onSelect={(key) => { const row = nodeRows.find((candidate) => entityKey(candidate) === key); if (row) onSelect({ type: row.type, id: row.id }); }}
          onDraftChange={(captured, text) => {
            const row = nodeRows.find((candidate) => entityKey(candidate) === captured.rowKey);
            const column = nodeGridColumns.find((candidate) => candidate.key === captured.columnKey);
            if (row && column && !gridCellReadonly(column, row) && captured.generation === tableGeneration) updateCell(row, column, text);
          }}
          onKeepDraft={() => ({ retained: true, messages: ["Draft retained; model unchanged."] })} />
      </div>
      <div className="node-review-workarea" hidden={entityType !== "materials"} inert={entityType !== "materials"}>
        <LibraryTable family="materials" model={model} drafts={drafts} generation={tableGeneration} projectSessionGeneration={projectSessionGeneration} review
          active={reviewVisible && entityType === "materials"} bounded={bounded} compact={compact} density={density} filter={filterText} selection={selection}
          onSelect={onSelect} updateCell={updateCell} resetEditsKey={materialReviewReset} />
      </div>
      <div className="node-review-workarea" hidden={entityType !== "sections"} inert={entityType !== "sections"}>
        <LibraryTable family="sections" model={model} drafts={drafts} generation={tableGeneration} projectSessionGeneration={projectSessionGeneration} review
          active={reviewVisible && entityType === "sections"} bounded={bounded} compact={compact} density={density} filter={filterText} selection={selection}
          onSelect={onSelect} updateCell={updateCell} resetEditsKey={sectionReviewReset} />
      </div>
      {!commonFamily ? <div className="entity-grid-scroll" role="region" aria-label="Editable model entity table">
        {virtualGrid ? (
          <div className="entity-grid-virtual-table" role="table" aria-label={`${entityType} editable grid`} data-testid={`entity-grid-table-${entityType}`}>
            <div className="entity-grid-virtual-row header" role="row" style={{ gridTemplateColumns: `minmax(100px, 1fr) repeat(${columns.length}, minmax(120px, 1fr))` }}>
              <strong role="columnheader">ID</strong>
              {columns.map((column) => <strong role="columnheader" key={column.key}>{column.label}</strong>)}
            </div>
            <div className="bulk-grid-body-slot" ref={bulkBody.ref}>
            <VirtualList
              height={bounded ? Math.min(bulkBody.height, visibleRows.length * (density === "compact" ? 36 : 42)) : 360}
              itemKey={(row) => `${row.type}:${row.id}`}
              items={visibleRows}
              pinIndex={focusedGridIndex === -1 ? null : focusedGridIndex}
              renderItem={(row) => (
                <div
                  className={`entity-grid-virtual-row${selection.type === row.type && selection.id === row.id ? " active" : ""}`}
                  onBlurCapture={(event) => { if (!event.currentTarget.contains(event.relatedTarget as Node | null)) setFocusedGridKey(null); }}
                  onFocusCapture={() => setFocusedGridKey(entityKey({ type: row.type, id: row.id }))}
                  role="row"
                  style={{ gridTemplateColumns: `minmax(100px, 1fr) repeat(${columns.length}, minmax(120px, 1fr))` }}
                >
                  <div role="rowheader"><button aria-pressed={selection.type === row.type && selection.id === row.id} data-testid={`entity-grid-row-${safeToken(row.type)}-${safeToken(row.id)}`} onClick={() => onSelect({ type: row.type, id: row.id })} type="button">{row.id}</button></div>
                  {columns.map((column) => {
                    const key = draftKey(row, column, projectSessionGeneration);
                    const value = drafts[key] ?? column.value(row);
                    return <div role="cell" key={column.key}>{gridCellControl(row, column, value, updateCell)}</div>;
                  })}
                </div>
              )}
              role="rowgroup"
              rowHeight={density === "compact" ? 36 : 42}
              testId="entity-grid-virtual-rows"
            />
            </div>
          </div>
        ) : (
        <table data-testid={`entity-grid-table-${entityType}`}>
          <thead>
            <tr>
              <th scope="col">ID</th>
              {columns.map((column) => (
                <th key={column.key} scope="col">
                  {column.label}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {visibleRows.map((row) => (
              <tr key={`${row.type}:${row.id}`} className={selection.type === row.type && selection.id === row.id ? "active" : ""}>
                <th scope="row">
                  <button
                    aria-pressed={selection.type === row.type && selection.id === row.id}
                    data-testid={`entity-grid-row-${safeToken(row.type)}-${safeToken(row.id)}`}
                    onClick={() => onSelect({ type: row.type, id: row.id })}
                    type="button"
                  >
                    {row.id}
                  </button>
                </th>
                {columns.map((column) => {
                  const key = draftKey(row, column, projectSessionGeneration);
                  const value = drafts[key] ?? column.value(row);
                  return (
                    <td key={column.key}>
                      {gridCellReadonly(column, row) ? (
                        <span data-testid={`entity-grid-cell-${safeToken(row.id)}-${safeToken(column.key)}`}>
                          {column.quantity ? <QuantityReadout quantity={{ value: numericGridValue(column.value(row)), unit: column.unit(row), dimension_id: column.dimension }} /> : value}
                        </span>
                      ) : column.options ? (
                        <select aria-label={`${row.id} ${column.label}`} data-testid={`entity-grid-input-${safeToken(row.id)}-${safeToken(column.key)}`} value={value} onChange={(event) => updateCell(row, column, event.target.value)}>
                          {!column.options.includes(value) ? <option value={value} disabled>{value} (existing)</option> : null}
                          {column.options.map((option) => <option key={option} value={option}>{option}</option>)}
                        </select>
                      ) : (
                        <input
                          aria-label={`${row.id} ${column.label}`}
                          data-testid={`entity-grid-input-${safeToken(row.id)}-${safeToken(column.key)}`}
                          onChange={(event) => updateCell(row, column, event.target.value)}
                          value={value}
                        />
                      )}
                      {column.quantity && !gridCellReadonly(column, row) ? <small aria-label="Quantity readout"><QuantityReadout quantity={{ value: numericGridValue(column.value(row)), unit: column.unit(row), dimension_id: column.dimension }} /></small> : null}
                    </td>
                  );
                })}
              </tr>
            ))}
          </tbody>
        </table>
        )}
        {visibleRows.length === 0 ? (
          <p className="muted" data-testid="entity-grid-empty">
            No grid rows match this filter.
          </p>
        ) : null}
      </div>
      : null}
      {!commonFamily && virtualGrid && bulkBody.allocationConflict ? <p role="alert">No space is available for review rows. Expand the table view to continue.</p> : null}
      {!compact ? gridActions : null}
      <p className="muted entity-grid-boundary" data-testid="entity-grid-boundary">
        Grid mode fans each changed cell into a structured review intent; storage remains local. {commonFamily ? "Blank or whitespace text becomes TBD when queued; keeping a draft does not change the model." : ""}
      </p>
      {queuedMessage && !compact ? (
        <p className="entity-grid-queued" data-testid="entity-grid-queued-message">
          {queuedMessage}
        </p>
      ) : null}
        </div>
      </div>
      {!compact ? reviewToggle : null}
    </section>
  );
}


function actualLibraryQuantity(row: GridRow, fieldPath: string): { value: unknown; unit: string } {
  const raw = fieldPath.split(".").slice(0, -1).reduce<unknown>((value, key) => value && typeof value === "object" ? (value as Record<string, unknown>)[key] : undefined, row.raw);
  const quantity = raw && typeof raw === "object" ? raw as Record<string, unknown> : {};
  return { value: quantity.value, unit: typeof quantity.unit === "string" ? quantity.unit : "" };
}

/** The two mounted policies retain independent editor state; both use the same
 * rendering/interaction implementation and the existing operation route. */
function LibraryTable({ family, model, drafts, generation, projectSessionGeneration, review = false, active, bounded, compact, density, filter, selection,
  onSelect, onApplyCellIntent, operationBusy, onDraftStateChange, updateCell, resetEditsKey
}: {
  family: "materials" | "sections"; model: PreviewModel; drafts: Record<string, string>; generation: string; projectSessionGeneration: number; review?: boolean;
  active: boolean; bounded: boolean; compact: boolean; density: "comfortable" | "compact"; filter: string; selection: EntityRef;
  onSelect: (selection: EntityRef) => void; onApplyCellIntent?: (intent: EditorOperationIntent) => Promise<TableApplyResult>;
  operationBusy?: boolean; onDraftStateChange?: (retained: boolean) => void;
  updateCell: (row: GridRow, column: GridColumn, value: string) => void; resetEditsKey: number;
}) {
  const sourceRows = useMemo(() => gridRows(model, family), [model, family]);
  const grid = useMemo(() => gridColumns(model, family), [model, family]);
  const columns = useMemo(() => family === "sections" ? sectionTableColumns(review) : materialTableColumns(review), [review, family]);
  const quantities = useMemo(() => grid.filter((column) => column.quantity), [grid]);
  const rows: TableRow[] = useMemo(() => sourceRows.map((row) => ({ key: entityKey(row), label: row.id, searchText: row.searchText,
    cells: Object.fromEntries(grid.map((column) => {
      const quantity = column.quantity ? actualLibraryQuantity(row, column.fieldPath) : null;
      const unavailable = quantity && (typeof quantity.value !== "number" || !Number.isFinite(quantity.value) || !quantity.unit.trim());
      const draft = review ? drafts[draftKey(row, column, projectSessionGeneration)] : undefined;
      return [column.key, { value: draft ?? column.value(row), unit: column.unit(row), enumSource: column.options ? column.value(row) : undefined,
        showUnit: Boolean(quantity),
        sortBasis: quantity ? JSON.stringify([typeof quantity.value, Object.is(quantity.value, -0) ? "-0" : String(quantity.value), quantity.unit, draft ?? null]) : undefined,
        convertible: quantity ? draft !== undefined || (typeof quantity.value === "number" && Number.isFinite(quantity.value)) : undefined,
        readonly: gridCellReadonly(column, row) || (!review && Boolean(unavailable)),
        unavailable: !review && unavailable ? "Direct editing requires an existing finite quantity and its actual compatible unit." : undefined,
        readout: column.quantity ? <QuantityReadout quantity={{ value: numericGridValue(column.value(row)), unit: column.unit(row), dimension_id: column.dimension }} /> : undefined }];
    }))
  })), [sourceRows, grid, review, drafts, projectSessionGeneration]);
  const projection = useQuantitySortProjection(rows, quantities, generation, `${family}:${review ? "review" : "direct"}`);
  const projectedRows = useMemo(() => rows.map((row) => ({ ...row, cells: Object.fromEntries(Object.entries(row.cells).map(([key, cell]) => {
    if (!quantities.some((column) => column.key === key)) return [key, cell];
    const cellKey = quantityCellKey(row.key, key);
    const value = projection.sortValues.get(cellKey);
    const eligible = projection.eligibleCells.has(cellKey);
    return [key, { ...cell, sortValue: value, readonly: cell.readonly || (!review && !eligible),
      unavailable: cell.unavailable ?? (!review && !eligible ? "Direct editing unavailable while this quantity or its actual unit cannot be converted." : undefined) }];
  })) })), [rows, quantities, projection, review]);
  async function apply(captured: CapturedCell, value: string): Promise<TableApplyResult> {
    if (!onApplyCellIntent) return { applied: false, messages: ["The operation route is unavailable."] };
    if (!capturedCellIsCurrent(captured, projectedRows, generation)) return { applied: false, messages: ["The cell or project changed. Cancel and edit the current value."] };
    const row = sourceRows.find((candidate) => entityKey(candidate) === captured.rowKey);
    const column = grid.find((candidate) => candidate.key === captured.columnKey);
    if (!row || !column || projectedRows.find((candidate) => candidate.key === captured.rowKey)?.cells[captured.columnKey].readonly) return { applied: false, messages: ["The target cell is unavailable."] };
    const intent = buildGridOperationIntent({ row, column: { ...column, unit: () => captured.unit, value: () => captured.before }, model, sequence: 1, value, interaction: "cell" });
    const identity = crypto.randomUUID(); intent.operation_id = `op:table-cell-${identity}`; intent.change.change_id = `change:table-cell-${identity}`;
    return onApplyCellIntent(intent);
  }
  const familyLabel = family === "sections" ? "Section" : "Material";
  const common = { label: `${familyLabel} ${review ? "review drafts" : "fields"}`, rowHeader: familyLabel, testIdPrefix: `${familyLabel.toLowerCase()}-`, persistentEditor: true, rows: projectedRows, columns,
    generation, active, bounded, compact, density, filter, selectedKey: entityKey(selection), onDraftStateChange,
    onSelect: (key: EntityKey) => { const row = sourceRows.find((candidate) => entityKey(candidate) === key); if (row) onSelect({ type: row.type, id: row.id }); } };
  return review ? <EngineeringTable {...common} policy="review" resetEditsKey={resetEditsKey}
    onDraftChange={(captured, text) => {
      const row = sourceRows.find((candidate) => entityKey(candidate) === captured.rowKey); const column = grid.find((candidate) => candidate.key === captured.columnKey);
      if (row && column && !gridCellReadonly(column, row) && captured.generation === generation && captured.unit === column.unit(row)) updateCell(row, column, text);
    }} onKeepDraft={() => ({ retained: true, messages: ["Draft retained; model unchanged."] })} />
    : <EngineeringTable {...common} busy={operationBusy} onApply={apply} />;
}

function gridCellControl(
  row: GridRow,
  column: GridColumn,
  value: string,
  onChange: (row: GridRow, column: GridColumn, value: string) => void
): React.ReactNode {
  if (gridCellReadonly(column, row)) {
    return <span data-testid={`entity-grid-cell-${safeToken(row.id)}-${safeToken(column.key)}`}>
      {column.quantity ? <QuantityReadout quantity={{ value: numericGridValue(column.value(row)), unit: column.unit(row), dimension_id: column.dimension }} /> : value}
    </span>;
  }
  if (column.options) {
    return <select aria-label={`${row.id} ${column.label}`} data-testid={`entity-grid-input-${safeToken(row.id)}-${safeToken(column.key)}`} value={value} onChange={(event) => onChange(row, column, event.target.value)}>
      {!column.options.includes(value) ? <option value={value} disabled>{value} (existing)</option> : null}
      {column.options.map((option) => <option key={option} value={option}>{option}</option>)}
    </select>;
  }
  return <input aria-label={`${row.id} ${column.label}`} data-testid={`entity-grid-input-${safeToken(row.id)}-${safeToken(column.key)}`} onChange={(event) => onChange(row, column, event.target.value)} value={value} />;
}

function provenanceKeyword(value: unknown): string {
  return typeof value === "string" ? value : JSON.stringify(value ?? "");
}

function componentKeyword(value: unknown): string {
  return JSON.stringify(value ?? "");
}

function gridRows(model: PreviewModel, entityType: GridEntityType): GridRow[] {
  if (entityType === "materials") {
    return (model.materials ?? []).map((material) =>
      row(material.id, material.label, "material", material, [material.id, material.label, material.provenance])
    );
  }
  if (entityType === "sections") {
    return (model.sections ?? []).map((section) =>
      row(section.id, section.name, "section", section, [
        section.id,
        section.name,
        section.section_type,
        provenanceKeyword(section.provenance)
      ])
    );
  }
  if (entityType === "nodes") {
    return model.nodes.map((node) => row(node.id, node.label, "node", node, [node.id, node.label, node.provenance]));
  }
  if (entityType === "pipes") {
    return model.pipe_segments.map((pipe) =>
      row(pipe.id, pipe.label, "pipe", pipe, [
        pipe.id,
        pipe.label,
        pipe.from,
        pipe.to,
        pipe.material,
        pipe.section.mill_tolerance ? `${pipe.section.mill_tolerance.value} ${pipe.section.mill_tolerance.unit}` : "",
        pipe.provenance
      ])
    );
  }
  if (entityType === "supports") {
    return model.supports.map((support) =>
      row(support.id, support.label, "support", support, [
        support.id,
        support.label,
        support.node,
        support.family ?? "",
        support.hanger?.hanger_type ?? "",
        support.hanger?.source_reference ?? "",
        support.restraints.join(" "),
        support.provenance
      ])
    );
  }
  if (entityType === "components") {
    return model.components.map((component) =>
      row(component.id, component.label, "component", component, [
        component.id,
        component.label,
        component.kind,
        component.node,
        component.provenance,
        componentKeyword(component)
      ])
    );
  }
  if (entityType === "load_cases") {
    return model.load_cases.map((loadCase) =>
      row(loadCase.id, loadCase.label, "load", loadCase, [
        loadCase.id,
        loadCase.label,
        loadCase.kind,
        loadCase.status,
        loadCase.modulus_basis_ref ?? "",
        String(loadCase.modulus_basis_temperature?.value ?? ""),
        loadCase.equivalent_static?.wind?.direction ?? "",
        (loadCase.equivalent_static?.wind?.exposed_pipe_refs ?? []).join(" "),
        loadCase.provenance
      ])
    );
  }
  return (model.combinations ?? []).map((combination) =>
    row(combination.id, combination.label, "combination", combination, [
      combination.id,
      combination.label,
      combination.basis,
      combination.provenance
    ])
  );
}

function row(id: string, label: string, type: EntityRef["type"], raw: unknown, keywords: string[]): GridRow {
  return { id, label, type, raw, searchText: keywords.join(" ").toLowerCase() };
}

function gridColumns(model: PreviewModel, entityType: GridEntityType): GridColumn[] {
  const lengthUnit = model.project.units.length ?? "m";
  if (entityType === "nodes") {
    return [
      scalarGridColumn("label", "Label", "label", "Node", "node label only"),
      quantityGridColumn("x", "X", "position.x", "Node", "length", lengthUnit),
      quantityGridColumn("y", "Y", "position.y", "Node", "length", lengthUnit),
      quantityGridColumn("z", "Z", "position.z", "Node", "length", lengthUnit),
      scalarGridColumn("provenance", "Provenance", "provenance", "Node", "public/private source note")
    ];
  }
  if (entityType === "pipes") {
    return [
      scalarGridColumn("label", "Label", "label", "Element", "pipe segment label only"),
      readonlyGridColumn("section-ref", "Shared section", "section_ref", "Element"),
      readonlyGridColumn("from", "From", "from", "Element"),
      readonlyGridColumn("to", "To", "to", "Element"),
      quantityGridColumn(
        "mill-tolerance",
        "Mill tol.",
        "section.mill_tolerance.value",
        "Element",
        "length",
        lengthUnit
      ),
      scalarGridColumn("material", "Material", "material", "Element", "material reference"),
      scalarGridColumn("provenance", "Provenance", "provenance", "Element", "public/private source note")
    ];
  }
  if (entityType === "supports") {
    return [
      scalarGridColumn("label", "Label", "label", "Support", "support label only", "update_support"),
      scalarGridColumn("node", "Node", "node", "Support", "target node reference", "update_support"),
      readonlyGridColumn("family", "Family", "family", "Support"),
      readonlyGridColumn("hanger_type", "Hanger", "hanger.hanger_type", "Support"),
      scalarGridColumn(
        "restraints",
        "Restraints",
        "restraints",
        "Support",
        "restraint direction set",
        "update_support"
      ),
      scalarGridColumn(
        "provenance",
        "Provenance",
        "provenance",
        "Support",
        "public/private source note",
        "update_support"
      )
    ];
  }
  if (entityType === "materials") {
    return [
      scalarGridColumn("label", "Label", "label", "Material", "material label only"),
      quantityGridColumn("elastic", "Elastic", "elastic_modulus.value", "Material", "stress", ""),
      quantityGridColumn("shear", "Shear", "shear_modulus.value", "Material", "stress", ""),
      quantityGridColumn(
        "thermal",
        "Thermal",
        "thermal_expansion_coefficient.value",
        "Material",
        "thermal_expansion_coefficient",
        ""
      ),
      scalarGridColumn("provenance", "Provenance", "provenance", "Material", "public/private source note")
    ];
  }
  if (entityType === "sections") {
    return [
      scalarGridColumn("name", "Name", "name", "Section", "section label only"),
      { ...scalarGridColumn("type", "Type", "section_type", "Section", "section type"), options: ["pipe"] },
      quantityGridColumn(
        "outside",
        "Outside dia.",
        "properties.outside_diameter.value",
        "Section",
        "length",
        ""
      ),
      quantityGridColumn("wall", "Wall", "properties.wall_thickness.value", "Section", "length", ""),
      scalarGridColumn("provenance", "Provenance", "provenance", "Section", "public/private source note")
    ];
  }
  if (entityType === "components") {
    return [
      scalarGridColumn("label", "Label", "label", "Component", "component label only"),
      readonlyGridColumn("kind", "Kind", "kind", "Component"),
      scalarGridColumn("node", "Node", "node", "Component", "target node reference"),
      quantityGridColumn("bend-radius", "Bend radius", "geometry.bend_radius.value", "Component", "length", lengthUnit),
      quantityGridColumn(
        "bend-angle",
        "Bend angle",
        "geometry.bend_angle.value",
        "Component",
        "angle",
        model.project.units.angle ?? "rad"
      ),
      scalarGridColumn(
        "bend-plane",
        "Bend plane",
        "geometry.bend_plane_orientation",
        "Component",
        "user-entered bend plane orientation"
      ),
      scalarGridColumn(
        "bend-pipe",
        "Bend pipe",
        "geometry.bend_pipe_ref",
        "Component",
        "user-entered curved-bend span mapping (DEC-070)"
      ),
      scalarGridColumn(
        "sif-user",
        "SIF user",
        "modifiers.sif_user_value.value",
        "Component",
        "user-entered modifier value; no code table default"
      ),
      scalarGridColumn(
        "header-sif-user",
        "Header SIF",
        "modifiers.branch_header_sif_user_value.value",
        "Component",
        "user-entered branch header modifier; no code table default"
      ),
      scalarGridColumn(
        "branch-sif-user",
        "Branch SIF",
        "modifiers.branch_branch_sif_user_value.value",
        "Component",
        "user-entered branch-side modifier; no code table default"
      ),
      scalarGridColumn(
        "flexibility-user",
        "Flexibility user",
        "modifiers.flexibility_factor_user_value.value",
        "Component",
        "user-entered modifier value; no code table default"
      ),
      scalarGridColumn(
        "mapped-pipe",
        "Mapped pipe",
        "geometry.rigid_pipe_ref",
        "Component",
        "generic frame member mapping"
      ),
      quantityGridColumn(
        "rigid-length",
        "Rigid length",
        "geometry.rigid_body_length.value",
        "Component",
        "length",
        lengthUnit
      ),
      quantityGridColumn("end-a-size", "End A size", "geometry.end_a_size.value", "Component", "length", lengthUnit),
      quantityGridColumn("end-b-size", "End B size", "geometry.end_b_size.value", "Component", "length", lengthUnit),
      quantityGridColumn(
        "weight",
        "Weight",
        "geometry.weight.value",
        "Component",
        "force",
        model.project.units.force ?? "N"
      ),
      scalarGridColumn(
        "stiffness-scale",
        "Stiffness scale",
        "modifiers.stiffness_scaling_user_value.value",
        "Component",
        "user-entered rigid stiffness scaling; no catalog default"
      ),
      quantityGridColumn(
        "linear-stiffness",
        "Linear stiffness",
        "modifiers.linear_stiffness_user_value.value",
        "Component",
        "linear_stiffness",
        "N/m"
      ),
      quantityGridColumn(
        "rot-stiffness",
        "Rot. stiffness",
        "modifiers.rotational_stiffness_user_value.value",
        "Component",
        "rotational_stiffness",
        "N*m/rad"
      ),
      scalarGridColumn(
        "ej-pipe",
        "EJ pipe",
        "geometry.expansion_joint_pipe_ref",
        "Component",
        "expansion-joint frame member mapping"
      ),
      quantityGridColumn(
        "ej-effective-area",
        "EJ eff. area",
        "geometry.effective_area.value",
        "Component",
        "area",
        "m^2"
      ),
      quantityGridColumn(
        "ej-movement-limit",
        "EJ move limit",
        "geometry.movement_limit.value",
        "Component",
        "length",
        lengthUnit
      ),
      scalarGridColumn(
        "ej-hardware",
        "EJ hardware",
        "geometry.hardware_reference",
        "Component",
        "user-entered expansion-joint hardware reference"
      ),
      scalarGridColumn(
        "ej-manufacturer",
        "EJ mfr ref",
        "geometry.manufacturer_reference",
        "Component",
        "user-entered manufacturer provenance reference"
      ),
      quantityGridColumn(
        "ej-axial-stiffness",
        "EJ axial k",
        "modifiers.axial_stiffness_user_value.value",
        "Component",
        "linear_stiffness",
        "N/m"
      ),
      quantityGridColumn(
        "ej-lateral-stiffness",
        "EJ lateral k",
        "modifiers.lateral_stiffness_user_value.value",
        "Component",
        "linear_stiffness",
        "N/m"
      ),
      quantityGridColumn(
        "ej-angular-stiffness",
        "EJ angular k",
        "modifiers.angular_stiffness_user_value.value",
        "Component",
        "rotational_stiffness",
        "N*m/rad"
      ),
      quantityGridColumn(
        "ej-torsional-stiffness",
        "EJ torsion k",
        "modifiers.torsional_stiffness_user_value.value",
        "Component",
        "rotational_stiffness",
        "N*m/rad"
      ),
      scalarGridColumn("provenance", "Provenance", "provenance", "Component", "public/private source note")
    ];
  }
  if (entityType === "load_cases") {
    return [
      scalarGridColumn("label", "Label", "label", "Load", "load case label only", "update_load"),
      scalarGridColumn("kind", "Kind", "kind", "Load", "load case kind", "update_load"),
      scalarGridColumn("status", "Status", "status", "Load", "load case status", "update_load"),
      scalarGridColumn(
        "modulus-basis",
        "Modulus basis",
        "modulus_basis_ref",
        "Load",
        "user-assigned temperature-point basis id; exact selection",
        "update_load"
      ),
      quantityGridColumn(
        "modulus-basis-temperature",
        "Solve temperature",
        "modulus_basis_temperature.value",
        "Load",
        "temperature",
        "K",
        "update_load"
      ),
      quantityGridColumn(
        "seismic-gravity",
        "Seismic gravity",
        "equivalent_static.seismic.gravity_acceleration.value",
        "Load",
        "acceleration",
        "m/s^2",
        "update_load"
      ),
      scalarGridColumn(
        "seismic-gx",
        "g X",
        "equivalent_static.seismic.g_factor_x.value",
        "Load",
        "user-entered per-axis g-factor; no code coefficient or default",
        "update_load"
      ),
      scalarGridColumn(
        "seismic-gy",
        "g Y",
        "equivalent_static.seismic.g_factor_y.value",
        "Load",
        "user-entered per-axis g-factor; no code coefficient or default",
        "update_load"
      ),
      scalarGridColumn(
        "seismic-gz",
        "g Z",
        "equivalent_static.seismic.g_factor_z.value",
        "Load",
        "user-entered per-axis g-factor; no code coefficient or default",
        "update_load"
      ),
      quantityGridColumn(
        "wind-pressure",
        "Wind pressure",
        "equivalent_static.wind.pressure.value",
        "Load",
        "pressure",
        model.project.units.pressure ?? "Pa",
        "update_load"
      ),
      scalarGridColumn(
        "wind-shape",
        "Wind shape",
        "equivalent_static.wind.shape_factor.value",
        "Load",
        "user-entered wind shape parameter; no code coefficient or default",
        "update_load"
      ),
      scalarGridColumn(
        "wind-direction",
        "Wind dir",
        "equivalent_static.wind.direction",
        "Load",
        "user-entered global axis token (global_x | global_y | global_z)",
        "update_load"
      ),
      scalarGridColumn(
        "wind-exposed",
        "Wind spans",
        "equivalent_static.wind.exposed_pipe_refs",
        "Load",
        "user-marked exposed spans by pipe id; no span is marked by default",
        "update_load"
      ),
      scalarGridColumn("provenance", "Provenance", "provenance", "Load", "public/private source note", "update_load")
    ];
  }
  return [
    scalarGridColumn("label", "Label", "label", "Combination", "combination label only"),
    scalarGridColumn("basis", "Basis", "basis", "Combination", "mechanics or user rule basis"),
    scalarGridColumn("provenance", "Provenance", "provenance", "Combination", "public/private source note")
  ];
}

function scalarGridColumn(
  key: string,
  label: string,
  fieldPath: string,
  objectType: EditorOperationObjectType,
  sourceNote: string,
  changeKind: GridColumn["changeKind"] = "set_field"
): GridColumn {
  return {
    key,
    label,
    fieldPath,
    objectType,
    changeKind,
    dimension: "dimensionless",
    sourceNote,
    unit: () => "none",
    value: (gridRow) => stringValueAtPath(gridRow.raw, fieldPath)
  };
}

function quantityGridColumn(
  key: string,
  label: string,
  fieldPath: string,
  objectType: EditorOperationObjectType,
  dimension: string,
  fallbackUnit: string,
  changeKind: GridColumn["changeKind"] = "set_field"
): GridColumn {
  return {
    key,
    label,
    fieldPath,
    objectType,
    changeKind,
    dimension,
    sourceNote: "unit metadata required; entered unit captured explicitly",
    unitEditable: false,
    quantity: true,
    unit: (gridRow) => objectType === "Material" || objectType === "Section" ? actualLibraryQuantity(gridRow, fieldPath).unit : quantityUnitValue(gridRow.raw, fieldPath, fallbackUnit),
    value: (gridRow) => stringValueAtPath(gridRow.raw, fieldPath)
  };
}

function readonlyGridColumn(
  key: string,
  label: string,
  fieldPath: string,
  objectType: EditorOperationObjectType
): GridColumn {
  return {
    ...scalarGridColumn(key, label, fieldPath, objectType, "reference field"),
    readonly: true
  };
}

function gridCellReadonly(column: GridColumn, row: GridRow): boolean {
  // Structured provenance must never be flattened into a user-entered string.
  return Boolean(column.readonly || (column.fieldPath === "provenance" && ((row.type === "section" && typeof (row.raw as { provenance?: unknown }).provenance !== "string") || ((row.type === "node" || row.type === "material") && typeof (row.raw as { provenance?: unknown }).provenance !== "string"))));
}

function changedGridCells({
  columns,
  drafts,
  rows,
  projectSessionGeneration
}: {
  columns: GridColumn[];
  drafts: Record<string, string>;
  rows: GridRow[];
  projectSessionGeneration: number;
}): Array<{ row: GridRow; column: GridColumn; value: string }> {
  return rows.flatMap((row) =>
    columns.flatMap((column) => {
      if (gridCellReadonly(column, row)) return [];
      const key = draftKey(row, column, projectSessionGeneration);
      const value = drafts[key];
      if (value === undefined || value === column.value(row) || (column.options && !column.options.includes(value))) return [];
      return [{ row, column, value }];
    })
  );
}

function gridEntityTypeFromSelection(selection: EntityRef): GridEntityType {
  if (selection.type === "material") return "materials";
  if (selection.type === "section") return "sections";
  if (selection.type === "node") return "nodes";
  if (selection.type === "pipe") return "pipes";
  if (selection.type === "support") return "supports";
  if (selection.type === "component") return "components";
  if (selection.type === "load") return "load_cases";
  if (selection.type === "combination") return "combinations";
  return "nodes";
}

function draftKey(row: GridRow, column: GridColumn, projectSessionGeneration: number): string {
  return JSON.stringify([projectSessionGeneration, row.type, row.id, column.fieldPath]);
}

function stringValueAtPath(source: unknown, fieldPath: string): string {
  const value = fieldPath.split(".").reduce<unknown>((current, part) => {
    if (current === null || current === undefined) return undefined;
    if (/^\d+$/.test(part) && Array.isArray(current)) return current[Number(part)];
    if (typeof current === "object") return (current as Record<string, unknown>)[part];
    return undefined;
  }, source);
  if (Array.isArray(value)) return value.join(", ");
  if (typeof value === "number" || typeof value === "boolean") return String(value);
  if (typeof value === "string") return value;
  if (value && typeof value === "object") return JSON.stringify(value);
  return "";
}

function quantityUnitValue(source: unknown, valuePath: string, fallbackUnit: string): string {
  const unitPath = valuePath.replace(/\.value$/, ".unit");
  if (unitPath === valuePath) return fallbackUnit;
  return stringValueAtPath(source, unitPath) || fallbackUnit;
}

function safeToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9:_-]+/g, "-").replace(/^-+|-+$/g, "") || "entity";
}

function numericGridValue(value: string): string | number {
  return value.trim() !== "" && Number.isFinite(Number(value)) ? Number(value) : value;
}
