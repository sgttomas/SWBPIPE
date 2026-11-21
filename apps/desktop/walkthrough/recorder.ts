// The walkthrough's recorder: one numbered picture per step, with what a person
// has in front of them at that moment (words, controls), written as one part
// file per journey. `compose-index.mjs` joins the parts into INDEX.json and
// index.html when the run ends.
import { mkdirSync, writeFileSync } from "node:fs";
import path from "node:path";
import type { Page } from "@playwright/test";

export type ScreenCounts = {
  /** Words a person can read without scrolling: in the window, not clipped, not covered. */
  words_on_screen: number;
  /** Words in the page's whole text, off-screen and scrolled-away text included. */
  words_in_page: number;
  /** Paragraphs and list items of prose at least partly on screen. */
  prose_blocks_on_screen: number;
  buttons_on_screen: number;
  inputs_on_screen: number;
  /** How many window-heights the tallest scrolling region on screen runs to. */
  longest_scroll_screens: number;
  /** Each control on screen, as "kind: name", with "(disabled)" where it cannot act. */
  controls: string[];
};

export type ScreenRecord = {
  number: number;
  journey: string;
  step: string;
  note: string;
  file?: string;
  failed?: string;
  not_reached?: boolean;
} & Partial<ScreenCounts>;

export const outputRoot = path.resolve(process.env.SWBPIPE_WALKTHROUGH_OUT || path.join(import.meta.dirname, "output"));

