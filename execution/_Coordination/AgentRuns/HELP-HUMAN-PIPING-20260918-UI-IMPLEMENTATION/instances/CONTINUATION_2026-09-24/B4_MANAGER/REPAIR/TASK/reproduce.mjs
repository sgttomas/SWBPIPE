import { pathToFileURL } from 'node:url';
import path from 'node:path';
import fs from 'node:fs';
const { chromium } = await import(pathToFileURL(path.resolve('projects/chirality-piping/node_modules/playwright/index.mjs')).href);
const browser = await chromium.launch({executablePath:'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
const results=[];
try {
for(const traversal of ['Tab','Shift+Tab']) {
 const page=await browser.newPage({viewport:{width:1024,height:768}});await page.goto('http://127.0.0.1:5174');await page.getByTestId('workspace-toolbar').waitFor();await page.getByTestId('view-switch-both').click();
 if(await page.getByTestId('toggle-tree').getAttribute('aria-expanded')!=='true')await page.getByTestId('toggle-tree').click();
 await page.getByTestId('layout-mode-grid').click();await page.getByRole('button',{name:'Table details',exact:true}).click();await page.locator('.compact-table-details:popover-open').waitFor();
 await page.keyboard.press(traversal);
 const before=await page.evaluate(()=>({focus:document.activeElement?.outerHTML,popover:!!document.querySelector('.compact-table-details:popover-open'),familyName:document.querySelector('[data-testid="entity-grid-family"]')?.getAttribute('aria-label')}));
 await page.keyboard.press('Escape');await page.waitForTimeout(100);
 const after=await page.evaluate(()=>({focus:document.activeElement?.outerHTML,popover:!!document.querySelector('.compact-table-details:popover-open'),drawer:document.querySelector('[data-testid="toggle-tree"]')?.getAttribute('aria-expanded')}));
 results.push({traversal,before,after});await page.close();
}
fs.writeFileSync(new URL('./_run_records/reproduction.json',import.meta.url),JSON.stringify({browser:browser.version(),results},null,2));console.log(JSON.stringify(results,null,2));
}finally{await browser.close();}
