# Context menus on entities, tree rows and grid cells (B9)

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). A context
menu is another way to reach commands that already exist. It adds no model
operation and no view state. Opening a menu changes nothing in the model; an
item does exactly what its existing command does.

## Surfaces

One menu component (`features/workspace/contextMenu/ContextMenu.tsx`) and one
set of entity commands (`EntityContextMenu.tsx`) serve:

- **The viewport.** A label target, and the canvas. On the canvas a secondary
  press released within 4 CSS px of where it began opens the menu, so a
  right-drag still pans. The entity under the pointer is the target; with
  none, the primary selection is. jsdom has no WebGL, so the tests cover the
  canvas menu with no pick.
- **The model tree.** Entity rows. Group rows have no entity commands and open
  no menu.
- **The bulk grid** (pipes, supports, components and the other non-table
  families). A row, outside its text and choice fields, which keep the
  platform's own editing menu.
- **The engineering tables** (node, material and section fields and review
  drafts). Row headers and cells.

## Commands

| Item | Runs | Shortcut | Without the menu |
|---|---|---|---|
| Hide selected | the viewport's `hide-selection` view command | H (viewport only) | viewport toolbar Hide; H in the viewport |
| Isolate selected | `isolate-selection` | I (viewport only) | viewport toolbar Isolate; I in the viewport |
| Fit selection | `fit-selection` | none | viewport toolbar Fit Selection |
| Show all | `show-all` | none | viewport toolbar Show All; the hidden-count status button |
| Queue removal for review | the inspector's Delete node, pipe or support intent (`model-tree/removalIntents.ts`), or its Remove panel's material, section or component intent (`toolkit/GuardedRemoval.tsx`) | none | the inspector's Delete or Remove panel |
| Edit cell (tables) | the table's own edit start, as Enter does | Enter | Enter, typing, or a double-click on the cell |
| Sort ⟨column⟩ ascending / descending (tables) | the table's own sort state | none | the column header button (cycles ascending, descending, none) |
| Clear sort (tables) | the same sort state | none | the footer's "Sorted by … · Clear" button, or a third header click |

The tree and the grids reach the viewport commands through the viewport's own
dispatcher, which `App.tsx` already holds (`viewportViewCommandRef`). The
H and I shortcuts act only while focus is in the viewport, so the menu shows
them only there. A viewport command is unavailable while no viewport is
mounted. While the Table view is shown the viewport is mounted but not
displayed; Hide, Isolate and Show all still apply there, and the tree shows
the hidden state. The removal builders moved unchanged from
`PropertyInspector.tsx`; their `source_ref` still names the inspector, because
the intent is the same intent, whichever surface queues it.

Commands not offered because none exists yet: copying an identifier, listing
every entity of a kind, splitting a span, setting a slope and inserting before
an entity. They are not added here.

## Disabled items

A disabled item stays in the menu, focusable, with `aria-disabled` and its
reason, which is shown and set as its accessible description. The reasons are
the existing commands' own where they have one:

- Hide and Isolate: "Select at least one valid drawable node, pipe, support,
  or component." (the toolbar's).
- Fit selection: "No visible valid selected geometry can be fitted."
- Show all: "All eligible entities are already shown."
- No viewport: "The viewport is unavailable."
- Removal with more than one entity selected: the inspector disables its
  Delete panel then; the menu says "Removal queues one entity at a time;
  select only ⟨id⟩."
- Material, section and component removal: "Preparing the current entity
  snapshot…" until the snapshot is ready, a failure stated as the Remove panel
  states it, and "Another operation is running." while one is.
- Load cases and combinations: "Load cases and combinations are removed in the
  Load Cases panel." The project: "This entity has no removal command."
- Edit cell: the cell's own read-only reason, "Another operation is running.",
  or "Finish or cancel the current edit first."

## Selection rule

- Right-clicking an entity that is not selected selects it first, replacing
  the selection, as a click does. Right-clicking a selected entity keeps the
  selection, including a multiple selection.
- Opening a menu from the keyboard never changes the selection. The selection
  commands act on the selection. When the menu's entity is not in it, they are
  disabled with "This acts on the selection, which does not include ⟨id⟩.
  Select it first."
- A table cell's right-click also makes that cell the table's current cell.

## Keyboard

- Shift+F10 or the ContextMenu key opens the menu:
  - on the tree's current row;
  - on a focused table cell or row header;
  - on a bulk-grid row's ID button;
  - on a focused viewport label;
  - on the canvas, for the primary selection.
- `role="menu"` with `menuitem` children and separators. The first enabled
  item takes focus.
- ArrowDown and ArrowUp move and wrap, Home and End go to the ends, and a
  printable character moves to the next item starting with it. Disabled items
  are visited.
- Enter or Space runs the item. Escape, Tab, Shift+F10 or the ContextMenu key
  closes the menu.
- Running an item or closing from the keyboard returns focus to the element
  the menu was opened from: the tree list, the cell, the row header, the label
  or the canvas. The menu closes before the item runs, so an item that moves
  focus (Edit cell opens the editor) keeps it.
- A pointer press outside, a window resize or the window losing focus closes
  the menu and leaves focus where the user put it.
- While the menu is open, it takes every key, and none reaches the tree, the
  grid or the viewport. H typed in the menu does not hide.

## Evidence

- `features/workspace/contextMenu/ContextMenu.test.tsx`: menu semantics and the
  keyboard model.
- `features/workspace/contextMenu/EntityContextMenu.test.tsx`: the Remove-panel
  intents and the reasons.
- `features/model-tree/ModelTree.contextMenu.test.tsx`:
  - the tree, the bulk grid and the engineering tables;
  - routing to the viewport command, the inspector's intents and the table's
    edit and sort;
  - focus return and the selection rule.
- `features/viewport/PipeViewport.contextMenu.test.tsx`: labels and the canvas,
  and the toolbar's own Hide and Isolate results.
- `App.contextMenu.test.tsx`: through the shell, from the tree to the mounted
  viewport's Hide and Show all, and to the pending-changes queue.

With the surface wiring reverted, 22 of the 23 surface tests fail. The one
that passes checks that a group row opens no menu.

Browser and native checks of the menus remain open.
