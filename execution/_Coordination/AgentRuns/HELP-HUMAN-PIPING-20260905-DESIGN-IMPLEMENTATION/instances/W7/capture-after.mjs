import { chromium, expect } from '@playwright/test';
import { writeFile } from 'node:fs/promises';
const browser=await chromium.launch({headless:true});
const page=await browser.newPage({viewport:{width:1024,height:768}});
const evidence=[];const errors=[];page.on('pageerror',error=>errors.push(error.message));
const out=name=>new URL(name,import.meta.url);
async function capture(name,note=''){
 await expect.poll(async()=>page.evaluate(()=>{const c=document.querySelector('canvas')?.getBoundingClientRect();const h=document.querySelector('[data-testid=viewport-canvas]')?.getBoundingClientRect();return Boolean(c&&h&&Math.abs(c.width-h.width)<2&&Math.abs(c.height-h.height)<2)}),{timeout:5000}).toBe(true);
 await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));
 const geometry=await page.evaluate(()=>({window:{width:innerWidth,height:innerHeight},body:{scrollWidth:document.body.scrollWidth,clientWidth:document.body.clientWidth},boxes:Object.fromEntries(['.modeling-workspace','.workspace-pane-viewport','canvas','.workspace-pane-inspector','.viewport-editor-intents','.workspace-dock','.workspace-status-bar','.toolkit-commands'].map(selector=>{const e=document.querySelector(selector);return[selector,e?{...e.getBoundingClientRect().toJSON(),display:getComputedStyle(e).display,visibility:getComputedStyle(e).visibility,clientHeight:e.clientHeight,scrollHeight:e.scrollHeight}:null]}))}));
 await page.screenshot({path:out('after-'+name+'.png').pathname});evidence.push({state:name,note,geometry});
 await writeFile(out('after-geometry.json'),JSON.stringify({substrate:'Chromium source preview; browser-memory storage and bundled fixture result are not native backend evidence',evidence,errors},null,2));
}
async function section(id){if(!(await page.getByTestId('workspace-section-'+id).isVisible())){await page.getByTestId('menu-view').click();await page.getByTestId('menu-item-view.section.'+id).click();}}
async function rail(id,open=true){const b=page.getByTestId('toggle-'+id);if((await b.getAttribute('aria-expanded')==='true')!==open)await b.click();}
async function tool(id){await page.getByTestId('toolkit-entry').click();await page.getByTestId('toolkit-'+id).click();}
async function closeDock(){if(await page.getByTestId('workspace-dock-close').isVisible())await page.getByTestId('workspace-dock-close').click();}
try{
 await page.goto('http://127.0.0.1:5178');await expect(page.getByTestId('operation-engine-chip')).toContainText('Engine ready');
 await capture('default','Public invented fixture, no new values or edits.');
 await page.getByTestId('toolkit-entry').click();await capture('discovery','Transient command discovery; geometry should retain bounds.');await page.keyboard.press('Escape');
 await page.getByTestId('menu-file').click();await page.getByTestId('menu-item-file.new-blank').click();await expect(page.getByTestId('local-project-message')).toContainText('Created blank');await capture('empty','Empty browser-memory project; no supplied engineering defaults.');
 await page.goto('http://127.0.0.1:5178');await expect(page.getByTestId('operation-engine-chip')).toContainText('Engine ready');
 await tool('build.pipe');await capture('routing','Explicit existing endpoint routing form; automatic bends/connections are not claimed.');
 await page.getByTestId('cancel-pipe-draft').click();await rail('tree');await page.getByTestId('tree-row-pipe:P-100').click();await rail('inspector');await rail('tree',false);await capture('selected-pipe','Name is text, geometry uses supplied quantities.');
 await page.getByTestId('command-support').click();await expect(page.getByTestId('create-support-id')).toBeFocused();await capture('direct-support','Direct Support command opens and focuses the creation form; no model mutation.');
 await rail('tree');await page.getByTestId('tree-row-support:S-120').click();await rail('tree',false);await tool('supports.restraint');await capture('support','Existing support configuration, no default engineering choice applied.');
 await section('loads');await capture('loads','Existing public fixture load editor in bounded drawer.');
 await section('solve');await page.getByTestId('run-mechanics-preview').click();await expect(page.getByTestId('solve-job-summary')).toContainText('state=completed');await section('results');await capture('results','Bundled unedited fixture results, browser_fixture_no_backend_job; no claim of native solve.');
 await closeDock();await rail('tree');await page.getByTestId('tree-row-node:N-100').click();await rail('tree',false);await rail('inspector');await page.getByTestId('editor-intent-field').selectOption('position.x');await page.getByTestId('editor-intent-value').fill('not-a-number');await capture('field-error','Invalid numeric draft; model not changed.');
 await page.getByTestId('editor-intent-value').fill('0');await rail('tree');await page.getByTestId('tree-row-pipe:P-100').click();await rail('tree',false);await page.getByTestId('editor-intent-field').selectOption('section.material_density.value');await page.getByTestId('editor-intent-value').fill('7800');await page.getByTestId('editor-intent-unit').fill('kg/m^3');await page.getByTestId('queue-editor-intent').click();await tool('review.pending');await page.getByTestId('apply-intent-editor-intent-1').click();await expect(page.getByTestId('operation-apply-summary')).toContainText('1 applied');
 await tool('loads.self-weight');const panel=page.locator('#self-weight-plan');
 for(const[label,value]of[['Self-weight case ID','load:layout-weight'],['Self-weight case label','Invented layout rehearsal'],['Gravity value','-9.81'],['Gravity unit','m/s^2'],['Self-weight provenance','invented layout verification data']])await panel.getByLabel(label,{exact:true}).fill(value);
 await panel.getByLabel('Gravity direction').selectOption('global_y');await panel.getByLabel('pipe:P-100',{exact:true}).check();await panel.getByRole('button',{name:'Generate self-weight plan'}).click();await panel.getByRole('button',{name:'Queue complete self-weight plan'}).click();await page.getByTestId('validate-batch-operation-batch-1').click();await expect(page.getByText(/Preview only. Temporary state was discarded/)).toBeVisible();await capture('batch-review','Two-step self-weight batch validation against real local Wasm; explicit invented data; no acceptance from validation.');
 await page.getByTestId('apply-batch-operation-batch-1').click();await expect(page.getByTestId('batch-review-summary')).toContainText('1 batches applied');await page.getByTestId('undo-session-model-edit').click();await expect(page.getByTestId('tree-row-load:layout-weight')).toHaveCount(0);
 await capture('batch-undone','Explicit batch application then one undo restored prior model.');
 await writeFile(out('after-result.json'),JSON.stringify({status:'PASS',states:evidence.map(e=>e.state),errors},null,2));
}catch(error){await capture('failure',String(error));await writeFile(out('after-result.json'),JSON.stringify({status:'FAIL',error:String(error),states:evidence.map(e=>e.state),errors},null,2));throw error;}finally{await browser.close();}
