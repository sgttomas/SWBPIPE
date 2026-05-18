import { Box, CircleDot, GitBranch, SquareStack } from "lucide-react";
import type React from "react";
import type { EntityRef, PreviewModel } from "../../types";

type Props = {
  model: PreviewModel;
  selection: EntityRef;
  onSelect: (selection: EntityRef) => void;
};

export function ModelTree({ model, selection, onSelect }: Props) {
  return (
    <div className="panel model-tree" aria-label="Model tree">
      <div className="panel-title">Model</div>
      <TreeButton
        active={selection.id === model.project.id}
        icon={<SquareStack size={16} />}
        label={model.project.name}
        onClick={() => onSelect({ type: "project", id: model.project.id })}
      />
      <TreeGroup title="Nodes">
        {model.nodes.map((node) => (
          <TreeButton
            key={node.id}
            active={selection.id === node.id}
            icon={<CircleDot size={14} />}
            label={node.label}
            detail={node.id}
            onClick={() => onSelect({ type: "node", id: node.id })}
          />
        ))}
      </TreeGroup>
      <TreeGroup title="Pipe Segments">
        {model.pipe_segments.map((pipe) => (
          <TreeButton
            key={pipe.id}
            active={selection.id === pipe.id}
            icon={<GitBranch size={14} />}
            label={pipe.label}
            detail={pipe.id}
            onClick={() => onSelect({ type: "pipe", id: pipe.id })}
          />
        ))}
      </TreeGroup>
      <TreeGroup title="Supports">
        {model.supports.map((support) => (
          <TreeButton
            key={support.id}
            active={selection.id === support.id}
            icon={<Box size={14} />}
            label={support.label}
            detail={support.id}
            onClick={() => onSelect({ type: "support", id: support.id })}
          />
        ))}
      </TreeGroup>
    </div>
  );
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
  icon,
  label,
  detail,
  onClick
}: {
  active: boolean;
  icon: React.ReactNode;
  label: string;
  detail?: string;
  onClick: () => void;
}) {
  return (
    <button className={`tree-row ${active ? "active" : ""}`} onClick={onClick} type="button">
      {icon}
      <span>
        <strong>{label}</strong>
        {detail ? <small>{detail}</small> : null}
      </span>
    </button>
  );
}
