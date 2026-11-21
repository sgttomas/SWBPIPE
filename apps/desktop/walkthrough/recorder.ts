// The walkthrough's recorder: one numbered picture per step, with what a person
// has in front of them at that moment (words, controls), written as one part
// file per journey. `compose-index.mjs` joins the parts into INDEX.json and
// index.html when the run ends.
import { mkdirSync, writeFileSync } from "node:fs";
import path from "node:path";
import type { Page } from "@playwright/test";
import { readScreenCounts, scrollLongestRegion as scrollRegion } from "./screen-counts.mjs";

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

/** The surface every browser journey's part file names (compose-index.mjs). */
export const SURFACE = "browser build";
/** The build every browser journey runs: the desktop's interface served by the development server, no native host. */
export const BUILD = "browser build: the desktop's interface from the development server, without the native host";
const REVISION = (() => { try { return JSON.parse(process.env.SWBPIPE_WALKTHROUGH_REVISION ?? "null"); } catch { return null; } })();

export const outputRoot = path.resolve(process.env.SWBPIPE_WALKTHROUGH_OUT || path.join(import.meta.dirname, "output"));

/** Reads the counts from the page as it is drawn now (screen-counts.mjs, shared with the native walkthrough). */
export async function readCounts(page: Page): Promise<ScreenCounts> {
  return page.evaluate(readScreenCounts);
}

/**
 * Scrolls the tallest scrolling region on screen by `screens` of its own
 * height. Returns false when nothing on screen scrolls further.
 */
export async function scrollLongestRegion(page: Page, screens: number, within?: string): Promise<boolean> {
  return page.evaluate(scrollRegion, { count: screens, scope: within ?? null });
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
    const size = this.page.viewportSize();
    const window = size ? `${size.width}x${size.height}` : null;
    writeFileSync(
      path.join(outputRoot, `part-${String(this.journeyNumber).padStart(2, "0")}-${this.name}.json`),
      `${JSON.stringify({ number: this.journeyNumber, name: this.name, title: this.title, use_case: this.title, revision: REVISION, build: BUILD, surface: SURFACE, window, screens: this.records }, null, 2)}\n`
    );
  }

  finish(): void {
    this.write();
  }
}
