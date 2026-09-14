import { useMemo, useState, type AnchorHTMLAttributes, type ReactNode } from "react";
import {
  controlRouteExport,
  type RedactionExportContext
} from "./redactionExportControls";

type RouteBinding = {
  routeId: string;
  context: RedactionExportContext;
  lossless?: boolean;
  knownPrivateScalar?: boolean;
};

const TEST_ID_BINDINGS: Record<string, RouteBinding> = {
  "report-export-link": { routeId: "DREP-JSON-002", context: "public_report" },
  "report-lint-export-link": { routeId: "DREP-LINT-JSON-007", context: "public_report" },
  "rendered-report-save": { routeId: "DREP-HTML-SAVE-005", context: "public_report", lossless: true },
  "caepipe-external-export-link": { routeId: "DOTH-CAEPIPE-LOCAL-006", context: "local_private", lossless: true },
  "caepipe-external-csv-link": {
    routeId: "DOTH-CAEPIPE-LOCAL-006",
    context: "local_private",
    lossless: true,
    knownPrivateScalar: true
  },
  "secret-private-library-export-link": { routeId: "DOTH-PRIVATE-004", context: "local_private" },
  "pcf-export-link": { routeId: "DOTH-FORMAT-003", context: "downstream_tool" },
  "pcf-text-link": { routeId: "DOTH-FORMAT-003", context: "downstream_tool", lossless: true },
  "caepipe-mbf-export-link": { routeId: "DOTH-FORMAT-003", context: "downstream_tool" },
  "caepipe-mbf-text-link": { routeId: "DOTH-FORMAT-003", context: "downstream_tool", lossless: true },
  "stress-neutral-export-link": { routeId: "DOTH-FORMAT-003", context: "downstream_tool" },
  "stress-neutral-csv-link": { routeId: "DOTH-FORMAT-003", context: "downstream_tool", lossless: true },
  "review-geometry-export-link": { routeId: "DOTH-FORMAT-003", context: "downstream_tool", lossless: true }
};

const DOWNSTREAM_TEST_IDS = new Set([
  "adapter-framework-export-link",
  "export-adapter-sdk-export-link",
  "handoff-export-link",
  "local-fea-export-link",
  "native-package-link",
  "result-export-link"
]);

export function routeBindingForTestId(testId: string): RouteBinding {
  const exact = TEST_ID_BINDINGS[testId];
  if (exact) return exact;
  if (DOWNSTREAM_TEST_IDS.has(testId)) {
    return { routeId: "DOTH-HANDOFF-002", context: "downstream_tool" };
  }
  return { routeId: "DOTH-JSON-001", context: "local_private" };
}

type Props = Omit<AnchorHTMLAttributes<HTMLAnchorElement>, "href"> & {
  href: string;
  children: ReactNode;
  "data-testid"?: string;
};

export function ControlledExportLink({ href, children, ...anchorProps }: Props) {
  const testId = String(anchorProps["data-testid"] ?? "controlled-export-link");
  const binding = routeBindingForTestId(testId);
  const [explicitIntent, setExplicitIntent] = useState(false);
  const decoded = useMemo(() => decodeDataHref(href), [href]);
  const routePayload = useMemo(
    () =>
      binding.knownPrivateScalar
        ? {
            field_id: `${binding.routeId}:parser_csv`,
            field_class: "parser_csv",
            privacy_classification: "private_project_data",
            redistribution_status: "private_only",
            review_status: "accepted",
            value: decoded.payload
          }
        : decoded.payload,
    [binding.knownPrivateScalar, binding.routeId, decoded.payload]
  );
  const controlled = useMemo(
    () =>
      controlRouteExport(routePayload, {
        routeId: binding.routeId,
        exportContext: binding.context,
        explicitLocalPrivateIntent: explicitIntent,
        requireLosslessMaterialization: binding.lossless
      }),
    [binding.context, binding.lossless, binding.routeId, explicitIntent, routePayload]
  );
  const controlledPayload =
    binding.knownPrivateScalar && isObject(controlled.payload)
      ? controlled.payload.value
      : controlled.payload;
  const controlledHref = controlled.blocked
    ? undefined
    : encodeDataHref(controlledPayload, decoded.mediaType, decoded.isJson);

  return (
    <span
      className="controlled-export-control"
      data-local-first-blocked={String(controlled.summary.local_first?.blocked ?? true)}
      data-local-first-reason={controlled.summary.local_first?.reason_code ?? "LOCAL_FIRST_EVIDENCE_MISSING"}
      data-route-id={binding.routeId}
    >
      {binding.context === "local_private" ? (
        <label>
          <input
            checked={explicitIntent}
            data-testid={`${testId}-local-private-intent`}
            onChange={(event) => setExplicitIntent(event.currentTarget.checked)}
            type="checkbox"
          />
          Include known private values in this local export
        </label>
      ) : null}
      <span data-testid={`${testId}-redaction-summary`}>
        decisions={controlled.summary.decision_count}; findings={controlled.summary.finding_count}; blocked=
        {String(controlled.blocked)}
      </span>
      <pre aria-label={`${testId} redaction decisions`} data-testid={`${testId}-redaction-decisions`}>
        {controlled.decisions
          .map(
            (decision) =>
              `path=${decision.path}; classification=${decision.privacy_classification}; action=${decision.action}; reason=${decision.reason_code}`
          )
          .join("\n")}
      </pre>
      <pre aria-label={`${testId} redaction findings`} data-testid={`${testId}-redaction-findings`}>
        {controlled.findings
          .map(
            (finding) =>
              `path=${finding.path}; class=${finding.class}; severity=${finding.severity}; reason=${finding.code}`
          )
          .join("\n")}
      </pre>
      {controlledHref ? (
        <a {...anchorProps} href={controlledHref}>
          {children}
        </a>
      ) : (
        <span {...anchorProps} aria-disabled="true">
          {children}
        </span>
      )}
    </span>
  );
}

function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function decodeDataHref(href: string): { payload: unknown; mediaType: string; isJson: boolean } {
  const [prefix, encoded = ""] = href.split(",", 2);
  const mediaType = prefix.startsWith("data:") ? prefix.slice(5).replace(/;.*$/, "") : "application/octet-stream";
  const text = decodeURIComponent(encoded);
  const isJson = mediaType.includes("json");
  if (!isJson) return { payload: text, mediaType, isJson };
  try {
    return { payload: JSON.parse(text), mediaType, isJson };
  } catch {
    return { payload: text, mediaType, isJson: false };
  }
}

function encodeDataHref(payload: unknown, mediaType: string, isJson: boolean): string {
  const text = isJson ? `${JSON.stringify(payload, null, 2)}\n` : String(payload ?? "");
  return `data:${mediaType};charset=utf-8,${encodeURIComponent(text)}`;
}
