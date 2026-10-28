// DEL-12-02 plain-data gate (HELP_HUMAN rulings, 2026-10-28: "(a), the plain-data gate at the TS entry
// points. It removes the class instead of modelling JS semantics one primitive at a time."; "An undefined
// own data property on an object is treated as absent: skipped, never classified, exactly as
// JSON.stringify writes it. An undefined array element still fails closed."; "Admit shared references in
// an acyclic graph and refuse only cycles."). Every entry point refuses a non-plain input before any read,
// as a whole-export block `EXPORT_INPUT_NOT_PLAIN_DATA`. All data is invented.
import { runInNewContext } from "node:vm";
import { afterEach, describe, expect, it, vi } from "vitest";
const invokeMock = vi.hoisted(() => vi.fn());
vi.mock("@tauri-apps/api/core", async () => ({ invoke: (await import("../../test/nativeIngress")).ingressDecodingInvoke(invokeMock) }));
import {
  classifyExportItem,
  controlRouteExport,
  controlUserReleasedReportPackageRequest,
  redactExportPayload,
  type RedactionDecision
} from "./redactionExportControls";
import { buildExportRepresentation } from "./RedactionExportControlsPanel";
import { MODEL_DOCUMENT_SAVE_ROUTE_ID, screenModelDocumentDispositions } from "../model-file/modelDocumentSave";
import { reportUnitSystemDisclosure } from "../report/ReportPanel";
import { buildReportPackageRequest } from "../report/reportPackageRequest";
import { buildRenderableReportInput } from "../report/renderableReportInput";
import {
  controlReportDomAndJson,
  controlReportPackageRequest,
  controlReportRendererInput
} from "../report/reportRedactionProjector";
import { demoDenseSolveTransport, demoModel } from "../../test-support/demoReportPackageRequest";
import { buildAnalysisRunPreview, runPreviewMechanics } from "../../services/previewService";
import { buildCurrentSessionInputManifest } from "../../services/inputManifestService";

type Item = Record<string, unknown>;
const P = { privacy_classification: "public", redistribution_status: "public_permissive" };
const REFUSED = ["block_export", "EXPORT_INPUT_NOT_PLAIN_DATA"];
const codes = (decisions: RedactionDecision[]) => decisions.map((decision) => [decision.action, decision.reason_code]);
const refusedEverywhere = (shape: unknown, where: string) => {
  // The shape as a record member, an item member, and the user-release request and model.
  const route = controlRouteExport({ record: { ...P, note: "INVENTED", shape } }, { routeId: "DREP-JSON-002", exportContext: "public_report" });
  expect(codes(route.decisions), `${where} route`).toEqual([REFUSED]);
  expect([route.blocked, route.payload], `${where} route`).toEqual([true, null]);
  const walked = redactExportPayload({ item: { value: "x", ...P, shape } }, { exportContext: "public_report" });
  expect(codes(walked.decisions), `${where} walk`).toEqual([REFUSED]);
  expect([walked.blocked, walked.payload], `${where} walk`).toEqual([true, null]);
  expect(codes([classifyExportItem({ value: "x", ...P, shape }, { exportContext: "public_report" })]), `${where} item`).toEqual([REFUSED]);
  const released = controlUserReleasedReportPackageRequest({ record: { ...P, shape } }, {}, true);
  expect(codes(released.decisions), `${where} release`).toEqual([REFUSED]);
  expect(codes(controlUserReleasedReportPackageRequest({}, { shape }, true).decisions), `${where} model`).toEqual([REFUSED]);
  expect(route.findings.map((finding) => [finding.severity, finding.code])).toEqual([["BLOCKING", "EXPORT_INPUT_NOT_PLAIN_DATA"]]);
};
const recordingGetter = (reads: string[], enumerable: boolean) => {
  const object: Item = { value: "x" };
  Object.defineProperty(object, "privacy_classification", {
    enumerable,
    configurable: true,
    get() {
      reads.push("read");
      return "public";
    }
  });
  return object;
};

