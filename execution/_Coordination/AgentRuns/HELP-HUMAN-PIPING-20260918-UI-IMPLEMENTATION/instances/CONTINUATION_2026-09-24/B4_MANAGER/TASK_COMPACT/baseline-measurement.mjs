import { pathToFileURL } from 'node:url';
import path from 'node:path';
const { chromium } = await import(pathToFileURL(path.resolve('projects/chirality-piping/node_modules/playwright/index.mjs')).href);
import fs from 'node:fs';
const out=new URL('.', import.meta.url).pathname;
const browser=await chromium.launch({executablePath:'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',headless:true});
try {
const results=[];
for(const density of ['comfortable','compact']) for(const drawer of [180,280,500]) for(const view of ['model','both']) {
 const page=await browser.newPage({viewport:{width:1280,height:800}});
 await page.addInitScript(({density,drawer})=>localStorage.setItem('chirality.desktop.ui-preferences.v1',JSON.stringify({version:1,density,tableDrawerPx:drawer})),{density,drawer});
 await page.goto('http://127.0.0.1:5174');await page.getByTestId('workspace-toolbar').waitFor();
 await page.getByTestId(`view-switch-${view}`).click();
 const toggle=page.getByTestId('toggle-tree');if(await toggle.count() && await toggle.getAttribute('aria-expanded')!=='true') await toggle.click();
 await page.getByTestId('layout-mode-grid').click();
 for(const mode of ['direct','error','review']) {
 if(mode==='error') {await page.getByTestId('table-cell-node:N-100-y').dblclick();const input=page.getByRole('textbox',{name:'node:N-100 Y [m]'});await input.fill('invalid');await input.press('Enter');}
 if(mode==='review') {await page.getByRole('button',{name:'Cancel',exact:true}).click();await page.getByTestId('node-grid-review-disclosure').click();}
 const geometry=await page.evaluate(()=>Object.fromEntries(Object.entries({host:'.shell-tree-host',model:'.model-tree',title:'.model-tree>.panel-title',layout:'.layout-mode-toggle',filter:'.model-tree-controls',family:'.entity-grid-tabs',header:'.engineering-table:not([hidden]) .engineering-table-header',rows:'[data-testid="engineering-table-rows"]',footer:'.engineering-table-footer',error:'.engineering-table-message[role="alert"]',review:'.entity-grid-review-toggle',summary:'.entity-grid-summary',actions:'.entity-grid-actions',boundary:'.entity-grid-boundary'}).map(([name,sel])=>{const els=[...document.querySelectorAll(sel)];const e=els.find(e=>e.getBoundingClientRect().height>0);if(!e)return[name,null];const r=e.getBoundingClientRect();return[name,{x:r.x,y:r.y,width:r.width,height:r.height,bottom:r.bottom,scrollHeight:e.scrollHeight,clientHeight:e.clientHeight}]})));
 results.push({density,drawer,view,mode,geometry});
 }
 await page.close();
}
fs.writeFileSync(out+'/baseline-geometry.json',JSON.stringify({browser:browser.version(),results},null,2));console.log('Measured',results.length,'states');
} finally {await browser.close();}
