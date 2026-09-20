# Actual Tauri tooltip/Close action ledger

Manager WORKING_ITEMS Astra/high, CUA only for UI. Source3861b6143, uninstrumented
binarye9eda3cca4e626dd00cfee62279113d0d5161a751ba40cd58cea091cd23ca5f9.
FreshPID80466/start Sun Sep20 02:31:31 2026, exact wt3 app path in NATIVE_LAUNCH.json.
No prior desktop executable process. Native Rust/config unchanged. Initial synthetic
project:invented-loop-01,27 entities, Both, Inspector closed, Select, empty history.
No create/open/save/delete/apply/model input/preferences changes performed.

Exact CUA actions, with fresh getAXState after each numbered action sequence:
1. getApp(explicit wt3 SWBPIPE.app path); initial AX observed title Invented Utility
   Loop Preview — SWBPIPE. Saved full native-initial-ax.txt.
2. click(33 Libraries). Tab→Rules; six Tabs→Refresh local list; one Tab→Agent154.
   Visible reason192. Saved native-agent-focused-ax.txt and native-agent-focused.png:
   tooltip below Agent, page Close visually clear.
3. Attempted click([1143,118]) from screenshot visual location to place pointer on
   reason. Focus became HTML and reason disappeared. Full observation saved as
   native-reason-pointer-ax.txt/png. This DOES NOT prove native hover persistence.
   No coordinate scaling or host dimensions inferred from resampled screenshot.
4. Tab→Issues61; ShiftTab→Agent58, reason75 visible. Ordinary click(40 Close) from
   current AX. Libraries closed, Modeling workspace restored, focus Libraries33;
   native-pointer-close-ax.txt. No force-click or UI script.
5. click(33 Libraries), eight Tabs→Agent154, reason192 visible. Escape: reason text
   removed visually, page Close40 still present, focus Agent58; saved native-first-
   escape-ax.txt/png. Accessible Agent help still reads Agent: not available yet.
6. Escape again: Libraries closed, Modeling workspace restored, Select12 focused;
   native-second-escape-ax.txt. Hidden reasons did not intercept normal page close.
7. click(33 Libraries), eight Tabs→Agent154, reason192 visible. click(192 tooltip text)
   again moved focus to HTML and hid reason. This is another unsuccessful native
   hover attempt, not evidence of hover retention or a product regression.
8. Tab then ShiftTab→Agent154, reason192 visible again. Ordinary click(175 Close)
   closes page and returns focus Libraries33. native-final-ax.txt/png retains final
   initial synthetic project, Both, empty Undo/Redo, no engineering changes.

Every app.click used documented CUA and observed AX IDs or the noted screenshot
coordinate. Native evidence demonstrates focus-visible reason, ordinary Close action,
Escape-only reason dismissal, reentry and later page close. CUA native API exposes
no hover primitive; do not substitute a click for hover proof. Browser pinned-Chromium
regressions separately establish actual pointer hover across the gap and tooltip,
including topmost hit ownership and page Close. No native-class browser shot is
claimed as Tauri. No performance, dimensions, contrast or usability qualification.
