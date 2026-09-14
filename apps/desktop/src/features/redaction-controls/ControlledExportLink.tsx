import { useEffect, useMemo, useRef, useState, type AnchorHTMLAttributes, type ReactNode } from "react";
import {
  controlRouteExport,
  type RedactionExportContext
} from "./redactionExportControls";

import {
  isNativeResultSaveRuntime,
  saveNativeResultJson,
  type NativeResultSaveError,
  type NativeResultSaveReceipt
} from "../result-export/nativeResultSave";

type RouteBinding = {
  routeId: string;
  context: RedactionExportContext;
  lossless?: boolean;
  knownPrivateScalar?: boolean;
  exactCanonicalPayload?: boolean;
};

const TEST_ID_BINDINGS: Record<string, RouteBinding> = {
  "result-export-link": { routeId: "DOTH-JSON-001", context: "local_private", lossless: true, exactCanonicalPayload: true },
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
  nativeCurrentBinding?: object | null;
  "data-testid"?: string;
};

export function ControlledExportLink({ href, children, nativeCurrentBinding, ...anchorProps }: Props) {
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
  const exactPayload = !binding.exactCanonicalPayload || sameDecodedJson(controlledPayload, decoded.payload);
  const canonicalIntentMissing = Boolean(binding.exactCanonicalPayload && !explicitIntent);
  const exposureBlocked = controlled.blocked || canonicalIntentMissing || !exactPayload;
  const exposureReason = canonicalIntentMissing ? "LOCAL_PRIVATE_INTENT_REQUIRED" : controlled.blocked
    ? controlled.summary.local_first?.reason_code ?? "EXPORT_POLICY_BLOCKED"
    : !exactPayload ? "CANONICAL_PAYLOAD_MATERIALIZATION_CHANGED" : null;
  const controlledHref = exposureBlocked ? undefined : binding.exactCanonicalPayload
    ? href // original serialized canonical document; never rehash a redacted derivative
    : encodeDataHref(controlledPayload, decoded.mediaType, decoded.isJson);

  const nativeCanonical = Boolean(binding.exactCanonicalPayload && isNativeResultSaveRuntime());
  const nativeName = typeof anchorProps.download === "string" ? anchorProps.download : null;
  const nativeReady = Boolean(controlledHref && nativeName && nativeCurrentBinding && controlled.summary.local_first);
  const generation = useRef(0);
  const identity = useRef<unknown[]>([]);
  const nextIdentity = [href, nativeName, nativeCurrentBinding, explicitIntent, exposureBlocked];
  if (nextIdentity.some((value, index) => value !== identity.current[index])) {
    identity.current = nextIdentity;
    generation.current += 1;
  }
  const mounted = useRef(true);
  const inFlight = useRef(false);
  const [busy, setBusy] = useState(false);
  const [saveStatus, setSaveStatus] = useState<{
    generation: number;
    outcome: "pending" | "saved" | "error";
    receipt?: NativeResultSaveReceipt;
    error?: NativeResultSaveError;
  } | null>(null);
  useEffect(() => {
    mounted.current = true;
    return () => { mounted.current = false; generation.current += 1; };
  }, []);
  const visibleStatus = saveStatus?.generation === generation.current ? saveStatus : null;
  const saveNative = () => {
    if (inFlight.current || !nativeReady || !explicitIntent || !controlledHref || !nativeName || !controlled.summary.local_first) return;
    inFlight.current = true;
    const capturedGeneration = generation.current;
    const request = {
      href: controlledHref,
      file_name: nativeName,
      screening: {
        route_id: binding.routeId,
        export_context: binding.context,
        explicit_local_private_intent: explicitIntent,
        blocked: controlled.blocked,
        materialization_withheld: controlled.summary.materialization_withheld,
        lossless_required: binding.lossless === true,
        exact_payload_match: exactPayload,
        blocking_count: controlled.summary.blocking_count
      },
      local_first: controlled.summary.local_first
    };
    setBusy(true);
    setSaveStatus({ generation: capturedGeneration, outcome: "pending" });
    saveNativeResultJson(request).then(receipt => {
      if (mounted.current && generation.current === capturedGeneration) setSaveStatus({ generation: capturedGeneration, outcome: "saved", receipt });
    }).catch((error: NativeResultSaveError) => {
      if (mounted.current && generation.current === capturedGeneration) setSaveStatus({ generation: capturedGeneration, outcome: "error", error });
    }).finally(() => {
      // Stale UI generations do not release the real in-flight guard early.
      inFlight.current = false;
      if (mounted.current) setBusy(false);
    });
  };

  return (
    <span
      className="controlled-export-control"
      data-local-first-blocked={String(binding.exactCanonicalPayload ? exposureBlocked : controlled.summary.local_first?.blocked ?? true)}
      data-local-first-reason={binding.exactCanonicalPayload && exposureReason ? exposureReason : controlled.summary.local_first?.reason_code ?? "LOCAL_FIRST_EVIDENCE_MISSING"}
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
        {String(exposureBlocked)}
      </span>
      {binding.exactCanonicalPayload && exposureReason ? <span data-testid={`${testId}-canonical-block-reason`}>{exposureReason}</span> : null}
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
      {nativeCanonical ? (
        <>
          <button
            type="button"
            className={anchorProps.className}
            title={anchorProps.title}
            aria-label={anchorProps["aria-label"]}
            data-testid={testId}
            disabled={!nativeReady || busy}
            aria-disabled={!nativeReady || busy}
            onClick={saveNative}
          >{children}</button>
          <span role="status" data-testid={`${testId}-native-save-status`}>
            {visibleStatus?.outcome === "pending" ? "Saving local result JSON…" :
              visibleStatus?.outcome === "saved" ? `Saved ${visibleStatus.receipt!.file_name} (${visibleStatus.receipt!.byte_count} bytes).` :
              visibleStatus?.outcome === "error" ? `Save failed (${visibleStatus.error!.stage}); cleanup=${visibleStatus.error!.cleanup}${visibleStatus.error!.partial_file_name ? `; partial file=${visibleStatus.error!.partial_file_name}` : ""}.` : busy ? "Previous authorized save is still pending." : ""}
          </span>
        </>
      ) : controlledHref ? (
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

// Exact decoded JSON equivalence checks that policy projection/intent stripping
// changed no facts. The original href preserves the separate serialized bytes.
function sameDecodedJson(left: unknown, right: unknown): boolean {
  if(left===right)return true;
  if(left===null||right===null||typeof left!=="object"||typeof right!=="object")return false;
  if(Array.isArray(left)!==Array.isArray(right))return false;
  const a=left as Record<string,unknown>,b=right as Record<string,unknown>;
  const keys=Object.keys(a);return keys.length===Object.keys(b).length&&keys.every(key=>Object.hasOwn(b,key)&&sameDecodedJson(a[key],b[key]));
}
