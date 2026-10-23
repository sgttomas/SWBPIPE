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

export const REPORT_RENDER_IPC_ROUTE_ID = "DREP-IPC-003";
export const RENDERED_REPORT_HTML_ROUTE_ID = "DREP-HTML-SAVE-005";

/** Renderer input control (DREP-IPC-003). Without the panel's own explicit
 * local-private selection the render runs in the public-report context, which
 * withholds private inputs. With it, the render runs in the local-private
 * context under that wrapper-owned intent; intent carried by the payload is
 * stripped by the route projector and never grants exposure. */
export function controlReportRendererInput(
  payload: unknown,
  explicitLocalPrivateIntent = false
): ControlledRouteExport {
  return controlRouteExport(
    payload,
    explicitLocalPrivateIntent
      ? {
          routeId: REPORT_RENDER_IPC_ROUTE_ID,
          exportContext: "local_private",
          explicitLocalPrivateIntent: true,
          requireLosslessMaterialization: true
        }
      : {
          routeId: REPORT_RENDER_IPC_ROUTE_ID,
          exportContext: "public_report",
          requireLosslessMaterialization: true
        }
  );
}

/** Control of a local-private rendered document (canonical HTML or its derived
 * print view) before any preview, save or print sink receives it. */
export function controlLocalPrivateRenderedHtml(
  html: string,
  explicitLocalPrivateIntent: boolean
): ControlledRouteExport {
  return controlRouteExport(html, {
    routeId: RENDERED_REPORT_HTML_ROUTE_ID,
    exportContext: "local_private",
    explicitLocalPrivateIntent,
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