describe("DEL-12-02 plain-data gate: Codex r3 repros", () => {
  it("refuses a class instance carrying a hidden __proto__", () => {
    class R {}
    const item = Object.assign(new R(), { value: "x", ...P });
    Object.defineProperty(item, "__proto__", { value: {} });
    expect(codes(redactExportPayload({ item }, { exportContext: "public_report" }).decisions)).toEqual([REFUSED]);
  });

  it("refuses an enumerable __proto__ getter without running it", () => {
    const reads: string[] = [];
    const record: Item = { ...P, note: "INVENTED" };
    Object.defineProperty(record, "__proto__", {
      enumerable: true,
      get() {
        reads.push("getter ran");
        throw new Error("getter ran");
      }
    });
    expect(codes(controlRouteExport({ record }, { routeId: "DREP-JSON-002", exportContext: "public_report" }).decisions)).toEqual([REFUSED]);
    expect(codes(redactExportPayload({ record }, { exportContext: "public_report" }).decisions)).toEqual([REFUSED]);
    expect(codes(controlUserReleasedReportPackageRequest({ record }, {}, true).decisions)).toEqual([REFUSED]);
    expect(reads).toEqual([]);
  });

  it("refuses an array with an own map and never calls it", () => {
    const calls: string[] = [];
    const items = [{ value: "x", privacy_classification: "protected_suspected" }];
    Object.defineProperty(items, "map", {
      value: () => {
        calls.push("map");
        return [{ value: "x", ...P }];
      }
    });
    expect(codes(redactExportPayload(items, { exportContext: "public_report" }).decisions)).toEqual([REFUSED]);
    expect(calls).toEqual([]);
  });

  it("refuses an object cycle and an array cycle instead of overflowing", () => {
    const item: Item = { value: "x", ...P };
    item.self = item;
    expect(codes(redactExportPayload({ item }, { exportContext: "public_report" }).decisions)).toEqual([REFUSED]);
    refusedEverywhere(item, "object cycle");
    const list: unknown[] = [];
    list.push(list);
    refusedEverywhere(list, "array cycle");
  });
});

