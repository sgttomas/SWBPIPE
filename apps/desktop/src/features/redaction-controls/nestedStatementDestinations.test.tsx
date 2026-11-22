// End to end through the governed links (DEL-12-02 AC-003): an explicit blocking statement held
// beneath a labelled item, or on an object with no scalar `value`, in a model member the packet
// builders copy whole, blocks the whole packet, so no downloadable link exists. Each statement is
// paired with a benign variant at the same slot, the statement's member made benign: its link
// exists and carries the slot, so what removes the link is the statement and not the edit.
// Invented content only.
import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import {
  demoSession,
  downloadableLinks,
  everySlotRefs,
  handoffLinkText,
  handoffSession,
  linkText,
  panelProps,
  withWeight
} from "../../test/wholeOutputSessions";
import { HandoffPanel } from "../handoff/HandoffPanel";
import { ReportPanel } from "../report/ReportPanel";

afterEach(cleanup);

const PUBLIC_LABELS = { privacy_classification: "public_metadata", redistribution_status: "public_permissive" };
const CARRIED = "invented-carried-note-c9";
/** A member name the benign variant adds at the slot, found in the link's bytes when the slot is
 * carried, redacted or not. */
const SLOT_MARK = "invented_benign_slot_c9";

type WeightEdit = (weight: Record<string, unknown>) => unknown;

describe("Report Packet Local JSON link: statements in a component quantity copied whole", () => {
  const cases: Array<[string, WeightEdit, WeightEdit]> = [
    [
      "a protected record nested beneath a labelled quantity",
      (w) => ({ ...w, ...PUBLIC_LABELS, extra: { privacy_classification: "protected_suspected", note: CARRIED } }),
      (w) => ({ ...w, ...PUBLIC_LABELS, extra: { privacy_classification: "public_metadata", note: CARRIED, [SLOT_MARK]: true } })
    ],
    [
      "a professional claim nested beneath a labelled quantity",
      (w) => ({ ...w, ...PUBLIC_LABELS, extra: { professional_claim: true, note: CARRIED } }),
      (w) => ({ ...w, ...PUBLIC_LABELS, extra: { professional_claim: false, note: CARRIED, [SLOT_MARK]: true } })
    ],
    [
      "a professional claim on a record with no scalar value",
      (w) => ({ ...w, note: { ...PUBLIC_LABELS, professional_claim: true, text_note: CARRIED } }),
      (w) => ({ ...w, note: { ...PUBLIC_LABELS, professional_claim: false, text_note: CARRIED, [SLOT_MARK]: true } })
    ],
    [
      "a professional claim on a record with no scalar value (benign as null)",
      (w) => ({ ...w, note: { ...PUBLIC_LABELS, professional_claim: true, text_note: CARRIED } }),
      (w) => ({ ...w, note: { ...PUBLIC_LABELS, professional_claim: null, text_note: CARRIED, [SLOT_MARK]: true } })
    ],
    [
      "a quarantined review status on a record with no scalar value",
      (w) => ({ ...w, note: { ...PUBLIC_LABELS, review_status: "quarantined", text_note: CARRIED } }),
      (w) => ({ ...w, note: { ...PUBLIC_LABELS, review_status: "accepted", text_note: CARRIED, [SLOT_MARK]: true } })
    ],
    [
      "secret material on a record with no scalar value",
      (w) => ({ ...w, note: { secret_material_present: true, text_note: CARRIED } }),
      (w) => ({ ...w, note: { secret_material_present: false, text_note: CARRIED, [SLOT_MARK]: true } })
    ]
  ];
  for (const [label, statement, benign] of cases) {
    it(`${label}: no packet link exists, an earlier one included`, async () => {
      const { rerender } = render(<ReportPanel {...panelProps(await demoSession())} />);
      expect(downloadableLinks()).toHaveLength(1);
      rerender(<ReportPanel {...panelProps(await demoSession(withWeight(statement)))} />);
      expect(screen.queryByTestId("report-export-link")).toBeNull();
      expect(downloadableLinks()).toHaveLength(0);
      expect(document.body.innerHTML).not.toContain(CARRIED);
    });

    it(`${label}, made benign at the same slot: the packet link exists and carries the slot`, async () => {
      render(<ReportPanel {...panelProps(await demoSession(withWeight(benign)))} />);
      const links = downloadableLinks();
      expect(links).toHaveLength(1);
      expect(screen.getByTestId("report-export-link")).toBe(links[0]);
      expect(linkText(links[0] as HTMLElement)).toContain(SLOT_MARK);
    });
  }
});

describe("handoff summary link: statements in a rule-pack reference copied whole", () => {
  const cases: Array<[string, Record<string, unknown>, Record<string, unknown>]> = [
    [
      "a protected redistribution status on a reference with no scalar value",
      { redistribution_status: "protected_suspected", note: CARRIED },
      { redistribution_status: "public_permissive", note: CARRIED, [SLOT_MARK]: true }
    ],
    [
      "a professional claim on a reference with no scalar value",
      { professional_claim: "invented", note: CARRIED },
      { professional_claim: false, note: CARRIED, [SLOT_MARK]: true }
    ],
    [
      "a protected record nested beneath a labelled reference",
      { value: "invented", ...PUBLIC_LABELS, extra: { privacy_classification: "protected_suspected", note: CARRIED } },
      { value: "invented", ...PUBLIC_LABELS, extra: { privacy_classification: "public_metadata", note: CARRIED, [SLOT_MARK]: true } }
    ]
  ];
  const editedRefs = (members: Record<string, unknown>) => {
    const refs = everySlotRefs();
    Object.assign(refs[0] as unknown as Record<string, unknown>, members);
    return refs;
  };
  for (const [label, statement, benign] of cases) {
    it(`${label}: no handoff link exists, an earlier one included`, async () => {
      const { rerender } = render(<HandoffPanel {...(await handoffSession(everySlotRefs()))} />);
      expect(downloadableLinks()).toHaveLength(1);
      rerender(<HandoffPanel {...(await handoffSession(editedRefs(statement)))} />);
      expect(screen.getByTestId("handoff-export-link")).not.toHaveAttribute("href");
      expect(downloadableLinks()).toHaveLength(0);
      expect(document.body.innerHTML).not.toContain(CARRIED);
    });

    it(`${label}, made benign at the same slot: the handoff link exists and carries the slot`, async () => {
      render(<HandoffPanel {...(await handoffSession(editedRefs(benign)))} />);
      expect(downloadableLinks()).toHaveLength(1);
      expect(handoffLinkText()).toContain(SLOT_MARK);
    });
  }
});
