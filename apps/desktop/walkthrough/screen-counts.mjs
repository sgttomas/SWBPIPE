// What a person has in front of them on one screen: the counts every walkthrough
// step records (README.md, "What it writes"). One source for both walkthroughs:
// the browser recorder passes this function to Playwright's page.evaluate, and the
// native runner writes its text into the driver it gives the launched app, which
// calls it in the real web view. It must therefore stay self-contained: no import,
// no reference to anything outside its own body, plain JavaScript.

/**
 * Reads the counts from the page as it is drawn now.
 * @returns {{ words_on_screen: number, words_in_page: number, prose_blocks_on_screen: number,
 *   buttons_on_screen: number, inputs_on_screen: number, longest_scroll_screens: number, controls: string[] }}
 */
export function readScreenCounts() {
  const viewport = { left: 0, top: 0, right: innerWidth, bottom: innerHeight };
  const meet = (a, b) => ({
    left: Math.max(a.left, b.left), top: Math.max(a.top, b.top),
    right: Math.min(a.right, b.right), bottom: Math.min(a.bottom, b.bottom)
  });
  const area = (box) => Math.max(0, box.right - box.left) * Math.max(0, box.bottom - box.top);
  // The part of the window an element's content can show in: the window, cut
  // down by every ancestor that clips its overflow.
  const clipCache = new Map();
  const clipOf = (element) => {
    if (!element || element === document.documentElement) return viewport;
    const cached = clipCache.get(element);
    if (cached) return cached;
    let box = clipOf(element.parentElement);
    const style = getComputedStyle(element);
    if (style.overflowX !== "visible" || style.overflowY !== "visible") box = meet(box, element.getBoundingClientRect());
    clipCache.set(element, box);
    return box;
  };
  const shown = (element) => element.checkVisibility({ checkOpacity: true, checkVisibilityCSS: true });
  // Covered: the topmost element at the middle of the visible part belongs to
  // another part of the page (a page over the workspace, a popover, a drawer).
  const uncovered = (element, box) => {
    const top = document.elementFromPoint((box.left + box.right) / 2, (box.top + box.bottom) / 2);
    return top !== null && (top === element || element.contains(top) || top.contains(element));
  };
  const onScreen = (element, rects) => {
    if (!shown(element)) return false;
    const clip = clipOf(element);
    for (const rect of rects) {
      const visible = meet(rect, clip);
      if (area(visible) >= 1 && uncovered(element, visible)) return true;
    }
    return false;
  };
  const countWords = (text) => (text.match(/[\p{L}\p{N}][\p{L}\p{N}'’._:/+-]*/gu) ?? []).length;

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

  const kindOf = (element) => {
    const role = element.getAttribute("role");
    if (role) return role;
    const tag = element.tagName.toLowerCase();
    if (tag === "input") return element.type || "input";
    if (tag === "a") return "link";
    if (tag === "summary") return "disclosure";
    return tag;
  };
  const nameOf = (element) => {
    const labelled = element.getAttribute("aria-labelledby");
    const fromIds = labelled ? labelled.split(/\s+/).map((id) => document.getElementById(id)?.textContent ?? "").join(" ") : "";
    const labels = element.labels;
    const fromLabels = labels && labels.length > 0 ? Array.from(labels).map((label) => label.textContent ?? "").join(" ") : "";
    const candidates = [
      element.getAttribute("aria-label"), fromIds, fromLabels,
      element.innerText, element.getAttribute("title"), element.getAttribute("placeholder")
    ];
    const name = candidates.find((candidate) => candidate && candidate.trim()) ?? "";
    return name.replace(/\s+/g, " ").trim().slice(0, 80);
  };
  const controls = [];
  let buttons = 0;
  let inputs = 0;
  const selector = "button, a[href], input:not([type=hidden]), select, textarea, summary, [role=button], [role=tab], [role=menuitem], [role=combobox], [role=checkbox], [role=switch], [role=option], [role=separator][tabindex]";
  for (const element of document.querySelectorAll(selector)) {
    if (!onScreen(element, [element.getBoundingClientRect()])) continue;
    const kind = kindOf(element);
    const entry = kind === "button" || kind === "tab" || kind === "menuitem" || kind === "link" || kind === "disclosure" || kind === "separator";
    if (entry) buttons += 1; else inputs += 1;
    const disabled = element.disabled === true || element.getAttribute("aria-disabled") === "true";
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
}

/**
 * Scrolls the tallest scrolling region on screen (or inside the element `scope`
 * names, a CSS selector) by `count` of its own heights. Returns false when
 * nothing on screen scrolls further. Self-contained, as readScreenCounts.
 */
export function scrollLongestRegion({ count, scope }) {
  let best = null;
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
}