/** Reads the counts from the page as it is drawn now. */
export async function readCounts(page: Page): Promise<ScreenCounts> {
  return page.evaluate(() => {
    const viewport = { left: 0, top: 0, right: innerWidth, bottom: innerHeight };
    type Box = { left: number; top: number; right: number; bottom: number };
    const meet = (a: Box, b: Box): Box => ({
      left: Math.max(a.left, b.left), top: Math.max(a.top, b.top),
      right: Math.min(a.right, b.right), bottom: Math.min(a.bottom, b.bottom)
    });
    const area = (box: Box) => Math.max(0, box.right - box.left) * Math.max(0, box.bottom - box.top);
    // The part of the window an element's content can show in: the window, cut
    // down by every ancestor that clips its overflow.
    const clipCache = new Map<Element, Box>();
    const clipOf = (element: Element | null): Box => {
      if (!element || element === document.documentElement) return viewport;
      const cached = clipCache.get(element);
      if (cached) return cached;
      let box = clipOf(element.parentElement);
      const style = getComputedStyle(element);
      if (style.overflowX !== "visible" || style.overflowY !== "visible") box = meet(box, element.getBoundingClientRect());
      clipCache.set(element, box);
      return box;
    };
    const shown = (element: Element) =>
      element.checkVisibility({ checkOpacity: true, checkVisibilityCSS: true });
    // Covered: the topmost element at the middle of the visible part belongs to
    // another part of the page (a page over the workspace, a popover, a drawer).
    const uncovered = (element: Element, box: Box) => {
      const top = document.elementFromPoint((box.left + box.right) / 2, (box.top + box.bottom) / 2);
      return top !== null && (top === element || element.contains(top) || top.contains(element));
    };
    const onScreen = (element: Element, rects: Iterable<DOMRect>): boolean => {
      if (!shown(element)) return false;
      const clip = clipOf(element);
      for (const rect of rects) {
        const visible = meet(rect, clip);
        if (area(visible) >= 1 && uncovered(element, visible)) return true;
      }
      return false;
    };
    const countWords = (text: string) => (text.match(/[\p{L}\p{N}][\p{L}\p{N}'’._:/+-]*/gu) ?? []).length;

    let wordsOnScreen = 0;
    const walker = document.createTreeWalker(document.body, NodeFilter.SHOW_TEXT);
    const range = document.createRange();
    for (let node = walker.nextNode(); node; node = walker.nextNode()) {
      const parent = node.parentElement;
      const text = node.nodeValue ?? "";
      if (!parent || !text.trim() || parent.closest("script, style, noscript, option")) continue;
      range.selectNodeContents(node);
      if (onScreen(parent, range.getClientRects())) wordsOnScreen += countWords(text);
    }

    const kindOf = (element: Element): string => {
      const role = element.getAttribute("role");
      if (role) return role;
      const tag = element.tagName.toLowerCase();
      if (tag === "input") return (element as HTMLInputElement).type || "input";
      if (tag === "a") return "link";
      if (tag === "summary") return "disclosure";
      return tag;
    };
    const nameOf = (element: Element): string => {
      const labelled = element.getAttribute("aria-labelledby");
      const fromIds = labelled ? labelled.split(/\s+/).map((id) => document.getElementById(id)?.textContent ?? "").join(" ") : "";
      const labels = (element as HTMLInputElement).labels;
      const fromLabels = labels && labels.length > 0 ? Array.from(labels).map((label) => label.textContent ?? "").join(" ") : "";
      const candidates = [
        element.getAttribute("aria-label"), fromIds, fromLabels,
        (element as HTMLElement).innerText, element.getAttribute("title"), element.getAttribute("placeholder")
      ];
      const name = candidates.find((candidate) => candidate && candidate.trim()) ?? "";
      return name.replace(/\s+/g, " ").trim().slice(0, 80);
    };
    const controls: string[] = [];
    let buttons = 0;
    let inputs = 0;
    const selector = "button, a[href], input:not([type=hidden]), select, textarea, summary, [role=button], [role=tab], [role=menuitem], [role=combobox], [role=checkbox], [role=switch], [role=option], [role=separator][tabindex]";
    for (const element of document.querySelectorAll(selector)) {
      if (!onScreen(element, [element.getBoundingClientRect()])) continue;
      const kind = kindOf(element);
      const entry = kind === "button" || kind === "tab" || kind === "menuitem" || kind === "link" || kind === "disclosure" || kind === "separator";
      if (entry) buttons += 1; else inputs += 1;
      const disabled = (element as HTMLButtonElement).disabled === true || element.getAttribute("aria-disabled") === "true";
      controls.push(`${kind}: ${nameOf(element) || "(no name)"}${disabled ? " (disabled)" : ""}`);
    }

    let proseBlocks = 0;
    for (const element of document.querySelectorAll("p, li, dd, blockquote")) {
      if ((element.textContent ?? "").trim() && onScreen(element, element.getClientRects())) proseBlocks += 1;
    }

    let longest = 1;
    for (const element of document.querySelectorAll("*")) {
      if (element.scrollHeight <= element.clientHeight + 8 || element.clientHeight < 120) continue;
      const overflow = getComputedStyle(element).overflowY;
      if (overflow !== "auto" && overflow !== "scroll") continue;
      if (!onScreen(element, [element.getBoundingClientRect()])) continue;
      longest = Math.max(longest, element.scrollHeight / innerHeight);
    }

    return {
      words_on_screen: wordsOnScreen,
      words_in_page: countWords(document.body.innerText),
      prose_blocks_on_screen: proseBlocks,
      buttons_on_screen: buttons,
      inputs_on_screen: inputs,
      longest_scroll_screens: Math.round(longest * 10) / 10,
      controls
    };
  });
}

/**
 * Scrolls the tallest scrolling region on screen by `screens` of its own
 * height. Returns false when nothing on screen scrolls further.
 */
export async function scrollLongestRegion(page: Page, screens: number, within?: string): Promise<boolean> {
  return page.evaluate(({ count, scope }) => {
    let best: Element | null = null;
    const root = scope ? document.querySelector(scope) : document.body;
    if (!root) return false;
    for (const element of [root, ...root.querySelectorAll("*")]) {
      if (element.scrollHeight <= element.clientHeight + 8 || element.clientHeight < 120) continue;
      const overflow = getComputedStyle(element).overflowY;
      if (overflow !== "auto" && overflow !== "scroll") continue;
      if (!element.checkVisibility()) continue;
      const rect = element.getBoundingClientRect();
      const top = document.elementFromPoint(rect.left + rect.width / 2, rect.top + rect.height / 2);
      if (!top || !(element === top || element.contains(top))) continue;
      if (!best || element.scrollHeight - element.clientHeight > best.scrollHeight - best.clientHeight) best = element;
    }
    if (!best) return false;
    const before = best.scrollTop;
    best.scrollTop = before + best.clientHeight * count * 0.9;
    return best.scrollTop > before + 4;
  }, { count: screens, scope: within ?? null });
}

/** One journey's screens. Numbers are `journeyNumber * 100 + step`, so they sort in the order a person meets them. */
export class Journey {
  private readonly records: ScreenRecord[] = [];
  private broken: string | null = null;

  constructor(private readonly page: Page, private readonly journeyNumber: number, readonly name: string, readonly title: string) {
    mkdirSync(outputRoot, { recursive: true });
  }

  private next(): number {
    return this.journeyNumber * 100 + this.records.length + 1;
  }

  /**
   * Does `action`, then takes the picture. A step that fails is recorded with
   * its reason, and the rest of the journey is recorded as not reached: a
   * person stopped there too.
   */
  async step(step: string, note: string, action?: () => Promise<void | string>): Promise<void> {
    const number = this.next();
    if (this.broken) {
      this.records.push({ number, journey: this.name, step, note, not_reached: true });
      return;
    }
    try {
      // An action may return what it found (a disabled control's stated reason), which joins the note.
      const found = action ? await action() : undefined;
      if (typeof found === "string") note = `${note}: ${found}`;
      // Let transitions and the canvas settle before the picture.
      await this.page.waitForTimeout(350);
      const file = `${String(number).padStart(3, "0")}-${this.name}-${step}.png`;
      await this.page.screenshot({ path: path.join(outputRoot, file), animations: "disabled" });
      this.records.push({ number, journey: this.name, step, note, file, ...(await readCounts(this.page)) });
    } catch (error) {
      const reason = (error instanceof Error ? error.message : String(error)).split("\n").slice(0, 3).join(" ").slice(0, 400);
      this.broken = step;
      let file: string | undefined = `${String(number).padStart(3, "0")}-${this.name}-${step}-FAILED.png`;
      try { await this.page.screenshot({ path: path.join(outputRoot, file), animations: "disabled" }); } catch { file = undefined; }
      this.records.push({ number, journey: this.name, step, note, file, failed: reason });
    }
    this.write();
  }

  /**
   * Steps that only apply where the page scrolls further: scroll, picture, up
   * to `screens` times. `within` is a CSS selector for the part of the screen
   * the person is reading (a drawer, the inspector) when it is not the tallest.
   */
  async scrolled(step: string, note: string, screens = 2, within?: string): Promise<void> {
    for (let index = 1; index <= screens; index += 1) {
      if (this.broken || !(await scrollLongestRegion(this.page, 1, within))) return;
      await this.step(`${step}-scroll${index}`, `${note}, scrolled ${index} screen${index === 1 ? "" : "s"} down`);
    }
  }

  get failedStep(): string | null {
    return this.broken;
  }

  private write(): void {
    writeFileSync(
      path.join(outputRoot, `part-${String(this.journeyNumber).padStart(2, "0")}-${this.name}.json`),
      `${JSON.stringify({ number: this.journeyNumber, name: this.name, title: this.title, screens: this.records }, null, 2)}\n`
    );
  }

  finish(): void {
    this.write();
  }
}
