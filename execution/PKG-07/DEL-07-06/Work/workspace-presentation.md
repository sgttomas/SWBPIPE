# Workspace presentation conventions

Standard claim fence applies (F-PIP-2; claims taxonomy per DEC-081). This is the shared
presentation layer of the desktop workspace: type, spacing, control size, state colours, panes,
tables and the shell's chrome. It changes how existing controls and records look. It adds,
removes, enables or disables no control, forms no result or status, and changes no numerical
value, persistence format, IPC command or permission. It is not a WCAG conformance result and
not independent practitioner validation; both stay held (Scope of Work, D-68 and PDU-045/046).
Paths are relative to the repository root.

## 1. Basis and scope

The owner judged the desktop a walking skeleton rather than a professional engineering
application and directed a coherent first increment toward the hill-climb goal in `AGENTS.md`
(on par with or better than CAEPIPE in modelling and interface). This account serves RT-U1/U2 and
RT-G2/R1. D-68 selects WCAG 2.2 AA criteria for the controls this touches: visible focus,
keyboard alternatives, 4.5:1 text contrast, 3:1 graphical contrast and the 24 px minimum target.
D-72 performance acceptance stays open and this work does not supply it.

Everything here is built from the maintained design tokens (`apps/desktop/src/design/tokens.json`,
version 1.2, generated into `apps/desktop/src/tokens.css`). No token was added or changed; when one
is needed, it comes from a later design-system version through `TOKENS_SOURCE.md`.

The sources are `apps/desktop/src/styles.css` (the generic control, table and panel rules and the
section headed "Workstation presentation conventions"),
`apps/desktop/src/features/workspace/shell/ShellStatusBar.tsx` and `StageRail.tsx`, the results
actions group in `apps/desktop/src/App.tsx`, and the standing-line layout in
`apps/desktop/src/features/results/ResultsPanel.tsx`.

## 2. The conventions

Every pane uses one vocabulary, so slices built by different lanes read as one product.

| Concern | Convention |
|---|---|
| Type | One scale in both densities: caption 11/14 for eyebrows, captions and identifiers; body 13/18 for everything read, the repeatedly-read minimum; title 15/20 for a pane's section heading. Weights 400, 500 and 600. The token system UI font; figures are tabular so columns of values align. |
| Space | 4 px inside a control group, 8 px between controls, 12 px pane padding, 16 px between sections. |
| Size | Controls are 28 px in comfortable density and 24 px in compact. Compact never goes below the 24 px target. Rows keep the geometry of their own table, such as the model tree's 34/28 px virtual rows. |
| State | Hover is the hover wash over the control's own fill. Pressed is the pressed fill and ink pair, with no layout-shifting bar. A selected row is the selection band plus the 3 px selection bar. Focus is the 2 px ring. Disabled is the disabled fill with its reason reachable. |
| Boundaries | Command and field boundaries keep the 3:1 border that the resolved-style check enforces. Text-identified items such as menu-bar triggers may be borderless. |
| Panes | A pane's eyebrow title is caption size, muted, in spaced capitals. Section headings use the title and body sizes. Fieldsets are hairline cards. Explanatory prose keeps a readable measure; tables and grids are not limited. |
| Tables | Body-size cells with tabular figures. A caption-size header on the header surface whose words never break mid-word. Hairline rows with the alternate-row tint and a hover wash. Padding is set by density. A value stays on one line with its unit; identifiers may wrap at their separators. Component tables keep their own column rules. |

## 3. What the increment changes, and why

Findings come from the running candidate with the invented demo model
(`fixtures/product_preview/invented_demo_model.json` and its invented reference results), in
light and dark themes, comfortable and compact density, at 1440 × 900 and 1100 × 720.

- **Type and controls.** The shell used 14 px Inter and 32/28 px controls, outside the token
  scale, and every button drew a 3 px left bar when pressed, shifting its label. The shell now uses
  the token type and the token pressed pair. Controls are 28/24 px.
- **Menu and toolbar.** The in-window menu bar was drawn as a row of boxed buttons, because the
  generic button rule outranked the menu's own transparent style. It now reads as a menu bar. The
  toolbar's groups are separated, the project name is the heading, and Run is the one primary
  command.
- **Canvas header.** The canvas commands wrapped inside a fixed 43 px scroll box, so their third
  row showed cut in half or as empty boxes. They now sit in one row that scrolls sideways. Every
  command stays reachable by Tab and by scrolling. Canvas identity labels had a fixed 58 px width
  and showed `N-1…`. They now size to their identifier, up to 120 px.
- **Truthful state colour.** The stage rail drew every caption, including Historical, in the
  failure colour. Each caption now takes the colour of its kind (failed, stale or historical) from
  the existing rail tokens.
- **Results hierarchy.** The two results commands were full-width bars. They are now one group of
  ordinary commands, "Results actions". The standing line ran its route's published limitations
  together as a block of about 25 lines before the first number. It now sets out the headline,
  each limitation on its own line and the closing sentence, in a tinted basis block. The text is
  exactly the sentence it was, every limitation stays visible, and the DEL-07-05 rule that the
  standing line states each route's limitations is kept.
- **Tables.** Rows were 37 px with browser-default headers, values broke mid-digit, and header
  words broke mid-word. The dense table primitive in section 2 replaces them.
- **Smaller repairs.** Disclosure summaries meet the 24 px target. Inline labels keep a gap before
  their field. The status bar sets the selection's type as a quiet label and its identifier in
  mono, with unchanged text.

## 4. Findings for later increments

These are recorded, not changed, because they belong to a feature's own Design or need a decision
beyond presentation:

- The browser solve-failure panel (DEL-07-07 `solve-progress-panel.md`) states one error three
  times (headline, alert and event list) and shows internal tokens such as the progress-basis name.
  Its event history is truthful, so de-duplicating it is that panel's own decision.
- The load-cases summary shows raw state tokens (`unit posture=…; saved_project_mutated=false`).
  Plain wording needs that panel's owner to confirm the meaning.
- Long authoring pages, such as the pressure-region and condition forms, are single scrolling
  columns of prose and controls. They need a form-layout primitive (label column, grouped fields,
  sticky actions) that the panels then adopt one by one.
- The results standing line is now scannable but still long on routes with many limitations. A
  result-first layout that keeps every limitation visible would need DEL-07-05's agreement.
- The model tree and property inspector ergonomics are a separate lane that uses these conventions.

## 5. Verification and limits

Each candidate is checked by:

- focused component tests for the edited paths, including the exact-text standing-line test and
  the dead-control audit, which requires a stated reason on every disabled control;
- the desktop type-check and production build;
- visual inspection of the running candidate in a browser over the maintained Vite route and
  Playwright's pinned Chromium, at both themes, both densities and a small window, with a
  recorded keyboard focus order and no page-level horizontal overflow.

A browser solve truthfully fails because it needs the native backend, so browser results are the
bundled reference, shown as Historical. These checks are not native measurements, a D-72 settled
frame demonstration, whole-product WCAG conformance or independent user validation.
