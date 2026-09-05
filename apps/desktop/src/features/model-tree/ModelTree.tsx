import { QuantityReadout } from "../display-units";
import { Anchor, Box, CircleDot, Circle, GitBranch, ListTree, Search, SquareStack, Table2, Waypoints, X, Zap } from "lucide-react";
import type React from "react";
import { useEffect, useMemo, useState } from "react";
import type { EditorOperationIntent, EditorOperationObjectType, EntityRef, PreviewModel } from "../../types";

type Props = {
  model: PreviewModel;
  selection: EntityRef;
  onSelect: (selection: EntityRef) => void;
  onQueueIntent?: (intent: EditorOperationIntent) => void;
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

export function ModelTree({ model, selection, onSelect, onQueueIntent }: Props) {
  const [filterText, setFilterText] = useState("");
  const [layoutMode, setLayoutMode] = useState<LayoutMode>("tree");
  const [gridEntityType, setGridEntityType] = useState<GridEntityType>(() => gridEntityTypeFromSelection(selection));
  const tree = useMemo(() => buildTree(model), [model]);
  const filteredTree = useMemo(() => filterTree(tree, filterText), [tree, filterText]);

  useEffect(() => {
    if (layoutMode === "grid") {
      setGridEntityType(gridEntityTypeFromSelection(selection));
    }
  }, [layoutMode, selection.id, selection.type]);

  return (
    <div className="panel model-tree" aria-label="Model tree">
      <div className="panel-title">Model</div>
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
          onClick={() => setLayoutMode("grid")}
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
        onFilterChange={setFilterText}
      />
      {layoutMode === "grid" ? (
        <EntityGrid
          entityType={gridEntityType}
          filterText={filterText}
          model={model}
          onEntityTypeChange={setGridEntityType}
          onQueueIntent={onQueueIntent}
          onSelect={onSelect}
          selection={selection}
        />
      ) : (
        <>
          {filteredTree.project ? (
            <TreeButton
              active={selection.id === model.project.id}
              item={filteredTree.project}
              onClick={() => onSelect({ type: "project", id: model.project.id })}
            />
          ) : null}
          {filteredTree.groups.length ? (
            filteredTree.groups.map((group) => (
              <TreeGroup key={group.title} title={group.title}>
                {group.items.map((item) => (
                  <TreeButton
                    key={item.id}
                    active={selection.id === item.id}
                    item={item}
                    onClick={() => onSelect({ type: item.type, id: item.id })}
                  />
                ))}
              </TreeGroup>
            ))
          ) : !filteredTree.project ? (
            <p className="muted" data-testid="model-tree-filter-empty">
              No model entities match this filter.
            </p>
          ) : null}
        </>
      )}
    </div>
  );
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
  onFilterChange: (value: string) => void;
}) {
  return (
    <section className="model-tree-controls" aria-label="Model tree filtering" data-testid="model-tree-controls">
      <label>
        <Search size={14} aria-hidden="true" />
        <span>Filter model</span>
        <input
          aria-label="Filter model tree"
          data-testid="model-tree-filter-input"
          onChange={(event) => onFilterChange(event.target.value)}
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
        onClick={() => onFilterChange("")}
        type="button"
      >
        <X size={14} aria-hidden="true" />
      </button>
    </section>
  );
}

