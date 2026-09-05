import type { EntityRef } from "../../types";
export type ToolkitGroup = "Build" | "Supports" | "Properties" | "Loads" | "Edit" | "Select and View" | "Review";
export type ToolkitRoute = {
  surface: "viewport" | "inspector" | "tree" | "loads" | "libraries" | "operations";
  focusTestId: string;
  elementId?: string;
  tool?: "node" | "pipe" | "component";
};
export type ToolkitCapability = {
  id: string;
  label: string;
  group: ToolkitGroup;
  vocabularyRows: readonly number[];
  status: "supported" | "partial" | "unavailable" | "gated";
  description: string;
  route?: ToolkitRoute;
  history?: "undo" | "redo";
  requires?: "load" | "removable" | "support-context" | "material" | "wind" | "pipe";
};
// Accepted SCA-009 vocabulary. Availability describes executable UI routes,
// not schema tokens or eventual deliverable acceptance. Routes never mutate.
export const toolkitCapabilities: readonly ToolkitCapability[] = [
  {
    id: "build.node",
    label: "Node",
    group: "Build",
    vocabularyRows: [1],
    status: "supported",
    description: "Enter coordinates or place a draft in the viewport.",
    route: {
      surface: "viewport",
      tool: "node",
      focusTestId: "viewport-create-node-id"
    }
  },
  {
    id: "build.pipe",
    label: "Pipe",
    group: "Build",
    vocabularyRows: [1],
    status: "partial",
    description: "Connect explicit existing endpoints, with optional continuation. Automated node creation during routing remains planned.",
    route: {
      surface: "viewport",
      tool: "pipe",
      focusTestId: "viewport-create-pipe-id"
    }
  },
  {
    id: "build.component",
    label: "Components and expansion joints",
    group: "Build",
    vocabularyRows: [14, 15],
    status: "supported",
    description: "Choose bend, tee, reducer, valve, flange or expansion joint, then enter geometry and provenance.",
    route: {
      surface: "inspector",
      focusTestId: "create-component-kind"
    }
  },
  {
    id: "build.split",
    label: "Split pipe / insert node",
    group: "Build",
    vocabularyRows: [19],
    status: "supported",
    description: "Split one explicit source pipe with a new node and two new pipe identities.",
    route: { surface: "operations", focusTestId: "", elementId: "geometry-tools" }
  },
  {
    id: "supports.restraint",
    label: "Restraint / anchor",
    group: "Supports",
    vocabularyRows: [2, 16],
    status: "supported",
    description: "Choose a named family and enter restrained degrees of freedom and provenance.",
    requires: "support-context",
    route: {
      surface: "inspector",
      focusTestId: "",
      elementId: "rich-support-form"
    }
  },
  {
    id: "supports.hanger",
    label: "Spring hanger",
    group: "Supports",
    vocabularyRows: [12],
    status: "supported",
    description: "Include hanger data, then enter the hanger type, loads, stiffness, travel and source.",
    requires: "support-context",
    route: {
      surface: "inspector",
      focusTestId: "",
      elementId: "rich-support-form"
    }
  },
  {
    id: "supports.nonlinear",
    label: "Nonlinear support",
    group: "Supports",
    vocabularyRows: [13],
    status: "supported",
    description: "Include nonlinear behavior and enter its direction, gap, friction and reaction inputs.",
    requires: "support-context",
    route: {
      surface: "inspector",
      focusTestId: "",
      elementId: "rich-support-form"
    }
  },
  {
    id: "supports.boundary",
    label: "Equipment / nozzle boundary",
    group: "Supports",
    vocabularyRows: [21],
    status: "supported",
    description: "Enter equipment or nozzle references and explicit rigid, free or spring choices for every degree of freedom.",
    route: { surface: "operations", focusTestId: "", elementId: "boundary-authoring" }
  },
  {
    id: "properties.material",
    label: "Create material",
    group: "Properties",
    vocabularyRows: [3],
    status: "supported",
    description: "Create explicit elastic, shear and thermal properties with provenance.",
    route: {
      surface: "inspector",
      focusTestId: "create-material-id"
    }
  },
  {
    id: "properties.temperature",
    label: "Material temperature table",
    group: "Properties",
    vocabularyRows: [3],
    status: "supported",
    description: "Add or edit temperature-dependent material properties with stable point identities.",
    requires: "material",
    route: {
      surface: "inspector",
      focusTestId: "",
      elementId: "temperature-table"
    }
  },
  {
    id: "properties.section",
    label: "Create section",
    group: "Properties",
    vocabularyRows: [17],
    status: "supported",
    description: "Create a section; use the model tree grid to edit the shared record.",
    route: {
      surface: "inspector",
      focusTestId: "create-section-id"
    }
  },
  {
    id: "properties.assign-section",
    label: "Assign / detach shared section",
    group: "Properties",
    vocabularyRows: [17],
    status: "supported",
    description: "Use a shared section or retain local pipe dimensions by detaching.",
    requires: "pipe",
    route: {
      surface: "inspector",
      focusTestId: "",
      elementId: "section-assignment"
    }
  },
  {
    id: "properties.libraries",
    label: "Imported libraries",
    group: "Properties",
    vocabularyRows: [3, 15],
    status: "supported",
    description: "Open user-controlled material, section and component libraries.",
    route: {
      surface: "libraries",
      focusTestId: "workspace-section-libraries"
    }
  },
  {
    id: "properties.hanger-library",
    label: "Imported hanger selection",
    group: "Properties",
    vocabularyRows: [23],
    status: "partial",
    description: "Select a user-imported hanger record in the native desktop app. Browser library import is unavailable.",
    route: { surface: "operations", focusTestId: "", elementId: "hanger-selection" }
  },
  {
    id: "loads.cases",
    label: "Load cases: temperature and pressure",
    group: "Loads",
    vocabularyRows: [4],
    status: "supported",
    description: "Create a case, then add explicit thermal or pressure loads.",
    route: {
      surface: "loads",
      focusTestId: "load-manager-create-load-id"
    }
  },
  {
    id: "loads.primitive",
    label: "Force, moment and displacement",
    group: "Loads",
    vocabularyRows: [5],
    status: "supported",
    description: "Choose a primitive category, case, target and units.",
    route: {
      surface: "loads",
      focusTestId: "load-manager-create-primitive-category"
    }
  },
  {
    id: "loads.wind",
    label: "Wind case fields",
    group: "Loads",
    vocabularyRows: [6],
    status: "supported",
    description: "Enter selected case pressure, shape and direction. Apply one of these fields before editing detailed exposure.",
    requires: "load",
    route: {
      surface: "inspector",
      focusTestId: "editor-intent-field"
    }
  },
  {
    id: "loads.wind-exposure",
    label: "Wind exposure spans",
    group: "Loads",
    vocabularyRows: [6],
    status: "supported",
    description: "Mark whole pipes or explicit partial exposure spans.",
    requires: "wind",
    route: {
      surface: "inspector",
      focusTestId: "",
      elementId: "wind-exposure-form"
    }
  },
  {
    id: "loads.seismic",
    label: "Seismic static-g fields",
    group: "Loads",
    vocabularyRows: [7],
    status: "supported",
    description: "Choose gravity acceleration or an axis factor in the selected case field editor.",
    requires: "load",
    route: {
      surface: "inspector",
      focusTestId: "editor-intent-field"
    }
  },
  {
    id: "loads.combinations",
    label: "Combinations",
    group: "Loads",
    vocabularyRows: [8],
    status: "supported",
    description: "Create mechanics, subtraction or range-envelope combinations.",
    route: {
      surface: "loads",
      focusTestId: "load-manager-create-combination-id"
    }
  },
  {
    id: "loads.self-weight",
    label: "Generate self-weight",
    group: "Loads",
    vocabularyRows: [22],
    status: "supported",
    description: "Generate a reviewed load-case batch from selected pipes, entered mass inputs and explicit gravity.",
    route: { surface: "operations", focusTestId: "", elementId: "self-weight-plan" }
  },
  {
    id: "edit.remove",
    label: "Remove selected entity",
    group: "Edit",
    vocabularyRows: [1, 2, 18],
    status: "partial",
    description: "Review removal of a node, pipe, support, material, section or component. References can block application.",
    requires: "removable",
    route: {
      surface: "inspector",
      focusTestId: ""
    }
  },
  ...(["copy", "rotate", "mirror"] as const).map((action): ToolkitCapability => ({
    id: `edit.${action}`,
    label: action[0].toUpperCase() + action.slice(1),
    group: "Edit",
    vocabularyRows: [20],
    status: "supported",
    description: "Select explicit pipe sources and enter the transformation, units and identities. Attached entities can block application.",
    route: { surface: "operations", focusTestId: "", elementId: "geometry-tools" }
  })),
  {
    id: "edit.undo",
    label: "Undo",
    group: "Edit",
    vocabularyRows: [10],
    status: "supported",
    description: "Restore the previous session checkpoint (up to 25 edits).",
    history: "undo"
  },
  {
    id: "edit.redo",
    label: "Redo",
    group: "Edit",
    vocabularyRows: [10],
    status: "supported",
    description: "Restore the next session checkpoint.",
    history: "redo"
  },
  {
    id: "view.select",
    label: "Select / search model",
    group: "Select and View",
    vocabularyRows: [9],
    status: "supported",
    description: "Find an entity by name, ID, type or group.",
    route: {
      surface: "tree",
      focusTestId: "model-tree-filter-input"
    }
  },
  {
    id: "view.units",
    label: "Display units",
    group: "Select and View",
    vocabularyRows: [24],
    status: "supported",
    description: "Select Entered, SI or US readout units. Editing stays in entered units; unavailable conversions are explained.",
    route: { surface: "viewport", focusTestId: "", elementId: "display-unit-preference" }
  },
  {
    id: "review.pending",
    label: "Pending changes and details",
    group: "Review",
    vocabularyRows: [11],
    status: "supported",
    description: "Validate, review the diff and explicitly apply user-authored changes.",
    route: {
      surface: "operations",
      focusTestId: "workspace-section-operations"
    }
  },
  {
    id: "review.agent",
    label: "Agent validate / preview / apply",
    group: "Review",
    vocabularyRows: [11],
    status: "partial",
    description: "Import an offline proposal, validate and preview, then explicitly apply. Live provider integration remains held.",
    route: { surface: "operations", focusTestId: "", elementId: "offline-proposal-intake" }
  }
];
export const toolkitRoadmap = ["Node renumbering", "Snubbers", "Cold spring"] as const;
export const toolkitGroups: readonly ToolkitGroup[] = ["Build", "Supports", "Properties", "Loads", "Edit", "Select and View", "Review"];
export type ToolkitContext = {
  selection: EntityRef;
  canUndo: boolean;
  canRedo: boolean;
  busy: boolean;
  windConfigured?: boolean;
};
export function capabilityAvailability(capability: ToolkitCapability, context: ToolkitContext): {
  enabled: boolean;
  reason: string;
} {
  if (capability.status === "unavailable" || capability.status === "gated")
    return {
      enabled: false,
    reason: capability.description
    };
  if (capability.history && context.busy)
    return {
      enabled: false,
    reason: "Wait for the current operation to finish."
    };
  if (capability.history === "undo" && !context.canUndo)
    return {
      enabled: false,
    reason: "No earlier session checkpoint."
    };
  if (capability.history === "redo" && !context.canRedo)
    return {
      enabled: false,
    reason: "No later session checkpoint."
    };
  if (capability.requires === "support-context" && !["project", "node", "support"].includes(context.selection.type))
    return {
      enabled: false,
    reason: "Select a node or support, or select the project to create a support."
    };
  if (capability.requires === "material" && context.selection.type !== "material")
    return {
      enabled: false,
    reason: "Select a material in the model tree first."
    };
  if (capability.requires === "pipe" && context.selection.type !== "pipe")
    return {
      enabled: false,
    reason: "Select a pipe in the model tree or viewport first."
    };
  if (capability.requires === "wind" && (context.selection.type !== "load" || !context.windConfigured))
    return {
      enabled: false,
    reason: "Select a load case and apply a wind pressure, shape or direction field first."
    };
  if (capability.requires === "load" && context.selection.type !== "load")
    return {
      enabled: false,
    reason: "Select a load case in the model tree first."
    };
  if (capability.requires === "removable" && !["node", "pipe", "support", "material", "section", "component"].includes(context.selection.type))
    return {
      enabled: false,
    reason: "Select a node, pipe, support, material, section or component for removal."
    };
  return {
    enabled: Boolean(capability.route || capability.history),
    reason: capability.description
  };
}
export function capabilityRoute(capability: ToolkitCapability, context: ToolkitContext): ToolkitRoute | undefined {
  if (!capabilityAvailability(capability, context).enabled)
    return undefined;
  return capability.requires === "removable" ? {
    surface: "inspector",
    focusTestId: ["material", "section", "component"].includes(context.selection.type) ? "guarded-removal-panel" : `queue-delete-${context.selection.type}-intent`
  } : capability.route;
}
