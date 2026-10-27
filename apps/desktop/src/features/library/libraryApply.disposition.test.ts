// W-17 object form (DEL-03-01 Design apply-library-records.md §2.5): a stored
// library record whose disposition no verdict can read is refused by name
// (MODEL-DISPOSITION-RECORD-MALFORMED), with the native export screen's rule
// (export_disposition_screen.rs StoredMembers::Refuse): a redistribution_status
// or review_status holding an array or an object, or a provenance written as an
// array, at any depth of the library metadata or the selected record. Before
// this rule such a record applied as `pending` (or `accepted` from its
// provenance), and every export of the model then refused. Every library value
// here is invented.
import { describe, expect, it } from "vitest";
import { loadPreviewModel } from "../../services/previewService";
import type { PreviewModel } from "../../types";
import { buildSectionInsert, DISPOSITION_RECORD_MALFORMED, hangerLibraryProvenance, unreadableDispositions } from "./libraryApply";

type Json = Record<string, any>;

const PRIVATE = { source_name: "Invented private test source", source_location: "invented-local:test", source_license: "private",
  contributor: "Test author", contributor_certification: "Invented values, no engineering authority", redistribution_status: "private_only", review_status: "pending" };
const ACCEPTED = { ...PRIVATE, review_status: "accepted" };

function dimension(kind: string, magnitude: number): Json {
  return { dimension_id: `d.${kind}`, dimension_kind: kind, value_status: "private_user_supplied", required_for: "section_property_calculation",
    provenance: PRIVATE, review_status: "pending",
    value: { magnitude, unit: "m", dimension: "length", value_status: "private_user_supplied", provenance: PRIVATE } };
}

function sectionLibrary(): Json {
  return {
    schema_version: "0.1.0",
    section_library: { library_id: "seclib.invented.private", name: "Invented private section library name", library_scope: "private_user_library",
      privacy_class: "private_user_supplied", provenance: PRIVATE, review_status: "pending" },
    section_records: [{ section_id: "sec.invented.pipe", name: "Invented library pipe", section_type: "pipe", privacy_class: "private_user_supplied",
      redistribution_status: "private_only", dimensions: [dimension("outside_diameter", 0.2), dimension("wall_thickness", 0.009)],
      completeness: [], provenance: ACCEPTED, review_status: "pending" }]
  };
}

async function demo(): Promise<PreviewModel> {
  return structuredClone(await loadPreviewModel());
}

const insert = async (document: Json) =>
  buildSectionInsert({ model: await demo(), document, recordId: "sec.invented.pipe", newId: "section:library" });

type Edit = (document: Json) => void;
const record = (document: Json) => document.section_records[0];

// [label, edit, the level the refusal names, the members it names]
const UNREADABLE: Array<[string, Edit, "record" | "library", string]> = [
  ["own review_status array (Codex's probe: applied as pending)", (d) => { record(d).review_status = ["quarantined"]; }, "record", "review_status"],
  ["own review_status object", (d) => { record(d).review_status = { status: "rejected" }; }, "record", "review_status"],
  ["own redistribution_status array", (d) => { record(d).redistribution_status = ["protected_suspected"]; }, "record", "redistribution_status"],
  ["own redistribution_status object", (d) => { record(d).redistribution_status = { status: "rejected" }; }, "record", "redistribution_status"],
  ["own review_status array over an accepted provenance", (d) => { record(d).review_status = ["quarantined"]; record(d).provenance = ACCEPTED; }, "record", "review_status"],
  ["provenance review_status array", (d) => { record(d).provenance = { ...ACCEPTED, review_status: ["quarantined"] }; }, "record", "review_status"],
  ["provenance redistribution_status object", (d) => { record(d).provenance = { ...ACCEPTED, redistribution_status: { status: "protected_suspected" } }; }, "record", "redistribution_status"],
  ["positional provenance", (d) => { record(d).provenance = ["Invented", "private_only", "accepted"]; }, "record", "provenance"],
  ["nested: a copied dimension's value provenance", (d) => { record(d).dimensions[0].value.provenance = { ...PRIVATE, review_status: ["quarantined"] }; }, "record", "review_status"],
  ["nested: a positional provenance deep in the record", (d) => { record(d).invented_notes = { deeper: [{ provenance: ["Invented", "quarantined"] }] }; }, "record", "provenance"],
  ["nested: several members", (d) => { record(d).review_status = { status: "rejected" }; record(d).dimensions[1].redistribution_status = ["rejected"]; }, "record", "redistribution_status, review_status"],
  ["library metadata review_status array", (d) => { d.section_library.review_status = ["quarantined"]; }, "library", "review_status"],
  ["library metadata provenance redistribution_status object", (d) => { d.section_library.provenance = { ...PRIVATE, redistribution_status: { status: "rejected" } }; }, "library", "redistribution_status"],
  ["library metadata positional provenance", (d) => { d.section_library.provenance = ["Invented", "private_only"]; }, "library", "provenance"]
];

