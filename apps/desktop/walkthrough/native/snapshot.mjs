// The native walkthrough's snapshot of the document as drawn (README.md, "The
// launched app"). The runner writes this function's text into the driver it gives
// the launched app, which calls it in the real web view at each step; it must stay
// self-contained (no import, nothing from outside its own body, plain JavaScript).
//
// It reads; it changes nothing the person would see, with one exception: a resize
// event is dispatched so that the 3D view draws a frame, and the canvases are read
// in an animation frame callback registered after it, while the drawing buffer is
// still valid. What it returns, with the page's own stylesheets inlined as text, is
// rendered later in Chromium (render.mjs), never in the app.

/**
 * @returns {Promise<{ html: string, width: number, height: number, device_pixel_ratio: number, media: object,
 *   canvases: Array<{ label: string, captured: boolean, reason?: string, width: number, height: number }>,
 *   view_3d: "captured" | "not captured" | "not on screen", notes: string[] }>}
 */
export async function takeSnapshot() {
  const notes = [];
  const VIEW_3D = '[data-testid="viewport-canvas"]';

  // The canvases' pixels, read in one animation frame after a redraw. WebKit gives no frame to a window it judges
  // hidden or covered; then each canvas is read as it stands, which for the 3D view is the last frame presented,
  // or nothing, and the blank test below says which.
  const canvases = [...document.querySelectorAll("canvas")];
  const pixels = new Map();
  let frame = "none needed";
  const readAll = (how) => {
    for (const canvas of canvases) {
      try {
        pixels.set(canvas, { src: canvas.toDataURL("image/png"), how });
      } catch (error) {
        pixels.set(canvas, { src: null, reason: `its pixels cannot be read (${error?.name ?? error})` });
      }
    }
  };
  if (canvases.length > 0) {
    window.dispatchEvent(new Event("resize"));
    frame = await new Promise((resolve) => {
      let done = false;
      requestAnimationFrame(() => { if (!done) { done = true; readAll("in an animation frame after a redraw"); resolve("given"); } });
      setTimeout(() => { if (!done) { done = true; readAll("outside an animation frame (none was given)"); resolve("not given"); } }, 1500);
    });
    if (frame === "not given") notes.push(`no animation frame was given within 1.5 s (page visibility ${document.visibilityState}, focus ${document.hasFocus() ? "held" : "not held"}); canvases were read as they stood`);
  }
  // Blank: every sampled pixel the same (a cleared buffer reads as one colour).
  const blank = (src) => new Promise((resolve) => {
    const image = new Image();
    image.onload = () => {
      try {
        const probe = document.createElement("canvas");
        probe.width = 48;
        probe.height = 48;
        const context = probe.getContext("2d");
        context.drawImage(image, 0, 0, 48, 48);
        const data = context.getImageData(0, 0, 48, 48).data;
        for (let index = 4; index < data.length; index += 4) {
          if (data[index] !== data[0] || data[index + 1] !== data[1] || data[index + 2] !== data[2] || data[index + 3] !== data[3]) return resolve(false);
        }
        resolve(true);
      } catch {
        resolve(true);
      }
    };
    image.onerror = () => resolve(true);
    image.src = src;
  });
  const canvasRecords = [];
  for (const canvas of canvases) {
    const rect = canvas.getBoundingClientRect();
    const label = canvas.closest(VIEW_3D) ? "3D view" : (canvas.getAttribute("aria-label") || canvas.closest("[data-testid]")?.getAttribute("data-testid") || "canvas");
    const read = pixels.get(canvas) ?? { src: null, reason: "it was not read" };
    const record = { label, captured: false, width: Math.round(rect.width), height: Math.round(rect.height) };
    if (rect.width < 1 || rect.height < 1) record.reason = "not drawn (no size)";
    else if (!read.src) record.reason = read.reason;
    else if (await blank(read.src)) record.reason = `its pixels read blank, ${read.how}`;
    else { record.captured = true; record.read = read.how; read.ok = true; }
    canvasRecords.push(record);
  }
  const shown3d = canvasRecords.filter((record) => record.label === "3D view" && record.width > 0 && record.height > 0);
  const view3d = shown3d.length === 0 ? "not on screen" : shown3d.every((record) => record.captured) ? "captured" : "not captured";

  // The document with the state the markup does not carry written into it.
  const clone = document.documentElement.cloneNode(true);
  const originals = [document.documentElement, ...document.documentElement.querySelectorAll("*")];
  const copies = [clone, ...clone.querySelectorAll("*")];
  if (originals.length !== copies.length) notes.push("the copy of the document did not match it element for element; form state may be missing");
  const replacements = [];
  const images = [];
  for (let index = 0; index < Math.min(originals.length, copies.length); index += 1) {
    const element = originals[index];
    const copy = copies[index];
    const tag = element.tagName.toLowerCase();
    if (element.scrollTop || element.scrollLeft) copy.setAttribute("data-walk-scroll", `${element.scrollTop},${element.scrollLeft}`);
    if (tag === "input") {
      const type = (element.type || "").toLowerCase();
      if (type === "checkbox" || type === "radio") {
        if (element.checked) copy.setAttribute("checked", ""); else copy.removeAttribute("checked");
        if (element.indeterminate) copy.setAttribute("data-walk-indeterminate", "");
      } else if (type !== "file" && type !== "password") {
        copy.setAttribute("value", element.value);
      }
    } else if (tag === "textarea") {
      copy.textContent = element.value;
    } else if (tag === "select") {
      Array.from(element.options).forEach((option, at) => {
        const copied = copy.options?.[at];
        if (!copied) return;
        if (option.selected) copied.setAttribute("selected", ""); else copied.removeAttribute("selected");
      });
    } else if (tag === "dialog") {
      if (element.open && element.matches(":modal")) copy.setAttribute("data-walk-modal", "");
    } else if (tag === "canvas") {
      replacements.push([element, copy]);
    } else if (tag === "img" && element.currentSrc && !element.currentSrc.startsWith("data:")) {
      images.push([element, copy]);
    } else if (tag === "iframe" && !element.hasAttribute("srcdoc")) {
      try {
        const inner = element.contentDocument?.documentElement?.outerHTML;
        if (inner) copy.setAttribute("srcdoc", inner);
      } catch { notes.push("an iframe's document could not be read"); }
      copy.removeAttribute("src");
    } else if (tag === "script" || tag === "noscript") {
      copy.setAttribute("data-walk-remove", "");
    } else if (tag === "link") {
      copy.setAttribute("data-walk-remove", "");
    } else if (tag === "style") {
      copy.setAttribute("data-walk-remove", "");
    }
    try {
      if (element.hasAttribute("popover") && element.matches(":popover-open")) copy.setAttribute("data-walk-popover-open", "");
    } catch { /* a WebKit without :popover-open */ }
  }

  // Each canvas becomes an image of its pixels at its drawn size, or a labelled placeholder.
  canvasRecords.forEach((record, at) => {
    const [element, copy] = replacements[at] ?? [];
    if (!element) return;
    const read = pixels.get(element);
    const rect = element.getBoundingClientRect();
    const style = getComputedStyle(element);
    const size = `width:${rect.width}px;height:${rect.height}px;display:${style.display === "inline" ? "inline-block" : style.display};`;
    let replacement;
    if (read?.ok) {
      replacement = document.createElement("img");
      replacement.setAttribute("src", read.src);
      replacement.setAttribute("alt", record.label);
    } else {
      replacement = document.createElement("div");
      replacement.setAttribute("data-walk-placeholder", "");
      replacement.textContent = `${record.label}: not captured (${record.reason})`;
    }
    for (const attribute of Array.from(copy.attributes)) if (attribute.name !== "width" && attribute.name !== "height") replacement.setAttribute(attribute.name, attribute.value);
    const placeholder = read?.ok ? "" : "box-sizing:border-box;border:2px dashed #b04a1a;background:#f4ece6;color:#6d2a0c;font:600 14px/1.4 system-ui,sans-serif;padding:12px;overflow:hidden;";
    replacement.setAttribute("style", `${copy.getAttribute("style") ?? ""};${size}${placeholder}`);
    copy.replaceWith(replacement);
  });

  // Pictures the page loaded from the app's own origin, as data.
  const asData = (url) => fetch(url).then((response) => response.blob()).then((blob) => new Promise((resolve, reject) => {
    const reader = new FileReader();
    reader.onload = () => resolve(reader.result);
    reader.onerror = () => reject(reader.error);
    reader.readAsDataURL(blob);
  }));
  for (const [element, copy] of images) {
    try { copy.setAttribute("src", await asData(element.currentSrc)); copy.removeAttribute("srcset"); }
    catch { notes.push(`a picture (${element.currentSrc.split("/").pop()}) could not be read`); }
  }

  // The page's stylesheets, in order, as text; url() made absolute (the redraw loads nothing from them).
  const cssParts = [];
  const sheetText = (sheet) => {
    let rules;
    try { rules = sheet.cssRules; } catch { notes.push(`a stylesheet could not be read (${sheet.href ?? "inline"})`); return ""; }
    const base = sheet.href ?? document.baseURI;
    return Array.from(rules, (rule) => rule.cssText).join("\n").replace(/url\((['"]?)([^'")]+)\1\)/g, (whole, quote, url) => {
      if (/^(data:|#)/.test(url)) return whole;
      try { return `url(${quote}${new URL(url, base).href}${quote})`; } catch { return whole; }
    });
  };
  for (const sheet of Array.from(document.styleSheets)) if (!sheet.disabled) cssParts.push(sheetText(sheet));
  for (const sheet of document.adoptedStyleSheets ?? []) cssParts.push(sheetText(sheet));
  for (const removed of clone.querySelectorAll("[data-walk-remove]")) removed.remove();
  let head = clone.querySelector("head");
  if (!head) { head = document.createElement("head"); clone.prepend(head); }
  const style = document.createElement("style");
  style.setAttribute("data-walk-styles", "");
  style.textContent = cssParts.join("\n");
  head.append(style);
  const charset = document.createElement("meta");
  charset.setAttribute("charset", "utf-8");
  head.prepend(charset);
  clone.setAttribute("data-walk-window-scroll", `${window.scrollY},${window.scrollX}`);
  if (document.activeElement && document.activeElement !== document.body) {
    const at = originals.indexOf(document.activeElement);
    if (at >= 0 && copies[at]?.isConnected !== false) copies[at].setAttribute("data-walk-focused", "");
  }

  return {
    html: `<!doctype html>\n${clone.outerHTML}`,
    width: innerWidth,
    height: innerHeight,
    device_pixel_ratio: devicePixelRatio,
    // The preferences the page's media queries read, so the redraw answers them as the web view did.
    media: {
      color_scheme: matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light",
      reduced_motion: matchMedia("(prefers-reduced-motion: reduce)").matches ? "reduce" : "no-preference",
      forced_colors: matchMedia("(forced-colors: active)").matches ? "active" : "none"
    },
    canvases: canvasRecords,
    view_3d: view3d,
    notes
  };
}
