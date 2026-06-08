import { Box, CircleDot, GitBranch, Search, SquareStack, X, Zap } from "lucide-react";
import type React from "react";
import { useMemo, useState } from "react";
import type { EntityRef, PreviewModel } from "../../types";

type Props = {
  model: PreviewModel;
  selection: EntityRef;
  onSelect: (selection: EntityRef) => void;
};

export function ModelTree({ model, selection, onSelect }: Props) {
  const [filterText, setFilterText] = useState("");
  const tree = useMemo(() => buildTree(model), [model]);
  const filteredTree = useMemo(() => filterTree(tree, filterText), [tree, filterText]);

  return (
    <div className="panel model-tree" aria-label="Model tree">
      <div className="panel-title">Model</div>
      <TreeControls
        filterText={filterText}
        filteredCount={filteredTree.count}
        totalCount={tree.count}
        onFilterChange={setFilterText}
      />
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
      title: "Pipe Segments",
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
        icon: <Box size={14} aria-hidden="true" />,
        keywords: [support.id, support.label, support.node, support.restraints.join(" "), support.provenance, "support", "supports"]
      }))
    },
    {
      title: "Components",
      items: model.components.map((component) => ({
        id: component.id,
        type: "component" as const,
        label: component.label,
        detail: component.id,
        icon: <Box size={14} aria-hidden="true" />,
        keywords: [component.id, component.label, component.kind, component.node, component.provenance, "component", "components"]
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
        keywords: [loadCase.id, loadCase.label, loadCase.kind, loadCase.status, loadCase.provenance, "load", "load case"]
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
          keywords: [combination.id, combination.label, combination.basis, combination.provenance, "combination", "combinations"]
        })) ?? []
    }
  ].filter((group) => group.items.length > 0);

  return {
    project,
    groups,
    count: 1 + groups.reduce((count, group) => count + group.items.length, 0)
  };
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

function TreeButton({
  active,
  item,
  onClick
}: {
  active: boolean;
  item: TreeItem;
  onClick: () => void;
}) {
  return (
    <button className={`tree-row ${active ? "active" : ""}`} data-testid={`tree-row-${item.id}`} onClick={onClick} type="button">
      {item.icon}
      <span>
        <strong>{item.label}</strong>
        {item.detail ? <small>{item.detail}</small> : null}
      </span>
    </button>
  );
}
