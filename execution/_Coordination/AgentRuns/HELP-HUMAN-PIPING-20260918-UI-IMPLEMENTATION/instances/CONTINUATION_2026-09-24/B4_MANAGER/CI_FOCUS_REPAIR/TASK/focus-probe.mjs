import fs from 'node:fs'; import path from 'node:path'; import {pathToFileURL} from 'node:url';
const {chromium}=await import(pathToFileURL(path.resolve('projects/chirality-piping/node_modules/playwright/index.mjs')).href);
const fixture=JSON.parse(fs.readFileSync('projects/chirality-piping/apps/desktop/e2e/ui-foundation/fixtures/precision-origin-base.model.json','utf8'));
const original=fixture.materials[0];fixture.materials=Array.from({length:5},(_,i)=>({...original,id:`material:scroll-${i}${i===0?'-retained-identity'.repeat(8):''}`,label:`Invented scroll material ${i}`}));fixture.pipe_segments=fixture.pipe_segments.map(p=>({...p,material:p.material===original.id?fixture.materials[0].id:p.material}));
const results=[];
function instrument(){
 window.__focusProbe=[];
 const desc=e=>e?{tag:e.tagName,label:e.getAttribute?.('aria-label'),id:e.id,disabled:e.disabled,connected:e.isConnected}:null;
 window.__recordFocus=(event,target,extra={})=>window.__focusProbe.push({time:performance.now(),event,target:desc(target),active:desc(document.activeElement),...extra});
 for(const name of ['pointerdown','mousedown','focus','blur','focusin','focusout','mouseup','click'])document.addEventListener(name,e=>window.__recordFocus(name,e.target,{related:desc(e.relatedTarget),trusted:e.isTrusted}),true);
 const originalSet=Element.prototype.setAttribute;
 Element.prototype.setAttribute=function(name,value){const watch=this.tagName==='BUTTON'&&name==='disabled';if(watch)window.__recordFocus('disabled-before',this);const result=originalSet.call(this,name,value);if(watch)window.__recordFocus('disabled-after',this);return result;};
}
for(const [name,options]of[['installed-chrome',{executablePath:'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome'}],['bundled-chromium',{}]]){
 const browser=await chromium.launch({...options,headless:true,ignoreDefaultArgs:['--hide-scrollbars']});
 try{
 const page=await browser.newPage({viewport:{width:1280,height:800}});await page.addInitScript(instrument);await page.addInitScript(()=>localStorage.setItem('chirality.desktop.ui-preferences.v1',JSON.stringify({version:1,density:'comfortable',tableDrawerPx:180})));
 await page.route(/fixtures\/product_preview\/invented_preview_model\.json(?:\?.*)?$/,r=>r.fulfill({contentType:'application/javascript',body:`export default ${JSON.stringify(fixture)};`}));
 await page.goto('http://127.0.0.1:5174');await page.getByTestId('workspace-toolbar').waitFor();await page.addStyleTag({content:'*::-webkit-scrollbar{width:18px;height:18px}'});await page.getByTestId('view-switch-model').click();if(await page.getByTestId('toggle-tree').getAttribute('aria-expanded')!=='true')await page.getByTestId('toggle-tree').click();await page.getByTestId('layout-mode-grid').click();await page.getByRole('combobox',{name:'Grid family'}).selectOption('materials');
 const later=page.getByRole('button',{name:'Later columns',exact:true}),earlier=page.getByRole('button',{name:'Earlier columns',exact:true});while(await later.isEnabled())await later.click();
 const table=page.getByTestId('material-engineering-table'),cell=table.getByTestId(`table-cell-${fixture.materials[0].id}-provenance`);await cell.click();await cell.press('Enter');await table.getByRole('textbox').fill('Retained valid draft while navigating chrome');await table.getByRole('button',{name:'Later table status',exact:true}).click();
 await page.evaluate(()=>{window.__focusProbe=[];window.__recordFocus('before-earlier',document.activeElement)});await earlier.click();await page.evaluate(()=>window.__recordFocus('after-earlier',document.activeElement));
 const product=await page.evaluate(()=>({events:window.__focusProbe,offset:document.querySelector('[data-testid="material-engineering-table"] .engineering-table-row').style.transform,draft:document.querySelector('[data-testid="material-engineering-table"] input')?.value,undoDisabled:document.querySelector('[data-testid="workspace-undo"]').disabled}));
 await page.setContent('<button id="disable" aria-label="Disable self">Disable</button><button id="other" aria-label="Other">Other</button>');await page.evaluate(instrument);await page.evaluate(()=>document.querySelector('#disable').onclick=e=>{e.currentTarget.setAttribute('disabled','');window.__recordFocus('after-handler',e.currentTarget)});await page.locator('#disable').click();const minimal=await page.evaluate(()=>window.__focusProbe);
 results.push({engine:name,version:browser.version(),platform:process.platform,product,minimal});await page.close();
 }finally{await browser.close();}
}
fs.writeFileSync(new URL('./_run_records/focus-probe.json',import.meta.url),JSON.stringify(results,null,2));console.log(results.map(r=>({engine:r.engine,version:r.version,events:r.product.events,offset:r.product.offset})))
