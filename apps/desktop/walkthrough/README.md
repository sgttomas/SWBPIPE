# Screenshot walkthrough

Drives the desktop through an engineer's journeys by its own visible controls
and takes a numbered picture at each step, with what is on screen at that
moment. It exists so that a change to the interface is judged from screens of
the application in use.

There are two halves. The browser walkthrough (below) drives the browser build
and takes real screenshots of it. The native walkthrough ("The launched app")
launches the real desktop app, solves, and records the screens from Run to a
saved report, which the browser build cannot reach. Both write the same output
form, and can share one output folder: browser journeys are numbered 1 to 19,
native ones from 20, and each run replaces only its own journeys' files.

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
window, and takes about two minutes. It first removes an earlier browser
walkthrough's files from the output folder, keeps the native journeys' files,
and refuses a folder that holds anything else.

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

## The launched app

```sh
npm run walkthrough:native --workspace apps/desktop -- --build-only
npm run walkthrough:native --workspace apps/desktop -- [--journey <names>] [--out <folder>]
```

Run each under a host slot, as separate jobs, as the native-smoke runners are
run. `--build-only` builds and binds the `native-smoke` build of the app, the
same build and binding `native-smoke/evidence-journeys.mjs` uses, and launches
nothing. A launch invocation builds nothing: anything stale refuses it by name.

| Journey | No. | What a person meets |
| --- | --- | --- |
| `solve-and-results` | 20 | The bundled demonstration model: Run, the progress, the Results stage and each of its panels scrolled to its foot, the filter to a support's rows, a row chosen and its place in the viewport, Key results and each of its views, the Evidence tab, the Analyze page, the Issues drawer. |
| `report` | 21 | Solved, then the Review stage, the Report tab, "Load for…" where offered, the composer, Render (local private), the preview, Save Shareable Report Package… to its release dialog and Cancel, Save Report Package… with its chooser answered inside the throwaway home, the outcome list. Print is not pressed. |
| `exports` | 22 | Solved, then the Exports page, top to foot. |
| `reference-system` | 23 | Named alone, for its long solve: File > Open Model Document… of the reference system fixture, Run, the standing, the health line, Results, Report. |
| `prepared-project` | 24 | A model document placed in the throwaway home before launch, opened by File > Open Model Document…: the start other journeys build on. |

Without `--journey` the first three run, one launch each. Files are in
`native/`: `run.mjs` (the command), `driver.js` (the steps, in the app),
`snapshot.mjs` (the record of the document) and `render.mjs` (the pictures).
The counts are the browser walkthrough's own (`screen-counts.mjs`, one source
for both), computed in the launched app's web view.

**How the pictures are made.** Capturing the window from outside needs a
permission this command does not ask for, so at each step the driver records,
in the real web view, the document as drawn: its markup, the page's stylesheets
inlined as text, each field's value and each box's ticked or chosen state,
open disclosures, dialogs and popovers, scroll positions, the window's inner
size, and each canvas as an image of its pixels. The 3D view is read in an
animation frame after a redraw; where it reads blank, a labelled placeholder
stands in its place and the index says the 3D view was not captured at that
step (`view_3d`). After the app has ended, Chromium renders each record at the
recorded size and takes the picture; it loads nothing.

So a native picture is the launched app's document redrawn by Chromium, not a
capture of its window. Fonts and form controls may differ in detail; hover,
focus rings, the cursor and the system menu bar are not shown. Every native
screen in `INDEX.json` and `index.html` says so (`picture`), and each journey
states its surface.

**How it is confined.** Each launch is a native-smoke launch, with the
native-smoke runners' own functions (`native-smoke/launch-env.mjs`,
`judged-launch.mjs`, `marks-check.mjs`): an environment built from nothing, a
new throwaway home (HOME and CFFIXED_USER_HOME) with TMPDIR inside it, removed
after its census; the strict chooser, which ends the app at a chooser with no
answer queued inside that home before any system dialog; a non-persistent web
view store; a private copy of the bound app; the marks check before each start;
the census of the private copy after each launch; the tripwire over the real
user's product-named Library paths; and the run's close on every exit it can
catch. The print dialog is replaced by a record in this build, and Print is not
pressed. The embedded agent is never started: no provider, model, key, network
or sign-in. The run's folder (reports, step records, snapshots) is kept in the
system's temporary folder; the command prints where.

**What it cannot show.** The 3D view, as a rule: on this host the launched
web view reports its page as hidden (the window is not the one in front), so
it gives no animation frame and the viewport, which pauses while its page is
hidden, draws nothing to read. Each such step says so in its `snapshot_notes`,
and the picture shows the placeholder. Bringing the window to the front is not
something this command does. Also not shown: the system menu bar and native dialogs (the choosers
are answered by the harness, the release dialog is the app's own), the
embedded agent, printing, and anything outside the window. A step is driven by
test identifiers, as in the browser walkthrough, so it finds a control a person
might not.

## Adding or changing a journey

Browser journeys are in `journeys.walk.ts`, one Playwright test each, in the order of
their numbers. A step is `journey.step(name, note, action)`: the action uses the
application's own controls, then the picture is taken. `journey.scrolled(...)`
adds pictures of what lies below the fold. Keep step names stable so that two
walkthroughs can be compared; add a step with a new name instead of renaming one.
Native journeys: see the next section.

## Adding a native journey that starts from a prepared project

A native journey can begin from files placed in the launch's throwaway home
before the app starts: a model document, or a local project store with its
saved states and runs. To add one:

1. **Commit the files** under `fixtures/` (invented data only). A model
   document is a JSON file the app opens; a local project store is the app's
   `openpipestress-projects.sqlite3`.
2. **Declare the journey** in `native/run.mjs`, `JOURNEYS`: a name of
   lower-case words joined by `-`, an unused `number` from 25 to 99, a
   `title` that names the use case, and:
   - `place`: one entry per file, `{ key, at, from }`. `at` is a path inside
     the home; `chooser/models/<file>` for a document the chooser returns, or
     `{app-data}/openpipestress-projects.sqlite3` for a store, which is the
     app's own data folder in that home. `from` is the committed path under
     `fixtures/`. A path outside the home, a source outside `fixtures/`, a
     link or a missing file is refused before anything launches.
   - `open`: `{ by: "model-document", file: "<key>" }` (File > Open Model
     Document…, the chooser's answer the placed file), or
     `{ by: "local-project", project_id: "<id>" }` (File > Open Local
     Project…, from the placed store).
   - Optionally `timeouts: { solve: <ms> }` and `launch_ms` for a long solve.
3. **Write its steps** in `native/driver.js` as an `async function (journey,
   plan)` and name it in the driver's `JOURNEYS` under the same name. Start
   with `await openPrepared(journey, plan, "<what it is>")`, which records the
   step `opened`. Then add steps:
   `await journey.step("<stable-name>", "<what the person sees>", async () => { ... })`.
   Inside a step, act as a person does: `press("<test id>")` for a visible
   control, `openStage("results")` for a stage, `showSection("exports")` for a
   page, and `menu("<command id>")` for a native menu command. Open a section
   or a form by its own visible control before using what is in it. A control
   that is missing, hidden or disabled stops the
   journey there, with the reason the window gives. Return a string to add
   what the window said to the step's note. Use
   `journey.scrolled("<name>", "<note>", <screens>)` for what lies below the
   fold.
4. **Run it alone**, after a build-only run at the same revision, each under a
   host slot:
   `npm run walkthrough:native --workspace apps/desktop -- --journey <name> --out <folder>`.
   Its screens join any already in that folder; only its own earlier screens
   are replaced.