function buildTree(model: PreviewModel): TreeModel {
  const project: TreeItem = {
    id: model.project.id,
    type: "project",
    label: model.project.name,
    icon: <SquareStack size={16} aria-hidden="true" />,
    keywords: [model.project.id, model.project.name, model.project.description, "project", "model"]
  };
  const groups: TreeGroupModel[] = [
    {
      title: "Materials",
      items:
        model.materials?.map((material) => ({
          id: material.id,
          type: "material" as const,
          label: material.label,
          detail: material.id,
          icon: <Box size={14} aria-hidden="true" />,
          keywords: [material.id, material.label, material.provenance, "material", "materials"]
        })) ?? []
    },
    {
      title: "Sections",
      items:
        model.sections?.map((section) => ({
          id: section.id,
          type: "section" as const,
          label: section.name,
          detail: section.id,
          icon: <Circle size={14} aria-hidden="true" />,
          keywords: [
            section.id,
            section.name,
            section.section_type,
            provenanceKeyword(section.provenance),
            "section",
            "sections",
            "pipe section"
          ]
        })) ?? []
    },
    {
      title: "Nodes",
      items: model.nodes.map((node) => ({
        id: node.id,
        type: "node" as const,
        label: node.label,
        detail: node.id,
        icon: <CircleDot size={14} aria-hidden="true" />,
        keywords: [node.id, node.label, node.provenance, "node", "nodes"]
      }))
    },
    {
      title: "Pipes",
      items: model.pipe_segments.map((pipe) => ({
        id: pipe.id,
        type: "pipe" as const,
        label: pipe.label,
        detail: pipe.id,
        icon: <GitBranch size={14} aria-hidden="true" />,
        keywords: [pipe.id, pipe.label, pipe.from, pipe.to, pipe.material, pipe.provenance, "pipe", "pipe segment"]
      }))
    },
    {
      title: "Supports",
      items: model.supports.map((support) => ({
        id: support.id,
        type: "support" as const,
        label: support.label,
        detail: support.id,
        icon: <Anchor size={14} aria-hidden="true" />,
        keywords: [
          support.id,
          support.label,
          support.node,
          support.family ?? "",
          support.hanger?.hanger_type ?? "",
          support.hanger?.source_reference ?? "",
          support.hanger?.mechanics_consumption ?? "",
          support.restraints.join(" "),
          support.provenance,
          "support",
          "supports"
        ]
      }))
    },
    {
      title: "Components",
      items: model.components.map((component) => ({
        id: component.id,
        type: "component" as const,
        label: component.label,
        detail: component.id,
        icon: <Waypoints size={14} aria-hidden="true" />,
        keywords: [
          component.id,
          component.label,
          component.kind,
          component.node,
          component.provenance,
          componentKeyword(component),
          "component",
          "components"
        ]
      }))
    },
    {
      title: "Load Cases",
      items: model.load_cases.map((loadCase) => ({
        id: loadCase.id,
        type: "load" as const,
        label: loadCase.label,
        detail: loadCase.id,
        icon: <Zap size={14} aria-hidden="true" />,
        keywords: [
          loadCase.id,
          loadCase.label,
          loadCase.kind,
          loadCase.status,
          loadCase.provenance,
          "load",
          "load case"
        ]
      }))
    },
    {
      title: "Combinations",
      items:
        model.combinations?.map((combination) => ({
          id: combination.id,
          type: "combination" as const,
          label: combination.label,
          detail: combination.id,
          icon: <GitBranch size={14} aria-hidden="true" />,
          keywords: [
            combination.id,
            combination.label,
            combination.basis,
            combination.provenance,
            "combination",
            "combinations"
          ]
        })) ?? []
    }
  ].filter((group) => group.items.length > 0);

  return {
    project,
    groups,
    count: 1 + groups.reduce((count, group) => count + group.items.length, 0)
  };
}

function provenanceKeyword(provenance: string | Record<string, unknown>): string {
  return typeof provenance === "string" ? provenance : JSON.stringify(provenance);
}

