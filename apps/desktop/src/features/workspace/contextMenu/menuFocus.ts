import { canSurfaceRevealNow } from "../surfaceReveal";

// B9: where focus goes when a context menu closes. Two questions:
// - Tab and Shift+Tab: which element would the browser's own Tab have reached from the element
//   the menu was opened from (`adjacentTabStop`)?
// - after a command, or Escape: if the command left focus nowhere, where should it go
//   (`recoverMenuFocus`)?
// Scope: the product has no positive tabindex and no authored shadow roots, so the order here
// is document order over tabindex 0 stops, with the browser's exclusions, radio groups and
// iframes. A positive tabindex is treated as 0 and a shadow root's contents are not entered.

const TAB_STOP = [
  "a[href]", "area[href]", "button", "input:not([type=\"hidden\"])", "select", "textarea", "iframe", "summary",
  "[tabindex]", "[contenteditable]:not([contenteditable=\"false\"])"
].join(", ");

/** Disabled itself, or inside a disabled fieldset outside that fieldset's first legend. */
function disabled(element: HTMLElement): boolean {
  if ("disabled" in element && (element as HTMLButtonElement).disabled) return true;
  for (let fieldset = element.closest("fieldset"); fieldset; fieldset = fieldset.parentElement?.closest("fieldset") ?? null) {
    if (!fieldset.disabled) continue;
    const legend = fieldset.querySelector(":scope > legend");
    if (!legend?.contains(element)) return true;
  }
  return false;
}

/**
 * Connected, laid out (the reveal helper's test), not inert, not CSS-invisible and not
 * disabled: an element the browser would let take focus.
 */
export function canTakeFocus(element: Element | null | undefined): element is HTMLElement {
  if (!(element instanceof HTMLElement) || !element.isConnected || element.closest("[inert]")) return false;
  if (!canSurfaceRevealNow(element) || disabled(element)) return false;
  const visibility = getComputedStyle(element).visibility;
  return visibility !== "hidden" && visibility !== "collapse";
}

function tryFocus(element: HTMLElement): boolean {
  element.focus();
  return document.activeElement === element;
}

function radioGroup(radio: HTMLInputElement): HTMLInputElement[] {
  const scope: ParentNode = radio.form ?? radio.ownerDocument;
  return [...scope.querySelectorAll<HTMLInputElement>("input[type=\"radio\"]")].filter((candidate) =>
    candidate.name === radio.name && (candidate.form ?? null) === (radio.form ?? null));
}

/** A named radio group is one stop: its checked radio, or else its first that can take focus. */
function radioStop(radio: HTMLInputElement): boolean {
  if (!radio.name) return true;
  const group = radioGroup(radio).filter(canTakeFocus);
  const checked = group.find((candidate) => candidate.checked);
  return (checked ?? group[0]) === radio;
}

/** Every element the browser's Tab visits, in order, outside `exclude` (the menu). */
export function tabStops(exclude: Element | null = null): HTMLElement[] {
  return [...document.querySelectorAll<HTMLElement>(TAB_STOP)].filter((element) =>
    element.tabIndex >= 0 && !exclude?.contains(element) && canTakeFocus(element) &&
    !(element instanceof HTMLInputElement && element.type === "radio" && !radioStop(element)));
}

/**
 * The tab stop after (or before) `reference`, as Tab from it would reach. Past the last stop
 * Tab wraps to the first, and Shift+Tab before the first wraps to the last: the desktop window
 * has no browser chrome for focus to leave to. With no other stop, the reference itself.
 */
export function adjacentTabStop(reference: HTMLElement, backward: boolean, exclude: Element | null): HTMLElement | null {
  const stops = tabStops(exclude).filter((element) => element !== reference && !reference.contains(element));
  const follows = (element: HTMLElement) => Boolean(reference.compareDocumentPosition(element) & Node.DOCUMENT_POSITION_FOLLOWING);
  if (stops.length === 0) return canTakeFocus(reference) ? reference : null;
  if (backward) return [...stops].reverse().find((element) => !follows(element)) ?? stops[stops.length - 1];
  return stops.find(follows) ?? stops[0];
}

/** A region, section, tab panel, dialog or heading that takes programmatic focus: a place, not an action. */
const PLACE = "section[tabindex], [role=\"region\"][tabindex], [role=\"tabpanel\"][tabindex], [role=\"dialog\"][tabindex], h1[tabindex], h2[tabindex], h3[tabindex], h4[tabindex], h5[tabindex], h6[tabindex]";

export type MenuFocusPlan = Readonly<{
  /** The invoker, then the surface's current equivalents and designated stable places. */
  candidates: readonly HTMLElement[];
  /** The invoker's ancestors when the menu opened, nearest first. */
  ancestry: readonly HTMLElement[];
}>;

/**
 * Puts focus somewhere sensible after the menu closed and its command ran (or threw):
 * 1. a command that put focus on an element that can take it keeps it (an editor, a dialog);
 * 2. else the first candidate that can take focus: the invoker, then the surface's
 *    equivalents and designated stable places, which may use tabindex="-1";
 * 3. else, from the nearest still-connected element of the surface (a candidate, or an
 *    ancestor the invoker had when the menu opened), the nearest shown ancestor: itself if it
 *    is a place, else its first place (a section or region, such as the Pending changes page
 *    that replaces the tree after a removal is queued), else its first tab stop.
 * Elements with tabindex="-1" other than places and designated candidates are never chosen,
 * and places come before buttons, so a destructive button is not focused in place of a page.
 */
export function recoverMenuFocus(plan: MenuFocusPlan, exclude: Element | null = null): HTMLElement | null {
  const active = document.activeElement;
  if (active instanceof HTMLElement && active !== document.body && !exclude?.contains(active) && canTakeFocus(active)) return active;
  for (const element of plan.candidates) if (canTakeFocus(element) && tryFocus(element)) return element;
  const start = plan.candidates.find((element) => element.isConnected) ?? plan.ancestry.find((element) => element.isConnected) ?? null;
  for (let ancestor: HTMLElement | null = start; ancestor; ancestor = ancestor.parentElement) {
    if (!canSurfaceRevealNow(ancestor)) continue;
    if (ancestor.matches(PLACE) && canTakeFocus(ancestor) && tryFocus(ancestor)) return ancestor;
    for (const place of ancestor.querySelectorAll<HTMLElement>(PLACE)) {
      if (!exclude?.contains(place) && canTakeFocus(place) && tryFocus(place)) return place;
    }
    for (const stop of tabStops(exclude)) {
      if (ancestor.contains(stop) && tryFocus(stop)) return stop;
    }
  }
  return null;
}
