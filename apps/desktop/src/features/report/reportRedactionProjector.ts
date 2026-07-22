import {
  controlRouteExport,
  type ControlledRouteExport
} from "../redaction-controls/redactionExportControls";

export function controlReportDomAndJson(payload: unknown): ControlledRouteExport {
  return controlRouteExport(payload, {
    routeId: "DREP-UI-001/DREP-JSON-002",
    exportContext: "public_report"
  });
}

export function controlReportRendererInput(payload: unknown): ControlledRouteExport {
  return controlRouteExport(payload, {
    routeId: "DREP-IPC-003",
    exportContext: "public_report",
    requireLosslessMaterialization: true
  });
}
