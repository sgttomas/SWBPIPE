export const UI_PREFERENCES_STORAGE_KEY = "chirality.desktop.ui-preferences.v1";
export const UI_PREFERENCES_VERSION = 1 as const;

export type UiThemePreference = "system" | "light" | "dark";
export type UiDensityPreference = "comfortable" | "compact";

export type UiPreferences = Readonly<{
  version: typeof UI_PREFERENCES_VERSION;
  theme: UiThemePreference;
  density: UiDensityPreference;
  leftRailPx: number;
  rightRailPx: number;
  dockPx: number;
  lastPanelTabs: Readonly<Record<string, string>>;
}>;

const DEFAULTS: UiPreferences = freezePreferences({
  version: UI_PREFERENCES_VERSION,
  theme: "system",
  density: "comfortable",
  leftRailPx: 280,
  rightRailPx: 340,
  dockPx: 260,
  lastPanelTabs: {}
});

export function defaultUiPreferences(): UiPreferences {
  return DEFAULTS;
}

export function readUiPreferences(storage?: Pick<Storage, "getItem">): UiPreferences {
  try {
    const target = storage ?? window.localStorage;
    const text = target.getItem(UI_PREFERENCES_STORAGE_KEY);
    if (!text) return DEFAULTS;
    const value: unknown = JSON.parse(text);
    if (!isRecord(value) || value.version !== UI_PREFERENCES_VERSION) return DEFAULTS;
    return freezePreferences({
      version: UI_PREFERENCES_VERSION,
      theme: isTheme(value.theme) ? value.theme : DEFAULTS.theme,
      density: isDensity(value.density) ? value.density : DEFAULTS.density,
      leftRailPx: boundedNumber(value.leftRailPx, 220, 420, DEFAULTS.leftRailPx),
      rightRailPx: boundedNumber(value.rightRailPx, 280, 520, DEFAULTS.rightRailPx),
      dockPx: boundedNumber(value.dockPx, 180, 600, DEFAULTS.dockPx),
      lastPanelTabs: stringRecord(value.lastPanelTabs)
    });
  } catch {
    return DEFAULTS;
  }
}

export function writeUiPreferences(
  preferences: UiPreferences,
  storage?: Pick<Storage, "setItem">
): void {
  try {
    const target = storage ?? window.localStorage;
    target.setItem(UI_PREFERENCES_STORAGE_KEY, JSON.stringify(preferences));
  } catch {
    // UI preferences are best-effort; storage denial must not interrupt authoring.
  }
}

export function updateUiPreferences(
  current: UiPreferences,
  patch: Partial<Omit<UiPreferences, "version">>
): UiPreferences {
  return freezePreferences({
    ...current,
    ...patch,
    version: UI_PREFERENCES_VERSION,
    leftRailPx: boundedNumber(patch.leftRailPx ?? current.leftRailPx, 220, 420, DEFAULTS.leftRailPx),
    rightRailPx: boundedNumber(patch.rightRailPx ?? current.rightRailPx, 280, 520, DEFAULTS.rightRailPx),
    dockPx: boundedNumber(patch.dockPx ?? current.dockPx, 180, 600, DEFAULTS.dockPx),
    lastPanelTabs: patch.lastPanelTabs ?? current.lastPanelTabs
  });
}

export function resolvedUiTheme(preference: UiThemePreference, systemDark: boolean): "light" | "dark" {
  return preference === "system" ? (systemDark ? "dark" : "light") : preference;
}

function freezePreferences(value: UiPreferences): UiPreferences {
  return Object.freeze({
    ...value,
    lastPanelTabs: Object.freeze({ ...value.lastPanelTabs })
  });
}

function boundedNumber(value: unknown, min: number, max: number, fallback: number): number {
  return typeof value === "number" && Number.isFinite(value)
    ? Math.min(max, Math.max(min, Math.round(value)))
    : fallback;
}

function isTheme(value: unknown): value is UiThemePreference {
  return value === "system" || value === "light" || value === "dark";
}

function isDensity(value: unknown): value is UiDensityPreference {
  return value === "comfortable" || value === "compact";
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object" && !Array.isArray(value);
}

function stringRecord(value: unknown): Readonly<Record<string, string>> {
  if (!isRecord(value)) return Object.freeze({});
  return Object.freeze(Object.fromEntries(
    Object.entries(value).filter((entry): entry is [string, string] => typeof entry[1] === "string")
  ));
}
