# Screenshot walkthrough

Drives the desktop through an engineer's journeys by its own visible controls
and takes a numbered picture at each step, with what is on screen at that
moment. It exists so that a change to the interface is judged from screens of
the application in use.

It is evidence of what a person sees. It is not a test: it asserts nothing
about whether a screen is right, and a clean run does not show that a use case
of DEL-07-06 AC-007 completes.

## Run it

From the repository root, after `npm ci` and `npm run build:wasm --workspace apps/desktop`:

```sh
npm run walkthrough --workspace apps/desktop
npm run walkthrough --workspace apps/desktop -- --out <folder> --size 1280x800 --grep loads
```

| Option | Meaning | Default |
| --- | --- | --- |
| `--out` | Where the screens and the index are written. | `apps/desktop/walkthrough/output` (not committed) |
| `--size` | The window, as `WIDTHxHEIGHT`. | `1440x920`, the product's default window |
| `--grep` | Only journeys whose title contains the text. | every journey |

The command starts its own development server on port 5176 and one Chromium
window, and takes about two minutes. It first removes an earlier walkthrough's
files from the output folder, and refuses a folder that holds anything else.

To compare before and after a change, run it once on each revision into two
folders and compare the two `INDEX.json` files screen by screen: step names
are stable, so the same step in both is the same moment of the journey.

## What it writes

- `NNN-<journey>-<step>.png`: one picture per step. The number is the journey's
  number times 100 plus the step, so files sort in the order a person meets them.
- `INDEX.json`: every step with its note and counts, and for each journey where
  it stopped and which steps were not reached.
- `index.html`: the same, for reading: every picture with its counts and the
  list of controls on screen.

| Count | What it is |
| --- | --- |
| `words_on_screen` | Words a person can read without scrolling: inside the window, not clipped by a scrolling region, not covered by a page, drawer or popover. |
| `words_in_page` | Words in the page's whole text, including text below the fold. |
| `prose_blocks_on_screen` | Paragraphs and list items at least partly on screen. |
| `buttons_on_screen` | Buttons, tabs, menu items, links and disclosures on screen. |
| `inputs_on_screen` | Fields, selectors and checkboxes on screen. |
| `longest_scroll_screens` | How many window-heights the tallest scrolling region on screen runs to. |
| `controls` | Each control on screen as `kind: name`, marked `(disabled)` where it cannot act. |

## When a journey stops

A step whose control is missing, hidden or disabled is recorded with the reason
(a disabled control's own stated reason where it gives one), a picture of the
screen at that moment, and every later step of that journey as not reached.
That is a finding about the application or about the journey's script, and the
picture usually says which. The command still exits 0; it exits 1 only when it
could not run or wrote no screens.

## What it cannot show

This is the browser build, not the launched application.

- The browser build cannot solve a model. Run is refused, so the Results and
  Review stages stay disabled, and the only results shown are the bundled
  reference results, which are not a solve of the model on screen.
- Native surfaces are absent: the system menu bar (the browser build draws its
  own menu row instead), file choosers, report save and print, autosave and
  recovery, and the embedded agent with a provider.
- A step is driven by test identifiers and accessible names, so it finds a
  control a person might not. A journey completing here does not show that a
  person would find the way.

## Adding or changing a journey

Journeys are in `journeys.walk.ts`, one Playwright test each, in the order of
their numbers. A step is `journey.step(name, note, action)`: the action uses the
application's own controls, then the picture is taken. `journey.scrolled(...)`
adds pictures of what lies below the fold. Keep step names stable so that two
walkthroughs can be compared; add a step with a new name instead of renaming one.