describe("DEL-12-02 plain-data gate: Codex r4 repros (the single-read snapshot)", () => {
  // HELP_HUMAN ruling, 2026-10-28, (i): one pass validates and copies, reading each descriptor once;
  // any exception in it is a refusal; everything downstream reads only the snapshot.
  const flipFlop = (target: Item) => {
    const reads: string[] = [];
    const proxy = new Proxy(target, {
      getOwnPropertyDescriptor(object, key) {
        const descriptor = Reflect.getOwnPropertyDescriptor(object, key);
        if (key !== "privacy_classification" || descriptor === undefined) return descriptor;
        reads.push("privacy_classification");
        return { ...descriptor, value: reads.length === 1 ? "protected_suspected" : "public" };
      }
    });
    return { proxy, reads };
  };
  const revokedProxy = () => {
    const { proxy, revoke } = Proxy.revocable({}, {});
    revoke();
    return proxy;
  };
  const throwing = (thrown: () => unknown = () => new Error("trap ran")) =>
    new Proxy({ value: "x", ...P } as Item, {
      ownKeys() {
        throw thrown();
      }
    });

  it("reads a flip-flopping Proxy once, at all four entry points, and never includes it", () => {
    const item = flipFlop({ value: "x", ...P });
    expect(codes([classifyExportItem(item.proxy, { exportContext: "public_report" })])).toEqual([["block_export", "PROTECTED_CONTENT_BLOCKED"]]);
    expect(item.reads).toEqual(["privacy_classification"]);
    const walked = flipFlop({ value: "x", ...P });
    expect(codes(redactExportPayload({ item: walked.proxy }, { exportContext: "public_report" }).decisions)).toEqual([
      ["block_export", "PROTECTED_CONTENT_BLOCKED"]
    ]);
    expect(walked.reads).toEqual(["privacy_classification"]);
    const record = flipFlop({ ...P, note: "INVENTED" });
    const route = controlRouteExport({ record: record.proxy }, { routeId: "DREP-JSON-002", exportContext: "public_report" });
    const note = route.decisions.filter((decision) => decision.path.replaceAll("__route_key__", "") === "$.record.note");
    expect(codes(note)).toEqual([["redact_value", "REDISTRIBUTION_STATUS_UNKNOWN"]]);
    expect(record.reads).toEqual(["privacy_classification"]);
    const released = flipFlop({ ...P, note: "INVENTED" });
    const release = controlUserReleasedReportPackageRequest({ record: released.proxy }, {}, true);
    expect(release.blocked).toBe(true);
    expect(codes(release.decisions)).toContainEqual(["block_export", "PROTECTED_CONTENT_BLOCKED"]);
    expect(released.reads).toEqual(["privacy_classification"]);
  });

  it("refuses a throwing Proxy at all four entry points instead of throwing", () => {
    expect(codes([classifyExportItem(throwing(), { exportContext: "public_report" })])).toEqual([REFUSED]);
    expect(codes(redactExportPayload({ item: throwing() }, { exportContext: "public_report" }).decisions)).toEqual([REFUSED]);
    expect(codes(controlRouteExport({ record: throwing() }, { routeId: "DREP-JSON-002", exportContext: "public_report" }).decisions)).toEqual([REFUSED]);
    expect(codes(controlUserReleasedReportPackageRequest({ record: throwing() }, {}, true).decisions)).toEqual([REFUSED]);
    expect(codes(controlUserReleasedReportPackageRequest({}, throwing(), true).decisions)).toEqual([REFUSED]);
  });

  it("refuses a Proxy whose trap throws a revoked Proxy, at all four entry points (Codex final round)", () => {
    // Inspecting the thrown value (`instanceof`) would run its own trap and throw again.
    const input = () => throwing(revokedProxy);
    expect(codes([classifyExportItem(input(), { exportContext: "public_report" })])).toEqual([REFUSED]);
    expect(codes(redactExportPayload(input(), { exportContext: "public_report" }).decisions)).toEqual([REFUSED]);
    expect(codes(redactExportPayload({ item: input() }, { exportContext: "public_report" }).decisions)).toEqual([REFUSED]);
    expect(codes(controlRouteExport({ record: input() }, { routeId: "DREP-JSON-002", exportContext: "public_report" }).decisions)).toEqual([REFUSED]);
    expect(codes(controlUserReleasedReportPackageRequest({ record: input() }, {}, true).decisions)).toEqual([REFUSED]);
    expect(codes(controlUserReleasedReportPackageRequest({}, input(), true).decisions)).toEqual([REFUSED]);
  });

  it("refuses the array key 4294967295, which is not an array index", () => {
    const items = new Array(1) as unknown[];
    (items as unknown as Item)["4294967295"] = "x";
    refusedEverywhere(items, "key 4294967295");
    const filled: unknown[] = ["x"];
    (filled as unknown as Item)["4294967295"] = "x";
    refusedEverywhere(filled, "key 4294967295 beside a full array");
  });

  it("refuses a cycle behind more than 10,000 ancestors, object and array, without overflowing", () => {
    const head: Item = {};
    let tail = head;
    for (let depth = 0; depth < 12_000; depth += 1) {
      const next: Item = {};
      tail.next = next;
      tail = next;
    }
    tail.next = head;
    refusedEverywhere(head, "deep object cycle");
    const first: unknown[] = [];
    let last = first;
    for (let depth = 0; depth < 12_000; depth += 1) {
      const next: unknown[] = [];
      last.push(next);
      last = next;
    }
    last.push(first);
    refusedEverywhere(first, "deep array cycle");
  });

  it("refuses a reparented boxed Boolean and a reparented Map by their internal slots", () => {
    refusedEverywhere(Object.setPrototypeOf(new Boolean(true), Object.prototype), "reparented Boolean");
    refusedEverywhere(Object.setPrototypeOf(new Map([["a", 1]]), Object.prototype), "reparented Map");
    refusedEverywhere(Object.setPrototypeOf(new Date(0), null), "reparented Date");
    refusedEverywhere(Object.setPrototypeOf(new Uint8Array(2), Object.prototype), "reparented typed array");
  });
});

describe("DEL-12-02 plain-data gate: refused shapes", () => {
  class Instance {
    label = "INVENTED";
  }
  const symbolKeyed: Item = { a: 1 };
  (symbolKeyed as Record<symbol, unknown>)[Symbol("invented")] = 1;
  const keyedArray: unknown[] = ["x"];
  (keyedArray as unknown as Item).extra = 1;
  // eslint-disable-next-line no-sparse-arrays
  const holed = [1, , 3];
  const nonEnumerable: Item = { a: 1 };
  Object.defineProperty(nonEnumerable, "hidden", { value: 1, enumerable: false });
  const shapes: [string, () => unknown][] = [
    ["class instance", () => new Instance()],
    ["cross-realm object", () => runInNewContext("({ label: 'INVENTED' })")],
    ["cross-realm array", () => runInNewContext("['INVENTED']")],
    ["Map", () => new Map([["a", 1]])],
    ["Set", () => new Set([1])],
    ["Date", () => new Date(0)],
    ["typed array", () => new Uint8Array([1, 2])],
    ["symbol key", () => symbolKeyed],
    ["non-index array key", () => keyedArray],
    ["array hole", () => holed],
    ["undefined array element", () => [1, undefined]],
    ["non-enumerable data member", () => nonEnumerable],
    ["NaN", () => Number.NaN],
    ["Infinity", () => Number.POSITIVE_INFINITY],
    ["function", () => () => "INVENTED"],
    ["bigint", () => BigInt(1)],
    ["symbol value", () => Symbol("invented")]
  ];
  for (const [name, shape] of shapes) {
    it(`refuses a ${name} at every entry point`, () => refusedEverywhere(shape(), name));
  }

  it("refuses an accessor, enumerable or not, without running its getter", () => {
    for (const enumerable of [true, false]) {
      const reads: string[] = [];
      refusedEverywhere(recordingGetter(reads, enumerable), `accessor enumerable=${enumerable}`);
      expect(codes([classifyExportItem(recordingGetter(reads, enumerable), { exportContext: "public_report" })])).toEqual([REFUSED]);
      expect(reads, `enumerable=${enumerable}`).toEqual([]);
    }
  });

  it("refuses at the entry point, before the payload is otherwise read", () => {
    // A refused input is decided once, at the first refusal's path, and nothing of it is materialized.
    const route = controlRouteExport({ record: { ...P, note: "INVENTED", when: new Date(0) } }, { routeId: "DREP-JSON-002", exportContext: "public_report" });
    expect(route.decisions.map((decision) => decision.path)).toEqual(["$.record.when"]);
    expect(route.summary.route_id).toBe("DREP-JSON-002");
  });
});

