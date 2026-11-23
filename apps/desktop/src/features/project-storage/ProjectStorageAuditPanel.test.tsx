// #547: the Project Storage Audit's Project snapshot line. The open's summary counts the result
// and run it opened (`object_count` of the returned envelope); when the open leaves the stored run
// in the store unopened, those members are not what the store holds, so they leave the line and
// it says so in words. The summary is the native open's reply for a refused stored run
// (`native_reopen_replies.json`). All data are invented.
import { fireEvent, render, screen } from "@testing-library/react";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import type { LocalProjectSummary, PreviewModel } from "../../types";
import { ProjectStorageAuditPanel, STORED_RUN_NOT_OPENED } from "./ProjectStorageAuditPanel";

const root = resolve(__dirname, "../../../../../");
const MODEL = JSON.parse(readFileSync(resolve(root, "fixtures/product_preview/invented_preview_model.json"), "utf8")) as PreviewModel;
const REFUSED_OPEN_SUMMARY = {
  project_id: MODEL.project.id, project_name: MODEL.project.name, database_path: "$STORE", storage_mode: "local_sqlite",
  migration_status: "current_store_schema_v14_no_pending_migrations", migration_framework: "versioned_sqlite_user_version_migration_ledger",
  store_schema_version: 14, store_schema_target_version: 14, migrations_applied_on_open: [], fts_indexed: false, copied_external_files: false,
  editor_intent_count: 0, proposal_count: 0, selected_review_target_count: 0, selected_review_target_ref: "not_selected",
  persisted_mechanics_result_count: 0, persisted_analysis_run_count: 0, persisted_analysis_run_ref: "not_persisted",
  persisted_model_hash_count: 1, persisted_model_hash_ref: `sha256:${"e".repeat(64)}`, persisted_project_envelope_hash_count: 1,
  persisted_project_envelope_hash_ref: `sha256:${"d".repeat(64)}`, unit_round_trip_status: "unit_metadata_preserved_in_local_project_envelope",
  unit_round_trip_checked_ref_count: 0, unit_round_trip_signature: "", message: "Opened local SQLite project snapshot."
} satisfies LocalProjectSummary;

function snapshotLine(summary: LocalProjectSummary, storedRunNotOpened?: boolean): string {
  render(
    <ProjectStorageAuditPanel
      model={MODEL} storageCapability={null} projectSummary={summary} projectIndex={null} projectMessage="" projectOperation="open_by_id"
      editorIntents={[]} proposal={null} storedRunNotOpened={storedRunNotOpened}
    />
  );
  return screen.getByTestId("project-storage-snapshot").textContent ?? "";
}

describe("the Project snapshot line (#547)", () => {
  it("states no result or run count as persisted when the open left the stored run unopened, and says the run is kept", () => {
    const line = snapshotLine(REFUSED_OPEN_SUMMARY, true);
    expect(STORED_RUN_NOT_OPENED).toBe("A stored run is kept and was not opened");
    expect(line).toContain(`persisted_review_target_ref=not_selected; ${STORED_RUN_NOT_OPENED}; copied_external_files=false`);
    expect(line).not.toMatch(/persisted_mechanics_results|persisted_analysis_runs|persisted_analysis_run_ref/);
  });

  it("keeps the counts when the open opened what the store holds", () => {
    const opened = { ...REFUSED_OPEN_SUMMARY, persisted_mechanics_result_count: 1, persisted_analysis_run_count: 1, persisted_analysis_run_ref: "run:preview-linear-static-001" };
    const line = snapshotLine(opened);
    expect(line).toContain("persisted_mechanics_results=1; persisted_analysis_runs=1; persisted_analysis_run_ref=run:preview-linear-static-001; copied_external_files=false");
    expect(line).not.toContain(STORED_RUN_NOT_OPENED);
  });
});

describe("Local storage JSON export project_message", () => {
  it("appends projectMessageDetails to project_message when set and keeps projectMessage unchanged when null", () => {
    const { rerender } = render(
      <ProjectStorageAuditPanel
        model={MODEL} storageCapability={null} projectSummary={REFUSED_OPEN_SUMMARY} projectIndex={null}
        projectMessage="The current solve's result is not retained. This run record is too large."
        projectMessageDetails="RESULT-DETAIL-WEBVIEW-LIMIT: run record 67108865 bytes; limit 67108864 bytes."
        projectOperation="open_by_id" editorIntents={[]} proposal={null}
      />
    );
    fireEvent.click(screen.getByTestId("project-storage-export-link-local-private-intent"));
    const withDetails = JSON.parse(decodeURIComponent(screen.getByTestId("project-storage-export-link").getAttribute("href")!.split(",", 2)[1])) as { project_message: string };
    expect(withDetails.project_message).toBe(
      "The current solve's result is not retained. This run record is too large. RESULT-DETAIL-WEBVIEW-LIMIT: run record 67108865 bytes; limit 67108864 bytes."
    );

    rerender(
      <ProjectStorageAuditPanel
        model={MODEL} storageCapability={null} projectSummary={REFUSED_OPEN_SUMMARY} projectIndex={null}
        projectMessage="Opened local SQLite project snapshot."
        projectMessageDetails={null}
        projectOperation="open_by_id" editorIntents={[]} proposal={null}
      />
    );
    const plain = JSON.parse(decodeURIComponent(screen.getByTestId("project-storage-export-link").getAttribute("href")!.split(",", 2)[1])) as { project_message: string };
    expect(plain.project_message).toBe("Opened local SQLite project snapshot.");
  });
});