function componentKeyword(component: PreviewModel["components"][number]): string {
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

function TreeGroup({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <section className="tree-group">
      <h3>{title}</h3>
      {children}
    </section>
  );
}

function TreeButton({ active, item, onClick }: { active: boolean; item: TreeItem; onClick: () => void }) {
  return (
    <button
      className={`tree-row ${active ? "active" : ""}`}
      data-testid={`tree-row-${item.id}`}
      aria-pressed={active}
      title={`${item.label}${item.detail ? ` (${item.detail})` : ""}`}
      onClick={onClick}
      type="button"
    >
      {item.icon}
      <span>
        <strong>{item.label}</strong>
        {item.detail ? <small>{item.detail}</small> : null}
      </span>
    </button>
  );
}

type GridColumn = {
  key: string;
  label: string;
  fieldPath: string;
  objectType: EditorOperationObjectType;
  changeKind: EditorOperationIntent["change"]["change_kind"];
  dimension: string;
  sourceNote: string;
  unit: (row: GridRow) => string;
  value: (row: GridRow) => string;
  unitEditable?: boolean;
  quantity?: boolean;
  readonly?: boolean;
  options?: readonly string[];
};

type GridRow = {
  id: string;
  label: string;
  type: EntityRef["type"];
  searchText: string;
  raw: unknown;
};

function EntityGrid({
  entityType,
  filterText,
  model,
  onEntityTypeChange,
  onQueueIntent,
  onSelect,
  selection
}: {
  entityType: GridEntityType;
  filterText: string;
  model: PreviewModel;
  onEntityTypeChange: (entityType: GridEntityType) => void;
  onQueueIntent?: (intent: EditorOperationIntent) => void;
  onSelect: (selection: EntityRef) => void;
  selection: EntityRef;
}) {
  const [drafts, setDrafts] = useState<Record<string, string>>({});
  const [queuedMessage, setQueuedMessage] = useState("");
  const rows = useMemo(() => gridRows(model, entityType), [model, entityType]);
  const columns = useMemo(() => gridColumns(model, entityType), [model, entityType]);
  const query = filterText.trim().toLowerCase();
  const visibleRows = query ? rows.filter((row) => row.searchText.includes(query)) : rows;
  const changedCells = changedGridCells({ columns, drafts, rows: visibleRows });
  const queueDisabled = !onQueueIntent || changedCells.length === 0;

  useEffect(() => {
    setDrafts({});
    setQueuedMessage("");
  }, [entityType, model.project.id]);

  function updateCell(row: GridRow, column: GridColumn, value: string) {
    setDrafts((current) => {
      const key = draftKey(row, column);
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
    setQueuedMessage(
      `Queued ${changedCells.length} review intent${changedCells.length === 1 ? "" : "s"} from Grid mode.`
    );
    setDrafts({});
  }

  return (
    <section className="entity-grid" aria-label="Bulk entity grid" data-testid="entity-grid">
      <div className="entity-grid-tabs" aria-label="Grid entity type">
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
      <div className="entity-grid-summary">
        <span data-testid="entity-grid-summary">
          {visibleRows.length} of {rows.length}{" "}
          {GRID_ENTITY_TYPES.find((item) => item.id === entityType)?.label ?? "rows"}
        </span>
        <span data-testid="entity-grid-change-count">{changedCells.length} changed cells</span>
      </div>
      <div className="entity-grid-scroll" role="region" aria-label="Editable model entity table">
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
              <tr key={row.id} className={selection.id === row.id ? "active" : ""}>
                <th scope="row">
                  <button
                    aria-pressed={selection.id === row.id}
                    data-testid={`entity-grid-row-${row.id}`}
                    onClick={() => onSelect({ type: row.type, id: row.id })}
                    type="button"
                  >
                    {row.id}
                  </button>
                </th>
                {columns.map((column) => {
                  const key = draftKey(row, column);
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
        {visibleRows.length === 0 ? (
          <p className="muted" data-testid="entity-grid-empty">
            No grid rows match this filter.
          </p>
        ) : null}
      </div>
      <div className="entity-grid-actions">
        <button
          data-testid="queue-entity-grid-intents"
          disabled={queueDisabled}
          onClick={handleQueueChangedCells}
          type="button"
        >
          <Table2 size={14} aria-hidden="true" />
          Queue changed cells
        </button>
        <button
          data-testid="clear-entity-grid-drafts"
          disabled={changedCells.length === 0}
          onClick={() => setDrafts({})}
          type="button"
        >
          <X size={14} aria-hidden="true" />
          Clear grid edits
        </button>
      </div>
      <p className="muted entity-grid-boundary" data-testid="entity-grid-boundary">
        Grid mode fans each changed cell into a structured review intent; storage remains local, and acceptance stays
        with the responsible engineer.
      </p>
      {queuedMessage ? (
        <p className="entity-grid-queued" data-testid="entity-grid-queued-message">
          {queuedMessage}
        </p>
      ) : null}
    </section>
  );
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
      quantityGridColumn("elastic", "Elastic", "elastic_modulus.value", "Material", "stress", "Pa"),
      quantityGridColumn("shear", "Shear", "shear_modulus.value", "Material", "stress", "Pa"),
      quantityGridColumn(
        "thermal",
        "Thermal",
        "thermal_expansion_coefficient.value",
        "Material",
        "thermal_expansion_coefficient",
        "1/degC"
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
        lengthUnit
      ),
      quantityGridColumn("wall", "Wall", "properties.wall_thickness.value", "Section", "length", lengthUnit),
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
    unit: (gridRow) => quantityUnitValue(gridRow.raw, fieldPath, fallbackUnit),
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
  return Boolean(column.readonly || (row.type === "section" && column.fieldPath === "provenance" && typeof (row.raw as { provenance?: unknown }).provenance === "object"));
}

function changedGridCells({
  columns,
  drafts,
  rows
}: {
  columns: GridColumn[];
  drafts: Record<string, string>;
  rows: GridRow[];
}): Array<{ row: GridRow; column: GridColumn; value: string }> {
  return rows.flatMap((row) =>
    columns.flatMap((column) => {
      if (gridCellReadonly(column, row)) return [];
      const key = draftKey(row, column);
      const value = drafts[key];
      if (value === undefined || value === column.value(row) || (column.options && !column.options.includes(value))) return [];
      return [{ row, column, value }];
    })
  );
}

function buildGridOperationIntent({
  column,
  model,
  row,
  sequence,
  value
}: {
  column: GridColumn;
  model: PreviewModel;
  row: GridRow;
  sequence: number;
  value: string;
}): EditorOperationIntent {
  const operationToken = `${safeToken(row.id)}-${safeToken(column.fieldPath)}-${sequence.toString().padStart(2, "0")}`;
  const unit = column.unit(row);
  const after =
    column.dimension === "dimensionless"
      ? value.trim() || "TBD"
      : JSON.stringify({ value: parseQuantityPayloadValue(value), unit });

  return {
    operation_id: `op:grid-intent-${operationToken}`,
    operation_kind: "modify",
    operation_status: "proposed",
    author_type: "user",
    source: {
      source_ref: `grid:${model.project.id}:${row.id}`,
      source_channel: "local_desktop_preview",
      source_role: "gui_editor"
    },
    target: {
      object_type: column.objectType,
      ref: row.id
    },
    change: {
      change_id: `change:grid:${operationToken}`,
      change_kind: column.changeKind,
      field_label: column.label,
      field_path: column.fieldPath,
      before: column.value(row),
      after,
      unit,
      dimension: column.dimension,
      source_note: `layout_grid_bulk_tabular; ${column.sourceNote}`
    },
    validation: {
      schema_validation: "not_run",
      constraint_validation: "not_run",
      unit_validation:
        column.dimension === "dimensionless" ? "not_required_dimensionless" : "model_metadata_unit_dimension_declared",
      diff_preview_status: "not_generated",
      application_status: "not_applied"
    },
    audit_boundary: {
      mutation_route: "structured_operations_only",
      direct_model_mutation_allowed: false,
      requires_user_acceptance: true,
      mutates_accepted_model_state: false
    },
    professional_boundary: {
      human_review_required: true,
      software_makes_compliance_claim: false,
      software_makes_certification_claim: false,
      software_makes_sealing_claim: false,
      software_makes_approval_claim: false,
      software_makes_authentication_claim: false
    },
    rationale: "layout_grid_bulk_tabular_review_change"
  };
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

function draftKey(row: GridRow, column: GridColumn): string {
  return `${row.id}::${column.fieldPath}`;
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

function parseQuantityPayloadValue(raw: string): number | string {
  const trimmed = raw.trim();
  if (!trimmed) return "TBD";
  const parsed = Number(trimmed);
  return Number.isFinite(parsed) ? parsed : trimmed;
}

function safeToken(value: string): string {
  return value.replace(/[^a-zA-Z0-9:_-]+/g, "-").replace(/^-+|-+$/g, "") || "entity";
}

function numericGridValue(value: string): string | number {
  return value.trim() !== "" && Number.isFinite(Number(value)) ? Number(value) : value;
}
