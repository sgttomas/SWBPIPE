import { describe, expect, it } from "vitest";
import {
  defaultUiPreferences,
  readUiPreferences,
  resolvedUiTheme,
  UI_PREFERENCES_STORAGE_KEY,
  updateUiPreferences,
  writeUiPreferences
} from "./uiPreferences";

function memoryStorage(initial: string | null = null) {
  let value = initial;
  return {
    getItem: (key: string) => key === UI_PREFERENCES_STORAGE_KEY ? value : null,
    setItem: (key: string, next: string) => {
      if (key === UI_PREFERENCES_STORAGE_KEY) value = next;
    },
    value: () => value
  };
}

describe("UI preferences", () => {
  it("falls back safely for corrupt and unknown versions", () => {
    expect(readUiPreferences(memoryStorage("{"))).toBe(defaultUiPreferences());
    expect(readUiPreferences(memoryStorage(JSON.stringify({ version: 2, theme: "dark" })))).toBe(
      defaultUiPreferences()
    );
  });

  it("persists only the versioned UI record and bounds layout sizes", () => {
    const storage = memoryStorage();
    const preferences = updateUiPreferences(defaultUiPreferences(), {
      theme: "dark",
      density: "compact",
      leftRailPx: 999,
      rightRailPx: 1,
      dockPx: 330,
      lastPanelTabs: { inspector: "properties" }
    });
    writeUiPreferences(preferences, storage);
    expect(readUiPreferences(storage)).toEqual({
      version: 1,
      theme: "dark",
      density: "compact",
      leftRailPx: 420,
      rightRailPx: 280,
      dockPx: 330,
      lastPanelTabs: { inspector: "properties" }
    });
    expect(JSON.parse(storage.value()!)).not.toHaveProperty("projectId");
  });

  it("follows system appearance only while System is selected", () => {
    expect(resolvedUiTheme("system", true)).toBe("dark");
    expect(resolvedUiTheme("system", false)).toBe("light");
    expect(resolvedUiTheme("light", true)).toBe("light");
    expect(resolvedUiTheme("dark", false)).toBe("dark");
  });

  it("treats storage access and write denial as a nonfatal UI preference failure", () => {
    const denied = {
      getItem: () => { throw new DOMException("denied"); },
      setItem: () => { throw new DOMException("quota"); }
    };
    expect(readUiPreferences(denied)).toBe(defaultUiPreferences());
    expect(() => writeUiPreferences(defaultUiPreferences(), denied)).not.toThrow();
  });
});
