// W-13 §3.5 / W-17 §2.7: the store's library-reference warnings appear as
// warning entries of the diagnostics list, keyed to the entity, and in that
// entity's Provenance record section; they disappear once the entity no
// longer carries the reference they were raised on. Invented data.
import { act, cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { loadPreviewModel } from "../../services/previewService";
import type { LibraryRefFinding } from "../../services/libraryImportService";
import type { PreviewModel } from "../../types";
import { DiagnosticsPanel } from "../diagnostics/DiagnosticsPanel";
import { ModelDataAnnotations } from "../model-data/ModelDataAnnotations";
import { currentLibraryRefFindings, publishLibraryRefFindings } from "./libraryRefFindings";

afterEach(() => { cleanup(); publishLibraryRefFindings("", []); });

async function referencedModel(): Promise<PreviewModel> {
  const model = structuredClone(await loadPreviewModel());
  model.materials![0].provenance_record = { source_name: "Invented", library_ref: { library_kind: "material", library_id: "matlib.invented.refs", library_version: "sha256:recorded", record_id: "mat.refs" } };
  model.sections = [{ id: "section:refs", name: "Invented", section_type: "pipe", provenance: "invented",
    properties: { outside_diameter: { value: 0.168, unit: "m" }, wall_thickness: { value: 0.007, unit: "m" } },
    provenance_record: { source_name: "Invented", library_ref: { library_kind: "section", library_id: "seclib.invented.gone", library_version: "sha256:s", record_id: "sec.refs" } } }];
  return model;
}

function findings(model: PreviewModel): LibraryRefFinding[] {
  const material = model.materials![0];
  return [
    { code: "MODEL-LIBRARY-REF-VERSION-CHANGED", severity: "warning", entity_ref: material.id, library_kind: "material", library_id: "matlib.invented.refs",
      record_id: "mat.refs", recorded_version: "sha256:recorded", stored_version: "sha256:stored", message: "The stored record has changed." },
    { code: "MODEL-LIBRARY-REF-UNRESOLVED", severity: "warning", entity_ref: "section:refs", library_kind: "section", library_id: "seclib.invented.gone",
      record_id: "sec.refs", recorded_version: "sha256:s", stored_version: null, message: "The local store does not hold the library." }
  ];
}

describe("library reference warnings in the diagnostics list and the inspector", () => {
  it("lists both warnings keyed to their entities and shows the entity's own in its Provenance record section", async () => {
    const model = await referencedModel();
    render(<DiagnosticsPanel model={model} knowledge={null} result={null} selectedDiagnosticId={null} onSelectDiagnostic={() => {}} />);
    expect(screen.queryByTestId("diagnostic-MODEL-LIBRARY-REF-VERSION-CHANGED")).toBeNull();
    act(() => publishLibraryRefFindings(model.project.id, findings(model)));
    const changed = screen.getByTestId("diagnostic-MODEL-LIBRARY-REF-VERSION-CHANGED");
    expect(changed.className).toContain("warning");
    expect(screen.getByTestId("diagnostic-MODEL-LIBRARY-REF-UNRESOLVED").className).toContain("warning");
    cleanup();
    render(<ModelDataAnnotations model={model} selection={{ type: "material", id: model.materials![0].id }} onQueueIntent={() => {}} />);
    expect(screen.getByTestId("provenance-record-library-warning-MODEL-LIBRARY-REF-VERSION-CHANGED").textContent).toContain("Re-apply from library");
    expect(screen.queryByTestId("provenance-record-library-warning-MODEL-LIBRARY-REF-UNRESOLVED")).toBeNull();
  });

  it("hides a finding once the entity's recorded reference changes, and for another project", async () => {
    const model = await referencedModel();
    publishLibraryRefFindings(model.project.id, findings(model));
    expect(currentLibraryRefFindings(model).map((finding) => finding.entity_ref)).toEqual([model.materials![0].id, "section:refs"]);
    const reapplied = structuredClone(model);
    reapplied.materials![0].provenance_record!.library_ref!.library_version = "sha256:stored";
    expect(currentLibraryRefFindings(reapplied).map((finding) => finding.entity_ref)).toEqual(["section:refs"]);
    const other = structuredClone(model);
    other.project.id = "project:other";
    expect(currentLibraryRefFindings(other)).toEqual([]);
  });
});
