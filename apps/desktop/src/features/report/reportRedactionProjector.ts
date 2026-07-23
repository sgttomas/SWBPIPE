import {
  controlRouteExport,
  type ControlledRouteExport
} from "../redaction-controls/redactionExportControls";

export const REPORT_PACKAGE_SAVE_ROUTE_ID = "DREP-PACKAGE-SAVE-009";

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

export function controlReportPackageRequest(
  payload: unknown,
  explicitLocalPrivateIntent: boolean
): ControlledRouteExport {
  return controlRouteExport(payload, {
    routeId: REPORT_PACKAGE_SAVE_ROUTE_ID,
    exportContext: "local_private",
    explicitLocalPrivateIntent,
    requireLosslessMaterialization: true
  });
}
