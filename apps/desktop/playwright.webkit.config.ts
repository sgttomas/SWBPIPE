import { defineConfig, devices } from "@playwright/test";
import base from "./playwright.config";

// Opt-in WebKit source lane. The packaged macOS app renders in WKWebView
// (Safari's engine); the registered source projects run Chromium only.
// Playwright's WebKit build is close to, but not identical with, WKWebView,
// so a pass here is browser evidence, never a native WKWebView witness.
// Validated on macOS only.
//
// A separate config keeps `playwright.config.ts` unchanged: the DEC-025 sweep
// runs that file's projects without a --project filter, DEC-093 binds its
// viewport projects exactly, and tools/ci/e2e_plan.py collects them. Adding a
// project there would change those gates. After `npm run build:wasm` and
// `npx playwright install webkit`, run
// `npx playwright test --config playwright.webkit.config.ts` from this folder.
//
// Own port and no server reuse, so a concurrent Chromium lane on 5174 (or
// another checkout's dev server) is never mistaken for this candidate. The
// lanes share `test-results/`, which each run clears, so run one lane at a
// time per checkout.
const port = 5176;

export default defineConfig({
  ...base,
  use: {
    baseURL: `http://127.0.0.1:${port}`,
    trace: "retain-on-failure"
  },
  webServer: {
    command: `npm run dev -- --port ${port} --strictPort`,
    reuseExistingServer: false,
    timeout: 30_000,
    url: `http://127.0.0.1:${port}`
  },
  projects: [
    // Packaged window default (1440x920, src-tauri/tauri.conf.json) with
    // Desktop Safari's device scale factor 2, as on a Retina Mac.
    {
      name: "webkit-desktop",
      // Chromium by construction (13 tests): the classic-scrollbar cases
      // launch their own Chromium (a WebKit pass there would be false
      // evidence), the causal method contract throws unless the browser is
      // Chromium, and six B3 cases query Chromium's accessibility tree over CDP.
      grepInvert: new RegExp([
        "B4 classic scrollbar ",
        "observer setup failure still invokes the original RAF callback",
        "covered pages exclude retained stage controls from Tab and accessibility",
        "keyboard page Close restores visible focus and retained state: ",
        "compact routing selector owns popup Escape before "
      ].join("|")),
      use: {
        ...devices["Desktop Safari"],
        viewport: { width: 1440, height: 920 }
      }
    }
  ]
});
