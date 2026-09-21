const { chromium } = require(require.resolve('@playwright/test', { paths: [process.cwd()] }));
(async () => {
  const browser = await chromium.launch({executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH});
  const page = await browser.newPage(); const results = [];
  for (const move of [false, true]) {
    await page.setContent('<div id="rows"><div id="active"><input value="50"></div><div id="other">other</div></div>');
    const input = page.locator('input'); await input.focus(); await input.press('End'); await input.press('ArrowLeft'); await input.press('3');
    const original = await input.elementHandle();
    if (move) await page.evaluate(() => { const input = document.querySelector('input'); const start=input.selectionStart,end=input.selectionEnd; document.querySelector('#rows').append(document.querySelector('#active')); input.focus(); input.setSelectionRange(start,end); });
    const before = await input.inputValue(); await input.press('ControlOrMeta+z');
    results.push({move,before,after:await input.inputValue(),same:await input.evaluate((node,original)=>node===original, original)});
  }
  console.log(JSON.stringify({version:browser.version(),platform:process.platform,results},null,2)); await browser.close();
})();
