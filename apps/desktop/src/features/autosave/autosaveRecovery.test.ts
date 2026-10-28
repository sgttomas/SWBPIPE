// The recovery offer's pairing rule (DEL-02-05 Design `autosave-and-backup.md`
// §3): offered with no stored project, or when written after the last save
// with a model that differs; the stored project newer is said, never offered.
import { describe, expect, it } from "vitest";
import { classifyRecords, recordStanding, type StoredProjectInfo } from "./autosaveRecovery";
import type { AutosaveEntry } from "./autosaveService";

const SAVED = 1_760_000_000;
const stored: StoredProjectInfo = { project_id: "project:invented-p", project_name: "Invented P", updated_at_unix: SAVED, model_hash: null };
const at = (millis: number, hash = "sha256:" + "a".repeat(64), slot = "s".repeat(32)): AutosaveEntry =>
  ({ file_name: `${slot}-${millis}.json`, slot, written_unix_millis: millis, model_hash: hash, byte_count: 1 });

describe("recordStanding", () => {
  it("orders a record against its stored project's last save, by the second", () => {
    expect(recordStanding(at(SAVED * 1000 + 1000), stored)).toBe("autosave-newer");
    expect(recordStanding(at(SAVED * 1000 + 999), stored)).toBe("same-second");
    expect(recordStanding(at(SAVED * 1000), stored)).toBe("same-second");
    expect(recordStanding(at(SAVED * 1000 - 1), stored)).toBe("stored-newer");
    expect(recordStanding(at(SAVED * 1000 + 5000), null)).toBe("no-stored");
    expect(recordStanding(at(SAVED * 1000 + 5000), { ...stored, model_hash: "sha256:" + "a".repeat(64) })).toBe("same-model");
    expect(recordStanding(at(SAVED * 1000 + 5000), { ...stored, model_hash: "sha256:" + "b".repeat(64) })).toBe("autosave-newer");
  });

  it("offers newer, same-second and unpaired records, reports older ones and drops the stored model's own", () => {
    const slot = "s".repeat(32);
    const otherSlot = "t".repeat(32);
    const newer = at(SAVED * 1000 + 2000);
    const older = at(SAVED * 1000 - 2000);
    const unpaired = at(1, "sha256:" + "c".repeat(64), otherSlot);
    const same = at(SAVED * 1000 + 3000, "sha256:" + "e".repeat(64));
    const { offers, olderThanStored } = classifyRecords([same, newer, older, unpaired], new Map([[slot, { ...stored, model_hash: "sha256:" + "e".repeat(64) }]]));
    expect(offers.map((item) => [item.entry, item.standing])).toEqual([[newer, "autosave-newer"], [unpaired, "no-stored"]]);
    expect(olderThanStored.map((item) => item.entry)).toEqual([older]);
  });
});
