/* Manager-run self-check only; independent witness belongs to root. Actual UI actions, read-only state hook. */
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
const here=path.dirname(fileURLToPath(import.meta.url));
const modulePath=process.env.PROTOTYPE_PLAYWRIGHT;
if(!modulePath)throw new Error('PROTOTYPE_PLAYWRIGHT exact module path is required and must be recorded in invocation evidence');
const {chromium}=await import(modulePath);
const output=path.join(here,'_run_records','witness-v1');
fs.mkdirSync(output,{recursive:true});
const reports=[];
const assert=(condition,message)=>{if(!condition)throw new Error(message);};
const browser=await chromium.launch({headless:true});
try {
matrix: for (const [width,height] of [[1024,768],[1280,800],[1440,920]]) for(const density of ['comfortable','compact']) {
 const context=await browser.newContext({viewport:{width,height}}),page=await context.newPage();
 const errors=[],network=[],states=[];
 page.on('pageerror',e=>errors.push(e.message));page.on('console',m=>{if(m.type()==='error')errors.push(m.text());});
 page.on('request',r=>{if(!r.url().startsWith('file:'))network.push(r.url());});
 const inspect=()=>page.evaluate(()=>window.prototypeInspect());
 const snap=async(label)=>states.push({label,state:await inspect()});
 const click=selector=>page.locator(selector).click();
 const tool=async(group,row)=>{await click(`[data-group="${group}"]`);await click(`[data-capability="${row}"]`);};
 const scenario=async(id,value)=>{await click('#scenarios-toggle');await page.locator('#'+id).selectOption(value);await click('#scenarios-close');};
 const apply=async()=>{await click('#add-draft');await click('#validate-simulation');await click('#apply');};
 const property=async(value)=>{await click('[data-group="Properties"]');await click('[data-property]');await page.locator('#pipe').selectOption('P5');await page.locator('#value').fill(String(value));await page.locator('#provenance').fill('Explicit invented witness property correction');await apply();};
 const started=Date.now();let failure=null;
 try {
  await page.goto(pathToFileURL(path.join(here,'prototype/index.html')).href);await page.locator('#density').selectOption(density);
  assert(await page.locator('.simulation').isVisible(),'simulation badge visible');await snap('initial');
  // A: one authoritative pointer/typed draft, orbit gate, frozen atomic apply, own continuation, cancel, history.
  await tool('Build',1);await page.locator('#from').selectOption('N5');await page.locator('#plane').selectOption('XZ');await page.locator('#axis').selectOption('X');
  const enteredUnit=width===1024?'mm':width===1280?'ft':'in';await page.locator('#unit').selectOption(enteredUnit);
  const rect=await page.locator('#canvas').boundingBox();const beforePoint=await inspect();
  await page.mouse.click(rect.x+rect.width*.75,rect.y+rect.height*.22);
  const point=await inspect();assert(point.generation>beforePoint.generation,'blank-plane click updated draft');assert(point.historyCount===0,'point creates no checkpoint');
  const xyz=[point.draft.x,point.draft.y,point.draft.z];await page.mouse.move(rect.x+rect.width*.4,rect.y+rect.height*.5);await page.mouse.down();await page.mouse.move(rect.x+rect.width*.4+30,rect.y+rect.height*.5+8,{steps:5});await page.mouse.up();
  const orbited=await inspect();assert(JSON.stringify(xyz)===JSON.stringify([orbited.draft.x,orbited.draft.y,orbited.draft.z]),'orbit authored no coordinates');
  // Explicit entered coordinates, each unit, replaces pointer capture in the same draft.
  const factor={mm:.001,ft:.3048,in:.0254}[enteredUnit];for(const [id,n] of [['x',6.2],['y',1.8],['z',3]])await page.locator('#'+id).fill(String(n/factor));
  await page.locator('#continue-check').check();await click('#add-draft');const frozen=await inspect();assert(frozen.frozen.submission.operations.length===2,'new route freezes two members');await snap('route-frozen');
  await click('#validate-simulation');await click('#apply');const applied=await inspect();assert(applied.historyCount===1&&applied.nodeCount===9&&applied.pipeCount===7,'atomic route one checkpoint');
  await click('#continue');const continued=await inspect();assert(continued.draft.from===applied.lastRoute.end,'continuation uses own endpoint');assert(continued.draft.unit===enteredUnit,'continuation retains entered units');assert(continued.draft.x==='','continuation requests explicit next XYZ');
  await click('#cancel-draft');assert((await inspect()).historyCount===1,'cancel no checkpoint');await click('#undo');assert((await inspect()).historyCount===0,'undo one checkpoint');await click('#redo');assert((await inspect()).historyCount===1,'redo one checkpoint');await snap('route-redo');
  // Busy/cancel/late callback guard is actual visible scenario interaction.
  await tool('Build',1);await click('#add-draft');await scenario('operation-scenario','busy');assert(await page.locator('#apply').isDisabled(),'busy blocks apply');const busyCount=(await inspect()).historyCount;await click('#cancel-draft');await click('#scenarios-toggle');await click('#complete-held');await click('#scenarios-close');assert((await inspect()).phase==='cancelled'&&(await inspect()).historyCount===busyCount,'withdrawn completion ignored');
  await scenario('operation-scenario','ready');await tool('Build',1);await click('#add-draft');await scenario('operation-scenario','stale');assert(await page.locator('#apply').isDisabled(),'stale blocks Apply');await click('#cancel-draft');await scenario('operation-scenario','ready');
  // B: explicit incidence symbol, finite backfill blocker, attached transform blocker, required support inputs, property correction.
  await click('[data-testid="tree-N5"]');await tool('Build',15);await page.locator('#node').selectOption('N5');await page.locator('#kind').selectOption('Valve');await page.locator('#primary').selectOption('P5');await page.locator('#secondary').selectOption('P4');await apply();assert((await inspect()).componentCount===1,'explicit component symbol applied');
  await tool('Build',15);await page.locator('#mode').selectOption('finite');await click('#add-draft');assert((await inspect()).phase==='blocked','finite insertion blocked');await click('#cancel-draft');
  const beforeTransform=(await inspect()).historyCount;await tool('Edit',20);await page.locator('#pipe').selectOption('P5');await click('#add-draft');assert((await inspect()).frozen.errors.some(e=>e.includes('ATTACHED-TRANSFORM-BLOCKED')),'attached transform named blocker');assert((await inspect()).historyCount===beforeTransform,'attached transform unchanged checkpoint');await click('#cancel-draft');
  await tool('Supports',2);await click('#add-draft');assert((await inspect()).phase==='blocked','missing support inputs remain missing');await page.locator('#provenance').fill('Invented witness anchor DOFs');for(const dof of ['Ux','Uy','Uz','Rx','Ry','Rz'])await page.locator(`[data-dof="${dof}"]`).check();await apply();assert((await inspect()).supportCount===2,'explicit support applied');
  await property(7.1);const correction=await inspect();assert(correction.frozen.submission.intent.change.before==='6.02'&&correction.frozen.submission.intent.change.after==='7.1','set_field exact before after');await snap('component-support-property');
  // C: actual solve controls generate invented quantities, edit clears Current, reopen excludes overlays/readiness.
  await click('#tab-results');await click('#solve');let current=await inspect();assert(current.result.status==='Current'&&current.result.overlay,'Current invented overlay');assert(current.result.rows.find(q=>q.kind==='rotation').dimension==='angle','rotation is angle');assert(current.result.rows.find(q=>q.kind==='diagnostic').unit==='N*m','work residual diagnostic N*m');await click('[data-result="1"]');assert((await inspect()).selection.ref==='N5','linked result typed selection');await snap('current-results');
  await page.screenshot({path:path.join(output,`${width}x${height}-${density}-current.png`)});
  await property(7.3);assert((await inspect()).result.status==='Unavailable'&&!(await inspect()).result.overlay,'edit invalidates Current');await click('#tab-results');await click('#reopen');let historical=await inspect();assert(historical.result.status==='Historical'&&historical.result.acceptance==='UNKNOWN'&&!historical.result.overlay&&!historical.result.readiness,'Historical excludes Current readiness/overlay');await snap('historical-results');
  await page.screenshot({path:path.join(output,`${width}x${height}-${density}-historical.png`)});
  for(const [outcome,status] of [['blocked','Blocked'],['nonconverged','Nonconverged'],['cancelled','Cancelled']]){await scenario('solve-scenario',outcome);await click('#solve');assert((await inspect()).result.status===status,'named solve '+status);assert((await inspect()).result.rows.length===0,'no successful rows for '+status);}
  await scenario('solve-scenario','current');await click('#solve');assert((await inspect()).result.status==='Current','Current solve restores Current');
  // Complete vocabulary discoverability uses actual search and visible information route; no state setter.
  const normative=await page.evaluate(()=>CAPABILITIES.filter(c=>typeof c.row==='number').map(c=>({row:c.row,label:c.label})));
  for(const {row,label} of normative){await page.locator('#tool-search').fill(label);assert(await page.locator(`[data-capability="${row}"]`).isVisible(),'normative row '+row+' discoverable');await click(`[data-capability="${row}"]`);}
  for(const row of ['R1','R2','R3']){await page.locator('#tool-search').fill(row==='R1'?'renumbering':row==='R2'?'snubbers':'cold spring');await click(`[data-capability="${row}"]`);assert(await page.locator('.task-rail .warning').isVisible(),'roadmap explanation visible');assert(await page.locator('#add-draft').isDisabled(),'roadmap no apply semantics');}
  await page.locator('#tool-search').fill('');await tool('Build',1);await page.keyboard.press('/');assert(await page.locator('#tool-search').evaluate(e=>e===document.activeElement),'keyboard command search');await page.keyboard.press('Tab');await page.keyboard.press('Escape');
  await click('#scenarios-toggle');await page.keyboard.press('Escape');assert(await page.locator('#scenarios-toggle').evaluate(e=>e===document.activeElement),'Escape returns scenario focus');
  const geometry=await page.evaluate(()=>{const b=e=>{const r=e.getBoundingClientRect();return{x:r.x,y:r.y,width:r.width,height:r.height,scrollWidth:e.scrollWidth,clientWidth:e.clientWidth,scrollHeight:e.scrollHeight,clientHeight:e.clientHeight};};return{viewport:{width:innerWidth,height:innerHeight},body:b(document.body),canvas:b(document.getElementById('canvas')),editor:b(document.getElementById('editor')),dock:b(document.getElementById('dock-body')),apply:b(document.getElementById('apply')),cancel:b(document.getElementById('cancel-draft')),rows:CAPABILITIES.map(c=>c.row)};});
  assert(geometry.body.scrollWidth<=width&&geometry.body.scrollHeight<=height,'no body overflow');assert(geometry.canvas.width>250&&geometry.canvas.height>200,'substantial visible canvas');assert(geometry.apply.x+geometry.apply.width<=width&&geometry.cancel.y+geometry.cancel.height<=height,'Apply/Cancel unclipped');assert(geometry.rows.length===27,'24 normative and 3 roadmap rows');assert(errors.length===0,'no page/console errors');assert(network.length===0,'no unexpected network');
  reports.push({width,height,density,pass:true,elapsedMs:Date.now()-started,geometry,states,errors,network});
 } catch(e){failure=String(e.stack||e);reports.push({width,height,density,pass:false,failure,states,errors,network});await page.screenshot({path:path.join(output,`${width}x${height}-${density}-failure.png`)});}
 await context.close();fs.writeFileSync(path.join(output,'REPORT.json'),JSON.stringify(reports,null,2));
 console.log(JSON.stringify({width,height,density,pass:!failure,failure}));
 if(failure)break matrix;
}
} finally {await browser.close();}
if(reports.some(r=>!r.pass))process.exitCode=1;
console.log('Evidence is browser interaction self-check of design simulation only. No Rust/schema/solver/acceptance proof.');
