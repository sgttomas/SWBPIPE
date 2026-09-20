# ROOT native look and text Undo backcheck

2026-09-20, approximately 04:49–05:02 UTC. ROOT HELP_HUMAN, Astra/high-or-greater
allocation; actual UI through mcp__cua_repl only. Native/CUA ownership was handed
from B3 manager to ROOT and released back after this check. No other native tester
ran concurrently; independent fidelity review was headless on a separate resource.

Uninstrumented product commit `2882acab94120cfa6c3c115ddb416a368cebd9cd`.
Process30560, started Sat Sep19 22:47:08 2026. Executable path verified by ps:
`swbpipe-wt3/projects/chirality-piping/apps/desktop/src-tauri/target/debug/bundle/macos/SWBPIPE.app/Contents/MacOS/openpipestress-desktop`.
Executable SHA256, independently checked by ROOT:
`1836d91ddbc5dd4809e53b12dde92c87fab587b8e9709e19fc920ec2e780af3a`.
Manager's RESPONDERS_BUILD_MANIFEST binds source/config/dist. Concurrent selector
work was source-only; this bundle was unchanged. An initial hash command using
MacOS/SWBPIPE failed because the executable retains the legacy binary name; the
corrected path above was taken from the actual process, not guessed as proof.

## Text actions and observations

Inherited disposable project `project:blank-local-20260920t044213z`, blank model,
two displayed entities, no model history. All typing below used actual pressKey
calls, not value injection. AX observations followed each listed action group.

| Focus/history | Real actions | Observed result |
|---|---|---|
| Filter model tree | click49; a; b; c; observe | Value abc; separate native suggestion list Abc |
| Same focused filter | super+z | Empty field; model Undo/Redo remained disabled |
| Empty filter | w; i; t; super+z in one action group | Empty field on first post-action observation |
| Node tool, new node label | click224; observe; click255; a; b; c; super+z | Empty label; no model operation |
| Fresh New Blank, label | native File→New Blank; click119; a; b; c; observe | abc plus native Abc suggestion |
| Same label | super+z | Empty label |
| Empty label | a; b; c; space; observe; super+z; observe | abc-space, then empty label |
| Restored Both, empty filter | click137; x; super+z; observe | Empty filter |
| Same filter | super+shift+z; observe; super+z; observe | x, then empty filter |

CUA key actions, with observation calls abbreviated, and returned AX excerpts
for the original single-character case:

```javascript
await swb.click(137);
await swb.pressKey('x');
await swb.pressKey('super+z');
await swb.getAXState();
await swb.pressKey('super+shift+z');
await swb.getAXState();
await swb.pressKey('super+z');
await swb.getAXState();
```

```text
The focused UI element is 137 search text field (settable) Filter model tree, Placeholder: Name, ID, type, or group
~ 137 search text field (settable) Filter model tree, Value: x, Placeholder: Name, ID, type, or group
The focused UI element is 137 search text field (settable) Filter model tree, Value: x, Placeholder: Name, ID, type, or group
~ 137 search text field (settable) Filter model tree, Placeholder: Name, ID, type, or group
The focused UI element is 137 search text field (settable) Filter model tree, Placeholder: Name, ID, type, or group
```

The excerpts above omit indentation only; the live tool calls used emit:false
and filtered the returned AX lines for the field, history controls and focus to
bound transcript volume. They are not a complete raw AX tree export. Full earlier
trees and the actual native screenshots remain in the active tool conversation.

## Native appearance, drawer and observation limits

ROOT inspected actual Both/routing shell in dark System appearance and Light,
then restored System. Title was Blank Local Model — SWBPIPE. Existing native
selects appeared as popup buttons; no compact-selector proof is claimed here.
Model view kept its inspector docked. ROOT expanded/collapsed the table drawer
and visually checked the scale, painted orientation frame and readout clear of
the overlay strip. Colour/contrast and fine spacing remain closing-pass work.
No exact native dimensions or D-72 claim is derived from these screenshots.

One immediate native AX observation demonstrably lagged a selection: after the
theme menu's System item was clicked, AX still reported Light. The next Space
opened that same selector with System selected; Return and the subsequent
AX/screenshot showed System/dark. This is direct evidence that an immediate
AX-only old value is not by itself proof an action failed. No arbitrary delay
or state injection was added. A transient cgWindowNotFound after drawer collapse
was resolved by refreshing the app binding and its exposed Raise action; process
and binary stayed unchanged, and the resulting AX/screenshot showed the collapse.

## N1 disposition proposed for independent backcheck

The original f5 single-character text Undo failure is no longer present in the
tested repaired native sequence; native Redo also restores that exact character.
Manager's uninstrumented witness separately covers model history, pointer commands,
text exhaustion and populated text-history interactions. Its first-accelerator
observations remain unchanged in PROD_2882_WITNESS.md; passing repeats do not erase
them. That record's later two separate redo transactions abc→Abc demonstrate a
native autocorrection layer. It is plausible that the initial apparently unchanged
value reflects correction/grouping or AX timing, but this record does not prove
which occurred in each earlier observation. One native Undo transaction is not
necessarily one whole-field clear. No new product Undo interception is justified
by the present evidence; independent review should assess this distinction.

## Store and release state

ROOT's File→New Blank persisted one additional disposable blank row:
`project:blank-local-20260920t045253z`. No model edit, explicit save, delete or user
project access occurred. End state: same PID/binary, System/Comfortable, Both,
inspector closed, routing disarmed, filter empty, Model drawer restored expanded,
no model history. Native/CUA released to manager; later rebuilding is permitted.
Future compact selectors and successful-New-Blank navigation repair still require
their own native and independent backchecks.

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081).