describe("an unreadable stored disposition is refused by name, never applied", () => {
  it("applies the clean control", async () => {
    const plan = await insert(sectionLibrary());
    expect(plan.provenanceRecord.review_status).toBe("pending");
  });

  for (const [label, edit, level, members] of UNREADABLE) {
    it(`refuses ${label}`, async () => {
      const document = sectionLibrary();
      edit(document);
      const refusal = insert(document);
      await expect(refusal).rejects.toThrow(`${DISPOSITION_RECORD_MALFORMED}: `);
      const message = await refusal.then(() => "", (error: Error) => error.message);
      // It names the library and the record, the members, and the way forward.
      expect(message).toContain("seclib.invented.private");
      expect(message).toContain("sec.invented.pipe");
      expect(message).toContain(level === "record" ? `Record sec.invented.pipe of the section library seclib.invented.private holds ${members} ` : `The library metadata of the section library seclib.invented.private holds ${members} `);
      expect(message).toContain("Open the library in the Library Manager");
    });
  }

  it("refuses library metadata that is not an object, naming the record", async () => {
    const document = sectionLibrary();
    document.section_library = ["seclib.invented.private", "pending"];
    await expect(insert(document)).rejects.toThrow(`${DISPOSITION_RECORD_MALFORMED}: The section library's section_library is an array, not an object, so its disposition cannot be read and record sec.invented.pipe is not applied.`);
  });

  it("does not refuse what the native screen accepts: object classification and non-text scalars", async () => {
    // Controls (export_disposition_screen.rs stored_item_malformed;
    // model_document_save b1r1_classification_objects_follow_the_import_verdict).
    const document = sectionLibrary();
    document.section_library.classification = { redistribution_status: "private_only" };
    record(document).classification = { value: "protected_suspected" };
    record(document).dimensions[0].classification = { review_status: "accepted" };
    record(document).privacy_classification = { value: "private_user_data" };
    record(document).invented_notes = { review_status: null, redistribution_status: 7 };
    expect(unreadableDispositions(document)).toEqual([]);
    const plan = await insert(document);
    expect(plan.provenanceRecord.review_status).toBe("pending");
  });

  it("refuses the hanger path the same way", async () => {
    const library = { library_id: "hanger.invented", name: "H", privacy_class: "private_user_supplied", provenance: PRIVATE };
    const hanger = { hanger_id: "hanger.invented.one", name: "H1", provenance: PRIVATE };
    const quantity = { magnitude: 1, unit: "N", dimension: "force", provenance: PRIVATE };
    await expect(hangerLibraryProvenance(library, hanger, [quantity])).resolves.toBeTruthy();
    for (const [lib, rec, level] of [
      [{ ...library, review_status: ["quarantined"] }, hanger, "The library metadata of the hanger library hanger.invented holds review_status "],
      [library, { ...hanger, provenance: ["Invented", "quarantined"] }, "Record hanger.invented.one of the hanger library hanger.invented holds provenance "],
      [library, { ...hanger, hanger: { installed_load: { ...quantity, provenance: { ...PRIVATE, redistribution_status: { status: "rejected" } } } } }, "Record hanger.invented.one of the hanger library hanger.invented holds redistribution_status "]
    ] as const) {
      await expect(hangerLibraryProvenance(lib as Json, rec as Json, [quantity])).rejects.toThrow(`${DISPOSITION_RECORD_MALFORMED}: ${level}`);
    }
  });
});