describe("DEL-12-02 plain-data gate: admitted data", () => {
  it("reads an undefined object member as absent, on the item and in its provenance (modelDocumentSave shapes)", () => {
    const contexts = ["public_report", "public_example", "shared_model", "downstream_tool", "local_private"] as const;
    const base = { field_class: "provenance_record", record_id: "invented-record", privacy_classification: "private_project_data" };
    for (const exportContext of contexts) {
      for (const explicitLocalPrivateIntent of [false, true]) {
        const decide = (item: Item) => {
          const decision = classifyExportItem(item, { exportContext, explicitLocalPrivateIntent });
          return [decision.action, decision.reason_code, decision.redistribution_status, decision.review_status];
        };
        const where = `${exportContext} ${explicitLocalPrivateIntent}`;
        expect(decide({ ...base, redistribution_status: undefined, review_status: undefined }), where).toEqual(decide(base));
        expect(decide({ ...base, provenance: { redistribution_status: undefined } }), where).toEqual(decide({ ...base, provenance: {} }));
        expect(decide({ field_class: "rule_pack_ref", record_id: "invented-ref", privacy_classification: undefined }), where).toEqual(
          decide({ field_class: "rule_pack_ref", record_id: "invented-ref" })
        );
      }
    }
  });

  it("decides a shared object exactly as two separate copies", () => {
    const shared = { value: "x", ...P };
    const route = (payload: unknown) => controlRouteExport(payload, { routeId: "DREP-JSON-002", exportContext: "public_report" });
    const walk = (payload: unknown) => redactExportPayload(payload, { exportContext: "public_report" });
    const record = { ...P, note: "INVENTED" };
    for (const control of [route, walk]) {
      const sharedRun = control({ a: shared, b: shared, c: { r: record }, d: { r: record } });
      const copiesRun = control({ a: { ...shared }, b: { ...shared }, c: { r: { ...record } }, d: { r: { ...record } } });
      expect(sharedRun.decisions).toEqual(copiesRun.decisions);
      expect(JSON.stringify(sharedRun.payload)).toBe(JSON.stringify(copiesRun.payload));
    }
  });

  it("never mutates another position, or the input, when it redacts one", () => {
    const shared = { value: "x", privacy_classification: "private_project_data", file_path: "INVENTED" };
    const input = { a: shared, b: shared };
    const run = redactExportPayload(input, { exportContext: "public_report" });
    const out = run.payload as { a: Item; b: Item };
    expect(out.a).not.toBe(out.b);
    expect([out.a.value, out.b.value]).toEqual(["[REDACTED]", "[REDACTED]"]);
    out.a.value = "changed";
    expect(out.b.value).toBe("[REDACTED]");
    expect(shared).toEqual({ value: "x", privacy_classification: "private_project_data", file_path: "INVENTED" });
    const included = redactExportPayload({ a: { value: "x", ...P }, b: null }, { exportContext: "public_report" });
    const sharedIncluded = { value: "x", ...P };
    const both = redactExportPayload({ a: sharedIncluded, b: sharedIncluded }, { exportContext: "public_report" }).payload as { a: Item; b: Item };
    expect(both.a).not.toBe(both.b);
    expect(both.a).not.toBe(sharedIncluded);
    expect(codes(included.decisions)).toEqual([["include", "SAFE_PUBLIC_METADATA"]]);
  });
});

