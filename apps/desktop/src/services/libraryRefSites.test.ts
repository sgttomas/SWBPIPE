// The one walk of the model's `library_ref`s (W-13 §3.5; W-6a): the open-time
// store check and the save screen both read it, and the native save's walk is
// pinned to the same table (`model_document_save.rs`
// `library_ref_walk_matches_the_shared_site_table`). Invented data.
import { describe, expect, it } from "vitest";
import siteTable from "../../../../fixtures/model_document/library_ref_sites.json";
import { modelLibraryRefs as saveScreenRefs } from "../features/model-file/modelDocumentSave";
import { libraryRefSites, modelLibraryRefs } from "./libraryImportService";

type Site = { path: string; entity_ref: string; identity: [string, string, string | null] | null };
const sites = siteTable.sites as Site[];

describe("the shared library_ref walk", () => {
  it("finds every library_ref at any depth, keyed to its nearest owner", () => {
    expect(libraryRefSites(siteTable.document).map(({ path, entity_ref }) => ({ path, entity_ref })))
      .toEqual(sites.map(({ path, entity_ref }) => ({ path, entity_ref })));
  });

  it("gives the save screen the table's normalized identities", () => {
    expect(saveScreenRefs(siteTable.document).map(({ path, library_kind, library_id, record_id }) => ({ path, identity: [library_kind, library_id, record_id] })))
      .toEqual(sites.filter((site) => site.identity !== null).map(({ path, identity }) => ({ path, identity })));
  });

  it("sends the open-time check every site as recorded; the backend normalizes", () => {
    const sent = modelLibraryRefs(siteTable.document);
    expect(sent.map((reference) => reference.entity_ref)).toEqual(sites.map((site) => site.entity_ref));
    expect(sent[2]).toEqual({ entity_ref: "material:padded", library_kind: " material ", library_id: " matlib.invented.sites ", record_id: "\tmat.padded " });
    // A non-string kind is sent empty, so it reads as unresolved instead of failing the check.
    expect(sent[7]).toEqual({ entity_ref: "support:not-text", library_kind: "", library_id: "matlib.invented.sites" });
  });
});
