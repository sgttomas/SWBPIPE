// The Report Packet's "Local JSON" link (DEL-12-02, route DREP-JSON-002 in the public_report
// context): its bytes are the route control's output for the packet, controlled once, the
// panel's own control decides whether the link is shown, and its Details shows only a
// classification the rule recognises. Invented content only.
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { demoSession, downloadableLinks, linkText, panelProps, withWeight } from "../../test/wholeOutputSessions";
import { controlRouteExport } from "../redaction-controls/redactionExportControls";
import { ReportPanel } from "./ReportPanel";

const observed = vi.hoisted(() => ({ packets: [] as unknown[], blockPanel: false }));

vi.mock("./reportRedactionProjector", async (importOriginal) => {
  const actual = await importOriginal<typeof import("./reportRedactionProjector")>();
  return {
    ...actual,
    controlReportDomAndJson: (payload: unknown) => {
      observed.packets.push(structuredClone(payload));
      const controlled = actual.controlReportDomAndJson(payload);
      return observed.blockPanel ? { ...controlled, blocked: true, payload: null } : controlled;
    }
  };
});

afterEach(() => {
  cleanup();
  observed.packets = [];
  observed.blockPanel = false;
});

const PUBLIC_LABELS = { privacy_classification: "public_metadata", redistribution_status: "public_permissive" };
const SLOT_MARK = "invented_benign_slot_c9";

/** What the route DREP-JSON-002 gives for `packet` in the public_report context, as link bytes. */
function controlledBytes(packet: unknown): string {
  const controlled = controlRouteExport(packet, { routeId: "DREP-JSON-002", exportContext: "public_report" });
  expect(controlled.blocked).toBe(false);
  expect(controlled.summary.redacted_count).toBeGreaterThan(0);
  return `${JSON.stringify(controlled.payload, null, 2)}\n`;
}

describe("Report Packet Local JSON link bytes", () => {
  const sessions: Array<[string, () => ReturnType<typeof demoSession>]> = [
    ["the maintained demo session", () => demoSession()],
    [
      "a false professional claim on a record with no scalar value",
      () => demoSession(withWeight((w) => ({ ...w, note: { ...PUBLIC_LABELS, professional_claim: false, [SLOT_MARK]: true } })))
    ]
  ];
  for (const [label, session] of sessions) {
    it(`${label}: the link holds the route's control of the packet, never the packet as given`, async () => {
      render(<ReportPanel {...panelProps(await session())} />);
      const packet = observed.packets.at(-1);
      expect(packet).toBeDefined();
      const bytes = linkText(screen.getByTestId("report-export-link"));
      expect(bytes).toBe(controlledBytes(packet));
      expect(bytes).not.toBe(`${JSON.stringify(packet, null, 2)}\n`);
      expect(bytes).not.toBe(JSON.stringify(packet, null, 2));
    });
  }

  it("the panel's own control blocking: no link and no report lines, though the link's control would admit the packet", async () => {
    observed.blockPanel = true;
    render(<ReportPanel {...panelProps(await demoSession())} />);
    controlledBytes(observed.packets.at(-1));
    expect(screen.queryByTestId("report-export-link")).toBeNull();
    expect(screen.queryByTestId("report-packet-body")).toBeNull();
    expect(downloadableLinks()).toHaveLength(0);
    expect(screen.getByTestId("report-redaction-blocked")).toHaveTextContent("Report exposure blocked by redaction controls");
  });
});

describe("Report Packet Local JSON link Details", () => {
  const INVENTED_LABEL = "invented-boundary-label-q7";
  it("shows a classification the rule recognises, and one fixed word for any other label", async () => {
    const base = await demoSession();
    const model = {
      ...base.model,
      data_boundary: { ...base.model.data_boundary, value: "invented-boundary-value", privacy_classification: INVENTED_LABEL }
    };
    render(<ReportPanel {...panelProps(await demoSession(model))} />);
    expect(linkText(screen.getByTestId("report-export-link"))).not.toContain(INVENTED_LABEL);
    const details = screen.getByTestId("report-export-link-redaction-decisions").textContent ?? "";
    expect(details).toMatch(/path=[^;\n]*data_boundary; classification=unrecognized;/);
    expect(details).toContain("classification=private_project_data");
    expect(document.body.innerHTML).not.toContain(INVENTED_LABEL);
  });
});