describe("DEL-12-02 plain-data gate: the real callers' payloads pass unchanged", () => {
  afterEach(() => {
    invokeMock.mockReset();
    delete (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__;
  });
  const sharesAnObject = (value: unknown): boolean => {
    const seen = new WeakSet<object>();
    const visit = (node: unknown): boolean => {
      if (typeof node !== "object" || node === null) return false;
      if (seen.has(node)) return true;
      seen.add(node);
      return Object.values(node).some(visit);
    };
    return visit(value);
  };
  const notRefused = (decisions: RedactionDecision[]) =>
    expect(decisions.filter((decision) => decision.reason_code === "EXPORT_INPUT_NOT_PLAIN_DATA")).toEqual([]);

  it("admits the report package request and the renderer input, shared references and all", async () => {
    // As `workspaceSession.ts` (DREP-PACKAGE-SAVE-009) and `renderedReportAssembly.ts` (DREP-IPC-003)
    // build them after a dense solve of the bundled demo model.
    (window as unknown as Record<string, unknown>).__TAURI_INTERNALS__ = {};
    invokeMock.mockImplementation(demoDenseSolveTransport().invoke);
    const model = demoModel();
    const result = await runPreviewMechanics(model, "dense_scrutiny");
    const producer = result.producer!;
    const inputManifest = await buildCurrentSessionInputManifest({
      model,
      solver: {
        solver_name: producer.component_name,
        solver_version: producer.component_version,
        solver_build_ref: `${producer.component_name}@${producer.component_version}`,
        solver_mode: "dense_scrutiny",
        settings: { nonlinear_iteration_policy: "DEC-046-CV-B-product-preview-active-set-count-v1", sparse_evidence_lane: false }
      },
      active_rule_packs: [],
      external_assets: []
    });
    const analysisRun = await buildAnalysisRunPreview(result, { inputManifest, rulePackExclusions: [] });
    const request = await buildReportPackageRequest({
      model, result, analysisRun, inputManifest, projectSummary: null, comparison: null, ruleCheckAggregate: null, ruleCheckRun: null
    });
    expect(sharesAnObject(request)).toBe(true);
    const packaged = controlReportPackageRequest(request, true);
    notRefused(packaged.decisions);
    expect(packaged.blocked).toBe(false);
    expect(JSON.stringify(packaged.payload)).toBe(JSON.stringify(request));
    const input = await buildRenderableReportInput({
      model, result, analysisRun, inputManifest, projectSummary: null, renderContext: { exportContext: "local_private", generatedAt: null }
    });
    expect(sharesAnObject(input)).toBe(true);
    const rendered = controlReportRendererInput(input, true);
    notRefused(rendered.decisions);
    expect(rendered.blocked).toBe(false);
    expect(JSON.stringify(rendered.payload)).toBe(JSON.stringify(input));
    // The report panel's DREP-UI-001/DREP-JSON-002 packet shares its unit-system reference
    // (`ReportPanel.tsx`); the panel's own tests decide the whole packet.
    const disclosure = reportUnitSystemDisclosure(model, result);
    expect(sharesAnObject({ disclosure })).toBe(true);
    notRefused(controlReportDomAndJson({ unit_system_disclosure: disclosure }).decisions);
  });

  it("admits the model-document save's screened items and model, and the review panel's representation", () => {
    // `modelDocumentSave.ts` builds items with `?? undefined` labels; `RedactionExportControlsPanel.tsx`
    // builds its representation from the model; `ControlledExportLink.tsx` passes parsed JSON.
    const model = demoModel();
    const screening = screenModelDocumentDispositions(
      { ...model, provenance_record: { record_id: "invented-provenance" } },
      { explicitLocalPrivateIntent: true }
    );
    expect(screening.decisions.length).toBeGreaterThan(0);
    notRefused(screening.decisions);
    const saved = controlRouteExport(model, { routeId: MODEL_DOCUMENT_SAVE_ROUTE_ID, exportContext: "local_private", explicitLocalPrivateIntent: true });
    notRefused(saved.decisions);
    const representation = buildExportRepresentation(model);
    const reviewed = redactExportPayload(representation, { exportContext: "local_private", explicitLocalPrivateIntent: true });
    notRefused(reviewed.decisions);
    expect(JSON.stringify(reviewed.payload)).toBe(JSON.stringify(representation));
    notRefused(controlRouteExport(JSON.parse(JSON.stringify(model)), { routeId: "DREP-JSON-002", exportContext: "public_report" }).decisions);
  });
});
